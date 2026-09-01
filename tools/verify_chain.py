#!/usr/bin/env python3
"""Readable independent verifier for the frozen portable Magpie JSONL language.

This module implements the selected portable language and ADR-0010 signature
profile directly. It does not import, invoke, or derive answers from another
conformer. ``ACCEPT`` concerns only the exact supplied finite bytes under the
caller-supplied external key; it establishes no trust, authority, freshness,
completeness, or permission to act.
"""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
import hmac
import json
from pathlib import Path
import sys
from typing import Callable, Mapping, Sequence

try:
    from nacl import bindings as sodium
except ImportError as error:  # mapped to an operational CLI failure on use
    sodium = None
    _SODIUM_IMPORT_ERROR: ImportError | None = error
else:
    _SODIUM_IMPORT_ERROR = None


SIGNATURE_PROFILE = "magpie-ed25519-canonical-prime-subgroup-v1"
CORE_PROFILE = "magpie-core-v1"
SIGNATURE_DOMAIN = b"magpie-sig-v1"
ZERO_HASH = b"\x00" * 32
MAX_U64_TEXT = "18446744073709551615"
MAX_U64 = (1 << 64) - 1

STATUS_CODES = {
    "Open": 0,
    "Conjectured": 1,
    "Supported": 2,
    "Settled": 3,
    "Refuted": 4,
}
ACTOR_CLASSES = frozenset(
    {
        "HumanRoot",
        "AgentProposer",
        "AutomatedVerifier",
        "DeadboltAnchorer",
        "LensWitness",
        "SourceImporter",
    }
)
EVIDENCE_KINDS = frozenset(
    {
        "DeterministicVerification",
        "HumanRatification",
        "DeadboltAnchor",
        "ExecutionEvidence",
        "BehavioralEvaluation",
        "ExternalSource",
        "ModelSelfReport",
        "LensReadout",
    }
)
EDGE_KINDS = frozenset(
    {"supports", "derived_from", "contradicts", "supersedes", "invalidates", "ratifies"}
)

FIELD_MODULUS = (1 << 255) - 19
L = 7237005577332262213973186563042994240857116359379907606001950938285454250989
GROUP_ORDER_MINUS_ONE_LE = (L - 1).to_bytes(32, "little")
IDENTITY_POINT = b"\x01" + b"\x00" * 31


class UnsupportedProfileError(RuntimeError):
    """A caller selected a profile other than the one implemented here."""


class _JsonConstantError(ValueError):
    pass


class _SchemaError(ValueError):
    pass


@dataclass(frozen=True)
class _NumberToken:
    raw: str


@dataclass(frozen=True)
class _ObjectMembers:
    pairs: tuple[tuple[str, object], ...]


@dataclass(frozen=True)
class Outcome:
    verdict: str
    rejection_class: str | None
    line: int | None
    record_index: int | None
    event_count: int | None
    tip: str | None
    ordered_recomputed_hashes: tuple[str, ...]

    def as_dict(self) -> dict[str, object]:
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
class _PreparedKey:
    encoded: bytes


@dataclass(frozen=True)
class _Payload:
    kind: str
    fields: Mapping[str, object]


@dataclass(frozen=True)
class _Event:
    sequence: int
    timestamp_nanos: int
    previous_hash: bytes
    agent: str
    source: str
    payload: _Payload
    stored_hash: bytes
    signature: bytes


def _reject(
    rejection_class: str,
    line: int | None,
    record_index: int | None,
) -> Outcome:
    return Outcome(
        verdict="REJECT",
        rejection_class=rejection_class,
        line=line,
        record_index=record_index,
        event_count=None,
        tip=None,
        ordered_recomputed_hashes=(),
    )


def _accept(count: int, hashes: Sequence[bytes]) -> Outcome:
    rendered = tuple(digest.hex() for digest in hashes)
    tip = rendered[-1] if rendered else ZERO_HASH.hex()
    return Outcome(
        verdict="ACCEPT",
        rejection_class=None,
        line=None,
        record_index=None,
        event_count=count,
        tip=tip,
        ordered_recomputed_hashes=rendered,
    )


def _require_sodium() -> None:
    if sodium is None:
        raise RuntimeError(
            "PyNaCl is required for the selected signature profile"
        ) from _SODIUM_IMPORT_ERROR
    required = (
        "crypto_core_ed25519_add",
        "crypto_core_ed25519_scalar_reduce",
        "crypto_scalarmult_ed25519_base_noclamp",
        "crypto_scalarmult_ed25519_noclamp",
    )
    missing = [name for name in required if not hasattr(sodium, name)]
    if missing:
        raise RuntimeError(
            "PyNaCl/libsodium lacks required Ed25519 operations: " + ", ".join(missing)
        )
    if not getattr(sodium, "has_crypto_core_ed25519", False) or not getattr(
        sodium, "has_crypto_scalarmult_ed25519", False
    ):
        raise RuntimeError("PyNaCl was built without required Ed25519 arithmetic")


def _is_canonical_point(encoded: bytes) -> bool:
    if len(encoded) != 32:
        return False
    encoded_y = int.from_bytes(encoded, "little") & ((1 << 255) - 1)
    if encoded_y >= FIELD_MODULUS:
        return False
    try:
        reencoded = sodium.crypto_core_ed25519_add(encoded, IDENTITY_POINT)
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(reencoded, encoded)


def _is_prime_subgroup_point(encoded: bytes) -> bool:
    if hmac.compare_digest(encoded, IDENTITY_POINT):
        return True
    try:
        order_minus_one_times_point = sodium.crypto_scalarmult_ed25519_noclamp(
            GROUP_ORDER_MINUS_ONE_LE,
            encoded,
        )
        order_times_point = sodium.crypto_core_ed25519_add(
            order_minus_one_times_point,
            encoded,
        )
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(order_times_point, IDENTITY_POINT)


def _lower_hex(value: object, octets: int) -> bytes | None:
    if type(value) is not str or len(value) != octets * 2:
        return None
    if any(character not in "0123456789abcdef" for character in value):
        return None
    try:
        decoded = bytes.fromhex(value)
    except ValueError:
        return None
    return decoded if len(decoded) == octets else None


def _prepare_key(profile_identity: str, external_key_text: str) -> _PreparedKey | None:
    if profile_identity != SIGNATURE_PROFILE:
        raise UnsupportedProfileError(f"unsupported signature profile {profile_identity!r}")
    _require_sodium()
    encoded = _lower_hex(external_key_text, 32)
    if encoded is None:
        return None
    if not _is_canonical_point(encoded) or hmac.compare_digest(encoded, IDENTITY_POINT):
        return None
    if not _is_prime_subgroup_point(encoded):
        return None
    return _PreparedKey(encoded=encoded)


def _verify_signature(prepared: _PreparedKey, signature: bytes, content_hash: bytes) -> bool:
    r_encoded = signature[:32]
    if not _is_canonical_point(r_encoded) or not _is_prime_subgroup_point(r_encoded):
        return False
    scalar_s = signature[32:]
    if int.from_bytes(scalar_s, "little") >= L:
        return False
    message = SIGNATURE_DOMAIN + content_hash
    challenge = sodium.crypto_core_ed25519_scalar_reduce(
        hashlib.sha512(r_encoded + prepared.encoded + message).digest()
    )
    try:
        left = (
            IDENTITY_POINT
            if not any(scalar_s)
            else sodium.crypto_scalarmult_ed25519_base_noclamp(scalar_s)
        )
        challenge_times_key = (
            IDENTITY_POINT
            if not any(challenge)
            else sodium.crypto_scalarmult_ed25519_noclamp(challenge, prepared.encoded)
        )
        right = sodium.crypto_core_ed25519_add(r_encoded, challenge_times_key)
    except (RuntimeError, TypeError, ValueError):
        return False
    return hmac.compare_digest(left, right)


def _object_hook(pairs: list[tuple[str, object]]) -> _ObjectMembers:
    return _ObjectMembers(tuple(pairs))


def _number_hook(token: str) -> _NumberToken:
    return _NumberToken(token)


def _constant_hook(token: str) -> object:
    raise _JsonConstantError(f"non-finite JSON number {token!r}")


def _contains_surrogate(value: str) -> bool:
    return any(0xD800 <= ord(character) <= 0xDFFF for character in value)


def _members(value: object) -> dict[str, object]:
    if not isinstance(value, _ObjectMembers):
        raise _SchemaError("expected object")
    observed: dict[str, object] = {}
    for name, member in value.pairs:
        if _contains_surrogate(name) or name in observed:
            raise _SchemaError("invalid or duplicate object member")
        observed[name] = member
    return observed


def _exact_object(value: object, expected_names: frozenset[str]) -> dict[str, object]:
    observed = _members(value)
    if frozenset(observed) != expected_names:
        raise _SchemaError("object shape differs")
    return observed


def _text(value: object) -> str:
    if type(value) is not str or _contains_surrogate(value):
        raise _SchemaError("expected Unicode scalar string")
    return value


def _u64_token(value: object) -> int:
    if not isinstance(value, _NumberToken):
        raise _SchemaError("expected integer token")
    token = value.raw
    if token != "0" and (not token or token[0] not in "123456789"):
        raise _SchemaError("invalid unsigned integer spelling")
    if any(character not in "0123456789" for character in token):
        raise _SchemaError("invalid unsigned integer spelling")
    if len(token) > len(MAX_U64_TEXT) or (
        len(token) == len(MAX_U64_TEXT) and token > MAX_U64_TEXT
    ):
        raise _SchemaError("unsigned integer overflow")
    return int(token)


def _status(value: object) -> int:
    name = _text(value)
    try:
        return STATUS_CODES[name]
    except KeyError as error:
        raise _SchemaError("unknown status") from error


_PAYLOAD_FIELDS: dict[str, frozenset[str]] = {
    "Genesis": frozenset({"kind", "canonicalization_profile", "verifying_key"}),
    "ClaimAsserted": frozenset({"kind", "claim_id", "statement", "status"}),
    "EvidenceRecorded": frozenset({"kind", "claim_id", "summary"}),
    "ClaimStatusChanged": frozenset({"kind", "claim_id", "from", "to", "reason"}),
    "Note": frozenset({"kind", "text"}),
    "SegmentAnchored": frozenset(
        {
            "kind",
            "bundle_kind",
            "witness_root",
            "witness_algorithm",
            "canonicalization_profile",
            "run_id",
        }
    ),
    "ClaimAssertedV2": frozenset(
        {
            "kind",
            "claim_id",
            "statement",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        }
    ),
    "EvidenceRegistered": frozenset(
        {
            "kind",
            "evidence_id",
            "evidence_kind",
            "summary",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        }
    ),
    "JustificationEdgeRecorded": frozenset(
        {
            "kind",
            "edge_id",
            "edge_kind",
            "source_id",
            "target_id",
            "scope_ref",
            "actor_class",
            "rationale",
            "metadata_json",
        }
    ),
}


def _decode_payload(value: object) -> _Payload:
    fields = _members(value)
    kind = _text(fields.get("kind"))
    expected = _PAYLOAD_FIELDS.get(kind)
    if expected is None or frozenset(fields) != expected:
        raise _SchemaError("unknown payload or payload shape differs")

    decoded: dict[str, object] = {"kind": kind}
    for name in expected - {"kind"}:
        member = fields[name]
        if name in {"status", "from", "to"}:
            decoded[name] = _status(member)
        else:
            decoded[name] = _text(member)
    return _Payload(kind=kind, fields=decoded)


def _required_lower_hex(value: object, octets: int) -> bytes:
    decoded = _lower_hex(value, octets)
    if decoded is None:
        raise _SchemaError("invalid lowercase hex transport")
    return decoded


def _decode_event(value: object) -> _Event:
    signed = _exact_object(value, frozenset({"core", "hash", "signature"}))
    core = _exact_object(
        signed["core"],
        frozenset({"seq", "timestamp_nanos", "prev_hash", "provenance", "payload"}),
    )
    provenance = _exact_object(core["provenance"], frozenset({"agent", "source"}))
    return _Event(
        sequence=_u64_token(core["seq"]),
        timestamp_nanos=_u64_token(core["timestamp_nanos"]),
        previous_hash=_required_lower_hex(core["prev_hash"], 32),
        agent=_text(provenance["agent"]),
        source=_text(provenance["source"]),
        payload=_decode_payload(core["payload"]),
        stored_hash=_required_lower_hex(signed["hash"], 32),
        signature=_required_lower_hex(signed["signature"], 64),
    )


def _parse_record(text: str) -> tuple[_Event | None, str | None]:
    try:
        value = json.loads(
            text,
            object_pairs_hook=_object_hook,
            parse_int=_number_hook,
            parse_float=_number_hook,
            parse_constant=_constant_hook,
        )
    except (json.JSONDecodeError, _JsonConstantError):
        return None, "JsonSyntax"
    try:
        return _decode_event(value), None
    except (_SchemaError, UnicodeError, ValueError, OverflowError):
        return None, "Schema"


def _u64_bytes(value: int) -> bytes:
    return value.to_bytes(8, "big")


def _string_bytes(value: str) -> bytes:
    encoded = value.encode("utf-8")
    return _u64_bytes(len(encoded)) + encoded


def _payload_bytes(payload: _Payload) -> bytes:
    fields = payload.fields
    string = lambda name: _string_bytes(str(fields[name]))
    if payload.kind == "Genesis":
        return b"\x00" + string("canonicalization_profile") + string("verifying_key")
    if payload.kind == "ClaimAsserted":
        return b"\x01" + string("claim_id") + string("statement") + bytes([int(fields["status"])])
    if payload.kind == "EvidenceRecorded":
        return b"\x02" + string("claim_id") + string("summary")
    if payload.kind == "ClaimStatusChanged":
        return (
            b"\x03"
            + string("claim_id")
            + bytes([int(fields["from"])])
            + bytes([int(fields["to"])])
            + string("reason")
        )
    if payload.kind == "Note":
        return b"\x04" + string("text")
    if payload.kind == "SegmentAnchored":
        return (
            b"\x05"
            + string("bundle_kind")
            + string("witness_root")
            + string("witness_algorithm")
            + string("canonicalization_profile")
            + string("run_id")
        )
    if payload.kind == "ClaimAssertedV2":
        names = (
            "claim_id",
            "statement",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        )
        return b"\x06" + b"".join(string(name) for name in names)
    if payload.kind == "EvidenceRegistered":
        names = (
            "evidence_id",
            "evidence_kind",
            "summary",
            "scope_ref",
            "actor_class",
            "content_hash",
            "metadata_json",
        )
        return b"\x07" + b"".join(string(name) for name in names)
    names = (
        "edge_id",
        "edge_kind",
        "source_id",
        "target_id",
        "scope_ref",
        "actor_class",
        "rationale",
        "metadata_json",
    )
    return b"\x08" + b"".join(string(name) for name in names)


def _canonical_core(event: _Event) -> bytes:
    return (
        CORE_PROFILE.encode("ascii")
        + _u64_bytes(event.sequence)
        + _u64_bytes(event.timestamp_nanos)
        + event.previous_hash
        + _string_bytes(event.agent)
        + _string_bytes(event.source)
        + _payload_bytes(event.payload)
    )


def _non_empty_text(fields: Mapping[str, object], name: str) -> bool:
    return bool(str(fields[name]))


def _optional_hash_text(value: object) -> bool:
    return value == "" or _lower_hex(value, 32) is not None


def _payload_is_valid(payload: _Payload) -> bool:
    fields = payload.fields
    if payload.kind == "SegmentAnchored":
        return _lower_hex(fields["witness_root"], 32) is not None
    if payload.kind == "ClaimAssertedV2":
        return (
            all(_non_empty_text(fields, name) for name in ("claim_id", "statement", "scope_ref"))
            and fields["actor_class"] in ACTOR_CLASSES
            and _optional_hash_text(fields["content_hash"])
        )
    if payload.kind == "EvidenceRegistered":
        return (
            all(_non_empty_text(fields, name) for name in ("evidence_id", "summary", "scope_ref"))
            and fields["evidence_kind"] in EVIDENCE_KINDS
            and fields["actor_class"] in ACTOR_CLASSES
            and _optional_hash_text(fields["content_hash"])
        )
    if payload.kind == "JustificationEdgeRecorded":
        return (
            all(
                _non_empty_text(fields, name)
                for name in ("edge_id", "source_id", "target_id", "scope_ref", "rationale")
            )
            and fields["edge_kind"] in EDGE_KINDS
            and fields["actor_class"] in ACTOR_CLASSES
        )
    return True


def _genesis_is_valid(payload: _Payload, count: int, external_key: bytes) -> bool:
    if count == 0:
        if payload.kind != "Genesis":
            return False
        return (
            payload.fields["canonicalization_profile"] == CORE_PROFILE
            and _lower_hex(payload.fields["verifying_key"], 32) == external_key
        )
    return payload.kind != "Genesis"


def _accepted_next_count(count: int) -> int:
    if count == MAX_U64:
        raise RuntimeError("accepted event count exceeded the governed u64 domain")
    return count + 1


def _verify_prepared(prepared: _PreparedKey, history: bytes) -> Outcome:
    if not history:
        return _accept(0, ())

    count = 0
    tip = ZERO_HASH
    accepted_hashes: list[bytes] = []
    offset = 0
    line = 1
    record_index = 0
    while offset < len(history):
        lf = history.find(b"\n", offset)
        if lf >= 0:
            raw_candidate = history[offset:lf]
            candidate = raw_candidate[:-1] if raw_candidate.endswith(b"\r") else raw_candidate
            next_offset = lf + 1
            next_line = line + 1
        else:
            candidate = history[offset:]
            next_offset = len(history)
            next_line = line

        if not candidate:
            return _reject("Framing", line, None)
        coordinate = record_index
        if (
            all(byte in (0x20, 0x09) for byte in candidate)
            or b"\r" in candidate
            or candidate.startswith(b"\xef\xbb\xbf")
        ):
            return _reject("Framing", line, coordinate)
        try:
            text = candidate.decode("utf-8")
        except UnicodeDecodeError:
            return _reject("Framing", line, coordinate)

        event, rejection_class = _parse_record(text)
        if rejection_class is not None:
            return _reject(rejection_class, line, coordinate)
        assert event is not None
        if event.sequence != count:
            return _reject("Sequence", line, coordinate)
        if event.previous_hash != tip:
            return _reject("PreviousLink", line, coordinate)
        recomputed = hashlib.sha256(_canonical_core(event)).digest()
        if event.stored_hash != recomputed:
            return _reject("ContentHash", line, coordinate)
        if not _verify_signature(prepared, event.signature, recomputed):
            return _reject("Signature", line, coordinate)
        if not _payload_is_valid(event.payload):
            return _reject("PayloadValidation", line, coordinate)
        if not _genesis_is_valid(event.payload, count, prepared.encoded):
            return _reject("Genesis", line, coordinate)

        count = _accepted_next_count(count)
        tip = recomputed
        accepted_hashes.append(recomputed)
        record_index += 1
        offset = next_offset
        line = next_line

    return _accept(count, accepted_hashes)


def verify_complete_history(
    profile_identity: str,
    external_key_text: str,
    history: bytes,
) -> Outcome:
    """Verify one exact immutable byte history and return its governed result."""

    prepared = _prepare_key(profile_identity, external_key_text)
    if prepared is None:
        return _reject("ExternalKey", None, None)
    if type(history) is not bytes:
        raise TypeError("history must be exact bytes")
    return _verify_prepared(prepared, history)


def verify_file(
    profile_identity: str,
    external_key_text: str,
    path: str | Path,
    *,
    read_bytes: Callable[[Path], bytes] | None = None,
) -> Outcome:
    """Preflight the external key, then acquire and verify one file image."""

    prepared = _prepare_key(profile_identity, external_key_text)
    if prepared is None:
        return _reject("ExternalKey", None, None)
    selected_path = Path(path)
    history = selected_path.read_bytes() if read_bytes is None else read_bytes(selected_path)
    if type(history) is not bytes:
        raise TypeError("file reader must return exact bytes")
    return _verify_prepared(prepared, history)


def main(argv: Sequence[str]) -> int:
    if len(argv) != 3:
        print(
            "usage: python tools/verify_chain.py <chain.jsonl> <lowercase-verifying-key-hex>",
            file=sys.stderr,
        )
        return 2
    try:
        outcome = verify_file(SIGNATURE_PROFILE, argv[2], argv[1])
    except (OSError, UnsupportedProfileError, RuntimeError, TypeError, ValueError) as error:
        print(f"portable verifier operational failure: {error}", file=sys.stderr)
        return 2
    print(json.dumps(outcome.as_dict(), ensure_ascii=True, separators=(",", ":")))
    return 0 if outcome.verdict == "ACCEPT" else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
