//! Explicit implementation of ADR-0010's signature-verification subprofile.
//!
//! This module implements only the cryptographic relation identified by
//! [`V_SIG_PROFILE_ID`]. It does not parse the portable JSONL language,
//! establish trust in an external key, replace Magpie's compatibility-only
//! legacy verification, or constitute full portable-verifier conformance.
//! A successful [`ProfiledSignatureVerifier::verify`] call is primitive
//! control flow, not a detached or coordinate-complete history result.

use core::fmt;

use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::traits::IsIdentity;
use curve25519_dalek::Scalar;
use sha2::{Digest, Sha512};

/// Immutable identity of the exact ADR-0010 signature-verification relation.
pub const V_SIG_PROFILE_ID: &str = "magpie-ed25519-canonical-prime-subgroup-v1";

const SIGNATURE_MESSAGE_DOMAIN: &[u8] = b"magpie-sig-v1";
const FIELD_MODULUS_LE: [u8; 32] = [
    0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum SelectedSignatureVerificationProfile {
    CanonicalPrimeSubgroupV1,
}

/// One explicitly selected, immutable signature-verification profile.
///
/// The private state prevents safe external construction outside the named
/// constant or exact-string parser:
///
/// ```compile_fail,E0451
/// use magpie_log::signature_profile::SignatureVerificationProfile;
///
/// let _ = SignatureVerificationProfile {
///     selected: unreachable!(),
/// };
/// ```
///
/// There is deliberately no ambient default:
///
/// ```compile_fail,E0277
/// use magpie_log::signature_profile::SignatureVerificationProfile;
///
/// let _ = <SignatureVerificationProfile as Default>::default();
/// ```
///
/// Serialized caller material cannot select or mint this profile:
///
/// ```compile_fail,E0277
/// use magpie_log::signature_profile::SignatureVerificationProfile;
///
/// let _: SignatureVerificationProfile = serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SignatureVerificationProfile {
    selected: SelectedSignatureVerificationProfile,
}

impl SignatureVerificationProfile {
    /// The exact canonical-prime-subgroup v1 profile ratified by ADR-0010.
    pub const CANONICAL_PRIME_SUBGROUP_V1: Self = Self {
        selected: SelectedSignatureVerificationProfile::CanonicalPrimeSubgroupV1,
    };

    /// Select a profile by its exact immutable identity, failing closed for
    /// every other string.
    pub fn from_identity(identity: &str) -> Result<Self, UnsupportedSignatureVerificationProfile> {
        match identity {
            V_SIG_PROFILE_ID => Ok(Self::CANONICAL_PRIME_SUBGROUP_V1),
            _ => Err(UnsupportedSignatureVerificationProfile { _private: () }),
        }
    }

    /// Return the exact immutable identity of this selected profile.
    pub fn identity(self) -> &'static str {
        match self.selected {
            SelectedSignatureVerificationProfile::CanonicalPrimeSubgroupV1 => V_SIG_PROFILE_ID,
        }
    }
}

impl fmt::Debug for SignatureVerificationProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("SignatureVerificationProfile")
            .field(&self.identity())
            .finish()
    }
}

/// An exact signature-profile identity was not supported.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedSignatureVerificationProfile {
    _private: (),
}

impl fmt::Debug for UnsupportedSignatureVerificationProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("UnsupportedSignatureVerificationProfile")
    }
}

impl fmt::Display for UnsupportedSignatureVerificationProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unsupported signature-verification profile")
    }
}

impl std::error::Error for UnsupportedSignatureVerificationProfile {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CanonicalPointFailure {
    EncodedYOutOfRange,
    NotOnCurve,
    NonCanonicalReencoding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExternalKeyFailure {
    NonCanonical(CanonicalPointFailure),
    Identity,
    NotPrimeSubgroup,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SignatureFailure {
    NonCanonicalR(CanonicalPointFailure),
    RNotPrimeSubgroup,
    SNotCanonical,
    EquationMismatch,
}

/// The exact external key bytes were structurally inadmissible under the
/// selected profile.
///
/// This rejection says nothing about whether another admissible key is
/// trusted, authorized, or controlled by a particular actor.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExternalKeyRejected {
    failure: ExternalKeyFailure,
}

impl fmt::Debug for ExternalKeyRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalKeyRejected")
            .field("failure", &self.failure)
            .finish()
    }
}

impl fmt::Display for ExternalKeyRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("external key rejected by the selected signature profile")
    }
}

impl std::error::Error for ExternalKeyRejected {}

/// The exact signature bytes did not satisfy the selected profile for the
/// supplied content hash and the verifier's exact external key bytes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SignatureRejected {
    failure: SignatureFailure,
}

impl fmt::Debug for SignatureRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignatureRejected")
            .field("failure", &self.failure)
            .finish()
    }
}

impl fmt::Display for SignatureRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("signature rejected by the selected signature profile")
    }
}

impl std::error::Error for SignatureRejected {}

/// A verifier bound to one explicit profile and one exact admissible external
/// key.
///
/// The normative APIs accept fixed-size arrays, so key and signature width are
/// type-boundary requirements:
///
/// ```compile_fail,E0308
/// use magpie_log::signature_profile::{
///     ProfiledSignatureVerifier, SignatureVerificationProfile,
/// };
///
/// let profile = SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1;
/// let _ = ProfiledSignatureVerifier::new(profile, [0_u8; 31]);
/// ```
///
/// ```compile_fail,E0308
/// use magpie_log::signature_profile::ProfiledSignatureVerifier;
///
/// fn wrong_signature_width(verifier: &ProfiledSignatureVerifier) {
///     let _ = verifier.verify(&[0_u8; 32], &[0_u8; 63]);
/// }
/// ```
pub struct ProfiledSignatureVerifier {
    profile: SignatureVerificationProfile,
    external_key_bytes: [u8; 32],
    external_key_point: EdwardsPoint,
}

impl ProfiledSignatureVerifier {
    /// Bind a verifier to an explicitly selected profile and exact external key
    /// bytes.
    ///
    /// Construction performs the key gates once, in order: canonical point
    /// decoding, non-identity, then exact prime-subgroup membership. Passing
    /// these structural gates does not establish trust in the key.
    pub fn new(
        profile: SignatureVerificationProfile,
        external_key_bytes: [u8; 32],
    ) -> Result<Self, ExternalKeyRejected> {
        let external_key_point =
            decode_canonical_point(&external_key_bytes).map_err(|failure| ExternalKeyRejected {
                failure: ExternalKeyFailure::NonCanonical(failure),
            })?;

        if external_key_point.is_identity() {
            return Err(ExternalKeyRejected {
                failure: ExternalKeyFailure::Identity,
            });
        }
        if !external_key_point.is_torsion_free() {
            return Err(ExternalKeyRejected {
                failure: ExternalKeyFailure::NotPrimeSubgroup,
            });
        }

        Ok(Self {
            profile,
            external_key_bytes,
            external_key_point,
        })
    }

    /// Verify one exact 64-byte `R || S` signature over
    /// `"magpie-sig-v1" || content_hash`.
    ///
    /// This enforces canonical `R`, exact prime-subgroup membership for `R`
    /// (while permitting canonical identity), canonical `S < L`, the ordinary
    /// Ed25519 SHA-512 challenge, and the uncofactored
    /// `[S]B = R + [k]A` equation.
    pub fn verify(
        &self,
        content_hash: &[u8; 32],
        signature: &[u8; 64],
    ) -> Result<(), SignatureRejected> {
        let mut r_bytes = [0_u8; 32];
        r_bytes.copy_from_slice(&signature[..32]);
        let r_point = decode_canonical_point(&r_bytes).map_err(|failure| SignatureRejected {
            failure: SignatureFailure::NonCanonicalR(failure),
        })?;
        if !r_point.is_torsion_free() {
            return Err(SignatureRejected {
                failure: SignatureFailure::RNotPrimeSubgroup,
            });
        }

        let mut s_bytes = [0_u8; 32];
        s_bytes.copy_from_slice(&signature[32..]);
        let s = Option::<Scalar>::from(Scalar::from_canonical_bytes(s_bytes)).ok_or(
            SignatureRejected {
                failure: SignatureFailure::SNotCanonical,
            },
        )?;

        let mut challenge_hasher = Sha512::new();
        challenge_hasher.update(r_bytes);
        challenge_hasher.update(self.external_key_bytes);
        challenge_hasher.update(SIGNATURE_MESSAGE_DOMAIN);
        challenge_hasher.update(content_hash);
        let challenge_wide: [u8; 64] = challenge_hasher.finalize().into();
        let challenge = Scalar::from_bytes_mod_order_wide(&challenge_wide);

        let left = EdwardsPoint::mul_base(&s);
        let right = r_point + (self.external_key_point * challenge);
        if left == right {
            Ok(())
        } else {
            Err(SignatureRejected {
                failure: SignatureFailure::EquationMismatch,
            })
        }
    }

    /// Return the exact profile selected for this verifier.
    pub fn profile(&self) -> SignatureVerificationProfile {
        self.profile
    }

    /// Return the exact original external key bytes used in the challenge.
    ///
    /// These bytes are a producing coordinate of this primitive. The accessor
    /// does not represent them as trusted or authoritative.
    pub fn external_key_bytes(&self) -> &[u8; 32] {
        &self.external_key_bytes
    }
}

fn encoded_y_is_below_field_modulus(encoded_point: &[u8; 32]) -> bool {
    let mut encoded_y = *encoded_point;
    encoded_y[31] &= 0x7f;

    for index in (0..encoded_y.len()).rev() {
        if encoded_y[index] < FIELD_MODULUS_LE[index] {
            return true;
        }
        if encoded_y[index] > FIELD_MODULUS_LE[index] {
            return false;
        }
    }
    false
}

fn decode_canonical_point(encoded_point: &[u8; 32]) -> Result<EdwardsPoint, CanonicalPointFailure> {
    if !encoded_y_is_below_field_modulus(encoded_point) {
        return Err(CanonicalPointFailure::EncodedYOutOfRange);
    }

    let point = CompressedEdwardsY(*encoded_point)
        .decompress()
        .ok_or(CanonicalPointFailure::NotOnCurve)?;
    if point.compress().to_bytes() != *encoded_point {
        return Err(CanonicalPointFailure::NonCanonicalReencoding);
    }

    Ok(point)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN_KEY_HEX: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";
    const K15_KEY_HEX: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d2ac";
    const GOLDEN_HASH_HEX: &str =
        "cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f";
    const GOLDEN_SIGNATURE_HEX: &str = concat!(
        "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db",
        "4421f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
    );
    const SECOND_GOLDEN_R_HEX: &str =
        "132738d446a7b4e9d1c9b27b4588e81d197bd6d0965bc8db11d1d29e0a9e2d7c";
    const D2_IDENTITY_R_SIGNATURE_HEX: &str = concat!(
        "0100000000000000000000000000000000000000000000000000000000000000",
        "0fe717c89f31e5f1d8ee5bf0a2c1bde5734344ec13f2891593b3969c587c3504",
    );
    const MIXED_TORSION_HEX: &str =
        "5252cc0a7f208133b620acbd4537eba2a4123bf0a8c2e4f980c3b31bb69765ea";

    fn decode_hex<const LENGTH: usize>(value: &str) -> [u8; LENGTH] {
        assert_eq!(value.len(), LENGTH * 2);
        let mut decoded = [0_u8; LENGTH];
        hex::decode_to_slice(value, &mut decoded).expect("test vector must be hexadecimal");
        decoded
    }

    fn selected_profile() -> SignatureVerificationProfile {
        SignatureVerificationProfile::CANONICAL_PRIME_SUBGROUP_V1
    }

    fn golden_verifier() -> ProfiledSignatureVerifier {
        ProfiledSignatureVerifier::new(selected_profile(), decode_hex(GOLDEN_KEY_HEX))
            .expect("the frozen golden key must be admissible")
    }

    fn external_key_failure(bytes: [u8; 32]) -> ExternalKeyFailure {
        match ProfiledSignatureVerifier::new(selected_profile(), bytes) {
            Ok(_) => panic!("hostile external key unexpectedly succeeded"),
            Err(error) => error.failure,
        }
    }

    fn signature_with_parts(r_bytes: [u8; 32], s_bytes: [u8; 32]) -> [u8; 64] {
        let mut signature = [0_u8; 64];
        signature[..32].copy_from_slice(&r_bytes);
        signature[32..].copy_from_slice(&s_bytes);
        signature
    }

    fn signature_failure(signature: [u8; 64]) -> SignatureFailure {
        golden_verifier()
            .verify(&decode_hex(GOLDEN_HASH_HEX), &signature)
            .expect_err("hostile signature unexpectedly succeeded")
            .failure
    }

    fn scalar_order_le() -> [u8; 32] {
        // L = 2^252 + 27742317777372353535851937790883648493.
        let mut order = [0_u8; 32];
        order[..16].copy_from_slice(&27742317777372353535851937790883648493_u128.to_le_bytes());
        order[31] = 0x10;
        order
    }

    fn increment_le(value: &mut [u8; 32]) {
        for byte in value {
            let (next, carry) = byte.overflowing_add(1);
            *byte = next;
            if !carry {
                return;
            }
        }
        panic!("test value overflowed");
    }

    fn decrement_le(value: &mut [u8; 32]) {
        for byte in value {
            let (next, borrow) = byte.overflowing_sub(1);
            *byte = next;
            if !borrow {
                return;
            }
        }
        panic!("test value underflowed");
    }

    #[test]
    fn external_key_gates_report_the_intended_reason() {
        let golden_key = decode_hex(GOLDEN_KEY_HEX);
        let golden = ProfiledSignatureVerifier::new(selected_profile(), golden_key)
            .expect("ordinary frozen golden key must pass");
        assert_eq!(golden.external_key_bytes(), &golden_key);

        let k15_key = decode_hex(K15_KEY_HEX);
        ProfiledSignatureVerifier::new(selected_profile(), k15_key)
            .expect("K15 sign-bit-one prime-subgroup key must pass");

        let y_equals_p = FIELD_MODULUS_LE;
        assert_eq!(
            external_key_failure(y_equals_p),
            ExternalKeyFailure::NonCanonical(CanonicalPointFailure::EncodedYOutOfRange)
        );

        let mut y_equals_p_plus_one = FIELD_MODULUS_LE;
        increment_le(&mut y_equals_p_plus_one);
        assert_eq!(
            external_key_failure(y_equals_p_plus_one),
            ExternalKeyFailure::NonCanonical(CanonicalPointFailure::EncodedYOutOfRange)
        );

        let mut y_equals_p_minus_one = FIELD_MODULUS_LE;
        decrement_le(&mut y_equals_p_minus_one);
        assert!(encoded_y_is_below_field_modulus(&y_equals_p_minus_one));
        decode_canonical_point(&y_equals_p_minus_one)
            .expect("y=p-1 is a canonical order-two point");
        assert_eq!(
            external_key_failure(y_equals_p_minus_one),
            ExternalKeyFailure::NotPrimeSubgroup
        );

        let mut y_ffed_regression = [0_u8; 32];
        y_ffed_regression[0] = 0xed;
        y_ffed_regression[1] = 0xff;
        assert!(encoded_y_is_below_field_modulus(&y_ffed_regression));

        let non_square =
            decode_hex("0200000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(
            external_key_failure(non_square),
            ExternalKeyFailure::NonCanonical(CanonicalPointFailure::NotOnCurve)
        );

        let x_zero_sign_one =
            decode_hex("0100000000000000000000000000000000000000000000000000000000000080");
        assert_eq!(
            external_key_failure(x_zero_sign_one),
            ExternalKeyFailure::NonCanonical(CanonicalPointFailure::NonCanonicalReencoding)
        );

        let identity =
            decode_hex("0100000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(external_key_failure(identity), ExternalKeyFailure::Identity);

        let order_two =
            decode_hex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        assert_eq!(
            external_key_failure(order_two),
            ExternalKeyFailure::NotPrimeSubgroup
        );

        let order_four = [0_u8; 32];
        assert_eq!(
            external_key_failure(order_four),
            ExternalKeyFailure::NotPrimeSubgroup
        );

        assert_eq!(
            external_key_failure(decode_hex(MIXED_TORSION_HEX)),
            ExternalKeyFailure::NotPrimeSubgroup
        );
    }

    #[test]
    fn signature_r_gates_and_identity_rule_report_the_intended_reason() {
        let content_hash = decode_hex(GOLDEN_HASH_HEX);
        golden_verifier()
            .verify(&content_hash, &decode_hex(GOLDEN_SIGNATURE_HEX))
            .expect("ordinary frozen signature must pass");
        golden_verifier()
            .verify(&content_hash, &decode_hex(D2_IDENTITY_R_SIGNATURE_HEX))
            .expect("A21-D2 canonical identity R equation must pass");

        let identity =
            decode_hex("0100000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(
            signature_failure(signature_with_parts(identity, [0_u8; 32])),
            SignatureFailure::EquationMismatch
        );

        let order_two =
            decode_hex("ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
        assert_eq!(
            signature_failure(signature_with_parts(order_two, [0_u8; 32])),
            SignatureFailure::RNotPrimeSubgroup
        );

        assert_eq!(
            signature_failure(signature_with_parts([0_u8; 32], [0_u8; 32])),
            SignatureFailure::RNotPrimeSubgroup
        );

        assert_eq!(
            signature_failure(signature_with_parts(
                decode_hex(MIXED_TORSION_HEX),
                [0_u8; 32],
            )),
            SignatureFailure::RNotPrimeSubgroup
        );

        assert_eq!(
            signature_failure(signature_with_parts(FIELD_MODULUS_LE, [0_u8; 32])),
            SignatureFailure::NonCanonicalR(CanonicalPointFailure::EncodedYOutOfRange)
        );

        let non_square =
            decode_hex("0200000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(
            signature_failure(signature_with_parts(non_square, [0_u8; 32])),
            SignatureFailure::NonCanonicalR(CanonicalPointFailure::NotOnCurve)
        );

        let x_zero_sign_one =
            decode_hex("0100000000000000000000000000000000000000000000000000000000000080");
        assert_eq!(
            signature_failure(signature_with_parts(x_zero_sign_one, [0_u8; 32])),
            SignatureFailure::NonCanonicalR(CanonicalPointFailure::NonCanonicalReencoding)
        );
    }

    #[test]
    fn signature_s_boundaries_are_derived_from_l_and_reach_the_intended_gate() {
        let golden_r =
            decode_hex("52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db");

        assert_eq!(
            signature_failure(signature_with_parts(golden_r, [0_u8; 32])),
            SignatureFailure::EquationMismatch
        );

        let mut l_minus_one = scalar_order_le();
        decrement_le(&mut l_minus_one);
        assert_eq!(
            signature_failure(signature_with_parts(golden_r, l_minus_one)),
            SignatureFailure::EquationMismatch
        );

        let l = scalar_order_le();
        assert_eq!(
            signature_failure(signature_with_parts(golden_r, l)),
            SignatureFailure::SNotCanonical
        );

        let mut l_plus_one = scalar_order_le();
        increment_le(&mut l_plus_one);
        assert_eq!(
            signature_failure(signature_with_parts(golden_r, l_plus_one)),
            SignatureFailure::SNotCanonical
        );

        assert_eq!(
            signature_failure(signature_with_parts(golden_r, [0xff_u8; 32])),
            SignatureFailure::SNotCanonical
        );
    }

    #[test]
    fn equation_binds_each_valid_input_independently() {
        let content_hash = decode_hex(GOLDEN_HASH_HEX);
        let golden_signature = decode_hex(GOLDEN_SIGNATURE_HEX);
        golden_verifier()
            .verify(&content_hash, &golden_signature)
            .expect("starting frozen equation must be valid");

        let mut changed_hash = content_hash;
        changed_hash[0] ^= 1;
        assert_eq!(
            golden_verifier()
                .verify(&changed_hash, &golden_signature)
                .expect_err("changed content hash unexpectedly verified")
                .failure,
            SignatureFailure::EquationMismatch
        );

        let alternate_key_verifier =
            ProfiledSignatureVerifier::new(selected_profile(), decode_hex(K15_KEY_HEX))
                .expect("alternate K15 key must pass every key gate");
        assert_eq!(
            alternate_key_verifier
                .verify(&content_hash, &golden_signature)
                .expect_err("changed admissible external key unexpectedly verified")
                .failure,
            SignatureFailure::EquationMismatch
        );

        let mut changed_r_signature = golden_signature;
        changed_r_signature[..32].copy_from_slice(&decode_hex::<32>(SECOND_GOLDEN_R_HEX));
        assert_eq!(
            golden_verifier()
                .verify(&content_hash, &changed_r_signature)
                .expect_err("changed canonical prime-subgroup R unexpectedly verified")
                .failure,
            SignatureFailure::EquationMismatch
        );

        let mut changed_s = [0_u8; 32];
        changed_s.copy_from_slice(&golden_signature[32..]);
        increment_le(&mut changed_s);
        assert!(Option::<Scalar>::from(Scalar::from_canonical_bytes(changed_s)).is_some());
        let mut changed_s_signature = golden_signature;
        changed_s_signature[32..].copy_from_slice(&changed_s);
        assert_eq!(
            golden_verifier()
                .verify(&content_hash, &changed_s_signature)
                .expect_err("changed canonical S unexpectedly verified")
                .failure,
            SignatureFailure::EquationMismatch
        );
    }
}
