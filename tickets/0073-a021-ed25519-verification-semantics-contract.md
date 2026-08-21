# Ticket 0073: A-021 Ed25519 verification semantics

> **2026-08-21 status update:** The owner accepted ADR-0010, not this ticket's
> permissive-v1 recommendation. Accepted ADR-0010 selects the additive
> `magpie-ed25519-canonical-prime-subgroup-v1` subprofile and keeps unprofiled
> behavior compatibility-only. This ticket remains preserved evidence. Ticket
> 0075 now records the exact candidate corpus/profile manifest; hostile corpus
> review is next. A-021 remains Confirmed pending independent conformer
> implementation and review.

## Status

Owner-decision contract only. No Rust, Python, Go, fixture, FORMAT, release,
audit, CI, or ledger change is authorized here. A-021 remains **Confirmed**.

## Finding

A-021 is broader than weak-key admission. The named `magpie-core-v1` format
does not yet say which exact Ed25519 relation makes a well-shaped signature
verify. Current Rust and Python use ordinary verification; strict dalek
verification differs on both the preserved low-order-root witness (A21-D1) and
the ordinary-golden-key/small-order-`R` witness (A21-D2).

## Recommended v1 decision

Retain v1 compatibility and make it explicit:

- #120's canonical external-key representation gate remains in force;
- signature transport is exactly 128 lowercase hex characters to 64 bytes;
- `R` is a canonical compressed Edwards25519 point;
- `S` is little-endian and strictly `0 <= S < L`;
- `k = SHA-512(R || A || ("magpie-sig-v1" || hash)) mod L`;
- verification is the exact simple equation `[S]B = R + [k]A`;
- no cofactor multiplication, small-order rejection, or subgroup/trust test is
  added to v1; and
- malformed `R`, non-canonical `R`, non-canonical `S`, or equation failure is
  `REJECT(Signature)` after transport.

A canonical low-order `A` or `R` can therefore verify under v1. This records
compatibility behavior; it does not grant trust, identity, custody, or
authority.

## Version boundary

Do not retrofit strict, cofactored, or subgroup-rejecting semantics into v1.
If the owner wants that safer rule, define an additive `magpie-core-v2` (with
a distinct signature context, recommended `magpie-sig-v2`), a new chain and a
predecessor-tip migration rule before implementation. The exact v2 relation is
not selected by this ticket.

## Required vectors

The future exact-byte corpus must promote A21-D1 and A21-D2 to normative v1
`ACCEPT` vectors after ratification, while retaining them as mandatory
non-exhaustive witnesses during review. It must also cover:

- ordinary frozen golden and Deadbolt positives;
- `S = L`, `S = L + 1`, both sides of the `S = 0` boundary, and
  `S = L - 1`: equation mismatch → `REJECT(Signature)`, while the complete
  accepting S4/S5 cases below prove `S = 0` and the true upper scalar boundary;
- non-decodable, non-canonical, and zero-`x`/sign-bit-invalid `R`;
- a cofactor-sensitive residue proving the uncofactored equation;
- mixed-torsion equation mismatch → `REJECT(Signature)` (`A21-T1`) and a
  genuinely mixed-torsion `A = B + T4` equation match → `ACCEPT` (`A21-T2`),
  plus a wrong-message negative; and
- the #120 representation-invalid external-key precedence case.

`A21-S4`, `A21-S5`, and `A21-T2` are complete one-event Genesis vectors, not
equation-only probes. For A21-S4/A21-S5 use `timestamp_nanos = 1`, identity external/Genesis key
`0100000000000000000000000000000000000000000000000000000000000000`, and the
canonical EventCore hash
`12145a3e7b798309e45a1c55211c1973a00bcbfd932355e464694944c1bf4810`. S4 uses
identity `R` and `S = 0`, with signature
`01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000`;
it must `ACCEPT` with count 1 and that hash as tip. S5 reuses the same event
and uses `R = -B` (`58666666666666666666666666666666666666666666666666666666666666e6`)
and `S = L - 1`, whose little-endian bytes are
`ecd3f55c1a631258d69cf7a2def9de1400000000000000000000000000000010`; it must
also `ACCEPT` with the same count/tip.

For T2, hold the same Genesis fields fixed, set external/Genesis key to
`A = B + T4` (`5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea`),
and search `timestamp_nanos = 1, 2, ...` for the first `k mod 4 = 0`; the
winning timestamp is 2. Its canonical EventCore hash is
`ae729d0b73930f9dd5b3c5999200e073efacba802d9156e10f0b246de3f1a100`, with
`k = 6874093215602964270231976269017849938138400845214860727886828009061180358252`,
`S = k + 1`, and scalar bytes
`6dbeed42331f93e657f6c1587e7b21d5bb30f2fb079b5d8bfafe6c5b5099320f`; the
complete vector must `ACCEPT` with count 1 and that hash as tip. Its
`[L]A = T4` and non-identity `[4]A` proof remains mandatory.

Every normative Signature vector binds exact key/input bytes, a recomputable
canonical EventCore and content hash, expected verdict/class, and (for
accepted chains) ordered hashes, count, and tip. A point/key or equation
substage result is not a portable manifest result.

The conformance runner must recompute the canonical EventCore and SHA-256 hash
before exercising the signature. An arbitrary content hash is forbidden in a
normative Signature vector; equation-only probes remain development evidence
only.

## Cross-language obligations

Rust, Python, and Go must implement the relation independently. No library
default defines it. Rust must not silently substitute `verify_strict`; Python
must constrain `cryptography`; Go must not rely on undocumented or unstable
stdlib edge behavior. All three must agree on the exact domain-separated
message, point encodings, scalar range, equation, torsion behavior, and
verdicts.

## Protected compatibility

The implementation must preserve `magpie-core-v1` canonical bytes,
`magpie-sig-v1 || hash`, frozen golden/Deadbolt histories, external trust-root
binding, and JSONL's non-canonical transport status. If any existing v1 byte
falls outside the selected relation, stop and require a compatibility vector or
successor profile; do not silently reinterpret history.

## Closure and owner gates

A-021 remains open until the owner ratifies the v1 relation, all three
conformers pass the complete vectors, frozen histories remain valid, hostile
review completes, and the implementation is merged. A-004/RQ-006 cannot close
before those gates and a separate ledger reconciliation.

Stop for owner judgment before implementation if the owner rejects v1 D1/D2,
S4, S5, or T2 acceptance; if a locked conformer rejects a mathematically
equation-satisfying complete S4/S5/T2 construction; if a conformer needs an unreviewed
dependency; if any frozen v1 history disagrees; or if a safer successor
relation/name/migration has not been ratified. A host-library observation is
evidence to investigate, not authority to silently change the selected
relation.

The complete design contract is
[`docs/design/a021-ed25519-verification-semantics-contract.md`](../docs/design/a021-ed25519-verification-semantics-contract.md).
