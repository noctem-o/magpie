#!/usr/bin/env python3
"""Run ``tools/verify_chain.py`` over the frozen portable-verifier corpus.

The manifest and case files are exact-byte inputs.  This runner therefore
performs a complete byte-hash preflight before invoking the verifier, and then
compares only the governed result fields from each case.  It does not parse,
canonicalize, or otherwise verify a history itself, and it does not use Rust
output as an oracle.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
import sys
from typing import Mapping, Sequence


ROOT = Path(__file__).resolve().parents[1]
# Running ``python tools/<script>.py`` puts ``tools/`` (rather than the
# repository root) first on ``sys.path``.  Make the repository package
# discoverable before importing the conformer below.
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
MANIFEST_PATH = ROOT / "fixtures" / "verifier-language-v1" / "manifest.json"
MANIFEST_SHA_PATH = MANIFEST_PATH.with_name("manifest.sha256")
MANIFEST_ID = "magpie-portable-verifier-corpus-v1"
EXPECTED_MANIFEST_SHA256 = (
    "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81"
)
EXPECTED_CASE_COUNT = 432
SIGNATURE_PROFILE_FIELD = "signature_profile_identity"
GOVERNED_FIELDS = (
    "verdict",
    "class",
    "line",
    "record_index",
    "event_count",
    "tip",
    "ordered_recomputed_hashes",
)
REJECTION_CLASSES = (
    "ExternalKey",
    "Framing",
    "JsonSyntax",
    "Schema",
    "Sequence",
    "PreviousLink",
    "ContentHash",
    "Signature",
    "PayloadValidation",
    "Genesis",
)
INVENTORY_KEYS = ("ACCEPT", *REJECTION_CLASSES)
REQUIRED_CASE_FIELDS = {
    "id",
    "input_path",
    "input_sha256",
    "external_verifying_key_hex",
    "expected",
}


class HarnessError(RuntimeError):
    """A manifest, fixture, result-shape, or conformance failure."""


@dataclass(frozen=True)
class VerifiedCase:
    """A case whose exact input bytes passed the manifest hash preflight."""

    case_id: str
    input_path: str
    input_bytes: bytes
    external_key: str
    expected: Mapping[str, object]


@dataclass(frozen=True)
class CaseComparison:
    case_id: str
    actual: Mapping[str, object]
    expected: Mapping[str, object]


@dataclass(frozen=True)
class CorpusReport:
    """Deterministic output of one complete corpus run."""

    manifest_digest: str
    comparisons: tuple[CaseComparison, ...]
    inventory: Mapping[str, int]
    mismatches: tuple[str, ...]

    @property
    def case_count(self) -> int:
        return len(self.comparisons)

    @property
    def inventory_vector(self) -> tuple[int, ...]:
        return tuple(self.inventory[key] for key in INVENTORY_KEYS)

    @property
    def passed(self) -> bool:
        return not self.mismatches


def _fail(message: str) -> None:
    raise HarnessError(message)


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _require(condition: bool, message: str) -> None:
    if not condition:
        _fail(message)


def _repository_file(root: Path, path_text: str, label: str) -> Path:
    """Resolve a manifest path without permitting traversal or symlinks."""

    relative = Path(path_text)
    _require(
        not relative.is_absolute()
        and not relative.drive
        and bool(relative.parts)
        and ".." not in relative.parts,
        f"{label}: path is not a repository-relative descendant: {path_text!r}",
    )
    path = root.joinpath(relative)
    current = root
    for part in relative.parts:
        current = current / part
        _require(
            not current.is_symlink(),
            f"{label}: path traverses symlink: {path_text!r}",
        )
    _require(path.is_file(), f"{label}: path is not a regular file: {path_text!r}")
    return path


def _load_json(manifest_bytes: bytes) -> dict[str, object]:
    try:
        value = json.loads(manifest_bytes.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        _fail(f"manifest is not valid UTF-8 JSON: {error}")
    _require(type(value) is dict, "manifest must be a JSON object")
    return value


def load_verified_manifest(root: Path = ROOT) -> tuple[dict[str, object], str]:
    """Verify the frozen manifest bytes and return ``(manifest, digest)``.

    Hashing happens before decoding or inspecting the manifest.  The sidecar is
    checked as an independent human-facing commitment as well.
    """

    manifest_path = root / "fixtures" / "verifier-language-v1" / "manifest.json"
    sidecar_path = root / "fixtures" / "verifier-language-v1" / "manifest.sha256"
    try:
        manifest_bytes = manifest_path.read_bytes()
    except OSError as error:
        _fail(f"cannot read manifest: {error}")
    digest = _sha256(manifest_bytes)
    _require(
        digest == EXPECTED_MANIFEST_SHA256,
        "manifest bytes do not match the independently pinned v1 digest",
    )

    try:
        sidecar_lines = sidecar_path.read_bytes().decode("ascii").splitlines()
    except (OSError, UnicodeDecodeError) as error:
        _fail(f"cannot read manifest identity sidecar: {error}")
    _require(
        sidecar_lines
        == [
            f"# {MANIFEST_ID}",
            f"{EXPECTED_MANIFEST_SHA256}  manifest.json",
        ],
        "manifest identity sidecar does not match the pinned v1 commitment",
    )

    manifest = _load_json(manifest_bytes)
    _require(
        manifest.get("manifest_identity") == MANIFEST_ID,
        "unexpected manifest identity",
    )
    return manifest, digest


def _expected_case_result(case_id: str, value: object) -> dict[str, object]:
    _require(type(value) is dict, f"{case_id}: expected result is not an object")
    expected = value
    _require(
        set(expected) == set(GOVERNED_FIELDS),
        f"{case_id}: expected result fields are not exactly governed fields",
    )
    _require(
        expected["verdict"] in {"ACCEPT", "REJECT"},
        f"{case_id}: expected result has an invalid verdict",
    )
    return expected


def load_verified_cases(
    manifest: Mapping[str, object], root: Path = ROOT
) -> tuple[VerifiedCase, ...]:
    """Read and hash every case before the verifier is invoked for any case."""

    raw_cases = manifest.get("cases")
    _require(type(raw_cases) is list, "manifest cases must be a list")
    _require(
        len(raw_cases) == EXPECTED_CASE_COUNT,
        f"manifest must contain exactly {EXPECTED_CASE_COUNT} cases",
    )

    verified: list[VerifiedCase] = []
    seen_ids: set[str] = set()
    for position, value in enumerate(raw_cases):
        _require(type(value) is dict, f"case {position}: entry is not an object")
        case = value
        case_id = case.get("id")
        _require(
            type(case_id) is str and bool(case_id),
            f"case {position}: id is not a non-empty string",
        )
        _require(case_id not in seen_ids, f"duplicate case id: {case_id}")
        seen_ids.add(case_id)
        _require(
            REQUIRED_CASE_FIELDS.issubset(case),
            f"{case_id}: missing manifest case field",
        )

        input_path = case.get("input_path")
        _require(
            type(input_path) is str and bool(input_path),
            f"{case_id}: input_path is not a non-empty string",
        )
        path = _repository_file(root, input_path, f"{case_id} input")
        try:
            input_bytes = path.read_bytes()
        except OSError as error:
            _fail(f"{case_id}: cannot read input bytes: {error}")
        expected_hash = case.get("input_sha256")
        _require(
            type(expected_hash) is str
            and len(expected_hash) == 64
            and all(character in "0123456789abcdef" for character in expected_hash),
            f"{case_id}: input_sha256 is not lowercase SHA-256 text",
        )
        actual_hash = _sha256(input_bytes)
        _require(
            actual_hash == expected_hash,
            f"{case_id}: input SHA-256 mismatch: expected {expected_hash}, got {actual_hash}",
        )

        external_key = case.get("external_verifying_key_hex")
        _require(
            type(external_key) is str,
            f"{case_id}: external_verifying_key_hex is not text",
        )
        expected = _expected_case_result(case_id, case.get("expected"))
        verified.append(
            VerifiedCase(
                case_id=case_id,
                input_path=input_path,
                input_bytes=input_bytes,
                external_key=external_key,
                expected=expected,
            )
        )
    return tuple(verified)


def _compare_case(
    case_id: str, actual_value: object, expected: Mapping[str, object]
) -> tuple[CaseComparison, str | None]:
    if not isinstance(actual_value, Mapping):
        message = f"{case_id}: verifier returned {type(actual_value).__name__}, not a mapping"
        return CaseComparison(case_id, {}, expected), message

    actual = dict(actual_value)
    if set(actual) != set(GOVERNED_FIELDS):
        message = (
            f"{case_id}: verifier result fields differ: "
            f"expected {list(GOVERNED_FIELDS)!r}, got {sorted(actual)!r}"
        )
        return CaseComparison(case_id, actual, expected), message

    for field in GOVERNED_FIELDS:
        if actual[field] != expected[field]:
            return (
                CaseComparison(case_id, actual, expected),
                f"{case_id}: {field} differs: expected {expected[field]!r}, "
                f"got {actual[field]!r}",
            )
    return CaseComparison(case_id, actual, expected), None


def _inventory_key(result: Mapping[str, object], case_id: str) -> str:
    verdict = result.get("verdict")
    if verdict == "ACCEPT":
        return "ACCEPT"
    if verdict == "REJECT" and result.get("class") in REJECTION_CLASSES:
        return str(result["class"])
    _fail(f"{case_id}: cannot inventory malformed result {result!r}")


def _expected_inventory(manifest: Mapping[str, object]) -> dict[str, int]:
    summary = manifest.get("summary")
    _require(type(summary) is dict, "manifest summary is not an object")
    _require(
        summary.get("case_count") == EXPECTED_CASE_COUNT,
        "manifest summary case count is not 432",
    )
    expected = {"ACCEPT": summary.get("accept_count")}
    reject_counts = summary.get("reject_counts_by_class")
    _require(type(reject_counts) is dict, "manifest rejection inventory is not an object")
    expected.update({name: reject_counts.get(name) for name in REJECTION_CLASSES})
    for name, count in expected.items():
        _require(
            type(count) is int and not isinstance(count, bool) and count >= 0,
            f"manifest inventory count for {name} is invalid",
        )
    _require(
        sum(expected.values()) == EXPECTED_CASE_COUNT,
        "manifest inventory does not total 432 cases",
    )
    return expected


def run_corpus(root: Path = ROOT) -> CorpusReport:
    """Run all 432 cases after complete manifest/input-byte preflight."""

    manifest, manifest_digest = load_verified_manifest(root)
    cases = load_verified_cases(manifest, root)
    producing_context = manifest.get("producing_context")
    _require(type(producing_context) is dict, "manifest producing_context is not an object")
    profile_identity = producing_context.get(SIGNATURE_PROFILE_FIELD)
    _require(
        type(profile_identity) is str and bool(profile_identity),
        "manifest signature profile identity is not text",
    )

    # The import is deliberately the contract-required readable reference
    # conformer. It is
    # kept inside the run so loading/hash failures cannot be mistaken for a
    # semantic verifier result.
    try:
        from tools import verify_chain
    except ImportError as error:
        _fail(f"cannot import tools.verify_chain: {error}")

    comparisons: list[CaseComparison] = []
    mismatches: list[str] = []
    inventory = {key: 0 for key in INVENTORY_KEYS}
    for case in cases:
        try:
            outcome = verify_chain.verify_complete_history(
                profile_identity,
                case.external_key,
                case.input_bytes,
            )
            actual_value = outcome.as_dict()
        except Exception as error:  # operational failures are not governed verdicts
            _fail(f"{case.case_id}: verifier operational failure: {error}")
        comparison, mismatch = _compare_case(
            case.case_id, actual_value, case.expected
        )
        comparisons.append(comparison)
        if mismatch is not None:
            mismatches.append(mismatch)
        inventory[_inventory_key(comparison.actual, case.case_id)] += 1

    expected_inventory = _expected_inventory(manifest)
    for key in INVENTORY_KEYS:
        if inventory[key] != expected_inventory[key]:
            mismatches.append(
                f"inventory {key} differs: expected {expected_inventory[key]}, "
                f"got {inventory[key]}"
            )
    return CorpusReport(
        manifest_digest=manifest_digest,
        comparisons=tuple(comparisons),
        inventory=inventory,
        mismatches=tuple(mismatches),
    )


def format_report(report: CorpusReport) -> str:
    """Return stable human/machine-readable success or failure output."""

    lines = [
        f"manifest: {MANIFEST_ID} sha256={report.manifest_digest}",
        f"cases: {report.case_count} (expected {EXPECTED_CASE_COUNT})",
        "inventory: "
        + json.dumps(
            {key: report.inventory[key] for key in INVENTORY_KEYS},
            ensure_ascii=True,
            separators=(",", ":"),
        ),
    ]
    if report.mismatches:
        lines.append(f"mismatches: {len(report.mismatches)}")
        lines.extend(f"- {message}" for message in report.mismatches)
    else:
        lines.append("result: PASS (all governed fields match)")
    return "\n".join(lines)


def main(argv: Sequence[str] | None = None) -> int:
    del argv  # This runner intentionally has one frozen corpus and no selector.
    try:
        report = run_corpus()
    except HarnessError as error:
        print(f"portable Python verifier conformance failed: {error}", file=sys.stderr)
        return 2
    print(format_report(report))
    return 0 if report.passed else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
