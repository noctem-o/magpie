# A-021 — Ed25519 signature-verification semantics contract

## Status and decision boundary

This is a narrow, documentation-only owner-decision contract for A-021. It
does not change Rust, Python, Go, `docs/FORMAT.md`, the release contract, the
audit ledger, fixtures, or the existing portable-verifier contract.

The reviewed baseline is `magpie-core-v1` at the post-PR #120 `origin/main`
commit. A-021 remains **Confirmed** until the owner ratifies the relation below
and a later implementation tranche proves it independently in Rust, Go, and
Python.

The contract has two deliberately different jobs:

1. freeze the compatibility interpretation of the already named v1 signature
   domain; and
2. prevent a safer, incompatible verification policy from being smuggled into
   v1. A stricter policy requires an additive successor profile and a new
   owner-ratified decision.

No signature result establishes identity, custody, trust, authorization, or
truth. The verifying key is supplied externally. Genesis only compares its
declared key with that external input; it is self-describing, not
self-authenticating.

## Evidence reconstructed

The following evidence is the decision surface, not an implementation
authority:

- `docs/FORMAT.md:160-186` freezes `magpie-core-v1`,
  `magpie-sig-v1 || hash`, canonical event bytes, external-key genesis
  binding, and the sequence/previous-link/hash/signature/genesis order.
- `docs/releases/v0.1.0-contract.md:64-72,278-289,386-390` freezes the
  existing signature domain and golden histories, while requiring a new
  profile for incompatible format meaning.
- `crates/magpie-log/src/logimpl.rs` constructs the message as
  `magpie-sig-v1 || hash` and currently calls `VerifyingKey::verify`.
- `tools/verify_chain.py:13,188-221` uses the same message and the current
  `cryptography` ordinary verifier.
- `Cargo.lock` pins `ed25519-dalek` 2.2.0. Its ordinary verifier is distinct
  from `verify_strict`: ordinary verification checks a canonical recomputed
  `R`, accepts weak points when the equation holds, and the default signature
  parser requires canonical `S`; `verify_strict` additionally rejects
  small-order `A` and `R`.
- The architecture audit's A-021 section records the canonical low-order root
  `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f`, for
  which the preserved Python forged-chain experiment is accepted; the current
  Rust path uses the same ordinary equation, while strict verification rejects
  it.
- A21-D2 records a second witness: the ordinary golden root with a
  keyholder-constructed signature whose `R` is the identity/small-order point
  and whose canonical `S` satisfies the ordinary equation. Ordinary accepts;
  strict rejects.

The witnesses prove that the disputed boundary is the complete verification
relation, not a classifier for unusual public keys. They are not an exhaustive
enumeration of Ed25519 edge cases.

## Scope retained from #120

The #120 external-key representation gate remains normative before signature
processing:

1. exactly 64 lowercase ASCII hex characters;
2. decode exactly 32 bytes;
3. interpret the bytes as compressed Edwards25519 `y` plus the `x` sign bit;
4. require the encoded `y` integer to be strictly less than
   `p = 2^255 - 19`;
5. require point recovery to succeed;
6. if recovered `x = 0`, require sign bit zero; and
7. require byte-identical canonical re-encoding.

This is representation validity only. It does not reject low-order or mixed
torsion points, establish trust, or select a signature-verification mode.

## Recommended v1 contract

The following is the compatibility-preserving recommendation for the named
`magpie-core-v1` profile. It makes the selected relation explicit without
rewriting canonical event bytes. It is a normative owner decision, not a
claim that every currently linked host library has already been exercised on
every newly required edge vector; the implementation tranche must prove that
each conformer realizes it.

### Message and algorithm

Magpie uses pure Ed25519, not Ed25519ctx or Ed25519ph, over the exact message:

```text
M = ASCII("magpie-sig-v1") || 32-byte content_hash
```

For a signature's 64 bytes, split:

```text
R_bytes = signature[0..32]
S_bytes = signature[32..64]
```

Let `A_bytes` be the exact canonical 32-byte external verifying-key encoding.
Let `A` and `R` be the points recovered from those encodings. Let `L` be the
Ed25519 basepoint order:

```text
L = 2^252 + 27742317777372353535851937790883648493
```

Decode `S_bytes` as a little-endian integer `S` and require:

```text
0 <= S < L
```

Compute:

```text
k = LE(SHA-512(R_bytes || A_bytes || M)) mod L
```

The v1 signature verifies exactly when:

```text
[S]B = R + [k]A
```

as equality of Edwards25519 group elements, where `B` is the standard
Ed25519 base point. This is the simple/uncofactored equation. V1 does **not**
multiply both sides by the cofactor.

### R encoding and point behavior

`R_bytes` must be one canonical compressed Edwards25519 encoding under the
same rules as the #120 point gate: canonical `y < p`, successful recovery,
the zero-`x` sign-bit rule, and byte-identical re-encoding. A malformed,
non-canonical, or non-decodable `R` is a `REJECT(Signature)` result after the
signature transport has already passed its exact-64-byte check.

The v1 equation permits a canonical small-order `R` when the equation holds.
That is intentional compatibility behavior, not an assertion that such a
signature is desirable or that its signer has authority.

### A handling, torsion, and weak roots

Any canonical point accepted by the #120 external-key gate is an admissible
input to the v1 relation, including small-order and mixed-torsion points.
V1 performs no `is_small_order`, subgroup, custody, or trust check. A
canonical low-order `A` may therefore satisfy the v1 equation for an
attacker-constructed signature. This is the behavior witnessed by A21-D1.

The supplied key remains an external parameter. A matching genesis field only
binds the record to that supplied byte string; it never makes the key trusted
or self-authenticating.

### S constraints

`S` is a canonical scalar, not an integer reduced modulo `L`. `S = L`,
`S > L`, and any other non-canonical 32-byte scalar are rejected at the
`Signature` stage. Accepting `S + L` would reintroduce signature malleability
and would disagree with RFC 8032's verification rule and the locked Rust
default. `S = 0` is representation-valid; it verifies only if the complete
equation holds.

### Required accepting edge constructions

The future corpus must prove both sides of the scalar and torsion boundaries;
negative cases alone are insufficient.

`A21-S4` is the canonical identity construction. Let `I` be the canonical
identity encoding
`0100000000000000000000000000000000000000000000000000000000000000`.
Use `A_bytes = I`, `R_bytes = I`, `S_bytes = 32 zero bytes`, and the exact
message
`M = ASCII("magpie-sig-v1") || 32 zero bytes`. The challenge is still computed
from `R_bytes || A_bytes || M`; its value is immaterial because `[k]I = I`.
Consequently `[0]B = I + [k]I = I`, so this is a complete equation-satisfying
`ACCEPT` vector. The future chain wrapper must bind the exact event bytes and
its ordered hash/count/tip; a key- or equation-substage result is not enough.

`A21-T2` is a genuinely mixed-torsion positive construction. Let `T4` be the
order-four point with canonical encoding
`0000000000000000000000000000000000000000000000000000000000000000`, and let
`B` be the standard base point. Use:

```text
A = B + T4
A_bytes = 5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea
R = B
R_bytes = 5866666666666666666666666666666666666666666666666666666666666666
S_bytes = a5dbeb5cd27780380b9c00153d328ef3399722dc9260abb20716a81cc62f510b
M = ASCII("magpie-sig-v1") || 32 zero bytes
```

For this exact message, `k =
5118885669828609414212241343031073060215324799265782113781910744455214521252`,
which is divisible by four, and the displayed scalar is `S = 1 + k` (strictly
less than `L`). Thus `[L]A = T4 != I` and `[4]A != I`, proving that `A` has
both a non-zero prime component and a non-zero torsion component rather than
being a purely small-order point, while `[S]B = B + [k]B = R + [k](B + T4)`
because `[k]T4 = I`. Both encodings decompress and byte-for-byte re-encode
canonically under #120. The future chain wrapper must bind the exact event
bytes and accepted summary fields.

`A21-T1` remains the equation-false mixed-torsion counterpart. Together the
two vectors prove that v1 decides by canonical representation plus the exact
simple equation, not by an unconditional mixed-torsion blacklist.

### Result boundary

Signature transport remains #120's exact 128 lowercase-hex-to-64-byte rule.
Once transport succeeds, point decoding, scalar range, and equation failure
all use the existing stable `Signature` rejection class. No new runtime
result such as `StrictnessSensitive` or `A021Required` is introduced.

The relation above is the v1 answer to “when does Magpie record that an
Ed25519 signature verifies?” It is deliberately not the host-library default:
each conformer must implement or constrain its library so that it realizes
this exact relation.

## Why v1 is compatibility, not a security-policy endorsement

The current Rust/Python path uses ordinary verification, and the two preserved
witnesses demonstrate accepted v1 bytes that strict verification rejects. A
retroactive switch to strict verification would change the accepted language
of a profile named `magpie-core-v1`, potentially invalidating already-valid
history. It would also change A21-D1 and A21-D2 from accepted compatibility
vectors to rejected vectors.

Therefore v1 keeps the simple relation above for historical verification. This
does not bless weak roots, small-order points, or untrusted signers; it records
what v1 means relative to the externally supplied key.

## Additive successor rule

If the owner requires rejection of weak/torsion points or a different
cofactor/strict equation, that change MUST NOT mutate v1. It requires an
additive successor profile, currently reserved as `magpie-core-v2` under the
existing FORMAT evolution rule:

- the successor has an explicit genesis profile identifier;
- it uses a distinct signature context, recommended as `magpie-sig-v2`, so
  the two verification relations cannot be confused by a shared signature
  domain;
- it specifies canonical `A`, canonical `R`, `S < L`, exact small-order and
  subgroup tests, and the exact equation (including whether any cofactor is
  multiplied);
- it starts a new chain and may carry the predecessor tip reference described
  by `docs/FORMAT.md`; and
- v1 readers retain the v1 relation for v1 histories.

This document does not ratify the v2 relation. Owner ratification is required
before any v2 implementation, writer change, or release claim. A stricter
successor SHOULD reject small-order `A` and `R` and define a prime-subgroup or
equivalent exact policy, but “strict” alone is not a mathematical
specification.

The version boundary is explicit for every disputed dimension:

| Dimension | `magpie-core-v1` | Successor requirement |
| --- | --- | --- |
| External `A` bytes | #120 canonical representation gate | May reuse the gate, but must name any stronger policy |
| `R` bytes | Canonical compressed point, exact re-encoding | Must remain explicit; no library default |
| `S` | Canonical little-endian `S < L` | Must be stated again; no modulo-`L` inference |
| Small-order `A` | Allowed when the simple equation holds | Owner chooses and names the rejection/subgroup rule |
| Small-order `R` | Allowed when the simple equation holds | Owner chooses and names the rejection/subgroup rule |
| Mixed torsion | Allowed when the exact equation holds | Owner chooses an exact subgroup/cofactor policy |
| Equation | `[S]B = R + [k]A`, no cofactor | Owner must state the exact equation and cofactor treatment |
| Signature domain | `magpie-sig-v1` | Distinct successor domain, recommended `magpie-sig-v2` |

## Cross-language law

Rust, Go, and Python are three conformers of this contract, not authorities
over it.

### Rust

The future Rust implementation must prove that its locked dalek path realizes
the explicit v1 checks above. Calling ordinary `verify` is acceptable only if
the conformance vectors prove canonical `A/R`, canonical `S`, the simple
equation, and v1 small-order outcomes. Calling `verify_strict` for v1 is not
conforming.

### Python

`cryptography` remains a readable independent reference implementation. It
must add explicit representation and scalar checks where the library API does
not expose the v1 boundary. Its ordinary `verify` result is evidence to test,
not the contract.

### Go

The future Go verifier remains an independently implemented release-oracle
candidate. The standard library's current classic verifier uses a simplified
equation and documents edge behavior outside the Go 1 compatibility promise;
those defaults cannot define Magpie. Go must either implement the v1 relation
explicitly or prove its wrapper has exactly the same behavior. No Rust/Python
FFI, shared verifier library, or dependency is authorized by this contract.

All three must agree on the exact message, little-endian encodings, `S < L`,
canonical `R`, simple equation, and v1 small-order outcomes. They may use
different diagnostic prose, but not different verdicts or signature-stage
coordinates.

## Minimal hostile vector family

The future exact-byte corpus must include at least the following semantic
families. Each vector binds the exact external-key bytes, exact message or
event bytes, expected v1 verdict, and observed Rust/Python/Go results.

| ID | Purpose | Required v1 result |
| --- | --- | --- |
| `A21-P0` | RFC 8032 ordinary positive plus the frozen golden and Deadbolt signatures | `ACCEPT` |
| `A21-S1` | `S = L` | `REJECT(Signature)` |
| `A21-S2` | `S = L + 1` or another non-canonical in-range-encoding mutation | `REJECT(Signature)` |
| `A21-S3` | `S = 0` boundary with a non-matching equation | `REJECT(Signature)` |
| `A21-S4` | canonical identity `A`, canonical identity `R`, `S = 0`, and the matching equation | `ACCEPT` |
| `A21-R1` | `R` has no point decompression | `REJECT(Signature)` |
| `A21-R2` | `R` is a non-canonical encoding of an otherwise recoverable point | `REJECT(Signature)` |
| `A21-R3` | `x = 0` with sign bit one | `REJECT(Signature)` |
| `A21-D1` | canonical low-order `A` plus the preserved forged chain | `ACCEPT` under v1 |
| `A21-D2` | ordinary golden `A` plus the preserved identity/small-order `R` signature | `ACCEPT` under v1 |
| `A21-C1` | a cofactor-sensitive residue that satisfies a cofactored equation but not the simple equation | `REJECT(Signature)` |
| `A21-T1` | a mixed-torsion `A` or `R` with a deliberately non-matching equation | `REJECT(Signature)` |
| `A21-T2` | the exact mixed-torsion `A = B + T4` construction above with the matching simple equation | `ACCEPT` |
| `A21-M1` | correct signature bytes under the wrong `magpie-sig-v1 || hash` message | `REJECT(Signature)` |
| `A21-K1` | #120 representation-invalid external key, before any signature work | `REJECT(ExternalKey)` |

`A21-D1` and `A21-D2` are mandatory named witnesses. They are not an
exhaustive classifier; the vector family must not be reduced to those two
examples. After v1 ratification they migrate from #120 dependency metadata to
normative v1 vectors with the `ACCEPT` result shown above. A future v2 corpus
may assign different results only under that explicitly versioned relation.

The implementation tranche must generate and review exact bytes, not
re-serialize parsed values. Every vector must be run through the real Rust
reader, `tools/verify_chain.py`, and the independent Go verifier. A vector is
not complete when only the key or point substage was tested.

Before implementation is called a compatibility freeze, the conformance run
must record the actual result of `A21-S4` and `A21-T2` from all three locked
implementations. The locked Rust source-level ordinary path computes the
uncofactored equation and has no `S = 0` or mixed-torsion blacklist; the Python
reference probe must exercise the exact bytes above. If any host rejects a
mathematically accepted construction, record that as implementation divergence
and stop for owner judgment rather than silently changing the v1 relation or
calling the host default normative.

## Compatibility analysis

Preserved without change:

- all `magpie-core-v1` canonical EventCore bytes;
- the `magpie-sig-v1 || content_hash` message domain;
- the golden and Deadbolt fixture bytes, hashes, signatures, event counts and
  tips;
- the external trust-root input and genesis equality rule; and
- JSONL's status as replaceable reference transport rather than canonical
  identity.

The v1 relation may reject malformed or non-canonical signature encodings that
were never valid under the current Rust ordinary path. If an implementation
probe finds a previously accepted v1 byte outside the stated relation, stop:
do not silently narrow v1; add a compatibility vector or use the successor
profile rule.

## Rejected alternatives

1. **Make `verify_strict` v1 normative.** Rejected because it retroactively
   changes the accepted language witnessed by A21-D1/A21-D2 and may invalidate
   existing v1 history. Use a successor profile.
2. **Make whichever library currently agrees normative.** Rejected because
   Rust, Python, and Go expose materially different edge behavior; Go itself
   does not promise stable edge rules under its general compatibility promise.
3. **Multiply by the cofactor in v1.** Rejected because RFC 8032 permits but
   does not require that equation, and it would change the low-order witness
   outcomes without a version boundary.
4. **Reject every weak key at the #120 input gate.** Rejected because it
   confuses representation with trust/policy, would not by itself settle
   small-order `R`, and risks changing v1 accepted roots.
5. **Reduce non-canonical point encodings modulo `p`.** Rejected by the #120
   canonical representation law and by cross-language reproducibility.
6. **Accept `S` modulo `L`.** Rejected as malleable and contrary to the
   canonical scalar rule.
7. **Treat the genesis key as authentication or identity.** Rejected by the
   FORMAT and release trust model.
8. **Define A-021 as only D1/D2 exceptions.** Rejected; both are witnesses,
   not a complete relation.

## Future implementation obligations

The implementation tranche must, before any A-004/RQ-006 closure claim:

1. freeze exact v1 vector bytes and a manifest containing profile, key, input,
   expected verdict, failure class/coordinate, and accepted history summary;
2. implement the canonical A/R and `S < L` checks independently in Rust,
   Python, and Go;
3. prove the v1 simple equation and no-cofactor behavior, including D1, D2,
   `A21-S4`, `A21-C1`, `A21-T1`, and the accepting `A21-T2` mixed-torsion
   construction;
4. run the real production Rust path, the real Python verifier, and the
   standalone Go binary against the same bytes;
5. preserve the frozen fixtures byte-for-byte and prove their counts/tips and
   ordered hashes;
6. pin the dependency/toolchain versions used for the conformance run and
   review any cryptographic-library upgrade as a semantic change;
7. stop before adding a dependency if a conformer cannot realize the relation
   without one; and
8. keep A-021 distinct from trust-root ownership, identity, authority,
   admission, resource, and storage policy.

## Owner stop conditions

Owner judgment is required before implementation if any of the following is
true:

- the owner does not accept v1's explicit acceptance of D1/D2;
- the owner does not accept the explicit equation-satisfying `A21-S4` or
  `A21-T2` edge vectors;
- a locked Rust, Python, or Go implementation rejects a mathematically
  equation-satisfying `A21-S4` or `A21-T2` construction; record the divergence
  and stop rather than silently narrowing the relation;
- Rust, Python, or Go cannot realize the v1 relation without an unreviewed
  dependency or undocumented host-library behavior;
- a frozen or already-published v1 history fails the relation;
- a proposed strict/cofactor/subgroup rule would change a v1 verdict;
- a successor profile name, signature context, predecessor-chain migration, or
  coexistence rule is needed but not ratified; or
- a reviewer proposes using key presence as trust, identity, authorization,
  or self-authentication.

Until those decisions are resolved, do not change runtime code, FORMAT,
fixtures, the v0.1.0 contract, or the living audit disposition.

## Target law

For `magpie-core-v1`, after #120's canonical external-key and signature
transport gates, “signature verifies” means the exact canonical `A`/`R`,
canonical `S < L`, SHA-512 challenge, and simple uncofactored Edwards equation
specified above. Small-order or mixed-torsion points are not implicitly
rejected, and the supplied key is never thereby trusted. Any safer relation is
an additive, explicitly identified successor—not a silent v1 reinterpretation.
