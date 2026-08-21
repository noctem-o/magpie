# ADR-0010: Ed25519 Verification Profile and External Trust-Root Admissibility

**Status:** Proposed (explicit repository-owner ratification required)

## Summary / Y-statement

In the context of `magpie-core-v1` histories signed with ordinary Ed25519 over
`"magpie-sig-v1" || content_hash`, facing host libraries that accept
structurally weak and non-canonical material differently and a current accepted
language that includes weak-root witnesses, we propose one additive, explicitly
selected verification profile:

```text
G = "magpie-ed25519-canonical-prime-subgroup-v1"

Verify_G(exact supplied history, exact externally supplied key bytes)
```

`G` requires canonical compressed Edwards25519 encodings, non-identity points
in the prime-order subgroup for both the public key `A` and signature point
`R`, a canonical scalar `0 <= S < L`, the ordinary Ed25519 SHA-512 challenge,
and the exact uncofactored equation `[S]B = R + [k]A`.

The profile is an external verification coordinate. It does not change
`magpie-core-v1` canonical bytes, the `magpie-sig-v1` message, existing
signature bytes, or the current unprofiled Rust/Python verifier behavior. It
must be selected explicitly, with no ambient default, alias, negotiation,
fallback, or downgrade. A result intended to be portable must retain the exact
profile identity.

Existing unprofiled verification remains compatibility-only behavior. It is
not silently renamed as `G`, and `G` is not declared to be the historical
meaning of `magpie-core-v1`. This is what prevents the proposal from hiding an
accepted-language break behind unchanged serialized bytes.

This is a Proposed constitutional decision. It is not Accepted doctrine until
the repository owner explicitly ratifies it. Its presence, review, merge, or
passing checks would not constitute ratification.

## Authority and relationship to existing material

`docs/FORMAT.md` remains the authority for `magpie-core-v1` bytes, the
`magpie-sig-v1` message, genesis key declaration, and the externally supplied
trust-root boundary. This ADR does not amend FORMAT while Proposed.

The following documents are evidence and design inputs, not authority capable
of silently amending FORMAT or ratifying this ADR:

- `docs/design/portable-verifier-input-language-contract.md` defines the
  proposed canonical external-input gate;
- `docs/design/a021-ed25519-verification-semantics-contract.md` reconstructs a
  portable permissive relation and supplies concrete weak/torsion witnesses;
- `docs/audits/magpie-architecture-audit-2026-08-01.md` preserves the historical
  A-021 finding; and
- `docs/audits/audit-disposition-2026-08.md` remains the living administrative
  ledger, where A-021 stays Confirmed.

If ratified, this ADR supersedes the design contract's recommendation to make
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

This ADR proposes:

1. one exact Ed25519 verification-profile identity;
2. its public-key, signature, challenge, and equation relation;
3. its explicit-selection and result-carriage law;
4. its compatibility relationship to existing unprofiled v1 verification;
5. its eligibility for the narrow candidate portable public pre-alpha path;
   and
6. the owner decision and implementation sequence required to proceed.

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

### Non-identity prime-subgroup membership

A decoded point `P` satisfies this predicate exactly when:

```text
P != I
and
[L]P = I
```

Because `L` is prime, this admits only non-identity points in the subgroup
generated by `B`. It rejects pure small-order points and points with any
non-zero torsion component. Checking only `[8]P != I`, or only a library's
`is_small_order` predicate, is not equivalent.

### Verification profile

`verification profile` means the complete relation selected for one
verification attempt. It is distinct from:

- the `magpie-core-v1` canonicalization profile carried in Genesis;
- the `magpie-sig-v1` bytes prepended to the content hash;
- the external verification key itself; and
- any decision to trust, authorize, or identify that key.

`unprofiled v1 verification` means the current Rust or Python path that invokes
its host library without an explicit Magpie verification-profile identity.

## Current-behavior characterization

This characterization was repeated against repository baseline
`2e3178bcb3e63e78ea729ff1873024ada91e5ac2`. It records evidence; it is not the
proposed normative relation.

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
one library function would therefore not define the relation proposed here.

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

Declare the proposed relation to be what `magpie-core-v1` always meant and
change both existing verifiers.

Rejected. A21-S4, A21-S5, A21-D1, A21-D2, and A21-T2 demonstrate inputs accepted
by current ordinary verification that the proposed relation rejects. The
non-canonical-key probe adds another accepted class. Unchanged EventCore bytes
do not make that accepted-language narrowing compatible.

### Option C — preserve core-v1 bytes and add the explicit canonical-prime-subgroup profile

Keep existing unprofiled behavior unchanged, define `G` independently, require
explicit selection and result carriage, and make only `G` eligible for a future
portable public verification claim.

**Recommended.** This creates a portable fail-closed boundary without
retroactively reclassifying existing verifier outcomes. It is a genuine
compatibility separation only if implementations preserve both sides: no
implicit upgrade of old calls, no fallback from `G`, and no result that omits
which profile produced it.

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

## Proposed normative relation

If this ADR is ratified, a conformer implements
`magpie-ed25519-canonical-prime-subgroup-v1` exactly as follows.

### 1. Explicit profile selection

The caller supplies the exact profile identity and external verification key.
Unknown profile identities fail closed before signature verification. There is
no `latest`, empty, legacy alias, library-default, negotiation, automatic
fallback, or try-strict-then-permissive mode.

The profile identity must be retained with any portable verification outcome.
A detached Boolean or summary without the profile identity is not a portable
Magpie verification result.

The profile is not a new field in `SignedEvent` or Genesis. Adding such a field
would be separate core-format work.

### 2. External public-key representation and admissibility

The cryptographic key input is exactly 32 bytes `A_bytes`. A textual Magpie
transport for this profile must use exactly 64 lowercase ASCII hexadecimal
characters, with no prefix, whitespace, case folding, or alternate encoding,
and decode to those 32 bytes.

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
R != I
and
[L]R = I
```

Decode `S_bytes` as a non-negative little-endian integer `S` and require:

```text
0 <= S < L
```

Do not reduce `S` modulo `L`. Do not accept a high-bit-only approximation such
as `S < 2^253` or the narrower, incorrect `S < 2^252` rule.

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

This is the simple, uncofactored equation. A batch verifier must produce the
same per-signature verdicts as this individual relation; a probabilistic batch
equation or library batch default may not change the accepted language.

Because admissible `A` and `R` are in the prime-order subgroup, multiplying
both sides by the cofactor does not change the verdict. The uncofactored form is
nevertheless normative so independent implementations need not infer which
equation to use.

### 6. RFC 8032 classification

The signature primitive and challenge are ordinary RFC 8032 Ed25519. The
acceptance relation is a stricter subset: it adds canonical representation,
non-identity, and full prime-subgroup requirements for `A` and `R`, then uses
the RFC-permitted simple equation. It must not be described merely as “RFC
8032 strict,” because that phrase does not identify these additional gates.

## Compatibility consequences

| Compatibility dimension | Consequence |
| --- | --- |
| Canonical byte compatibility | `SignedEvent`, EventCore, `magpie-core-v1` canonical bytes, hashes, and Genesis fields are unchanged. |
| Signature byte compatibility | Existing ordinary writers still sign the same `"magpie-sig-v1" || content_hash` bytes and produce the same signatures. |
| Accepted-language compatibility | `G` is a strict subset of current unprofiled behavior. Weak, mixed-torsion, and non-canonical-key witnesses accepted today are rejected by `G`. Existing unprofiled verdicts are not changed. |
| Golden compatibility | An independent scratch implementation of the proposed relation accepted all 9 frozen golden records and both Deadbolt anchor records. No fixture bytes were changed. |
| Profile identity compatibility | The stricter relation cannot honestly be called the meaning of `magpie-core-v1` alone. It requires the additive external identity `magpie-ed25519-canonical-prime-subgroup-v1`. |

The frozen inputs checked above were:

```text
crates/magpie-log/testdata/golden-v1.jsonl
SHA-256 767ef86b7e2ad1d65e8315ae28bac682bceee29223b5f1a0612ec7ef062cffb4

fixtures/deadbolt-anchor-v1/anchor-log.jsonl
SHA-256 2c715006e3e5bef571b3d31b95f47d3c4bbb3ea68fa9750365b4a62b66cd41e5
```

The scratch checker also rejected A21-S4, A21-S5, and A21-T2 at public-key
admissibility and A21-D2 at `R` admissibility. Those are intentional profile
differences, not claims that the bytes were malformed under the old path.

### Coexistence and migration consequence

If ratified:

- existing unprofiled Rust/Python APIs retain their current behavior and are
  labelled compatibility-only, not portable `G` verification;
- new APIs are additive and require an explicit closed profile identity;
- a history may be evaluated under more than one explicitly named relation and
  may receive different verdicts without either result silently rewriting the
  other;
- any portable result retains at least the exact profile identity, external key
  bytes or their immutable identity, and exact supplied-history identity;
- there is no automatic migration, fallback, or reinterpretation of an old
  success as a `G` success; and
- a future requirement that the chain itself sign or declare the verification
  profile reopens Option D and requires separate owner-ratified format work.

No successor core profile or signature domain is required by this proposal.
The additive boundary is the verification profile, not `magpie-core-v2`.

## Security and epistemic consequences

The proposal removes library-default ambiguity from the candidate portable
boundary and rejects structural weak-root and torsion witnesses before their
equations can produce a success. Full subgroup checks are more exact than
dalek's `verify_strict` small-order test and are deliberately reviewable by an
independent implementer.

That improvement is narrow. A successful result establishes only:

> This exact supplied history satisfies
> `magpie-ed25519-canonical-prime-subgroup-v1` under this exact externally
> supplied verification key.

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

This ADR intentionally contains no permanent vector files. After ratification,
the next tranche must be the **exact normative portable corpus/profile
manifest**.

That manifest must identify `G` exactly and freeze, at minimum:

- ordinary valid key/signature and all existing golden inputs;
- malformed key and signature lengths;
- canonical and non-canonical public-key encodings;
- identity, other small-order, and mixed-torsion public keys;
- non-decompressing, non-canonical, identity, small-order, and mixed-torsion
  `R` values;
- `S = 0`, `S = L - 1`, `S = L`, `S = L + 1`, and other out-of-range values;
- equation-true and equation-false witnesses that isolate each gate;
- altered message/hash and altered signature;
- cofactor-sensitive cases; and
- complete Magpie pipeline results, including exact failure stage and accepted
  history summary.

The existing A21-S4, A21-S5, A21-D1, A21-D2, and A21-T2 bytes remain important
negative compatibility witnesses under `G`. Their expected `G` verdicts must
not be inherited from the old design contract's proposed permissive relation.

Conformer obligations are:

- **Rust:** do not equate `G` with `verify_strict()`. Enforce canonical `A` and
  `R`, non-identity, and `[L]P = I` explicitly, then the specified equation.
- **Python:** do not equate `G` with the active OpenSSL default. Enforce the
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
8. Consider a small FORMAT cross-reference only after this ADR is Accepted and
   the implementation boundary exists.

No step above is authorized by this Proposed ADR alone. A-021 remains
Confirmed and blocking for a portable-verification claim until owner
ratification and conforming implementation evidence exist.

## Owner decision required

> **Accept, amend, or reject:** Do we ratify
> `magpie-ed25519-canonical-prime-subgroup-v1` as the only Ed25519 verification
> profile eligible for the candidate portable public pre-alpha path—requiring
> canonical non-identity prime-subgroup `A` and `R`, canonical `0 <= S < L`,
> ordinary Ed25519 over `"magpie-sig-v1" || content_hash`, and the exact
> uncofactored equation `[S]B = R + [k]A`—while keeping key trust external,
> retaining existing unprofiled `magpie-core-v1` verification unchanged and
> compatibility-only, and accepting that weak, torsion, and non-canonical-key
> inputs previously accepted by those paths are rejected only when this
> additive profile is explicitly selected?
