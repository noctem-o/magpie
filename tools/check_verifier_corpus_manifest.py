#!/usr/bin/env python3
"""Validate the candidate portable-verifier corpus manifest and byte bindings.

This checker validates corpus metadata, coverage inventories, and exact file
hashes. It deliberately does not implement the portable verifier or decide any
case's semantic result; the reviewed manifest is the candidate oracle.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = ROOT / "fixtures" / "verifier-language-v1" / "manifest.json"
CASES_PATH = MANIFEST_PATH.parent / "cases"
MANIFEST_ID = "magpie-portable-verifier-corpus-v1"
V_SIG = "magpie-ed25519-canonical-prime-subgroup-v1"
ZERO_HASH = "0" * 64
FAILURE_CLASSES = [
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
]
PAYLOAD_MEMBERS = {
    "Genesis": ["kind", "canonicalization_profile", "verifying_key"],
    "ClaimAsserted": ["kind", "claim_id", "statement", "status"],
    "EvidenceRecorded": ["kind", "claim_id", "summary"],
    "ClaimStatusChanged": ["kind", "claim_id", "from", "to", "reason"],
    "Note": ["kind", "text"],
    "SegmentAnchored": [
        "kind",
        "bundle_kind",
        "witness_root",
        "witness_algorithm",
        "canonicalization_profile",
        "run_id",
    ],
    "ClaimAssertedV2": [
        "kind",
        "claim_id",
        "statement",
        "scope_ref",
        "actor_class",
        "content_hash",
        "metadata_json",
    ],
    "EvidenceRegistered": [
        "kind",
        "evidence_id",
        "evidence_kind",
        "summary",
        "scope_ref",
        "actor_class",
        "content_hash",
        "metadata_json",
    ],
    "JustificationEdgeRecorded": [
        "kind",
        "edge_id",
        "edge_kind",
        "source_id",
        "target_id",
        "scope_ref",
        "actor_class",
        "rationale",
        "metadata_json",
    ],
}
VOCABULARIES = {
    "status": ["Open", "Conjectured", "Supported", "Settled", "Refuted"],
    "actor_class": [
        "HumanRoot",
        "AgentProposer",
        "AutomatedVerifier",
        "DeadboltAnchorer",
        "LensWitness",
        "SourceImporter",
    ],
    "evidence_kind": [
        "DeterministicVerification",
        "HumanRatification",
        "DeadboltAnchor",
        "ExecutionEvidence",
        "BehavioralEvaluation",
        "ExternalSource",
        "ModelSelfReport",
        "LensReadout",
    ],
    "edge_kind": [
        "supports",
        "derived_from",
        "contradicts",
        "supersedes",
        "invalidates",
        "ratifies",
    ],
    "payload_kind": list(PAYLOAD_MEMBERS),
}


def fail(message: str) -> None:
    raise RuntimeError(message)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def expected_schema_coordinates() -> set[str]:
    coordinates = {
        "SignedEvent.core",
        "SignedEvent.hash",
        "SignedEvent.signature",
        "EventCore.seq",
        "EventCore.timestamp_nanos",
        "EventCore.prev_hash",
        "EventCore.provenance",
        "EventCore.payload",
        "Provenance.agent",
        "Provenance.source",
    }
    for variant, members in PAYLOAD_MEMBERS.items():
        coordinates.update(f"Payload.{variant}.{member}" for member in members)
    return coordinates


def byte_properties(data: bytes) -> dict:
    try:
        data.decode("utf-8")
        valid_utf8 = True
    except UnicodeDecodeError:
        valid_utf8 = False
    return {
        "length": len(data),
        "lf_count": data.count(b"\n"),
        "cr_count": data.count(b"\r"),
        "starts_utf8_bom": data.startswith(b"\xef\xbb\xbf"),
        "valid_utf8": valid_utf8,
        "final_byte_hex": None if not data else f"{data[-1]:02x}",
    }


def load_manifest() -> dict:
    try:
        return json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot load manifest: {error}")


def validate_case(case: dict, seen_ids: set[str], seen_paths: set[str]) -> None:
    case_id = case.get("id")
    require(isinstance(case_id, str) and case_id, "case ID must be a non-empty string")
    require(case_id not in seen_ids, f"duplicate case ID: {case_id}")
    seen_ids.add(case_id)

    path_text = case.get("input_path")
    require(isinstance(path_text, str) and path_text, f"{case_id}: missing input_path")
    path = (ROOT / path_text).resolve()
    try:
        path.relative_to(ROOT.resolve())
    except ValueError:
        fail(f"{case_id}: input path escapes repository root")
    require(path.is_file(), f"{case_id}: input path does not exist: {path_text}")
    require(sha256(path) == case.get("input_sha256"), f"{case_id}: SHA-256 mismatch")
    require(byte_properties(path.read_bytes()) == case.get("byte_properties"), f"{case_id}: byte_properties mismatch")
    if path_text.startswith("fixtures/verifier-language-v1/cases/"):
        require(path_text not in seen_paths, f"{case_id}: generated case path reused: {path_text}")
        seen_paths.add(path_text)
    else:
        require(
            path_text
            in {
                "crates/magpie-log/testdata/golden-v1.jsonl",
                "fixtures/deadbolt-anchor-v1/anchor-log.jsonl",
            },
            f"{case_id}: unexpected out-of-subtree reference {path_text}",
        )

    require(isinstance(case.get("external_verifying_key_hex"), str), f"{case_id}: external key is not exact text")
    expected = case.get("expected")
    required_result_keys = {
        "verdict",
        "class",
        "line",
        "record_index",
        "event_count",
        "tip",
        "ordered_recomputed_hashes",
    }
    require(isinstance(expected, dict) and set(expected) == required_result_keys, f"{case_id}: incomplete or extra result fields")
    verdict = expected["verdict"]
    require(verdict in {"ACCEPT", "REJECT"}, f"{case_id}: third or missing verdict {verdict!r}")
    if verdict == "ACCEPT":
        require(expected["class"] is None and expected["line"] is None and expected["record_index"] is None, f"{case_id}: ACCEPT has rejection metadata")
        require(isinstance(expected["event_count"], int) and expected["event_count"] >= 0, f"{case_id}: invalid event_count")
        require(isinstance(expected["tip"], str) and len(expected["tip"]) == 64, f"{case_id}: invalid tip")
        hashes = expected["ordered_recomputed_hashes"]
        require(isinstance(hashes, list) and len(hashes) == expected["event_count"], f"{case_id}: ordered hash count mismatch")
        require(all(isinstance(value, str) and len(value) == 64 for value in hashes), f"{case_id}: malformed ordered hash")
        if expected["event_count"] == 0:
            require(expected["tip"] == ZERO_HASH and hashes == [], f"{case_id}: empty ACCEPT identity mismatch")
        else:
            require(expected["tip"] == hashes[-1], f"{case_id}: ACCEPT tip is not final ordered hash")
    else:
        require(expected["class"] in FAILURE_CLASSES, f"{case_id}: unknown failure class")
        require(expected["event_count"] is None and expected["tip"] is None and expected["ordered_recomputed_hashes"] == [], f"{case_id}: REJECT carries success summary")
        for coordinate in ("line", "record_index"):
            value = expected[coordinate]
            require(value is None or (isinstance(value, int) and value >= (1 if coordinate == "line" else 0)), f"{case_id}: invalid {coordinate}")
        if expected["class"] == "ExternalKey":
            require(expected["line"] is None and expected["record_index"] is None, f"{case_id}: ExternalKey has record coordinates")
        else:
            require(expected["line"] is not None, f"{case_id}: record rejection has no physical line")

    require(isinstance(case.get("owning_rules"), list) and case["owning_rules"], f"{case_id}: no owning rules")
    require(isinstance(case.get("rationale"), str) and case["rationale"], f"{case_id}: no rationale")
    require(isinstance(case.get("tags"), list) and case["tags"], f"{case_id}: no tags")


def validate_coverage(manifest: dict, by_id: dict[str, dict]) -> None:
    coverage = manifest["coverage"]
    obligations = coverage["obligation_case_ids"]
    required_obligations = (
        [f"N{number}" for number in range(1, 31)]
        + [f"K{number}" for number in list(range(1, 8)) + list(range(9, 16))]
        + [f"P{number}" for number in range(1, 13)]
        + [f"U64-P{number}" for number in range(1, 6)]
        + ["A21-S4", "A21-S5", "A21-D1", "A21-D2", "A21-T2"]
    )
    future = coverage["future_conformer_assertions"]
    for obligation in required_obligations:
        require(obligation in obligations or obligation in future, f"missing obligation coverage: {obligation}")
    for obligation, ids in obligations.items():
        require(isinstance(ids, list) and ids, f"{obligation}: empty case mapping")
        require(len(ids) == len(set(ids)), f"{obligation}: duplicate mapped case")
        require(all(case_id in by_id for case_id in ids), f"{obligation}: unknown case ID")
        for case_id in ids:
            require(obligation in by_id[case_id]["owning_rules"], f"{obligation}: mapping not reciprocated by {case_id}")

    schema = coverage["schema_required_members"]
    require(set(schema) == expected_schema_coordinates(), "schema required-member coordinate set mismatch")
    require(len(schema) == 57, "schema coordinate count must remain 57")
    used_mutations: set[str] = set()
    for coordinate, mapping in schema.items():
        require(set(mapping) == {"duplicate", "omission", "null"}, f"{coordinate}: incomplete mutation map")
        for mode, case_id in mapping.items():
            require(case_id not in used_mutations, f"schema mutation case reused: {case_id}")
            used_mutations.add(case_id)
            case = by_id[case_id]
            require(case["expected"]["verdict"] == "REJECT" and case["expected"]["class"] == "Schema", f"{coordinate}/{mode}: wrong result")
            expected_rule = {"duplicate": "N8", "omission": "N17", "null": "N18"}[mode]
            require(expected_rule in case["owning_rules"], f"{coordinate}/{mode}: wrong owning rule")

    expected_shapes = {"SignedEvent", "EventCore", "Provenance"} | {f"Payload.{variant}" for variant in PAYLOAD_MEMBERS}
    shapes = coverage["schema_unknown_member_shapes"]
    require(set(shapes) == expected_shapes and len(shapes) == 12, "N10 object-shape inventory mismatch")
    require(len(set(shapes.values())) == len(shapes), "N10 object shape reuses a case")
    for shape, case_id in shapes.items():
        case = by_id[case_id]
        require(case["expected"]["class"] == "Schema" and "N10" in case["owning_rules"], f"{shape}: invalid N10 mapping")

    escaped = coverage["escaped_duplicate_cases"]
    require(len(escaped) >= 3 and len(escaped) == len(set(escaped)), "escaped duplicate coverage is missing or ambiguous")
    for case_id in escaped:
        require(by_id[case_id]["expected"]["class"] == "Schema" and "N8" in by_id[case_id]["owning_rules"], f"{case_id}: invalid escaped duplicate result")

    numeric = coverage["numeric_boundaries"]
    require(set(numeric) == {f"U64-P{number}" for number in range(1, 6)}, "numeric boundary inventory mismatch")
    for obligation, case_id in numeric.items():
        require(case_id in by_id and obligation in by_id[case_id]["owning_rules"], f"{obligation}: invalid numeric mapping")

    vocabularies = coverage["positive_vocabularies"]
    require(set(vocabularies) == set(VOCABULARIES), "positive vocabulary family mismatch")
    for family, expected_values in VOCABULARIES.items():
        actual = vocabularies[family]
        require(set(actual) == set(expected_values), f"{family}: positive vocabulary mismatch")
        for value, ids in actual.items():
            require(isinstance(ids, list) and ids, f"{family}.{value}: no ACCEPT case")
            require(all(by_id[case_id]["expected"]["verdict"] == "ACCEPT" for case_id in ids), f"{family}.{value}: mapped to non-ACCEPT")


def validate_a21(manifest: dict, by_id: dict[str, dict]) -> None:
    vectors = manifest["a21_named_vectors"]
    require(set(vectors) == {"S4", "S5", "D1", "D2", "T2"}, "A-021 named-vector set mismatch")
    for name in ("S4", "S5", "D1", "T2"):
        vector = vectors[name]
        case = by_id[vector["case_id"]]
        expected = case["expected"]
        require(vector["expected_verdict"] == "REJECT" and vector["first_gate"] == "ExternalKey", f"A21-{name}: manifest classification mismatch")
        require(expected["verdict"] == "REJECT" and expected["class"] == "ExternalKey", f"A21-{name}: case result mismatch")
        require(expected["line"] is None and expected["record_index"] is None, f"A21-{name}: key rejection has coordinates")
    d2 = vectors["D2"]
    d2_case = by_id[d2["case_id"]]
    require(d2["expected_verdict"] == "ACCEPT" and d2_case["expected"]["verdict"] == "ACCEPT", "A21-D2 must be ACCEPT")
    require("identity-r" in d2_case["tags"] and "equation" in d2_case["tags"], "A21-D2 lacks construction tags")


def validate_exact_byte_families(by_id: dict[str, dict]) -> None:
    require(by_id["n30-final-crlf"]["byte_properties"]["cr_count"] == 1, "CRLF case lost CR")
    mixed = by_id["n30-mixed-crlf-lf"]["byte_properties"]
    require(mixed["cr_count"] == 1 and mixed["lf_count"] == 2, "mixed-EOL case lost exact separators")
    require(by_id["n30-no-final-terminator"]["byte_properties"]["final_byte_hex"] != "0a", "unterminated case gained LF")
    require(not by_id["n12-invalid-leading-ff"]["byte_properties"]["valid_utf8"], "invalid UTF-8 case was normalized")
    require(by_id["n13-bom-first"]["byte_properties"]["starts_utf8_bom"], "BOM case lost BOM")
    require(by_id["n7-k6-k14-empty-golden-key"]["byte_properties"]["length"] == 0, "empty case is not zero bytes")


def main() -> int:
    try:
        manifest = load_manifest()
        require(manifest.get("schema_version") == 1, "unexpected manifest schema version")
        require(manifest.get("manifest_identity") == MANIFEST_ID, "unexpected manifest identity")
        require(manifest.get("status") == "candidate-pending-owner-review-and-merge", "manifest is not explicitly candidate")
        context = manifest.get("producing_context", {})
        require(context.get("signature_profile_identity") == V_SIG, "V_sig is not fixed")
        require(context.get("complete_history_derivation_profile") is None, "manifest silently minted G_history")
        require("latest" not in manifest["identity_law"].lower() or "may not" in manifest["identity_law"].lower(), "manifest identity law permits latest")

        for source in manifest.get("governing_source_commitments", []):
            path = ROOT / source["path"]
            require(path.is_file() and sha256(path) == source["sha256"], f"governing source commitment mismatch: {source['path']}")

        fixture_commitments = manifest["existing_fixture_commitments"]
        require(sha256(ROOT / "crates/magpie-log/testdata/golden-v1.jsonl") == fixture_commitments["golden_v1_sha256"], "golden fixture changed")
        require(sha256(ROOT / "fixtures/deadbolt-anchor-v1/anchor-log.jsonl") == fixture_commitments["deadbolt_anchor_v1_sha256"], "Deadbolt fixture changed")

        cases = manifest.get("cases")
        require(isinstance(cases, list) and cases, "manifest has no cases")
        seen_ids: set[str] = set()
        seen_paths: set[str] = set()
        for case in cases:
            validate_case(case, seen_ids, seen_paths)
        by_id = {case["id"]: case for case in cases}
        actual_case_paths = {
            path.relative_to(ROOT).as_posix() for path in CASES_PATH.iterdir() if path.is_file()
        }
        require(actual_case_paths == seen_paths, "unreferenced or missing generated case file")

        accept_count = sum(case["expected"]["verdict"] == "ACCEPT" for case in cases)
        reject_count = len(cases) - accept_count
        class_counts = {
            name: sum(case["expected"]["class"] == name for case in cases)
            for name in FAILURE_CLASSES
        }
        summary = manifest["summary"]
        require(summary["case_count"] == len(cases), "summary case count mismatch")
        require(summary["accept_count"] == accept_count and summary["reject_count"] == reject_count, "summary verdict counts mismatch")
        require(summary["reject_counts_by_class"] == class_counts, "summary class counts mismatch")

        result_schema = manifest["result_schema"]
        require(result_schema["verdicts"] == ["ACCEPT", "REJECT"] and result_schema["no_third_semantic_result"] is True, "result vocabulary drift")
        require(result_schema["rejection_classes"] == FAILURE_CLASSES, "failure class vocabulary/order drift")
        require(result_schema["rejection_precedence"] == FAILURE_CLASSES[1:], "record-stage precedence drift")

        validate_coverage(manifest, by_id)
        validate_a21(manifest, by_id)
        validate_exact_byte_families(by_id)
    except (KeyError, OSError, TypeError, ValueError, RuntimeError) as error:
        print(f"portable verifier corpus check failed: {error}", file=sys.stderr)
        return 1

    print(
        "portable verifier corpus is internally consistent: "
        f"{len(cases)} cases ({accept_count} ACCEPT, {reject_count} REJECT)"
    )
    print(
        "schema inventory: 57 required members x duplicate/omission/null; "
        "12 unknown-member object shapes"
    )
    print("exact-byte hashes, A-021 named classifications, and positive vocabularies checked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
