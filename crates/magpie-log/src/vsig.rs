//! # ADR-0010 — the `V_sig` signature-verification subprofile
//!
//! Reference implementation of the frozen relation
//! `magpie-ed25519-canonical-prime-subgroup-v1` (Accepted ADR-0010,
//! owner-ratified 2026-08-21). See `docs/adr/0010-*.md` §Normative relation
//! for the normative gates; this file is an implementation, not authority.
//!
//! Gate order (exact, per ADR-0010 + the portable input-language contract's
//! composed external-key gate):
//!
//! ```text
//! ExternalKey stage (before any record processing):
//!   1. A_bytes exactly 32
//!   2. canonical compressed-Edwards decode (y < p, sqrt, parity,
//!      x = 0 => sign bit 0, byte-identical re-encode)
//!   3. A != I
//!   4. [L]A = I
//! Signature stage:
//!   5. signature exactly 64 = R_bytes || S_bytes
//!   6. R_bytes canonical decode (same decoder)
//!   7. [L]R = I          (canonical R = I is PERMITTED)
//!   8. S little-endian, 0 <= S < L, no reduction
//!   9. M = ASCII("magpie-sig-v1") || content_hash(32)
//!  10. k = LE(SHA-512(R_bytes || A_bytes || M)) mod L
//!  11. accept iff [S]B = R + [k]A      (uncofactored)
//! ```
//!
//! Deliberate notes for reviewers:
//! - `curve25519-dalek` 4.1.3 `CompressedEdwardsY::decompress` does **not**
//!   enforce `y < p` (it field-reduces); the explicit y-bound gate below is
//!   mandatory, not belt-and-braces. (Executed probe: `y = p` and `y = p+1`
//!   both decompress in the locked 4.1.3.)
//! - The re-encode byte-equality check enforces the `x = 0 => sign bit 0`
//!   rule and full canonicality without a separate x test.
//! - There is no `Default` on the profile type: selection is explicit only.
//! - The profile field is private: `CANONICAL_PRIME_SUBGROUP_V1` and
//!   `from_identity` are the only construction paths (no direct literal).

use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::traits::IsIdentity;
use curve25519_dalek::Scalar;
use sha2::{Digest, Sha512};

/// The frozen subprofile identity (ADR-0010 §Immutable subprofile identity
/// law). Never repointable; any semantic change requires a distinct identity.
pub const V_SIG_PROFILE_ID: &str = "magpie-ed25519-canonical-prime-subgroup-v1";

/// The signature-verification subprofile. A unit type: it carries no state,
/// because the identity string *is* the complete selection coordinate.
/// No `Default` impl by law (explicit selection only, no ambient default).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SignatureVerificationProfile {
    // Opaque: the one field is private, so the profile can only be produced
    // through `CANONICAL_PRIME_SUBGROUP_V1` or `from_identity` — never by
    // direct literal construction from outside this module.
    _private: (),
}

impl SignatureVerificationProfile {
    /// The one and only selectable subprofile.
    pub const CANONICAL_PRIME_SUBGROUP_V1: Self = Self { _private: () };

    /// Exact-identity selection. Unknown identities fail closed to `None` —
    /// no alias, `latest`, case folding, or negotiation.
    pub fn from_identity(identity: &str) -> Option<Self> {
        (identity == V_SIG_PROFILE_ID).then_some(Self { _private: () })
    }
}

/// Rejection classes at the `V_sig` boundary. The enclosing history verifier
/// maps `ExternalKey` to the composed external-key stage and `Signature` to
/// the existing Signature stage (ADR-0010: no new rejection classes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VsigRejection {
    /// A_bytes failed the composed external-key gate (steps 1-4).
    ExternalKey,
    /// Signature bytes or equation failed (steps 5-11).
    Signature,
}

/// The exact `V_sig` relation. Fails closed on every gate in the order
/// documented above. `content_hash` is exactly the 32-byte
/// `SHA-256(canonical EventCore bytes)`; the `magpie-sig-v1` domain prefix is
/// applied inside.
///
/// The hash arrives as an already-valid `[u8; 32]`: its representation and
/// first-failure ordering are owned by the enclosing portable
/// input-language/history profile, NOT by `V_sig`. This function does not
/// invent rejection vocabulary for malformed hash lengths.
pub fn verify_signature_with_profile(
    profile: &SignatureVerificationProfile,
    a_bytes: &[u8],
    signature: &[u8],
    content_hash: [u8; 32],
) -> Result<(), VsigRejection> {
    debug_assert_eq!(
        *profile,
        SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1
    );

    // --- ExternalKey stage -------------------------------------------------
    if a_bytes.len() != 32 {
        return Err(VsigRejection::ExternalKey);
    }
    let a = canonical_decode(a_bytes.try_into().unwrap()).ok_or(VsigRejection::ExternalKey)?;
    if a.is_identity() {
        return Err(VsigRejection::ExternalKey);
    }
    if !prime_subgroup(&a) {
        return Err(VsigRejection::ExternalKey);
    }

    // --- Signature stage ---------------------------------------------------
    if signature.len() != 64 {
        return Err(VsigRejection::Signature);
    }
    let r_bytes = &signature[..32];
    let s_bytes = &signature[32..];

    let r = canonical_decode(r_bytes.try_into().unwrap()).ok_or(VsigRejection::Signature)?;
    if !prime_subgroup(&r) {
        return Err(VsigRejection::Signature);
    }
    // S: little-endian, 0 <= S < L, no reduction.
    // `from_canonical_bytes` is the library's canonical-scalar gate:
    // `Some(s)` iff `bytes` is the unique reduced representative, i.e.
    // `0 <= S < L`; anything >= L (including L itself, L+1, all-0xFF)
    // yields `None`. This is exactly the ADR-0010 gate — no bespoke
    // comparison, no reduction, no wrap.
    let s: Scalar = match Scalar::from_canonical_bytes(s_bytes.try_into().unwrap()).into() {
        Some(s) => s,
        None => return Err(VsigRejection::Signature),
    };

    // Challenge: k = LE(SHA-512(R_bytes || A_bytes || M)) mod L,
    // M = ASCII("magpie-sig-v1") || content_hash.
    let mut mac = Sha512::new();
    mac.update(r_bytes);
    mac.update(a_bytes);
    mac.update(b"magpie-sig-v1");
    mac.update(content_hash);
    let k = Scalar::from_bytes_mod_order_wide(&mac.finalize().into());

    // Equation: [S]B = R + [k]A, uncofactored.
    let lhs = ED25519_BASEPOINT_POINT * s;
    let rhs = r + a * k;
    if lhs == rhs {
        Ok(())
    } else {
        Err(VsigRejection::Signature)
    }
}

/// Canonical compressed-Edwards25519 decode per ADR-0010 / RFC 8032 §5.1.3.
///
/// 1. low 255 bits encode y; require `0 <= y < p` (no reduction);
/// 2. field-sqrt recovery with sign-bit parity selection (via the locked
///    `decompress`);
/// 3. byte-identical re-encode, which also enforces `x = 0 => sign bit 0`.
fn canonical_decode(enc: &[u8; 32]) -> Option<EdwardsPoint> {
    // Gate the y-bound BEFORE any field interpretation. `curve25519-dalek`
    // 4.1.3 `decompress` does not enforce `y < p` (it field-reduces), so this
    // gate is mandatory, not belt-and-braces.
    if !y_below_p(enc) {
        return None;
    }
    let compressed = CompressedEdwardsY(*enc);
    let point = compressed.decompress()?;
    (point.compress().to_bytes() == *enc).then_some(point)
}

/// `y < p` for the little-endian 255-bit `y` encoded in `enc` (the low 255
/// bits of the 32-byte encoding; bit 7 of `enc[31]` is the x sign bit and is
/// NOT part of `y`).
///
/// `p = 2^255 - 19`, so `y >= p` is exactly the region: top 7 bits all 1,
/// middle 29 bytes all `0xFF`, and low byte `>= 0xED` (its bottom is `p`
/// itself). Everything else is below `p`. A two-byte low prefix test
/// (`enc[0] == 0xED && enc[1] == 0xFF`) would be WRONG: it rejects valid
/// values that merely share the low 16 bits with `p` (e.g. `y = 0xFFED`),
/// while this predicate rejects exactly `y >= p` — no more, no less.
fn y_below_p(enc: &[u8; 32]) -> bool {
    !((enc[31] & 0x7f) == 0x7f && (1..31).all(|i| enc[i] == 0xff) && enc[0] >= 0xed)
}

/// `[L]P = I` — the prime-order-subgroup predicate (includes I).
///
/// `[L]P` is computed as `(L-1)·P + P`, NOT `P * Scalar::from_bytes_mod_order(L_LE)`:
/// `from_bytes_mod_order` reduces its input modulo `L`, so `L mod L == 0` would make
/// the scalar the zero element and the predicate vacuously true for every point
/// (accepting order-2, order-4, and mixed-torsion keys — the D1/T2 forgery).
/// `L-1` is below `L`, so `from_bytes_mod_order(L-1)` is exact.
fn prime_subgroup(p: &EdwardsPoint) -> bool {
    const LM1_LE: [u8; 32] = *b"\xec\xd3\xf5\x5c\x1a\x63\x12\x58\xd6\x9c\xf7\xa2\xde\xf9\xde\x14\
                             \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10";
    let l_minus_1 = Scalar::from_bytes_mod_order(LM1_LE);
    let l_p = l_minus_1 * *p + *p;
    l_p.is_identity()
}

#[cfg(test)]
mod tests {
    //! Focused ADR-0010 vector tests. Every byte below is frozen from the
    //! ratified portable-verifier corpus (`fixtures/verifier-language-v1`),
    //! keyed to the case `id` in the comment. These are the permanent,
    //! committed record of the load-bearing `V_sig` relation; the full
    //! 37-vector cross-check additionally runs via the corpus harness.

    use super::*;

    /// The golden external key (A) — the keyholder for every A21 signature-stage
    /// vector. `ea4a6c…1446d22c`.
    const A_KEY: &[u8; 32] = b"\xea\x4a\x6c\x63\xe2\x9c\x52\x0a\xbe\xf5\x50\x7b\x13\x2e\xc5\xf9\
                              \x95\x47\x76\xae\xbe\xbe\x7b\x92\x42\x1e\xea\x69\x14\x46\xd2\x2c";
    /// The content hash shared by the A21 single-record vectors.
    const HASH: [u8; 32] = *b"\xcb\xb7\x68\x5e\xfd\x5d\x67\x9a\x5a\x6f\x54\x8f\xe4\x2a\x36\xd8\
                              \xda\xcd\x39\x16\x06\xa0\xc2\x3c\x76\x7d\x89\x05\x2f\x01\xe9\x0f";

    fn sig(hex: &str) -> [u8; 64] {
        let mut out = [0u8; 64];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            out[i] = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap();
        }
        out
    }
    fn key(hex: &str) -> [u8; 32] {
        let mut out = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            out[i] = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap();
        }
        out
    }
    fn profile() -> SignatureVerificationProfile {
        SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1
    }

    // ---- Positive controls -------------------------------------------------

    /// a21-d2-identity-r-equation-true: canonical R = I is PERMITTED and the
    /// keyholder-derived S satisfies the exact uncofactored equation.
    #[test]
    fn positive_identity_r_equation_true() {
        let s = sig(
            "0100000000000000000000000000000000000000000000000000000000000000\
                     0fe717c89f31e5f1d8ee5bf0a2c1bde5734344ec13f2891593b3969c587c3504",
        );
        assert_eq!(
            verify_signature_with_profile(&profile(), A_KEY, &s, HASH),
            Ok(())
        );
    }

    /// golden-v1 rec0: non-identity R, valid S, the real in-crate golden chain
    /// (`crates/magpie-log/testdata/golden-v1.jsonl`). Proves the equation holds
    /// for an ordinary point, not just R = I.
    #[test]
    fn positive_golden_v1_rec0_nonidentity_r() {
        let s = sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
                     4421f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
        );
        assert_eq!(
            verify_signature_with_profile(&profile(), A_KEY, &s, HASH),
            Ok(())
        );
    }

    // ---- ExternalKey stage (steps 1-4) -------------------------------------

    fn expect_external_key(a: &[u8]) {
        // Any 64-byte signature: the key gate must fire before record/signature
        // processing. Use the a21-d2 signature for concreteness.
        let s = sig(
            "0100000000000000000000000000000000000000000000000000000000000000\
                     fe717c89f31e5f1d8ee5bf0a2c1bde5734344ec13f2891593b3969c587c3504",
        );
        assert_eq!(
            verify_signature_with_profile(&profile(), a, &s, HASH),
            Err(VsigRejection::ExternalKey),
        );
    }

    #[test]
    fn external_key_wrong_length() {
        expect_external_key(&A_KEY[..31]); // 31 bytes
        let mut long = [0u8; 33];
        long[..32].copy_from_slice(A_KEY);
        expect_external_key(&long); // 33 bytes
        expect_external_key(&[]); // 0 bytes
    }

    /// k9-key-y-equals-p: y == p is out of range (no reduction).
    #[test]
    fn external_key_y_equals_p() {
        expect_external_key(&key(
            "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        ));
    }

    /// k10-key-y-p-plus-one: y == p+1 is out of range.
    #[test]
    fn external_key_y_p_plus_one() {
        expect_external_key(&key(
            "eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        ));
    }

    /// a21-order-two-key: (0,-1) is order 2, [L]A != I.
    #[test]
    fn external_key_order_two() {
        expect_external_key(&key(
            "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
        ));
    }

    /// a21-other-small-order-key: (1,0) is the canonical order-4 point.
    #[test]
    fn external_key_order_four() {
        expect_external_key(&key(
            "0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// a21-mixed-torsion-key: B+T4 has a torsion residue, [L]A != I.
    #[test]
    fn external_key_mixed_torsion() {
        expect_external_key(&key(
            "5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea",
        ));
    }

    /// k13-a21-identity-key: A = I is structurally inadmissible (A != I).
    #[test]
    fn external_key_identity() {
        expect_external_key(&key(
            "0100000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// k12-key-x-zero-sign-one: identity with the x sign bit set is non-canonical.
    #[test]
    fn external_key_identity_noncanonical_sign() {
        expect_external_key(&key(
            "0100000000000000000000000000000000000000000000000000000000000080",
        ));
    }

    /// k4-k5-k11-key-no-square-root: encodes a y with no Edwards25519 square root.
    #[test]
    fn external_key_no_square_root() {
        expect_external_key(&key(
            "0200000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    // ---- Signature stage (steps 5-11) --------------------------------------

    fn expect_signature(s: &[u8]) {
        assert_eq!(
            verify_signature_with_profile(&profile(), A_KEY, s, HASH),
            Err(VsigRejection::Signature),
        );
    }

    #[test]
    fn signature_wrong_length() {
        let good = sig(
            "0100000000000000000000000000000000000000000000000000000000000000\
                        fe717c89f31e5f1d8ee5bf0a2c1bde5734344ec13f2891593b3969c587c3504",
        );
        expect_signature(&good[..63]); // 63 bytes
        let mut long = [0u8; 65];
        long[..64].copy_from_slice(&good);
        expect_signature(&long); // 65 bytes
        expect_signature(&[]); // 0 bytes
    }

    /// a21-r-order-four-cofactor: canonical order-4 R is outside the prime
    /// subgroup; [L]R != I. (Cofactor multiplication would hide the residue.)
    #[test]
    fn signature_r_order_four() {
        expect_signature(&sig(
            "0000000000000000000000000000000000000000000000000000000000000000\
             092bca330f75d53ff55660018588a15dbe912d8fd4faaa50b235d93bf4c63509",
        ));
    }

    /// a21-r-mixed-torsion-cofactor: R = B + T4 leaves a torsion residue.
    #[test]
    fn signature_r_mixed_torsion() {
        expect_signature(&sig(
            "5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea\
             973c85b2e7de740ef185798f053f0c94f8a13a247b0bde8eacc39c1ab5222f04",
        ));
    }

    /// a21-r-nonidentity-small-order: R = (0,-1), canonical small-order R.
    #[test]
    fn signature_r_nonidentity_small_order() {
        expect_signature(&sig(
            "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f\
             6f73fb8ebc395b70faf6c5a82e9ae25669601e7b446aefa81316a80d48cec402",
        ));
    }

    /// a21-r-identity-equation-false: canonical identity R is admissible, but
    /// S = 0 makes the equation false for an ordinary key.
    #[test]
    fn signature_r_identity_equation_false() {
        expect_signature(&sig(
            "0100000000000000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// a21-r-x-zero-sign-one: R = (0,-1) encoded with x sign bit 1 (non-canonical
    /// identity alternate sign) — wait, this is R=(0,1)? No: y=1, sign=1 => the
    /// forbidden alternate-sign encoding. Rejected by canonical decode.
    #[test]
    fn signature_r_x_zero_sign_one() {
        expect_signature(&sig(
            "0100000000000000000000000000000000000000000000000000000000000080\
             0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// a21-r-y-equals-p: R with y == p is out of range.
    #[test]
    fn signature_r_y_equals_p() {
        expect_signature(&sig(
            "edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f\
             0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// a21-r-nondecompress: R encodes a y with no square root.
    #[test]
    fn signature_r_nondecompress() {
        expect_signature(&sig(
            "0200000000000000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    /// a21-s-equals-l: S == L is not a reduced representative, rejected (no
    /// reduction). This is the regression for the `L mod L == 0` trap.
    #[test]
    fn signature_s_equals_l() {
        expect_signature(&sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
             edd3f55c1a631258d69cf7a2def9de1400000000000000000000000000000010",
        ));
    }

    /// a21-s-l-plus-one: S == L+1 is out of the reduced range.
    #[test]
    fn signature_s_l_plus_one() {
        expect_signature(&sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
             eed3f55c1a631258d69cf7a2def9de1400000000000000000000000000000010",
        ));
    }

    /// a21-s-all-ff: S = 2^256-1 is out of the reduced range.
    #[test]
    fn signature_s_all_ff() {
        expect_signature(&sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ));
    }

    /// a21-s-l-minus-one-equation-false: S = L-1 is canonical (in range) but the
    /// complete equation is false. Proves the S gate does not over-reject a
    /// valid scalar, and the equation still rejects.
    #[test]
    fn signature_s_l_minus_one_equation_false() {
        expect_signature(&sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
             ecd3f55c1a631258d69cf7a2def9de1400000000000000000000000000000010",
        ));
    }

    /// a21-altered-message: same signature, but the content hash was altered.
    /// a21-altered-message: this signature is valid for a *different* content
    /// hash (`cab768...`, per the manifest `construction_evidence` field
    /// `signed_other_content_hash`), not for the record's own hash `cbb768...`
    /// (`HASH`). Direction pinned from the frozen fixture: it verifies for
    /// the other hash and is REJECTED for the record's exact domain-separated
    /// message.
    #[test]
    fn signature_altered_message() {
        let s = sig(
            "8b4f2dd06a5e0ac96fea81b357d9845dbfeeca47a872258b22b6407c0dee280\
                     50993e087c44502068fd814dc1262268bc45e18e3f97c19a2f0d7bb059f4d3400",
        );
        // The hash the signature was actually produced for (frozen evidence).
        let signed_other = key("cab7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f");
        assert_eq!(
            verify_signature_with_profile(&profile(), A_KEY, &s, signed_other),
            Ok(()),
            "signature must verify for its own (other) content hash"
        );
        // ...and reject for the record's own hash: the challenge binds M.
        assert_eq!(
            verify_signature_with_profile(&profile(), A_KEY, &s, HASH),
            Err(VsigRejection::Signature),
        );
    }

    /// a21-altered-signature: a single bit flipped in the valid signature must
    /// fail the equation.
    #[test]
    fn signature_altered_signature() {
        expect_signature(&sig(
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db\
             4521f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
        ));
    }

    // ---- Adversarial witness (independent of the fixture corpus) --------

    /// W1: the order-2 key A2 = (0,-1) with the self-canceling signature
    /// R = I, S = 0. Because k = LE(SHA-512(R||A2||M)) mod L is even for this
    /// M (verified against the golden content hash), [k]A2 = I and the
    /// uncofactored equation [0]B = I + [k]A2 VERIFIES. Only the [L]A = I
    /// prime-subgroup gate rejects it. Any vacuous or missing [L]A predicate
    /// would accept this forgery. (Independent witness; see the hostile-review
    /// W1 note.)
    #[test]
    fn adversarial_w1_order_two_self_canceling_equation_holds() {
        let a2 = key("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        let r_i = sig(
            "0100000000000000000000000000000000000000000000000000000000000000\
                       0000000000000000000000000000000000000000000000000000000000000000",
        );
        assert_eq!(
            verify_signature_with_profile(&profile(), &a2, &r_i, HASH),
            Err(VsigRejection::ExternalKey),
            "order-2 key must be rejected at the ExternalKey stage"
        );
    }

    /// W2: the same order-2 bytes used as R under a valid prime-subgroup key
    /// must be rejected as Signature by the [L]R = I gate (identity R is
    /// permitted, this is not identity). With prime-subgroup A no witness
    /// can make the equation hold for R = T2, so the gate is
    /// defense-in-depth at this stage; the test pins the rejection class.
    #[test]
    fn adversarial_w2_order_two_r_rejected_by_subgroup_gate() {
        expect_signature(&sig(
            "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f\
             0000000000000000000000000000000000000000000000000000000000000000",
        ));
    }

    // ---- The y-bound gate (the suspected false-reject, now pinned) ---------

    /// Regression for the `enc[0]==0xed && enc[1]==0xff` false-reject: `y = 0xFFED`
    /// (`ed ff 00 … 00`) shares p's low two bytes but is a VALID y well below p.
    /// It must NOT be rejected by the y-bound gate. We probe the private decoder
    /// directly to isolate the gate from the rest of the relation.
    #[test]
    fn y_bound_does_not_false_reject_y_equals_0xffed() {
        let mut enc = [0u8; 32];
        enc[0] = 0xed;
        enc[1] = 0xff;
        // y = 0xFFED < p, so the bound gate must pass. (Whether the full point
        // then decodes depends on the square root; we assert the *bound* gate.)
        assert!(y_below_p(&enc), "y=0xffed must satisfy y < p");
    }

    /// The bound gate rejects exactly y >= p. Probe the boundary directly.
    #[test]
    fn y_bound_rejects_exactly_y_ge_p() {
        // y = p (enc[0]=0xed, enc[1..31]=0xff, top-7=0x7f): reject.
        let mut enc_p = [0xffu8; 31];
        enc_p[0] = 0xed;
        let full_p = [0xffu8; 32];
        let _ = full_p;
        assert!(
            !y_below_p(&[
                0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f
            ]),
            "y=p rejected"
        );
        // y = p + 1 (enc[0]=0xee, rest 0xff): reject.
        assert!(
            !y_below_p(&[
                0xee, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f
            ]),
            "y=p+1 rejected"
        );
        // y = p - 1 (enc[0]=0xec, enc[1..31]=0xff): accept.
        assert!(
            y_below_p(&[
                0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0x7f
            ]),
            "y=p-1 accepted"
        );
        // A tiny y with a 0xed 0xff prefix (0xffed): accept.
        let mut small = [0u8; 32];
        small[0] = 0xed;
        small[1] = 0xff;
        assert!(y_below_p(&small), "y=0xffed accepted");
        let _ = enc_p;
    }

    // ---- Profile selection -------------------------------------------------

    #[test]
    fn from_identity_exact_and_fail_closed() {
        assert!(SignatureVerificationProfile::from_identity(V_SIG_PROFILE_ID).is_some());
        // Unknown / alias / case-folded / latest all fail closed to None.
        assert!(SignatureVerificationProfile::from_identity("").is_none());
        assert!(SignatureVerificationProfile::from_identity("latest").is_none());
        assert!(SignatureVerificationProfile::from_identity(
            "magpie-ed25519-canonical-prime-subgroup-v2"
        )
        .is_none());
        assert!(SignatureVerificationProfile::from_identity(
            "Magpie-Ed25519-Canonical-Prime-Subgroup-V1"
        )
        .is_none());
        assert!(SignatureVerificationProfile::from_identity(
            "magpie-ed25519-canonical-prime-subgroup-v1 "
        )
        .is_none());
        assert!(SignatureVerificationProfile::from_identity(
            "magpie-ed25519-canonical-prime-subgroup-v1\n"
        )
        .is_none());
    }

    #[test]
    fn no_default_on_profile() {
        // The type must NOT implement Default (explicit selection only). This is
        // a compile-time law; here we assert the const is the only selector.
        let p = SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1;
        assert_eq!(p, SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1);
    }
}
