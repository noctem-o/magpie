use std::collections::BTreeSet;

use magpie_log::Status;
use serde::Serialize;

use crate::deadbolt_context::{
    resolve_deadbolt_occurrence_context, DeadboltAnchorOccurrence,
    DeadboltOccurrenceContextResolution,
};
use crate::policy::{
    support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
    SupportContextRequirement,
};
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::standing::{StandingCurrentness, StandingTraceEntry, StandingTraceReason};

/// Fixed identity for the first policy that can apply achieved standing.
///
/// V1 is selected only through the explicitly versioned snapshot methods. It
/// does not replace, alias, negotiate with, or become an ambient default for
/// governed-standing v0.
pub const MAGPIE_CLAIMS_POLICY_V1_ID: &str = "magpie-claims-standing-v1";

/// The only achieved-standing rule implemented by policy v1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingPolicyRule {
    DeadboltOccurrenceInclusionV1,
}

/// Serializable audit outcome for exact Deadbolt occurrence context.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum DeadboltOccurrenceContextTrace {
    Matched {
        occurrence: DeadboltAnchorOccurrence,
    },
    WrongEvidenceKind,
    WrongClaimDomain,
    MissingClaimPredicate,
    MalformedClaimPredicate,
    MissingEvidenceReference,
    MalformedEvidenceReference,
    PredicateReferenceMismatch,
    AnchorNotFound,
}

impl From<DeadboltOccurrenceContextResolution> for DeadboltOccurrenceContextTrace {
    fn from(resolution: DeadboltOccurrenceContextResolution) -> Self {
        match resolution {
            DeadboltOccurrenceContextResolution::Matched(occurrence) => {
                Self::Matched { occurrence }
            }
            DeadboltOccurrenceContextResolution::WrongEvidenceKind => Self::WrongEvidenceKind,
            DeadboltOccurrenceContextResolution::WrongClaimDomain => Self::WrongClaimDomain,
            DeadboltOccurrenceContextResolution::MissingClaimPredicate => {
                Self::MissingClaimPredicate
            }
            DeadboltOccurrenceContextResolution::MalformedClaimPredicate => {
                Self::MalformedClaimPredicate
            }
            DeadboltOccurrenceContextResolution::MissingEvidenceReference => {
                Self::MissingEvidenceReference
            }
            DeadboltOccurrenceContextResolution::MalformedEvidenceReference => {
                Self::MalformedEvidenceReference
            }
            DeadboltOccurrenceContextResolution::PredicateReferenceMismatch => {
                Self::PredicateReferenceMismatch
            }
            DeadboltOccurrenceContextResolution::AnchorNotFound => Self::AnchorNotFound,
        }
    }
}

impl DeadboltOccurrenceContextTrace {
    fn achieved_standing(&self) -> Option<Status> {
        match self {
            Self::Matched { .. } => Some(Status::Settled),
            Self::WrongEvidenceKind
            | Self::WrongClaimDomain
            | Self::MissingClaimPredicate
            | Self::MalformedClaimPredicate
            | Self::MissingEvidenceReference
            | Self::MalformedEvidenceReference
            | Self::PredicateReferenceMismatch
            | Self::AnchorNotFound => None,
        }
    }
}

/// One attempted application of the closed policy-v1 rule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingPolicyApplication {
    pub rule: StandingPolicyRule,
    pub context: DeadboltOccurrenceContextTrace,
    pub achieved_standing: Option<Status>,
}

impl StandingPolicyApplication {
    fn deadbolt_occurrence(context: DeadboltOccurrenceContextTrace) -> Self {
        let achieved_standing = context.achieved_standing();
        debug_assert_eq!(
            matches!(context, DeadboltOccurrenceContextTrace::Matched { .. }),
            achieved_standing == Some(Status::Settled),
        );
        Self {
            rule: StandingPolicyRule::DeadboltOccurrenceInclusionV1,
            context,
            achieved_standing,
        }
    }
}

/// One v0 candidate evaluation plus any policy-v1 application for that path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingTraceEntryV1 {
    pub candidate: StandingTraceEntry,
    pub application: Option<StandingPolicyApplication>,
}

/// Deterministic governed-standing explanation under explicit policy v1.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingResolutionV1 {
    pub claim_id: String,
    pub governed_standing: Option<Status>,
    pub legacy_raw_standing: Option<Status>,
    pub currentness: StandingCurrentness,
    pub policy_id: &'static str,
    pub trace: Vec<StandingTraceEntryV1>,
    pub blockers: Vec<StandingTraceReason>,
}

impl StandingResolutionV1 {
    /// Deterministic derived bytes for replay and explanation tests.
    ///
    /// These bytes are not L0 encoding or a persistent protocol contract.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingResolutionV1 is always serializable")
    }
}

impl StandingReplaySnapshot {
    /// Resolve one claim under the explicit governed-standing v1 policy.
    pub fn resolved_standing_v1(&self, claim_id: &str) -> Option<Status> {
        self.resolved_standing_with_trace_v1(claim_id)
            .governed_standing
    }

    /// Resolve one claim with layered v0-candidate and v1-application audit material.
    pub fn resolved_standing_with_trace_v1(&self, claim_id: &str) -> StandingResolutionV1 {
        let baseline = self.standing().resolved_standing_with_trace(claim_id);
        let mut achieved_settled = false;
        let trace = baseline
            .trace
            .iter()
            .cloned()
            .map(|candidate| {
                let application = classify_contribution_lane(&candidate)
                    .and_then(|rule| self.apply_contribution_lane(claim_id, &candidate, rule));
                achieved_settled |= application
                    .as_ref()
                    .is_some_and(|value| value.achieved_standing == Some(Status::Settled));
                StandingTraceEntryV1 {
                    candidate,
                    application,
                }
            })
            .collect::<Vec<_>>();

        let governed_standing = if baseline.governed_standing.is_some() && achieved_settled {
            Some(Status::Settled)
        } else {
            baseline.governed_standing
        };
        let blockers = unresolved_blockers(&baseline.blockers, &trace);

        StandingResolutionV1 {
            claim_id: baseline.claim_id,
            governed_standing,
            legacy_raw_standing: baseline.legacy_raw_standing,
            currentness: baseline.currentness,
            policy_id: MAGPIE_CLAIMS_POLICY_V1_ID,
            trace,
            blockers,
        }
    }

    fn apply_contribution_lane(
        &self,
        claim_id: &str,
        candidate: &StandingTraceEntry,
        rule: StandingPolicyRule,
    ) -> Option<StandingPolicyApplication> {
        match rule {
            StandingPolicyRule::DeadboltOccurrenceInclusionV1 => {
                self.apply_deadbolt_occurrence_lane(claim_id, candidate)
            }
        }
    }

    fn apply_deadbolt_occurrence_lane(
        &self,
        claim_id: &str,
        candidate: &StandingTraceEntry,
    ) -> Option<StandingPolicyApplication> {
        if candidate.target_id != claim_id {
            return None;
        }

        let edge_id = candidate.edge_id.as_deref()?;
        let source_id = candidate.source_id.as_deref()?;
        let edge = self.standing().justification_edge(edge_id)?;
        let target = self.standing().typed_claim(claim_id)?;
        let evidence = self.standing().typed_evidence(source_id)?;

        if self.standing().get(claim_id).is_none()
            || edge.source_id != source_id
            || edge.target_id != claim_id
            || edge.edge_kind != "supports"
            || evidence.scope_ref != edge.scope_ref
            || edge.scope_ref != target.scope_ref
            || evidence.evidence_kind != EvidenceKind::DeadboltAnchor.as_str()
            || support_ceiling(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion,
            ) != Some(Status::Settled)
            || support_context_requirement(
                EvidenceKind::DeadboltAnchor,
                ClaimDomain::OccurrenceInclusion,
            ) != SupportContextRequirement::DeadboltVerifierContext
        {
            return None;
        }

        let context = resolve_deadbolt_occurrence_context(
            EvidenceKind::DeadboltAnchor,
            ClaimDomain::OccurrenceInclusion,
            &target.metadata_json,
            &evidence.metadata_json,
            self.anchors(),
        )
        .into();
        Some(StandingPolicyApplication::deadbolt_occurrence(context))
    }
}

fn classify_contribution_lane(candidate: &StandingTraceEntry) -> Option<StandingPolicyRule> {
    (candidate.edge_id.is_some()
        && candidate.source_id.is_some()
        && candidate.evidence_kind.as_deref() == Some(EvidenceKind::DeadboltAnchor.as_str())
        && candidate.claim_domain.as_deref() == Some(ClaimDomain::OccurrenceInclusion.as_str())
        && candidate.candidate_ceiling == Some(Status::Settled)
        && candidate.reasons
            == [
                StandingTraceReason::AcceptedCandidate,
                StandingTraceReason::RequiresVerifierContext,
                StandingTraceReason::CeilingIsCandidateOnly,
            ])
    .then_some(StandingPolicyRule::DeadboltOccurrenceInclusionV1)
}

fn unresolved_blockers(
    baseline_blockers: &[StandingTraceReason],
    trace: &[StandingTraceEntryV1],
) -> Vec<StandingTraceReason> {
    let mut blockers = BTreeSet::new();
    for reason in baseline_blockers {
        if *reason == StandingTraceReason::AcceptedCandidate
            || (matches!(
                reason,
                StandingTraceReason::RequiresVerifierContext
                    | StandingTraceReason::CeilingIsCandidateOnly
            ) && !trace.iter().any(|entry| {
                entry.candidate.reasons.contains(reason)
                    && !entry.application.as_ref().is_some_and(|application| {
                        application.achieved_standing == Some(Status::Settled)
                    })
            }))
        {
            continue;
        }
        blockers.insert(*reason);
    }
    for entry in trace {
        let matched = entry
            .application
            .as_ref()
            .is_some_and(|application| application.achieved_standing == Some(Status::Settled));
        for reason in &entry.candidate.reasons {
            if *reason == StandingTraceReason::AcceptedCandidate
                || (matched
                    && matches!(
                        reason,
                        StandingTraceReason::RequiresVerifierContext
                            | StandingTraceReason::CeilingIsCandidateOnly
                    ))
            {
                continue;
            }
            blockers.insert(*reason);
        }
    }
    blockers.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exact_deadbolt_occurrence_candidate() -> StandingTraceEntry {
        StandingTraceEntry {
            edge_id: Some("edge".into()),
            source_id: Some("evidence".into()),
            target_id: "claim".into(),
            evidence_kind: Some(EvidenceKind::DeadboltAnchor.as_str().into()),
            claim_domain: Some(ClaimDomain::OccurrenceInclusion.as_str().into()),
            candidate_ceiling: Some(Status::Settled),
            reasons: vec![
                StandingTraceReason::AcceptedCandidate,
                StandingTraceReason::RequiresVerifierContext,
                StandingTraceReason::CeilingIsCandidateOnly,
            ],
        }
    }

    #[test]
    fn exact_candidate_selects_the_only_closed_contribution_lane() {
        let selected = classify_contribution_lane(&exact_deadbolt_occurrence_candidate());

        assert_eq!(
            selected,
            Some(StandingPolicyRule::DeadboltOccurrenceInclusionV1)
        );
        assert_eq!(selected.into_iter().count(), 1);
    }

    #[test]
    fn closed_contribution_lane_classification_rejects_boundary_changes() {
        let exact = exact_deadbolt_occurrence_candidate();
        let mut cases = Vec::new();

        let mut wrong_evidence_kind = exact.clone();
        wrong_evidence_kind.evidence_kind = Some(EvidenceKind::ExternalSource.as_str().into());
        cases.push(("wrong evidence kind", wrong_evidence_kind));

        let mut wrong_claim_domain = exact.clone();
        wrong_claim_domain.claim_domain = Some(ClaimDomain::Interpretation.as_str().into());
        cases.push(("wrong claim domain", wrong_claim_domain));

        let mut wrong_candidate_ceiling = exact.clone();
        wrong_candidate_ceiling.candidate_ceiling = Some(Status::Supported);
        cases.push(("wrong candidate ceiling", wrong_candidate_ceiling));

        let mut missing_edge = exact.clone();
        missing_edge.edge_id = None;
        cases.push(("missing edge", missing_edge));

        let mut missing_source = exact.clone();
        missing_source.source_id = None;
        cases.push(("missing source", missing_source));

        let mut altered_reason_set = exact.clone();
        altered_reason_set.reasons.pop();
        cases.push(("altered reason set", altered_reason_set));

        let mut altered_reason_order = exact.clone();
        altered_reason_order.reasons.swap(1, 2);
        cases.push(("altered reason order", altered_reason_order));

        let mut unknown_vocabulary = exact;
        unknown_vocabulary.evidence_kind = Some("UnknownEvidenceKind".into());
        unknown_vocabulary.candidate_ceiling = None;
        unknown_vocabulary.reasons = vec![StandingTraceReason::UnknownEvidenceKind];
        cases.push(("unknown vocabulary", unknown_vocabulary));

        for (case, candidate) in cases {
            assert_eq!(classify_contribution_lane(&candidate), None, "{case}");
        }
    }

    #[test]
    fn context_outcome_and_achieved_standing_correspond_exactly() {
        let unmatched = [
            DeadboltOccurrenceContextTrace::WrongEvidenceKind,
            DeadboltOccurrenceContextTrace::WrongClaimDomain,
            DeadboltOccurrenceContextTrace::MissingClaimPredicate,
            DeadboltOccurrenceContextTrace::MalformedClaimPredicate,
            DeadboltOccurrenceContextTrace::MissingEvidenceReference,
            DeadboltOccurrenceContextTrace::MalformedEvidenceReference,
            DeadboltOccurrenceContextTrace::PredicateReferenceMismatch,
            DeadboltOccurrenceContextTrace::AnchorNotFound,
        ];
        for context in unmatched {
            assert_eq!(
                StandingPolicyApplication::deadbolt_occurrence(context).achieved_standing,
                None
            );
        }

        let occurrence = DeadboltAnchorOccurrence {
            identity: crate::DeadboltAnchorIdentity {
                bundle_kind: "kind".into(),
                witness_root: "a".repeat(64),
                witness_algorithm: "sha256".into(),
                canonicalization_profile: "profile".into(),
                run_id: "run".into(),
            },
            sequence: 1,
            event_hash: "hash".into(),
            provenance_agent: "agent".into(),
            provenance_source: "source".into(),
        };
        assert_eq!(
            StandingPolicyApplication::deadbolt_occurrence(
                DeadboltOccurrenceContextTrace::Matched { occurrence }
            )
            .achieved_standing,
            Some(Status::Settled)
        );
    }

    #[test]
    fn unknown_evidence_kind_trace_is_not_v1_eligible() {
        let candidate = StandingTraceEntry {
            edge_id: Some("edge".into()),
            source_id: Some("evidence".into()),
            target_id: "claim".into(),
            evidence_kind: Some("UnknownEvidenceKind".into()),
            claim_domain: Some("Occurrence/Inclusion".into()),
            candidate_ceiling: None,
            reasons: vec![StandingTraceReason::UnknownEvidenceKind],
        };

        assert_eq!(classify_contribution_lane(&candidate), None);
    }
}
