use crate::signature_profile::{
    ProfiledSignatureVerifier, SignatureVerificationProfile,
    UnsupportedSignatureVerificationProfile,
};

use super::framing::FrameCursor;
use super::lexical::decode_lower_hex;
use super::FrontendRejection;

#[derive(Debug)]
pub(super) enum FrontendStartError {
    UnsupportedProfile(UnsupportedSignatureVerificationProfile),
    Rejected(FrontendRejection),
}

pub(super) struct PreparedFrontend<'input> {
    verifier: ProfiledSignatureVerifier,
    cursor: FrameCursor<'input>,
}

impl<'input> PreparedFrontend<'input> {
    pub(super) fn new(
        profile_identity: &str,
        external_key_text: &str,
        history: &'input [u8],
    ) -> Result<Self, FrontendStartError> {
        let profile = SignatureVerificationProfile::from_identity(profile_identity)
            .map_err(FrontendStartError::UnsupportedProfile)?;
        let external_key_bytes = decode_lower_hex::<32>(external_key_text)
            .ok_or_else(|| FrontendStartError::Rejected(FrontendRejection::external_key()))?;
        let verifier = ProfiledSignatureVerifier::new(profile, external_key_bytes)
            .map_err(|_| FrontendStartError::Rejected(FrontendRejection::external_key()))?;

        // Creating the cursor is intentionally last. No history byte is read
        // until profile selection, transport decoding, and structural key
        // admissibility have all succeeded.
        Ok(Self {
            verifier,
            cursor: FrameCursor::new(history),
        })
    }

    pub(super) fn into_parts(self) -> (ProfiledSignatureVerifier, FrameCursor<'input>) {
        (self.verifier, self.cursor)
    }
}
