# Hermes hostile review — Qwen Rust V_sig (#134 @ 0289dcc)

Self-contained: quotes each reviewed element, gives verdict + basis.
Normative basis: ADR-0010 normative gates; A-021 verification-semantics
contract; portable input-language contract (V_sig boundary only here).

## [H1] Challenge-hash bytes — CORRECT

Code (vsig.rs ~128-133):

```rust
mac.update(r_bytes);
mac.update(a_bytes);
mac.update(b"magpie-sig-v1");
mac.update(content_hash);
let k = Scalar::from_bytes_mod_order_wide(&mac.finalize().into());
```

ADR-0010: `k = LE(SHA-512(R_bytes || A_bytes || ("magpie-sig-v1" || content_hash))) mod L`.
Byte order matches exactly (R first, then A, then domain‖hash).
`from_bytes_mod_order_wide` takes the full 64-byte SHA-512 output and reduces
mod L — identical to the specified LE-interpretation-then-mod-L.

**VERDICT: PASS — V_sig relation only.**

## [H2] S canonicality via `Scalar::from_canonical_bytes` — CORRECT

`Some(s)` iff the bytes are the unique reduced representative (0 ≤ S < L),
`None` otherwise; no reduction. S=L, L+1, all-FF all yield None — exactly
the strict gate. Confirmed by their passing s_equals_l / s_l_plus_one /
s_all_ff tests (once literals are fixed).

**VERDICT: PASS.**

## [H3] `prime_subgroup()` as `(L-1)*P + P` — CORRECT and well-reasoned

`Scalar::from_bytes_mod_order(L)` would reduce to zero, making the predicate
vacuously true for every point. `(L-1)` is exact and `(L-1)P + P = LP`. My
independent adversarial witness W1/W2 (`adversarial_witness.py`, PR #133)
targets precisely this: an order-2 key whose self-canceling signature
satisfies the uncofactored equation (`[0]B = -[k]A₂ + [k]A₂`). Any vacuity
would ACCEPT it; Qwen's construction rejects at ExternalKey per my oracle.

**VERDICT: PASS.**

## [H4] `y_below_p` bit predicate — EXACT (independently verified)

`hostile_y_bound.py` (PR #133): boundary-region sweep, p±δ encodings, the
y=0xffed false-reject trap, and 100k random samples → **zero mismatches**.
The region test `(top7==0x7f && enc[1..31]==0xFF*29 && enc[0]>=0xED)`
rejects exactly y ≥ p.

**VERDICT: PASS.**

## [H5] `canonical_decode()` re-encode equality — CORRECT

decompress → compress byte-equality enforces x=0⇒sign=0 and full
canonicality; matches my `_encode` round-trip check.

**VERDICT: PASS.**

## [H6] Gate order — CORRECT

len(A)==32 → decode → ¬identity → prime_subgroup, all before any signature
access; len(sig)==64 precedes R work; `[L]R` precedes S-range which precedes
the equation. Matches ADR-0010 ordering and my wrapper's stage mapping.

**VERDICT: PASS.**

## [H7] Profile selection — CORRECT, one API-hardening observation

Unit struct + named const + exact-match `from_identity` failing closed (no
alias / latest / case-fold / whitespace). Tests cover "", "latest", v2,
case-folded, trailing space/newline. No Default.

OBSERVATION (API-hardening, not a semantic defect): a bare pub unit struct
`SignatureVerificationProfile` can be constructed by any external crate as
`SignatureVerificationProfile {}` — the const adds naming discipline but no
constructive protection. If hardening is wanted before promotion: add a
private ZST field or `#[non_exhaustive]` so only the const / from_identity
can produce values. Also `debug_assert_eq!` on the profile fires only in
debug builds — harmless. **CLASSIFICATION: API-hardening issue; reviewer/owner
choice.**

## [H8] Test-literal defects — CONFIRMED (matches my earlier diagnosis)

Five tests fail at head `0289dcc`:

| test | defect | effect |
|---|---|---|
| `positive_identity_r_equation_true` | sig literal 127 chars (odd) | last S byte parses 0x04; equation fails; expected Ok(()) |
| `signature_s_equals_l` | S-part literal 65 chars (extra '0') | panic index 64 of [u8;64] |
| `signature_s_l_plus_one` | same | panic |
| `signature_s_l_minus_one_equation_false` | same | panic |
| `signature_altered_message` | wrong direction/value | got Ok(()), expected Err(Signature) |

Plus `cargo fmt --check` drift. **Repair rule: derive replacements from the
frozen fixture or the exact mathematical boundary — do not flip expectations
to green.** Note for altered_message: verify against the corpus whether the
intended case is "valid signature over different M" (equation must fail) —
if their constant is actually valid for the UNALTERED hash, the constant is
simply wrong, not the expectation.

**CLASSIFICATION: test-oracle defects + harness hygiene. NOT semantic.**

## [H9] Corpus-harness scope wording — keep honest

`.scratch/vsig-harness` exercises the RELATION over fixture cases (413 files
exist; key-only empty cases skipped). This is V_sig-relation evidence, NOT
full portable conformance (no framing/schema/duplicate-member/lexical-law
coverage). Required phrasing: **"37 V_sig-relevant cases exercised"**, never
"413 portable cases conform".

## [H10] Constants provenance — VERIFIED genuine

- A_KEY literal == golden-v1.jsonl genesis verifying_key
  `ea4a6c63…d22c` (byte-for-byte vs fixture file). ✔
- HASH == rec0 stored content_hash `cbb7685e…e90f` AND independently
  recomputed by my from-FORMAT.md encoder. ✔
- Golden rec0 signature ACCEPTs under my checker with these bytes. ✔

So Qwen's frozen constants are real fixture bytes (unlike the five typo'd
synthetic literals in H8).

---

# VERDICT SUMMARY

- Relation semantics: **PASS — V_sig mathematical relation only**
- Test suite: **FAILS AS COMMITTED** (5 broken tests + fmt). Fix by deriving
  from frozen fixtures/math; do not flip expectations.
- API: minor hardening opportunity (H7); owner/reviewer choice.
- Harness claims: scope to V_sig relation; **NOT YET — portable verifier
  conformance**.
