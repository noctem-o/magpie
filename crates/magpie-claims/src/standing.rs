use std::collections::{BTreeMap, BTreeSet};

use magpie_log::{Payload, Projection, SignedEvent, Status};
use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::policy::{
    support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
    SupportContextRequirement,
};

/// Fixed policy identifier for the first governed-standing explanation engine.
///
/// V0 deliberately has no runtime-selectable policy versions. Every resolution
/// names the one compiled policy so callers cannot mistake an ambient policy
/// change for evidence from L0.
pub const MAGPIE_CLAIMS_POLICY_ID: &str = "magpie-claims-standing-v0";

/// Whether a resolution is known to describe the current claim.
///
/// Supersession is outside v0, so the resolver conservatively returns
/// [`StandingCurrentness::Unknown`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingCurrentness {
    Current,
    Unknown,
}

/// Closed explanation vocabulary for governed-standing v0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingTraceReason {
    AcceptedCandidate,
    UnsupportedEdgeKindForV0,
    MissingTargetClaim,
    MissingSourceEvidence,
    MissingTypedTargetClaim,
    ScopeMismatch,
    UnknownEvidenceKind,
    MissingClaimDomain,
    MalformedMetadata,
    WrongTypeClaimDomain,
    UnknownClaimDomain,
    DuplicateMetadataKey,
    RequiresAdmission,
    RequiresVerifierContext,
    NoSupportCeiling,
    CeilingIsCandidateOnly,
    LegacyRawStatusQuarantined,
}

impl StandingTraceReason {
    fn is_blocker(self) -> bool {
        self != Self::AcceptedCandidate
    }
}

/// Deterministic explanation for one target edge, or for a claim-level blocker
/// when `edge_id` is absent.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingTraceEntry {
    pub edge_id: Option<String>,
    pub source_id: Option<String>,
    pub target_id: String,
    pub evidence_kind: Option<String>,
    pub claim_domain: Option<String>,
    pub candidate_ceiling: Option<Status>,
    pub reasons: Vec<StandingTraceReason>,
}

/// Canonical governed-standing explanation for one claim under policy v0.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingResolution {
    pub claim_id: String,
    pub governed_standing: Option<Status>,
    pub legacy_raw_standing: Option<Status>,
    pub currentness: StandingCurrentness,
    pub policy_id: &'static str,
    pub trace: Vec<StandingTraceEntry>,
    pub blockers: Vec<StandingTraceReason>,
}

impl StandingResolution {
    /// Deterministic projection bytes for replay and explanation tests.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingResolution is always serializable")
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SupportCandidateStructureFailureV0 {
    MissingTargetClaim,
    MissingTypedTargetClaim,
    UnsupportedEdgeKindForV0,
    MalformedMetadata,
    MissingClaimDomain,
    WrongTypeClaimDomain,
    DuplicateMetadataKey,
    UnknownClaimDomain,
    MissingSourceEvidence,
    ScopeMismatch,
    UnknownEvidenceKind,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct SupportCandidateStructureEvaluationV0<'a> {
    failure: Option<SupportCandidateStructureFailureV0>,
    evidence: Option<&'a TypedEvidenceNode>,
    claim_domain: Option<ClaimDomain>,
    evidence_kind: Option<EvidenceKind>,
}

impl<'a> SupportCandidateStructureEvaluationV0<'a> {
    pub(crate) fn failure(&self) -> Option<SupportCandidateStructureFailureV0> {
        self.failure
    }

    pub(crate) fn evidence(&self) -> Option<&'a TypedEvidenceNode> {
        self.evidence
    }

    pub(crate) fn claim_domain(&self) -> Option<ClaimDomain> {
        self.claim_domain
    }

    pub(crate) fn evidence_kind(&self) -> Option<EvidenceKind> {
        self.evidence_kind
    }
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

    /// Compatibility scalar for callers that do not yet consume explanations.
    ///
    /// This delegates to [`Self::resolved_standing_with_trace`] and returns only
    /// its governed result. Legacy/manual status is intentionally available only
    /// as [`StandingResolution::legacy_raw_standing`]; it is not an authoritative
    /// input to governed standing.
    pub fn resolved_standing(&self, claim_id: &str) -> Option<Status> {
        self.resolved_standing_with_trace(claim_id)
            .governed_standing
    }

    /// Resolve one typed claim under the fixed, fail-closed v0 standing policy.
    ///
    /// V0 explains candidate support ceilings but performs no support
    /// aggregation or promotion. A typed claim therefore remains Conjectured;
    /// legacy status is quarantined, and every edge targeting the claim receives
    /// one deterministic trace entry.
    pub fn resolved_standing_with_trace(&self, claim_id: &str) -> StandingResolution {
        let legacy_raw_standing = self.claims.get(claim_id).map(|claim| claim.standing);
        let typed_target = self.typed_claims.get(claim_id);
        let governed_standing = if legacy_raw_standing.is_some() && typed_target.is_some() {
            Some(Status::Conjectured)
        } else {
            None
        };
        let claim_domain = typed_target.map(|target| parse_claim_domain(&target.metadata_json));
        let mut trace = Vec::new();
        let mut blockers = BTreeSet::new();

        if let Some(claim) = self.claims.get(claim_id) {
            if typed_target.is_some()
                && (claim.legacy_transitions > 0 || claim.standing != Status::Conjectured)
            {
                push_claim_trace(
                    &mut trace,
                    claim_id,
                    StandingTraceReason::LegacyRawStatusQuarantined,
                );
                blockers.insert(StandingTraceReason::LegacyRawStatusQuarantined);
            }
        } else {
            blockers.insert(StandingTraceReason::MissingTargetClaim);
        }

        if typed_target.is_none() {
            blockers.insert(StandingTraceReason::MissingTypedTargetClaim);
        }

        if let Some(Err(reason)) = claim_domain {
            blockers.insert(standing_reason_for_structure_failure(reason));
        }

        let mut relevant_edge_count = 0usize;
        for (edge_id, edge) in &self.justification_edges {
            if edge.target_id != claim_id {
                continue;
            }
            relevant_edge_count += 1;
            let entry = self.resolve_support_candidate(claim_id, edge_id, edge, claim_domain);
            for reason in &entry.reasons {
                if reason.is_blocker() {
                    blockers.insert(*reason);
                }
            }
            trace.push(entry);
        }

        if relevant_edge_count == 0 {
            if legacy_raw_standing.is_none() {
                push_claim_trace(
                    &mut trace,
                    claim_id,
                    StandingTraceReason::MissingTargetClaim,
                );
            } else if typed_target.is_none() {
                push_claim_trace(
                    &mut trace,
                    claim_id,
                    StandingTraceReason::MissingTypedTargetClaim,
                );
            } else if let Some(Err(reason)) = claim_domain {
                push_claim_trace(
                    &mut trace,
                    claim_id,
                    standing_reason_for_structure_failure(reason),
                );
            }
        }

        StandingResolution {
            claim_id: claim_id.to_owned(),
            governed_standing,
            legacy_raw_standing,
            currentness: StandingCurrentness::Unknown,
            policy_id: MAGPIE_CLAIMS_POLICY_ID,
            trace,
            blockers: blockers.into_iter().collect(),
        }
    }

    fn resolve_support_candidate(
        &self,
        claim_id: &str,
        edge_id: &str,
        edge: &TypedJustificationEdge,
        claim_domain: Option<Result<ClaimDomain, SupportCandidateStructureFailureV0>>,
    ) -> StandingTraceEntry {
        let mut entry = StandingTraceEntry {
            edge_id: Some(edge_id.to_owned()),
            source_id: Some(edge.source_id.clone()),
            target_id: claim_id.to_owned(),
            evidence_kind: None,
            claim_domain: claim_domain
                .and_then(Result::ok)
                .map(|domain| domain.as_str().to_owned()),
            candidate_ceiling: None,
            reasons: Vec::new(),
        };

        let evaluation = self.evaluate_support_candidate_structure_v0(claim_id, edge);
        entry.evidence_kind = evaluation
            .evidence()
            .map(|evidence| evidence.evidence_kind.clone());
        if let Some(failure) = evaluation.failure() {
            entry
                .reasons
                .push(standing_reason_for_structure_failure(failure));
            return entry;
        }
        let domain = evaluation
            .claim_domain()
            .expect("successful structural evaluation has a claim domain");
        let kind = evaluation
            .evidence_kind()
            .expect("successful structural evaluation has an evidence kind");

        entry.candidate_ceiling = support_ceiling(kind, domain);
        let context_requirement = support_context_requirement(kind, domain);
        debug_assert_eq!(
            entry.candidate_ceiling.is_none(),
            context_requirement == SupportContextRequirement::NoSupportContribution,
        );
        if support_candidate_is_rejected(entry.candidate_ceiling, context_requirement) {
            entry.candidate_ceiling = None;
            entry.reasons.push(StandingTraceReason::NoSupportCeiling);
            return entry;
        }

        entry.reasons.push(StandingTraceReason::AcceptedCandidate);
        match context_requirement {
            SupportContextRequirement::NoSupportContribution => {
                unreachable!("no-support requirements return before candidate acceptance")
            }
            SupportContextRequirement::HumanAdmission => {
                entry.reasons.push(StandingTraceReason::RequiresAdmission)
            }
            SupportContextRequirement::DeterministicVerifierContext
            | SupportContextRequirement::DeadboltVerifierContext => entry
                .reasons
                .push(StandingTraceReason::RequiresVerifierContext),
            SupportContextRequirement::NoPrivilegedContext => {}
        }
        entry
            .reasons
            .push(StandingTraceReason::CeilingIsCandidateOnly);
        entry
    }

    pub(crate) fn evaluate_support_candidate_structure_v0<'a>(
        &'a self,
        target_claim_id: &str,
        edge: &TypedJustificationEdge,
    ) -> SupportCandidateStructureEvaluationV0<'a> {
        if !self.claims.contains_key(target_claim_id) {
            return SupportCandidateStructureEvaluationV0 {
                failure: Some(SupportCandidateStructureFailureV0::MissingTargetClaim),
                evidence: None,
                claim_domain: None,
                evidence_kind: None,
            };
        }
        let Some(target) = self.typed_claims.get(target_claim_id) else {
            return SupportCandidateStructureEvaluationV0 {
                failure: Some(SupportCandidateStructureFailureV0::MissingTypedTargetClaim),
                evidence: None,
                claim_domain: None,
                evidence_kind: None,
            };
        };
        if edge.edge_kind != "supports" {
            return SupportCandidateStructureEvaluationV0 {
                failure: Some(SupportCandidateStructureFailureV0::UnsupportedEdgeKindForV0),
                evidence: None,
                claim_domain: None,
                evidence_kind: None,
            };
        }
        let domain = match parse_claim_domain(&target.metadata_json) {
            Ok(domain) => domain,
            Err(failure) => {
                return SupportCandidateStructureEvaluationV0 {
                    failure: Some(failure),
                    evidence: None,
                    claim_domain: None,
                    evidence_kind: None,
                }
            }
        };
        let Some(evidence) = self.typed_evidence.get(&edge.source_id) else {
            return SupportCandidateStructureEvaluationV0 {
                failure: Some(SupportCandidateStructureFailureV0::MissingSourceEvidence),
                evidence: None,
                claim_domain: Some(domain),
                evidence_kind: None,
            };
        };
        if evidence.scope_ref != edge.scope_ref || edge.scope_ref != target.scope_ref {
            return SupportCandidateStructureEvaluationV0 {
                failure: Some(SupportCandidateStructureFailureV0::ScopeMismatch),
                evidence: Some(evidence),
                claim_domain: Some(domain),
                evidence_kind: None,
            };
        }
        let kind = match EvidenceKind::try_from(evidence.evidence_kind.as_str()) {
            Ok(kind) => kind,
            Err(_) => {
                return SupportCandidateStructureEvaluationV0 {
                    failure: Some(SupportCandidateStructureFailureV0::UnknownEvidenceKind),
                    evidence: Some(evidence),
                    claim_domain: Some(domain),
                    evidence_kind: None,
                }
            }
        };

        SupportCandidateStructureEvaluationV0 {
            failure: None,
            evidence: Some(evidence),
            claim_domain: Some(domain),
            evidence_kind: Some(kind),
        }
    }
}

fn support_candidate_is_rejected(
    candidate_ceiling: Option<Status>,
    context_requirement: SupportContextRequirement,
) -> bool {
    candidate_ceiling.is_none()
        || context_requirement == SupportContextRequirement::NoSupportContribution
}

fn push_claim_trace(
    trace: &mut Vec<StandingTraceEntry>,
    claim_id: &str,
    reason: StandingTraceReason,
) {
    trace.push(StandingTraceEntry {
        edge_id: None,
        source_id: None,
        target_id: claim_id.to_owned(),
        evidence_kind: None,
        claim_domain: None,
        candidate_ceiling: None,
        reasons: vec![reason],
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClaimDomainMetadataIssue {
    Missing,
    WrongType,
    DuplicateKey,
}

#[derive(Debug)]
struct ClaimDomainMetadata {
    claim_domain: Option<String>,
    issue: Option<ClaimDomainMetadataIssue>,
}

impl<'de> Deserialize<'de> for ClaimDomainMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClaimDomainMetadataVisitor;

        impl<'de> Visitor<'de> for ClaimDomainMetadataVisitor {
            type Value = ClaimDomainMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a JSON object containing one string claim_domain")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut seen_keys = BTreeSet::new();
                let mut claim_domain = None;
                let mut issue = None;

                while let Some(key) = map.next_key::<String>()? {
                    let duplicate = !seen_keys.insert(key.clone());
                    if duplicate {
                        issue = Some(ClaimDomainMetadataIssue::DuplicateKey);
                    }
                    if key == "claim_domain" {
                        let value = map.next_value::<serde_json::Value>()?;
                        if !duplicate {
                            if let serde_json::Value::String(domain) = value {
                                claim_domain = Some(domain);
                            } else if issue.is_none() {
                                issue = Some(ClaimDomainMetadataIssue::WrongType);
                            }
                        }
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }

                if claim_domain.is_none() && issue.is_none() {
                    issue = Some(ClaimDomainMetadataIssue::Missing);
                }
                Ok(ClaimDomainMetadata {
                    claim_domain,
                    issue,
                })
            }
        }

        deserializer.deserialize_map(ClaimDomainMetadataVisitor)
    }
}

fn parse_claim_domain(
    metadata_json: &str,
) -> Result<ClaimDomain, SupportCandidateStructureFailureV0> {
    let mut deserializer = serde_json::Deserializer::from_str(metadata_json);
    let metadata = ClaimDomainMetadata::deserialize(&mut deserializer)
        .map_err(|_| SupportCandidateStructureFailureV0::MalformedMetadata)?;
    deserializer
        .end()
        .map_err(|_| SupportCandidateStructureFailureV0::MalformedMetadata)?;

    match metadata.issue {
        Some(ClaimDomainMetadataIssue::Missing) => {
            return Err(SupportCandidateStructureFailureV0::MissingClaimDomain)
        }
        Some(ClaimDomainMetadataIssue::WrongType) => {
            return Err(SupportCandidateStructureFailureV0::WrongTypeClaimDomain)
        }
        Some(ClaimDomainMetadataIssue::DuplicateKey) => {
            return Err(SupportCandidateStructureFailureV0::DuplicateMetadataKey)
        }
        None => {}
    }

    let raw = metadata
        .claim_domain
        .ok_or(SupportCandidateStructureFailureV0::MissingClaimDomain)?;
    ClaimDomain::try_from(raw.as_str())
        .map_err(|_| SupportCandidateStructureFailureV0::UnknownClaimDomain)
}

fn standing_reason_for_structure_failure(
    failure: SupportCandidateStructureFailureV0,
) -> StandingTraceReason {
    match failure {
        SupportCandidateStructureFailureV0::MissingTargetClaim => {
            StandingTraceReason::MissingTargetClaim
        }
        SupportCandidateStructureFailureV0::MissingTypedTargetClaim => {
            StandingTraceReason::MissingTypedTargetClaim
        }
        SupportCandidateStructureFailureV0::UnsupportedEdgeKindForV0 => {
            StandingTraceReason::UnsupportedEdgeKindForV0
        }
        SupportCandidateStructureFailureV0::MalformedMetadata => {
            StandingTraceReason::MalformedMetadata
        }
        SupportCandidateStructureFailureV0::MissingClaimDomain => {
            StandingTraceReason::MissingClaimDomain
        }
        SupportCandidateStructureFailureV0::WrongTypeClaimDomain => {
            StandingTraceReason::WrongTypeClaimDomain
        }
        SupportCandidateStructureFailureV0::DuplicateMetadataKey => {
            StandingTraceReason::DuplicateMetadataKey
        }
        SupportCandidateStructureFailureV0::UnknownClaimDomain => {
            StandingTraceReason::UnknownClaimDomain
        }
        SupportCandidateStructureFailureV0::MissingSourceEvidence => {
            StandingTraceReason::MissingSourceEvidence
        }
        SupportCandidateStructureFailureV0::ScopeMismatch => StandingTraceReason::ScopeMismatch,
        SupportCandidateStructureFailureV0::UnknownEvidenceKind => {
            StandingTraceReason::UnknownEvidenceKind
        }
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
        LogWriter::<MemStore>::open_with_clock(
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

    #[test]
    fn support_candidate_gate_fails_closed_on_policy_disagreement() {
        assert!(support_candidate_is_rejected(
            None,
            SupportContextRequirement::NoPrivilegedContext
        ));
        assert!(support_candidate_is_rejected(
            Some(Status::Supported),
            SupportContextRequirement::NoSupportContribution
        ));
        assert!(!support_candidate_is_rejected(
            Some(Status::Supported),
            SupportContextRequirement::NoPrivilegedContext
        ));
    }

    #[test]
    fn shared_structure_evaluator_retains_unknown_raw_evidence_kind_fail_closed() {
        let mut view = StandingView::new();
        view.claims.insert(
            "claim".into(),
            StandingClaim {
                statement: "claim".into(),
                asserted_initial_status: Status::Conjectured,
                standing: Status::Conjectured,
                legacy_evidence: Vec::new(),
                legacy_transitions: 0,
            },
        );
        view.typed_claims.insert(
            "claim".into(),
            TypedClaimNode {
                statement: "claim".into(),
                scope_ref: "scope".into(),
                actor_class: "AgentProposer".into(),
                content_hash: String::new(),
                metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
            },
        );
        view.typed_evidence.insert(
            "evidence".into(),
            TypedEvidenceNode {
                evidence_kind: "UnknownEvidenceKind".into(),
                summary: "synthetic private evaluator input".into(),
                scope_ref: "scope".into(),
                actor_class: "SourceImporter".into(),
                content_hash: "a".repeat(64),
                metadata_json: "{}".into(),
            },
        );
        let edge = TypedJustificationEdge {
            edge_kind: "supports".into(),
            source_id: "evidence".into(),
            target_id: "claim".into(),
            scope_ref: "scope".into(),
            actor_class: "HumanRoot".into(),
            rationale: "candidate only".into(),
            metadata_json: "{}".into(),
        };

        let evaluation = view.evaluate_support_candidate_structure_v0("claim", &edge);
        assert_eq!(
            evaluation.failure(),
            Some(SupportCandidateStructureFailureV0::UnknownEvidenceKind)
        );
        assert_eq!(
            evaluation
                .evidence()
                .map(|evidence| evidence.evidence_kind.as_str()),
            Some("UnknownEvidenceKind")
        );
        assert_eq!(evaluation.claim_domain(), Some(ClaimDomain::ExternalReport));
        assert_eq!(evaluation.evidence_kind(), None);
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
    fn supported_tier_ceiling_does_not_promote_in_v0() {
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
                Some(Status::Conjectured),
                "{kind} must remain a candidate in v0"
            );
            // Raw standing is never mutated by resolution.
            assert_eq!(view.get("c1").unwrap().standing, Status::Conjectured);
        }
    }

    #[test]
    fn weak_evidence_does_not_promote_in_v0() {
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
    fn multiple_supports_do_not_aggregate_or_promote_in_v0() {
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
        assert_eq!(view.resolved_standing("c1"), Some(Status::Conjectured));
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
        assert_eq!(view.resolved_standing("c0"), None);
    }

    #[test]
    fn typed_claim_without_support_resolves_to_governed_baseline() {
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

        assert_eq!(forward.resolved_standing("c1"), Some(Status::Conjectured));
        assert_eq!(
            forward.resolved_standing("c1"),
            reversed.resolved_standing("c1")
        );
    }

    #[test]
    fn refuted_raw_claim_is_quarantined_from_governed_standing() {
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
        assert_eq!(view.resolved_standing("c1"), Some(Status::Conjectured));
        assert_eq!(
            view.resolved_standing_with_trace("c1").legacy_raw_standing,
            Some(Status::Refuted)
        );
    }

    #[test]
    fn settled_raw_claim_is_quarantined_from_governed_standing() {
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
        assert_eq!(view.resolved_standing("c1"), Some(Status::Conjectured));
        assert_eq!(
            view.resolved_standing_with_trace("c1").legacy_raw_standing,
            Some(Status::Settled)
        );
    }
}
