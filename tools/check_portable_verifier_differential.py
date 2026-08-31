#!/usr/bin/env python3
"""Run the frozen portable-verifier corpus through three real conformers.

The Python checker owns the frozen manifest commitment and complete input-byte
preflight.  This runner then obtains independent actual results from
``tools.portable_verifier``, the standalone Go batch command, and the
crate-internal Rust conformer test exporter.  No expected result is supplied
to any conformer while it is producing an actual result.

An ``ACCEPT`` here means only that the exact caller-supplied finite history
passed the selected conformers.  This tool does not establish trust,
authority, currentness, completeness, durability, or permission to act.
"""

from __future__ import annotations

import argparse
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, replace
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools import check_portable_verifier_python as python_checker  # noqa: E402
from tools import portable_verifier  # noqa: E402


MANIFEST_RELATIVE_PATH = Path("fixtures") / "verifier-language-v1" / "manifest.json"
GO_MODULE_RELATIVE_PATH = Path("tools") / "go-verify-chain"
RUST_OUTPUT_ENVIRONMENT = "MAGPIE_PORTABLE_DIFFERENTIAL_OUTPUT"
RUST_TEST_NAME = "export_complete_conformer_results_when_requested"
SOURCE_NAMES = ("rust", "python", "go")

GOVERNED_FIELDS = python_checker.GOVERNED_FIELDS
INVENTORY_KEYS = python_checker.INVENTORY_KEYS
REJECTION_CLASSES = python_checker.REJECTION_CLASSES
EXPECTED_CASE_COUNT = python_checker.EXPECTED_CASE_COUNT
MANIFEST_ID = python_checker.MANIFEST_ID
MAX_U64 = (1 << 64) - 1


class DifferentialHarnessError(RuntimeError):
    """A manifest, tool, output, or other operational harness failure."""


@dataclass(frozen=True)
class ObservedCase:
    """One tool-produced result, retaining the tool's case order and ID."""

    case_id: str
    result: Mapping[str, object]


@dataclass(frozen=True)
class SourceReport:
    """Actual results and derived inventory for one conformer."""

    name: str
    cases: tuple[ObservedCase, ...]
    inventory: Mapping[str, int]


@dataclass(frozen=True)
class DifferentialReport:
    """Deterministic result of one complete three-way run."""

    manifest_digest: str
    case_ids: tuple[str, ...]
    expected: tuple[Mapping[str, object], ...]
    expected_inventory: Mapping[str, int]
    sources: tuple[SourceReport, ...]
    mismatches: tuple[str, ...]

    @property
    def case_count(self) -> int:
        return len(self.case_ids)

    @property
    def passed(self) -> bool:
        return not self.mismatches


def _fail(message: str) -> None:
    raise DifferentialHarnessError(message)


def _manifest_path(root: Path) -> Path:
    return root / MANIFEST_RELATIVE_PATH


def _profile_identity(manifest: Mapping[str, object]) -> str:
    producing_context = manifest.get("producing_context")
    if type(producing_context) is not dict:
        _fail("manifest producing_context is not an object")
    profile = producing_context.get(python_checker.SIGNATURE_PROFILE_FIELD)
    if type(profile) is not str or not profile:
        _fail("manifest signature profile identity is not text")
    return profile


def _load_frozen_cases(
    root: Path,
) -> tuple[dict[str, object], str, tuple[python_checker.VerifiedCase, ...]]:
    """Pin the manifest and hash every input before any verifier runs."""

    # These are deliberately the existing checker entry points.  In
    # particular, load_verified_cases reads and hashes all 432 inputs before
    # returning, so no tool can consume a partially preflighted corpus.
    manifest, manifest_digest = python_checker.load_verified_manifest(root)
    cases = python_checker.load_verified_cases(manifest, root)
    return manifest, manifest_digest, cases


def _strict_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    value: dict[str, object] = {}
    for key, member in pairs:
        if key in value:
            raise ValueError(f"duplicate JSON member {key!r}")
        value[key] = member
    return value


def _reject_json_constant(token: str) -> Any:
    raise ValueError(f"non-RFC JSON constant {token!r}")


def _decode_json(raw: bytes, label: str) -> object:
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as error:
        _fail(f"{label}: output is not UTF-8: {error}")
    try:
        return json.loads(
            text,
            object_pairs_hook=_strict_object,
            parse_constant=_reject_json_constant,
        )
    except (json.JSONDecodeError, ValueError, TypeError) as error:
        _fail(f"{label}: invalid JSON: {error}")


def _validate_result(result: object, label: str) -> dict[str, object]:
    if not isinstance(result, Mapping):
        _fail(f"{label}: result is not an object")
    observed = dict(result)
    if set(observed) != set(GOVERNED_FIELDS):
        _fail(
            f"{label}: result fields differ: expected {list(GOVERNED_FIELDS)!r}, "
            f"got {sorted(observed)!r}"
        )

    verdict = observed["verdict"]
    if type(verdict) is not str or verdict not in {"ACCEPT", "REJECT"}:
        _fail(f"{label}: verdict is not exactly ACCEPT or REJECT text")

    for field in ("line", "record_index"):
        value = observed[field]
        if value is not None and not (
            type(value) is int and 0 <= value <= MAX_U64
        ):
            _fail(f"{label}: {field} is not null or an unsigned 64-bit integer")

    raw_hashes = observed["ordered_recomputed_hashes"]
    if type(raw_hashes) is not list or any(
        type(value) is not str
        or len(value) != 64
        or any(character not in "0123456789abcdef" for character in value)
        for value in raw_hashes
    ):
        _fail(f"{label}: ordered_recomputed_hashes is not an array of lowercase SHA-256 text")

    if verdict == "ACCEPT":
        if observed["class"] is not None:
            _fail(f"{label}: ACCEPT class is not null")
        if observed["line"] is not None or observed["record_index"] is not None:
            _fail(f"{label}: ACCEPT coordinates are not null")
        event_count = observed["event_count"]
        if not (type(event_count) is int and 0 <= event_count <= MAX_U64):
            _fail(f"{label}: ACCEPT event_count is not an unsigned 64-bit integer")
        if len(raw_hashes) != event_count:
            _fail(f"{label}: ACCEPT hash count differs from event_count")
        tip = observed["tip"]
        if event_count == 0:
            if tip != "0" * 64:
                _fail(f"{label}: empty ACCEPT tip is not the zero hash")
        elif type(tip) is not str or tip != raw_hashes[-1]:
            _fail(f"{label}: non-empty ACCEPT tip is not its final recomputed hash")
    else:
        rejection_class = observed["class"]
        if type(rejection_class) is not str or rejection_class not in REJECTION_CLASSES:
            _fail(f"{label}: REJECT class is not governed rejection text")
        if observed["event_count"] is not None or observed["tip"] is not None:
            _fail(f"{label}: REJECT count/tip are not null")
        if raw_hashes:
            _fail(f"{label}: REJECT recomputed-hash array is not empty")
    return observed


def _validate_observed_case(value: object, label: str) -> ObservedCase:
    if type(value) is not dict:
        _fail(f"{label}: case export is not an object")
    if set(value) != {"id", "result"}:
        _fail(f"{label}: case export fields differ: got {sorted(value)!r}")
    case_id = value.get("id")
    if type(case_id) is not str or not case_id:
        _fail(f"{label}: case export ID is not a non-empty string")
    result = _validate_result(value.get("result"), f"{label} {case_id}")
    return ObservedCase(case_id=case_id, result=result)


def _parse_go_output(stdout: bytes) -> tuple[ObservedCase, ...]:
    lines = stdout.splitlines()
    if len(lines) != EXPECTED_CASE_COUNT:
        _fail(
            "Go batch output has the wrong line count: "
            f"expected {EXPECTED_CASE_COUNT}, got {len(lines)}"
        )
    cases: list[ObservedCase] = []
    for position, line in enumerate(lines, start=1):
        if not line.strip():
            _fail(f"Go batch output line {position} is empty")
        value = _decode_json(line, f"Go batch output line {position}")
        cases.append(_validate_observed_case(value, f"Go batch output line {position}"))
    return tuple(cases)


def _parse_rust_output(
    stdout: bytes,
    manifest_digest: str,
) -> tuple[ObservedCase, ...]:
    value = _decode_json(stdout, "Rust exporter output")
    if type(value) is not dict:
        _fail("Rust exporter output is not an object")
    required = {"manifest_identity", "manifest_sha256", "results"}
    if set(value) != required:
        _fail(
            "Rust exporter envelope fields differ: "
            f"expected {sorted(required)!r}, got {sorted(value)!r}"
        )
    if value.get("manifest_identity") != MANIFEST_ID:
        _fail("Rust exporter manifest identity differs from the pinned identity")
    if value.get("manifest_sha256") != manifest_digest:
        _fail("Rust exporter manifest digest differs from the pinned digest")
    raw_results = value.get("results")
    if type(raw_results) is not list:
        _fail("Rust exporter results is not an array")
    if len(raw_results) != EXPECTED_CASE_COUNT:
        _fail(
            "Rust exporter has the wrong result count: "
            f"expected {EXPECTED_CASE_COUNT}, got {len(raw_results)}"
        )
    return tuple(
        _validate_observed_case(item, f"Rust exporter result {position}")
        for position, item in enumerate(raw_results, start=1)
    )


def _inventory_key(result: Mapping[str, object], label: str) -> str:
    verdict = result.get("verdict")
    if verdict == "ACCEPT":
        return "ACCEPT"
    if verdict == "REJECT" and result.get("class") in REJECTION_CLASSES:
        return str(result["class"])
    _fail(f"{label}: cannot inventory malformed result {result!r}")


def _inventory(cases: Sequence[ObservedCase], source: str) -> dict[str, int]:
    inventory = {key: 0 for key in INVENTORY_KEYS}
    for position, case in enumerate(cases, start=1):
        key = _inventory_key(case.result, f"{source} result {position} ({case.case_id})")
        inventory[key] += 1
    return inventory


def _run_python_actuals(
    cases: Sequence[python_checker.VerifiedCase], profile: str
) -> tuple[ObservedCase, ...]:
    observed: list[ObservedCase] = []
    for case in cases:
        try:
            outcome = portable_verifier.verify_complete_history(
                profile,
                case.external_key,
                case.input_bytes,
            )
            result = outcome.as_dict()
        except Exception as error:  # operational failures are not verdicts
            _fail(f"Python case {case.case_id}: verifier operational failure: {error}")
        observed.append(
            ObservedCase(
                case_id=case.case_id,
                result=_validate_result(result, f"Python case {case.case_id}"),
            )
        )
    return tuple(observed)


def _resolve_executable(executable: str | os.PathLike[str], root: Path) -> str:
    """Resolve explicit relative paths against the repository root.

    Bare names remain PATH lookups, so ``go`` and ``cargo`` work on Unix and
    Windows.  A path such as ``.scratch/toolchains/.../go.exe`` is resolved
    before changing cwd to the nested Go module.
    """

    text = os.fspath(executable)
    if not any(separator in text for separator in ("/", "\\")) and not text.startswith("."):
        return text
    path = Path(text)
    if not path.is_absolute():
        path = root / path
    return str(path.resolve())


def _run_process(
    command: Sequence[str],
    *,
    cwd: Path,
    label: str,
    environment: Mapping[str, str] | None = None,
) -> bytes:
    try:
        completed = subprocess.run(
            list(command),
            cwd=str(cwd),
            env=None if environment is None else dict(environment),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    except OSError as error:
        _fail(f"{label} could not be started: {error}")
    if completed.returncode != 0:
        diagnostics = bytes(completed.stderr or b"").decode("utf-8", "replace").strip()
        if not diagnostics:
            diagnostics = bytes(completed.stdout or b"").decode("utf-8", "replace").strip()
        suffix = f": {diagnostics}" if diagnostics else ""
        _fail(f"{label} failed with exit {completed.returncode}{suffix}")
    return bytes(completed.stdout or b"")


def _run_go_actuals(root: Path, manifest_path: Path, go_executable: str) -> tuple[ObservedCase, ...]:
    """Run the independent Go batch command from its nested module cwd."""

    command = [
        _resolve_executable(go_executable, root),
        "run",
        ".",
        "--manifest",
        str(manifest_path),
    ]
    stdout = _run_process(
        command,
        cwd=root / GO_MODULE_RELATIVE_PATH,
        label="Go portable verifier batch",
    )
    return _parse_go_output(stdout)


def _run_rust_actuals(
    root: Path,
    manifest_digest: str,
    cargo_executable: str,
) -> tuple[ObservedCase, ...]:
    """Run the test-only Rust exporter and parse its JSON envelope."""

    with tempfile.TemporaryDirectory(prefix="magpie-portable-differential-") as directory:
        output_path = Path(directory) / "rust-results.json"
        environment = os.environ.copy()
        environment[RUST_OUTPUT_ENVIRONMENT] = str(output_path)
        command = [
            _resolve_executable(cargo_executable, root),
            "test",
            "-p",
            "magpie-log",
            RUST_TEST_NAME,
            "--locked",
        ]
        _run_process(
            command,
            cwd=root,
            label="Rust portable verifier exporter",
            environment=environment,
        )
        try:
            stdout = output_path.read_bytes()
        except OSError as error:
            _fail(f"Rust exporter did not produce its result file: {error}")
    return _parse_rust_output(stdout, manifest_digest)


def _stable_value(value: object) -> str:
    return json.dumps(value, ensure_ascii=True, sort_keys=True, separators=(",", ":"))


def _compare_field_sets(
    mismatches: list[str],
    label: str,
    left: Mapping[str, object],
    right: Mapping[str, object],
    left_name: str,
    right_name: str,
) -> None:
    for field in GOVERNED_FIELDS:
        if left[field] != right[field]:
            mismatches.append(
                f"{label}: {field} differs: "
                f"{left_name}={_stable_value(left[field])} "
                f"{right_name}={_stable_value(right[field])}"
            )


def _compare_results(
    cases: Sequence[python_checker.VerifiedCase],
    source_reports: Sequence[SourceReport],
    expected_inventory: Mapping[str, int],
) -> tuple[str, ...]:
    expected_ids = tuple(case.case_id for case in cases)
    mismatches: list[str] = []

    for source in source_reports:
        if len(source.cases) != len(expected_ids):
            mismatches.append(
                f"{source.name}: case count differs: "
                f"expected {len(expected_ids)}, got {len(source.cases)}"
            )
        for position, expected_id in enumerate(expected_ids[: len(source.cases)]):
            observed_id = source.cases[position].case_id
            if observed_id != expected_id:
                mismatches.append(
                    f"{source.name}: case {position + 1} ID differs: "
                    f"expected {_stable_value(expected_id)}, got {_stable_value(observed_id)}"
                )

    # The frozen sequence is the reference for each tool, and the explicit
    # pairwise check makes a disagreement between tools visible even when one
    # tool happens to agree with the frozen expected result.
    for left_index, left in enumerate(source_reports):
        for right in source_reports[left_index + 1 :]:
            for position in range(min(len(left.cases), len(right.cases))):
                left_case = left.cases[position]
                right_case = right.cases[position]
                if left_case.case_id != right_case.case_id:
                    mismatches.append(
                        f"{left.name} vs {right.name}: case {position + 1} ID differs: "
                        f"{left.name}={_stable_value(left_case.case_id)} "
                        f"{right.name}={_stable_value(right_case.case_id)}"
                    )

    # Compare every governed field for a source only when its case ID is the
    # frozen case at that position.  This avoids producing misleading field
    # diffs for a shifted output while still reporting every valid aligned
    # result.  Pairwise fields are compared for aligned source IDs.
    for source in source_reports:
        for position, expected_case in enumerate(cases[: len(source.cases)]):
            observed = source.cases[position]
            if observed.case_id == expected_case.case_id:
                _compare_field_sets(
                    mismatches,
                    f"{source.name} case {expected_case.case_id} vs frozen expected",
                    observed.result,
                    expected_case.expected,
                    source.name,
                    "expected",
                )

    for left_index, left in enumerate(source_reports):
        for right in source_reports[left_index + 1 :]:
            for position in range(min(len(left.cases), len(right.cases))):
                left_case = left.cases[position]
                right_case = right.cases[position]
                if left_case.case_id == right_case.case_id:
                    _compare_field_sets(
                        mismatches,
                        f"{left.name} vs {right.name} case {left_case.case_id}",
                        left_case.result,
                        right_case.result,
                        left.name,
                        right.name,
                    )

    for source in source_reports:
        for key in INVENTORY_KEYS:
            if source.inventory[key] != expected_inventory[key]:
                mismatches.append(
                    f"inventory {source.name} {key} differs: "
                    f"expected {expected_inventory[key]}, got {source.inventory[key]}"
                )
    return tuple(mismatches)


def run_differential(
    root: Path | os.PathLike[str] | None = None,
    *,
    go_executable: str | os.PathLike[str] = "go",
    cargo_executable: str | os.PathLike[str] = "cargo",
) -> DifferentialReport:
    """Run Python, Go, and Rust actuals and compare the complete corpus."""

    repository_root = (ROOT if root is None else Path(root)).resolve()
    manifest, manifest_digest, cases = _load_frozen_cases(repository_root)
    for case in cases:
        _validate_result(case.expected, f"frozen expected case {case.case_id}")
    profile = _profile_identity(manifest)
    manifest_path = _manifest_path(repository_root)

    # Keep acquisition order explicit: Python is called directly, Go is run
    # in its isolated module, and Rust is the production conformer exporter.
    python_cases = _run_python_actuals(cases, profile)
    go_cases = _run_go_actuals(
        repository_root,
        manifest_path,
        os.fspath(go_executable),
    )
    rust_cases = _run_rust_actuals(
        repository_root,
        manifest_digest,
        os.fspath(cargo_executable),
    )

    source_reports = (
        SourceReport("rust", rust_cases, _inventory(rust_cases, "Rust")),
        SourceReport("python", python_cases, _inventory(python_cases, "Python")),
        SourceReport("go", go_cases, _inventory(go_cases, "Go")),
    )
    expected_inventory = python_checker._expected_inventory(manifest)
    mismatches = _compare_results(cases, source_reports, expected_inventory)
    return DifferentialReport(
        manifest_digest=manifest_digest,
        case_ids=tuple(case.case_id for case in cases),
        expected=tuple(case.expected for case in cases),
        expected_inventory=expected_inventory,
        sources=source_reports,
        mismatches=mismatches,
    )


def run_repeated(
    root: Path | os.PathLike[str] | None = None,
    *,
    repetitions: int = 1,
    go_executable: str | os.PathLike[str] = "go",
    cargo_executable: str | os.PathLike[str] = "cargo",
) -> tuple[DifferentialReport, bool]:
    """Run the complete harness repeatedly and report whether output is stable."""

    if repetitions < 1:
        _fail("repeat count must be at least 1")
    first = run_differential(
        root,
        go_executable=go_executable,
        cargo_executable=cargo_executable,
    )
    stable = True
    for number in range(2, repetitions + 1):
        current = run_differential(
            root,
            go_executable=go_executable,
            cargo_executable=cargo_executable,
        )
        if current != first:
            stable = False
            first = replace(
                first,
                mismatches=first.mismatches
                + (f"repeat run {number} differs from run 1",),
            )
            # One deterministic diagnostic is sufficient; subsequent runs
            # cannot improve the first observed mismatch explanation.
            break
    return first, stable


def format_report(report: DifferentialReport, *, repetitions: int = 1, stable: bool = True) -> str:
    """Format a stable human- and machine-readable report."""

    inventory: dict[str, dict[str, int]] = {
        "expected": {
            key: report.expected_inventory[key] for key in INVENTORY_KEYS
        }
    }
    # Source inventories are copied in a fixed order for deterministic
    # serialization.  The expected inventory was validated by the existing
    # Python checker during the preflight and is carried through unchanged.
    for source in SOURCE_NAMES:
        matching = next(item for item in report.sources if item.name == source)
        inventory[source] = {key: matching.inventory[key] for key in INVENTORY_KEYS}

    lines = [
        f"manifest: {MANIFEST_ID} sha256={report.manifest_digest}",
        f"cases: {report.case_count} (expected {EXPECTED_CASE_COUNT})",
        "inventory: "
        + json.dumps(inventory, ensure_ascii=True, separators=(",", ":")),
    ]
    if repetitions > 1:
        lines.append(f"repeat: {repetitions} ({'stable' if stable else 'unstable'})")
    if report.mismatches:
        lines.append(f"mismatches: {len(report.mismatches)}")
        lines.extend(f"- {message}" for message in report.mismatches)
    else:
        lines.append("result: PASS (Rust/Python/Go governed fields match frozen expected)")
    return "\n".join(lines)


def _argument_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--go",
        default="go",
        help="Go executable (default: go; relative paths resolve from the repository root)",
    )
    parser.add_argument(
        "--cargo",
        default="cargo",
        help="Cargo executable (default: cargo; relative paths resolve from the repository root)",
    )
    parser.add_argument(
        "--repeat",
        nargs="?",
        const=2,
        default=1,
        type=int,
        metavar="N",
        help="run the complete differential N times and require stable output (default: 1)",
    )
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = _argument_parser().parse_args(argv)
    try:
        report, stable = run_repeated(
            repetitions=args.repeat,
            go_executable=args.go,
            cargo_executable=args.cargo,
        )
    except DifferentialHarnessError as error:
        print(f"portable verifier differential failed: {error}", file=sys.stderr)
        return 2
    print(format_report(report, repetitions=args.repeat, stable=stable))
    if not stable:
        return 1
    return 0 if report.passed else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
