use crate::signature_profile::V_SIG_PROFILE_ID;
use crate::FileStore;

use super::conformer::{
    prepare_complete_history, CompleteHistoryError, CompleteHistoryOutcome, PortableRejection,
    PreparedCompleteHistory,
};

enum PreparedFileImage {
    Ready {
        verifier: PreparedCompleteHistory,
        bytes: Vec<u8>,
    },
    Rejected(PortableRejection),
}

pub(super) struct FileVerificationTrace {
    pub(super) outcome: CompleteHistoryOutcome,
    pub(super) hashes: Vec<crate::ContentHash>,
    pub(super) acquired_image: Option<Vec<u8>>,
}

#[cfg(test)]
std::thread_local! {
    /// Test-only observation of the exact public call on the current test
    /// thread. This lets the frozen exporter inspect governed detail and the
    /// acquired bytes without reopening the file or widening the public API.
    static PUBLIC_FILE_TRACE: std::cell::RefCell<Option<FileVerificationTrace>> =
        const { std::cell::RefCell::new(None) };
}

/// Operational failures from the portable `FileStore` adapter.
#[derive(Debug)]
pub(crate) enum FileVerificationError {
    Read(std::io::Error),
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

impl FileVerificationError {
    fn into_io_error(self) -> std::io::Error {
        match self {
            Self::Read(error) => error,
            Self::Verification(CompleteHistoryError::EventCountExhausted) => {
                std::io::Error::other("portable verification event count is exhausted")
            }
            Self::Verification(CompleteHistoryError::UnsupportedProfile(_)) => {
                std::io::Error::other("selected portable verification profile is unavailable")
            }
            Self::Verification(CompleteHistoryError::Frontend(_)) => {
                std::io::Error::other("portable frontend failed operationally")
            }
        }
    }
}

impl From<std::io::Error> for FileVerificationError {
    fn from(error: std::io::Error) -> Self {
        Self::Read(error)
    }
}

impl From<CompleteHistoryError> for FileVerificationError {
    fn from(error: CompleteHistoryError) -> Self {
        Self::Verification(error)
    }
}

fn prepare_and_acquire_file(
    store: &FileStore,
    external_key_text: &str,
) -> Result<PreparedFileImage, FileVerificationError> {
    match prepare_complete_history(V_SIG_PROFILE_ID, external_key_text)? {
        Ok(verifier) => Ok(PreparedFileImage::Ready {
            verifier,
            bytes: store.read_portable_input_bytes()?,
        }),
        Err(rejection) => Ok(PreparedFileImage::Rejected(rejection)),
    }
}

/// Verify the exact finite bytes acquired from the real `FileStore` surface.
///
/// Profile selection and the complete external-key gate finish before the
/// adapter attempts to acquire the file image. Legacy `LogStore` record
/// splitting is deliberately not involved. The governed detail, accepted
/// hashes, and acquired image remain private and are discarded by the public
/// facade outside test observation.
pub(crate) fn verify_complete_file(
    store: &FileStore,
    external_key_text: &str,
) -> Result<FileVerificationTrace, FileVerificationError> {
    match prepare_and_acquire_file(store, external_key_text)? {
        PreparedFileImage::Ready { verifier, bytes } => {
            let (outcome, hashes) = verifier.verify_with_trace(&bytes)?;
            Ok(FileVerificationTrace {
                outcome,
                hashes,
                acquired_image: Some(bytes),
            })
        }
        PreparedFileImage::Rejected(rejection) => Ok(FileVerificationTrace {
            outcome: CompleteHistoryOutcome::Reject(rejection),
            hashes: Vec::new(),
            acquired_image: None,
        }),
    }
}

impl FileStore {
    /// Verify the exact current file image under the selected portable profile.
    ///
    /// The complete external-key gate for
    /// `magpie-ed25519-canonical-prime-subgroup-v1` finishes before the file is
    /// opened. The file is then read once as bytes, without legacy record
    /// splitting or newline normalization, and that exact image is verified.
    ///
    /// `Ok(true)` is portable `ACCEPT`, `Ok(false)` is a governed portable
    /// `REJECT`, and `Err` is an operational failure. The detailed governed
    /// result remains crate-internal. In particular, a missing path is an I/O
    /// error; only an existing zero-byte file is the accepted empty snapshot.
    ///
    /// Acceptance verifies only this caller-selected finite history under this
    /// caller-supplied key. It does not establish key trust or ownership,
    /// authority, freshness, currentness, canonical-branch status, or global
    /// completeness.
    pub fn verify_portable_history(
        &self,
        external_verifying_key_hex: &str,
    ) -> std::io::Result<bool> {
        #[cfg(test)]
        PUBLIC_FILE_TRACE.with(|slot| *slot.borrow_mut() = None);
        let trace = verify_complete_file(self, external_verifying_key_hex)
            .map_err(FileVerificationError::into_io_error)?;
        let accepted = matches!(trace.outcome, CompleteHistoryOutcome::Accept(_));
        #[cfg(test)]
        PUBLIC_FILE_TRACE.with(|slot| *slot.borrow_mut() = Some(trace));
        Ok(accepted)
    }
}

#[cfg(test)]
pub(super) fn take_public_file_trace() -> Option<FileVerificationTrace> {
    PUBLIC_FILE_TRACE.with(|slot| slot.borrow_mut().take())
}
