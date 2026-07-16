//! Standing-inert admitted-contribution audit v0.
//!
//! Candidate audits cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionCandidateAuditV0;
//! let _candidate = AdmittedContributionCandidateAuditV0 {};
//! ```
//!
//! Admitted contributions cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionV0;
//! let _contribution = AdmittedContributionV0 {};
//! ```
//!
//! Complete audits cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionAuditV0;
//! let _audit = AdmittedContributionAuditV0 {};
//! ```
//!
//! Completion values cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionAuditCompletionV0;
//! let _: AdmittedContributionAuditCompletionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Candidate reasons cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionCandidateReasonV0;
//! let _: AdmittedContributionCandidateReasonV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Candidate dispositions cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionCandidateDispositionV0;
//! let _: AdmittedContributionCandidateDispositionV0 =
//!     serde_json::from_str("{}").unwrap();
//! ```
//!
//! Candidate audits cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionCandidateAuditV0;
//! let _: AdmittedContributionCandidateAuditV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Admitted contributions cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionV0;
//! let _: AdmittedContributionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Complete audits cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::AdmittedContributionAuditV0;
//! let _: AdmittedContributionAuditV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! No resolver accepts a caller-selected edge list:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     edge_ids: &[String],
//! ) {
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, edge_ids);
//! }
//! ```
//!
//! No resolver accepts a caller-created origin-admission audit:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionAuditV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     origin_audit: OriginAdmissionAuditV0,
//! ) {
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, origin_audit);
//! }
//! ```
//!
//! No resolver accepts a caller-created origin-admission decision:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionDecisionV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     decision: OriginAdmissionDecisionV0,
//! ) {
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, decision);
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
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, policy_id);
//! }
//! ```
//!
//! No resolver accepts a caller-created snapshot:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     StandingReplaySnapshot,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     snapshot: &StandingReplaySnapshot,
//! ) {
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, snapshot);
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
//!     let _ = context.resolve_admitted_contribution_audit_v0(closure, anchors);
//! }
//! ```
//!
//! Serialized or cloned audit output cannot substitute for replay authority:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     AdmittedContributionAuditV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     audit: AdmittedContributionAuditV0,
//! ) {
//!     let bytes = audit.canonical_bytes();
//!     let _ = context.resolve_admitted_contribution_audit_v0(
//!         closure,
//!         audit.clone(),
//!         bytes,
//!     );
//! }
//! ```
//!
//! A standalone standing view has no admitted-contribution resolver:
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingView};
//! fn resolve(view: &StandingView, closure: &ResolutionContentClosureV0) {
//!     let _ = view.resolve_admitted_contribution_audit_v0(closure);
//! }
//! ```
//!
//! A standalone standing replay snapshot has no complete resolver:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureV0, StandingReplaySnapshot,
//! };
//! fn resolve(
//!     snapshot: &StandingReplaySnapshot,
//!     closure: &ResolutionContentClosureV0,
//! ) {
//!     let _ = snapshot.resolve_admitted_contribution_audit_v0(closure);
//! }
//! ```
//!
//! Contribution identities still have no public constructor:
//!
//! ```compile_fail
//! use magpie_claims::ContributionIdentityV0;
//! let _ = ContributionIdentityV0::from_admitted_contribution_graph_v0(
//!     "claim", "evidence", "edge", "scope", "digest",
//! );
//! ```
//!
//! Origin-comparison namespaces still have no public constructor:
//!
//! ```compile_fail
//! use magpie_claims::OriginComparisonNamespaceV0;
//! let _ = OriginComparisonNamespaceV0::for_admitted_contribution_v0(
//!     "claim", "scope",
//! );
//! ```

use std::collections::BTreeMap;

use magpie_log::Status;
use serde::Serialize;

use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
use crate::origin_admission_audit::{
    OriginAdmissionAuditCompletionV0, OriginAdmissionDecisionV0, ORIGIN_ADMISSION_POLICY_ID_V0,
};
use crate::origin_admission_replay::{OriginAdmissionReplayContextV0, VerifiedLogPrefixIdentityV0};
use crate::origin_binding_verifier::{ContributionIdentityV0, OriginComparisonNamespaceV0};
use crate::policy::{support_ceiling, ClaimDomain, EvidenceKind};
use crate::resolution_content_closure::{
    ResolutionContentClosureIdentityV0, ResolutionContentClosureV0,
};
use crate::standing::{SupportCandidateStructureEvaluationV0, SupportCandidateStructureFailureV0};

pub const ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0: &str = "magpie-admitted-contribution-audit-v0";

pub const ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-admitted-contribution-audit-json-v0";

pub const ADMITTED_CONTRIBUTION_POLICY_ID_V0: &str = "magpie-admitted-contribution-v0";

pub const ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0: &str = "sha256";

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum AdmittedContributionAuditCompletionV0 {
    Complete,

    OriginAdmissionIncomplete {
        unavailable_binding_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum AdmittedContributionCandidateReasonV0 {
    MissingTargetClaim,
    MissingTypedTargetClaim,
    UnsupportedEdgeKindForV0,
    MalformedClaimMetadata,
    MissingClaimDomain,
    WrongTypeClaimDomain,
    DuplicateClaimDomainKey,
    UnknownClaimDomain,
    MissingSourceEvidence,
    ScopeMismatch,
    UnknownEvidenceKind,
    UnsupportedEvidenceKindForV0,
    UnsupportedClaimDomainForV0,
    NoSupportCeiling,
    CandidateCeilingMismatch,
    MissingEvidenceContentHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum AdmittedContributionCandidateDispositionV0 {
    PolicyIneligible {
        reason: AdmittedContributionCandidateReasonV0,
    },

    OriginAdmissionIncomplete,

    OriginDecisionAbsent,

    OriginNotAdmitted {
        decision: OriginAdmissionDecisionV0,
    },

    Admitted {
        admitted_contribution: AdmittedContributionV0,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AdmittedContributionCandidateAuditV0 {
    edge_id: String,
    source_evidence_id: String,
    target_claim_id: String,
    evidence_kind: Option<String>,
    claim_domain: Option<String>,
    candidate_ceiling: Option<Status>,
    candidate_contribution: Option<ContributionIdentityV0>,
    candidate_namespace: Option<OriginComparisonNamespaceV0>,
    disposition: AdmittedContributionCandidateDispositionV0,
}

impl AdmittedContributionCandidateAuditV0 {
    pub fn edge_id(&self) -> &str {
        &self.edge_id
    }

    pub fn source_evidence_id(&self) -> &str {
        &self.source_evidence_id
    }

    pub fn target_claim_id(&self) -> &str {
        &self.target_claim_id
    }

    pub fn evidence_kind(&self) -> Option<&str> {
        self.evidence_kind.as_deref()
    }

    pub fn claim_domain(&self) -> Option<&str> {
        self.claim_domain.as_deref()
    }

    pub fn candidate_ceiling(&self) -> Option<Status> {
        self.candidate_ceiling
    }

    pub fn candidate_contribution(&self) -> Option<&ContributionIdentityV0> {
        self.candidate_contribution.as_ref()
    }

    pub fn candidate_namespace(&self) -> Option<&OriginComparisonNamespaceV0> {
        self.candidate_namespace.as_ref()
    }

    pub fn disposition(&self) -> &AdmittedContributionCandidateDispositionV0 {
        &self.disposition
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AdmittedContributionV0 {
    policy_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    evidence_kind: String,
    claim_domain: String,
    candidate_ceiling: Status,
    supporting_origin_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
}

impl AdmittedContributionV0 {
    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn origin_admission_policy_id(&self) -> &str {
        &self.origin_admission_policy_id
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

    pub fn evidence_kind(&self) -> &str {
        &self.evidence_kind
    }

    pub fn claim_domain(&self) -> &str {
        &self.claim_domain
    }

    pub fn candidate_ceiling(&self) -> Status {
        self.candidate_ceiling
    }

    pub fn supporting_origin_candidate_selectors(&self) -> &[ArtifactProvenanceAnchorSelectorV0] {
        &self.supporting_origin_candidate_selectors
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AdmittedContributionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: AdmittedContributionAuditCompletionV0,
    candidate_audits: Vec<AdmittedContributionCandidateAuditV0>,
    admitted_contributions: Vec<AdmittedContributionV0>,
}

impl AdmittedContributionAuditV0 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn canonicalization_profile(&self) -> &str {
        &self.canonicalization_profile
    }

    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn origin_admission_policy_id(&self) -> &str {
        &self.origin_admission_policy_id
    }

    pub fn verified_prefix_identity(&self) -> &VerifiedLogPrefixIdentityV0 {
        &self.verified_prefix_identity
    }

    pub fn closure_identity(&self) -> &ResolutionContentClosureIdentityV0 {
        &self.closure_identity
    }

    pub fn completion(&self) -> &AdmittedContributionAuditCompletionV0 {
        &self.completion
    }

    pub fn candidate_audits(&self) -> &[AdmittedContributionCandidateAuditV0] {
        &self.candidate_audits
    }

    pub fn admitted_contributions(&self) -> &[AdmittedContributionV0] {
        &self.admitted_contributions
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("admitted-contribution audits are always serializable")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OriginDecisionKeyV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
}

pub(crate) fn resolve_admitted_contribution_audit_v0(
    context: &OriginAdmissionReplayContextV0,
    closure: &ResolutionContentClosureV0,
) -> AdmittedContributionAuditV0 {
    let origin_audit = context.resolve_origin_admission_audit_v0(closure);
    assert_eq!(
        origin_audit.verified_prefix_identity(),
        context.verified_prefix_identity()
    );
    assert_eq!(origin_audit.closure_identity(), closure.identity());
    assert_eq!(origin_audit.policy_id(), ORIGIN_ADMISSION_POLICY_ID_V0);
    assert_eq!(ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0, "sha256");

    let (completion, origin_complete) = match origin_audit.completion() {
        OriginAdmissionAuditCompletionV0::Complete => {
            (AdmittedContributionAuditCompletionV0::Complete, true)
        }
        OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
            unavailable_binding_selectors,
        } => (
            AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
                unavailable_binding_selectors: unavailable_binding_selectors.clone(),
            },
            false,
        ),
    };
    let decision_index = if origin_complete {
        build_origin_decision_index(&origin_audit)
    } else {
        BTreeMap::new()
    };

    let standing = context.snapshot().standing();
    let mut candidate_audits = Vec::with_capacity(standing.justification_edges.len());
    let mut admitted_contributions = BTreeMap::new();

    for (edge_id, edge) in &standing.justification_edges {
        let evaluation = standing.evaluate_support_candidate_structure_v0(&edge.target_id, edge);
        let evidence_kind = staged_evidence_kind(&evaluation);
        let claim_domain = staged_claim_domain(&evaluation);
        let mut candidate_ceiling = None;
        let mut candidate_contribution = None;
        let mut candidate_namespace = None;

        let disposition = if let Some(failure) = evaluation.failure() {
            policy_ineligible(structure_reason(failure))
        } else {
            let evidence = evaluation
                .evidence()
                .expect("successful structural evaluation has evidence");
            let kind = evaluation
                .evidence_kind()
                .expect("successful structural evaluation has an evidence kind");
            let domain = evaluation
                .claim_domain()
                .expect("successful structural evaluation has a claim domain");

            if kind != EvidenceKind::ExternalSource {
                policy_ineligible(
                    AdmittedContributionCandidateReasonV0::UnsupportedEvidenceKindForV0,
                )
            } else if domain != ClaimDomain::ExternalReport {
                policy_ineligible(
                    AdmittedContributionCandidateReasonV0::UnsupportedClaimDomainForV0,
                )
            } else {
                let selected_ceiling = support_ceiling(kind, domain);
                candidate_ceiling = selected_ceiling;
                match exact_supported_ceiling(selected_ceiling) {
                    Err(reason) => policy_ineligible(reason),
                    Ok(_ceiling) if evidence.content_hash.is_empty() => policy_ineligible(
                        AdmittedContributionCandidateReasonV0::MissingEvidenceContentHash,
                    ),
                    Ok(ceiling) => {
                        let contribution =
                            ContributionIdentityV0::from_admitted_contribution_graph_v0(
                                &edge.target_id,
                                &edge.source_id,
                                edge_id,
                                &edge.scope_ref,
                                &evidence.content_hash,
                            );
                        let namespace = OriginComparisonNamespaceV0::for_admitted_contribution_v0(
                            &edge.target_id,
                            &edge.scope_ref,
                        );
                        assert_eq!(
                            contribution.artifact_algorithm(),
                            ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0
                        );
                        assert_eq!(
                            namespace.origin_admission_policy_id(),
                            ORIGIN_ADMISSION_POLICY_ID_V0
                        );
                        candidate_contribution = Some(contribution.clone());
                        candidate_namespace = Some(namespace.clone());

                        if !origin_complete {
                            AdmittedContributionCandidateDispositionV0::OriginAdmissionIncomplete
                        } else {
                            resolve_origin_disposition(
                                contribution,
                                namespace,
                                kind,
                                domain,
                                ceiling,
                                &decision_index,
                                &mut admitted_contributions,
                            )
                        }
                    }
                }
            }
        };

        candidate_audits.push(AdmittedContributionCandidateAuditV0 {
            edge_id: edge_id.clone(),
            source_evidence_id: edge.source_id.clone(),
            target_claim_id: edge.target_id.clone(),
            evidence_kind,
            claim_domain,
            candidate_ceiling,
            candidate_contribution,
            candidate_namespace,
            disposition,
        });
    }

    AdmittedContributionAuditV0 {
        schema: ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0.to_owned(),
        canonicalization_profile: ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
            .to_owned(),
        policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
        origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
        verified_prefix_identity: context.verified_prefix_identity().clone(),
        closure_identity: closure.identity().clone(),
        completion,
        candidate_audits,
        admitted_contributions: admitted_contributions.into_values().collect(),
    }
}

fn resolve_origin_disposition(
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    kind: EvidenceKind,
    domain: ClaimDomain,
    ceiling: Status,
    decision_index: &BTreeMap<OriginDecisionKeyV0, &OriginAdmissionDecisionV0>,
    admitted_contributions: &mut BTreeMap<ContributionIdentityV0, AdmittedContributionV0>,
) -> AdmittedContributionCandidateDispositionV0 {
    let key = OriginDecisionKeyV0 {
        contribution: contribution.clone(),
        namespace: namespace.clone(),
    };
    let Some(decision) = decision_index.get(&key).copied() else {
        return AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent;
    };

    match decision {
        OriginAdmissionDecisionV0::ConflictingBindings { .. }
        | OriginAdmissionDecisionV0::EligibleBindingUnresolved { .. } => {
            AdmittedContributionCandidateDispositionV0::OriginNotAdmitted {
                decision: decision.clone(),
            }
        }
        OriginAdmissionDecisionV0::OriginAdmitted { admitted_origin } => {
            assert_eq!(admitted_origin.contribution(), &contribution);
            assert_eq!(admitted_origin.namespace(), &namespace);
            assert_eq!(admitted_origin.policy_id(), ORIGIN_ADMISSION_POLICY_ID_V0);
            let admitted_contribution = AdmittedContributionV0 {
                policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
                origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                contribution: contribution.clone(),
                namespace,
                origin_group: admitted_origin.origin_group().to_owned(),
                evidence_kind: kind.as_str().to_owned(),
                claim_domain: domain.as_str().to_owned(),
                candidate_ceiling: ceiling,
                supporting_origin_candidate_selectors: admitted_origin
                    .supporting_candidate_selectors()
                    .to_vec(),
            };
            assert_eq!(
                admitted_contribution.evidence_kind(),
                EvidenceKind::ExternalSource.as_str()
            );
            assert_eq!(
                admitted_contribution.claim_domain(),
                ClaimDomain::ExternalReport.as_str()
            );
            assert_eq!(admitted_contribution.candidate_ceiling(), Status::Supported);
            let previous =
                admitted_contributions.insert(contribution, admitted_contribution.clone());
            assert!(
                previous.is_none(),
                "distinct candidate edges must not overwrite an exact admitted contribution"
            );
            AdmittedContributionCandidateDispositionV0::Admitted {
                admitted_contribution,
            }
        }
    }
}

fn build_origin_decision_index(
    origin_audit: &crate::origin_admission_audit::OriginAdmissionAuditV0,
) -> BTreeMap<OriginDecisionKeyV0, &OriginAdmissionDecisionV0> {
    let mut index = BTreeMap::new();
    for decision in origin_audit.contribution_decisions() {
        let key = match decision {
            OriginAdmissionDecisionV0::ConflictingBindings { conflict } => OriginDecisionKeyV0 {
                contribution: conflict.contribution().clone(),
                namespace: conflict.namespace().clone(),
            },
            OriginAdmissionDecisionV0::EligibleBindingUnresolved {
                contribution,
                namespace,
                ..
            } => OriginDecisionKeyV0 {
                contribution: contribution.clone(),
                namespace: namespace.clone(),
            },
            OriginAdmissionDecisionV0::OriginAdmitted { admitted_origin } => OriginDecisionKeyV0 {
                contribution: admitted_origin.contribution().clone(),
                namespace: admitted_origin.namespace().clone(),
            },
        };
        let previous = index.insert(key, decision);
        assert!(
            previous.is_none(),
            "origin admission produced duplicate terminal decisions for one exact key"
        );
    }
    index
}

fn policy_ineligible(
    reason: AdmittedContributionCandidateReasonV0,
) -> AdmittedContributionCandidateDispositionV0 {
    AdmittedContributionCandidateDispositionV0::PolicyIneligible { reason }
}

fn staged_evidence_kind(evaluation: &SupportCandidateStructureEvaluationV0<'_>) -> Option<String> {
    evaluation
        .evidence()
        .map(|evidence| evidence.evidence_kind.clone())
}

fn staged_claim_domain(evaluation: &SupportCandidateStructureEvaluationV0<'_>) -> Option<String> {
    evaluation
        .claim_domain()
        .map(|domain| domain.as_str().to_owned())
}

fn structure_reason(
    failure: SupportCandidateStructureFailureV0,
) -> AdmittedContributionCandidateReasonV0 {
    match failure {
        SupportCandidateStructureFailureV0::MissingTargetClaim => {
            AdmittedContributionCandidateReasonV0::MissingTargetClaim
        }
        SupportCandidateStructureFailureV0::MissingTypedTargetClaim => {
            AdmittedContributionCandidateReasonV0::MissingTypedTargetClaim
        }
        SupportCandidateStructureFailureV0::UnsupportedEdgeKindForV0 => {
            AdmittedContributionCandidateReasonV0::UnsupportedEdgeKindForV0
        }
        SupportCandidateStructureFailureV0::MalformedMetadata => {
            AdmittedContributionCandidateReasonV0::MalformedClaimMetadata
        }
        SupportCandidateStructureFailureV0::MissingClaimDomain => {
            AdmittedContributionCandidateReasonV0::MissingClaimDomain
        }
        SupportCandidateStructureFailureV0::WrongTypeClaimDomain => {
            AdmittedContributionCandidateReasonV0::WrongTypeClaimDomain
        }
        SupportCandidateStructureFailureV0::DuplicateMetadataKey => {
            AdmittedContributionCandidateReasonV0::DuplicateClaimDomainKey
        }
        SupportCandidateStructureFailureV0::UnknownClaimDomain => {
            AdmittedContributionCandidateReasonV0::UnknownClaimDomain
        }
        SupportCandidateStructureFailureV0::MissingSourceEvidence => {
            AdmittedContributionCandidateReasonV0::MissingSourceEvidence
        }
        SupportCandidateStructureFailureV0::ScopeMismatch => {
            AdmittedContributionCandidateReasonV0::ScopeMismatch
        }
        SupportCandidateStructureFailureV0::UnknownEvidenceKind => {
            AdmittedContributionCandidateReasonV0::UnknownEvidenceKind
        }
    }
}

fn exact_supported_ceiling(
    candidate_ceiling: Option<Status>,
) -> Result<Status, AdmittedContributionCandidateReasonV0> {
    match candidate_ceiling {
        None => Err(AdmittedContributionCandidateReasonV0::NoSupportCeiling),
        Some(Status::Supported) => Ok(Status::Supported),
        Some(_) => Err(AdmittedContributionCandidateReasonV0::CandidateCeilingMismatch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standing::{
        StandingClaim, StandingView, TypedClaimNode, TypedEvidenceNode, TypedJustificationEdge,
    };

    #[test]
    fn selected_policy_cell_fails_closed_on_absence_or_status_drift() {
        assert_eq!(
            exact_supported_ceiling(None),
            Err(AdmittedContributionCandidateReasonV0::NoSupportCeiling)
        );
        assert_eq!(
            exact_supported_ceiling(Some(Status::Conjectured)),
            Err(AdmittedContributionCandidateReasonV0::CandidateCeilingMismatch)
        );
        assert_eq!(
            exact_supported_ceiling(Some(Status::Settled)),
            Err(AdmittedContributionCandidateReasonV0::CandidateCeilingMismatch)
        );
        assert_eq!(
            exact_supported_ceiling(Some(Status::Supported)),
            Ok(Status::Supported)
        );
    }

    #[test]
    fn shared_unknown_evidence_kind_maps_to_the_versioned_audit_reason() {
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
                summary: "synthetic private audit staging input".into(),
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
            staged_evidence_kind(&evaluation).as_deref(),
            Some("UnknownEvidenceKind")
        );
        assert_eq!(
            staged_claim_domain(&evaluation).as_deref(),
            Some("ExternalReport")
        );
        assert_eq!(
            structure_reason(evaluation.failure().unwrap()),
            AdmittedContributionCandidateReasonV0::UnknownEvidenceKind
        );
    }
}
