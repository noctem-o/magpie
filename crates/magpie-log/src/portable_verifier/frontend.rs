use crate::signature_profile::ProfiledSignatureVerifier;
use crate::EventCore;

use super::framing::{FrameCursor, FrameStep};
use super::json_syntax;
use super::preflight::{FrontendStartError, PreparedFrontend};
use super::schema::{self, SchemaDecodeError};
use super::{FrontendRejection, FrontendRejectionClass};

/// A ready, crate-internal cursor over one exact immutable history input.
///
/// `next` consumes the ready session. If it yields a [`PendingRecord`], no
/// usable continuation exists outside that pending value.
pub(crate) struct FrontendSession<'input> {
    verifier: ProfiledSignatureVerifier,
    cursor: FrameCursor<'input>,
}

pub(crate) enum FrontendStep<'input> {
    End,
    Rejected(FrontendRejection),
    Pending(PendingRecord<'input>),
}

/// Operational failure after the syntax pass that cannot be explained by the
/// frozen Schema law.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FrontendOperationalError {
    SchemaDecoderInconsistency,
}

/// One frontend-accepted current record.
///
/// The fields are private, the type is crate-internal, and it implements no
/// deserializer or public constructor. It is not a verified event or receipt.
pub(crate) struct FrontendRecord {
    line: usize,
    record_index: usize,
    core: EventCore,
    stored_hash: [u8; 32],
    signature: [u8; 64],
}

/// One accepted current record together with the only continuation.
///
/// Dropping this value terminates traversal. The continuation is returned only
/// when the later history caller explicitly reports success for all remaining
/// current-record semantic stages.
pub(crate) struct PendingRecord<'input> {
    record: FrontendRecord,
    verifier: ProfiledSignatureVerifier,
    continuation: FrameCursor<'input>,
}

impl<'input> FrontendSession<'input> {
    pub(crate) fn new(
        profile_identity: &str,
        external_key_text: &str,
        history: &'input [u8],
    ) -> Result<Self, FrontendStartError> {
        let prepared = PreparedFrontend::new(profile_identity, external_key_text, history)?;
        let (verifier, cursor) = prepared.into_parts();
        Ok(Self { verifier, cursor })
    }

    pub(crate) fn next(self) -> Result<FrontendStep<'input>, FrontendOperationalError> {
        let Self { verifier, cursor } = self;
        let candidate = match cursor.next() {
            FrameStep::End => return Ok(FrontendStep::End),
            FrameStep::Rejected(rejection) => return Ok(FrontendStep::Rejected(rejection)),
            FrameStep::Candidate(candidate) => candidate,
        };

        let line = candidate.line();
        let record_index = candidate.record_index();
        if json_syntax::recognize(candidate.text()).is_err() {
            return Ok(FrontendStep::Rejected(FrontendRejection::current_record(
                FrontendRejectionClass::JsonSyntax,
                line,
                record_index,
            )));
        }

        let decoded = match schema::decode_record(candidate.text()) {
            Ok(decoded) => decoded,
            Err(SchemaDecodeError::Rejected) => {
                return Ok(FrontendStep::Rejected(FrontendRejection::current_record(
                    FrontendRejectionClass::Schema,
                    line,
                    record_index,
                )));
            }
            Err(SchemaDecodeError::InternalParserInconsistency) => {
                return Err(FrontendOperationalError::SchemaDecoderInconsistency);
            }
        };

        let continuation = candidate.into_continuation();
        Ok(FrontendStep::Pending(PendingRecord {
            record: FrontendRecord {
                line,
                record_index,
                core: decoded.core,
                stored_hash: decoded.stored_hash,
                signature: decoded.signature,
            },
            verifier,
            continuation,
        }))
    }
}

impl<'input> PendingRecord<'input> {
    pub(crate) fn record(&self) -> &FrontendRecord {
        &self.record
    }

    pub(crate) fn signature_verifier(&self) -> &ProfiledSignatureVerifier {
        &self.verifier
    }

    /// Consume the pending record and report the later semantic stages' result.
    ///
    /// Only `Ok(())` returns the continuation. This frontend does not implement
    /// or simulate those stages; the conspicuously named test driver is the
    /// only caller in this tranche.
    pub(crate) fn complete_semantics<E>(
        self,
        semantic_result: Result<(), E>,
    ) -> Result<FrontendSession<'input>, E> {
        semantic_result?;
        Ok(FrontendSession {
            verifier: self.verifier,
            cursor: self.continuation,
        })
    }
}

impl FrontendRecord {
    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn record_index(&self) -> usize {
        self.record_index
    }

    pub(crate) fn core(&self) -> &EventCore {
        &self.core
    }

    pub(crate) fn stored_hash(&self) -> &[u8; 32] {
        &self.stored_hash
    }

    pub(crate) fn signature(&self) -> &[u8; 64] {
        &self.signature
    }
}
