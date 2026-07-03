# Magpie log format — `magpie-core-v1`

This document is **normative**. A from-scratch implementation written from this
page alone must reproduce the golden vectors in
`crates/magpie-log/testdata/golden-v1.jsonl` and `tests/golden.rs`, byte for
byte. If it can't, one of the two is wrong — and it's probably not this page.

Any change to anything below is a **format break** and requires a new profile
name. A chain carries exactly one profile, declared in its genesis event.

Vocabulary history (additive, non-breaking per §3's evolution rule): tags 0-4
at the v1 freeze; tag 5 (`SegmentAnchored`) added 2026-07-02 by ADR-0001, with
golden coverage appended at seq 5 — seqs 0-4 hashes unchanged.

## 1. Primitives

| name     | encoding                                                            |
|----------|---------------------------------------------------------------------|
| `u64`    | 8 bytes, big-endian                                                 |
| `u8`     | 1 byte                                                              |
| `str`    | `u64` byte-length, then the UTF-8 bytes **exactly as held** — no Unicode normalization, no escaping |
| `hash32` | 32 raw bytes                                                        |

## 2. Canonical bytes of an event core

The preimage of the content hash. Fixed field order, no field names, no
padding:

```
"magpie-core-v1"      14 bytes ASCII magic (hash-domain separation)
seq                   u64
timestamp_nanos       u64      transaction time (write time); valid-time, when
                               it exists, lives inside payloads
prev_hash             hash32   32 zero bytes for the genesis event
provenance.agent      str
provenance.source     str
payload               u8 tag, then the variant's fields in order (§3)
```

## 3. Payload tags

| tag | variant             | fields, in order                                    |
|-----|---------------------|-----------------------------------------------------|
| 0   | `Genesis`           | `canonicalization_profile: str`, `verifying_key: str` (64 lowercase hex chars) |
| 1   | `ClaimAsserted`     | `claim_id: str`, `statement: str`, `status: u8`     |
| 2   | `EvidenceRecorded`  | `claim_id: str`, `summary: str`                     |
| 3   | `ClaimStatusChanged`| `claim_id: str`, `from: u8`, `to: u8`, `reason: str`|
| 4   | `Note`              | `text: str`                                         |
| 5   | `SegmentAnchored`   | `bundle_kind: str`, `witness_root: str` (64 lowercase hex chars), `witness_algorithm: str`, `canonicalization_profile: str`, `run_id: str` |

### Anchors (tag 5)

`SegmentAnchored` commits an externally sealed evidence segment into the chain
by its root. The chain guarantees **order, signature, and inclusion**; the
segment's contents verify separately against `witness_root` using the sealing
subsystem's own verifier, selected by (`witness_algorithm`,
`canonicalization_profile`). The `canonicalization_profile` field names the
**foreign** profile that produced the root (e.g.
`phase5-interim-jcs-like-v1`); it is not, and must not claim to be,
`magpie-core-v1`. The variant is deliberately kind-agnostic: new bundle kinds
are new `bundle_kind` strings, never new Magpie payload variants. Under the
anchored-hierarchy model (ADR-0001), a segment that is not anchored is not
part of the record.

New variants may be appended with fresh tags without breaking existing chains
(old bytes are unaffected); changing an **existing** variant's encoding is a
format break.

Status tags follow the epistemic spectrum's order — the encoding is
meaningful:

| tag | status      |
|-----|-------------|
| 0   | Open        |
| 1   | Conjectured |
| 2   | Supported   |
| 3   | Settled     |
| 4   | Refuted     |

## 4. Hash and signature

```
hash      = SHA-256( canonical bytes )
signature = Ed25519-sign( "magpie-sig-v1" || hash )        (13 + 32 bytes)
```

Two independent domain separations: the magic inside the preimage means a
Magpie content hash can never collide with a hash of non-Magpie material; the
signature context means this key's log signatures can never be replayed onto
anything else it signs.

## 5. Chain rules

- `seq` starts at 0 and increments by exactly 1.
- `prev_hash` of event *n+1* equals the `hash` of event *n*; the genesis's
  `prev_hash` is 32 zero bytes.
- Event 0 of any non-empty chain **must** be `Genesis`, and `Genesis` must
  appear nowhere else. Writers enforce this at append time; verifiers at read
  time.
- The genesis's `canonicalization_profile` must equal the profile the verifier
  implements (`magpie-core-v1`), and its `verifying_key` must equal the hex of
  the externally provided trust root. The genesis makes a chain
  **self-describing, not self-authenticating** — trust in the key always comes
  from outside the chain.
- Verification of event *n* checks, in order: sequence, prev-link, recomputed
  content hash, signature, genesis rules.

## 6. Storage (non-normative)

The reference store is JSON-lines: one serialized `SignedEvent` per line, hash
and signature as lowercase hex. **Stored bytes are not the preimage** — the
hash is always recomputed from canonical bytes, so the storage format may
change freely without a format break.

## 7. Evolution

One chain, one profile. The intended migration path to a future
`magpie-core-v2` is a **new chain** whose genesis carries a reference to the
predecessor chain's tip hash (chain-of-chains), preserving the old chain's
signatures instead of re-signing history. That reference field is reserved for
v2; it does not exist in v1.

## 8. Worked example

Golden event seq 0 (the fixture genesis; seed `[7u8; 32]`, first clock tick),
190 bytes:

```
6d61677069652d636f72652d7631                                      "magpie-core-v1"
0000000000000000                                                  seq = 0
0000000000000001                                                  timestamp_nanos = 1
0000…0000 (32 bytes)                                              prev_hash = zero
000000000000000a 6d61677069652d6c6f67                             agent  = "magpie-log"
0000000000000007 67656e65736973                                   source = "genesis"
00                                                                tag: Genesis
000000000000000e 6d61677069652d636f72652d7631                     profile = "magpie-core-v1"
0000000000000040 6561346136…3232 (64 hex chars)                   verifying_key
```

`SHA-256` of the whole preimage:
`cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f` — as pinned
in `tests/golden.rs`.

## Why not JSON canonicalization

RFC 8785 (JCS) serializes numbers by ECMAScript rules, which are exact only to
2⁵³; `timestamp_nanos` is ~1.8×10¹⁸ today, so a conformant JCS implementation
cannot represent this type. JSON-subset profiles that keep integers are
already nonstandard, but retain JSON's whole ambiguity surface (string
escaping, key ordering, number formatting) while buying no interoperability.
This format is bespoke on purpose: the spec is one page, has no ambiguity
surface, and reimplementation is an afternoon.
