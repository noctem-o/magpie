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

    /// ADR-0002 governed belief surface — the first fold slice, settlement-blind.
    ///
    /// Derives a claim's standing by layering admissible `supports` edges from
    /// typed evidence onto the raw [`StandingClaim::standing`], under the
    /// conservative support-slice ceilings. It is a *pure query over the
    /// accumulated tables*: it never mutates, never touches the raw `standing`
    /// field, and is therefore order-independent and regenerable. Raw standing
    /// stays the legacy surface; this is the governed one.
    ///
    /// A `supports` edge contributes only when the target typed claim and the
    /// source typed evidence both exist and the evidence, edge, and target typed
    /// claim share one exact `scope_ref`.
    ///
    /// Deliberately *not* inferred from absence in this slice, all deferred:
    /// claim→claim support, ratification, contradiction debt, invalidation,
    /// supersession, settlement, and any `claim_domain` / `metadata_json`
    /// reading. No support edge derives [`Status::Settled`] here — a claim is
    /// `Settled` only if the legacy path already made its raw standing `Settled`.
    pub fn resolved_standing(&self, claim_id: &str) -> Option<Status> {
        let raw = self.claims.get(claim_id)?.standing;

        // `Refuted` is off the positive ladder — support cannot revive it.
        // `Settled` (reachable here only via the legacy path) is already at the
        // top and support never derives it, so both pass through untouched.
        if matches!(raw, Status::Refuted | Status::Settled) {
            return Some(raw);
        }

        let mut best = positive_rank(raw);

        // Support requires a typed target; a v0-only claim cannot be promoted by
        // ADR-0002 support edges.
        if let Some(target) = self.typed_claims.get(claim_id) {
            for edge in self.justification_edges.values() {
                if edge.edge_kind != "supports" || edge.target_id != claim_id {
                    continue;
                }
                // Missing source evidence: no contribution.
                let Some(evidence) = self.typed_evidence.get(&edge.source_id) else {
                    continue;
                };
                // Exact triple scope match: evidence, edge, and target typed claim.
                if evidence.scope_ref != edge.scope_ref || edge.scope_ref != target.scope_ref {
                    continue;
                }
                if let Some(ceiling) = support_slice_ceiling(&evidence.evidence_kind) {
                    best = best.max(positive_rank(ceiling));
                }
            }
        }

        Some(status_from_positive_rank(best))
    }
}

/// Rank on the positive standing ladder (`Open < Conjectured < Supported <
/// Settled`). `Refuted` is off-ladder and must be handled before ranking.
fn positive_rank(status: Status) -> u8 {
    match status {
        Status::Open => 0,
        Status::Conjectured => 1,
        Status::Supported => 2,
        Status::Settled => 3,
        // Off-ladder; `resolved_standing` handles `Refuted` before it ranks.
        Status::Refuted => 0,
    }
}

fn status_from_positive_rank(rank: u8) -> Status {
    match rank {
        0 => Status::Open,
        1 => Status::Conjectured,
        2 => Status::Supported,
        _ => Status::Settled,
    }
}

/// Conservative, settlement-blind support ceiling: the most a single `supports`
/// edge from typed evidence of this kind may derive in the first fold slice.
/// `None` means no admissible contribution (an unrecognized kind). No kind
/// derives [`Status::Settled`] here — settlement is claim_domain-aware and
/// deferred.
fn support_slice_ceiling(evidence_kind: &str) -> Option<Status> {
    match evidence_kind {
        "ModelSelfReport" | "LensReadout" => Some(Status::Conjectured),
        "DeterministicVerification"
        | "HumanRatification"
        | "DeadboltAnchor"
        | "ExecutionEvidence"
        | "BehavioralEvaluation"
        | "ExternalSource" => Some(Status::Supported),
        _ => None,
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
        justification_edge_recorded_with(EdgeInput {
            edge_id: "edge-v2",
            edge_kind: "supports",
            source_id: "evidence-v2",
            target_id: "claim-v2",
            scope_ref: "magpie:test",
            actor_class: "HumanRoot",
            rationale: "Typed edges are inert until staged StandingView rules land.",
            metadata_json: "{}",
        })
    }

    struct EdgeInput<'a> {
        edge_id: &'a str,
        edge_kind: &'a str,
        source_id: &'a str,
        target_id: &'a str,
        scope_ref: &'a str,
        actor_class: &'a str,
        rationale: &'a str,
        metadata_json: &'a str,
    }

    fn justification_edge_recorded_with(input: EdgeInput<'_>) -> Payload {
        Payload::JustificationEdgeRecorded {
            edge_id: input.edge_id.into(),
            edge_kind: input.edge_kind.into(),
            source_id: input.source_id.into(),
            target_id: input.target_id.into(),
            scope_ref: input.scope_ref.into(),
            actor_class: input.actor_class.into(),
            rationale: input.rationale.into(),
            metadata_json: input.metadata_json.into(),
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
                    justification_edge_recorded_with(EdgeInput {
                        edge_id: "edge-v2",
                        edge_kind: "supports",
                        source_id: "evidence-v2",
                        target_id: "claim-v2",
                        scope_ref: "scope:edge",
                        actor_class: "HumanRoot",
                        rationale: "Typed edge metadata is replayed.",
                        metadata_json: "{\"edge\":\"metadata\"}",
                    }),
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
                    justification_edge_recorded_with(EdgeInput {
                        edge_id: "edge-v2",
                        edge_kind: "supports",
                        source_id: "evidence-v2",
                        target_id: "claim-v2",
                        scope_ref: "scope:first",
                        actor_class: "HumanRoot",
                        rationale: "first",
                        metadata_json: "{\"n\":1}",
                    }),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(EdgeInput {
                        edge_id: "edge-v2",
                        edge_kind: "contradicts",
                        source_id: "other-evidence",
                        target_id: "claim-v2",
                        scope_ref: "scope:second",
                        actor_class: "AgentProposer",
                        rationale: "second",
                        metadata_json: "{\"n\":2}",
                    }),
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
                    justification_edge_recorded_with(EdgeInput {
                        edge_id: "edge-a",
                        edge_kind: "supports",
                        source_id: "evidence-a",
                        target_id: "claim-a",
                        scope_ref: "scope:a",
                        actor_class: "HumanRoot",
                        rationale: "Edge A.",
                        metadata_json: "{}",
                    }),
                )
                .unwrap();
            writer
                .append(
                    provenance(),
                    justification_edge_recorded_with(EdgeInput {
                        edge_id: "edge-b",
                        edge_kind: "derived_from",
                        source_id: "claim-a",
                        target_id: "claim-b",
                        scope_ref: "scope:b",
                        actor_class: "AgentProposer",
                        rationale: "Edge B.",
                        metadata_json: "{}",
                    }),
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

    #[test]
    fn legacy_status_change_still_settles_typed_v2_claim_pending_gate() {
        // CHARACTERIZATION of a known compatibility gap, not an endorsement.
        //
        // A legacy tag-3 `ClaimStatusChanged` still drives a typed tag-6
        // `ClaimAssertedV2` claim to `Settled`, bypassing the ADR-0002 evidence
        // ceilings entirely: `ClaimAssertedV2` alone only reaches `Conjectured`,
        // and an `AgentProposer` may not settle — yet the legacy manual path
        // settles it because no `EpistemicGate` exists to refuse the write.
        //
        // See docs/design/standing-view-evidence-ceilings.md ("Compatibility with
        // v0 events" / `ClaimStatusChanged`) and ADR-0002. When the gate lands it
        // must consciously close this path, and this test must change with it —
        // it is a tripwire, not a promise.
        let store = MemStore::new();
        {
            let mut writer = writer(store.clone());
            writer
                .append(provenance(), claim_asserted_v2("AgentProposer"))
                .unwrap();
            writer
                .append(
                    provenance(),
                    Payload::ClaimStatusChanged {
                        claim_id: "claim-v2".into(),
                        from: Status::Conjectured,
                        to: Status::Settled,
                        reason: "Legacy manual settlement bypasses ceilings.".into(),
                    },
                )
                .unwrap();
        }

        let view = replay(store, 3);
        let claim = view.get("claim-v2").unwrap();

        // The gap: a typed claim asserted by an ordinary agent is Settled through
        // the legacy path, despite the ceiling law forbidding exactly this.
        assert_eq!(claim.asserted_initial_status, Status::Conjectured);
        assert_eq!(claim.standing, Status::Settled);
        assert_eq!(claim.legacy_transitions, 1);

        // The typed side table is untouched by the legacy status change.
        let typed = view.typed_claim("claim-v2").unwrap();
        assert_eq!(typed.actor_class, "AgentProposer");
    }

    // ---- resolved_standing: first fold slice (evidence -> claim supports) ----

    const SCOPE: &str = "scope:one";
    const OTHER_SCOPE: &str = "scope:other";

    /// A typed claim `c1`, one typed evidence `e1` of the given kind, and one
    /// `supports` edge `e1 -> c1`, each with its own scope so scope-match rules
    /// can be exercised.
    fn support_view(
        evidence_kind: &str,
        ev_scope: &str,
        edge_scope: &str,
        claim_scope: &str,
    ) -> StandingView {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                claim_asserted_v2_with(
                    "c1",
                    "Target typed claim.",
                    claim_scope,
                    "AgentProposer",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    evidence_kind,
                    "Supporting evidence.",
                    ev_scope,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1",
                    target_id: "c1",
                    scope_ref: edge_scope,
                    actor_class: "HumanRoot",
                    rationale: "Evidence supports the claim.",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        replay(store, 4)
    }

    #[test]
    fn supported_tier_evidence_promotes_conjectured_to_supported() {
        for kind in [
            "DeterministicVerification",
            "HumanRatification",
            "DeadboltAnchor",
            "ExecutionEvidence",
            "BehavioralEvaluation",
            "ExternalSource",
        ] {
            let view = support_view(kind, SCOPE, SCOPE, SCOPE);
            assert_eq!(
                view.resolved_standing("c1"),
                Some(Status::Supported),
                "{kind} should derive Supported"
            );
            // Settlement is deferred: no support path may reach Settled.
            assert_ne!(view.resolved_standing("c1"), Some(Status::Settled));
            // Raw standing is never mutated by resolution.
            assert_eq!(view.get("c1").unwrap().standing, Status::Conjectured);
        }
    }

    #[test]
    fn weak_evidence_is_admitted_but_capped_at_conjectured() {
        for kind in ["ModelSelfReport", "LensReadout"] {
            let view = support_view(kind, SCOPE, SCOPE, SCOPE);
            assert_eq!(
                view.resolved_standing("c1"),
                Some(Status::Conjectured),
                "{kind} must stay Conjectured"
            );
            assert_eq!(view.get("c1").unwrap().standing, Status::Conjectured);
        }
    }

    #[test]
    fn multiple_supports_never_derive_settled() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                claim_asserted_v2_with("c1", "Target.", SCOPE, "AgentProposer", "", "{}"),
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    "DeterministicVerification",
                    "One.",
                    SCOPE,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e2",
                    "ExecutionEvidence",
                    "Two.",
                    SCOPE,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            for (edge_id, source_id) in [("j1", "e1"), ("j2", "e2")] {
                w.append(
                    provenance(),
                    justification_edge_recorded_with(EdgeInput {
                        edge_id,
                        edge_kind: "supports",
                        source_id,
                        target_id: "c1",
                        scope_ref: SCOPE,
                        actor_class: "HumanRoot",
                        rationale: "supports",
                        metadata_json: "{}",
                    }),
                )
                .unwrap();
            }
        }
        let view = replay(store, 6);
        assert_eq!(view.resolved_standing("c1"), Some(Status::Supported));
    }

    #[test]
    fn scope_mismatch_blocks_support() {
        // A mismatch in any leg of the evidence/edge/claim triple refuses support.
        assert_eq!(
            support_view("ExecutionEvidence", OTHER_SCOPE, SCOPE, SCOPE).resolved_standing("c1"),
            Some(Status::Conjectured),
            "evidence scope mismatch"
        );
        assert_eq!(
            support_view("ExecutionEvidence", SCOPE, OTHER_SCOPE, SCOPE).resolved_standing("c1"),
            Some(Status::Conjectured),
            "edge scope mismatch"
        );
        assert_eq!(
            support_view("ExecutionEvidence", SCOPE, SCOPE, OTHER_SCOPE).resolved_standing("c1"),
            Some(Status::Conjectured),
            "claim scope mismatch"
        );
    }

    #[test]
    fn missing_source_evidence_blocks_support() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                claim_asserted_v2_with("c1", "Target.", SCOPE, "AgentProposer", "", "{}"),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1", // never registered
                    target_id: "c1",
                    scope_ref: SCOPE,
                    actor_class: "HumanRoot",
                    rationale: "dangling support",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        let view = replay(store, 3);
        assert_eq!(view.resolved_standing("c1"), Some(Status::Conjectured));
    }

    #[test]
    fn missing_target_claim_yields_none() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    "ExecutionEvidence",
                    "Evidence.",
                    SCOPE,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1",
                    target_id: "c1", // never asserted
                    scope_ref: SCOPE,
                    actor_class: "HumanRoot",
                    rationale: "support for a missing claim",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        let view = replay(store, 3);
        assert_eq!(view.resolved_standing("c1"), None);
    }

    #[test]
    fn v0_claim_is_not_promoted_by_support() {
        // A legacy v0 ClaimAsserted has no typed-claim node, so ADR-0002 support
        // cannot reach it.
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                Payload::ClaimAsserted {
                    claim_id: "c0".into(),
                    statement: "Legacy v0 claim.".into(),
                    status: Status::Conjectured,
                },
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    "ExecutionEvidence",
                    "Evidence.",
                    SCOPE,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1",
                    target_id: "c0",
                    scope_ref: SCOPE,
                    actor_class: "HumanRoot",
                    rationale: "supports",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        let view = replay(store, 4);
        assert_eq!(view.resolved_standing("c0"), Some(Status::Conjectured));
    }

    #[test]
    fn typed_claim_without_support_resolves_to_raw() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(provenance(), claim_asserted_v2("AgentProposer"))
                .unwrap();
        }
        let view = replay(store, 2);
        assert_eq!(
            view.resolved_standing("claim-v2"),
            Some(Status::Conjectured)
        );
    }

    #[test]
    fn resolved_standing_is_order_independent() {
        // claim, evidence, edge in two different append orders.
        let claim = || claim_asserted_v2_with("c1", "Target.", SCOPE, "AgentProposer", "", "{}");
        let evidence = || {
            evidence_registered_with(
                "e1",
                "ExecutionEvidence",
                "Evidence.",
                SCOPE,
                "AutomatedVerifier",
                "",
                "{}",
            )
        };
        let edge = || {
            justification_edge_recorded_with(EdgeInput {
                edge_id: "j1",
                edge_kind: "supports",
                source_id: "e1",
                target_id: "c1",
                scope_ref: SCOPE,
                actor_class: "HumanRoot",
                rationale: "supports",
                metadata_json: "{}",
            })
        };

        let build = |payloads: [Payload; 3]| {
            let store = MemStore::new();
            {
                let mut w = writer(store.clone());
                for p in payloads {
                    w.append(provenance(), p).unwrap();
                }
            }
            replay(store, 4)
        };

        let forward = build([claim(), evidence(), edge()]);
        let reversed = build([edge(), evidence(), claim()]);

        assert_eq!(forward.resolved_standing("c1"), Some(Status::Supported));
        assert_eq!(
            forward.resolved_standing("c1"),
            reversed.resolved_standing("c1")
        );
    }

    #[test]
    fn refuted_raw_claim_is_not_promoted_by_support() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                claim_asserted_v2_with("c1", "Target.", SCOPE, "AgentProposer", "", "{}"),
            )
            .unwrap();
            w.append(
                provenance(),
                Payload::ClaimStatusChanged {
                    claim_id: "c1".into(),
                    from: Status::Conjectured,
                    to: Status::Refuted,
                    reason: "Refuted by later work.".into(),
                },
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    "ExecutionEvidence",
                    "Evidence.",
                    SCOPE,
                    "AutomatedVerifier",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1",
                    target_id: "c1",
                    scope_ref: SCOPE,
                    actor_class: "HumanRoot",
                    rationale: "supports",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        let view = replay(store, 5);
        assert_eq!(view.get("c1").unwrap().standing, Status::Refuted);
        assert_eq!(view.resolved_standing("c1"), Some(Status::Refuted));
    }

    #[test]
    fn settled_raw_claim_is_preserved_under_support() {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                provenance(),
                claim_asserted_v2_with("c1", "Target.", SCOPE, "AgentProposer", "", "{}"),
            )
            .unwrap();
            w.append(
                provenance(),
                Payload::ClaimStatusChanged {
                    claim_id: "c1".into(),
                    from: Status::Conjectured,
                    to: Status::Settled,
                    reason: "Settled via the legacy path.".into(),
                },
            )
            .unwrap();
            w.append(
                provenance(),
                evidence_registered_with(
                    "e1",
                    "ModelSelfReport",
                    "Weak evidence.",
                    SCOPE,
                    "AgentProposer",
                    "",
                    "{}",
                ),
            )
            .unwrap();
            w.append(
                provenance(),
                justification_edge_recorded_with(EdgeInput {
                    edge_id: "j1",
                    edge_kind: "supports",
                    source_id: "e1",
                    target_id: "c1",
                    scope_ref: SCOPE,
                    actor_class: "HumanRoot",
                    rationale: "supports",
                    metadata_json: "{}",
                }),
            )
            .unwrap();
        }
        let view = replay(store, 5);
        assert_eq!(view.get("c1").unwrap().standing, Status::Settled);
        assert_eq!(view.resolved_standing("c1"), Some(Status::Settled));
    }
}
