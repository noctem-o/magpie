//! Standing-inert complete origin-admission audit v0.
//!
//! Candidate audits cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionCandidateAuditV0;
//! let _audit = OriginAdmissionCandidateAuditV0 {};
//! ```
//!
//! Admitted origins cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedOriginV0;
//! let _origin = AdmittedOriginV0 {};
//! ```
//!
//! Conflicts cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::OriginBindingConflictV0;
//! let _conflict = OriginBindingConflictV0 {};
//! ```
//!
//! Complete audits cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionAuditV0;
//! let _audit = OriginAdmissionAuditV0 {};
//! ```
//!
//! Audit completion values cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionAuditCompletionV0;
//! let _: OriginAdmissionAuditCompletionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Candidate dispositions cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionCandidateDispositionV0;
//! let _: OriginAdmissionCandidateDispositionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Candidate audits cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionCandidateAuditV0;
//! let _: OriginAdmissionCandidateAuditV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Admitted origins cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedOriginV0;
//! let _: AdmittedOriginV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Conflicts cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginBindingConflictV0;
//! let _: OriginBindingConflictV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Decisions cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionDecisionV0;
//! let _: OriginAdmissionDecisionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Complete audits cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionAuditV0;
//! let _: OriginAdmissionAuditV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! No resolver accepts a caller-selected candidate list:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ArtifactProvenanceAnchorSelectorV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     selectors: &[ArtifactProvenanceAnchorSelectorV0],
//! ) {
//!     let _ = context.resolve_origin_admission_audit_v0(closure, selectors);
//! }
//! ```
//!
//! No resolver accepts a caller-created anchor index:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     DeadboltAnchorIndex, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     anchors: &DeadboltAnchorIndex,
//! ) {
//!     let _ = context.resolve_origin_admission_audit_v0(closure, anchors);
//! }
//! ```
//!
//! No resolver accepts a caller-selected policy ID:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     policy_id: &str,
//! ) {
//!     let _ = context.resolve_origin_admission_audit_v0(closure, policy_id);
//! }
//! ```
//!
//! No resolver accepts a caller-created authority table:
//!
//! ```compile_fail
//! use std::collections::BTreeMap;
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     authorities: &BTreeMap<String, bool>,
//! ) {
//!     let _ = context.resolve_origin_admission_audit_v0(closure, authorities);
//! }
//! ```
//!
//! Serialized or cloned audits cannot substitute for replay and closure:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionAuditV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     audit: OriginAdmissionAuditV0,
//! ) {
//!     let serialized = serde_json::to_vec(&audit).unwrap();
//!     let _ = context.resolve_origin_admission_audit_v0(
//!         closure,
//!         audit.clone(),
//!         serialized,
//!     );
//! }
//! ```
//!
//! Admitted origins, conflicts and decisions cannot substitute for authority:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     AdmittedOriginV0, OriginAdmissionDecisionV0,
//!     OriginAdmissionReplayContextV0, OriginBindingConflictV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     admitted: AdmittedOriginV0,
//!     conflict: OriginBindingConflictV0,
//!     decision: OriginAdmissionDecisionV0,
//! ) {
//!     let _ = context.resolve_origin_admission_audit_v0(
//!         closure,
//!         admitted,
//!         conflict,
//!         decision,
//!     );
//! }
//! ```
//!
//! A standalone standing snapshot has no complete audit resolver:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureV0, StandingReplaySnapshot,
//! };
//! fn resolve(
//!     snapshot: &StandingReplaySnapshot,
//!     closure: &ResolutionContentClosureV0,
//! ) {
//!     let _ = snapshot.resolve_origin_admission_audit_v0(closure);
//! }
//! ```
//!
//! Other public replay-derived views are not complete audit carriers:
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingView};
//! fn resolve(view: &StandingView, closure: &ResolutionContentClosureV0) {
//!     let _ = view.resolve_origin_admission_audit_v0(closure);
//! }
//! ```

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
use crate::deadbolt_context::DeadboltAnchorOccurrence;
use crate::origin_admission_replay::OriginAdmissionReplayContextV0;
use crate::origin_binding_verifier::{
    evaluate_origin_binding_context_v0, ContributionIdentityV0, OriginBindingContextTraceV0,
    OriginBindingEvaluationClassV0, OriginComparisonNamespaceV0, ValidatedOriginBindingSubjectV0,
};
use crate::resolution_content_closure::{
    ResolutionContentClosureIdentityV0, ResolutionContentClosureV0,
};
use crate::VerifiedLogPrefixIdentityV0;

pub const ORIGIN_ADMISSION_AUDIT_SCHEMA_V0: &str = "magpie-origin-admission-audit-v0";

pub const ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-origin-admission-audit-json-v0";

pub const ORIGIN_ADMISSION_POLICY_ID_V0: &str = "magpie-origin-admission-v0";

pub const ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0: &str = "human-review";

pub const ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0: &str = "authority:origin-review-v0";

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum OriginAdmissionAuditCompletionV0 {
    Complete,

    IncompleteCandidateUniverse {
        unavailable_binding_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum OriginAdmissionCandidateDispositionV0 {
    BindingBytesUnavailable,

    DefinitivelyRejected,

    PolicyMismatch {
        claimed_policy_id: String,
    },

    AuthorityUntrusted {
        authority_kind: String,
        authority_reference: String,
    },

    TrustedEligibleUnresolved {
        claimed_origin_group: String,
    },

    TrustedMatched {
        origin_group: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OriginAdmissionCandidateAuditV0 {
    selector: ArtifactProvenanceAnchorSelectorV0,
    anchor_occurrences: Vec<DeadboltAnchorOccurrence>,
    trace: OriginBindingContextTraceV0,
    disposition: OriginAdmissionCandidateDispositionV0,
    validated_contribution: Option<ContributionIdentityV0>,
    validated_namespace: Option<OriginComparisonNamespaceV0>,
}

impl OriginAdmissionCandidateAuditV0 {
    pub fn selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.selector
    }

    pub fn anchor_occurrences(&self) -> &[DeadboltAnchorOccurrence] {
        &self.anchor_occurrences
    }

    pub fn trace(&self) -> &OriginBindingContextTraceV0 {
        &self.trace
    }

    pub fn disposition(&self) -> &OriginAdmissionCandidateDispositionV0 {
        &self.disposition
    }

    pub fn validated_contribution(&self) -> Option<&ContributionIdentityV0> {
        self.validated_contribution.as_ref()
    }

    pub fn validated_namespace(&self) -> Option<&OriginComparisonNamespaceV0> {
        self.validated_namespace.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AdmittedOriginV0 {
    policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    supporting_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
}

impl AdmittedOriginV0 {
    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn contribution(&self) -> &ContributionIdentityV0 {
        &self.contribution
    }

    pub fn namespace(&self) -> &OriginComparisonNamespaceV0 {
        &self.namespace
    }

    pub fn origin_group(&self) -> &str {
        &self.origin_group
    }

    pub fn supporting_candidate_selectors(&self) -> &[ArtifactProvenanceAnchorSelectorV0] {
        &self.supporting_candidate_selectors
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OriginBindingConflictV0 {
    policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_groups: Vec<String>,
    matched_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    unresolved_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
}

impl OriginBindingConflictV0 {
    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn contribution(&self) -> &ContributionIdentityV0 {
        &self.contribution
    }

    pub fn namespace(&self) -> &OriginComparisonNamespaceV0 {
        &self.namespace
    }

    pub fn origin_groups(&self) -> &[String] {
        &self.origin_groups
    }

    pub fn matched_candidate_selectors(&self) -> &[ArtifactProvenanceAnchorSelectorV0] {
        &self.matched_candidate_selectors
    }

    pub fn unresolved_candidate_selectors(&self) -> &[ArtifactProvenanceAnchorSelectorV0] {
        &self.unresolved_candidate_selectors
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum OriginAdmissionDecisionV0 {
    ConflictingBindings {
        conflict: OriginBindingConflictV0,
    },

    EligibleBindingUnresolved {
        policy_id: String,
        contribution: ContributionIdentityV0,
        namespace: OriginComparisonNamespaceV0,
        matched_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
        unresolved_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    },

    OriginAdmitted {
        admitted_origin: AdmittedOriginV0,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OriginAdmissionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: OriginAdmissionAuditCompletionV0,
    candidate_audits: Vec<OriginAdmissionCandidateAuditV0>,
    contribution_decisions: Vec<OriginAdmissionDecisionV0>,
}

impl OriginAdmissionAuditV0 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn canonicalization_profile(&self) -> &str {
        &self.canonicalization_profile
    }

    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn verified_prefix_identity(&self) -> &VerifiedLogPrefixIdentityV0 {
        &self.verified_prefix_identity
    }

    pub fn closure_identity(&self) -> &ResolutionContentClosureIdentityV0 {
        &self.closure_identity
    }

    pub fn completion(&self) -> &OriginAdmissionAuditCompletionV0 {
        &self.completion
    }

    pub fn candidate_audits(&self) -> &[OriginAdmissionCandidateAuditV0] {
        &self.candidate_audits
    }

    pub fn contribution_decisions(&self) -> &[OriginAdmissionDecisionV0] {
        &self.contribution_decisions
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("origin-admission audits are always serializable")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ContributionAdmissionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}

enum TrustedFoldInputV0 {
    Matched {
        key: ContributionAdmissionKeyV0,
        origin_group: String,
        selector: ArtifactProvenanceAnchorSelectorV0,
    },
    Unresolved {
        key: ContributionAdmissionKeyV0,
        selector: ArtifactProvenanceAnchorSelectorV0,
    },
}

#[derive(Default)]
struct ContributionAdmissionAccumulatorV0 {
    matched_by_group: BTreeMap<String, BTreeSet<ArtifactProvenanceAnchorSelectorV0>>,
    unresolved_selectors: BTreeSet<ArtifactProvenanceAnchorSelectorV0>,
}

pub(crate) fn resolve_origin_admission_audit_v0(
    context: &OriginAdmissionReplayContextV0,
    closure: &ResolutionContentClosureV0,
) -> OriginAdmissionAuditV0 {
    let candidates = context.origin_binding_candidates_v0();
    let mut unavailable_binding_selectors = Vec::new();
    let mut candidate_audits = Vec::with_capacity(candidates.len());
    let mut trusted_fold_inputs = Vec::new();

    for candidate in candidates {
        let selector = candidate.selector().clone();
        let anchor_occurrences = candidate.occurrences().to_vec();
        let evaluation = evaluate_origin_binding_context_v0(context.snapshot(), &selector, closure);
        let (trace, class, validated_subject) = evaluation.into_parts();

        if class == OriginBindingEvaluationClassV0::BindingBytesUnavailable {
            unavailable_binding_selectors.push(selector.clone());
        }

        let validated_contribution = validated_subject
            .as_ref()
            .map(|subject| subject.contribution().clone());
        let validated_namespace = validated_subject
            .as_ref()
            .map(|subject| subject.namespace().clone());
        let (disposition, fold_input) =
            classify_candidate(&selector, class, validated_subject.as_ref());
        if let Some(fold_input) = fold_input {
            trusted_fold_inputs.push(fold_input);
        }

        candidate_audits.push(OriginAdmissionCandidateAuditV0 {
            selector,
            anchor_occurrences,
            trace,
            disposition,
            validated_contribution,
            validated_namespace,
        });
    }

    let (completion, contribution_decisions) = if unavailable_binding_selectors.is_empty() {
        (
            OriginAdmissionAuditCompletionV0::Complete,
            fold_trusted_inputs(trusted_fold_inputs),
        )
    } else {
        (
            OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
                unavailable_binding_selectors,
            },
            Vec::new(),
        )
    };

    OriginAdmissionAuditV0 {
        schema: ORIGIN_ADMISSION_AUDIT_SCHEMA_V0.to_owned(),
        canonicalization_profile: ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0.to_owned(),
        policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
        verified_prefix_identity: context.verified_prefix_identity().clone(),
        closure_identity: closure.identity().clone(),
        completion,
        candidate_audits,
        contribution_decisions,
    }
}

fn classify_candidate(
    selector: &ArtifactProvenanceAnchorSelectorV0,
    class: OriginBindingEvaluationClassV0,
    validated_subject: Option<&ValidatedOriginBindingSubjectV0>,
) -> (
    OriginAdmissionCandidateDispositionV0,
    Option<TrustedFoldInputV0>,
) {
    match class {
        OriginBindingEvaluationClassV0::BindingBytesUnavailable => {
            assert!(validated_subject.is_none());
            (
                OriginAdmissionCandidateDispositionV0::BindingBytesUnavailable,
                None,
            )
        }
        OriginBindingEvaluationClassV0::DefinitivelyRejected => (
            OriginAdmissionCandidateDispositionV0::DefinitivelyRejected,
            None,
        ),
        OriginBindingEvaluationClassV0::AvailabilityOnly
        | OriginBindingEvaluationClassV0::Matched => {
            let subject = validated_subject
                .expect("late origin-binding evaluations always retain a validated subject");
            if subject.claimed_policy_id() != ORIGIN_ADMISSION_POLICY_ID_V0 {
                return (
                    OriginAdmissionCandidateDispositionV0::PolicyMismatch {
                        claimed_policy_id: subject.claimed_policy_id().to_owned(),
                    },
                    None,
                );
            }
            if subject.authority_kind() != ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0
                || subject.authority_reference() != ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0
            {
                return (
                    OriginAdmissionCandidateDispositionV0::AuthorityUntrusted {
                        authority_kind: subject.authority_kind().to_owned(),
                        authority_reference: subject.authority_reference().to_owned(),
                    },
                    None,
                );
            }

            let key = ContributionAdmissionKeyV0 {
                contribution: subject.contribution().clone(),
                namespace: subject.namespace().clone(),
            };
            match class {
                OriginBindingEvaluationClassV0::AvailabilityOnly => (
                    OriginAdmissionCandidateDispositionV0::TrustedEligibleUnresolved {
                        claimed_origin_group: subject.claimed_origin_group().to_owned(),
                    },
                    Some(TrustedFoldInputV0::Unresolved {
                        key,
                        selector: selector.clone(),
                    }),
                ),
                OriginBindingEvaluationClassV0::Matched => (
                    OriginAdmissionCandidateDispositionV0::TrustedMatched {
                        origin_group: subject.claimed_origin_group().to_owned(),
                    },
                    Some(TrustedFoldInputV0::Matched {
                        key,
                        origin_group: subject.claimed_origin_group().to_owned(),
                        selector: selector.clone(),
                    }),
                ),
                OriginBindingEvaluationClassV0::BindingBytesUnavailable
                | OriginBindingEvaluationClassV0::DefinitivelyRejected => unreachable!(),
            }
        }
    }
}

fn fold_trusted_inputs(inputs: Vec<TrustedFoldInputV0>) -> Vec<OriginAdmissionDecisionV0> {
    let mut accumulators: BTreeMap<ContributionAdmissionKeyV0, ContributionAdmissionAccumulatorV0> =
        BTreeMap::new();

    for input in inputs {
        match input {
            TrustedFoldInputV0::Matched {
                key,
                origin_group,
                selector,
            } => {
                accumulators
                    .entry(key)
                    .or_default()
                    .matched_by_group
                    .entry(origin_group)
                    .or_default()
                    .insert(selector);
            }
            TrustedFoldInputV0::Unresolved { key, selector } => {
                accumulators
                    .entry(key)
                    .or_default()
                    .unresolved_selectors
                    .insert(selector);
            }
        }
    }

    let mut decisions = Vec::with_capacity(accumulators.len());
    for (key, accumulator) in accumulators {
        let matched_group_count = accumulator.matched_by_group.len();
        if matched_group_count >= 2 {
            let origin_groups = accumulator
                .matched_by_group
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            let matched_candidate_selectors = accumulator
                .matched_by_group
                .values()
                .flat_map(|selectors| selectors.iter().cloned())
                .collect();
            let unresolved_candidate_selectors =
                accumulator.unresolved_selectors.into_iter().collect();
            decisions.push(OriginAdmissionDecisionV0::ConflictingBindings {
                conflict: OriginBindingConflictV0 {
                    policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                    contribution: key.contribution,
                    namespace: key.namespace,
                    origin_groups,
                    matched_candidate_selectors,
                    unresolved_candidate_selectors,
                },
            });
            continue;
        }

        if !accumulator.unresolved_selectors.is_empty() {
            let matched_candidate_selectors = accumulator
                .matched_by_group
                .values()
                .flat_map(|selectors| selectors.iter().cloned())
                .collect();
            let unresolved_candidate_selectors =
                accumulator.unresolved_selectors.into_iter().collect();
            decisions.push(OriginAdmissionDecisionV0::EligibleBindingUnresolved {
                policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                contribution: key.contribution,
                namespace: key.namespace,
                matched_candidate_selectors,
                unresolved_candidate_selectors,
            });
            continue;
        }

        if let Some((origin_group, selectors)) = accumulator.matched_by_group.into_iter().next() {
            decisions.push(OriginAdmissionDecisionV0::OriginAdmitted {
                admitted_origin: AdmittedOriginV0 {
                    policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                    contribution: key.contribution,
                    namespace: key.namespace,
                    origin_group,
                    supporting_candidate_selectors: selectors.into_iter().collect(),
                },
            });
        }
    }
    decisions
}
