#!/usr/bin/env python3
"""Verify the governed A-018 compile-fail witnesses with Rust JSON diagnostics.

The source snippets are compiled as an external consumer of ``magpie-claims``.
The committed inventory records the expected diagnostic and a hash of the
normalized snippet, so adding, removing, reordering, or changing a governed
witness fails before compiler results are considered. Only structured Rust
diagnostic codes are inspected; human-readable compiler wording is ignored.
"""

from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import dataclass
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
from typing import Any, Iterable, Mapping, Sequence


ROOT = Path(__file__).resolve().parents[1]
INVENTORY_PATH = ROOT / "tools/a018_diagnostics_inventory.json"
RUST_TOOLCHAIN = "1.98.1"

GOVERNED_SOURCES = (
    "crates/magpie-claims/src/origin_binding_verifier.rs",
    "crates/magpie-claims/src/origin_admission_audit.rs",
    "crates/magpie-claims/src/support_contribution_audit.rs",
)
EXPECTED_TOTAL = 43
EXPECTED_SOURCE_COUNTS = {
    "crates/magpie-claims/src/origin_binding_verifier.rs": 7,
    "crates/magpie-claims/src/origin_admission_audit.rs": 20,
    "crates/magpie-claims/src/support_contribution_audit.rs": 16,
}
EXPECTED_DIAGNOSTIC_COUNTS = {
    "E0451": 7,
    "E0277": 15,
    "E0308": 16,
    "E0599": 5,
}
EXPECTED_OCCURRENCE_COUNTS = {
    ("crates/magpie-claims/src/origin_admission_audit.rs", 18): 3,
}

RUSTDOC_INFO_WHITESPACE = r"[ \t\v\f]*"
RUSTDOC_INFO_TOKEN_SEPARATOR = re.compile(r"[ \t\v\f,]+")
FENCE_OPENING = re.compile(
    r"^(?P<indent> {0,3})"
    r"(?P<delimiter>(?P<delimiter_char>[`~])(?P=delimiter_char){2,})"
    r"(?P<raw_info_string>.*)$"
)
FENCE_CLOSING = re.compile(
    r"^(?P<indent> {0,3})"
    r"(?P<delimiter>(?P<delimiter_char>[`~])(?P=delimiter_char){2,})"
    r"[ \t]*$"
)
CANONICAL_COMPILE_FAIL_INFO = re.compile(
    RUSTDOC_INFO_WHITESPACE
    + r"compile_fail,(?P<code>E\d{4})"
    + RUSTDOC_INFO_WHITESPACE
    + r"$"
)
RUSTDOC_INFO_PREFIXES = frozenset(
    {
        "rust",
        "compile_fail",
        "no_run",
        "should_panic",
        "ignore",
        "edition2015",
        "edition2018",
        "edition2021",
        "edition2024",
        "test_harness",
        "standalone_crate",
        "no_crate_inject",
    }
)
RUST_ERROR_CODE = re.compile(r"^E\d{4}$")
SNIPPET_WRAPPER_PREFIX_LINES = 1


class A018CheckError(RuntimeError):
    """A deterministic failure in the inventory or compiler gate."""


@dataclass(frozen=True)
class ExtractedWitness:
    source: str
    ordinal: int
    diagnostic: str
    source_line: int
    snippet: str
    snippet_sha256: str

    @property
    def key(self) -> tuple[str, int]:
        return self.source, self.ordinal


@dataclass(frozen=True)
class FenceOpening:
    indent: int
    delimiter_char: str
    delimiter_length: int
    raw_info_string: str


@dataclass(frozen=True)
class ExpectedDiagnosticOccurrence:
    code: str
    relative_line: int | None = None
    relative_column: int | None = None


@dataclass(frozen=True)
class DiagnosticOccurrence:
    code: str
    file_name: str | None
    line_start: int | None
    column_start: int | None


@dataclass(frozen=True)
class InventoryWitness:
    witness_id: str
    source: str
    ordinal: int
    diagnostic: str
    snippet_sha256: str
    diagnostic_occurrences: tuple[ExpectedDiagnosticOccurrence, ...]

    @property
    def key(self) -> tuple[str, int]:
        return self.source, self.ordinal


def _doc_text(line: str, source: str, line_number: int) -> str:
    if not line.startswith("//!"):
        raise A018CheckError(
            f"{source}:{line_number}: expected a Rust module-doc line inside witness"
        )
    text = line[3:]
    return text[1:] if text.startswith(" ") else text


def normalize_snippet(lines: Iterable[str]) -> str:
    """Return stable source text after removing Rust module-doc markers."""

    return "\n".join(lines).rstrip() + "\n"


def snippet_sha256(snippet: str) -> str:
    return hashlib.sha256(snippet.encode("utf-8")).hexdigest()


def _remove_markdown_indent(text: str, indent: int) -> str:
    """Apply CommonMark's opening-fence indentation to a content line."""

    leading_spaces = len(text) - len(text.lstrip(" "))
    return text[min(indent, leading_spaces) :]


def _source_lines(text: str) -> list[str]:
    """Split Rust source lines without treating Rustdoc info whitespace as a break."""

    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    return [line[:-1] if line.endswith("\r") else line for line in lines]


def _parse_opening_fence(text: str) -> FenceOpening | None:
    match = FENCE_OPENING.fullmatch(text)
    if match is None:
        return None
    return FenceOpening(
        indent=len(match.group("indent")),
        delimiter_char=match.group("delimiter_char"),
        delimiter_length=len(match.group("delimiter")),
        raw_info_string=match.group("raw_info_string"),
    )


def _rustdoc_info_tokens(raw_info_string: str) -> tuple[str, ...]:
    return tuple(
        token
        for token in RUSTDOC_INFO_TOKEN_SEPARATOR.split(raw_info_string)
        if token
    )


def _is_rustdoc_compile_fail(raw_info_string: str) -> bool:
    """Recognize Rustdoc's compile-fail attribute without accepting its syntax."""

    tokens = _rustdoc_info_tokens(raw_info_string)
    return bool(tokens) and tokens[0] in RUSTDOC_INFO_PREFIXES and (
        "compile_fail" in tokens
    )


def _classify_compile_fail_fence(
    opening: FenceOpening, source: str, line_number: int
) -> str | None:
    """Return the governed code, or fail closed for Rustdoc compile-fail syntax."""

    if not _is_rustdoc_compile_fail(opening.raw_info_string):
        return None
    match = CANONICAL_COMPILE_FAIL_INFO.fullmatch(opening.raw_info_string)
    if match is None:
        raise A018CheckError(
            f"{source}:{line_number}: compile_fail fence must name a Rust error code "
            "using the canonical governed annotation compile_fail,E####"
        )
    return match.group("code")


def _is_closing_fence(text: str, opening: FenceOpening) -> bool:
    match = FENCE_CLOSING.fullmatch(text)
    return (
        match is not None
        and match.group("delimiter_char") == opening.delimiter_char
        and len(match.group("delimiter")) >= opening.delimiter_length
    )


def extract_witnesses(source: str, text: str) -> list[ExtractedWitness]:
    """Extract every explicitly coded compile-fail fence from one source file."""

    lines = _source_lines(text)
    witnesses: list[ExtractedWitness] = []
    index = 0
    while index < len(lines):
        line = lines[index]
        doc_line = _doc_text(line, source, index + 1) if line.startswith("//!") else line
        opening = _parse_opening_fence(doc_line)
        if opening is not None:
            diagnostic = _classify_compile_fail_fence(opening, source, index + 1)
            closing = index + 1
            body: list[str] = []
            while closing < len(lines):
                closing_doc_line = _doc_text(lines[closing], source, closing + 1)
                if _is_closing_fence(closing_doc_line, opening):
                    break
                body.append(
                    _remove_markdown_indent(closing_doc_line, opening.indent)
                )
                closing += 1
            if closing == len(lines):
                if diagnostic is not None:
                    raise A018CheckError(
                        f"{source}:{index + 1}: compile_fail fence has no closing fence"
                    )
                break
            if diagnostic is None:
                index = closing + 1
                continue
            snippet = normalize_snippet(body)
            witnesses.append(
                ExtractedWitness(
                    source=source,
                    ordinal=len(witnesses) + 1,
                    diagnostic=diagnostic,
                    source_line=index + 1,
                    snippet=snippet,
                    snippet_sha256=snippet_sha256(snippet),
                )
            )
            index = closing + 1
            continue
        index += 1
    return witnesses


def extract_governed_sources() -> list[ExtractedWitness]:
    witnesses: list[ExtractedWitness] = []
    for source in GOVERNED_SOURCES:
        path = ROOT / source
        if not path.is_file():
            raise A018CheckError(f"governed source is missing: {source}")
        witnesses.extend(extract_witnesses(source, path.read_text(encoding="utf-8")))
    return witnesses


def _counter(values: Iterable[str]) -> dict[str, int]:
    return dict(sorted(Counter(values).items()))


def load_inventory(path: Path = INVENTORY_PATH) -> tuple[dict[str, Any], list[InventoryWitness]]:
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise A018CheckError(f"cannot read inventory {path}: {exc}") from exc
    if not isinstance(document, dict):
        raise A018CheckError("A-018 inventory must be a JSON object")
    raw_witnesses = document.get("witnesses")
    if not isinstance(raw_witnesses, list):
        raise A018CheckError("A-018 inventory must contain a witnesses list")

    witnesses: list[InventoryWitness] = []
    for entry in raw_witnesses:
        if not isinstance(entry, dict):
            raise A018CheckError("A-018 inventory witness entries must be objects")
        try:
            diagnostic = str(entry["diagnostic"])
            raw_occurrences = entry.get("diagnostic_occurrences")
            if raw_occurrences is None:
                diagnostic_occurrences = (
                    ExpectedDiagnosticOccurrence(code=diagnostic),
                )
            else:
                if not isinstance(raw_occurrences, list) or not raw_occurrences:
                    raise A018CheckError(
                        "A-018 inventory diagnostic_occurrences must be a non-empty list"
                    )
                parsed_occurrences: list[ExpectedDiagnosticOccurrence] = []
                for occurrence in raw_occurrences:
                    if not isinstance(occurrence, dict):
                        raise A018CheckError(
                            "A-018 inventory diagnostic occurrences must be objects"
                        )
                    code = str(occurrence["code"])
                    relative_line = occurrence.get("relative_line")
                    relative_column = occurrence.get("relative_column")
                    if (relative_line is None) != (relative_column is None):
                        raise A018CheckError(
                            "A-018 inventory occurrence spans must include both "
                            "relative_line and relative_column"
                        )
                    if relative_line is not None and (
                        isinstance(relative_line, bool)
                        or not isinstance(relative_line, int)
                        or relative_line < 1
                        or isinstance(relative_column, bool)
                        or not isinstance(relative_column, int)
                        or relative_column < 1
                    ):
                        raise A018CheckError(
                            "A-018 inventory occurrence spans must use positive integers"
                        )
                    parsed_occurrences.append(
                        ExpectedDiagnosticOccurrence(
                            code=code,
                            relative_line=relative_line,
                            relative_column=relative_column,
                        )
                    )
                diagnostic_occurrences = tuple(parsed_occurrences)
            witnesses.append(
                InventoryWitness(
                    witness_id=str(entry["id"]),
                    source=str(entry["source"]),
                    ordinal=int(entry["ordinal"]),
                    diagnostic=diagnostic,
                    snippet_sha256=str(entry["snippet_sha256"]),
                    diagnostic_occurrences=diagnostic_occurrences,
                )
            )
        except (KeyError, TypeError, ValueError) as exc:
            raise A018CheckError(f"malformed A-018 inventory entry: {entry!r}") from exc
    return document, witnesses


def validate_inventory_contract(
    document: Mapping[str, Any], witnesses: Sequence[InventoryWitness]
) -> None:
    """Enforce the reviewed 43-witness inventory contract."""

    if document.get("schema_version") != 1:
        raise A018CheckError("A-018 inventory schema_version must be 1")
    if document.get("source_file_counts") != EXPECTED_SOURCE_COUNTS:
        raise A018CheckError(
            "A-018 inventory source counts changed: "
            f"expected {EXPECTED_SOURCE_COUNTS}, got {document.get('source_file_counts')}"
        )
    if document.get("diagnostic_counts") != EXPECTED_DIAGNOSTIC_COUNTS:
        raise A018CheckError(
            "A-018 inventory diagnostic counts changed: "
            f"expected {EXPECTED_DIAGNOSTIC_COUNTS}, got {document.get('diagnostic_counts')}"
        )
    if len(witnesses) != EXPECTED_TOTAL:
        raise A018CheckError(
            f"A-018 inventory witness count changed: expected {EXPECTED_TOTAL}, got {len(witnesses)}"
        )

    ids = [witness.witness_id for witness in witnesses]
    if len(set(ids)) != len(ids):
        raise A018CheckError("A-018 inventory contains duplicate witness IDs")
    if any(witness.source not in EXPECTED_SOURCE_COUNTS for witness in witnesses):
        raise A018CheckError("A-018 inventory contains an ungoverned source")

    source_counts = _counter(witness.source for witness in witnesses)
    diagnostic_counts = _counter(witness.diagnostic for witness in witnesses)
    if source_counts != EXPECTED_SOURCE_COUNTS:
        raise A018CheckError(
            f"A-018 inventory source counts disagree: expected {EXPECTED_SOURCE_COUNTS}, got {source_counts}"
        )
    if diagnostic_counts != EXPECTED_DIAGNOSTIC_COUNTS:
        raise A018CheckError(
            f"A-018 inventory diagnostic counts disagree: expected {EXPECTED_DIAGNOSTIC_COUNTS}, got {diagnostic_counts}"
        )

    for witness in witnesses:
        expected_occurrence_count = EXPECTED_OCCURRENCE_COUNTS.get(
            witness.key, 1
        )
        if len(witness.diagnostic_occurrences) != expected_occurrence_count:
            raise A018CheckError(
                f"{witness.witness_id}: expected {expected_occurrence_count} "
                "diagnostic occurrences, got "
                f"{len(witness.diagnostic_occurrences)}"
            )
        if any(
            occurrence.code != witness.diagnostic
            for occurrence in witness.diagnostic_occurrences
        ):
            raise A018CheckError(
                f"{witness.witness_id}: diagnostic occurrence codes must all be "
                f"{witness.diagnostic}"
            )
        if witness.key in EXPECTED_OCCURRENCE_COUNTS and any(
            occurrence.relative_line is None
            or occurrence.relative_column is None
            for occurrence in witness.diagnostic_occurrences
        ):
            raise A018CheckError(
                f"{witness.witness_id}: governed multi-call occurrences must "
                "include relative primary spans"
            )

    for source, expected_count in EXPECTED_SOURCE_COUNTS.items():
        ordinals = sorted(
            witness.ordinal for witness in witnesses if witness.source == source
        )
        expected_ordinals = list(range(1, expected_count + 1))
        if ordinals != expected_ordinals:
            raise A018CheckError(
                f"A-018 inventory ordinals for {source} changed: "
                f"expected {expected_ordinals}, got {ordinals}"
            )


def compare_source_inventory(
    extracted: Sequence[ExtractedWitness], inventory: Sequence[InventoryWitness]
) -> None:
    """Reject source disappearance, insertion, reordering, or snippet drift."""

    actual_by_key = {witness.key: witness for witness in extracted}
    expected_by_key = {witness.key: witness for witness in inventory}
    if set(actual_by_key) != set(expected_by_key):
        missing = sorted(set(expected_by_key) - set(actual_by_key))
        unexpected = sorted(set(actual_by_key) - set(expected_by_key))
        raise A018CheckError(
            f"A-018 governed witness inventory changed: missing={missing}, unexpected={unexpected}"
        )

    mismatches: list[str] = []
    for key, expected in expected_by_key.items():
        actual = actual_by_key[key]
        if actual.diagnostic != expected.diagnostic:
            mismatches.append(
                f"{expected.witness_id}: expected {expected.diagnostic}, observed fence {actual.diagnostic}"
            )
        if actual.snippet_sha256 != expected.snippet_sha256:
            mismatches.append(
                f"{expected.witness_id}: snippet hash changed "
                f"(expected {expected.snippet_sha256}, observed {actual.snippet_sha256})"
            )
    if mismatches:
        raise A018CheckError("A-018 governed witness drift:\n" + "\n".join(mismatches))


def _primary_span(payload: Mapping[str, Any]) -> Mapping[str, Any] | None:
    spans = payload.get("spans")
    if not isinstance(spans, list):
        return None
    for span in spans:
        if isinstance(span, dict) and span.get("is_primary") is True:
            return span
    return None


def parse_diagnostic_occurrences(output: str) -> tuple[DiagnosticOccurrence, ...]:
    """Parse top-level coded compiler errors without collapsing occurrences."""

    occurrences: list[DiagnosticOccurrence] = []
    for line_number, line in enumerate(output.splitlines(), start=1):
        if not line.strip():
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError as exc:
            raise A018CheckError(
                f"rustc emitted non-JSON diagnostic output on line {line_number}: {line[:160]!r}"
            ) from exc
        if not isinstance(payload, dict):
            continue
        if payload.get("level") != "error":
            continue
        code = payload.get("code")
        if (
            isinstance(code, dict)
            and isinstance(code.get("code"), str)
            and RUST_ERROR_CODE.fullmatch(code["code"])
        ):
            span = _primary_span(payload)
            occurrences.append(
                DiagnosticOccurrence(
                    code=code["code"],
                    file_name=(
                        span.get("file_name")
                        if span is not None and isinstance(span.get("file_name"), str)
                        else None
                    ),
                    line_start=(
                        span.get("line_start")
                        if span is not None and isinstance(span.get("line_start"), int)
                        else None
                    ),
                    column_start=(
                        span.get("column_start")
                        if span is not None
                        and isinstance(span.get("column_start"), int)
                        else None
                    ),
                )
            )
    return tuple(occurrences)


def _relative_primary_position(
    occurrence: DiagnosticOccurrence, source_path: Path
) -> tuple[int, int] | None:
    if occurrence.file_name is None:
        return None
    try:
        if Path(occurrence.file_name).resolve() != source_path.resolve():
            return None
    except OSError:
        return None
    if occurrence.line_start is None or occurrence.column_start is None:
        return None
    return (
        occurrence.line_start - SNIPPET_WRAPPER_PREFIX_LINES,
        occurrence.column_start,
    )


def validate_compilation_result(
    witness_id: str,
    expected: str,
    returncode: int,
    diagnostic_output: str,
    expected_occurrences: Sequence[ExpectedDiagnosticOccurrence] | None = None,
    source_path: Path | None = None,
) -> tuple[DiagnosticOccurrence, ...]:
    """Require exact top-level coded errors and, when supplied, their spans."""

    observed = parse_diagnostic_occurrences(diagnostic_output)
    if returncode == 0:
        raise A018CheckError(
            f"{witness_id}: unexpectedly compiled successfully; expected {expected}"
        )
    expected_occurrences = tuple(
        expected_occurrences or (ExpectedDiagnosticOccurrence(code=expected),)
    )
    expected_codes = Counter(occurrence.code for occurrence in expected_occurrences)
    observed_codes = Counter(occurrence.code for occurrence in observed)
    if observed_codes != expected_codes:
        raise A018CheckError(
            f"{witness_id}: expected diagnostic occurrences "
            f"{dict(sorted(expected_codes.items()))}, observed "
            f"{dict(sorted(observed_codes.items()))}"
        )

    remaining = list(observed)
    for expected_occurrence in expected_occurrences:
        matching_index: int | None = None
        for index, occurrence in enumerate(remaining):
            if occurrence.code != expected_occurrence.code:
                continue
            if (
                expected_occurrence.relative_line is not None
                or expected_occurrence.relative_column is not None
            ):
                if source_path is None:
                    raise A018CheckError(
                        f"{witness_id}: relative diagnostic spans require the "
                        "generated source path"
                    )
                if _relative_primary_position(occurrence, source_path) != (
                    expected_occurrence.relative_line,
                    expected_occurrence.relative_column,
                ):
                    continue
            matching_index = index
            break
        if matching_index is None:
            expected_position = (
                f" at relative line {expected_occurrence.relative_line}, "
                f"column {expected_occurrence.relative_column}"
                if expected_occurrence.relative_line is not None
                else ""
            )
            raise A018CheckError(
                f"{witness_id}: missing diagnostic occurrence "
                f"{expected_occurrence.code}{expected_position}"
            )
        remaining.pop(matching_index)
    return observed


def _cargo_claims_command(target_directory: Path) -> list[str]:
    return [
        "cargo",
        f"+{RUST_TOOLCHAIN}",
        "build",
        "-p",
        "magpie-claims",
        "--lib",
        "--locked",
        "--target-dir",
        str(target_directory),
        "--message-format=json-render-diagnostics",
    ]


def _cargo_claims_rlib(target_directory: Path) -> Path:
    command = _cargo_claims_command(target_directory)
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise A018CheckError(
            "magpie-claims build failed:\n" + (result.stderr or result.stdout).strip()
        )

    rlibs: list[Path] = []
    for line in result.stdout.splitlines():
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        if (
            isinstance(payload, dict)
            and payload.get("reason") == "compiler-artifact"
            and isinstance(payload.get("target"), dict)
            and payload["target"].get("name", "").replace("_", "-")
            == "magpie-claims"
        ):
            rlibs.extend(
                Path(filename)
                for filename in payload.get("filenames", [])
                if isinstance(filename, str) and filename.endswith(".rlib")
            )
    if len(rlibs) != 1:
        raise A018CheckError(
            "expected one magpie-claims rlib from cargo, got "
            + repr([str(path) for path in rlibs])
        )
    return rlibs[0]


def _single_dependency_rlib(dependency_directory: Path, crate_name: str) -> Path:
    candidates = sorted(dependency_directory.glob(f"lib{crate_name}-*.rlib"))
    if len(candidates) != 1:
        raise A018CheckError(
            f"expected one {crate_name} rlib, got {[str(path) for path in candidates]}"
        )
    return candidates[0]


def _compile_external_witness(
    witness: ExtractedWitness,
    inventory: InventoryWitness,
    claims_rlib: Path,
    dependency_directory: Path,
    source_directory: Path,
) -> tuple[DiagnosticOccurrence, ...]:
    source_path = source_directory / f"{inventory.witness_id}.rs"
    source_path.write_text(
        "fn main() {\n" + witness.snippet + "}\n", encoding="utf-8"
    )
    command = [
        "rustc",
        f"+{RUST_TOOLCHAIN}",
        "--edition=2021",
        "--crate-name",
        inventory.witness_id.replace("-", "_"),
        "--crate-type",
        "bin",
        "--emit=metadata",
        "--error-format=json",
        "--extern",
        f"magpie_claims={claims_rlib}",
        "--extern",
        f"serde_json={_single_dependency_rlib(dependency_directory, 'serde_json')}",
        "-L",
        f"dependency={dependency_directory}",
        str(source_path),
    ]
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    try:
        return validate_compilation_result(
            inventory.witness_id,
            inventory.diagnostic,
            result.returncode,
            result.stderr,
            inventory.diagnostic_occurrences,
            source_path,
        )
    except A018CheckError as exc:
        raise A018CheckError(
            f"{witness.source}:{witness.source_line}: {exc}"
        ) from exc


def run_check() -> None:
    document, inventory = load_inventory()
    validate_inventory_contract(document, inventory)
    extracted = extract_governed_sources()
    compare_source_inventory(extracted, inventory)

    with tempfile.TemporaryDirectory(prefix="magpie-a018-") as temporary_directory:
        temporary_root = Path(temporary_directory)
        target_directory = temporary_root / "target"
        source_directory = temporary_root / "sources"
        source_directory.mkdir()
        claims_rlib = _cargo_claims_rlib(target_directory)
        dependency_directory = target_directory / "debug" / "deps"
        observed_counts: Counter[str] = Counter()
        inventory_by_key = {expected.key: expected for expected in inventory}
        for witness in extracted:
            expected = inventory_by_key[witness.key]
            observed_counts.update(
                occurrence.code
                for occurrence in _compile_external_witness(
                    witness,
                    expected,
                    claims_rlib,
                    dependency_directory,
                    source_directory,
                )
            )

    print(
        "A-018 exact diagnostics passed: "
        f"{len(extracted)} witnesses; "
        f"files={_counter(witness.source for witness in extracted)}; "
        f"witness_codes={_counter(witness.diagnostic for witness in extracted)}; "
        f"occurrence_codes={dict(sorted(observed_counts.items()))}; "
        f"toolchain=rustc {RUST_TOOLCHAIN}; external_consumer=true; "
        "coded_error_set=exact"
    )


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.parse_args(argv)
    try:
        run_check()
    except A018CheckError as exc:
        print(f"A-018 exact diagnostics FAILED: {exc}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
