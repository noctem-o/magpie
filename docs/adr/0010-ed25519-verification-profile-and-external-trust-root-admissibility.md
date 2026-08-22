# ADR-0010: Ed25519 Verification Profile and External Trust-Root Admissibility

**Status:** Accepted (explicitly owner-ratified, 2026-08-21)

## Summary / Y-statement

In the context of `magpie-core-v1` histories signed with ordinary Ed25519 over
`"magpie-sig-v1" || content_hash`, facing host libraries that accept
structurally weak and non-canonical material differently and a current accepted
language that includes weak-root witnesses, we decide one additive, explicitly
selected Ed25519 signature-verification subprofile:

```text
V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"

VerifySignature_{V_sig}(A_bytes, signature, M)
```

`V_sig` requires canonical compressed Edwards25519 encodings, a non-identity
prime-subgroup public key `A`, a prime-subgroup signature point `R` for which
the canonical identity is permitted, a canonical scalar `0 <= S < L`, the
ordinary Ed25519 SHA-512 challenge, and the exact uncofactored equation
`[S]B = R + [k]A`.

`V_sig` is a cryptographic semantic dependency, not the complete derivation
profile `G` defined by ADR-0008. A complete history-verification derivation
profile must also commit to transport interpretation, canonical EventCore
interpretation, chain ordering, typed semantic failures, external-root
handling, use of `V_sig`, and successful-result meaning. This ADR deliberately
does not define that complete history profile.

The subprofile is selected outside the history. It does not change
`magpie-core-v1` canonical bytes, the `magpie-sig-v1` message, existing
signature bytes, or the current unprofiled Rust/Python verifier behavior. It
must be selected explicitly, with no ambient default, alias, negotiation,
fallback, or downgrade. A result intended to be portable must retain the exact
subprofile identity directly or through an immutable, unambiguous commitment
inside its complete ADR-0008 producing context; that identity alone is not a
complete history-verification coordinate set.

Existing unprofiled verification remains compatibility-only behavior. It is
not silently renamed as `V_sig`, and `V_sig` is not declared to be the
historical meaning of `magpie-core-v1`. This is what prevents the decision from
hiding an accepted-language break behind unchanged serialized bytes.

This is an Accepted constitutional decision, explicitly owner-ratified on
2026-08-21. The owner accepted the exact relation merged by PR #130; no
implementation, corpus, or finding closure is implied by ratification.

## Authority and relationship to existing material

`docs/FORMAT.md` remains the authority for `magpie-core-v1` bytes, the
`magpie-sig-v1` message, genesis key declaration, and the externally supplied
trust-root boundary. This ADR does not amend FORMAT.

Accepted ADR-0008 remains the authority for complete producing coordinates and
for the constitutional meaning of derivation-profile symbol `G`. This ADR's
`V_sig` names only the frozen Ed25519 signature subrelation. It neither
redefines `G` nor claims that `V_sig` alone identifies a complete
history-verification derivation or result.

The following documents are evidence and design inputs, not authority capable
of silently amending FORMAT or ratifying this ADR:

- `docs/design/portable-verifier-input-language-contract.md` defines the
  canonical external-input gate;
- `docs/design/a021-ed25519-verification-semantics-contract.md` reconstructs a
  portable permissive relation and supplies concrete weak/torsion witnesses;
- `docs/audits/magpie-architecture-audit-2026-08-01.md` preserves the historical
  A-021 finding; and
- `docs/audits/audit-disposition-2026-08.md` remains the living administrative
  ledger, where A-021 stays Confirmed.

This Accepted ADR supersedes the design contract's recommendation to make
its permissive relation the portable public meaning of v1. It does not erase
that contract, change the historical audit, or retroactively change a verdict
returned by an existing unprofiled verifier.

## Context and problem

Magpie fixes the bytes signed by each event:

```text
M = ASCII("magpie-sig-v1") || content_hash
content_hash = SHA-256(canonical EventCore bytes)
```

The caller supplies the verification key. Genesis records the same key bytes
and verification compares them, making the history self-describing but not
self-authenticating.

That contract does not by itself select all Ed25519 acceptance semantics.
Different implementations can disagree about:

- which 32-byte public-key encodings decode;
- canonical compressed-point representation;
- identity, small-order, and mixed-torsion public keys;
- canonicality and subgroup membership of signature `R`;
- scalar `S` range and reduction;
- cofactored versus uncofactored verification equations; and
- additional checks hidden behind library terms such as `strict`.

Those choices determine the accepted history language even when EventCore and
signature bytes do not change. A portable verifier cannot inherit them from an
unversioned host-library default.

## Scope

This ADR decides:

1. one exact Ed25519 signature-verification subprofile identity;
2. its public-key, signature, challenge, and equation relation;
3. its immutable-identity, explicit-selection, and ADR-0008 composition law;
4. its compatibility relationship to existing unprofiled v1 verification;
5. its eligibility for the narrow candidate portable public pre-alpha path;
   and
6. the recorded owner decision and post-ratification implementation sequence.

It does not implement the profile, ratify a corpus, or decide that any supplied
key is trusted.

## Definitions and fixed parameters

All integer decoding in this section is little-endian.

```text
p = 2^255 - 19

d = -121665 / 121666 mod p

E = { (x, y) in F_p^2 : -x^2 + y^2 = 1 + d*x^2*y^2 }

L = 2^252 + 27742317777372353535851937790883648493

B = the RFC 8032 Ed25519 base point of order L
enc(B) = 5866666666666666666666666666666666666666666666666666666666666666

I = (0, 1), the Edwards identity
```

`[n]P` means scalar multiplication of point `P` by integer `n` in `E`.
`H` means SHA-512. `mod L` means reduction of the decoded non-negative
512-bit integer modulo `L`.

The external algorithm reference is
[RFC 8032](https://www.rfc-editor.org/rfc/rfc8032). Where this ADR is stricter
than RFC 8032's permitted verification choices, the explicit gates below win.

### Canonical compressed Edwards25519 decoding

For an exact 32-byte string `enc(P)`:

1. the low 255 bits encode `y`; the high bit encodes the parity of `x`;
2. require `0 <= y < p`; no reduction of `y` modulo `p` is allowed;
3. recover `x` from the Edwards25519 equation and reject if no field square
   root exists;
4. select the root whose parity equals the encoded high bit;
5. when `x = 0`, require the encoded high bit to be zero; and
6. re-encode the recovered point and require byte-for-byte equality with the
   original 32 bytes.

This definition rejects non-canonical encodings such as `y >= p` and the
identity encoded with the `x` sign bit set. It does not use ZIP-215 reduction
or a host decoder's undocumented acceptance set.

### Prime-subgroup membership and public-root non-identity

A decoded point `P` is in the prime-order subgroup exactly when:

```text
[L]P = I
```

This predicate includes `I`. It otherwise admits only points in the subgroup
generated by `B` and rejects every point with a non-zero torsion component,
including non-identity pure small-order points and mixed-torsion points.

The externally supplied public key has the additional structural-admissibility
requirement `A != I`. Signature point `R` does not: canonical `R = I` remains
admissible when the complete verification equation holds. Checking only
`[8]P != I`, or only a library's `is_small_order` predicate, is not equivalent
to the prime-subgroup predicate.

### Signature-verification subprofile and ADR-0008 composition

`signature-verification subprofile` means the exact cryptographic relation
identified here by:

```text
V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"
```

It is a semantic dependency of a complete history verification, not an
ADR-0008 derivation profile `G` by itself. In ADR-0008 notation, an enclosing
history verifier has a relation of the form:

```text
D_history = Derive_{G_history}(I_{G_history})
```

`G_history` must immutably commit to the complete history semantics, including
how this signature subprofile is invoked and how its failures affect history
verification. If the caller selects `V_sig`, its exact identity is a member of
`I_G_history`; if a future `G_history` fixes it, that immutable dependency is
incorporated into `G_history`. Either packaging must preserve the same complete
context and may not resolve `V_sig` through an ambient or mutable lookup. This
ADR does not select, name, or otherwise broaden into that future complete
history-verification profile.

Accordingly, a detached portable history result must carry or be immutably
bound to its complete `(G_history, I_G_history)` under ADR-0008. The selected
`V_sig` must remain identifiable in that complete producing context, either as
a direct member of `I_G_history` or transitively through an immutable,
unambiguous `G_history` commitment to exactly that subprofile. Redundant direct
carriage is not required when `G_history` provides that commitment. Retaining
only `V_sig`, without the remaining complete producing context, is not
sufficient.

The signature subprofile is distinct from:

- the `magpie-core-v1` canonicalization profile carried in Genesis;
- the `magpie-sig-v1` bytes prepended to the content hash;
- the external verification key itself; and
- any decision to trust, authorize, or identify that key.

`unprofiled v1 verification` means the current Rust or Python path that invokes
its host library without an explicit Magpie signature-subprofile identity.

### Immutable subprofile identity law

The exact identifier
`magpie-ed25519-canonical-prime-subgroup-v1` names exactly the frozen relation
in this ADR forever. Its meaning may not be repointed by a registry, living
document, release annotation, alias, deployment setting, or implementation
default.

Any semantic change to any of the following requires a
distinct subprofile identity and a separate owner decision:

- accepted `A_bytes` length, compressed-point decoding, or public-key
  structural admissibility;
- signature length or `R || S` layout;
- accepted `R` representation, subgroup law, or identity treatment;
- the scalar `S` range or decoding law;
- message or challenge construction;
- the verification equation, cofactor treatment, or per-signature batch
  equivalence law; or
- fail-closed selection and rejection semantics owned by this subprofile.

There is no mutable registry entry, `latest`, negotiated substitute, fallback,
or compatibility alias that may make another relation answer to this
identifier. A conforming implementation may change algorithms, language,
library, or internal representation while preserving identical semantics;
those implementation identities are assurance provenance, not new
cryptographic semantic subprofiles.

This identity law freezes the cryptographic byte relation. It does not make a
particular JSON, CLI, hexadecimal, or other transport spelling part of
`V_sig`. The enclosing frozen input-language or transport profile owns how it
produces the exact `A_bytes` consumed here, including lexical failures and
failure ordering. A semantic change at that layer requires the appropriate new
input-language or transport identity, but it does not require a new `V_sig`
when the same exact `A_bytes` reach the unchanged cryptographic relation.

## Current-behavior characterization

This characterization was repeated against repository baseline
`2e3178bcb3e63e78ea729ff1873024ada91e5ac2`. It records evidence; it is not the
normative relation.

### Rust path

`crates/magpie-log/src/logimpl.rs` constructs
`"magpie-sig-v1" || content_hash` and calls
`ed25519_dalek::VerifyingKey::verify`.

`Cargo.lock` selects:

- `ed25519-dalek` 2.2.0;
- `curve25519-dalek` 4.1.3;
- `ed25519` 2.2.3;
- `signature` 2.2.0; and
- `sha2` 0.10.9.

The selected dalek feature graph does not enable `legacy_compatibility`.
Inspection of the locked source establishes:

- `VerifyingKey::from_bytes` uses ZIP-215 point validation rather than RFC
  8032/NIST canonical point validation;
- the default signature parser requires the decoded little-endian `S` to be
  strictly less than `L`;
- ordinary verification computes
  `k = H(R_bytes || A_bytes || M) mod L`, computes
  `-[k]A + [S]B`, compresses it, and compares that encoding byte-for-byte with
  `R_bytes`;
- that byte comparison rejects non-canonical `R`, while ordinary verification
  does not reject a weak `A` or weak `R` when the equation holds; and
- `verify_strict` adds only `is_small_order` rejection for decoded `A` and `R`
  before using the same equation. It is not a full prime-subgroup test and
  accepts the equation-satisfying A21-T2 mixed-torsion construction.

The temporary Rust executable probe did not link in this environment because
the installed MSVC toolchain could not locate `msvcrt.lib` (`LNK1104`). The
Rust statements above are therefore locked-source characterization, not a new
executed Rust conformance result. No Rust behavior is claimed to have passed in
this tranche.

### Python path

`tools/verify_chain.py` uses `Ed25519PublicKey.from_public_bytes` and
`Ed25519PublicKey.verify`. Its public CLI lowercases the supplied hexadecimal
key before its current length/hex check. CI installs `cryptography` without a
version pin.

The active characterization environment was:

```text
Python       3.11.15
cryptography 50.0.0
backend      OpenSSL 4.0.1 9 Jun 2026
```

The tagged `cryptography` source delegates raw public-key construction and
ordinary verification to OpenSSL. The tagged OpenSSL source checks `S < L`,
decodes `A`, computes the ordinary SHA-512 challenge and uncofactored equation,
re-encodes the computed point, and compares those 32 bytes with `R_bytes`. It
does not impose a Magpie canonical-key or prime-subgroup gate.

An out-of-tree/in-memory probe of the active backend established:

| Hostile class | Active Python/OpenSSL result |
| --- | --- |
| ordinary frozen key/signature | accept |
| 31- or 33-byte public key | reject construction |
| 63- or 65-byte signature | reject |
| canonical identity `A`, identity `R`, `S = 0` (A21-S4) | accept |
| canonical identity `A`, `R = -B`, `S = L - 1` (A21-S5) | accept |
| ordinary `A`, identity `R`, equation true (A21-D2) | accept |
| mixed-torsion `A = B + T4`, equation true (A21-T2) | accept |
| non-canonical identity `A`, equation true | accept |
| non-canonical `y = p + 1` encoding of `A`, equation true | accept |
| non-canonical identity or `y = p + 1` encoding of `R` | reject |
| non-decompressing `R` | reject |
| `S = L` or `S = L + 1` | reject |
| altered message/hash or altered signature | reject |

The same probe exercised `tools/verify_chain.py`'s real `verify_event` path on
the complete A21-S4, A21-S5, and A21-T2 Genesis records. All three were
accepted.

The evidence can be compared directly as follows. Rust cells are derived from
the locked 2.2.0 source and the stated equations; Python cells are active probe
results.

| Witness | Rust ordinary `verify` | Rust `verify_strict` | Python/OpenSSL ordinary |
| --- | --- | --- | --- |
| ordinary frozen signature | accept | accept | accept |
| A21-S4 / A21-S5: identity `A` | accept | reject small-order `A` | accept |
| A21-D2: ordinary `A`, identity `R` | accept | reject small-order `R` | accept |
| A21-T2: mixed-torsion `A`, equation true | accept | accept | accept |
| non-canonical identity `A`, equation true | accept under ZIP-215 decode | reject because decoded `A` is small-order | accept |
| non-canonical equation-equivalent `R` | reject byte comparison | reject byte comparison | reject byte comparison |
| `S = L` or `S = L + 1` | reject | reject | reject |

### Confirmed agreement and remaining uncertainty

The locked Rust ordinary source and active Python/OpenSSL path agree on the
tested equation-sensitive witnesses: both use canonical `S`, canonical
recomputed `R`, the uncofactored equation, and accept equation-satisfying
small-order or mixed-torsion material. This is not evidence that their entire
accepted languages are identical. Their public-key decoders are independently
implemented, Python's dependency is unpinned in CI, and no exhaustive shared
corpus exists.

The term `strict` is particularly unsafe as doctrine. Dalek's
`verify_strict` rejects pure small-order `A` and `R` but does not reject all
mixed-torsion points; Python exposes no corresponding Magpie profile. Calling
one library function would therefore not define the relation selected here.

All scratch probes were characterization only and were removed. This ADR adds
no conformance corpus.

## Options considered

### Option A — freeze permissive v1 semantics

Specify the current uncofactored equation and allow weak/torsion points when it
holds, leaving stronger external-root rejection to caller policy.

This is mathematically specifiable, and the A-021 design contract supplies
useful complete witnesses. It is not the recommended public profile because:

- exact compatibility would also have to freeze the current permissive
  public-key decoding language, not quietly add a canonical-key gate;
- the two current libraries have no proven exhaustive agreement over that
  decoder language;
- a pure small-order externally supplied root can admit signatures without the
  ordinary private-key relation a caller is likely to assume; and
- making weak-root rejection optional caller policy leaves the portable public
  boundary dependent on whether every caller remembered a separate gate.

The current behavior may remain compatibility-only without being promoted to a
portable public profile.

### Option B — tighten `magpie-core-v1` in place

Declare the selected relation to be what `magpie-core-v1` always meant and
change both existing verifiers.

Rejected. A21-S4, A21-S5, A21-D1, and A21-T2 demonstrate inputs accepted by
current ordinary verification that the selected relation rejects. The
non-canonical-key probe adds another accepted class. A21-D2 is not in that
rejected set under the revised relation because canonical identity `R` remains
admissible. Unchanged EventCore bytes do not make the remaining
accepted-language narrowing compatible.

### Option C — preserve core-v1 bytes and add the explicit canonical-prime-subgroup profile

Keep existing unprofiled behavior unchanged, define `V_sig` independently,
require explicit selection and complete ADR-0008 context carriage, and make
only `V_sig` eligible as the Ed25519 subprofile of a future portable public
verification claim.

**Recommended.** This creates a portable fail-closed boundary without
retroactively reclassifying existing verifier outcomes. It is a genuine
compatibility separation only if implementations preserve both sides: no
implicit upgrade of old calls, no fallback from `V_sig`, and no result that
omits the selected subprofile from its complete producing context.

### Option D — additive future core/signature version

Create a new canonicalization profile, new Genesis field meaning, new signature
domain such as `magpie-sig-v2`, and possibly a new chain.

This is defensible if the verification-profile identity must be signed and
self-described by the chain, or if cross-profile signature reuse must be
cryptographically impossible. It is not required for the current narrow goal:
the external key is already a caller-supplied verification coordinate, and an
explicit external verification profile can be selected alongside it without
changing any signed bytes. Forcing a new chain here would expand the format and
migration problem without improving the stated external-trust boundary.

### Option E — leave verification unchanged and add only a weak-key caller gate

Rejected. It does not settle `R`, scalar, equation, batch, or mixed-torsion
semantics, and it preserves the risk of callers omitting or varying the gate.

## Normative relation

A conformer implements
`V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"` exactly as follows.

### 1. Explicit profile selection

The complete producing context of an enclosing history verifier explicitly and
immutably identifies exactly one signature-subprofile identity. This
identification is direct when the caller supplies `V_sig` as a member of
`I_G_history`, or transitive when immutable `G_history` itself commits to
exactly `V_sig`; the caller need not supply `V_sig` separately in the latter
case. The external verification key remains an explicit input.

An unknown directly supplied subprofile identity fails closed before signature
verification. There is no ambient, empty, `latest`, legacy alias,
library-default, negotiated, automatically selected, fallback, or
try-strict-then-permissive mode.

The exact `V_sig` identity must remain identifiable in the complete producing
context of any portable verification outcome. A detached Boolean or summary is
portable only when it carries, or is immutably and unambiguously bound to, the
complete `(G_history, I_G_history)` that identifies the selected `V_sig`.
`V_sig` may be identified directly through `I_G_history` or transitively
through an immutable `G_history` commitment; the latter does not require a
redundant direct field. A value that carries only `V_sig` but omits the
remaining complete producing context is still not coordinate-complete under
ADR-0008.

The subprofile identity is not a new field in `SignedEvent` or Genesis. Adding
such a field would be separate core-format work.

### 2. Cryptographic public-key byte representation and admissibility

The cryptographic key input to `V_sig` is exactly 32 bytes `A_bytes`. `V_sig`
begins after an enclosing input-language or transport profile has produced
those exact bytes; it does not define their JSON, CLI, hexadecimal, binary, or
other external representation.

For the current reference portable JSONL path, the portable-verifier
input-language contract independently requires exactly 64 lowercase ASCII
hexadecimal characters, with no prefix, whitespace, case folding, or alternate
encoding, and decodes them to `A_bytes` before this subprofile is invoked. This
ADR preserves that #120 lexical gate by cross-reference; it neither weakens nor
absorbs it. A future binary or other transport may supply the same exact
`A_bytes` under its own frozen semantics without requiring a new `V_sig`.

Decode `A_bytes` using canonical compressed Edwards25519 decoding above.
Reject any failure. Then require:

```text
A != I
and
[L]A = I
```

There is no reduction of a non-canonical `y`, no acceptance of the alternate
sign bit for `x = 0`, no identity key, no other small-order key, and no key with
a non-zero torsion component.

This is structural admissibility only. It does not make `A` trusted.

### 3. Signature representation and admissibility

The signature is exactly 64 bytes:

```text
signature = R_bytes || S_bytes
len(R_bytes) = 32
len(S_bytes) = 32
```

Decode `R_bytes` using the same canonical compressed Edwards25519 decoder.
Reject any failure. Then require:

```text
[L]R = I
```

Canonical `R = I` is permitted. Every non-identity small-order `R` and every
mixed-torsion `R` is rejected by `[L]R = I`.

Decode `S_bytes` as a non-negative little-endian integer `S` and require:

```text
0 <= S < L
```

Do not reduce `S` modulo `L`. Do not accept a high-bit-only approximation such
as `S < 2^253` or the narrower, incorrect `S < 2^252` rule.

#### Why canonical identity `R` is permitted

RFC 8032 signing computes a nonce scalar from SHA-512, reduces it modulo `L`,
and sets `R = [r]B`. The reduced nonce can in principle be zero, so an ordinary
standards-conforming signer can produce canonical `R = I`, with vanishing
probability under the hash model. RFC 8032 decoding and either permitted
verification equation add no `R != I` gate.

For admissible `A = [a]B`, identity `R` reduces the uncofactored equation to
`[S]B = [k]A`. Producing `S = k*a mod L` still requires the signing scalar (or
breaking the ordinary discrete-log/preimage assumptions). A21-D2 is exactly a
deliberate keyholder construction of that equation-valid form. It demonstrates
ordinary-versus-small-order-rejecting library divergence; it is not evidence
that this subprofile admits a non-keyholder forgery.

The identity is the neutral element of the prime-order subgroup. Canonical
decoding, `[L]R = I`, prime-subgroup non-identity `A`, canonical `S`, and the
exact uncofactored equation leave no torsion or mixed-torsion ambiguity merely
because `R = I` is allowed. A standards-conforming signer that actually
encounters zero nonce can expose its signing scalar when `k != 0`, but rejecting
the already-published signature cannot undo that signer-side disclosure or
prevent subsequent ordinary-looking forgeries. Magpie therefore has no
concrete security or interoperability reason to exclude this otherwise valid
signature from `V_sig`.

This reasoning does not weaken the public-key rule. `A != I` remains a
structural-admissibility requirement for the externally supplied verification
root.

### 4. Message and challenge

For each event, preserve the existing message exactly:

```text
content_hash = SHA-256(canonical magpie-core-v1 EventCore bytes)
M            = ASCII("magpie-sig-v1") || content_hash
k            = LE_INT(SHA-512(R_bytes || A_bytes || M)) mod L
```

This is ordinary, pure Ed25519 over `M`. It is not Ed25519ctx, Ed25519ph, a
second prehash mode, or an RFC 8032 context string. Magpie's byte prefix is part
of `M`; it is not an Ed25519ctx parameter.

### 5. Equation

Accept the signature if and only if all prior gates succeed and:

```text
[S]B = R + [k]A
```

At the subprofile boundary, any failed gate or false equation is a fail-closed
signature rejection. The enclosing `G_history` must commit to the typed
history-verification failure class, ordering, and coordinate that carries that
rejection. An operational inability to complete evaluation produces no
semantic verdict under ADR-0008; it is neither acceptance nor a substitute
profile result.

This is the simple, uncofactored equation. A batch verifier must produce the
same per-signature verdicts as this individual relation; a probabilistic batch
equation or library batch default may not change the accepted language.

Because admissible `A` and `R` are in the prime-order subgroup, multiplying
both sides by the cofactor does not change the verdict. The uncofactored form is
nevertheless normative so independent implementations need not infer which
equation to use.

### 6. RFC 8032 classification

The signature primitive and challenge are ordinary RFC 8032 Ed25519. The
acceptance relation is a stricter subset: it adds canonical representation and
non-identity/full-prime-subgroup requirements for `A`, a full prime-subgroup
requirement for `R`, and the RFC-permitted simple equation. Unlike dalek
`verify_strict`, it permits canonical identity `R` when that equation holds. It
must not be described merely as “RFC 8032 strict,” because that phrase does not
identify these exact gates.

## Compatibility consequences

| Compatibility dimension | Consequence |
| --- | --- |
| Canonical byte compatibility | `SignedEvent`, EventCore, `magpie-core-v1` canonical bytes, hashes, and Genesis fields are unchanged. |
| Signature byte compatibility | Existing ordinary writers still sign the same `"magpie-sig-v1" || content_hash` bytes and produce the same signatures. The `R` rule admits every canonical prime-subgroup point such a signer can emit, including the theoretical zero-nonce `R = I` case. |
| Accepted-language compatibility | `V_sig` is a strict subset of current unprofiled behavior. Non-canonical keys, identity/other small-order keys, mixed-torsion keys, non-subgroup `R`, and other inputs outside the exact relation are rejected by `V_sig`. Canonical identity `R` is permitted when the equation holds. Existing unprofiled verdicts are not changed. |
| Golden compatibility | An independent scratch implementation of the selected relation accepted all 9 frozen golden records and both Deadbolt anchor records. No fixture bytes were changed. |
| Profile identity compatibility | The stricter relation cannot honestly be called the meaning of `magpie-core-v1` alone. It requires the additive external identity `magpie-ed25519-canonical-prime-subgroup-v1`. |

The frozen inputs checked above were:

```text
crates/magpie-log/testdata/golden-v1.jsonl
SHA-256 767ef86b7e2ad1d65e8315ae28bac682bceee29223b5f1a0612ec7ef062cffb4

fixtures/deadbolt-anchor-v1/anchor-log.jsonl
SHA-256 2c715006e3e5bef571b3d31b95f47d3c4bbb3ea68fa9750365b4a62b66cd41e5
```

The selected public-key gate rejects A21-S4, A21-S5, A21-D1, and A21-T2 at
public-key admissibility. Independent re-evaluation of a concrete A21-D2
construction over the unchanged golden Genesis bytes gives `ACCEPT` under
`V_sig`: its ordinary golden `A` is non-identity and prime-subgroup, its
canonical `R = I` is prime-subgroup, its `S` is canonical, and the
uncofactored equation holds. These classifications are intentional profile
consequences, not claims that any rejected bytes were malformed under the old
path.

### Coexistence and migration consequence

As ratified:

- existing unprofiled Rust/Python APIs retain their current behavior and are
  labelled compatibility-only, not portable `V_sig` verification;
- new APIs are additive and require the exact immutable subprofile identity;
- a history may be evaluated under more than one explicitly named relation and
  may receive different verdicts without either result silently rewriting the
  other;
- any portable result is bound to its complete ADR-0008 producing context,
  including direct or transitive immutable identification of the exact `V_sig`,
  external key bytes or their immutable identity, exact supplied-history
  identity, and the enclosing complete history-verification profile and other
  semantic inputs;
- there is no automatic migration, fallback, or reinterpretation of an old
  success as a `V_sig` success; and
- a future requirement that the chain itself sign or declare the verification
  profile reopens Option D and requires separate owner-ratified format work.

No successor core profile or signature domain is required by this decision.
The additive boundary is the verification profile, not `magpie-core-v2`.

## Security and epistemic consequences

The decision removes library-default ambiguity from the candidate portable
boundary and rejects structural weak-root and torsion witnesses before their
equations can produce a success. Full subgroup checks are more exact than
dalek's `verify_strict` small-order test and are deliberately reviewable by an
independent implementer.

That improvement is narrow. Signature-subprofile success establishes only:

> These exact signature bytes satisfy
> `magpie-ed25519-canonical-prime-subgroup-v1` for this exact message under
> this exact externally supplied verification key.

An enclosing history verifier may make only the separate conclusion defined by
its complete `G_history` over its complete `I_G_history`, which must include or
immutably incorporate that exact subprofile and key. `V_sig` alone does not
define the history-level conclusion or make a detached result
coordinate-complete.

It does not establish:

- who owns or controls the key;
- that the key is trustworthy;
- identity, authorship, custody, delegation, authority, or admission;
- that an event or claim is true;
- freshness, expected head, latest history, canonical history, global
  completeness, durability, rollback resistance, or non-equivocation;
- permission to act; or
- that another verification profile would return the same verdict.

Genesis still only binds the supplied key bytes into the self-described
history. It does not bootstrap trust in them. No PKI, certificate, Web of Trust,
DID, ambient key discovery, or key-rotation authority is introduced.

## Implications for the portable corpus and conformers

This ADR intentionally contains no permanent vector files. The first
post-ratification tranche must be the **exact normative portable
corpus/profile manifest**.

That manifest must identify `V_sig` exactly, bind it into the complete
history-verification case context, and freeze, at minimum:

- ordinary valid key/signature and all existing golden inputs;
- malformed key and signature lengths;
- canonical and non-canonical public-key encodings;
- identity, other small-order, and mixed-torsion public keys;
- non-decompressing and non-canonical `R`; canonical identity `R` with both an
  equation-true `ACCEPT` and equation-false `REJECT`; and non-identity
  small-order and mixed-torsion `R` rejections;
- `S = 0`, `S = L - 1`, `S = L`, `S = L + 1`, and other out-of-range values;
- equation-true and equation-false witnesses that isolate each gate;
- altered message/hash and altered signature;
- cofactor-sensitive cases; and
- complete Magpie pipeline results, including exact failure stage and accepted
  history summary.

The named A21-S4, A21-S5, A21-D1, A21-D2, and A21-T2 constructions remain
important compatibility witnesses. Under `V_sig`, A21-S4, A21-S5, A21-D1,
and A21-T2 are negative public-key-admissibility cases, while A21-D2 is a
positive identity-`R` case. Their expected `V_sig` verdicts must be assigned
from this exact relation, not inherited wholesale from either a host library
or the old design contract's permissive recommendation.

Conformer obligations are:

- **Rust:** do not equate `V_sig` with `verify_strict()`. Enforce canonical
  `A` and `R`, `A != I`, `[L]A = I`, and `[L]R = I` explicitly; permit
  canonical `R = I`; then enforce the specified equation.
- **Python:** do not equate `V_sig` with the active OpenSSL default. Enforce the
  profile independently or constrain a reviewed primitive to the exact gates;
  pin the dependency/backend used for conformance.
- **Go:** implement the document and corpus independently. Do not read the Rust
  implementation as the specification or inherit `crypto/ed25519` edge
  behavior as doctrine.

All three must exercise the same exact bytes through their real public paths.
An equation-only probe is supporting evidence, not a complete corpus result.

## Rejected alternatives

1. **Call dalek `verify_strict()` and match it elsewhere.** Rejected because a
   library API is not doctrine and its small-order checks do not reject all
   mixed-torsion points.
2. **Match Python/OpenSSL.** Rejected because the active backend accepts the
   weak and non-canonical key witnesses, CI does not pin it, and another backend
   version may drift.
3. **Make the permissive A-021 design contract the public v1 profile.** Rejected
   for the candidate public path because it admits structurally weak roots and
   does not exactly preserve the current non-canonical-key language unless that
   language is also frozen.
4. **Silently tighten v1 because the project is pre-alpha.** Rejected because
   pre-alpha status does not make an accepted-language change disappear.
5. **Require `magpie-core-v2` immediately.** Rejected as unnecessary while the
   profile and key are explicit external verification coordinates. Reconsider
   only if self-description inside signed bytes becomes a requirement.
6. **Treat structural key rejection as trust.** Rejected. Admissibility narrows
   a cryptographic relation; trust and authority remain external.
7. **Accept `S` modulo `L`, reduce non-canonical points, or use a cofactored
   host default.** Rejected because each changes the explicit relation and can
   create cross-implementation disagreement.
8. **Reject canonical identity `R` merely because a library calls it small
   order.** Rejected because RFC 8032 signing can in principle produce it,
   A21-D2 requires the signing scalar rather than demonstrating a
   non-keyholder forgery, and canonical identity adds no torsion ambiguity
   under the selected subgroup gates and uncofactored equation.

## Follow-up sequence after ratification

1. Freeze the exact normative portable corpus/profile manifest.
2. Obtain hostile review of its gate-isolating and full-pipeline vectors.
3. Add a typed, explicit Rust profile-selection API without changing the old
   unprofiled path.
4. Add the matching Python profile path without relying on OpenSSL defaults.
5. Add the independent Go conformer.
6. Run differential conformance over exact identical inputs and pin the tested
   dependency/toolchain identities.
7. Only after agreement, reconcile A-021 administrative status and the public
   portability claim.
8. Consider a small FORMAT cross-reference only after the implementation
   boundary exists.

No implementation step above is authorized by this Accepted ADR alone. A-021
remains Confirmed and blocking for a portable-verification claim until
conforming implementation evidence exists.

## Owner decision recorded

On 2026-08-21, the repository owner explicitly **Accepted** the following
proposition as the exact meaning of this ADR:

> **Accepted:** We ratify
> `V_sig = "magpie-ed25519-canonical-prime-subgroup-v1"` as the only Ed25519
> signature-verification subprofile eligible for the candidate portable public
> pre-alpha path, with that identifier frozen and non-repointable and every
> change to its cryptographic byte relation requiring a distinct identity;
> with `A_bytes` exactly 32 canonically encoded bytes satisfying `A != I` and
> `[L]A = I`, while textual encoding remains outside `V_sig` and the current
> reference portable JSONL path independently retains #120's exact external-key
> gate of 64 lowercase ASCII hexadecimal characters; with each signature exactly
> `R_bytes || S_bytes`, canonical `R` satisfying `[L]R = I` and explicitly
> permitting canonical `R = I`, and canonical little-endian `0 <= S < L`; with
> `M = ASCII("magpie-sig-v1") || content_hash`,
> `k = LE_INT(SHA-512(R_bytes || A_bytes || M)) mod L`, and acceptance exactly
> when `[S]B = R + [k]A`; with explicit selection, unknown-identity rejection,
> no alias, negotiation, substitution, or fallback, and direct or transitive
> immutable identification of `V_sig` inside the complete ADR-0008 producing
> context; while keeping the profile identity outside signed
> `magpie-core-v1` bytes, keeping existing unprofiled verification unchanged and
> compatibility-only, keeping key trust external, and accepting that
> structurally inadmissible keys and signatures formerly accepted by unprofiled
> paths are rejected only when this additive subprofile is explicitly selected.
