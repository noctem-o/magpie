"""Independent magpie-core-v1 canonical encoder + JSONL history verifier.

Written from docs/FORMAT.md only (not from crates/magpie-log/src/canonical.rs)
so encoding bugs don't correlate with the reference prototype.
"""
import hashlib
import json

from vsig import vsig_verify

MAGIC = b"magpie-core-v1"
SIG_DOMAIN = b"magpie-sig-v1"

# FORMAT.md §3 payload tables: kind -> (tag, [(fieldname, type), ...])
# types: 'str' = u64-BE len + UTF-8; 'u8' = single byte; 'hex32' = str field
_PAYLOADS = {
    "Genesis": (0, [("canonicalization_profile", "str"), ("verifying_key", "str")]),
    "ClaimAsserted": (1, [("claim_id", "str"), ("statement", "str"), ("status", "status")]),
    "EvidenceRecorded": (2, [("claim_id", "str"), ("summary", "str")]),
    "ClaimStatusChanged": (3, [("claim_id", "str"), ("from", "status"), ("to", "status"), ("reason", "str")]),
    "Note": (4, [("text", "str")]),
    "SegmentAnchored": (5, [("bundle_kind", "str"), ("witness_root", "str"),
                            ("witness_algorithm", "str"), ("canonicalization_profile", "str"),
                            ("run_id", "str")]),
    "ClaimAssertedV2": (6, [("claim_id", "str"), ("statement", "str"), ("scope_ref", "str"),
                            ("actor_class", "str"), ("content_hash", "str"), ("metadata_json", "str")]),
    "EvidenceRegistered": (7, [("evidence_id", "str"), ("evidence_kind", "str"), ("summary", "str"),
                               ("scope_ref", "str"), ("actor_class", "str"), ("content_hash", "str"),
                               ("metadata_json", "str")]),
    "JustificationEdgeRecorded": (8, [("edge_id", "str"), ("edge_kind", "str"), ("source_id", "str"),
                                      ("target_id", "str"), ("scope_ref", "str"), ("actor_class", "str"),
                                      ("rationale", "str"), ("metadata_json", "str")]),
}


def _s(s):
    b = s.encode("utf-8")
    return len(b).to_bytes(8, "big") + b


def _u8(n):
    return bytes([n])


def encode_eventcore(core):
    """Encode EventCore dict -> exact canonical magpie-core-v1 bytes."""
    out = bytearray(MAGIC)
    out += core["seq"].to_bytes(8, "big")
    out += core["timestamp_nanos"].to_bytes(8, "big")
    prev = bytes.fromhex(core["prev_hash"])
    if len(prev) != 32:
        raise ValueError("prev_hash must be 32 bytes hex")
    out += prev
    out += _s(core["provenance"]["agent"])
    out += _s(core["provenance"]["source"])
    p = core["payload"]
    kind = p["kind"]
    if kind not in _PAYLOADS:
        raise ValueError(f"unknown payload kind {kind!r}")
    tag, fields = _PAYLOADS[kind]
    out.append(tag)
    for name, typ in fields:
        v = p[name]
        if typ == "str":
            out += _s(v)
        elif typ == "status":
            # Closed status spectrum per FORMAT §3: encoding order is meaningful.
            names = {"Open": 0, "Conjectured": 1, "Supported": 2, "Settled": 3, "Refuted": 4}
            if isinstance(v, str):
                if v not in names:
                    raise ValueError(f"unknown status {v!r}")
                out += _u8(names[v])
            else:
                out += _u8(v)
        elif typ == "u8":
            out += _u8(v)
        else:
            raise ValueError(typ)
    return bytes(out)


def content_hash(core):
    return hashlib.sha256(encode_eventcore(core)).hexdigest()


def message(core):
    return SIG_DOMAIN + hashlib.sha256(encode_eventcore(core)).digest()


class Rejection(Exception):
    def __init__(self, stage, detail):
        self.stage = stage
        self.detail = detail
        super().__init__(f"{stage}: {detail}")


def verify_history(lines_text, external_key_hex):
    """Verify a JSONL history per FORMAT chain rules + ADR-0010 V_sig.

    Returns dict with verdict, event_count, tip, ordered hashes on ACCEPT;
    raises Rejection with stage/detail otherwise. ExternalKey gate precedes
    input processing per the portable precedence chain.
    """
    # --- ExternalKey gate (COMPLETE composed gate before any input processing)
    # Per ADR-0010 the full key-admissibility relation — transport, canonical
    # decode, A != I, [L]A == I — must run BEFORE framing/empty-snapshot and
    # JSON handling, so a bad key rejects as ExternalKey regardless of input.
    if not isinstance(external_key_hex, str) or len(external_key_hex) != 64 \
            or any(c not in "0123456789abcdef" for c in external_key_hex):
        raise Rejection("ExternalKey", "key must be exactly 64 lowercase ASCII hex chars")
    A_bytes = bytes.fromhex(external_key_hex)
    from vsig import vsig_verify as _vv
    _key_gate = _vv(A_bytes, b"\x00" * 64, b"\x00" * 32)
    # Any ExternalKey-stage failure fires here; only a fully admissible key
    # proceeds. (The probe signature bytes are irrelevant: the key gate
    # precedes all signature work inside vsig_verify.)
    if _key_gate["stage"] == "ExternalKey":
        raise Rejection("ExternalKey", f"record 0: {_key_gate['detail']}")

    lines = [ln for ln in lines_text.split("\n") if ln.strip() != ""]
    if not lines:
        raise Rejection("Framing", "empty history")

    events = []
    for i, ln in enumerate(lines):
        try:
            ev = json.loads(ln)
        except json.JSONDecodeError as e:
            raise Rejection("JsonSyntax", f"line {i+1}: {e}")
        events.append(ev)

    ordered_hashes = []
    prev_hash = "00" * 32
    genesis_seen = False

    for idx, ev in enumerate(events):
        # Schema
        try:
            core = ev["core"]
            h = ev["hash"].lower()
            sig_hex = ev["signature"].lower()
            seq = core["seq"]
            ts = core["timestamp_nanos"]
            ph = core["prev_hash"].lower()
            prov = core["provenance"]
            agent, source = prov["agent"], prov["source"]
            payload = core["payload"]
            kind = payload["kind"]
        except (KeyError, TypeError, AttributeError) as e:
            raise Rejection("Schema", f"record {idx}: missing/invalid member {e}")
        if not isinstance(seq, int) or seq < 0 or seq > 2**64 - 1:
            raise Rejection("Sequence", f"record {idx}: seq out of u64 range")
        if kind == "Genesis" and idx == 0:
            genesis_seen = True
        if kind == "Genesis" and idx != 0:
            raise Rejection("Genesis", "Genesis appears after event 0")

        # Sequence + previous link
        expected_seq = idx
        if seq != expected_seq:
            raise Rejection("Sequence", f"record {idx}: seq {seq} != {expected_seq}")
        if ph != prev_hash:
            raise Rejection("PreviousLink", f"record {idx}: prev_hash mismatch")

        # Content hash over independently computed canonical bytes
        try:
            recomputed = content_hash(core)
        except (ValueError, KeyError, TypeError) as e:
            raise Rejection("PayloadValidation", f"record {idx}: canonical encode failed: {e}")
        if recomputed != h:
            raise Rejection("ContentHash",
                            f"record {idx}: stored hash {h} != recomputed {recomputed}")

        # Signature via V_sig
        M = SIG_DOMAIN + bytes.fromhex(h)
        res = vsig_verify(A_bytes, bytes.fromhex(sig_hex), M)
        if res["verdict"] != "ACCEPT":
            # vsig_verify already computes the correct stage (ExternalKey for
            # every failed external-key admissibility check — transport,
            # decode canonicity, A != I, [L]A = I — before any signature work;
            # Signature otherwise). Trust the stage field, never match on
            # detail strings.
            raise Rejection(res["stage"] or "Signature", f"record {idx}: {res['detail']}")

        # Genesis binding rules (checked at genesis position per FORMAT §5 order)
        if idx == 0:
            if kind != "Genesis":
                raise Rejection("Genesis", "event 0 is not Genesis")
            if payload.get("canonicalization_profile") != "magpie-core-v1":
                raise Rejection("Genesis", "canonicalization_profile mismatch")
            if payload.get("verifying_key").lower() != external_key_hex:
                raise Rejection("Genesis", "genesis verifying_key != supplied trust root")

        ordered_hashes.append(h)
        prev_hash = h

    return {
        "verdict": "ACCEPT",
        "event_count": len(events),
        "tip": ordered_hashes[-1],
        "ordered_recomputed_hashes": ordered_hashes,
    }
