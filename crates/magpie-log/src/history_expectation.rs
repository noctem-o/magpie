use crate::hashing::ContentHash;
use crate::logimpl::{LogReader, Projection, VerifiedReplayEvent, VerifiedReplaySummary};
use crate::store::LogStore;
use crate::LogError;

/// Immutable identity of the exact history-expectation semantics implemented here.
///
/// This versioned identity freezes the complete v0 interpretation: Magpie's
/// existing complete `magpie-core-v1` replay verification under the exact
/// externally supplied verification key; event-count positions; the event hash
/// as the commitment through a non-empty position; zero as the empty-position
/// commitment; the two explicit relations; and the two semantic outcomes.
/// Historical-input verification interpretation is part of this profile where
/// it affects evaluation. Any normative change requires a different immutable
/// profile identity rather than silently reusing this one.
pub const HISTORY_EXPECTATION_PROFILE_V0_ID: &str = "magpie-log-history-expectation-v0";

/// The exact frozen semantic profile selected by
/// [`LogReader::evaluate_history_expectation_v0`].
///
/// The concrete v0 API selects this profile without a mutable registry, alias,
/// implementation default, or ambient "latest" lookup. Values are produced by
/// successful evaluations so their result can retain the selected semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryExpectationProfileV0 {
    _private: (),
}

impl HistoryExpectationProfileV0 {
    const SELECTED: Self = Self { _private: () };

    /// Return the immutable, versioned identity of this exact semantic profile.
    pub fn id(&self) -> &'static str {
        HISTORY_EXPECTATION_PROFILE_V0_ID
    }
}

/// One explicit caller-supplied historical checkpoint expectation.
///
/// `event_count` is the exact historical position. For position zero,
/// `commitment` is [`ContentHash::ZERO`]. For position `N > 0`, `commitment`
/// is the hash of the event at sequence `N - 1`, which transitively commits to
/// the chain through that position under the v0 profile.
///
/// A checkpoint is expectation material, not a credential. Constructing one
/// establishes no authenticity, trusted origin, authority, freshness, prior
/// observation, persistence, or rollback resistance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryCheckpointV0 {
    event_count: u64,
    commitment: ContentHash,
}

impl HistoryCheckpointV0 {
    /// Construct an arbitrary explicit checkpoint expectation.
    pub fn new(event_count: u64, commitment: ContentHash) -> Self {
        Self {
            event_count,
            commitment,
        }
    }

    /// Return the exact expected event-count position.
    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    /// Return the exact expected chain commitment through that position.
    pub fn commitment(&self) -> ContentHash {
        self.commitment
    }
}

/// The explicitly selected v0 relation between verified history and checkpoint.
///
/// There is deliberately no default relation and no fallback between variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryExpectationRelationV0 {
    /// The verified supplied history must terminate at the exact checkpoint.
    Exact,
    /// The exact checkpoint must occur at its exact position in the verified history.
    ContainsCheckpoint,
}

/// The semantic outcome of one completed v0 expectation evaluation.
///
/// Verification, parsing, and storage failures reported by the evaluator
/// remain [`LogError`] values. Resource exhaustion, interruption, host failure,
/// or any other inability to complete evaluation produces no returned semantic
/// outcome and adds no environmental state to the producing inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryExpectationOutcomeV0 {
    /// The selected relation held for the exact verified supplied history.
    Satisfied,
    /// Verification succeeded, but the selected relation did not hold.
    NotSatisfied,
}

/// A profile-specific, context-bearing result of one completed evaluation.
///
/// `Satisfied` means only that the exact completely verified supplied history
/// satisfied the exact caller-supplied checkpoint under the exact explicit
/// relation and v0 producing context carried here. It does not establish that
/// the history is latest, current, fresh, globally complete, canonical, unique,
/// non-equivocated, trusted, witnessed, authorized, durable, rollback-safe, or
/// securely persisted. Trust in the carried externally supplied verification
/// key remains external.
///
/// The fields are private and there is no public constructor, so a caller
/// cannot replace the outcome while retaining genuine coordinates:
///
/// ```compile_fail,E0451
/// use magpie_log::{
///     ContentHash, HistoryCheckpointV0, HistoryExpectationOutcomeV0,
///     HistoryExpectationRelationV0, LogReader, MemStore, SigningKey,
/// };
///
/// let key = SigningKey::from_bytes(&[7u8; 32]);
/// let reader = LogReader::open(MemStore::new(), key.verifying_key());
/// let genuine = reader
///     .evaluate_history_expectation_v0(
///         HistoryCheckpointV0::new(0, ContentHash::ZERO),
///         HistoryExpectationRelationV0::Exact,
///     )
///     .unwrap();
/// let _fabricated = magpie_log::HistoryExpectationEvaluationV0 {
///     outcome: HistoryExpectationOutcomeV0::NotSatisfied,
///     ..genuine
/// };
/// ```
///
/// Serialized caller material cannot mint an evaluation result:
///
/// ```compile_fail,E0277
/// use magpie_log::HistoryExpectationEvaluationV0;
/// let _: HistoryExpectationEvaluationV0 = serde_json::from_str("{}").unwrap();
/// ```
///
/// A genuine result cannot bless an unrelated raw event or history:
///
/// ```compile_fail,E0599
/// use magpie_log::{HistoryExpectationEvaluationV0, SignedEvent};
/// fn cannot_bless(result: &HistoryExpectationEvaluationV0, raw: &SignedEvent) {
///     result.wrap(raw);
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryExpectationEvaluationV0 {
    profile: HistoryExpectationProfileV0,
    verification_key_bytes: [u8; 32],
    verified_history: VerifiedReplaySummary,
    checkpoint: HistoryCheckpointV0,
    relation: HistoryExpectationRelationV0,
    outcome: HistoryExpectationOutcomeV0,
}

impl HistoryExpectationEvaluationV0 {
    fn new(
        verification_key_bytes: [u8; 32],
        verified_history: VerifiedReplaySummary,
        checkpoint: HistoryCheckpointV0,
        relation: HistoryExpectationRelationV0,
        outcome: HistoryExpectationOutcomeV0,
    ) -> Self {
        Self {
            profile: HistoryExpectationProfileV0::SELECTED,
            verification_key_bytes,
            verified_history,
            checkpoint,
            relation,
            outcome,
        }
    }

    /// Return the exact immutable semantic profile selected for this result.
    pub fn profile(&self) -> HistoryExpectationProfileV0 {
        self.profile
    }

    /// Return the exact externally supplied verification key used by the reader.
    ///
    /// This records the verification coordinate; it does not call the key trusted.
    pub fn verification_key_bytes(&self) -> &[u8; 32] {
        &self.verification_key_bytes
    }

    /// Return the exact count and terminal commitment of the verified snapshot.
    pub fn verified_history(&self) -> VerifiedReplaySummary {
        self.verified_history
    }

    /// Return the exact caller-supplied checkpoint.
    pub fn checkpoint(&self) -> HistoryCheckpointV0 {
        self.checkpoint
    }

    /// Return the exact explicitly selected relation.
    pub fn relation(&self) -> HistoryExpectationRelationV0 {
        self.relation
    }

    /// Return the exact semantic outcome.
    pub fn outcome(&self) -> HistoryExpectationOutcomeV0 {
        self.outcome
    }
}

struct CheckpointObserver {
    target_event_count: u64,
    observed_commitment: Option<ContentHash>,
}

impl CheckpointObserver {
    fn new(target_event_count: u64) -> Self {
        Self {
            target_event_count,
            observed_commitment: (target_event_count == 0).then_some(ContentHash::ZERO),
        }
    }
}

impl Projection for CheckpointObserver {
    fn apply(&mut self, replay_event: &VerifiedReplayEvent<'_>) {
        let event = replay_event.event();
        if event.core.seq.checked_add(1) == Some(self.target_event_count) {
            self.observed_commitment = Some(event.hash);
        }
    }
}

pub(crate) fn evaluate_history_expectation_v0<S: LogStore>(
    reader: &LogReader<S>,
    verification_key_bytes: [u8; 32],
    checkpoint: HistoryCheckpointV0,
    relation: HistoryExpectationRelationV0,
) -> Result<HistoryExpectationEvaluationV0, LogError> {
    let mut observer = CheckpointObserver::new(checkpoint.event_count);
    let verified_history = reader.replay_with_summary(&mut observer)?;

    let satisfied = match relation {
        HistoryExpectationRelationV0::Exact => {
            verified_history.event_count() == checkpoint.event_count
                && verified_history.tip() == checkpoint.commitment
        }
        HistoryExpectationRelationV0::ContainsCheckpoint => {
            observer.observed_commitment == Some(checkpoint.commitment)
        }
    };
    let outcome = if satisfied {
        HistoryExpectationOutcomeV0::Satisfied
    } else {
        HistoryExpectationOutcomeV0::NotSatisfied
    };

    Ok(HistoryExpectationEvaluationV0::new(
        verification_key_bytes,
        verified_history,
        checkpoint,
        relation,
        outcome,
    ))
}
