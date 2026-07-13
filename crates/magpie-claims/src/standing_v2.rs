//! Explicit policy-v2 direct support over trusted same-snapshot verifier context.
//!
//! V2 is selected only through explicitly versioned [`StandingReplaySnapshot`]
//! methods. It does not replace v0 or v1, is not an alias for "latest", and is
//! neither runtime-selected nor negotiated from log data. No ambient mutable
//! policy participates.
//!
//! A standalone standing projection deliberately has no v2 resolver:
//!
//! ```compile_fail
//! use magpie_claims::StandingView;
//! let view = StandingView::new();
//! let _ = view.resolved_standing_v2("claim");
//! ```

use std::collections::BTreeSet;

use magpie_log::Status;
use serde::Serialize;

use crate::deterministic_verifier_context::DeterministicVerifierContextTraceV0;
use crate::policy::{ClaimDomain, EvidenceKind};
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::standing::{StandingCurrentness, StandingTraceEntry, StandingTraceReason};
use crate::standing_v1::{
    DeadboltOccurrenceContextTrace, StandingPolicyApplication, StandingPolicyRule,
};

/// Fixed identity for the explicitly selected governed-standing v2 policy.
pub const MAGPIE_CLAIMS_POLICY_V2_ID: &str = "magpie-claims-standing-v2";

/// Closed rule vocabulary for policy v2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingPolicyRuleV2 {
    DeadboltOccurrenceInclusionV1,
    Sha256BytesEqualsDirectSupportV0,
}

/// Exact inherited or deterministic context retained by a v2 application.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingPolicyContextV2 {
    DeadboltOccurrenceV1 {
        trace: DeadboltOccurrenceContextTrace,
    },
    DeterministicVerifierV0 {
        trace: DeterministicVerifierContextTraceV0,
    },
    CandidateRevalidationFailed,
}

/// One attempted application of the closed policy-v2 rule vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingPolicyApplicationV2 {
    pub rule: StandingPolicyRuleV2,
    pub context: StandingPolicyContextV2,
    pub achieved_standing: Option<Status>,
}

impl StandingPolicyApplicationV2 {
    fn inherited_deadbolt(application: StandingPolicyApplication) -> Self {
        let StandingPolicyApplication {
            rule,
            context,
            achieved_standing,
        } = application;
        debug_assert_eq!(rule, StandingPolicyRule::DeadboltOccurrenceInclusionV1);
        debug_assert_eq!(
            matches!(context, DeadboltOccurrenceContextTrace::Matched { .. }),
            achieved_standing == Some(Status::Settled),
        );
        Self {
            rule: StandingPolicyRuleV2::DeadboltOccurrenceInclusionV1,
            context: StandingPolicyContextV2::DeadboltOccurrenceV1 { trace: context },
            achieved_standing,
        }
    }

    fn deterministic(context: Option<DeterministicVerifierContextTraceV0>) -> Self {
        let (context, achieved_standing) = match context {
            None => (StandingPolicyContextV2::CandidateRevalidationFailed, None),
            Some(trace) => {
                let achieved = matches!(trace, DeterministicVerifierContextTraceV0::Matched(_))
                    .then_some(Status::Supported);
                (
                    StandingPolicyContextV2::DeterministicVerifierV0 { trace },
                    achieved,
                )
            }
        };
        debug_assert_eq!(
            matches!(
                context,
                StandingPolicyContextV2::DeterministicVerifierV0 {
                    trace: DeterministicVerifierContextTraceV0::Matched(_)
                }
            ),
            achieved_standing == Some(Status::Supported),
        );
        debug_assert_ne!(achieved_standing, Some(Status::Settled));
        Self {
            rule: StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0,
            context,
            achieved_standing,
        }
    }
}

/// One v0 candidate evaluation plus any policy-v2 application for that path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingTraceEntryV2 {
    pub candidate: StandingTraceEntry,
    pub application: Option<StandingPolicyApplicationV2>,
}

/// Deterministic governed-standing explanation under explicit policy v2.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingResolutionV2 {
    pub claim_id: String,
    pub governed_standing: Option<Status>,
    pub legacy_raw_standing: Option<Status>,
    pub currentness: StandingCurrentness,
    pub policy_id: &'static str,
    pub trace: Vec<StandingTraceEntryV2>,
    pub blockers: Vec<StandingTraceReason>,
}

impl StandingResolutionV2 {
    /// Deterministic policy-pinned derived audit bytes.
    ///
    /// These bytes are not L0 canonical encoding, a new event format, a
    /// persistent network protocol, or evidence that the policy is wise.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingResolutionV2 is always serializable")
    }
}

impl StandingReplaySnapshot {
    /// Resolve one claim under the explicitly selected governed-standing v2 policy.
    pub fn resolved_standing_v2(&self, claim_id: &str) -> Option<Status> {
        self.resolved_standing_with_trace_v2(claim_id)
            .governed_standing
    }

    /// Resolve one claim with inherited v1 and direct deterministic v2 audit material.
    pub fn resolved_standing_with_trace_v2(&self, claim_id: &str) -> StandingResolutionV2 {
        let baseline = self.standing().resolved_standing_with_trace(claim_id);
        let inherited = self.resolved_standing_with_trace_v1(claim_id);
        debug_assert_eq!(baseline.trace.len(), inherited.trace.len());

        let mut achieved_deterministic_supported = false;
        let trace = inherited
            .trace
            .into_iter()
            .zip(&baseline.trace)
            .map(|(entry, baseline_candidate)| {
                debug_assert_eq!(&entry.candidate, baseline_candidate);
                let application = match entry.application {
                    Some(application) => {
                        let achieved = application.achieved_standing;
                        let converted =
                            StandingPolicyApplicationV2::inherited_deadbolt(application);
                        debug_assert_eq!(converted.achieved_standing, achieved);
                        Some(converted)
                    }
                    None => classify_deterministic_candidate(&entry.candidate).map(|rule| {
                        debug_assert_eq!(
                            rule,
                            StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0
                        );
                        let edge_id = entry
                            .candidate
                            .edge_id
                            .as_deref()
                            .expect("classified deterministic candidate has an edge id");
                        let source_id = entry
                            .candidate
                            .source_id
                            .as_deref()
                            .expect("classified deterministic candidate has a source id");
                        StandingPolicyApplicationV2::deterministic(
                            self.resolve_deterministic_verifier_context_v0(
                                claim_id, source_id, edge_id,
                            ),
                        )
                    }),
                };
                achieved_deterministic_supported |= application.as_ref().is_some_and(|value| {
                    value.rule == StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0
                        && value.achieved_standing == Some(Status::Supported)
                });
                StandingTraceEntryV2 {
                    candidate: entry.candidate,
                    application,
                }
            })
            .collect::<Vec<_>>();

        let governed_standing = combine_v2_standing(
            inherited.governed_standing,
            achieved_deterministic_supported,
        );
        let blockers = unresolved_blockers_v2(&baseline.blockers, &trace);

        StandingResolutionV2 {
            claim_id: baseline.claim_id,
            governed_standing,
            legacy_raw_standing: baseline.legacy_raw_standing,
            currentness: baseline.currentness,
            policy_id: MAGPIE_CLAIMS_POLICY_V2_ID,
            trace,
            blockers,
        }
    }
}

fn classify_deterministic_candidate(
    candidate: &StandingTraceEntry,
) -> Option<StandingPolicyRuleV2> {
    (candidate.edge_id.is_some()
        && candidate.source_id.is_some()
        && candidate.evidence_kind.as_deref()
            == Some(EvidenceKind::DeterministicVerification.as_str())
        && candidate.claim_domain.as_deref() == Some(ClaimDomain::ExactMachineCheckable.as_str())
        && candidate.candidate_ceiling == Some(Status::Settled)
        && candidate.reasons
            == [
                StandingTraceReason::AcceptedCandidate,
                StandingTraceReason::RequiresVerifierContext,
                StandingTraceReason::CeilingIsCandidateOnly,
            ])
    .then_some(StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0)
}

fn combine_v2_standing(inherited: Option<Status>, deterministic_supported: bool) -> Option<Status> {
    match inherited {
        Some(Status::Settled) => Some(Status::Settled),
        Some(Status::Refuted) => Some(Status::Refuted),
        Some(_) if deterministic_supported => Some(Status::Supported),
        other => other,
    }
}

fn unresolved_blockers_v2(
    baseline_blockers: &[StandingTraceReason],
    trace: &[StandingTraceEntryV2],
) -> Vec<StandingTraceReason> {
    let mut blockers = BTreeSet::new();
    for reason in baseline_blockers {
        if !matches!(
            reason,
            StandingTraceReason::AcceptedCandidate
                | StandingTraceReason::RequiresVerifierContext
                | StandingTraceReason::CeilingIsCandidateOnly
        ) {
            blockers.insert(*reason);
        }
    }
    for entry in trace {
        for reason in &entry.candidate.reasons {
            if !matches!(
                reason,
                StandingTraceReason::AcceptedCandidate
                    | StandingTraceReason::RequiresVerifierContext
                    | StandingTraceReason::CeilingIsCandidateOnly
            ) {
                blockers.insert(*reason);
            }
        }
    }

    for reason in [
        StandingTraceReason::RequiresVerifierContext,
        StandingTraceReason::CeilingIsCandidateOnly,
    ] {
        let relevant = trace
            .iter()
            .filter(|entry| entry.candidate.reasons.contains(&reason))
            .collect::<Vec<_>>();
        let has_unresolved = relevant.iter().any(|entry| !successful_application(entry));
        if has_unresolved || (relevant.is_empty() && baseline_blockers.contains(&reason)) {
            blockers.insert(reason);
        }
    }

    blockers.into_iter().collect()
}

fn successful_application(entry: &StandingTraceEntryV2) -> bool {
    entry.application.as_ref().is_some_and(|application| {
        matches!(
            application.achieved_standing,
            Some(Status::Supported | Status::Settled)
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exact_candidate() -> StandingTraceEntry {
        StandingTraceEntry {
            edge_id: Some("edge".into()),
            source_id: Some("evidence".into()),
            target_id: "claim".into(),
            evidence_kind: Some(EvidenceKind::DeterministicVerification.as_str().into()),
            claim_domain: Some(ClaimDomain::ExactMachineCheckable.as_str().into()),
            candidate_ceiling: Some(Status::Settled),
            reasons: vec![
                StandingTraceReason::AcceptedCandidate,
                StandingTraceReason::RequiresVerifierContext,
                StandingTraceReason::CeilingIsCandidateOnly,
            ],
        }
    }

    #[test]
    fn exact_candidate_selects_only_the_deterministic_rule() {
        assert_eq!(
            classify_deterministic_candidate(&exact_candidate()),
            Some(StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0)
        );
    }

    #[test]
    fn classifier_rejects_every_boundary_change() {
        let exact = exact_candidate();
        let mut cases = Vec::new();
        let mut value = exact.clone();
        value.evidence_kind = Some("DeadboltAnchor".into());
        cases.push(value);
        let mut value = exact.clone();
        value.claim_domain = Some("Interpretation".into());
        cases.push(value);
        let mut value = exact.clone();
        value.candidate_ceiling = Some(Status::Supported);
        cases.push(value);
        let mut value = exact.clone();
        value.edge_id = None;
        cases.push(value);
        let mut value = exact.clone();
        value.source_id = None;
        cases.push(value);
        let mut value = exact.clone();
        value.reasons.pop();
        cases.push(value);
        let mut value = exact.clone();
        value.reasons.swap(1, 2);
        cases.push(value);
        let mut value = exact;
        value.evidence_kind = Some("UnknownEvidenceKind".into());
        value.candidate_ceiling = None;
        value.reasons = vec![StandingTraceReason::UnknownEvidenceKind];
        cases.push(value);

        for candidate in cases {
            assert_eq!(classify_deterministic_candidate(&candidate), None);
        }
    }

    #[test]
    fn wrong_evidence_kind_has_no_v2_application() {
        let mut candidate = exact_candidate();
        candidate.evidence_kind = Some("DeadboltAnchor".into());
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn wrong_claim_domain_has_no_v2_application() {
        let mut candidate = exact_candidate();
        candidate.claim_domain = Some("Interpretation".into());
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn wrong_edge_kind_has_no_v2_application() {
        let mut candidate = exact_candidate();
        candidate.candidate_ceiling = None;
        candidate.reasons = vec![StandingTraceReason::UnsupportedEdgeKindForV0];
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn missing_source_id_has_no_v2_rule() {
        let mut candidate = exact_candidate();
        candidate.source_id = None;
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn altered_reason_set_has_no_v2_rule() {
        let mut candidate = exact_candidate();
        candidate.reasons.pop();
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn altered_reason_order_has_no_v2_rule() {
        let mut candidate = exact_candidate();
        candidate.reasons.swap(1, 2);
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn wrong_candidate_ceiling_has_no_v2_rule() {
        let mut candidate = exact_candidate();
        candidate.candidate_ceiling = Some(Status::Supported);
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn unknown_vocabulary_has_no_v2_rule() {
        let mut candidate = exact_candidate();
        candidate.evidence_kind = Some("UnknownEvidenceKind".into());
        candidate.candidate_ceiling = None;
        candidate.reasons = vec![StandingTraceReason::UnknownEvidenceKind];
        assert_eq!(classify_deterministic_candidate(&candidate), None);
    }

    #[test]
    fn combination_is_closed_and_non_amplifying() {
        assert_eq!(
            combine_v2_standing(Some(Status::Settled), true),
            Some(Status::Settled)
        );
        assert_eq!(
            combine_v2_standing(Some(Status::Refuted), true),
            Some(Status::Refuted)
        );
        assert_eq!(
            combine_v2_standing(Some(Status::Conjectured), true),
            Some(Status::Supported)
        );
        assert_eq!(
            combine_v2_standing(Some(Status::Supported), true),
            Some(Status::Supported)
        );
        assert_eq!(
            combine_v2_standing(Some(Status::Conjectured), false),
            Some(Status::Conjectured)
        );
        assert_eq!(combine_v2_standing(None, true), None);
    }

    #[test]
    fn candidate_revalidation_failure_is_fail_closed_and_serialized_exactly() {
        let application = StandingPolicyApplicationV2::deterministic(None);
        assert_eq!(application.achieved_standing, None);
        assert_eq!(
            serde_json::to_vec(&application).unwrap(),
            br#"{"rule":"sha256_bytes_equals_direct_support_v0","context":{"kind":"candidate_revalidation_failed"},"achieved_standing":null}"#
        );
    }

    #[test]
    fn rule_and_context_names_are_exact() {
        assert_eq!(
            serde_json::to_string(&StandingPolicyRuleV2::DeadboltOccurrenceInclusionV1).unwrap(),
            r#""deadbolt_occurrence_inclusion_v1""#
        );
        assert_eq!(
            serde_json::to_string(&StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0).unwrap(),
            r#""sha256_bytes_equals_direct_support_v0""#
        );
    }
}
