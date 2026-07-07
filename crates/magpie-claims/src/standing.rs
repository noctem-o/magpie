use std::collections::BTreeMap;

use magpie_log::{Payload, Projection, SignedEvent, Status};
use serde::Serialize;

/// A claim as seen by ADR-0002's first read-only standing projection.
///
/// The fields deliberately separate the v0 assertion payload from current
/// standing so the future governed write path can replace legacy semantics
/// without rewriting old events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingClaim {
    pub statement: String,
    pub asserted_initial_status: Status,
    pub standing: Status,
    pub legacy_evidence: Vec<String>,
    pub legacy_transitions: u64,
}

/// ADR-0002 standing, derived from the verified append-only log.
///
/// This skeleton folds only the current v0 payload vocabulary. It intentionally
/// does not add claim-writing vocabulary, event tags, or writer-facing surfaces.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct StandingView {
    pub claims: BTreeMap<String, StandingClaim>,
}

impl StandingView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, claim_id: &str) -> Option<&StandingClaim> {
        self.claims.get(claim_id)
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    /// Deterministic projection bytes. This is not L0 canonical encoding.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingView is always serializable")
    }
}

impl Projection for StandingView {
    fn apply(&mut self, event: &SignedEvent) {
        match &event.core.payload {
            Payload::Genesis { .. } => {}
            Payload::ClaimAsserted {
                claim_id,
                statement,
                status,
            } => {
                // v0 compatibility: the asserted status is replayed as legacy standing here.
                // ADR-0002's governed actor/evidence ceilings are not implemented until typed
                // events and EpistemicGate exist.
                self.claims
                    .entry(claim_id.clone())
                    .or_insert_with(|| StandingClaim {
                        statement: statement.clone(),
                        asserted_initial_status: *status,
                        standing: *status,
                        legacy_evidence: Vec::new(),
                        legacy_transitions: 0,
                    });
            }
            Payload::EvidenceRecorded { claim_id, summary } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.legacy_evidence.push(summary.clone());
                }
            }
            Payload::ClaimStatusChanged { claim_id, to, .. } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    // v0/manual compatibility path. Future ADR-0002 work should prefer
                    // ratification-as-edge once typed justification events exist.
                    claim.standing = *to;
                    claim.legacy_transitions += 1;
                }
            }
            Payload::Note { .. } => {}
            Payload::SegmentAnchored { .. } => {
                // SegmentAnchored is occurrence/inclusion evidence, not interpretation truth.
                // Future typed evidence may cite it; this v0 StandingView must not create or
                // promote claims from anchors alone.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magpie_log::{LogReader, LogWriter, MemStore, Provenance, SigningKey};

    const SEED: [u8; 32] = [7u8; 32];

    fn test_key() -> SigningKey {
        SigningKey::from_bytes(&SEED)
    }

    fn writer(store: MemStore) -> LogWriter<MemStore> {
        let mut tick = 0u64;
        LogWriter::open_with_clock(
            store,
            test_key(),
            Box::new(move || {
                tick += 1;
                tick
            }),
        )
        .unwrap()
    }

    fn provenance() -> Provenance {
        Provenance::new("test", "standing-view")
    }

    fn anchor() -> Payload {
        Payload::SegmentAnchored {
            bundle_kind: "observation".into(),
            witness_root: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            witness_algorithm: "sha256".into(),
            canonicalization_profile: "phase5-interim-jcs-like-v1".into(),
            run_id: "run-standing-view".into(),
        }
    }

    #[test]
    fn orphan_evidence_does_not_create_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::EvidenceRecorded {
                        claim_id: "missing-claim".into(),
                        summary: "Evidence without an asserted claim.".into(),
                    },
                )
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        assert!(view.is_empty());
        assert!(view.get("missing-claim").is_none());
        assert_eq!(
            view.canonical_bytes(),
            StandingView::new().canonical_bytes()
        );
    }

    #[test]
    fn orphan_status_change_does_not_create_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimStatusChanged {
                        claim_id: "missing-claim".into(),
                        from: Status::Open,
                        to: Status::Settled,
                        reason: "Manual transition without an asserted claim.".into(),
                    },
                )
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        assert!(view.is_empty());
        assert!(view.get("missing-claim").is_none());
    }

    #[test]
    fn claim_asserted_does_not_overwrite_existing_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "claim-1".into(),
                        statement: "first statement".into(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::EvidenceRecorded {
                        claim_id: "claim-1".into(),
                        summary: "Evidence before duplicate assertion.".into(),
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "claim-1".into(),
                        statement: "second statement".into(),
                        status: Status::Settled,
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::EvidenceRecorded {
                        claim_id: "claim-1".into(),
                        summary: "Evidence after duplicate assertion.".into(),
                    },
                )
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 5);

        let claim = view.get("claim-1").unwrap();
        assert_eq!(claim.statement, "first statement");
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);
        assert_eq!(claim.standing, Status::Conjectured);
        assert_eq!(
            claim.legacy_evidence,
            vec![
                "Evidence before duplicate assertion.".to_string(),
                "Evidence after duplicate assertion.".to_string()
            ]
        );
        assert_eq!(claim.legacy_transitions, 0);
    }

    #[test]
    fn segment_anchor_does_not_affect_existing_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "claim-1".into(),
                        statement: "Anchors do not interpret this claim.".into(),
                        status: Status::Supported,
                    },
                )
                .unwrap();
            writer.append(provenance(), anchor()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 3);

        let claim = view.get("claim-1").unwrap();
        assert_eq!(view.len(), 1);
        assert_eq!(claim.standing, Status::Supported);
        assert!(claim.legacy_evidence.is_empty());
        assert_eq!(claim.legacy_transitions, 0);
    }

    #[test]
    fn note_does_not_affect_standing() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "claim-1".into(),
                        statement: "Notes do not affect this claim.".into(),
                        status: Status::Open,
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::Note {
                        text: "A note is not claim standing.".into(),
                    },
                )
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 3);

        let claim = view.get("claim-1").unwrap();
        assert_eq!(view.len(), 1);
        assert_eq!(claim.standing, Status::Open);
        assert!(claim.legacy_evidence.is_empty());
        assert_eq!(claim.legacy_transitions, 0);
    }

    #[test]
    fn standing_replay_is_deterministic() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "claim-1".into(),
                        statement: "Claim one is replayable.".into(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::EvidenceRecorded {
                        claim_id: "claim-1".into(),
                        summary: "Legacy evidence summary.".into(),
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::ClaimStatusChanged {
                        claim_id: "claim-1".into(),
                        from: Status::Conjectured,
                        to: Status::Supported,
                        reason: "Legacy manual update.".into(),
                    },
                )
                .unwrap();
            writer.append(provenance(), anchor()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view_a = StandingView::new();
        let mut view_b = StandingView::new();

        assert_eq!(reader.replay(&mut view_a).unwrap(), 5);
        assert_eq!(reader.replay(&mut view_b).unwrap(), 5);

        assert_eq!(view_a, view_b);
        assert_eq!(view_a.canonical_bytes(), view_b.canonical_bytes());

        let claim = view_a.get("claim-1").unwrap();
        assert_eq!(claim.statement, "Claim one is replayable.");
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);
        assert_eq!(claim.standing, Status::Supported);
        assert_eq!(
            claim.legacy_evidence,
            vec!["Legacy evidence summary.".to_string()]
        );
        assert_eq!(claim.legacy_transitions, 1);
    }

    #[test]
    fn legacy_v0_chains_still_replay() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: "legacy-claim".into(),
                        statement: "Legacy v0 claim.".into(),
                        status: Status::Open,
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::EvidenceRecorded {
                        claim_id: "legacy-claim".into(),
                        summary: "Legacy untyped evidence.".into(),
                    },
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::ClaimStatusChanged {
                        claim_id: "legacy-claim".into(),
                        from: Status::Open,
                        to: Status::Refuted,
                        reason: "Legacy manual correction.".into(),
                    },
                )
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 4);

        let claim = view.get("legacy-claim").unwrap();
        assert_eq!(claim.statement, "Legacy v0 claim.");
        assert_eq!(claim.asserted_initial_status, Status::Open);
        assert_eq!(claim.standing, Status::Refuted);
        assert_eq!(
            claim.legacy_evidence,
            vec!["Legacy untyped evidence.".to_string()]
        );
        assert_eq!(claim.legacy_transitions, 1);
    }

    #[test]
    fn segment_anchor_is_occurrence_evidence_not_interpretation_truth() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer.append(provenance(), anchor()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        assert!(view.is_empty());
        assert_eq!(view.len(), 0);
        assert!(view.get("claim-created-from-anchor").is_none());
    }
}
