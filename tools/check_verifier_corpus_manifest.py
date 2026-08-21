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
MANIFEST_DIGEST_PATH = MANIFEST_PATH.with_name("manifest.sha256")
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
FRAMING_WITHOUT_RECORD_INDEX_CASES = {
    "n30-extra-final-terminator",
    "n6-interior-zero-length",
    "n7-lf-only-empty-record",
}
REQUIRED_GOVERNING_SOURCE_PATHS = {
    "docs/FORMAT.md",
    "docs/adr/0008-complete-producing-coordinates.md",
    "docs/adr/0009-verified-supplied-history-and-explicit-checkpoint-expectations.md",
    "docs/adr/0010-ed25519-verification-profile-and-external-trust-root-admissibility.md",
    "docs/design/portable-verifier-input-language-contract.md",
}
FUTURE_ONLY_OBLIGATIONS = {"P3", "P4", "P5", "P6", "P10", "P11"}
HEX_MUTATIONS = (
    "short",
    "odd",
    "nonhex",
    "empty",
    "whitespace",
    "prefix",
    "unicode-lookalike",
)
HEX_ROLE_CASES = {
    role: [f"n3-{role}-{mutation}" for mutation in HEX_MUTATIONS]
    for role in (
        "prev-hash",
        "stored-hash",
        "signature",
        "genesis-key",
        "witness-root",
        "claim-content-hash",
        "evidence-content-hash",
    )
}
HEX_ROLE_FAILURE_CLASS = {
    "prev-hash": "Schema",
    "stored-hash": "Schema",
    "signature": "Schema",
    "genesis-key": "Genesis",
    "witness-root": "PayloadValidation",
    "claim-content-hash": "PayloadValidation",
    "evidence-content-hash": "PayloadValidation",
}
SIGNATURE_BOUNDARIES = {
    "R-nondecompress": ("a21-r-nondecompress", "A21-R1", "r-encoding"),
    "R-noncanonical": ("a21-r-y-equals-p", "A21-R2", "r-encoding"),
    "R-x0-sign1": ("a21-r-x-zero-sign-one", "A21-R3", "r-encoding"),
    "R-identity-false": (
        "a21-r-identity-equation-false",
        "A21-R-IDENTITY-FALSE",
        "identity-r",
    ),
    "R-small-order": (
        "a21-r-nonidentity-small-order",
        "A21-R-SMALL",
        "small-order-r",
    ),
    "R-mixed": ("a21-r-mixed-torsion-cofactor", "A21-R-MIXED", "mixed-torsion-r"),
    "R-order4": ("a21-r-order-four-cofactor", "A21-C1", "small-order-r"),
    "S-zero": ("a21-r-identity-equation-false", "A21-S3", "identity-r"),
    "S-L-minus-one": (
        "a21-s-l-minus-one-equation-false",
        "A21-S-L-1",
        "scalar-boundary",
    ),
    "S-L": ("a21-s-equals-l", "A21-S1", "scalar-boundary"),
    "S-L-plus-one": ("a21-s-l-plus-one", "A21-S2", "scalar-boundary"),
    "S-max": ("a21-s-all-ff", "A21-S2", "scalar-boundary"),
    "altered-signature": ("a21-altered-signature", "A21-ALTERED-SIGNATURE", "equation"),
    "altered-message": ("a21-altered-message", "A21-M1", "message"),
    "A21-D2": ("a21-d2-identity-r-equation-true", "A21-D2", "identity-r"),
}
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


def is_exact_lower_hex(value: object, length: int) -> bool:
    return (
        isinstance(value, str)
        and len(value) == length
        and all(character in "0123456789abcdef" for character in value)
    )


def is_coordinate(value: object, minimum: int) -> bool:
    return isinstance(value, int) and not isinstance(value, bool) and value >= minimum


def load_json_without_duplicate_members(text: str, source: str) -> object:
    def reject_duplicate_members(pairs: list[tuple[str, object]]) -> dict:
        result: dict = {}
        for name, value in pairs:
            require(name not in result, f"{source}: duplicate JSON object member {name!r}")
            result[name] = value
        return result

    try:
        return json.loads(text, object_pairs_hook=reject_duplicate_members)
    except json.JSONDecodeError as error:
        fail(f"cannot load {source}: {error}")


def validate_manifest_identity_commitment() -> str:
    manifest_digest = sha256(MANIFEST_PATH)
    try:
        commitment_lines = MANIFEST_DIGEST_PATH.read_text(encoding="ascii").splitlines()
    except (OSError, UnicodeError) as error:
        fail(f"cannot load manifest identity commitment: {error}")
    require(
        commitment_lines
        == [f"# {MANIFEST_ID}", f"{manifest_digest}  {MANIFEST_PATH.name}"],
        "manifest identity commitment does not match exact manifest bytes",
    )
    return manifest_digest


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
        text = MANIFEST_PATH.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot load manifest: {error}")
    manifest = load_json_without_duplicate_members(text, "manifest")
    require(isinstance(manifest, dict), "manifest must be a JSON object")
    return manifest


def accepted_case_vocabulary(case: dict) -> dict[str, set[str]]:
    case_id = case["id"]
    path = ROOT / case["input_path"]
    data = path.read_bytes()
    if not data:
        record_bytes: list[bytes] = []
    else:
        record_bytes = data.split(b"\n")
        if record_bytes[-1] == b"":
            record_bytes.pop()

    values = {family: set() for family in VOCABULARIES}
    for record_index, encoded_record in enumerate(record_bytes):
        if encoded_record.endswith(b"\r"):
            encoded_record = encoded_record[:-1]
        require(encoded_record, f"{case_id}: ACCEPT vocabulary input has an empty record")
        try:
            text = encoded_record.decode("utf-8")
        except UnicodeDecodeError as error:
            fail(f"{case_id}: cannot decode ACCEPT record {record_index}: {error}")
        record = load_json_without_duplicate_members(
            text, f"{case_id} record {record_index}"
        )
        require(isinstance(record, dict), f"{case_id}: ACCEPT record is not an object")
        core = record.get("core")
        require(isinstance(core, dict), f"{case_id}: ACCEPT record has no core object")
        payload = core.get("payload")
        require(
            isinstance(payload, dict),
            f"{case_id}: ACCEPT record has no payload object",
        )
        kind = payload.get("kind")
        if isinstance(kind, str):
            values["payload_kind"].add(kind)
        if kind == "ClaimAsserted" and isinstance(payload.get("status"), str):
            values["status"].add(payload["status"])
        if kind == "ClaimStatusChanged":
            for field in ("from", "to"):
                if isinstance(payload.get(field), str):
                    values["status"].add(payload[field])
        if kind in {
            "ClaimAssertedV2",
            "EvidenceRegistered",
            "JustificationEdgeRecorded",
        } and isinstance(payload.get("actor_class"), str):
            values["actor_class"].add(payload["actor_class"])
        if kind == "EvidenceRegistered" and isinstance(
            payload.get("evidence_kind"), str
        ):
            values["evidence_kind"].add(payload["evidence_kind"])
        if kind == "JustificationEdgeRecorded" and isinstance(
            payload.get("edge_kind"), str
        ):
            values["edge_kind"].add(payload["edge_kind"])
    return values


def validate_governing_source_commitments(manifest: dict) -> None:
    sources = manifest.get("governing_source_commitments")
    require(isinstance(sources, list), "governing_source_commitments must be a list")
    paths: list[str] = []
    for index, source in enumerate(sources):
        require(
            isinstance(source, dict) and set(source) == {"path", "sha256"},
            f"governing source commitment {index} has an invalid shape",
        )
        path_text = source["path"]
        expected_hash = source["sha256"]
        require(isinstance(path_text, str) and path_text, f"governing source commitment {index} has no path")
        require(
            isinstance(expected_hash, str)
            and len(expected_hash) == 64
            and all(character in "0123456789abcdef" for character in expected_hash),
            f"governing source commitment {path_text} has an invalid SHA-256",
        )
        paths.append(path_text)
    require(len(paths) == len(set(paths)), "duplicate governing source commitment path")
    require(set(paths) == REQUIRED_GOVERNING_SOURCE_PATHS, "governing source commitment path set mismatch")
    for source in sources:
        path = ROOT / source["path"]
        require(
            path.is_file() and sha256(path) == source["sha256"],
            f"governing source commitment mismatch: {source['path']}",
        )


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

    external_key = case.get("external_verifying_key_hex")
    require(isinstance(external_key, str), f"{case_id}: external key is not exact text")
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
    if verdict == "ACCEPT" or expected["class"] != "ExternalKey":
        require(
            is_exact_lower_hex(external_key, 64),
            f"{case_id}: case expected past ExternalKey lacks lowercase hex64 key text",
        )
    if verdict == "ACCEPT":
        require(expected["class"] is None and expected["line"] is None and expected["record_index"] is None, f"{case_id}: ACCEPT has rejection metadata")
        require(isinstance(expected["event_count"], int) and expected["event_count"] >= 0, f"{case_id}: invalid event_count")
        require(is_exact_lower_hex(expected["tip"], 64), f"{case_id}: invalid tip")
        hashes = expected["ordered_recomputed_hashes"]
        require(isinstance(hashes, list) and len(hashes) == expected["event_count"], f"{case_id}: ordered hash count mismatch")
        require(all(is_exact_lower_hex(value, 64) for value in hashes), f"{case_id}: malformed ordered hash")
        if expected["event_count"] == 0:
            require(expected["tip"] == ZERO_HASH and hashes == [], f"{case_id}: empty ACCEPT identity mismatch")
        else:
            require(expected["tip"] == hashes[-1], f"{case_id}: ACCEPT tip is not final ordered hash")
    else:
        require(expected["class"] in FAILURE_CLASSES, f"{case_id}: unknown failure class")
        require(expected["event_count"] is None and expected["tip"] is None and expected["ordered_recomputed_hashes"] == [], f"{case_id}: REJECT carries success summary")
        if expected["class"] == "ExternalKey":
            require(expected["line"] is None and expected["record_index"] is None, f"{case_id}: ExternalKey has record coordinates")
        elif expected["class"] == "Framing":
            require(is_coordinate(expected["line"], 1), f"{case_id}: invalid line")
            if case_id in FRAMING_WITHOUT_RECORD_INDEX_CASES:
                require(
                    expected["record_index"] is None,
                    f"{case_id}: zero-length Framing slice has a record index",
                )
            else:
                require(
                    is_coordinate(expected["record_index"], 0),
                    f"{case_id}: candidate-bound Framing failure has no record index",
                )
        else:
            require(is_coordinate(expected["line"], 1), f"{case_id}: invalid line")
            require(
                is_coordinate(expected["record_index"], 0),
                f"{case_id}: candidate-bound rejection has no record index",
            )

    require(isinstance(case.get("owning_rules"), list) and case["owning_rules"], f"{case_id}: no owning rules")
    require(isinstance(case.get("rationale"), str) and case["rationale"], f"{case_id}: no rationale")
    require(isinstance(case.get("tags"), list) and case["tags"], f"{case_id}: no tags")


def validate_coverage(manifest: dict, by_id: dict[str, dict]) -> None:
    coverage = manifest["coverage"]
    obligations = coverage["obligation_case_ids"]
    required_obligations = set(
        [f"N{number}" for number in range(1, 31)]
        + [f"K{number}" for number in list(range(1, 8)) + list(range(9, 16))]
        + [f"P{number}" for number in range(1, 13)]
        + [f"U64-P{number}" for number in range(1, 6)]
        + ["A21-S4", "A21-S5", "A21-D1", "A21-D2", "A21-T2"]
    )
    future = coverage["future_conformer_assertions"]
    require(isinstance(obligations, dict), "obligation_case_ids must be an object")
    require(isinstance(future, dict), "future_conformer_assertions must be an object")
    require(set(future) == FUTURE_ONLY_OBLIGATIONS, "future-conformer obligation set mismatch")
    require(
        all(isinstance(description, str) and description for description in future.values()),
        "future-conformer assertion must have a non-empty description",
    )
    require(not (set(obligations) & set(future)), "obligation appears in both static and future coverage")
    for obligation in required_obligations - FUTURE_ONLY_OBLIGATIONS:
        require(obligation in obligations, f"missing static obligation coverage: {obligation}")
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
    vocabulary_by_case: dict[str, dict[str, set[str]]] = {}
    for family, expected_values in VOCABULARIES.items():
        actual = vocabularies[family]
        require(set(actual) == set(expected_values), f"{family}: positive vocabulary mismatch")
        for value, ids in actual.items():
            require(isinstance(ids, list) and ids, f"{family}.{value}: no ACCEPT case")
            for case_id in ids:
                require(case_id in by_id, f"{family}.{value}: unknown case ID {case_id}")
                case = by_id[case_id]
                require(
                    case["expected"]["verdict"] == "ACCEPT",
                    f"{family}.{value}: mapped to non-ACCEPT",
                )
                if case_id not in vocabulary_by_case:
                    vocabulary_by_case[case_id] = accepted_case_vocabulary(case)
                require(
                    value in vocabulary_by_case[case_id][family],
                    f"{family}.{value}: absent from mapped ACCEPT case {case_id}",
                )

    hex_roles = coverage["hex_role_cases"]
    require(isinstance(hex_roles, dict), "hex_role_cases must be an object")
    require(set(hex_roles) == set(HEX_ROLE_CASES), "hex-role inventory mismatch")
    used_hex_cases: set[str] = set()
    for role, expected_ids in HEX_ROLE_CASES.items():
        actual_ids = hex_roles[role]
        require(actual_ids == expected_ids, f"{role}: hex-role case mapping mismatch")
        for mutation, case_id in zip(HEX_MUTATIONS, actual_ids, strict=True):
            require(case_id not in used_hex_cases, f"hex-role case reused: {case_id}")
            used_hex_cases.add(case_id)
            require(case_id in by_id, f"{role}/{mutation}: unknown case ID")
            case = by_id[case_id]
            expected_class = HEX_ROLE_FAILURE_CLASS[role]
            if mutation == "empty" and role in {"claim-content-hash", "evidence-content-hash"}:
                expected_class = "Genesis"
            require(
                case["expected"]["verdict"] == "REJECT"
                and case["expected"]["class"] == expected_class,
                f"{role}/{mutation}: wrong result",
            )
            require("N3" in case["owning_rules"], f"{role}/{mutation}: wrong owning rule")
            require(
                {"hex", role, mutation}.issubset(case["tags"]),
                f"{role}/{mutation}: incompatible case tags",
            )

    signature_boundaries = coverage["signature_boundaries"]
    require(isinstance(signature_boundaries, dict), "signature_boundaries must be an object")
    require(set(signature_boundaries) == set(SIGNATURE_BOUNDARIES), "signature-boundary inventory mismatch")
    for boundary, (expected_id, expected_rule, expected_tag) in SIGNATURE_BOUNDARIES.items():
        case_id = signature_boundaries[boundary]
        require(case_id == expected_id, f"{boundary}: signature-boundary case mapping mismatch")
        require(case_id in by_id, f"{boundary}: unknown case ID")
        case = by_id[case_id]
        expected_verdict = "ACCEPT" if boundary == "A21-D2" else "REJECT"
        expected_class = None if boundary == "A21-D2" else "Signature"
        require(
            case["expected"]["verdict"] == expected_verdict
            and case["expected"]["class"] == expected_class,
            f"{boundary}: wrong result",
        )
        require(expected_rule in case["owning_rules"], f"{boundary}: wrong owning rule")
        require(
            "signature" in case["tags"] and expected_tag in case["tags"],
            f"{boundary}: incompatible case tags",
        )


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
        manifest_digest = validate_manifest_identity_commitment()
        manifest = load_manifest()
        require(manifest.get("schema_version") == 1, "unexpected manifest schema version")
        require(manifest.get("manifest_identity") == MANIFEST_ID, "unexpected manifest identity")
        require(manifest.get("status") == "candidate-pending-owner-review-and-merge", "manifest is not explicitly candidate")
        context = manifest.get("producing_context", {})
        require(context.get("signature_profile_identity") == V_SIG, "V_sig is not fixed")
        require(context.get("complete_history_derivation_profile") is None, "manifest silently minted G_history")
        require("latest" not in manifest["identity_law"].lower() or "may not" in manifest["identity_law"].lower(), "manifest identity law permits latest")

        validate_governing_source_commitments(manifest)

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
        framing_without_index = {
            case["id"]
            for case in cases
            if case["expected"]["verdict"] == "REJECT"
            and case["expected"]["class"] == "Framing"
            and case["expected"]["record_index"] is None
        }
        require(
            framing_without_index == FRAMING_WITHOUT_RECORD_INDEX_CASES,
            "Framing no-record-index case set mismatch",
        )
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
    print(
        "exact-byte hashes, governing commitments, coverage inventories, "
        "A-021 named classifications, and positive vocabularies checked"
    )
    print(f"manifest identity commitment: {MANIFEST_ID} -> {manifest_digest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
