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

/// Typed claim metadata retained from ADR-0002 `ClaimAssertedV2` events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypedClaimNode {
    pub statement: String,
    pub scope_ref: String,
    pub actor_class: String,
    pub content_hash: String,
    pub metadata_json: String,
}

/// Typed evidence retained from ADR-0002 `EvidenceRegistered` events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypedEvidenceNode {
    pub evidence_kind: String,
    pub summary: String,
    pub scope_ref: String,
    pub actor_class: String,
    pub content_hash: String,
    pub metadata_json: String,
}

/// Typed justification edge retained from ADR-0002 `JustificationEdgeRecorded` events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TypedJustificationEdge {
    pub edge_kind: String,
    pub source_id: String,
    pub target_id: String,
    pub scope_ref: String,
    pub actor_class: String,
    pub rationale: String,
    pub metadata_json: String,
}

/// ADR-0002 standing, derived from the verified append-only log.
///
/// This projection folds legacy v0 claim events and the additive ADR-0002 typed
/// assertion vocabulary. It intentionally does not add writer-facing surfaces or
/// authority gates.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct StandingView {
    pub claims: BTreeMap<String, StandingClaim>,
    pub typed_claims: BTreeMap<String, TypedClaimNode>,
    pub typed_evidence: BTreeMap<String, TypedEvidenceNode>,
    pub justification_edges: BTreeMap<String, TypedJustificationEdge>,
}

impl StandingView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, claim_id: &str) -> Option<&StandingClaim> {
        self.claims.get(claim_id)
    }

    pub fn typed_claim(&self, claim_id: &str) -> Option<&TypedClaimNode> {
        self.typed_claims.get(claim_id)
    }

    pub fn typed_evidence(&self, evidence_id: &str) -> Option<&TypedEvidenceNode> {
        self.typed_evidence.get(evidence_id)
    }

    pub fn justification_edge(&self, edge_id: &str) -> Option<&TypedJustificationEdge> {
        self.justification_edges.get(edge_id)
    }

    /// Number of current standing claims. This preserves the original
    /// `StandingView::len` meaning after typed side tables were added.
    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn claim_len(&self) -> usize {
        self.claims.len()
    }

    pub fn typed_claim_len(&self) -> usize {
        self.typed_claims.len()
    }

    pub fn typed_evidence_len(&self) -> usize {
        self.typed_evidence.len()
    }

    pub fn justification_edge_len(&self) -> usize {
        self.justification_edges.len()
    }

    /// Claim-oriented emptiness. Typed evidence or edges may exist without a
    /// current standing claim; use [`Self::is_structurally_empty`] to check all
    /// replay tables.
    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    pub fn is_structurally_empty(&self) -> bool {
        self.claims.is_empty()
            && self.typed_claims.is_empty()
            && self.typed_evidence.is_empty()
            && self.justification_edges.is_empty()
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
            Payload::ClaimAssertedV2 {
                claim_id,
                statement,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json,
            } => {
                // ADR-0002 v1 typed assertions start at Conjectured. They register a
                // scoped claim node but do not settle truth or bypass future ceilings.
                self.claims
                    .entry(claim_id.clone())
                    .or_insert_with(|| StandingClaim {
                        statement: statement.clone(),
                        asserted_initial_status: Status::Conjectured,
                        standing: Status::Conjectured,
                        legacy_evidence: Vec::new(),
                        legacy_transitions: 0,
                    });
                self.typed_claims
                    .entry(claim_id.clone())
                    .or_insert_with(|| TypedClaimNode {
                        statement: statement.clone(),
                        scope_ref: scope_ref.clone(),
                        actor_class: actor_class.clone(),
                        content_hash: content_hash.clone(),
                        metadata_json: metadata_json.clone(),
                    });
            }
            Payload::EvidenceRegistered {
                evidence_id,
                evidence_kind,
                summary,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json,
            } => {
                // Typed evidence alone does not create or settle claims. Later fold
                // rules may evaluate this table through explicit justification edges.
                self.typed_evidence
                    .entry(evidence_id.clone())
                    .or_insert_with(|| TypedEvidenceNode {
                        evidence_kind: evidence_kind.clone(),
                        summary: summary.clone(),
                        scope_ref: scope_ref.clone(),
                        actor_class: actor_class.clone(),
                        content_hash: content_hash.clone(),
                        metadata_json: metadata_json.clone(),
                    });
            }
            Payload::JustificationEdgeRecorded {
                edge_id,
                edge_kind,
                source_id,
                target_id,
                scope_ref,
                actor_class,
                rationale,
                metadata_json,
            } => {
                // Edges are inert until the staged ADR-0002 fold implements support,
                // debt, supersession, invalidation, and ratification semantics.
                self.justification_edges
                    .entry(edge_id.clone())
                    .or_insert_with(|| TypedJustificationEdge {
                        edge_kind: edge_kind.clone(),
                        source_id: source_id.clone(),
                        target_id: target_id.clone(),
                        scope_ref: scope_ref.clone(),
                        actor_class: actor_class.clone(),
                        rationale: rationale.clone(),
                        metadata_json: metadata_json.clone(),
                    });
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

    fn claim_asserted_v2(actor_class: &str) -> Payload {
        claim_asserted_v2_with(
            "claim-v2",
            "Typed claims begin as governed conjectures.",
            "magpie:test",
            actor_class,
            "",
            "{}",
        )
    }

    fn claim_asserted_v2_with(
        claim_id: &str,
        statement: &str,
        scope_ref: &str,
        actor_class: &str,
        content_hash: &str,
        metadata_json: &str,
    ) -> Payload {
        Payload::ClaimAssertedV2 {
            claim_id: claim_id.into(),
            statement: statement.into(),
            scope_ref: scope_ref.into(),
            actor_class: actor_class.into(),
            content_hash: content_hash.into(),
            metadata_json: metadata_json.into(),
        }
    }

    fn evidence_registered() -> Payload {
        evidence_registered_with(
            "evidence-v2",
            "DeadboltAnchor",
            "Typed evidence alone does not create standing.",
            "magpie:test",
            "DeadboltAnchorer",
            "",
            "{}",
        )
    }

    fn evidence_registered_with(
        evidence_id: &str,
        evidence_kind: &str,
        summary: &str,
        scope_ref: &str,
        actor_class: &str,
        content_hash: &str,
        metadata_json: &str,
    ) -> Payload {
        Payload::EvidenceRegistered {
            evidence_id: evidence_id.into(),
            evidence_kind: evidence_kind.into(),
            summary: summary.into(),
            scope_ref: scope_ref.into(),
            actor_class: actor_class.into(),
            content_hash: content_hash.into(),
            metadata_json: metadata_json.into(),
        }
    }

    fn justification_edge_recorded() -> Payload {
        justification_edge_recorded_with(
            "edge-v2",
            "supports",
            "evidence-v2",
            "claim-v2",
            "magpie:test",
            "HumanRoot",
            "Typed edges are inert until staged StandingView rules land.",
            "{}",
        )
    }

    fn justification_edge_recorded_with(
        edge_id: &str,
        edge_kind: &str,
        source_id: &str,
        target_id: &str,
        scope_ref: &str,
        actor_class: &str,
        rationale: &str,
        metadata_json: &str,
    ) -> Payload {
        Payload::JustificationEdgeRecorded {
            edge_id: edge_id.into(),
            edge_kind: edge_kind.into(),
            source_id: source_id.into(),
            target_id: target_id.into(),
            scope_ref: scope_ref.into(),
            actor_class: actor_class.into(),
            rationale: rationale.into(),
            metadata_json: metadata_json.into(),
        }
    }

    fn replay(store: MemStore, expected_count: u64) -> StandingView {
        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), expected_count);
        view
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
        assert!(view.is_structurally_empty());
        assert!(view.get("claim-created-from-anchor").is_none());
    }

    #[test]
    fn claim_asserted_v2_creates_conjectured_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), claim_asserted_v2("HumanRoot"))
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        let claim = view.get("claim-v2").unwrap();
        assert_eq!(view.len(), 1);
        assert_eq!(view.claim_len(), 1);
        assert_eq!(view.typed_claim_len(), 1);
        assert_eq!(
            claim.statement,
            "Typed claims begin as governed conjectures."
        );
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);
        assert_eq!(claim.standing, Status::Conjectured);
        assert!(claim.legacy_evidence.is_empty());
        assert_eq!(claim.legacy_transitions, 0);
    }

    #[test]
    fn agent_proposer_claim_asserted_v2_does_not_settle() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), claim_asserted_v2("AgentProposer"))
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        let claim = view.get("claim-v2").unwrap();
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);
        assert_eq!(claim.standing, Status::Conjectured);
    }

    #[test]
    fn evidence_registered_alone_does_not_create_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer.append(provenance(), evidence_registered()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        assert!(view.is_empty());
        assert!(view.get("claim-v2").is_none());
        assert_eq!(view.typed_claim_len(), 0);
        assert_eq!(view.typed_evidence_len(), 1);
        assert_eq!(view.justification_edge_len(), 0);
    }

    #[test]
    fn justification_edge_recorded_alone_does_not_create_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), justification_edge_recorded())
                .unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 2);

        assert!(view.is_empty());
        assert!(view.get("claim-v2").is_none());
        assert_eq!(view.typed_claim_len(), 0);
        assert_eq!(view.typed_evidence_len(), 0);
        assert_eq!(view.justification_edge_len(), 1);
    }

    #[test]
    fn segment_anchor_still_does_not_create_or_promote_claim_after_tags_6_8() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), claim_asserted_v2("HumanRoot"))
                .unwrap();
            writer.append(provenance(), evidence_registered()).unwrap();
            writer
                .append(provenance(), justification_edge_recorded())
                .unwrap();
            writer.append(provenance(), anchor()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view = StandingView::new();
        assert_eq!(reader.replay(&mut view).unwrap(), 5);

        let claim = view.get("claim-v2").unwrap();
        assert_eq!(view.len(), 1);
        assert_eq!(view.typed_claim_len(), 1);
        assert_eq!(view.typed_evidence_len(), 1);
        assert_eq!(view.justification_edge_len(), 1);
        assert_eq!(claim.standing, Status::Conjectured);
        assert!(claim.legacy_evidence.is_empty());
        assert_eq!(claim.legacy_transitions, 0);
    }

    #[test]
    fn claim_asserted_v2_retains_typed_claim_metadata() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    claim_asserted_v2_with(
                        "claim-v2",
                        "Typed claim metadata is replayed.",
                        "scope:typed",
                        "HumanRoot",
                        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                        "{\"claim\":\"metadata\"}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 2);

        let claim = view.get("claim-v2").unwrap();
        assert_eq!(claim.standing, Status::Conjectured);
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);

        let typed = view.typed_claim("claim-v2").unwrap();
        assert_eq!(typed.statement, "Typed claim metadata is replayed.");
        assert_eq!(typed.scope_ref, "scope:typed");
        assert_eq!(typed.actor_class, "HumanRoot");
        assert_eq!(
            typed.content_hash,
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
        assert_eq!(typed.metadata_json, "{\"claim\":\"metadata\"}");
        assert_eq!(view.typed_claim_len(), 1);
        assert_eq!(view.typed_evidence_len(), 0);
        assert_eq!(view.justification_edge_len(), 0);
    }

    #[test]
    fn duplicate_claim_asserted_v2_does_not_overwrite_typed_claim() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    claim_asserted_v2_with(
                        "claim-v2",
                        "first",
                        "scope:first",
                        "HumanRoot",
                        "",
                        "{\"n\":1}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    claim_asserted_v2_with(
                        "claim-v2",
                        "second",
                        "scope:second",
                        "AgentProposer",
                        "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
                        "{\"n\":2}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 3);
        let claim = view.get("claim-v2").unwrap();
        let typed = view.typed_claim("claim-v2").unwrap();

        assert_eq!(claim.statement, "first");
        assert_eq!(claim.standing, Status::Conjectured);
        assert_eq!(typed.statement, "first");
        assert_eq!(typed.scope_ref, "scope:first");
        assert_eq!(typed.actor_class, "HumanRoot");
        assert_eq!(typed.content_hash, "");
        assert_eq!(typed.metadata_json, "{\"n\":1}");
        assert_eq!(view.typed_claim_len(), 1);
    }

    #[test]
    fn evidence_registered_is_retained_without_claim_promotion() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    evidence_registered_with(
                        "evidence-v2",
                        "ExecutionEvidence",
                        "Typed evidence metadata is replayed.",
                        "scope:evidence",
                        "AutomatedVerifier",
                        "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
                        "{\"evidence\":\"metadata\"}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 2);

        assert!(view.is_empty());
        assert!(!view.is_structurally_empty());
        assert!(view.get("claim-v2").is_none());
        assert_eq!(view.typed_claim_len(), 0);
        assert_eq!(view.typed_evidence_len(), 1);
        assert_eq!(view.justification_edge_len(), 0);

        let evidence = view.typed_evidence("evidence-v2").unwrap();
        assert_eq!(evidence.evidence_kind, "ExecutionEvidence");
        assert_eq!(evidence.summary, "Typed evidence metadata is replayed.");
        assert_eq!(evidence.scope_ref, "scope:evidence");
        assert_eq!(evidence.actor_class, "AutomatedVerifier");
        assert_eq!(
            evidence.content_hash,
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        );
        assert_eq!(evidence.metadata_json, "{\"evidence\":\"metadata\"}");
    }

    #[test]
    fn duplicate_evidence_registered_does_not_overwrite() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    evidence_registered_with(
                        "evidence-v2",
                        "ExternalSource",
                        "first",
                        "scope:first",
                        "SourceImporter",
                        "",
                        "{\"n\":1}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    evidence_registered_with(
                        "evidence-v2",
                        "ModelSelfReport",
                        "second",
                        "scope:second",
                        "AgentProposer",
                        "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                        "{\"n\":2}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 3);
        let evidence = view.typed_evidence("evidence-v2").unwrap();

        assert_eq!(evidence.evidence_kind, "ExternalSource");
        assert_eq!(evidence.summary, "first");
        assert_eq!(evidence.scope_ref, "scope:first");
        assert_eq!(evidence.actor_class, "SourceImporter");
        assert_eq!(evidence.content_hash, "");
        assert_eq!(evidence.metadata_json, "{\"n\":1}");
        assert_eq!(view.typed_evidence_len(), 1);
    }

    #[test]
    fn justification_edge_recorded_is_retained_without_promotion() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(
                        "edge-v2",
                        "supports",
                        "evidence-v2",
                        "claim-v2",
                        "scope:edge",
                        "HumanRoot",
                        "Typed edge metadata is replayed.",
                        "{\"edge\":\"metadata\"}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 2);

        assert!(view.is_empty());
        assert!(!view.is_structurally_empty());
        assert!(view.get("claim-v2").is_none());
        assert_eq!(view.typed_claim_len(), 0);
        assert_eq!(view.typed_evidence_len(), 0);
        assert_eq!(view.justification_edge_len(), 1);

        let edge = view.justification_edge("edge-v2").unwrap();
        assert_eq!(edge.edge_kind, "supports");
        assert_eq!(edge.source_id, "evidence-v2");
        assert_eq!(edge.target_id, "claim-v2");
        assert_eq!(edge.scope_ref, "scope:edge");
        assert_eq!(edge.actor_class, "HumanRoot");
        assert_eq!(edge.rationale, "Typed edge metadata is replayed.");
        assert_eq!(edge.metadata_json, "{\"edge\":\"metadata\"}");
    }

    #[test]
    fn duplicate_justification_edge_recorded_does_not_overwrite() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(
                        "edge-v2",
                        "supports",
                        "evidence-v2",
                        "claim-v2",
                        "scope:first",
                        "HumanRoot",
                        "first",
                        "{\"n\":1}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(
                        "edge-v2",
                        "contradicts",
                        "other-evidence",
                        "claim-v2",
                        "scope:second",
                        "AgentProposer",
                        "second",
                        "{\"n\":2}",
                    ),
                )
                .unwrap();
        }

        let view = replay(store, 3);
        let edge = view.justification_edge("edge-v2").unwrap();

        assert_eq!(edge.edge_kind, "supports");
        assert_eq!(edge.source_id, "evidence-v2");
        assert_eq!(edge.target_id, "claim-v2");
        assert_eq!(edge.scope_ref, "scope:first");
        assert_eq!(edge.actor_class, "HumanRoot");
        assert_eq!(edge.rationale, "first");
        assert_eq!(edge.metadata_json, "{\"n\":1}");
        assert_eq!(view.justification_edge_len(), 1);
    }

    #[test]
    fn typed_tables_replay_deterministically() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(
                    provenance(),
                    claim_asserted_v2_with("claim-a", "Claim A.", "scope:a", "HumanRoot", "", "{}"),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    claim_asserted_v2_with(
                        "claim-b",
                        "Claim B.",
                        "scope:b",
                        "AgentProposer",
                        "",
                        "{}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    evidence_registered_with(
                        "evidence-a",
                        "ExecutionEvidence",
                        "Evidence A.",
                        "scope:a",
                        "AutomatedVerifier",
                        "",
                        "{}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    evidence_registered_with(
                        "evidence-b",
                        "LensReadout",
                        "Evidence B.",
                        "scope:b",
                        "LensWitness",
                        "",
                        "{}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(
                        "edge-a",
                        "supports",
                        "evidence-a",
                        "claim-a",
                        "scope:a",
                        "HumanRoot",
                        "Edge A.",
                        "{}",
                    ),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(
                        "edge-b",
                        "derived_from",
                        "claim-a",
                        "claim-b",
                        "scope:b",
                        "AgentProposer",
                        "Edge B.",
                        "{}",
                    ),
                )
                .unwrap();
            writer.append(provenance(), anchor()).unwrap();
        }

        let reader = LogReader::open(store, test_key().verifying_key());
        let mut view_a = StandingView::new();
        let mut view_b = StandingView::new();

        assert_eq!(reader.replay(&mut view_a).unwrap(), 8);
        assert_eq!(reader.replay(&mut view_b).unwrap(), 8);

        assert_eq!(view_a, view_b);
        assert_eq!(view_a.canonical_bytes(), view_b.canonical_bytes());
        assert_eq!(view_a.claim_len(), 2);
        assert_eq!(view_a.typed_claim_len(), 2);
        assert_eq!(view_a.typed_evidence_len(), 2);
        assert_eq!(view_a.justification_edge_len(), 2);
        assert!(view_a.get("claim-created-from-anchor").is_none());
        assert!(view_a.typed_evidence("run-standing-view").is_none());
    }

    #[test]
    fn typed_evidence_and_edges_do_not_change_claim_standing_yet() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), claim_asserted_v2("HumanRoot"))
                .unwrap();
            writer.append(provenance(), evidence_registered()).unwrap();
            writer
                .append(provenance(), justification_edge_recorded())
                .unwrap();
        }

        let view = replay(store, 4);
        let claim = view.get("claim-v2").unwrap();

        assert_eq!(claim.standing, Status::Conjectured);
        assert!(claim.legacy_evidence.is_empty());
        assert_eq!(claim.legacy_transitions, 0);
        assert!(view.typed_evidence("evidence-v2").is_some());
        assert!(view.justification_edge("edge-v2").is_some());
    }

    #[test]
    fn segment_anchor_still_does_not_create_typed_evidence() {
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer.append(provenance(), anchor()).unwrap();
        }

        let view = replay(store, 2);

        assert!(view.is_empty());
        assert!(view.is_structurally_empty());
        assert_eq!(view.typed_claim_len(), 0);
        assert_eq!(view.typed_evidence_len(), 0);
        assert_eq!(view.justification_edge_len(), 0);
    }
}
