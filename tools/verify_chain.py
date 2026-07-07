#!/usr/bin/env python3
import hashlib, json, sys

try:
    from cryptography.exceptions import InvalidSignature
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey
except ImportError:
    print("missing crypto dependency: cryptography", file=sys.stderr)
    sys.exit(2)

PROFILE = "magpie-core-v1"
MAGIC = b"magpie-core-v1"
SIG_CTX = b"magpie-sig-v1"
ZERO = b"\x00" * 32
HEX = set("0123456789abcdef")
STATUS = {"Open": 0, "Conjectured": 1, "Supported": 2, "Settled": 3, "Refuted": 4}
ACTOR_CLASSES = {
    "HumanRoot",
    "AgentProposer",
    "AutomatedVerifier",
    "DeadboltAnchorer",
    "LensWitness",
    "SourceImporter",
}
EVIDENCE_KINDS = {
    "DeterministicVerification",
    "HumanRatification",
    "DeadboltAnchor",
    "ExecutionEvidence",
    "BehavioralEvaluation",
    "ExternalSource",
    "ModelSelfReport",
    "LensReadout",
}
EDGE_KINDS = {"supports", "derived_from", "contradicts", "supersedes", "invalidates", "ratifies"}
def obj(value, name):
    if not isinstance(value, dict):
        raise ValueError(f"{name} must be an object")
    return value
def u64(value, name):
    if isinstance(value, bool) or not isinstance(value, int):
        raise ValueError(f"{name} must be a u64")
    if value < 0 or value > 0xFFFFFFFFFFFFFFFF:
        raise ValueError(f"{name} out of u64 range")
    return value.to_bytes(8, "big")
def u8(value, name):
    if isinstance(value, bool) or not isinstance(value, int) or not 0 <= value <= 0xFF:
        raise ValueError(f"{name} must be a u8")
    return bytes([value])
def status(value, name):
    if isinstance(value, str) and value in STATUS:
        return u8(STATUS[value], name)
    if isinstance(value, int) and not isinstance(value, bool) and 0 <= value <= 4:
        return u8(value, name)
    raise ValueError(f"{name} must be a known status")
def s(value, name):
    if not isinstance(value, str):
        raise ValueError(f"{name} must be a string")
    raw = value.encode("utf-8")
    return u64(len(raw), f"{name} length") + raw
def hx(value, byte_len, name):
    if not isinstance(value, str) or len(value) != byte_len * 2 or any(c not in HEX for c in value):
        raise ValueError(f"{name} must be {byte_len * 2} lowercase hex chars")
    return bytes.fromhex(value)
def non_empty(value, name):
    if not isinstance(value, str):
        raise ValueError(f"{name} must be a string")
    if value == "":
        raise ValueError(f"{name} must be non-empty")
    return value
def closed(value, allowed, name, label):
    if not isinstance(value, str):
        raise ValueError(f"{name} must be a string")
    if value not in allowed:
        raise ValueError(f"{name} must be a known ADR-0002 {label}")
    return value
def optional_hex64(value, name):
    if not isinstance(value, str):
        raise ValueError(f"{name} must be a string")
    if value != "" and (len(value) != 64 or any(c not in HEX for c in value)):
        raise ValueError(f"{name} must be empty or 64 lowercase hex chars")
    return value
def payload_bytes(payload):
    p = obj(payload, "payload")
    kind = p.get("kind")
    if kind == "Genesis":
        return b"\x00" + s(p.get("canonicalization_profile"), "payload.canonicalization_profile") + s(p.get("verifying_key"), "payload.verifying_key")
    if kind == "ClaimAsserted":
        return b"\x01" + s(p.get("claim_id"), "payload.claim_id") + s(p.get("statement"), "payload.statement") + status(p.get("status"), "payload.status")
    if kind == "EvidenceRecorded":
        return b"\x02" + s(p.get("claim_id"), "payload.claim_id") + s(p.get("summary"), "payload.summary")
    if kind == "ClaimStatusChanged":
        return b"\x03" + s(p.get("claim_id"), "payload.claim_id") + status(p.get("from"), "payload.from") + status(p.get("to"), "payload.to") + s(p.get("reason"), "payload.reason")
    if kind == "Note":
        return b"\x04" + s(p.get("text"), "payload.text")
    if kind == "SegmentAnchored":
        witness_root = p.get("witness_root")
        hx(witness_root, 32, "payload.witness_root")
        return (b"\x05" + s(p.get("bundle_kind"), "payload.bundle_kind")
                + s(witness_root, "payload.witness_root")
                + s(p.get("witness_algorithm"), "payload.witness_algorithm")
                + s(p.get("canonicalization_profile"), "payload.canonicalization_profile")
                + s(p.get("run_id"), "payload.run_id"))
    if kind == "ClaimAssertedV2":
        claim_id = non_empty(p.get("claim_id"), "payload.claim_id")
        statement = non_empty(p.get("statement"), "payload.statement")
        scope_ref = non_empty(p.get("scope_ref"), "payload.scope_ref")
        actor_class = closed(p.get("actor_class"), ACTOR_CLASSES, "payload.actor_class", "actor class")
        content_hash = optional_hex64(p.get("content_hash"), "payload.content_hash")
        return (b"\x06" + s(claim_id, "payload.claim_id")
                + s(statement, "payload.statement")
                + s(scope_ref, "payload.scope_ref")
                + s(actor_class, "payload.actor_class")
                + s(content_hash, "payload.content_hash")
                + s(p.get("metadata_json"), "payload.metadata_json"))
    if kind == "EvidenceRegistered":
        evidence_id = non_empty(p.get("evidence_id"), "payload.evidence_id")
        evidence_kind = closed(p.get("evidence_kind"), EVIDENCE_KINDS, "payload.evidence_kind", "evidence kind")
        summary = non_empty(p.get("summary"), "payload.summary")
        scope_ref = non_empty(p.get("scope_ref"), "payload.scope_ref")
        actor_class = closed(p.get("actor_class"), ACTOR_CLASSES, "payload.actor_class", "actor class")
        content_hash = optional_hex64(p.get("content_hash"), "payload.content_hash")
        return (b"\x07" + s(evidence_id, "payload.evidence_id")
                + s(evidence_kind, "payload.evidence_kind")
                + s(summary, "payload.summary")
                + s(scope_ref, "payload.scope_ref")
                + s(actor_class, "payload.actor_class")
                + s(content_hash, "payload.content_hash")
                + s(p.get("metadata_json"), "payload.metadata_json"))
    if kind == "JustificationEdgeRecorded":
        edge_id = non_empty(p.get("edge_id"), "payload.edge_id")
        edge_kind = closed(p.get("edge_kind"), EDGE_KINDS, "payload.edge_kind", "edge kind")
        source_id = non_empty(p.get("source_id"), "payload.source_id")
        target_id = non_empty(p.get("target_id"), "payload.target_id")
        scope_ref = non_empty(p.get("scope_ref"), "payload.scope_ref")
        actor_class = closed(p.get("actor_class"), ACTOR_CLASSES, "payload.actor_class", "actor class")
        rationale = non_empty(p.get("rationale"), "payload.rationale")
        return (b"\x08" + s(edge_id, "payload.edge_id")
                + s(edge_kind, "payload.edge_kind")
                + s(source_id, "payload.source_id")
                + s(target_id, "payload.target_id")
                + s(scope_ref, "payload.scope_ref")
                + s(actor_class, "payload.actor_class")
                + s(rationale, "payload.rationale")
                + s(p.get("metadata_json"), "payload.metadata_json"))
    raise ValueError("payload.kind must be a known v1 variant")
def canonical_core(core):
    c = obj(core, "core")
    p = obj(c.get("provenance"), "provenance")
    return (MAGIC + u64(c.get("seq"), "seq") + u64(c.get("timestamp_nanos"), "timestamp_nanos")
            + hx(c.get("prev_hash"), 32, "prev_hash") + s(p.get("agent"), "provenance.agent")
            + s(p.get("source"), "provenance.source") + payload_bytes(c.get("payload")))
def seq_of(event):
    if isinstance(event, dict) and isinstance(event.get("core"), dict):
        return event["core"].get("seq", "?")
    return "?"
def genesis_error(event, expected_seq, trust_hex):
    payload = event["core"]["payload"]
    kind = payload.get("kind")
    if expected_seq == 0:
        if kind != "Genesis":
            return "genesis rules failed at seq 0: event 0 is not Genesis"
        if payload.get("canonicalization_profile") != PROFILE:
            return "genesis rules failed at seq 0: wrong canonicalization_profile"
        if payload.get("verifying_key") != trust_hex:
            return "genesis rules failed at seq 0: verifying_key does not match trust root"
    elif kind == "Genesis":
        return f"genesis rules failed at seq {expected_seq}: Genesis appears after seq 0"
    return None
def verify_event(event, expected_seq, expected_prev, trust_hex, public_key):
    seq, digest, reason = seq_of(event), None, None
    try:
        core = obj(obj(event, "event").get("core"), "core")
        if core.get("seq") != expected_seq:
            reason = f"sequence failed at seq {seq}: expected {expected_seq}"
        elif expected_prev is None:
            reason = f"prev-link failed at seq {seq}: previous hash unavailable"
        else:
            try:
                if hx(core.get("prev_hash"), 32, "prev_hash") != expected_prev:
                    reason = f"prev-link failed at seq {seq}: prev_hash mismatch"
            except ValueError as exc:
                reason = f"prev-link failed at seq {seq}: {exc}"
        digest = hashlib.sha256(canonical_core(core)).digest()
        hash_hex = digest.hex()
        if reason is None and event.get("hash") != hash_hex:
            reason = f"hash failed at seq {seq}: stored hash mismatch"
        if reason is None:
            try:
                public_key.verify(hx(event.get("signature"), 64, "signature"), SIG_CTX + digest)
            except (InvalidSignature, ValueError) as exc:
                reason = str(exc) or "invalid signature"
                reason = f"signature failed at seq {seq}: {reason}"
        if reason is None:
            reason = genesis_error(event, expected_seq, trust_hex)
    except (KeyError, TypeError, ValueError) as exc:
        if reason is None:
            reason = f"parse/canonicalization failed at seq {seq}: {exc}"
        hash_hex = digest.hex() if digest is not None else "?"
    print(f"{seq} {hash_hex} {'OK' if reason is None else 'FAIL: ' + reason}")
    return reason is None, digest
def main(argv):
    if len(argv) != 3:
        print("usage: python tools/verify_chain.py <chain.jsonl> <verifying_key_hex>", file=sys.stderr)
        return 2
    trust_hex = argv[2].lower()
    try:
        public_key = Ed25519PublicKey.from_public_bytes(hx(trust_hex, 32, "verifying_key_hex"))
    except ValueError as exc:
        print(f"invalid verifying key: {exc}", file=sys.stderr)
        return 2
    ok, expected_prev, seen = True, ZERO, 0
    try:
        with open(argv[1], "r", encoding="utf-8") as chain:
            for expected_seq, line in enumerate(chain):
                seen += 1
                try:
                    event = json.loads(line)
                    event_ok, digest = verify_event(event, expected_seq, expected_prev, trust_hex, public_key)
                    ok = ok and event_ok
                    expected_prev = digest if digest is not None else None
                except json.JSONDecodeError as exc:
                    print(f"? ? FAIL: parse failed at line {seen}: {exc}")
                    ok, expected_prev = False, None
    except OSError as exc:
        print(f"cannot read chain file: {exc}", file=sys.stderr)
        return 2
    if seen == 0:
        print("? ? FAIL: empty chain has no genesis")
        return 1
    return 0 if ok else 1
if __name__ == "__main__":
    sys.exit(main(sys.argv))
