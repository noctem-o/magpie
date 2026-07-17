//! Standing-inert support-contribution audit v0.
//!
//! This module implements the contract ratified by Ticket 0051
//! (`docs/design/support-contribution-audit-v0.md`). It consumes exactly one
//! internally derived `AdmittedContributionAuditV0` from the same
//! private-construction replay context and immutable closure, validates the
//! complete upstream composition in the frozen eighteen-stage order, and maps
//! each aligned admitted contribution one-to-one into a standing-inert
//! `SupportContributionV0`.
//!
//! The audit establishes only that each exact admitted contribution is a valid
//! positive, ceiling-bounded input to a future aggregation policy. It does not
//! create support, count origin groups, collapse same-origin material, define
//! an aggregation lane, or change standing.
//!
//! There is exactly one composition path. The public resolver decomposes the
//! internally derived admitted audit through its public getters into a
//! crate-private plain-data view and calls one crate-private pure composer.
//! Hostile tests call the same composer with staged malformed inputs; no test
//! switch, policy override, or alternate authority carrier exists.
//!
//! Declaration order of every field and variant below is canonical byte
//! order. Do not reorder.

use magpie_log::Status;
use serde::Serialize;

use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
use crate::origin_admission_replay::VerifiedLogPrefixIdentityV0;
use crate::origin_binding_verifier::{ContributionIdentityV0, OriginComparisonNamespaceV0};
use crate::resolution_content_closure::ResolutionContentClosureIdentityV0;

pub const SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0: &str = "magpie-support-contribution-audit-v0";

pub const SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-support-contribution-audit-json-v0";

pub const SUPPORT_CONTRIBUTION_POLICY_ID_V0: &str = "magpie-support-contribution-v0";

/// Frozen completion vocabulary for one support-contribution audit.
///
/// `AdmittedContributionIncomplete` inherits the exact unavailable-selector
/// vector, in exact order, from the upstream admitted-contribution audit.
/// `UpstreamCompositionRejected` carries the first composition failure in the
/// frozen stage order and always accompanies zero support contributions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum SupportContributionAuditCompletionV0 {
    Complete,

    AdmittedContributionIncomplete {
        unavailable_binding_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    },

    UpstreamCompositionRejected {
        failure: SupportContributionCompositionFailureV0,
    },
}

/// Closed composition-failure vocabulary in the frozen stage-correspondence
/// order.
///
/// `SupportCeilingPolicyMismatch`, `SupportContextRequirementMismatch`,
/// `DuplicateCandidateEdgeId` and `EmptyUnavailableBindingSelectors` are
/// global failures: they carry no `ContributionIdentityV0`.
/// `EmptyUnavailableBindingSelectors` additionally carries no candidate
/// `edge_id`, because the malformed inherited value is the completion itself;
/// there is no candidate edge to report, and fabricating one would be a
/// synthetic value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum SupportContributionCompositionFailureV0 {
    UpstreamSchemaMismatch,

    UpstreamCanonicalizationProfileMismatch,

    UpstreamAdmittedPolicyMismatch,

    UpstreamOriginPolicyMismatch,

    VerifiedPrefixIdentityMismatch,

    ClosureIdentityMismatch,

    SupportCeilingPolicyMismatch,

    SupportContextRequirementMismatch,

    DuplicateCandidateEdgeId {
        edge_id: String,
    },

    AdmittedCandidateTraceMismatch {
        edge_id: String,
    },

    DuplicateCandidateAdmission {
        contribution: ContributionIdentityV0,
    },

    DuplicateTopLevelAdmission {
        contribution: ContributionIdentityV0,
    },

    AdmittedSetMismatch {
        candidate_only: Vec<ContributionIdentityV0>,

        top_level_only: Vec<ContributionIdentityV0>,
    },

    AdmittedValueMismatch {
        contribution: ContributionIdentityV0,
    },

    UpstreamCompletionDispositionMismatch {
        edge_id: String,
    },

    EmptyUnavailableBindingSelectors,

    ContributionInvariantMismatch {
        contribution: ContributionIdentityV0,
        reason: SupportContributionInvariantReasonV0,
    },
}

/// Closed per-contribution invariant-failure vocabulary in the frozen
/// eighteen-step first-failure order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum SupportContributionInvariantReasonV0 {
    AdmittedPolicyMismatch,
    OriginPolicyMismatch,
    EvidenceKindMismatch,
    ClaimDomainMismatch,
    CandidateCeilingMismatch,
    InvalidTargetClaimReference,
    InvalidSourceEvidenceReference,
    InvalidJustificationEdgeReference,
    InvalidScopeReference,
    ArtifactAlgorithmMismatch,
    InvalidArtifactDigest,
    NamespacePolicyMismatch,
    NamespaceTargetMismatch,
    NamespaceScopeMismatch,
    InvalidOriginGroup,
    MissingSupportingOriginSelector,
    InvalidSupportingOriginSelector,
    NonCanonicalSupportingOriginSelectorOrder,
}

/// One exact admitted contribution validated as a positive support input.
///
/// This value means only that the exact admitted contribution is a valid
/// positive input to a future aggregation policy, capped at `Supported`. It
/// does not mean the target is currently supported, that the contribution has
/// been counted, that another origin group corroborates it, or that the
/// report is true.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SupportContributionV0 {
    policy_id: String,
    admitted_contribution_policy_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: String,
    evidence_kind: String,
    claim_domain: String,
    support_ceiling: Status,
    supporting_origin_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
}

impl SupportContributionV0 {
    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn admitted_contribution_policy_id(&self) -> &str {
        &self.admitted_contribution_policy_id
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

    pub fn support_ceiling(&self) -> Status {
        self.support_ceiling
    }

    pub fn supporting_origin_candidate_selectors(&self) -> &[ArtifactProvenanceAnchorSelectorV0] {
        &self.supporting_origin_candidate_selectors
    }
}

/// Deterministic, standing-inert support-contribution audit output.
///
/// A rejected audit still binds the expected current verified-prefix and
/// closure identities; it never copies a mismatched upstream identity into
/// the new top-level authority surface. Only this top-level type exposes
/// canonical bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SupportContributionAuditV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    admitted_contribution_policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: SupportContributionAuditCompletionV0,
    support_contributions: Vec<SupportContributionV0>,
}

impl SupportContributionAuditV0 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn canonicalization_profile(&self) -> &str {
        &self.canonicalization_profile
    }

    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn admitted_contribution_policy_id(&self) -> &str {
        &self.admitted_contribution_policy_id
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

    pub fn completion(&self) -> &SupportContributionAuditCompletionV0 {
        &self.completion
    }

    pub fn support_contributions(&self) -> &[SupportContributionV0] {
        &self.support_contributions
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("support-contribution audits are always serializable")
    }
}

#[cfg(test)]
mod serde_shape_tests {
    use super::*;
    use crate::origin_binding_verifier::{
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
    };

    fn selector(run_id: &str) -> ArtifactProvenanceAnchorSelectorV0 {
        ArtifactProvenanceAnchorSelectorV0::new(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            "a".repeat(64),
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            run_id,
        )
    }

    fn contribution() -> ContributionIdentityV0 {
        ContributionIdentityV0::from_admitted_contribution_graph_v0(
            "claim",
            "evidence",
            "edge",
            "scope",
            &"b".repeat(64),
        )
    }

    fn contribution_json() -> String {
        format!(
            concat!(
                "{{\"target_claim_id\":\"claim\",\"source_evidence_id\":\"evidence\",",
                "\"justification_edge_id\":\"edge\",\"scope_ref\":\"scope\",",
                "\"artifact_algorithm\":\"sha256\",\"artifact_digest\":\"{}\"}}"
            ),
            "b".repeat(64)
        )
    }

    fn bytes(value: &impl Serialize) -> String {
        String::from_utf8(serde_json::to_vec(value).unwrap()).unwrap()
    }

    #[test]
    fn completion_variants_serialize_with_exact_adjacent_tagging() {
        assert_eq!(
            bytes(&SupportContributionAuditCompletionV0::Complete),
            r#"{"outcome":"complete"}"#
        );

        let incomplete = SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
            unavailable_binding_selectors: vec![selector("run-test")],
        };
        assert_eq!(
            bytes(&incomplete),
            format!(
                concat!(
                    "{{\"outcome\":\"admitted_contribution_incomplete\",",
                    "\"details\":{{\"unavailable_binding_selectors\":[",
                    "{{\"bundle_kind\":\"magpie-origin-binding-acquisition-v0\",",
                    "\"witness_root\":\"{}\",\"witness_algorithm\":\"sha256\",",
                    "\"canonicalization_profile\":\"magpie-origin-binding-json-v0\",",
                    "\"run_id\":\"run-test\"}}]}}}}"
                ),
                "a".repeat(64)
            )
        );

        let rejected = SupportContributionAuditCompletionV0::UpstreamCompositionRejected {
            failure: SupportContributionCompositionFailureV0::EmptyUnavailableBindingSelectors,
        };
        assert_eq!(
            bytes(&rejected),
            concat!(
                r#"{"outcome":"upstream_composition_rejected","details":{"failure":"#,
                r#"{"outcome":"empty_unavailable_binding_selectors"}}}"#
            )
        );
    }

    #[test]
    fn unit_variants_omit_the_details_key() {
        let unit_failures: Vec<(SupportContributionCompositionFailureV0, &str)> = vec![
            (
                SupportContributionCompositionFailureV0::UpstreamSchemaMismatch,
                "upstream_schema_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::UpstreamCanonicalizationProfileMismatch,
                "upstream_canonicalization_profile_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::UpstreamAdmittedPolicyMismatch,
                "upstream_admitted_policy_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::UpstreamOriginPolicyMismatch,
                "upstream_origin_policy_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::VerifiedPrefixIdentityMismatch,
                "verified_prefix_identity_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::ClosureIdentityMismatch,
                "closure_identity_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::SupportCeilingPolicyMismatch,
                "support_ceiling_policy_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::SupportContextRequirementMismatch,
                "support_context_requirement_mismatch",
            ),
            (
                SupportContributionCompositionFailureV0::EmptyUnavailableBindingSelectors,
                "empty_unavailable_binding_selectors",
            ),
        ];
        for (failure, outcome) in unit_failures {
            assert_eq!(bytes(&failure), format!(r#"{{"outcome":"{outcome}"}}"#));
        }

        let unit_reasons: Vec<(SupportContributionInvariantReasonV0, &str)> = vec![
            (
                SupportContributionInvariantReasonV0::AdmittedPolicyMismatch,
                "admitted_policy_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::OriginPolicyMismatch,
                "origin_policy_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::EvidenceKindMismatch,
                "evidence_kind_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::ClaimDomainMismatch,
                "claim_domain_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::CandidateCeilingMismatch,
                "candidate_ceiling_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidTargetClaimReference,
                "invalid_target_claim_reference",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidSourceEvidenceReference,
                "invalid_source_evidence_reference",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidJustificationEdgeReference,
                "invalid_justification_edge_reference",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidScopeReference,
                "invalid_scope_reference",
            ),
            (
                SupportContributionInvariantReasonV0::ArtifactAlgorithmMismatch,
                "artifact_algorithm_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidArtifactDigest,
                "invalid_artifact_digest",
            ),
            (
                SupportContributionInvariantReasonV0::NamespacePolicyMismatch,
                "namespace_policy_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::NamespaceTargetMismatch,
                "namespace_target_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::NamespaceScopeMismatch,
                "namespace_scope_mismatch",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidOriginGroup,
                "invalid_origin_group",
            ),
            (
                SupportContributionInvariantReasonV0::MissingSupportingOriginSelector,
                "missing_supporting_origin_selector",
            ),
            (
                SupportContributionInvariantReasonV0::InvalidSupportingOriginSelector,
                "invalid_supporting_origin_selector",
            ),
            (
                SupportContributionInvariantReasonV0::NonCanonicalSupportingOriginSelectorOrder,
                "non_canonical_supporting_origin_selector_order",
            ),
        ];
        for (reason, outcome) in unit_reasons {
            assert_eq!(bytes(&reason), format!(r#"{{"outcome":"{outcome}"}}"#));
        }
    }

    #[test]
    fn payload_variants_include_exact_details_objects() {
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::DuplicateCandidateEdgeId {
                    edge_id: "edge".to_owned(),
                }
            ),
            r#"{"outcome":"duplicate_candidate_edge_id","details":{"edge_id":"edge"}}"#
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::AdmittedCandidateTraceMismatch {
                    edge_id: "edge".to_owned(),
                }
            ),
            r#"{"outcome":"admitted_candidate_trace_mismatch","details":{"edge_id":"edge"}}"#
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::UpstreamCompletionDispositionMismatch {
                    edge_id: "edge".to_owned(),
                }
            ),
            r#"{"outcome":"upstream_completion_disposition_mismatch","details":{"edge_id":"edge"}}"#
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::DuplicateCandidateAdmission {
                    contribution: contribution(),
                }
            ),
            format!(
                r#"{{"outcome":"duplicate_candidate_admission","details":{{"contribution":{}}}}}"#,
                contribution_json()
            )
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::DuplicateTopLevelAdmission {
                    contribution: contribution(),
                }
            ),
            format!(
                r#"{{"outcome":"duplicate_top_level_admission","details":{{"contribution":{}}}}}"#,
                contribution_json()
            )
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::AdmittedSetMismatch {
                    candidate_only: Vec::new(),
                    top_level_only: Vec::new(),
                }
            ),
            r#"{"outcome":"admitted_set_mismatch","details":{"candidate_only":[],"top_level_only":[]}}"#
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::AdmittedValueMismatch {
                    contribution: contribution(),
                }
            ),
            format!(
                r#"{{"outcome":"admitted_value_mismatch","details":{{"contribution":{}}}}}"#,
                contribution_json()
            )
        );
        assert_eq!(
            bytes(
                &SupportContributionCompositionFailureV0::ContributionInvariantMismatch {
                    contribution: contribution(),
                    reason: SupportContributionInvariantReasonV0::InvalidArtifactDigest,
                }
            ),
            format!(
                concat!(
                    r#"{{"outcome":"contribution_invariant_mismatch","details":"#,
                    r#"{{"contribution":{},"reason":{{"outcome":"invalid_artifact_digest"}}}}}}"#
                ),
                contribution_json()
            )
        );
    }
}
