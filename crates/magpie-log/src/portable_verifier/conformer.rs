use crate::signature_profile::UnsupportedSignatureVerificationProfile;
use crate::{ContentHash, Payload, CANONICALIZATION_PROFILE};

use super::frontend::{FrontendOperationalError, FrontendSession, FrontendStep, PendingRecord};
use super::lexical::decode_lower_hex;
use super::preflight::{FrontendStartError, PreparedVerifier};
use super::{FrontendRejection, FrontendRejectionClass};

/// The ten governed portable-verifier rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PortableRejectionClass {
    ExternalKey,
    Framing,
    JsonSyntax,
    Schema,
    Sequence,
    PreviousLink,
    ContentHash,
    Signature,
    PayloadValidation,
    Genesis,
}

/// One governed portable-verifier rejection with its frozen coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PortableRejection {
    class: PortableRejectionClass,
    line: Option<usize>,
    record_index: Option<usize>,
}

impl PortableRejection {
    fn current_record(class: PortableRejectionClass, pending: &PendingRecord<'_>) -> Self {
        debug_assert!(matches!(
            class,
            PortableRejectionClass::Sequence
                | PortableRejectionClass::PreviousLink
                | PortableRejectionClass::ContentHash
                | PortableRejectionClass::Signature
                | PortableRejectionClass::PayloadValidation
                | PortableRejectionClass::Genesis
        ));
        Self {
            class,
            line: Some(pending.record().line()),
            record_index: Some(pending.record().record_index()),
        }
    }

    pub(crate) fn class(&self) -> PortableRejectionClass {
        self.class
    }

    pub(crate) fn line(&self) -> Option<usize> {
        self.line
    }

    pub(crate) fn record_index(&self) -> Option<usize> {
        self.record_index
    }
}

impl From<FrontendRejection> for PortableRejection {
    fn from(rejection: FrontendRejection) -> Self {
        let class = match rejection.class() {
            FrontendRejectionClass::ExternalKey => PortableRejectionClass::ExternalKey,
            FrontendRejectionClass::Framing => PortableRejectionClass::Framing,
            FrontendRejectionClass::JsonSyntax => PortableRejectionClass::JsonSyntax,
            FrontendRejectionClass::Schema => PortableRejectionClass::Schema,
        };
        Self {
            class,
            line: rejection.line(),
            record_index: rejection.record_index(),
        }
    }
}

/// The minimum governed complete-history success product.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AcceptedHistory {
    event_count: u64,
    tip: ContentHash,
}

impl AcceptedHistory {
    pub(crate) fn event_count(&self) -> u64 {
        self.event_count
    }

    pub(crate) fn tip(&self) -> ContentHash {
        self.tip
    }
}

/// The complete governed portable-history outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CompleteHistoryOutcome {
    Accept(AcceptedHistory),
    Reject(PortableRejection),
}

/// Configuration and internal failures outside governed ACCEPT/REJECT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CompleteHistoryError {
    UnsupportedProfile(UnsupportedSignatureVerificationProfile),
    Frontend(FrontendOperationalError),
    EventCountExhausted,
}

impl From<FrontendOperationalError> for CompleteHistoryError {
    fn from(error: FrontendOperationalError) -> Self {
        Self::Frontend(error)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HistoryState {
    event_count: u64,
    tip: ContentHash,
}

impl HistoryState {
    const EMPTY: Self = Self {
        event_count: 0,
        tip: ContentHash::ZERO,
    };

    fn accepted_record(self, tip: ContentHash) -> Result<Self, CompleteHistoryError> {
        let event_count = self
            .event_count
            .checked_add(1)
            .ok_or(CompleteHistoryError::EventCountExhausted)?;
        Ok(Self { event_count, tip })
    }
}

/// Non-forgeable success authority for releasing the frontend continuation.
///
/// The type can be named by the sibling frontend module, but its field and
/// constructor are private to this conformer module.
pub(super) struct SemanticSuccess {
    _private: (),
}

impl SemanticSuccess {
    fn after_all_stages() -> Self {
        Self { _private: () }
    }
}

struct CompleteHistoryConformer<'input> {
    session: FrontendSession<'input>,
    state: HistoryState,
}

/// A complete portable verifier whose profile and external key have passed
/// before any history bytes are acquired or framed.
pub(super) struct PreparedCompleteHistory {
    frontend: PreparedVerifier,
}

trait AcceptedHashObserver {
    fn observe(&mut self, hash: ContentHash);
}

struct IgnoreAcceptedHashes;

impl AcceptedHashObserver for IgnoreAcceptedHashes {
    fn observe(&mut self, _hash: ContentHash) {}
}

struct CollectAcceptedHashes<'trace>(&'trace mut Vec<ContentHash>);

impl AcceptedHashObserver for CollectAcceptedHashes<'_> {
    fn observe(&mut self, hash: ContentHash) {
        self.0.push(hash);
    }
}

enum CurrentRecordFailure {
    Rejected(PortableRejection),
    Operational(CompleteHistoryError),
}

impl From<CompleteHistoryError> for CurrentRecordFailure {
    fn from(error: CompleteHistoryError) -> Self {
        Self::Operational(error)
    }
}

impl<'input> CompleteHistoryConformer<'input> {
    fn new(session: FrontendSession<'input>) -> Self {
        Self {
            session,
            state: HistoryState::EMPTY,
        }
    }

    fn run<O>(mut self, observer: &mut O) -> Result<CompleteHistoryOutcome, CompleteHistoryError>
    where
        O: AcceptedHashObserver,
    {
        loop {
            match self.session.next()? {
                FrontendStep::End => {
                    return Ok(CompleteHistoryOutcome::Accept(AcceptedHistory {
                        event_count: self.state.event_count,
                        tip: self.state.tip,
                    }));
                }
                FrontendStep::Rejected(rejection) => {
                    return Ok(CompleteHistoryOutcome::Reject(rejection.into()));
                }
                FrontendStep::Pending(pending) => {
                    let candidate_state = match Self::validate_current(self.state, &pending) {
                        Ok(candidate_state) => candidate_state,
                        Err(CurrentRecordFailure::Rejected(rejection)) => {
                            return Ok(CompleteHistoryOutcome::Reject(rejection));
                        }
                        Err(CurrentRecordFailure::Operational(error)) => return Err(error),
                    };

                    // The checked candidate state exists before success is
                    // minted or the sole continuation is released.
                    let success = SemanticSuccess::after_all_stages();
                    let continuation = pending.release_after_semantic_success(success);
                    self.session = continuation;
                    self.state = candidate_state;
                    observer.observe(candidate_state.tip);
                }
            }
        }
    }

    fn validate_current(
        state: HistoryState,
        pending: &PendingRecord<'_>,
    ) -> Result<HistoryState, CurrentRecordFailure> {
        let record = pending.record();

        if record.core().seq != state.event_count {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(PortableRejectionClass::Sequence, pending),
            ));
        }

        if record.core().prev_hash != state.tip {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(PortableRejectionClass::PreviousLink, pending),
            ));
        }

        let recomputed_hash = record.core().hash();
        if recomputed_hash.as_bytes() != record.stored_hash() {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(PortableRejectionClass::ContentHash, pending),
            ));
        }

        if pending
            .signature_verifier()
            .verify(recomputed_hash.as_bytes(), record.signature())
            .is_err()
        {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(PortableRejectionClass::Signature, pending),
            ));
        }

        if record.core().payload.validate().is_err() {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(
                    PortableRejectionClass::PayloadValidation,
                    pending,
                ),
            ));
        }

        if !genesis_is_valid(pending, state.event_count) {
            return Err(CurrentRecordFailure::Rejected(
                PortableRejection::current_record(PortableRejectionClass::Genesis, pending),
            ));
        }

        Ok(state.accepted_record(recomputed_hash)?)
    }
}

impl PreparedCompleteHistory {
    fn bind(self, history: &[u8]) -> CompleteHistoryConformer<'_> {
        CompleteHistoryConformer::new(FrontendSession::from_prepared(self.frontend, history))
    }

    pub(super) fn verify(
        self,
        history: &[u8],
    ) -> Result<CompleteHistoryOutcome, CompleteHistoryError> {
        self.bind(history).run(&mut IgnoreAcceptedHashes)
    }

    pub(super) fn verify_with_trace(
        self,
        history: &[u8],
    ) -> Result<(CompleteHistoryOutcome, Vec<ContentHash>), CompleteHistoryError> {
        let mut hashes = Vec::new();
        let outcome = self
            .bind(history)
            .run(&mut CollectAcceptedHashes(&mut hashes))?;
        Ok((outcome, hashes))
    }
}

fn genesis_is_valid(pending: &PendingRecord<'_>, event_count: u64) -> bool {
    match (event_count, &pending.record().core().payload) {
        (
            0,
            Payload::Genesis {
                canonicalization_profile,
                verifying_key,
            },
        ) => {
            if canonicalization_profile != CANONICALIZATION_PROFILE {
                return false;
            }
            let Some(declared_key) = decode_lower_hex::<32>(verifying_key) else {
                return false;
            };
            &declared_key == pending.signature_verifier().external_key_bytes()
        }
        (0, _) | (_, Payload::Genesis { .. }) => false,
        (_, _) => true,
    }
}

/// Select the profile and apply the complete external-key gate without
/// acquiring, binding, or inspecting a history input.
pub(super) fn prepare_complete_history(
    profile_identity: &str,
    external_key_text: &str,
) -> Result<Result<PreparedCompleteHistory, PortableRejection>, CompleteHistoryError> {
    match PreparedVerifier::new(profile_identity, external_key_text) {
        Ok(frontend) => Ok(Ok(PreparedCompleteHistory { frontend })),
        Err(FrontendStartError::Rejected(rejection)) => Ok(Err(rejection.into())),
        Err(FrontendStartError::UnsupportedProfile(error)) => {
            Err(CompleteHistoryError::UnsupportedProfile(error))
        }
    }
}

/// Verify one exact immutable portable history through all ten frozen stages.
pub(crate) fn verify_complete_history(
    profile_identity: &str,
    external_key_text: &str,
    history: &[u8],
) -> Result<CompleteHistoryOutcome, CompleteHistoryError> {
    match prepare_complete_history(profile_identity, external_key_text)? {
        Ok(prepared) => prepared.verify(history),
        Err(rejection) => Ok(CompleteHistoryOutcome::Reject(rejection)),
    }
}

#[cfg(test)]
pub(super) fn verify_complete_history_with_trace(
    profile_identity: &str,
    external_key_text: &str,
    history: &[u8],
) -> Result<(CompleteHistoryOutcome, Vec<ContentHash>), CompleteHistoryError> {
    match prepare_complete_history(profile_identity, external_key_text)? {
        Ok(prepared) => prepared.verify_with_trace(history),
        Err(rejection) => Ok((CompleteHistoryOutcome::Reject(rejection), Vec::new())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN_KEY: &str = "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";

    #[test]
    fn checked_count_exhaustion_precedes_success_authority() {
        let old_state = HistoryState {
            event_count: u64::MAX,
            tip: ContentHash::ZERO,
        };

        assert_eq!(
            old_state.accepted_record(ContentHash::ZERO),
            Err(CompleteHistoryError::EventCountExhausted)
        );
        assert_eq!(old_state.event_count, u64::MAX);
        assert_eq!(old_state.tip, ContentHash::ZERO);
    }

    #[test]
    fn unsupported_profile_is_operational_while_external_key_is_governed() {
        assert!(matches!(
            verify_complete_history("latest", GOLDEN_KEY, b""),
            Err(CompleteHistoryError::UnsupportedProfile(_))
        ));

        match verify_complete_history(
            crate::signature_profile::V_SIG_PROFILE_ID,
            "not-a-key",
            b"{malformed",
        )
        .unwrap()
        {
            CompleteHistoryOutcome::Reject(rejection) => {
                assert_eq!(rejection.class(), PortableRejectionClass::ExternalKey);
                assert_eq!(rejection.line(), None);
                assert_eq!(rejection.record_index(), None);
            }
            actual => panic!("invalid key must be governed ExternalKey: {actual:?}"),
        }
    }
}
