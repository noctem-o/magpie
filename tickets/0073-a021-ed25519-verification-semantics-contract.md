# Ticket 0073: A-021 Ed25519 verification semantics

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
- `S = L`, `S = L + 1`, and `S = 0` boundaries;
- non-decodable, non-canonical, and zero-`x`/sign-bit-invalid `R`;
- a cofactor-sensitive residue proving the uncofactored equation;
- mixed-torsion `A`/`R` and a wrong-message negative; and
- the #120 representation-invalid external-key precedence case.

Every vector binds exact key/input bytes, expected verdict/class, and (for
accepted chains) ordered hashes, count, and tip. A point/key substage result is
not a portable manifest result.

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

Stop for owner judgment before implementation if the owner rejects v1 D1/D2
acceptance, if a conformer needs an unreviewed dependency, if any frozen v1
history disagrees, or if a safer successor relation/name/migration has not been
ratified.

The complete design contract is
[`docs/design/a021-ed25519-verification-semantics-contract.md`](../docs/design/a021-ed25519-verification-semantics-contract.md).
