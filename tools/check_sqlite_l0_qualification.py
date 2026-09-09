"""Verify the governed SQLite L0 qualification witnesses.

Completeness is bound to the raw bytes of the two governed Rust sources. The
checker verifies the inventory digest before extraction, refuses to run Cargo
when the reviewed source bytes change, and then discovers and executes every
inventory witness through exact Cargo test selection. Checking never updates
inventory hashes, witness metadata, or source files.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INVENTORY_PATH = ROOT / "tools" / "sqlite_l0_qualification_inventory.json"
RUST_TOOLCHAIN = "1.98.1"
PROFILE = "magpie-public-prealpha-assurance-v1"
LANE = "sqlite-l0-resource-crash-recovery-v0"
INTEGRATION_SOURCE = "crates/magpie-log/tests/sqlite_l0.rs"
LIB_SOURCE = "crates/magpie-log/src/sqlite_l0.rs"
GOVERNED_SOURCES = (INTEGRATION_SOURCE, LIB_SOURCE)
LIB_TEST_PREFIX = "sqlite_l0::tests::"
SELF_SPAWNING_WITNESS_IDS = (
    "integration/p12_lost_process_acknowledgement_requires_reopen_instead_of_blind_retry",
    "integration/supported_reopen_recovers_owned_hot_journal_and_preserves_committed_prefix",
)
EXPECTED_TOTAL = 40
EXPECTED_SOURCE_COUNTS = {
    INTEGRATION_SOURCE: 32,
    LIB_SOURCE: 8,
}
EXPECTED_CLASSIFICATION_COUNTS = {"EXACT": 36, "PARTIAL": 4}
EXPECTED_INVENTORY_SHA256 = "22766d3fe7dc09469a561ec88e09b467cf919b5349360ef867545c1be9056cf1"
ALLOWED_CLASSIFICATIONS = frozenset({"EXACT", "PARTIAL", "ADJACENT"})

SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}$")
FAMILY_PATTERN = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
TEST_ATTRIBUTE = re.compile(r"^[ \t]*#\[test\][ \t]*$")
FUNCTION_DECLARATION = re.compile(
    r"^[ \t]*(?:pub(?:\(crate\))? )?fn (?P<name>[A-Za-z_][A-Za-z0-9_]*)[ \t]*\("
)
LIST_LINE = re.compile(r"^(?P<name>.+): test$")
RESULT_LINE = re.compile(
    r"^test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured;"
    r" [0-9]+ filtered out; finished in [0-9.]+s$"
)


class SqliteL0CheckError(Exception):
    """Fail-closed SQLite L0 qualification error."""


@dataclass(frozen=True)
class ExtractedWitness:
    source: str
    ordinal: int
    test_kind: str
    test_name: str
    cargo_test_name: str
    source_line: int

    @property
    def witness_id(self) -> str:
        return f"{self.test_kind}/{self.test_name}"


@dataclass(frozen=True)
class InventoryWitness:
    witness_id: str
    source: str
    ordinal: int
    test_kind: str
    test_name: str
    cargo_test_name: str
    source_line: int
    classification: str
    families: tuple[str, ...]
    overclaim_risk: str
    claim_fence: str


def _source_lines(text: str) -> list[str]:
    lines = text.split("\n")
    if text.endswith("\n"):
        del lines[-1]
    return lines


def _is_sha256(value: object) -> bool:
    return isinstance(value, str) and bool(SHA256_PATTERN.fullmatch(value))


def _string_field(entry: dict[str, object], field: str) -> str:
    value = entry[field]
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{field} must be a non-empty string")
    return value


def _positive_int_field(entry: dict[str, object], field: str) -> int:
    value = entry[field]
    if isinstance(value, bool) or not isinstance(value, int) or value < 1:
        raise ValueError(f"{field} must be a positive integer")
    return value


def _families_field(entry: dict[str, object], field: str) -> tuple[str, ...]:
    value = entry[field]
    if not isinstance(value, list) or not value:
        raise ValueError(f"{field} must be a non-empty list")
    families = []
    for item in value:
        if not isinstance(item, str) or FAMILY_PATTERN.fullmatch(item) is None:
            raise ValueError(f"{field} contains an invalid family")
        families.append(item)
    if len(set(families)) != len(families):
        raise ValueError(f"{field} contains duplicates")
    if families != sorted(families):
        raise ValueError(f"{field} must be sorted")
    return tuple(families)


def _claim_fence_field(entry: dict[str, object], field: str) -> str:
    value = _string_field(entry, field)
    if "WITNESSES: " not in value or "NOT: " not in value:
        raise ValueError(f"{field} must contain WITNESSES and NOT boundaries")
    return value


def verify_inventory_digest(raw_text: str) -> None:
    actual = hashlib.sha256(raw_text.encode("utf-8")).hexdigest()
    if actual != EXPECTED_INVENTORY_SHA256:
        raise SqliteL0CheckError(
            "SQLite L0 qualification inventory digest does not match the governed "
            f"inventory (expected {EXPECTED_INVENTORY_SHA256}, actual {actual}); "
            "explicit review of the qualification inventory is required"
        )


def load_inventory(
    path: Path = INVENTORY_PATH,
) -> tuple[dict[str, object], list[InventoryWitness]]:
    try:
        raw_text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as error:
        raise SqliteL0CheckError(f"cannot read SQLite L0 qualification inventory: {error}") from error
    verify_inventory_digest(raw_text)
    try:
        raw = json.loads(raw_text)
    except json.JSONDecodeError as error:
        raise SqliteL0CheckError(f"cannot read SQLite L0 qualification inventory: {error}") from error
    if not isinstance(raw, dict):
        raise SqliteL0CheckError("SQLite L0 qualification inventory must be a JSON object")
    entries = raw.get("witnesses")
    if not isinstance(entries, list):
        raise SqliteL0CheckError("inventory witnesses must be a list")
    witnesses: list[InventoryWitness] = []
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise SqliteL0CheckError(f"malformed witness at index {index}: not an object")
        try:
            witness = InventoryWitness(
                witness_id=_string_field(entry, "witness_id"),
                source=_string_field(entry, "source"),
                ordinal=_positive_int_field(entry, "ordinal"),
                test_kind=_string_field(entry, "test_kind"),
                test_name=_string_field(entry, "test_name"),
                cargo_test_name=_string_field(entry, "cargo_test_name"),
                source_line=_positive_int_field(entry, "source_line"),
                classification=_string_field(entry, "classification"),
                families=_families_field(entry, "families"),
                overclaim_risk=_string_field(entry, "overclaim_risk"),
                claim_fence=_claim_fence_field(entry, "claim_fence"),
            )
        except (KeyError, TypeError, ValueError) as error:
            raise SqliteL0CheckError(f"malformed witness at index {index}: {error}") from error
        witnesses.append(witness)
    return raw, witnesses


def extract_test_witnesses(source: str, text: str) -> list[ExtractedWitness]:
    if source == INTEGRATION_SOURCE:
        test_kind = "integration"
    elif source == LIB_SOURCE:
        test_kind = "lib"
    else:
        raise SqliteL0CheckError(f"not a governed SQLite L0 source: {source}")

    lines = _source_lines(text)
    witnesses: list[ExtractedWitness] = []
    index = 0
    while index < len(lines):
        if TEST_ATTRIBUTE.fullmatch(lines[index]) is None:
            index += 1
            continue

        function_index = index + 1
        while function_index < len(lines) and not lines[function_index].strip():
            function_index += 1
        if function_index >= len(lines):
            raise SqliteL0CheckError(
                f"{source}:{index + 1}: #[test] has no following function declaration"
            )
        match = FUNCTION_DECLARATION.match(lines[function_index])
        if match is None:
            raise SqliteL0CheckError(
                f"{source}:{function_index + 1}: #[test] is not immediately followed by a test function"
            )

        test_name = match.group("name")
        cargo_test_name = (
            f"{LIB_TEST_PREFIX}{test_name}" if test_kind == "lib" else test_name
        )
        witnesses.append(
            ExtractedWitness(
                source=source,
                ordinal=len(witnesses) + 1,
                test_kind=test_kind,
                test_name=test_name,
                cargo_test_name=cargo_test_name,
                source_line=function_index + 1,
            )
        )
        index = function_index
    return witnesses


def validate_inventory_contract(
    document: dict[str, object], witnesses: list[InventoryWitness]
) -> None:
    schema_version = document.get("schema_version")
    if isinstance(schema_version, bool) or schema_version != 1:
        raise SqliteL0CheckError("inventory schema_version must be 1")
    if document.get("profile") != PROFILE:
        raise SqliteL0CheckError(f"inventory profile must be {PROFILE}")
    if document.get("lane") != LANE:
        raise SqliteL0CheckError(f"inventory lane must be {LANE}")

    identities = document.get("governed_sources")
    if not isinstance(identities, dict) or set(identities) != set(GOVERNED_SOURCES):
        raise SqliteL0CheckError("inventory must bind exactly the two governed SQLite L0 sources")
    for source in GOVERNED_SOURCES:
        if not _is_sha256(identities[source]):
            raise SqliteL0CheckError(f"invalid governed source identity for {source}")

    if document.get("source_file_counts") != EXPECTED_SOURCE_COUNTS:
        raise SqliteL0CheckError("inventory source_file_counts changed")
    if document.get("classification_counts") != EXPECTED_CLASSIFICATION_COUNTS:
        raise SqliteL0CheckError("inventory classification_counts changed")
    if len(witnesses) != EXPECTED_TOTAL:
        raise SqliteL0CheckError(
            f"inventory witness count changed: expected {EXPECTED_TOTAL}, observed {len(witnesses)}"
        )

    seen_ids: set[str] = set()
    for witness in witnesses:
        if witness.witness_id in seen_ids:
            raise SqliteL0CheckError(f"duplicate witness identity: {witness.witness_id}")
        seen_ids.add(witness.witness_id)

        expected_kind = (
            "integration" if witness.source == INTEGRATION_SOURCE else "lib"
        )
        if witness.source not in GOVERNED_SOURCES or witness.test_kind != expected_kind:
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: test_kind must match its governed source"
            )
        expected_cargo_name = (
            f"{LIB_TEST_PREFIX}{witness.test_name}"
            if witness.test_kind == "lib"
            else witness.test_name
        )
        if witness.cargo_test_name != expected_cargo_name:
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: cargo_test_name must be {expected_cargo_name}"
            )
        if witness.witness_id != f"{witness.test_kind}/{witness.test_name}":
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: witness_id must use test_kind/test_name"
            )
        if witness.classification not in ALLOWED_CLASSIFICATIONS:
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: invalid classification {witness.classification!r}"
            )
        if not witness.families:
            raise SqliteL0CheckError(f"witness {witness.witness_id}: families must be non-empty")
        if not witness.overclaim_risk.strip():
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: overclaim_risk must be non-empty"
            )
        if "WITNESSES: " not in witness.claim_fence or "NOT: " not in witness.claim_fence:
            raise SqliteL0CheckError(
                f"witness {witness.witness_id}: claim_fence must contain WITNESSES and NOT boundaries"
            )

    source_counts = Counter(witness.source for witness in witnesses)
    if dict(source_counts) != EXPECTED_SOURCE_COUNTS:
        raise SqliteL0CheckError("derived source_file_counts do not match inventory")
    classification_counts = Counter(witness.classification for witness in witnesses)
    if dict(classification_counts) != EXPECTED_CLASSIFICATION_COUNTS:
        raise SqliteL0CheckError("derived classification_counts do not match inventory")
    for source in GOVERNED_SOURCES:
        expected_count = EXPECTED_SOURCE_COUNTS[source]
        ordinals = [
            witness.ordinal for witness in witnesses if witness.source == source
        ]
        if ordinals != list(range(1, expected_count + 1)):
            raise SqliteL0CheckError(f"source ordinals are not sequential for {source}")


def verified_governed_source_bytes(
    document: dict[str, object],
) -> dict[str, bytes]:
    identities = document.get("governed_sources")
    if not isinstance(identities, dict) or set(identities) != set(GOVERNED_SOURCES):
        raise SqliteL0CheckError(
            "inventory must bind exactly the two governed SQLite L0 sources"
        )

    snapshots: dict[str, bytes] = {}
    for source in GOVERNED_SOURCES:
        expected = identities[source]
        if not _is_sha256(expected):
            raise SqliteL0CheckError(f"invalid governed source identity for {source}")
        try:
            raw = (ROOT / source).read_bytes()
        except OSError as error:
            raise SqliteL0CheckError(f"cannot read governed source {source}: {error}") from error
        actual = hashlib.sha256(raw).hexdigest()
        if actual != expected:
            raise SqliteL0CheckError(
                "governed source digest changed; explicit SQLite L0 inventory review required "
                f"for {source}\n"
                f"  expected: {expected}\n"
                f"  actual:   {actual}\n"
                "Review the complete source diff and all qualification witnesses before manually "
                "updating the inventory; checking never updates hashes."
            )
        snapshots[source] = raw
    return snapshots


def extract_governed_sources(document: dict[str, object]) -> list[ExtractedWitness]:
    snapshots = verified_governed_source_bytes(document)
    extracted: list[ExtractedWitness] = []
    for source in GOVERNED_SOURCES:
        try:
            text = snapshots[source].decode("utf-8")
        except UnicodeDecodeError as error:
            raise SqliteL0CheckError(f"governed source is not valid UTF-8: {source}") from error
        extracted.extend(extract_test_witnesses(source, text))
    return extracted


def compare_source_inventory(
    extracted: list[ExtractedWitness], inventory: list[InventoryWitness]
) -> None:
    seen_ids: set[str] = set()
    for witness in extracted:
        if witness.witness_id in seen_ids:
            raise SqliteL0CheckError(f"duplicate extracted witness identity: {witness.witness_id}")
        seen_ids.add(witness.witness_id)

    actual_by_id = {witness.witness_id: witness for witness in extracted}
    expected_by_id = {witness.witness_id: witness for witness in inventory}
    missing = sorted(set(expected_by_id) - set(actual_by_id))
    unexpected = sorted(set(actual_by_id) - set(expected_by_id))
    if missing or unexpected:
        details = [f"missing {witness_id}" for witness_id in missing]
        details.extend(f"unexpected {witness_id}" for witness_id in unexpected)
        raise SqliteL0CheckError(
            "source test inventory drifted: " + "; ".join(details)
        )

    for witness_id, expected in expected_by_id.items():
        actual = actual_by_id[witness_id]
        mismatches = [
            f"{field}: actual={getattr(actual, field)!r} expected={getattr(expected, field)!r}"
            for field in (
                "source",
                "ordinal",
                "test_kind",
                "test_name",
                "cargo_test_name",
                "source_line",
            )
            if getattr(actual, field) != getattr(expected, field)
        ]
        if mismatches:
            raise SqliteL0CheckError(
                f"extracted witness {witness_id} drifted from inventory: "
                + "; ".join(mismatches)
            )


def parse_test_listing(
    output: str, *, module_prefix: str | None = None
) -> tuple[str, ...]:
    names: list[str] = []
    for line in output.splitlines():
        match = LIST_LINE.fullmatch(line.strip())
        if match is None:
            continue
        name = match.group("name")
        if module_prefix is not None and not name.startswith(module_prefix):
            continue
        names.append(name)
    if len(set(names)) != len(names):
        raise SqliteL0CheckError("Cargo test listing reported duplicate test names")
    return tuple(names)


def _cargo_list_command(test_kind: str) -> list[str]:
    command = [
        "cargo",
        f"+{RUST_TOOLCHAIN}",
        "test",
        "-p",
        "magpie-log",
        "--locked",
    ]
    if test_kind == "integration":
        command.extend(("--test", "sqlite_l0"))
    elif test_kind == "lib":
        command.extend(("--lib",))
    else:
        raise SqliteL0CheckError(f"unsupported test kind: {test_kind}")
    command.extend(("--", "--list"))
    return command


def discover_cargo_tests(test_kind: str) -> tuple[str, ...]:
    result = subprocess.run(
        _cargo_list_command(test_kind),
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise SqliteL0CheckError(
            f"Cargo discovery failed for {test_kind} tests "
            f"(exit {result.returncode}):\n{result.stdout}\n{result.stderr}"
        )
    output = result.stdout + "\n" + result.stderr
    prefix = LIB_TEST_PREFIX if test_kind == "lib" else None
    return parse_test_listing(output, module_prefix=prefix)


def execution_command(witness: InventoryWitness) -> list[str]:
    command = [
        "cargo",
        f"+{RUST_TOOLCHAIN}",
        "test",
        "-p",
        "magpie-log",
        "--locked",
    ]
    if witness.test_kind == "integration":
        command.extend(("--test", "sqlite_l0", witness.test_name, "--", "--exact"))
    elif witness.test_kind == "lib":
        command.extend(("--lib", witness.cargo_test_name, "--", "--exact"))
    else:
        raise SqliteL0CheckError(f"unsupported test kind: {witness.test_kind}")
    return command


def validate_execution_result(
    witness: InventoryWitness, returncode: int, output: str
) -> None:
    if returncode != 0:
        raise SqliteL0CheckError(
            f"exact execution failed for {witness.witness_id} "
            f"(exit {returncode}):\n{output}"
        )

    lines = [line.strip() for line in output.splitlines()]
    expected_running_count = (
        2
        if witness.witness_id in SELF_SPAWNING_WITNESS_IDS
        else 1
    )
    running_count = sum(line == "running 1 test" for line in lines)
    if running_count != expected_running_count:
        raise SqliteL0CheckError(
            f"expected exactly {expected_running_count} 'running 1 test' line(s) for "
            f"{witness.witness_id}, observed {running_count}"
        )

    expected_test_line = f"test {witness.cargo_test_name} ... ok"
    test_line_count = sum(line == expected_test_line for line in lines)
    if test_line_count != 1:
        raise SqliteL0CheckError(
            f"expected exactly one passing test line for {witness.cargo_test_name}, "
            f"observed {test_line_count}"
        )

    result_line_count = sum(
        RESULT_LINE.fullmatch(line) is not None for line in lines
    )
    if result_line_count != 1:
        raise SqliteL0CheckError(
            f"expected exactly one passing single-test result for {witness.witness_id}, "
            f"observed {result_line_count}"
        )


def execute_witness(witness: InventoryWitness) -> None:
    result = subprocess.run(
        execution_command(witness),
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    validate_execution_result(
        witness,
        result.returncode,
        result.stdout + "\n" + result.stderr,
    )


def run_check() -> None:
    document, inventory = load_inventory()
    validate_inventory_contract(document, inventory)
    extracted = extract_governed_sources(document)
    compare_source_inventory(extracted, inventory)

    discovery_fields = {
        "integration": "test_name",
        "lib": "cargo_test_name",
    }
    for test_kind, field_name in discovery_fields.items():
        expected = {
            getattr(witness, field_name)
            for witness in inventory
            if witness.test_kind == test_kind
        }
        observed = set(discover_cargo_tests(test_kind))
        missing = expected - observed
        unexpected = observed - expected
        if missing or unexpected:
            details = []
            if missing:
                details.append("missing " + ", ".join(sorted(missing)))
            if unexpected:
                details.append("unexpected " + ", ".join(sorted(unexpected)))
            raise SqliteL0CheckError(
                f"Cargo discovery drifted for {test_kind} tests: " + "; ".join(details)
            )

    for witness in inventory:
        execute_witness(witness)

    source_counts = Counter(witness.source for witness in inventory)
    classification_counts = Counter(witness.classification for witness in inventory)
    executed_counts = Counter(witness.test_kind for witness in inventory)
    source_text = ", ".join(
        f"{source}={source_counts[source]}" for source in GOVERNED_SOURCES
    )
    classification_text = ", ".join(
        f"{classification}={classification_counts[classification]}"
        for classification in sorted(classification_counts)
    )
    print(
        "SQLite L0 qualification passed: "
        f"total={len(inventory)}; "
        f"integration={executed_counts['integration']}; "
        f"lib={executed_counts['lib']}; "
        f"sources[{source_text}]; "
        f"classifications[{classification_text}]; "
        "discovery=exact; execution=exact; "
        "governed_source_sha256=verified; "
        f"toolchain={RUST_TOOLCHAIN}"
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args(argv)
    try:
        run_check()
    except SqliteL0CheckError as error:
        print(f"SQLite L0 qualification FAILED: {error}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
