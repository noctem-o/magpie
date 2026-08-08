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
use crate::standing::{
    StandingCurrentness, StandingResolution, StandingTraceEntry, StandingTraceReason,
};
use crate::standing_v1::{
    DeadboltOccurrenceContextTrace, StandingPolicyApplication, StandingPolicyRule,
    StandingResolutionV1,
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

/// Closed failure vocabulary for policy-v2 composition invariants.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingResolutionFailureV2 {
    InheritedV1TraceLengthMismatch {
        v0_trace_len: usize,
        v1_trace_len: usize,
    },
    InheritedV1CandidateMismatch {
        index: usize,
    },
}

/// Deterministic governed-standing explanation under explicit policy v2.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingResolutionV2 {
    pub claim_id: String,
    pub governed_standing: Option<Status>,
    pub legacy_raw_standing: Option<Status>,
    pub currentness: StandingCurrentness,
    pub policy_id: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_failure: Option<StandingResolutionFailureV2>,
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
        resolve_standing_v2_from_resolutions(self, claim_id, baseline, inherited)
    }
}

fn resolve_standing_v2_from_resolutions(
    snapshot: &StandingReplaySnapshot,
    claim_id: &str,
    baseline: StandingResolution,
    inherited: StandingResolutionV1,
) -> StandingResolutionV2 {
    let v0_trace_len = baseline.trace.len();
    let v1_trace_len = inherited.trace.len();
    if v0_trace_len != v1_trace_len {
        return failed_v2_resolution(
            baseline,
            StandingResolutionFailureV2::InheritedV1TraceLengthMismatch {
                v0_trace_len,
                v1_trace_len,
            },
        );
    }

    for index in 0..baseline.trace.len() {
        if inherited.trace[index].candidate != baseline.trace[index] {
            return failed_v2_resolution(
                baseline,
                StandingResolutionFailureV2::InheritedV1CandidateMismatch { index },
            );
        }
    }

    let mut achieved_deterministic_supported = false;
    let trace = inherited
        .trace
        .into_iter()
        .map(|entry| {
            let application = match entry.application {
                Some(application) => {
                    let achieved = application.achieved_standing;
                    let converted = StandingPolicyApplicationV2::inherited_deadbolt(application);
                    debug_assert_eq!(converted.achieved_standing, achieved);
                    Some(converted)
                }
                None => classify_deterministic_candidate(&entry.candidate).map(|rule| {
                    debug_assert_eq!(rule, StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0);
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
                        snapshot.resolve_deterministic_verifier_context_v0(
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
        resolution_failure: None,
        trace,
        blockers,
    }
}

fn failed_v2_resolution(
    baseline: StandingResolution,
    resolution_failure: StandingResolutionFailureV2,
) -> StandingResolutionV2 {
    StandingResolutionV2 {
        claim_id: baseline.claim_id,
        governed_standing: baseline.governed_standing,
        legacy_raw_standing: baseline.legacy_raw_standing,
        currentness: baseline.currentness,
        policy_id: MAGPIE_CLAIMS_POLICY_V2_ID,
        resolution_failure: Some(resolution_failure),
        trace: baseline
            .trace
            .into_iter()
            .map(|candidate| StandingTraceEntryV2 {
                candidate,
                application: None,
            })
            .collect(),
        blockers: baseline.blockers,
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
    // Fail-closed status evolution (A-005/RQ-005): every current `Status`
    // variant is named explicitly, so adding a future variant stops
    // compilation here and forces an explicit policy decision. Only the
    // Boolean dimension may be wildcarded, once the status is fully selected.
    // V2 never synthesizes standing from an absent inherited claim.
    match (inherited, deterministic_supported) {
        (Some(Status::Settled), _) => Some(Status::Settled),
        (Some(Status::Refuted), _) => Some(Status::Refuted),
        (Some(Status::Supported), _) => Some(Status::Supported),
        (Some(Status::Open | Status::Conjectured), true) => Some(Status::Supported),
        (inherited @ Some(Status::Open | Status::Conjectured), false) => inherited,
        (None, _) => None,
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
    use crate::standing_v1::StandingTraceEntryV1;
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey};

    const MATCHED_CLAIM_ID: &str = "claim-machine";
    const MATCHED_SCOPE: &str = "scope:machine";
    const MATCHED_STATEMENT: &str =
        "sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    const MATCHED_CONTENT_HASH: &str =
        "70cea2ac9ed9d373b422ef3543e45ddaf289fa4e64a8aed726fc6b13df797421";
    const MATCHED_CLAIM_METADATA: &str = r#"{"claim_domain":"ExactMachineCheckable","machine_predicate":{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}"#;
    const MATCHED_WITNESS_METADATA: &str = r#"{"verification_witness":{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"claim-machine","scope_ref":"scope:machine","witness_hex":"616263"}}"#;

    fn test_snapshot() -> StandingReplaySnapshot {
        let store = MemStore::new();
        let key = SigningKey::from_bytes(&[101; 32]);
        {
            let _writer = LogWriter::open(store.clone(), key.clone()).unwrap();
        }
        let reader = LogReader::open(store, key.verifying_key());
        crate::replay_standing_context(&reader).unwrap()
    }

    fn matched_deterministic_snapshot() -> StandingReplaySnapshot {
        let store = MemStore::new();
        let key = SigningKey::from_bytes(&[92; 32]);
        {
            let mut writer = LogWriter::open(store.clone(), key.clone()).unwrap();
            let payloads = [
                Payload::ClaimAsserted {
                    claim_id: MATCHED_CLAIM_ID.into(),
                    statement: MATCHED_STATEMENT.into(),
                    status: Status::Conjectured,
                },
                Payload::ClaimAssertedV2 {
                    claim_id: MATCHED_CLAIM_ID.into(),
                    statement: MATCHED_STATEMENT.into(),
                    scope_ref: MATCHED_SCOPE.into(),
                    actor_class: "AgentProposer".into(),
                    content_hash: MATCHED_CONTENT_HASH.into(),
                    metadata_json: MATCHED_CLAIM_METADATA.into(),
                },
                Payload::EvidenceRegistered {
                    evidence_id: "evidence-a".into(),
                    evidence_kind: "DeterministicVerification".into(),
                    summary: "inline witness only".into(),
                    scope_ref: MATCHED_SCOPE.into(),
                    actor_class: "SourceImporter".into(),
                    content_hash: String::new(),
                    metadata_json: MATCHED_WITNESS_METADATA.into(),
                },
                Payload::JustificationEdgeRecorded {
                    edge_id: "edge-a".into(),
                    edge_kind: "supports".into(),
                    source_id: "evidence-a".into(),
                    target_id: MATCHED_CLAIM_ID.into(),
                    scope_ref: MATCHED_SCOPE.into(),
                    actor_class: "HumanRoot".into(),
                    rationale: "candidate direct support".into(),
                    metadata_json: "{}".into(),
                },
            ];
            for payload in payloads {
                writer
                    .append(Provenance::new("test", "standing-v2"), payload)
                    .unwrap();
            }
        }
        let reader = LogReader::open(store, key.verifying_key());
        crate::replay_standing_context(&reader).unwrap()
    }

    fn alignment_candidate(suffix: &str) -> StandingTraceEntry {
        StandingTraceEntry {
            edge_id: Some(format!("edge-{suffix}")),
            source_id: Some(format!("evidence-{suffix}")),
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

    fn alignment_baseline() -> StandingResolution {
        StandingResolution {
            claim_id: "claim".into(),
            governed_standing: Some(Status::Conjectured),
            legacy_raw_standing: Some(Status::Settled),
            currentness: StandingCurrentness::Unknown,
            policy_id: crate::MAGPIE_CLAIMS_POLICY_ID,
            trace: vec![alignment_candidate("a"), alignment_candidate("b")],
            blockers: vec![
                StandingTraceReason::RequiresVerifierContext,
                StandingTraceReason::CeilingIsCandidateOnly,
            ],
        }
    }

    fn fabricated_inherited(candidates: Vec<StandingTraceEntry>) -> StandingResolutionV1 {
        StandingResolutionV1 {
            claim_id: "claim".into(),
            governed_standing: Some(Status::Settled),
            legacy_raw_standing: Some(Status::Settled),
            currentness: StandingCurrentness::Unknown,
            policy_id: crate::MAGPIE_CLAIMS_POLICY_V1_ID,
            trace: candidates
                .into_iter()
                .map(|candidate| StandingTraceEntryV1 {
                    candidate,
                    application: Some(StandingPolicyApplication {
                        rule: StandingPolicyRule::DeadboltOccurrenceInclusionV1,
                        context: DeadboltOccurrenceContextTrace::AnchorNotFound,
                        achieved_standing: Some(Status::Settled),
                    }),
                })
                .collect(),
            blockers: Vec::new(),
        }
    }

    fn assert_alignment_failure(
        resolution: &StandingResolutionV2,
        baseline: &StandingResolution,
        expected: StandingResolutionFailureV2,
    ) {
        assert_eq!(resolution.claim_id, baseline.claim_id);
        assert_eq!(resolution.governed_standing, baseline.governed_standing);
        assert_eq!(resolution.legacy_raw_standing, baseline.legacy_raw_standing);
        assert_eq!(resolution.currentness, baseline.currentness);
        assert_eq!(resolution.policy_id, MAGPIE_CLAIMS_POLICY_V2_ID);
        assert_eq!(resolution.resolution_failure, Some(expected));
        assert_eq!(resolution.trace.len(), baseline.trace.len());
        assert!(resolution
            .trace
            .iter()
            .all(|entry| entry.application.is_none()));
        assert_eq!(
            resolution
                .trace
                .iter()
                .map(|entry| &entry.candidate)
                .collect::<Vec<_>>(),
            baseline.trace.iter().collect::<Vec<_>>()
        );
        assert_eq!(resolution.blockers, baseline.blockers);
        assert_ne!(resolution.governed_standing, Some(Status::Settled));
    }

    #[test]
    fn alignment_failure_short_inherited_trace_preserves_complete_v0_baseline() {
        let snapshot = test_snapshot();
        let baseline = alignment_baseline();
        let inherited = fabricated_inherited(vec![alignment_candidate("a")]);

        let resolution =
            resolve_standing_v2_from_resolutions(&snapshot, "claim", baseline.clone(), inherited);

        assert_alignment_failure(
            &resolution,
            &baseline,
            StandingResolutionFailureV2::InheritedV1TraceLengthMismatch {
                v0_trace_len: 2,
                v1_trace_len: 1,
            },
        );
    }

    #[test]
    fn alignment_failure_long_inherited_trace_preserves_complete_v0_baseline() {
        let snapshot = test_snapshot();
        let baseline = alignment_baseline();
        let inherited = fabricated_inherited(vec![
            alignment_candidate("a"),
            alignment_candidate("b"),
            alignment_candidate("c"),
        ]);

        let resolution =
            resolve_standing_v2_from_resolutions(&snapshot, "claim", baseline.clone(), inherited);

        assert_alignment_failure(
            &resolution,
            &baseline,
            StandingResolutionFailureV2::InheritedV1TraceLengthMismatch {
                v0_trace_len: 2,
                v1_trace_len: 3,
            },
        );
    }

    #[test]
    fn alignment_failure_reports_first_candidate_mismatch_and_discards_applications() {
        let snapshot = test_snapshot();
        let baseline = alignment_baseline();
        let mut changed = alignment_candidate("b");
        changed.source_id = Some("standing-bearing-source-mismatch".into());
        let inherited = fabricated_inherited(vec![alignment_candidate("a"), changed]);

        let resolution =
            resolve_standing_v2_from_resolutions(&snapshot, "claim", baseline.clone(), inherited);

        assert_alignment_failure(
            &resolution,
            &baseline,
            StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 1 },
        );
    }

    #[test]
    fn alignment_failure_serialization_is_literal_and_deterministic() {
        let snapshot = test_snapshot();
        let baseline = alignment_baseline();
        let inherited = fabricated_inherited(vec![alignment_candidate("a")]);

        let resolution =
            resolve_standing_v2_from_resolutions(&snapshot, "claim", baseline, inherited);

        assert_eq!(
            String::from_utf8(resolution.canonical_bytes()).unwrap(),
            r#"{"claim_id":"claim","governed_standing":"Conjectured","legacy_raw_standing":"Settled","currentness":"unknown","policy_id":"magpie-claims-standing-v2","resolution_failure":{"kind":"inherited_v1_trace_length_mismatch","v0_trace_len":2,"v1_trace_len":1},"trace":[{"candidate":{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":null},{"candidate":{"edge_id":"edge-b","source_id":"evidence-b","target_id":"claim","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":null}],"blockers":["requires_verifier_context","ceiling_is_candidate_only"]}"#
        );
    }

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
    fn inherited_governed_refuted_overrides_matched_deterministic_support() {
        let snapshot = matched_deterministic_snapshot();
        let baseline = snapshot
            .standing()
            .resolved_standing_with_trace(MATCHED_CLAIM_ID);
        let mut inherited = snapshot.resolved_standing_with_trace_v1(MATCHED_CLAIM_ID);
        assert_eq!(baseline.governed_standing, Some(Status::Conjectured));
        assert_eq!(baseline.legacy_raw_standing, Some(Status::Conjectured));
        assert_eq!(inherited.governed_standing, Some(Status::Conjectured));
        assert_eq!(inherited.legacy_raw_standing, Some(Status::Conjectured));

        // No currently landed public policy derives governed Refuted.
        // This test pins v2 inheritance precedence without inventing
        // a new authority-producing public surface.
        inherited.governed_standing = Some(Status::Refuted);

        let resolution = resolve_standing_v2_from_resolutions(
            &snapshot,
            MATCHED_CLAIM_ID,
            baseline.clone(),
            inherited.clone(),
        );
        let repeated =
            resolve_standing_v2_from_resolutions(&snapshot, MATCHED_CLAIM_ID, baseline, inherited);

        assert!(resolution.resolution_failure.is_none());
        assert_eq!(resolution.governed_standing, Some(Status::Refuted));
        assert_ne!(resolution.governed_standing, Some(Status::Supported));
        assert_ne!(resolution.governed_standing, Some(Status::Settled));
        assert_eq!(resolution.legacy_raw_standing, Some(Status::Conjectured));
        assert!(resolution.blockers.is_empty());
        assert_eq!(resolution.trace, repeated.trace);
        assert_eq!(resolution.blockers, repeated.blockers);

        let application = resolution
            .trace
            .iter()
            .filter_map(|entry| entry.application.as_ref())
            .find(|application| {
                application.rule == StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0
            })
            .expect("deterministic candidate must be evaluated");
        assert_eq!(application.achieved_standing, Some(Status::Supported));
        let StandingPolicyContextV2::DeterministicVerifierV0 {
            trace: DeterministicVerifierContextTraceV0::Matched(receipt),
        } = &application.context
        else {
            panic!("application must contain the replay-derived matched receipt")
        };
        assert_eq!(receipt.claim_id(), MATCHED_CLAIM_ID);
        assert_eq!(receipt.evidence_id(), "evidence-a");
        assert_eq!(receipt.edge_id(), "edge-a");
        assert_eq!(receipt.scope_ref(), MATCHED_SCOPE);
        assert_eq!(receipt.canonical_statement(), MATCHED_STATEMENT);
        assert_eq!(receipt.claim_content_hash(), MATCHED_CONTENT_HASH);
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
    fn combination_truth_table_is_exact_for_every_current_status() {
        // Compatibility law: the complete 12-case
        // `Option<Status> x deterministic_supported` matrix. Every current
        // variant appears explicitly; a future variant is a compile error in
        // `combine_v2_standing`, never a silent promotion.
        let cases = [
            (None, false, None),
            (None, true, None),
            (Some(Status::Open), false, Some(Status::Open)),
            (Some(Status::Open), true, Some(Status::Supported)),
            (Some(Status::Conjectured), false, Some(Status::Conjectured)),
            (Some(Status::Conjectured), true, Some(Status::Supported)),
            (Some(Status::Supported), false, Some(Status::Supported)),
            (Some(Status::Supported), true, Some(Status::Supported)),
            (Some(Status::Settled), false, Some(Status::Settled)),
            (Some(Status::Settled), true, Some(Status::Settled)),
            (Some(Status::Refuted), false, Some(Status::Refuted)),
            (Some(Status::Refuted), true, Some(Status::Refuted)),
        ];
        for (inherited, deterministic_supported, expected) in cases {
            assert_eq!(
                combine_v2_standing(inherited, deterministic_supported),
                expected,
                "v2 combination changed for inherited={inherited:?} deterministic_supported={deterministic_supported}"
            );
        }
    }

    #[test]
    fn combination_dominance_and_none_laws_are_exact() {
        for deterministic_supported in [false, true] {
            // Settlement remains dominant over deterministic support.
            assert_eq!(
                combine_v2_standing(Some(Status::Settled), deterministic_supported),
                Some(Status::Settled)
            );
            // Governed refutation remains dominant over deterministic support.
            assert_eq!(
                combine_v2_standing(Some(Status::Refuted), deterministic_supported),
                Some(Status::Refuted)
            );
            // Existing support is preserved with either Boolean value.
            assert_eq!(
                combine_v2_standing(Some(Status::Supported), deterministic_supported),
                Some(Status::Supported)
            );
            // V2 never synthesizes standing from an absent inherited claim.
            assert_eq!(combine_v2_standing(None, deterministic_supported), None);
        }
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
