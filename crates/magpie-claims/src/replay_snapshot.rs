use magpie_log::{LogError, LogReader, LogStore, Projection, SignedEvent, VerifiedReplaySummary};
use serde::Serialize;

use crate::deadbolt_context::DeadboltAnchorIndex;
use crate::standing::StandingView;

/// Standing and exact anchor context co-derived by one verified replay.
///
/// The snapshot is returned only after one successful [`LogReader::replay`]
/// into a private composite projection. The contained [`StandingView`] and
/// [`DeadboltAnchorIndex`] observed the same verified event vector during the
/// same replay invocation.
///
/// Trust in the verifying key supplied to [`LogReader`] remains external. This
/// snapshot does not establish foreign bundle verification, actor authority,
/// admission, achieved standing, interpretation truth, policy wisdom, or
/// scientific correctness.
///
/// Independent `StandingView` and `DeadboltAnchorIndex` projections remain
/// useful derived views, but do not prove shared replay provenance. This type's
/// private fields and sole construction function preserve that distinction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandingReplaySnapshot {
    standing: StandingView,
    anchors: DeadboltAnchorIndex,
    event_count: u64,
}

impl StandingReplaySnapshot {
    pub fn standing(&self) -> &StandingView {
        &self.standing
    }

    pub fn anchors(&self) -> &DeadboltAnchorIndex {
        &self.anchors
    }

    pub fn event_count(&self) -> u64 {
        self.event_count
    }

    /// Deterministic derived bytes for replay-equivalence checks.
    ///
    /// These bytes frame the event count and the existing deterministic bytes
    /// of both projections. They are not Magpie L0 canonical encoding and do
    /// not define a persistent format contract.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        #[derive(Serialize)]
        struct SnapshotBytes<'a> {
            event_count: u64,
            standing: &'a [u8],
            anchors: &'a [u8],
        }

        let standing = self.standing.canonical_bytes();
        let anchors = self.anchors.canonical_bytes();
        serde_json::to_vec(&SnapshotBytes {
            event_count: self.event_count,
            standing: &standing,
            anchors: &anchors,
        })
        .expect("StandingReplaySnapshot components are always serializable")
    }
}

#[derive(Default)]
struct StandingContextProjection {
    standing: StandingView,
    anchors: DeadboltAnchorIndex,
}

impl Projection for StandingContextProjection {
    fn apply(&mut self, event: &SignedEvent) {
        // Fixed order: standing first, then anchor occurrence context. Both
        // receive the exact same event reference from one replay invocation.
        self.standing.apply(event);
        self.anchors.apply(event);
    }
}

/// Co-derive standing and exact anchor context through one verified replay.
///
/// A snapshot is constructed only after the reader successfully replays its
/// complete verified event vector into the private composite projection. Trust
/// in the reader's configured verifying key remains the caller's responsibility.
pub fn replay_standing_context<S: LogStore>(
    reader: &LogReader<S>,
) -> Result<StandingReplaySnapshot, LogError> {
    replay_standing_context_with_summary(reader).map(|(snapshot, _summary)| snapshot)
}

pub(crate) fn replay_standing_context_with_summary<S: LogStore>(
    reader: &LogReader<S>,
) -> Result<(StandingReplaySnapshot, VerifiedReplaySummary), LogError> {
    let mut projection = StandingContextProjection::default();
    let summary = reader.replay_with_summary(&mut projection)?;

    Ok((
        StandingReplaySnapshot {
            standing: projection.standing,
            anchors: projection.anchors,
            event_count: summary.event_count(),
        },
        summary,
    ))
}
