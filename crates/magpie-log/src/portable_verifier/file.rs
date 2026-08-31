use crate::{FileStore, LogError};

use super::conformer::{prepare_complete_history, CompleteHistoryError, CompleteHistoryOutcome};

/// Operational failures from the crate-internal portable `FileStore` adapter.
#[derive(Debug)]
pub(crate) enum FileVerificationError {
    Read(LogError),
    Verification(CompleteHistoryError),
}

impl std::fmt::Display for FileVerificationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(formatter, "portable file read failed: {error}"),
            Self::Verification(error) => {
                write!(
                    formatter,
                    "portable verification failed operationally: {error:?}"
                )
            }
        }
    }
}

impl std::error::Error for FileVerificationError {}

impl From<LogError> for FileVerificationError {
    fn from(error: LogError) -> Self {
        Self::Read(error)
    }
}

impl From<CompleteHistoryError> for FileVerificationError {
    fn from(error: CompleteHistoryError) -> Self {
        Self::Verification(error)
    }
}

/// Verify the exact finite bytes acquired from the real `FileStore` surface.
///
/// Profile selection and the complete external-key gate finish before the
/// adapter attempts to acquire the file image. Legacy `LogStore` record
/// splitting is deliberately not involved.
pub(crate) fn verify_complete_file(
    store: &FileStore,
    profile_identity: &str,
    external_key_text: &str,
) -> Result<CompleteHistoryOutcome, FileVerificationError> {
    match prepare_complete_history(profile_identity, external_key_text)? {
        Ok(prepared) => {
            let history = store.read_portable_input_bytes()?;
            Ok(prepared.verify(&history)?)
        }
        Err(rejection) => Ok(CompleteHistoryOutcome::Reject(rejection)),
    }
}

#[cfg(test)]
pub(super) fn verify_complete_file_with_trace(
    store: &FileStore,
    profile_identity: &str,
    external_key_text: &str,
) -> Result<(CompleteHistoryOutcome, Vec<crate::ContentHash>), FileVerificationError> {
    match prepare_complete_history(profile_identity, external_key_text)? {
        Ok(prepared) => {
            let history = store.read_portable_input_bytes()?;
            Ok(prepared.verify_with_trace(&history)?)
        }
        Err(rejection) => Ok((CompleteHistoryOutcome::Reject(rejection), Vec::new())),
    }
}
