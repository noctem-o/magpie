#!/usr/bin/env python3
"""Independent complete conformer for Magpie's frozen portable history language.

This module is deliberately separate from ``tools/verify_chain.py``.  The
legacy tool remains compatibility evidence for ordinary ``magpie-core-v1``
histories; this tool implements the explicitly selected ADR-0010 signature
profile and the frozen portable input-language stages.

The implementation is derived from FORMAT, Accepted ADR-0010, and the frozen
portable-verifier contracts.  A governed result says only whether the exact
supplied finite bytes satisfy that language under the exact supplied key.
It establishes no trust, authority, currentness, or global completeness.
"""

from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import NoReturn, Sequence

try:
    from nacl import bindings as sodium
except ImportError as error:  # pragma: no cover - exercised as operational CLI failure
    sodium = None
    _SODIUM_IMPORT_ERROR: ImportError | None = error
else:
    _SODIUM_IMPORT_ERROR = None


SIGNATURE_PROFILE_ID = "magpie-ed25519-canonical-prime-subgroup-v1"
CANONICALIZATION_PROFILE = "magpie-core-v1"
CORE_MAGIC = b"magpie-core-v1"
SIGNATURE_DOMAIN = b"magpie-sig-v1"

_ZERO_HASH = bytes(32)
_IDENTITY_POINT = b"\x01" + bytes(31)
_FIELD_MODULUS = 2**255 - 19
_SCALAR_ORDER = 2**252 + 27742317777372353535851937790883648493
_SCALAR_ORDER_MINUS_ONE = (_SCALAR_ORDER - 1).to_bytes(32, "little")
_LOWER_HEX_32 = re.compile(r"[0-9a-f]{64}\Z", re.ASCII)
_LOWER_HEX_64 = re.compile(r"[0-9a-f]{128}\Z", re.ASCII)
_U64_TOKEN = re.compile(r"(?:0|[1-9][0-9]*)\Z", re.ASCII)

_STATUS_TAG = {
    "Open": 0,
    "Conjectured": 1,
    "Supported": 2,
    "Settled": 3,
    "Refuted": 4,
}
_ACTOR_CLASSES = {
    "HumanRoot",
    "AgentProposer",
    "AutomatedVerifier",
    "DeadboltAnchorer",
    "LensWitness",
    "SourceImporter",
}
_EVIDENCE_KINDS = {
    "DeterministicVerification",
    "HumanRatification",
    "DeadboltAnchor",
    "ExecutionEvidence",
    "BehavioralEvaluation",
    "ExternalSource",
    "ModelSelfReport",
    "LensReadout",
}
_EDGE_KINDS = {
    "supports",
    "derived_from",
    "contradicts",
    "supersedes",
    "invalidates",
    "ratifies",
}


class OperationalError(RuntimeError):
    """A configuration, dependency, I/O, or internal failure outside verdicts."""


class UnsupportedProfile(OperationalError):
    """The caller did not select the one frozen signature profile."""


class _SchemaFailure(ValueError):
    pass


class _JsonSyntaxFailure(ValueError):
    pass


class _ObjectPairs(list[tuple[str, object]]):
    """A JSON object that retains decoded member order and duplicates."""


class _NumberToken(str):
    """An uncoerced JSON number spelling."""


@dataclass(frozen=True)
class Outcome:
    verdict: str
    rejection_class: str | None
    line: int | None
    record_index: int | None
    event_count: int | None
    tip: str | None
    ordered_recomputed_hashes: tuple[str, ...]

    @classmethod
    def accept(cls, hashes: Sequence[bytes]) -> "Outcome":
        encoded = tuple(value.hex() for value in hashes)
        tip = encoded[-1] if encoded else _ZERO_HASH.hex()
        return cls("ACCEPT", None, None, None, len(encoded), tip, encoded)

    @classmethod
    def reject(
        cls,
        rejection_class: str,
        line: int | None = None,
        record_index: int | None = None,
    ) -> "Outcome":
        return cls("REJECT", rejection_class, line, record_index, None, None, ())

    def as_dict(self) -> dict[str, object]:
        """Return exactly the governed manifest comparison fields."""

        return {
            "verdict": self.verdict,
            "class": self.rejection_class,
            "line": self.line,
            "record_index": self.record_index,
            "event_count": self.event_count,
            "tip": self.tip,
            "ordered_recomputed_hashes": list(self.ordered_recomputed_hashes),
        }


@dataclass(frozen=True)
class _Payload:
    kind: str
    fields: dict[str, str]


@dataclass(frozen=True)
class _Record:
    seq: int
    timestamp_nanos: int
    prev_hash: bytes
    agent: str
    source: str
    payload: _Payload
    stored_hash: bytes
    signature: bytes


def _require_sodium() -> None:
    if sodium is None:
        raise OperationalError("PyNaCl is required for the selected signature profile") from _SODIUM_IMPORT_ERROR
    required = (
        "crypto_core_ed25519_add",
        "crypto_core_ed25519_scalar_reduce",
        "crypto_scalarmult_ed25519_base_noclamp",
        "crypto_scalarmult_ed25519_noclamp",
    )
    missing = [name for name in required if not hasattr(sodium, name)]
    if missing:
        raise OperationalError(
            "PyNaCl/libsodium lacks required Ed25519 operations: " + ", ".join(missing)
        )
    if not getattr(sodium, "has_crypto_core_ed25519", False) or not getattr(
        sodium, "has_crypto_scalarmult_ed25519", False
    ):
        raise OperationalError("PyNaCl was built without required Ed25519 arithmetic")


def _canonical_curve_point(encoded: bytes) -> bool:
    """Check RFC 8032 decoding plus byte-identical canonical re-encoding."""

    if len(encoded) != 32:
        return False
    encoded_y = int.from_bytes(encoded, "little") & ((1 << 255) - 1)
    if encoded_y >= _FIELD_MODULUS:
        return False
    try:
        reencoded = sodium.crypto_core_ed25519_add(encoded, _IDENTITY_POINT)
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(reencoded, encoded)


def _prime_subgroup_point(encoded: bytes) -> bool:
    """Apply the libsodium-documented L-1 subgroup workaround.

    Identity is explicitly in the prime subgroup and is handled separately
    because libsodium's non-clamped scalar multiplication rejects small-order
    input points.
    """

    if hmac.compare_digest(encoded, _IDENTITY_POINT):
        return True
    try:
        almost_identity = sodium.crypto_scalarmult_ed25519_noclamp(
            _SCALAR_ORDER_MINUS_ONE, encoded
        )
        multiplied_by_l = sodium.crypto_core_ed25519_add(almost_identity, encoded)
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(multiplied_by_l, _IDENTITY_POINT)


def _prepare_external_key(profile_identity: str, external_key_text: str) -> bytes | None:
    if profile_identity != SIGNATURE_PROFILE_ID:
        raise UnsupportedProfile(f"unsupported signature-verification profile: {profile_identity!r}")
    _require_sodium()
    if _LOWER_HEX_32.fullmatch(external_key_text) is None:
        return None
    encoded = bytes.fromhex(external_key_text)
    if not _canonical_curve_point(encoded):
        return None
    if hmac.compare_digest(encoded, _IDENTITY_POINT):
        return None
    if not _prime_subgroup_point(encoded):
        return None
    return encoded


def _reject_non_rfc_constant(token: str) -> NoReturn:
    raise _JsonSyntaxFailure(f"non-RFC JSON constant {token!r}")


def _parse_json(candidate: str) -> object:
    try:
        return json.loads(
            candidate,
            object_pairs_hook=_ObjectPairs,
            parse_int=_NumberToken,
            parse_float=_NumberToken,
            parse_constant=_reject_non_rfc_constant,
            strict=True,
        )
    except _JsonSyntaxFailure:
        raise
    except RecursionError as error:
        raise OperationalError("JSON nesting exceeded the host parser capacity") from error
    except (json.JSONDecodeError, UnicodeError, ValueError) as error:
        raise _JsonSyntaxFailure from error


def _object(value: object, required: Sequence[str]) -> dict[str, object]:
    if type(value) is not _ObjectPairs:
        raise _SchemaFailure
    decoded: dict[str, object] = {}
    for name, member in value:
        if type(name) is not str or not _unicode_scalar_string(name) or name in decoded:
            raise _SchemaFailure
        decoded[name] = member
    if len(decoded) != len(required) or set(decoded) != set(required):
        raise _SchemaFailure
    return decoded


def _unicode_scalar_string(value: str) -> bool:
    try:
        value.encode("utf-8", "strict")
    except UnicodeEncodeError:
        return False
    return True


def _string(value: object) -> str:
    if type(value) is not str or not _unicode_scalar_string(value):
        raise _SchemaFailure
    return value


def _u64(value: object) -> int:
    if type(value) is not _NumberToken or _U64_TOKEN.fullmatch(value) is None:
        raise _SchemaFailure
    maximum = "18446744073709551615"
    if len(value) > len(maximum) or (len(value) == len(maximum) and value > maximum):
        raise _SchemaFailure
    decoded = int(value, 10)
    return decoded


def _lower_hex(value: object, octets: int) -> bytes:
    text = _string(value)
    pattern = _LOWER_HEX_32 if octets == 32 else _LOWER_HEX_64
    if pattern.fullmatch(text) is None:
        raise _SchemaFailure
    return bytes.fromhex(text)


def _status(value: object) -> str:
    decoded = _string(value)
    if decoded not in _STATUS_TAG:
        raise _SchemaFailure
    return decoded


def _decode_payload(value: object) -> _Payload:
    if type(value) is not _ObjectPairs:
        raise _SchemaFailure

    # Read only enough to select the closed variant.  The full object check
    # below independently catches duplicates, escaped-equivalent names, and
    # unknown members.
    kind_values: list[object] = []
    decoded_names: set[str] = set()
    for name, member in value:
        if type(name) is not str or not _unicode_scalar_string(name) or name in decoded_names:
            raise _SchemaFailure
        decoded_names.add(name)
        if name == "kind":
            kind_values.append(member)
    if len(kind_values) != 1:
        raise _SchemaFailure
    kind = _string(kind_values[0])

    variant_fields: dict[str, tuple[str, ...]] = {
        "Genesis": ("canonicalization_profile", "verifying_key"),
        "ClaimAsserted": ("claim_id", "statement", "status"),
        "EvidenceRecorded": ("claim_id", "summary"),
        "ClaimStatusChanged": ("claim_id", "from", "to", "reason"),
        "Note": ("text",),
        "SegmentAnchored": (
            "bundle_kind",
            "witness_root",
            "witness_algorithm",
            "canonicalization_profile",
            "run_id",
        ),
        "ClaimAssertedV2": (
            "claim_id",
            "statement",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        ),
        "EvidenceRegistered": (
            "evidence_id",
            "evidence_kind",
            "summary",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        ),
        "JustificationEdgeRecorded": (
            "edge_id",
            "edge_kind",
            "source_id",
            "target_id",
            "scope_ref",
            "actor_class",
            "rationale",
            "metadata_json",
        ),
    }
    selected = variant_fields.get(kind)
    if selected is None:
        raise _SchemaFailure
    members = _object(value, ("kind", *selected))
    fields: dict[str, str] = {}
    for name in selected:
        if kind == "ClaimAsserted" and name == "status":
            fields[name] = _status(members[name])
        elif kind == "ClaimStatusChanged" and name in ("from", "to"):
            fields[name] = _status(members[name])
        else:
            fields[name] = _string(members[name])
    return _Payload(kind, fields)


def _decode_record(value: object) -> _Record:
    outer = _object(value, ("core", "hash", "signature"))
    core = _object(
        outer["core"],
        ("seq", "timestamp_nanos", "prev_hash", "provenance", "payload"),
    )
    provenance = _object(core["provenance"], ("agent", "source"))
    return _Record(
        seq=_u64(core["seq"]),
        timestamp_nanos=_u64(core["timestamp_nanos"]),
        prev_hash=_lower_hex(core["prev_hash"], 32),
        agent=_string(provenance["agent"]),
        source=_string(provenance["source"]),
        payload=_decode_payload(core["payload"]),
        stored_hash=_lower_hex(outer["hash"], 32),
        signature=_lower_hex(outer["signature"], 64),
    )


def _put_string(output: bytearray, value: str) -> None:
    encoded = value.encode("utf-8")
    output.extend(len(encoded).to_bytes(8, "big"))
    output.extend(encoded)


def _canonical_core(record: _Record) -> bytes:
    output = bytearray(CORE_MAGIC)
    output.extend(record.seq.to_bytes(8, "big"))
    output.extend(record.timestamp_nanos.to_bytes(8, "big"))
    output.extend(record.prev_hash)
    _put_string(output, record.agent)
    _put_string(output, record.source)

    payload = record.payload
    fields = payload.fields
    tags = {
        "Genesis": 0,
        "ClaimAsserted": 1,
        "EvidenceRecorded": 2,
        "ClaimStatusChanged": 3,
        "Note": 4,
        "SegmentAnchored": 5,
        "ClaimAssertedV2": 6,
        "EvidenceRegistered": 7,
        "JustificationEdgeRecorded": 8,
    }
    output.append(tags[payload.kind])

    if payload.kind == "Genesis":
        _put_string(output, fields["canonicalization_profile"])
        _put_string(output, fields["verifying_key"])
    elif payload.kind == "ClaimAsserted":
        _put_string(output, fields["claim_id"])
        _put_string(output, fields["statement"])
        output.append(_STATUS_TAG[fields["status"]])
    elif payload.kind == "EvidenceRecorded":
        _put_string(output, fields["claim_id"])
        _put_string(output, fields["summary"])
    elif payload.kind == "ClaimStatusChanged":
        _put_string(output, fields["claim_id"])
        output.append(_STATUS_TAG[fields["from"]])
        output.append(_STATUS_TAG[fields["to"]])
        _put_string(output, fields["reason"])
    elif payload.kind == "Note":
        _put_string(output, fields["text"])
    elif payload.kind == "SegmentAnchored":
        for name in (
            "bundle_kind",
            "witness_root",
            "witness_algorithm",
            "canonicalization_profile",
            "run_id",
        ):
            _put_string(output, fields[name])
    elif payload.kind == "ClaimAssertedV2":
        for name in (
            "claim_id",
            "statement",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        ):
            _put_string(output, fields[name])
    elif payload.kind == "EvidenceRegistered":
        for name in (
            "evidence_id",
            "evidence_kind",
            "summary",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        ):
            _put_string(output, fields[name])
    else:
        for name in (
            "edge_id",
            "edge_kind",
            "source_id",
            "target_id",
            "scope_ref",
            "actor_class",
            "rationale",
            "metadata_json",
        ):
            _put_string(output, fields[name])
    return bytes(output)


def _valid_lower_hex_32_or_empty(value: str) -> bool:
    return value == "" or _LOWER_HEX_32.fullmatch(value) is not None


def _payload_is_valid(payload: _Payload) -> bool:
    fields = payload.fields
    if payload.kind in {
        "Genesis",
        "ClaimAsserted",
        "EvidenceRecorded",
        "ClaimStatusChanged",
        "Note",
    }:
        return True
    if payload.kind == "SegmentAnchored":
        return _LOWER_HEX_32.fullmatch(fields["witness_root"]) is not None
    if payload.kind == "ClaimAssertedV2":
        return (
            all(fields[name] != "" for name in ("claim_id", "statement", "scope_ref"))
            and fields["actor_class"] in _ACTOR_CLASSES
            and _valid_lower_hex_32_or_empty(fields["content_hash"])
        )
    if payload.kind == "EvidenceRegistered":
        return (
            all(fields[name] != "" for name in ("evidence_id", "summary", "scope_ref"))
            and fields["evidence_kind"] in _EVIDENCE_KINDS
            and fields["actor_class"] in _ACTOR_CLASSES
            and _valid_lower_hex_32_or_empty(fields["content_hash"])
        )
    return (
        all(
            fields[name] != ""
            for name in ("edge_id", "source_id", "target_id", "scope_ref", "rationale")
        )
        and fields["edge_kind"] in _EDGE_KINDS
        and fields["actor_class"] in _ACTOR_CLASSES
    )


def _genesis_is_valid(payload: _Payload, event_count: int, external_key: bytes) -> bool:
    if event_count == 0:
        if payload.kind != "Genesis":
            return False
        if payload.fields["canonicalization_profile"] != CANONICALIZATION_PROFILE:
            return False
        declared = payload.fields["verifying_key"]
        if _LOWER_HEX_32.fullmatch(declared) is None:
            return False
        return hmac.compare_digest(bytes.fromhex(declared), external_key)
    return payload.kind != "Genesis"


def _signature_is_valid(external_key: bytes, content_hash: bytes, signature: bytes) -> bool:
    r_encoded = signature[:32]
    s_encoded = signature[32:]
    if not _canonical_curve_point(r_encoded) or not _prime_subgroup_point(r_encoded):
        return False
    if int.from_bytes(s_encoded, "little") >= _SCALAR_ORDER:
        return False

    challenge_wide = hashlib.sha512(
        r_encoded + external_key + SIGNATURE_DOMAIN + content_hash
    ).digest()
    challenge = sodium.crypto_core_ed25519_scalar_reduce(challenge_wide)
    try:
        left = (
            _IDENTITY_POINT
            if not any(s_encoded)
            else sodium.crypto_scalarmult_ed25519_base_noclamp(s_encoded)
        )
        challenge_times_key = (
            _IDENTITY_POINT
            if not any(challenge)
            else sodium.crypto_scalarmult_ed25519_noclamp(challenge, external_key)
        )
        right = sodium.crypto_core_ed25519_add(r_encoded, challenge_times_key)
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(left, right)


def verify_complete_history(
    profile_identity: str,
    external_key_text: str,
    history: bytes,
) -> Outcome:
    """Verify one exact supplied finite history through all frozen stages."""

    external_key = _prepare_external_key(profile_identity, external_key_text)
    if external_key is None:
        return Outcome.reject("ExternalKey")
    if type(history) is not bytes:
        raise OperationalError("history input must be exact bytes")

    offset = 0
    line = 1
    event_count = 0
    tip = _ZERO_HASH
    accepted_hashes: list[bytes] = []

    while offset < len(history):
        lf = history.find(b"\n", offset)
        if lf < 0:
            raw_candidate = history[offset:]
            next_offset = len(history)
            next_line = line
        else:
            raw_candidate = history[offset:lf]
            next_offset = lf + 1
            next_line = line + 1

        candidate = (
            raw_candidate[:-1]
            if lf >= 0 and raw_candidate.endswith(b"\r")
            else raw_candidate
        )
        if candidate == b"":
            return Outcome.reject("Framing", line, None)

        record_index = event_count
        if (
            all(byte in (0x20, 0x09) for byte in candidate)
            or b"\r" in candidate
            or candidate.startswith(b"\xef\xbb\xbf")
        ):
            return Outcome.reject("Framing", line, record_index)
        try:
            candidate_text = candidate.decode("utf-8", "strict")
        except UnicodeDecodeError:
            return Outcome.reject("Framing", line, record_index)

        try:
            syntax_value = _parse_json(candidate_text)
        except _JsonSyntaxFailure:
            return Outcome.reject("JsonSyntax", line, record_index)
        try:
            record = _decode_record(syntax_value)
        except _SchemaFailure:
            return Outcome.reject("Schema", line, record_index)

        if record.seq != event_count:
            return Outcome.reject("Sequence", line, record_index)
        if not hmac.compare_digest(record.prev_hash, tip):
            return Outcome.reject("PreviousLink", line, record_index)

        recomputed_hash = hashlib.sha256(_canonical_core(record)).digest()
        if not hmac.compare_digest(record.stored_hash, recomputed_hash):
            return Outcome.reject("ContentHash", line, record_index)
        if not _signature_is_valid(external_key, recomputed_hash, record.signature):
            return Outcome.reject("Signature", line, record_index)
        if not _payload_is_valid(record.payload):
            return Outcome.reject("PayloadValidation", line, record_index)
        if not _genesis_is_valid(record.payload, event_count, external_key):
            return Outcome.reject("Genesis", line, record_index)

        if event_count == 0xFFFF_FFFF_FFFF_FFFF:
            raise OperationalError("accepted event count exceeded the governed u64 domain")

        # The continuation and state become usable only after every current
        # record stage succeeds.
        accepted_hashes.append(recomputed_hash)
        event_count += 1
        tip = recomputed_hash
        offset = next_offset
        line = next_line

    return Outcome.accept(accepted_hashes)


def _argument_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("profile_identity", help="exact ADR-0010 profile identity")
    parser.add_argument("external_key", help="exact lowercase 32-byte external key hex")
    parser.add_argument("history", type=Path, help="exact history byte file")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = _argument_parser().parse_args(argv)
    try:
        history = args.history.read_bytes()
        outcome = verify_complete_history(args.profile_identity, args.external_key, history)
    except (OSError, OperationalError) as error:
        print(f"operational error: {error}", file=sys.stderr)
        return 2
    print(json.dumps(outcome.as_dict(), sort_keys=True, separators=(",", ":")))
    return 0 if outcome.verdict == "ACCEPT" else 1


if __name__ == "__main__":
    raise SystemExit(main())
