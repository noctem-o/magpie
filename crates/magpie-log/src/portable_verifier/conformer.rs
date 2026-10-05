use crate::signature_profile::UnsupportedSignatureVerificationProfile;
use crate::{ContentHash, Payload, SignedEvent, CANONICALIZATION_PROFILE};

use super::frontend::{FrontendOperationalError, FrontendSession, FrontendStep, PendingRecord};
use super::lexical::decode_lower_hex;
use super::preflight::{FrontendStartError, PreparedVerifier};
use super::{FrontendRejection, FrontendRejectionClass};

/// The ten governed portable-verifier rejection classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortableRejectionClass {
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
pub struct PortableRejection {
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

    pub fn class(&self) -> PortableRejectionClass {
        self.class
    }

    pub fn line(&self) -> Option<usize> {
        self.line
    }

    pub fn record_index(&self) -> Option<usize> {
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
    EventRetentionAllocation,
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

trait AcceptedRecordObserver {
    fn observe(&mut self, event: SignedEvent) -> Result<(), CompleteHistoryError>;
}

struct IgnoreAcceptedHashes;

impl AcceptedRecordObserver for IgnoreAcceptedHashes {
    fn observe(&mut self, _event: SignedEvent) -> Result<(), CompleteHistoryError> {
        Ok(())
    }
}

struct CollectAcceptedHashes<'trace>(&'trace mut Vec<ContentHash>);

impl AcceptedRecordObserver for CollectAcceptedHashes<'_> {
    fn observe(&mut self, event: SignedEvent) -> Result<(), CompleteHistoryError> {
        self.0.push(event.hash);
        Ok(())
    }
}

struct CollectAcceptedEvents(Vec<SignedEvent>);

impl CollectAcceptedEvents {
    fn reserve(&mut self, additional: usize) -> Result<(), CompleteHistoryError> {
        self.0
            .try_reserve(additional)
            .map_err(|_| CompleteHistoryError::EventRetentionAllocation)
    }
}

impl AcceptedRecordObserver for CollectAcceptedEvents {
    fn observe(&mut self, event: SignedEvent) -> Result<(), CompleteHistoryError> {
        self.reserve(1)?;
        self.0.push(event);
        Ok(())
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
        O: AcceptedRecordObserver,
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
                    let (continuation, event) = pending.release_after_semantic_success(success);
                    self.session = continuation;
                    self.state = candidate_state;
                    observer.observe(event)?;
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

/// Operational/configuration failures, outside the frozen portable rejection law.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortableHistoryOperationalErrorV0 {
    UnsupportedProfile(UnsupportedSignatureVerificationProfile),
    SchemaDecoderInconsistency,
    EventCountExhausted,
    EventRetentionAllocation,
}

/// A governed rejection is distinct from inability to complete verification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortableHistoryVerificationErrorV0 {
    Rejected(PortableRejection),
    Operational(PortableHistoryOperationalErrorV0),
}

impl From<CompleteHistoryError> for PortableHistoryVerificationErrorV0 {
    fn from(error: CompleteHistoryError) -> Self {
        Self::Operational(match error {
            CompleteHistoryError::UnsupportedProfile(error) => {
                PortableHistoryOperationalErrorV0::UnsupportedProfile(error)
            }
            CompleteHistoryError::Frontend(
                FrontendOperationalError::SchemaDecoderInconsistency,
            ) => PortableHistoryOperationalErrorV0::SchemaDecoderInconsistency,
            CompleteHistoryError::EventCountExhausted => {
                PortableHistoryOperationalErrorV0::EventCountExhausted
            }
            CompleteHistoryError::EventRetentionAllocation => {
                PortableHistoryOperationalErrorV0::EventRetentionAllocation
            }
        })
    }
}

impl std::fmt::Display for PortableHistoryVerificationErrorV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "portable history verification: {self:?}")
    }
}

impl std::error::Error for PortableHistoryVerificationErrorV0 {}

/// Profile/key preflight, with no bound or inspected history yet.
/// No public constructor or deserializer can skip the external-key gate.
pub struct PreparedPortableHistoryV0 {
    verifier: PreparedCompleteHistory,
    verification_key_bytes: [u8; 32],
}

/// Explicit portable profile/key preflight before history acquisition or hashing.
/// Key admissibility does not establish trust in the caller-selected root.
pub fn prepare_portable_history_v0(
    profile_identity: &str,
    external_key_text: &str,
) -> Result<PreparedPortableHistoryV0, PortableHistoryVerificationErrorV0> {
    let verifier = prepare_complete_history(profile_identity, external_key_text)?
        .map_err(PortableHistoryVerificationErrorV0::Rejected)?;
    let verification_key_bytes = verifier.frontend.external_key_bytes();
    Ok(PreparedPortableHistoryV0 {
        verifier,
        verification_key_bytes,
    })
}

impl PreparedPortableHistoryV0 {
    /// Verify this exact immutable byte image through the complete conformer.
    /// No reparsing, compatibility verifier, or projection runs on a rejected prefix.
    pub fn verify(
        self,
        history: &[u8],
    ) -> Result<VerifiedPortableHistoryV0, PortableHistoryVerificationErrorV0> {
        let mut collector = CollectAcceptedEvents(Vec::new());
        let outcome = self.verifier.bind(history).run(&mut collector)?;
        match outcome {
            CompleteHistoryOutcome::Accept(summary) => Ok(VerifiedPortableHistoryV0 {
                events: collector.0,
                summary,
                verification_key_bytes: self.verification_key_bytes,
            }),
            CompleteHistoryOutcome::Reject(rejection) => {
                Err(PortableHistoryVerificationErrorV0::Rejected(rejection))
            }
        }
    }
}

/// Complete portable verification of one retained event vector, read-only.
/// Only this conformer mints it, after the entire input reaches ACCEPT.
/// It grants replay, not key trust, truth, standing, or write authority.
///
/// Raw events cannot mint the capability:
/// ```compile_fail
/// use magpie_log::{SignedEvent, VerifiedPortableHistoryV0};
/// fn forge(events: Vec<SignedEvent>) -> VerifiedPortableHistoryV0 {
///     VerifiedPortableHistoryV0 { events }
/// }
/// ```
/// ```compile_fail
/// use magpie_log::{SignedEvent, VerifiedPortableHistoryV0};
/// fn forge(events: Vec<SignedEvent>) -> VerifiedPortableHistoryV0 { events.into() }
/// ```
/// ```compile_fail
/// let _: magpie_log::VerifiedPortableHistoryV0 = serde_json::from_str("{}").unwrap();
/// ```
/// ```compile_fail
/// fn write(history: &mut magpie_log::VerifiedPortableHistoryV0) {
///     history.append_record(b"raw");
/// }
/// ```
#[derive(Debug)]
pub struct VerifiedPortableHistoryV0 {
    events: Vec<SignedEvent>,
    summary: AcceptedHistory,
    verification_key_bytes: [u8; 32],
}

impl VerifiedPortableHistoryV0 {
    pub fn event_count(&self) -> u64 {
        self.summary.event_count()
    }
    pub fn tip(&self) -> ContentHash {
        self.summary.tip()
    }
    pub fn verification_key_bytes(&self) -> &[u8; 32] {
        &self.verification_key_bytes
    }

    // Raw observation cannot mint another capability or replay wrapper.
    pub(crate) fn events(&self) -> &[SignedEvent] {
        &self.events
    }

    /// Present only this completely verified vector through the existing projection boundary.
    pub fn replay_into<P: crate::Projection>(
        &self,
        projection: &mut P,
    ) -> crate::VerifiedReplaySummary {
        for index in 0..self.events.len() {
            let event = crate::VerifiedReplayEvent::from_portable_history(self, index);
            projection.apply(&event);
        }
        crate::VerifiedReplaySummary::from_verified_parts(self.event_count(), self.tip())
    }

    /// Apply the existing checkpoint law to this vector, including any verified suffix.
    pub fn evaluate_expectation(
        &self,
        checkpoint: crate::HistoryCheckpointV0,
        relation: crate::HistoryExpectationRelationV0,
    ) -> crate::HistoryExpectationEvaluationV0 {
        let observed = if checkpoint.event_count() == 0 {
            Some(ContentHash::ZERO)
        } else {
            usize::try_from(checkpoint.event_count() - 1)
                .ok()
                .and_then(|index| self.events.get(index))
                .map(|event| event.hash)
        };
        crate::history_expectation::evaluate_verified_history_expectation_v0(
            self.verification_key_bytes,
            crate::VerifiedReplaySummary::from_verified_parts(self.event_count(), self.tip()),
            checkpoint,
            relation,
            observed,
        )
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
    fn retention_inability_is_operational_and_never_accepts_a_prefix() {
        let mut collector = CollectAcceptedEvents(Vec::new());
        let error = collector.reserve(usize::MAX).unwrap_err();
        assert_eq!(error, CompleteHistoryError::EventRetentionAllocation);
        assert!(collector.0.is_empty());
        assert_eq!(
            PortableHistoryVerificationErrorV0::from(error),
            PortableHistoryVerificationErrorV0::Operational(
                PortableHistoryOperationalErrorV0::EventRetentionAllocation
            )
        );
        struct Fails;
        impl AcceptedRecordObserver for Fails {
            fn observe(&mut self, _: SignedEvent) -> Result<(), CompleteHistoryError> {
                Err(CompleteHistoryError::EventRetentionAllocation)
            }
        }
        let mut bytes = include_bytes!("../../testdata/golden-v1.jsonl").to_vec();
        bytes.extend_from_slice(b"{malformed later record");
        let prepared =
            prepare_complete_history(crate::signature_profile::V_SIG_PROFILE_ID, GOLDEN_KEY)
                .unwrap()
                .unwrap();
        assert_eq!(
            prepared.bind(&bytes).run(&mut Fails),
            Err(CompleteHistoryError::EventRetentionAllocation)
        );
    }

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
