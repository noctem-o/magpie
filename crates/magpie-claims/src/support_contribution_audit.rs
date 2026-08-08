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
//!
//! Support contributions cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionV0;
//! let _contribution = SupportContributionV0 {};
//! ```
//!
//! Support audits cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionAuditV0;
//! let _audit = SupportContributionAuditV0 {};
//! ```
//!
//! Completion values cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionAuditCompletionV0;
//! let _: SupportContributionAuditCompletionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Composition failures cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionCompositionFailureV0;
//! let _: SupportContributionCompositionFailureV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Invariant reasons cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionInvariantReasonV0;
//! let _: SupportContributionInvariantReasonV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Support contributions cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionV0;
//! let _: SupportContributionV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Support audits cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::SupportContributionAuditV0;
//! let _: SupportContributionAuditV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! No resolver accepts a caller-created admitted audit:
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
//!     let _ = context.resolve_support_contribution_audit_v0(closure, audit);
//! }
//! ```
//!
//! No resolver accepts a caller-created admitted-contribution list:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     AdmittedContributionV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     admitted: Vec<AdmittedContributionV0>,
//! ) {
//!     let _ = context.resolve_support_contribution_audit_v0(closure, admitted);
//! }
//! ```
//!
//! No resolver accepts a caller-selected claim ID:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     claim_id: &str,
//! ) {
//!     let _ = context.resolve_support_contribution_audit_v0(closure, claim_id);
//! }
//! ```
//!
//! No resolver accepts a caller-selected origin group:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     origin_group: &str,
//! ) {
//!     let _ = context.resolve_support_contribution_audit_v0(closure, origin_group);
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
//!     let _ = context.resolve_support_contribution_audit_v0(closure, policy_id);
//! }
//! ```
//!
//! Serialized or cloned audit output cannot substitute for replay authority:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionAuditV0,
//! };
//! fn resolve(
//!     context: &OriginAdmissionReplayContextV0,
//!     closure: &ResolutionContentClosureV0,
//!     audit: SupportContributionAuditV0,
//! ) {
//!     let bytes = audit.canonical_bytes();
//!     let _ = context.resolve_support_contribution_audit_v0(
//!         closure,
//!         audit.clone(),
//!         bytes,
//!     );
//! }
//! ```
//!
//! A standalone standing view has no support-contribution resolver:
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingView};
//! fn resolve(view: &StandingView, closure: &ResolutionContentClosureV0) {
//!     let _ = view.resolve_support_contribution_audit_v0(closure);
//! }
//! ```
//!
//! A standalone standing replay snapshot has no complete support-contribution
//! resolver:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureV0, StandingReplaySnapshot,
//! };
//! fn resolve(
//!     snapshot: &StandingReplaySnapshot,
//!     closure: &ResolutionContentClosureV0,
//! ) {
//!     let _ = snapshot.resolve_support_contribution_audit_v0(closure);
//! }
//! ```

use std::collections::{BTreeMap, BTreeSet};

use magpie_log::Status;
use serde::Serialize;

use crate::admitted_contribution_audit::{
    AdmittedContributionAuditCompletionV0, AdmittedContributionAuditV0,
    AdmittedContributionCandidateAuditV0, AdmittedContributionCandidateDispositionV0,
    AdmittedContributionCandidateReasonV0, AdmittedContributionV0,
    ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0,
    ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0, ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0,
    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
};
use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
use crate::origin_admission_audit::{OriginAdmissionDecisionV0, ORIGIN_ADMISSION_POLICY_ID_V0};
use crate::origin_admission_replay::{OriginAdmissionReplayContextV0, VerifiedLogPrefixIdentityV0};
use crate::origin_binding_verifier::{
    valid_origin_binding_artifact_digest_v0, valid_origin_binding_replay_reference_v0,
    valid_origin_binding_selector_v0, valid_origin_group_v0, ContributionIdentityV0,
    OriginComparisonNamespaceV0,
};
use crate::policy::{
    support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
    SupportContextRequirement,
};
use crate::resolution_content_closure::{
    ResolutionContentClosureIdentityV0, ResolutionContentClosureV0,
};

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

/// Test-only constructor for hostile policy-v3 composition inputs.
///
/// `#[cfg(test)]` keeps this outside production builds and public API. It does
/// not create another support-audit composition path.
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn stage_support_contribution_v0_for_tests(
    policy_id: &str,
    admitted_contribution_policy_id: &str,
    origin_admission_policy_id: &str,
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    origin_group: &str,
    evidence_kind: &str,
    claim_domain: &str,
    support_ceiling: Status,
    supporting_origin_candidate_selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
) -> SupportContributionV0 {
    SupportContributionV0 {
        policy_id: policy_id.to_owned(),
        admitted_contribution_policy_id: admitted_contribution_policy_id.to_owned(),
        origin_admission_policy_id: origin_admission_policy_id.to_owned(),
        contribution,
        namespace,
        origin_group: origin_group.to_owned(),
        evidence_kind: evidence_kind.to_owned(),
        claim_domain: claim_domain.to_owned(),
        support_ceiling,
        supporting_origin_candidate_selectors,
    }
}

/// Test-only constructor for hostile policy-v3 composition inputs.
///
/// See [`stage_support_contribution_v0_for_tests`].
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn stage_support_contribution_audit_v0_for_tests(
    schema: &str,
    canonicalization_profile: &str,
    policy_id: &str,
    admitted_contribution_policy_id: &str,
    origin_admission_policy_id: &str,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: SupportContributionAuditCompletionV0,
    support_contributions: Vec<SupportContributionV0>,
) -> SupportContributionAuditV0 {
    SupportContributionAuditV0 {
        schema: schema.to_owned(),
        canonicalization_profile: canonicalization_profile.to_owned(),
        policy_id: policy_id.to_owned(),
        admitted_contribution_policy_id: admitted_contribution_policy_id.to_owned(),
        origin_admission_policy_id: origin_admission_policy_id.to_owned(),
        verified_prefix_identity,
        closure_identity,
        completion,
        support_contributions,
    }
}

/// Expected top-level authority for one composition.
///
/// Carries the current replay-prefix and closure identities plus the two
/// fixed compiled support-policy cells. The production resolver fills the
/// cells from the compiled policy; hostile tests may stage drifted cells only
/// through this crate-private view. There is no public policy override,
/// runtime policy parameter, mock registry, or caller-selected cell.
#[derive(Clone, Debug)]
pub(crate) struct ExpectedIdentitiesV0 {
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    support_ceiling_cell: Option<Status>,
    support_context_requirement_cell: SupportContextRequirement,
}

/// Crate-private plain-data view of one inherited admitted-contribution
/// audit. The production resolver builds this view through the admitted
/// audit's public getters only; hostile tests stage it directly. This is the
/// single input to the single composition path.
#[derive(Clone, Debug)]
pub(crate) struct AdmittedAuditCompositionInputV0 {
    schema: String,
    canonicalization_profile: String,
    policy_id: String,
    origin_admission_policy_id: String,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    completion: AdmittedContributionAuditCompletionV0,
    candidate_audits: Vec<AdmittedCandidateCompositionInputV0>,
    admitted_contributions: Vec<AdmittedContributionV0>,
}

impl AdmittedAuditCompositionInputV0 {
    fn from_audit(audit: &AdmittedContributionAuditV0) -> Self {
        Self {
            schema: audit.schema().to_owned(),
            canonicalization_profile: audit.canonicalization_profile().to_owned(),
            policy_id: audit.policy_id().to_owned(),
            origin_admission_policy_id: audit.origin_admission_policy_id().to_owned(),
            verified_prefix_identity: audit.verified_prefix_identity().clone(),
            closure_identity: audit.closure_identity().clone(),
            completion: audit.completion().clone(),
            candidate_audits: audit
                .candidate_audits()
                .iter()
                .map(AdmittedCandidateCompositionInputV0::from_candidate)
                .collect(),
            admitted_contributions: audit.admitted_contributions().to_vec(),
        }
    }
}

/// Crate-private plain-data view of one inherited candidate audit.
#[derive(Clone, Debug)]
pub(crate) struct AdmittedCandidateCompositionInputV0 {
    edge_id: String,
    source_evidence_id: String,
    target_claim_id: String,
    evidence_kind: Option<String>,
    claim_domain: Option<String>,
    candidate_ceiling: Option<Status>,
    candidate_contribution: Option<ContributionIdentityV0>,
    candidate_namespace: Option<OriginComparisonNamespaceV0>,
    disposition: AdmittedCandidateDispositionInputV0,
}

impl AdmittedCandidateCompositionInputV0 {
    fn from_candidate(candidate: &AdmittedContributionCandidateAuditV0) -> Self {
        Self {
            edge_id: candidate.edge_id().to_owned(),
            source_evidence_id: candidate.source_evidence_id().to_owned(),
            target_claim_id: candidate.target_claim_id().to_owned(),
            evidence_kind: candidate.evidence_kind().map(str::to_owned),
            claim_domain: candidate.claim_domain().map(str::to_owned),
            candidate_ceiling: candidate.candidate_ceiling(),
            candidate_contribution: candidate.candidate_contribution().cloned(),
            candidate_namespace: candidate.candidate_namespace().cloned(),
            disposition: AdmittedCandidateDispositionInputV0::from_disposition(
                candidate.disposition(),
            ),
        }
    }
}

/// Crate-private candidate disposition holding whole public upstream values.
///
/// The composition laws classify by variant; the `reason` and `decision`
/// payloads are carried for input fidelity (the view mirrors the upstream
/// audit exactly) even though no stage reads them.
#[derive(Clone, Debug)]
pub(crate) enum AdmittedCandidateDispositionInputV0 {
    PolicyIneligible {
        #[allow(dead_code)]
        reason: AdmittedContributionCandidateReasonV0,
    },

    OriginAdmissionIncomplete,

    OriginDecisionAbsent,

    OriginNotAdmitted {
        #[allow(dead_code)]
        decision: OriginAdmissionDecisionV0,
    },

    Admitted {
        admitted_contribution: AdmittedContributionV0,
    },
}

impl AdmittedCandidateDispositionInputV0 {
    fn from_disposition(disposition: &AdmittedContributionCandidateDispositionV0) -> Self {
        match disposition {
            AdmittedContributionCandidateDispositionV0::PolicyIneligible { reason } => {
                Self::PolicyIneligible {
                    reason: reason.clone(),
                }
            }
            AdmittedContributionCandidateDispositionV0::OriginAdmissionIncomplete => {
                Self::OriginAdmissionIncomplete
            }
            AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent => {
                Self::OriginDecisionAbsent
            }
            AdmittedContributionCandidateDispositionV0::OriginNotAdmitted { decision } => {
                Self::OriginNotAdmitted {
                    decision: decision.clone(),
                }
            }
            AdmittedContributionCandidateDispositionV0::Admitted {
                admitted_contribution,
            } => Self::Admitted {
                admitted_contribution: admitted_contribution.clone(),
            },
        }
    }
}

/// The single composition path: validate one inherited admitted-contribution
/// audit against the expected identities in the frozen eighteen-stage order,
/// then map upstream completion and project aligned admissions one-to-one.
///
/// Returns the first composition failure in stage order. The caller
/// (production resolver or hostile test) decides how to present it; the
/// composer never normalizes, repairs, sorts, deduplicates, or prefers any
/// inherited representation.
#[allow(clippy::result_large_err)]
pub(crate) fn compose_support_contribution_audit_v0(
    expected: &ExpectedIdentitiesV0,
    input: AdmittedAuditCompositionInputV0,
) -> Result<SupportContributionAuditV0, SupportContributionCompositionFailureV0> {
    use SupportContributionCompositionFailureV0 as Failure;

    // Stages 1-6: exact upstream identity validation. Any mismatch is a
    // global composition rejection, never reinterpreted or repaired.
    if input.schema != ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0 {
        return Err(Failure::UpstreamSchemaMismatch);
    }
    if input.canonicalization_profile != ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0 {
        return Err(Failure::UpstreamCanonicalizationProfileMismatch);
    }
    if input.policy_id != ADMITTED_CONTRIBUTION_POLICY_ID_V0 {
        return Err(Failure::UpstreamAdmittedPolicyMismatch);
    }
    if input.origin_admission_policy_id != ORIGIN_ADMISSION_POLICY_ID_V0 {
        return Err(Failure::UpstreamOriginPolicyMismatch);
    }
    if input.verified_prefix_identity != expected.verified_prefix_identity {
        return Err(Failure::VerifiedPrefixIdentityMismatch);
    }
    if input.closure_identity != expected.closure_identity {
        return Err(Failure::ClosureIdentityMismatch);
    }

    // Stages 7-8: fixed compiled support-policy cells, checked exactly once,
    // independently of admitted contribution count.
    if expected.support_ceiling_cell != Some(Status::Supported) {
        return Err(Failure::SupportCeilingPolicyMismatch);
    }
    if expected.support_context_requirement_cell != SupportContextRequirement::NoPrivilegedContext {
        return Err(Failure::SupportContextRequirementMismatch);
    }

    // Stage 9: globally unique exact edge IDs across the complete candidate
    // universe, regardless of disposition. The lexical-first duplicate is
    // reported independent of caller or vector order.
    let mut edge_id_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for candidate in &input.candidate_audits {
        *edge_id_counts
            .entry(candidate.edge_id.as_str())
            .or_default() += 1;
    }
    if let Some((&edge_id, _)) = edge_id_counts.iter().find(|(_, count)| **count > 1) {
        return Err(Failure::DuplicateCandidateEdgeId {
            edge_id: edge_id.to_owned(),
        });
    }

    // Edge IDs are unique from here on. All "first candidate" selections below
    // iterate in exact edge-ID order, independent of vector order.
    let candidate_order: BTreeMap<&str, usize> = input
        .candidate_audits
        .iter()
        .enumerate()
        .map(|(index, candidate)| (candidate.edge_id.as_str(), index))
        .collect();

    // Stage 10: every exactly `Admitted` candidate must align with its nested
    // admitted value across all eight exact fields, checked in the frozen
    // field order. All trace checks complete before any map insertion.
    for (edge_id, &index) in &candidate_order {
        let candidate = &input.candidate_audits[index];
        let AdmittedCandidateDispositionInputV0::Admitted {
            admitted_contribution,
        } = &candidate.disposition
        else {
            continue;
        };
        let contribution = admitted_contribution.contribution();
        let trace_aligned = candidate.edge_id == contribution.justification_edge_id()
            && candidate.source_evidence_id == contribution.source_evidence_id()
            && candidate.target_claim_id == contribution.target_claim_id()
            && candidate.evidence_kind.as_deref() == Some(admitted_contribution.evidence_kind())
            && candidate.claim_domain.as_deref() == Some(admitted_contribution.claim_domain())
            && candidate.candidate_ceiling == Some(admitted_contribution.candidate_ceiling())
            && candidate.candidate_contribution.as_ref() == Some(contribution)
            && candidate.candidate_namespace.as_ref() == Some(admitted_contribution.namespace());
        if !trace_aligned {
            return Err(Failure::AdmittedCandidateTraceMismatch {
                edge_id: (*edge_id).to_owned(),
            });
        }
    }

    // Stage 11: duplicate exact contribution identities among candidate
    // admissions. The first affected identity in exact
    // `ContributionIdentityV0` order is reported.
    let mut candidate_identity_counts: BTreeMap<ContributionIdentityV0, usize> = BTreeMap::new();
    for candidate in &input.candidate_audits {
        if let AdmittedCandidateDispositionInputV0::Admitted {
            admitted_contribution,
        } = &candidate.disposition
        {
            *candidate_identity_counts
                .entry(admitted_contribution.contribution().clone())
                .or_default() += 1;
        }
    }
    if let Some((identity, _)) = candidate_identity_counts
        .iter()
        .find(|(_, count)| **count > 1)
    {
        return Err(Failure::DuplicateCandidateAdmission {
            contribution: identity.clone(),
        });
    }
    let candidate_admitted: BTreeMap<ContributionIdentityV0, AdmittedContributionV0> = input
        .candidate_audits
        .iter()
        .filter_map(|candidate| match &candidate.disposition {
            AdmittedCandidateDispositionInputV0::Admitted {
                admitted_contribution,
            } => Some((
                admitted_contribution.contribution().clone(),
                admitted_contribution.clone(),
            )),
            _ => None,
        })
        .collect();

    // Stage 12: duplicate exact contribution identities among top-level
    // admissions, first affected identity in exact order.
    let mut top_level_identity_counts: BTreeMap<ContributionIdentityV0, usize> = BTreeMap::new();
    for admitted in &input.admitted_contributions {
        *top_level_identity_counts
            .entry(admitted.contribution().clone())
            .or_default() += 1;
    }
    if let Some((identity, _)) = top_level_identity_counts
        .iter()
        .find(|(_, count)| **count > 1)
    {
        return Err(Failure::DuplicateTopLevelAdmission {
            contribution: identity.clone(),
        });
    }
    let top_level_admitted: BTreeMap<ContributionIdentityV0, AdmittedContributionV0> = input
        .admitted_contributions
        .iter()
        .map(|admitted| (admitted.contribution().clone(), admitted.clone()))
        .collect();

    // Stage 13: admitted key-set equality. Difference vectors use exact
    // `ContributionIdentityV0` order.
    let candidate_keys: BTreeSet<&ContributionIdentityV0> = candidate_admitted.keys().collect();
    let top_level_keys: BTreeSet<&ContributionIdentityV0> = top_level_admitted.keys().collect();
    if candidate_keys != top_level_keys {
        return Err(Failure::AdmittedSetMismatch {
            candidate_only: candidate_keys
                .difference(&top_level_keys)
                .map(|key| (*key).clone())
                .collect(),
            top_level_only: top_level_keys
                .difference(&candidate_keys)
                .map(|key| (*key).clone())
                .collect(),
        });
    }

    // Stage 14: complete admitted-value equality for every key, first
    // mismatch in exact `ContributionIdentityV0` order.
    for (identity, candidate_value) in &candidate_admitted {
        let top_level_value = top_level_admitted
            .get(identity)
            .expect("admitted key sets are equal at stage 14");
        if candidate_value != top_level_value {
            return Err(Failure::AdmittedValueMismatch {
                contribution: identity.clone(),
            });
        }
    }

    // Stage 15, shape first: an incomplete upstream completion requires a
    // non-empty unavailable-selector vector. The upstream constructor
    // produces `Complete` whenever that vector is empty, so an empty-vector
    // incomplete completion is unrepresentable from honest upstream output.
    // It is never normalized to `Complete`, never mapped, never repaired.
    if let AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
        unavailable_binding_selectors,
    } = &input.completion
    {
        if unavailable_binding_selectors.is_empty() {
            return Err(Failure::EmptyUnavailableBindingSelectors);
        }
    }

    // Stage 15, disposition agreement: the upstream completion must agree
    // with every candidate disposition, first inconsistent candidate in
    // exact edge-ID order.
    for (edge_id, &index) in &candidate_order {
        let candidate = &input.candidate_audits[index];
        let inconsistent = match (&input.completion, &candidate.disposition) {
            (
                AdmittedContributionAuditCompletionV0::Complete,
                AdmittedCandidateDispositionInputV0::OriginAdmissionIncomplete,
            ) => true,
            (
                AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete { .. },
                AdmittedCandidateDispositionInputV0::PolicyIneligible { .. }
                | AdmittedCandidateDispositionInputV0::OriginAdmissionIncomplete,
            ) => false,
            (AdmittedContributionAuditCompletionV0::Complete, _) => false,
            (AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete { .. }, _) => true,
        };
        if inconsistent {
            return Err(Failure::UpstreamCompletionDispositionMismatch {
                edge_id: (*edge_id).to_owned(),
            });
        }
    }

    // Stage 16: the eighteen per-contribution invariants in exact
    // first-failure order, first affected contribution in exact
    // `ContributionIdentityV0` order.
    for (identity, admitted) in &candidate_admitted {
        if let Some(reason) = first_invariant_failure(admitted) {
            return Err(Failure::ContributionInvariantMismatch {
                contribution: identity.clone(),
                reason,
            });
        }
    }

    // Stage 17: upstream completion mapping, only after every composition
    // and invariant check has succeeded. No partial support vector survives.
    // Stage 18: one-to-one projection in exact contribution order.
    let (completion, support_contributions) = match input.completion {
        AdmittedContributionAuditCompletionV0::Complete => (
            SupportContributionAuditCompletionV0::Complete,
            candidate_admitted
                .values()
                .map(project_support_contribution_v0)
                .collect(),
        ),
        AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
            unavailable_binding_selectors,
        } => (
            SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                unavailable_binding_selectors,
            },
            Vec::new(),
        ),
    };

    Ok(audit_shell(
        &expected.verified_prefix_identity,
        &expected.closure_identity,
    )
    .with_output(completion, support_contributions))
}

/// The eighteen per-contribution invariants in the frozen first-failure
/// order. Grammar checks delegate to the four crate-private origin-binding
/// seams so each grammar has exactly one implementation source.
fn first_invariant_failure(
    admitted: &AdmittedContributionV0,
) -> Option<SupportContributionInvariantReasonV0> {
    use SupportContributionInvariantReasonV0 as Reason;

    if admitted.policy_id() != ADMITTED_CONTRIBUTION_POLICY_ID_V0 {
        return Some(Reason::AdmittedPolicyMismatch);
    }
    if admitted.origin_admission_policy_id() != ORIGIN_ADMISSION_POLICY_ID_V0 {
        return Some(Reason::OriginPolicyMismatch);
    }
    if admitted.evidence_kind() != EvidenceKind::ExternalSource.as_str() {
        return Some(Reason::EvidenceKindMismatch);
    }
    if admitted.claim_domain() != ClaimDomain::ExternalReport.as_str() {
        return Some(Reason::ClaimDomainMismatch);
    }
    if admitted.candidate_ceiling() != Status::Supported {
        return Some(Reason::CandidateCeilingMismatch);
    }

    let contribution = admitted.contribution();
    if !valid_origin_binding_replay_reference_v0(contribution.target_claim_id()) {
        return Some(Reason::InvalidTargetClaimReference);
    }
    if !valid_origin_binding_replay_reference_v0(contribution.source_evidence_id()) {
        return Some(Reason::InvalidSourceEvidenceReference);
    }
    if !valid_origin_binding_replay_reference_v0(contribution.justification_edge_id()) {
        return Some(Reason::InvalidJustificationEdgeReference);
    }
    if !valid_origin_binding_replay_reference_v0(contribution.scope_ref()) {
        return Some(Reason::InvalidScopeReference);
    }
    if contribution.artifact_algorithm() != ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0 {
        return Some(Reason::ArtifactAlgorithmMismatch);
    }
    if !valid_origin_binding_artifact_digest_v0(contribution.artifact_digest()) {
        return Some(Reason::InvalidArtifactDigest);
    }

    let namespace = admitted.namespace();
    if namespace.origin_admission_policy_id() != ORIGIN_ADMISSION_POLICY_ID_V0 {
        return Some(Reason::NamespacePolicyMismatch);
    }
    if namespace.target_claim_id() != contribution.target_claim_id() {
        return Some(Reason::NamespaceTargetMismatch);
    }
    if namespace.scope_ref() != contribution.scope_ref() {
        return Some(Reason::NamespaceScopeMismatch);
    }

    if !valid_origin_group_v0(admitted.origin_group()) {
        return Some(Reason::InvalidOriginGroup);
    }

    let selectors = admitted.supporting_origin_candidate_selectors();
    if selectors.is_empty() {
        return Some(Reason::MissingSupportingOriginSelector);
    }
    if selectors
        .iter()
        .any(|selector| !valid_origin_binding_selector_v0(selector))
    {
        return Some(Reason::InvalidSupportingOriginSelector);
    }
    if selectors.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Some(Reason::NonCanonicalSupportingOriginSelectorOrder);
    }

    None
}

/// Project one aligned, invariant-valid admitted contribution one-to-one.
/// The admitted `candidate_ceiling` becomes the support `support_ceiling`
/// only after the exact compiled ceiling and context cells were revalidated
/// globally (stages 7-8).
fn project_support_contribution_v0(admitted: &AdmittedContributionV0) -> SupportContributionV0 {
    SupportContributionV0 {
        policy_id: SUPPORT_CONTRIBUTION_POLICY_ID_V0.to_owned(),
        admitted_contribution_policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
        origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
        contribution: admitted.contribution().clone(),
        namespace: admitted.namespace().clone(),
        origin_group: admitted.origin_group().to_owned(),
        evidence_kind: admitted.evidence_kind().to_owned(),
        claim_domain: admitted.claim_domain().to_owned(),
        support_ceiling: admitted.candidate_ceiling(),
        supporting_origin_candidate_selectors: admitted
            .supporting_origin_candidate_selectors()
            .to_vec(),
    }
}

/// Top-level audit shell binding the expected current identities. A rejected
/// audit never copies a mismatched upstream identity into the new top-level
/// authority surface.
fn audit_shell(
    verified_prefix_identity: &VerifiedLogPrefixIdentityV0,
    closure_identity: &ResolutionContentClosureIdentityV0,
) -> SupportContributionAuditV0 {
    SupportContributionAuditV0 {
        schema: SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0.to_owned(),
        canonicalization_profile: SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0.to_owned(),
        policy_id: SUPPORT_CONTRIBUTION_POLICY_ID_V0.to_owned(),
        admitted_contribution_policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
        origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
        verified_prefix_identity: verified_prefix_identity.clone(),
        closure_identity: closure_identity.clone(),
        completion: SupportContributionAuditCompletionV0::Complete,
        support_contributions: Vec::new(),
    }
}

impl SupportContributionAuditV0 {
    fn with_output(
        mut self,
        completion: SupportContributionAuditCompletionV0,
        support_contributions: Vec<SupportContributionV0>,
    ) -> Self {
        self.completion = completion;
        self.support_contributions = support_contributions;
        self
    }
}

/// Resolve the deterministic support-contribution audit for this exact replay
/// context and closure.
///
/// The admitted-contribution audit is derived internally from the same
/// context and closure, decomposed through its public getters, and composed
/// through the single crate-private composition path. The resolver never
/// panics on composed output; every composition failure becomes a rejected
/// audit with zero support contributions.
pub(crate) fn resolve_support_contribution_audit_v0(
    context: &OriginAdmissionReplayContextV0,
    closure: &ResolutionContentClosureV0,
) -> SupportContributionAuditV0 {
    let admitted_audit = context.resolve_admitted_contribution_audit_v0(closure);
    let expected = ExpectedIdentitiesV0 {
        verified_prefix_identity: context.verified_prefix_identity().clone(),
        closure_identity: closure.identity().clone(),
        support_ceiling_cell: support_ceiling(
            EvidenceKind::ExternalSource,
            ClaimDomain::ExternalReport,
        ),
        support_context_requirement_cell: support_context_requirement(
            EvidenceKind::ExternalSource,
            ClaimDomain::ExternalReport,
        ),
    };
    let input = AdmittedAuditCompositionInputV0::from_audit(&admitted_audit);

    match compose_support_contribution_audit_v0(&expected, input) {
        Ok(audit) => audit,
        Err(failure) => audit_shell(
            &expected.verified_prefix_identity,
            &expected.closure_identity,
        )
        .with_output(
            SupportContributionAuditCompletionV0::UpstreamCompositionRejected { failure },
            Vec::new(),
        ),
    }
}

#[cfg(test)]
mod composition_tests {
    use super::*;
    use crate::admitted_contribution_audit::{
        stage_admitted_contribution_v0_for_tests, stage_contribution_identity_v0_for_tests,
        stage_namespace_v0_for_tests,
    };
    use crate::origin_admission_replay::replay_origin_admission_context_v0;
    use crate::origin_binding_verifier::{
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
    };
    use crate::resolution_content_closure::{
        ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0,
    };
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey};

    const SEED: [u8; 32] = [60; 32];

    fn replay_identity(notes: usize) -> VerifiedLogPrefixIdentityV0 {
        let store = MemStore::new();
        {
            let mut timestamp = 0u64;
            let mut writer = LogWriter::<MemStore>::open_with_clock(
                store.clone(),
                SigningKey::from_bytes(&SEED),
                Box::new(move || {
                    timestamp += 1;
                    timestamp
                }),
            )
            .unwrap();
            for index in 0..notes {
                writer
                    .append(
                        Provenance::new("support-composition-test", "identity"),
                        Payload::Note {
                            text: format!("identity note {index}"),
                        },
                    )
                    .unwrap();
            }
        }
        let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
        replay_origin_admission_context_v0(&reader)
            .unwrap()
            .verified_prefix_identity()
            .clone()
    }

    fn closure_identity() -> ResolutionContentClosureIdentityV0 {
        ResolutionContentClosureV0::construct(&[], &[])
            .unwrap()
            .identity()
            .clone()
    }

    fn different_closure_identity() -> ResolutionContentClosureIdentityV0 {
        let artifact = ResolutionArtifactObjectInputV0::new(
            ResolutionArtifactObjectKeyV0::new("sha256", "a".repeat(64)),
            b"different closure",
        );
        ResolutionContentClosureV0::construct(&[artifact], &[])
            .unwrap()
            .identity()
            .clone()
    }

    fn expected() -> ExpectedIdentitiesV0 {
        ExpectedIdentitiesV0 {
            verified_prefix_identity: replay_identity(0),
            closure_identity: closure_identity(),
            support_ceiling_cell: support_ceiling(
                EvidenceKind::ExternalSource,
                ClaimDomain::ExternalReport,
            ),
            support_context_requirement_cell: support_context_requirement(
                EvidenceKind::ExternalSource,
                ClaimDomain::ExternalReport,
            ),
        }
    }

    fn honest_input(expected: &ExpectedIdentitiesV0) -> AdmittedAuditCompositionInputV0 {
        AdmittedAuditCompositionInputV0 {
            schema: ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0.to_owned(),
            canonicalization_profile: ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
                .to_owned(),
            policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
            origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
            verified_prefix_identity: expected.verified_prefix_identity.clone(),
            closure_identity: expected.closure_identity.clone(),
            completion: AdmittedContributionAuditCompletionV0::Complete,
            candidate_audits: Vec::new(),
            admitted_contributions: Vec::new(),
        }
    }

    fn valid_selector(run_id: &str) -> ArtifactProvenanceAnchorSelectorV0 {
        ArtifactProvenanceAnchorSelectorV0::new(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            "a".repeat(64),
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            run_id,
        )
    }

    fn staged_admitted(edge_id: &str, origin_group: &str) -> AdmittedContributionV0 {
        staged_admitted_with_selectors(edge_id, origin_group, vec![valid_selector("run-alpha")])
    }

    fn staged_admitted_with_selectors(
        edge_id: &str,
        origin_group: &str,
        selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    ) -> AdmittedContributionV0 {
        stage_admitted_contribution_v0_for_tests(
            ADMITTED_CONTRIBUTION_POLICY_ID_V0,
            ORIGIN_ADMISSION_POLICY_ID_V0,
            stage_contribution_identity_v0_for_tests(
                "claim-shared",
                "evidence-shared",
                edge_id,
                "scope:shared",
                &"c".repeat(64),
            ),
            stage_namespace_v0_for_tests("claim-shared", "scope:shared"),
            origin_group,
            EvidenceKind::ExternalSource.as_str(),
            ClaimDomain::ExternalReport.as_str(),
            Status::Supported,
            selectors,
        )
    }

    fn aligned_candidate(
        edge_id: &str,
        admitted: &AdmittedContributionV0,
    ) -> AdmittedCandidateCompositionInputV0 {
        AdmittedCandidateCompositionInputV0 {
            edge_id: edge_id.to_owned(),
            source_evidence_id: admitted.contribution().source_evidence_id().to_owned(),
            target_claim_id: admitted.contribution().target_claim_id().to_owned(),
            evidence_kind: Some(admitted.evidence_kind().to_owned()),
            claim_domain: Some(admitted.claim_domain().to_owned()),
            candidate_ceiling: Some(admitted.candidate_ceiling()),
            candidate_contribution: Some(admitted.contribution().clone()),
            candidate_namespace: Some(admitted.namespace().clone()),
            disposition: AdmittedCandidateDispositionInputV0::Admitted {
                admitted_contribution: admitted.clone(),
            },
        }
    }

    fn bare_candidate(
        edge_id: &str,
        disposition: AdmittedCandidateDispositionInputV0,
    ) -> AdmittedCandidateCompositionInputV0 {
        AdmittedCandidateCompositionInputV0 {
            edge_id: edge_id.to_owned(),
            source_evidence_id: "evidence-x".to_owned(),
            target_claim_id: "claim-x".to_owned(),
            evidence_kind: None,
            claim_domain: None,
            candidate_ceiling: None,
            candidate_contribution: None,
            candidate_namespace: None,
            disposition,
        }
    }

    fn policy_ineligible_candidate(edge_id: &str) -> AdmittedCandidateCompositionInputV0 {
        bare_candidate(
            edge_id,
            AdmittedCandidateDispositionInputV0::PolicyIneligible {
                reason: AdmittedContributionCandidateReasonV0::MissingEvidenceContentHash,
            },
        )
    }

    fn origin_incomplete_candidate(edge_id: &str) -> AdmittedCandidateCompositionInputV0 {
        bare_candidate(
            edge_id,
            AdmittedCandidateDispositionInputV0::OriginAdmissionIncomplete,
        )
    }

    fn decision_absent_candidate(edge_id: &str) -> AdmittedCandidateCompositionInputV0 {
        bare_candidate(
            edge_id,
            AdmittedCandidateDispositionInputV0::OriginDecisionAbsent,
        )
    }

    fn not_admitted_candidate(
        edge_id: &str,
        admitted: &AdmittedContributionV0,
    ) -> AdmittedCandidateCompositionInputV0 {
        bare_candidate(
            edge_id,
            AdmittedCandidateDispositionInputV0::OriginNotAdmitted {
                decision: OriginAdmissionDecisionV0::EligibleBindingUnresolved {
                    policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                    contribution: admitted.contribution().clone(),
                    namespace: admitted.namespace().clone(),
                    matched_candidate_selectors: Vec::new(),
                    unresolved_candidate_selectors: vec![valid_selector("run-unresolved")],
                },
            },
        )
    }

    fn one_admitted_input(expected: &ExpectedIdentitiesV0) -> AdmittedAuditCompositionInputV0 {
        let admitted = staged_admitted("edge-a", "origin-group-alpha");
        let mut input = honest_input(expected);
        input.candidate_audits = vec![aligned_candidate("edge-a", &admitted)];
        input.admitted_contributions = vec![admitted];
        input
    }

    fn compose_failure(
        expected: &ExpectedIdentitiesV0,
        input: AdmittedAuditCompositionInputV0,
    ) -> SupportContributionCompositionFailureV0 {
        compose_support_contribution_audit_v0(expected, input).expect_err("composition must fail")
    }

    use SupportContributionCompositionFailureV0 as Failure;

    mod identity_and_policy_cells {
        use super::*;

        #[test]
        fn each_upstream_identity_mismatch_is_its_exact_failure() {
            let expected = expected();

            let mut input = honest_input(&expected);
            input.schema = "other-schema".into();
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamSchemaMismatch
            );

            let mut input = honest_input(&expected);
            input.canonicalization_profile = "other-profile".into();
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamCanonicalizationProfileMismatch
            );

            let mut input = honest_input(&expected);
            input.policy_id = "other-policy".into();
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamAdmittedPolicyMismatch
            );

            let mut input = honest_input(&expected);
            input.origin_admission_policy_id = "other-origin-policy".into();
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamOriginPolicyMismatch
            );

            let mut input = honest_input(&expected);
            input.verified_prefix_identity = replay_identity(1);
            assert_eq!(
                compose_failure(&expected, input),
                Failure::VerifiedPrefixIdentityMismatch
            );

            let mut input = honest_input(&expected);
            input.closure_identity = different_closure_identity();
            assert_eq!(
                compose_failure(&expected, input),
                Failure::ClosureIdentityMismatch
            );
        }

        #[test]
        fn compiled_cells_are_checked_even_when_both_admitted_maps_are_empty() {
            let mut drifted = expected();
            drifted.support_ceiling_cell = Some(Status::Settled);
            assert_eq!(
                compose_failure(&drifted, honest_input(&drifted)),
                Failure::SupportCeilingPolicyMismatch
            );

            let mut drifted = expected();
            drifted.support_ceiling_cell = None;
            assert_eq!(
                compose_failure(&drifted, honest_input(&drifted)),
                Failure::SupportCeilingPolicyMismatch
            );

            let mut drifted = expected();
            drifted.support_context_requirement_cell = SupportContextRequirement::HumanAdmission;
            assert_eq!(
                compose_failure(&drifted, honest_input(&drifted)),
                Failure::SupportContextRequirementMismatch
            );
        }

        #[test]
        fn policy_cell_rejection_precedes_completion_mapping() {
            let mut drifted = expected();
            drifted.support_ceiling_cell = Some(Status::Conjectured);
            let mut input = honest_input(&drifted);
            input.completion = AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
                unavailable_binding_selectors: vec![valid_selector("run-absent")],
            };
            input.candidate_audits = vec![origin_incomplete_candidate("edge-a")];
            assert_eq!(
                compose_failure(&drifted, input),
                Failure::SupportCeilingPolicyMismatch
            );
        }

        #[test]
        fn valid_complete_empty_maps_to_complete_with_zero_contributions() {
            let expected = expected();
            let audit = compose_support_contribution_audit_v0(&expected, honest_input(&expected))
                .expect("complete-empty composition is valid");
            assert_eq!(
                audit.completion(),
                &SupportContributionAuditCompletionV0::Complete
            );
            assert!(audit.support_contributions().is_empty());
            assert_eq!(
                audit.verified_prefix_identity(),
                &expected.verified_prefix_identity
            );
            assert_eq!(audit.closure_identity(), &expected.closure_identity);
        }

        #[test]
        fn origin_incomplete_maps_to_incomplete_inheriting_exact_selector_order() {
            let expected = expected();
            let mut input = honest_input(&expected);
            let selectors = vec![valid_selector("run-beta"), valid_selector("run-alpha")];
            input.completion = AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
                unavailable_binding_selectors: selectors.clone(),
            };
            input.candidate_audits = vec![
                origin_incomplete_candidate("edge-a"),
                policy_ineligible_candidate("edge-b"),
            ];
            let audit = compose_support_contribution_audit_v0(&expected, input)
                .expect("valid incomplete composition maps to incomplete");
            match audit.completion() {
                SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                    unavailable_binding_selectors,
                } => assert_eq!(unavailable_binding_selectors, &selectors),
                other => panic!("expected incomplete completion, got {other:?}"),
            }
            assert!(audit.support_contributions().is_empty());
        }

        #[test]
        fn origin_incomplete_does_not_hide_support_context_drift() {
            let mut drifted = expected();
            drifted.support_context_requirement_cell =
                SupportContextRequirement::DeterministicVerifierContext;
            let mut input = honest_input(&drifted);
            input.completion = AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
                unavailable_binding_selectors: vec![valid_selector("run-absent")],
            };
            assert_eq!(
                compose_failure(&drifted, input),
                Failure::SupportContextRequirementMismatch
            );
        }
    }

    mod edge_id_uniqueness {
        use super::*;

        #[test]
        fn two_admitted_candidates_sharing_an_edge_id_are_rejected() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-alpha");
            let admitted_b = stage_admitted_contribution_v0_for_tests(
                ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                ORIGIN_ADMISSION_POLICY_ID_V0,
                stage_contribution_identity_v0_for_tests(
                    "claim-other",
                    "evidence-other",
                    "edge-a",
                    "scope:other",
                    &"d".repeat(64),
                ),
                stage_namespace_v0_for_tests("claim-other", "scope:other"),
                "origin-group-beta",
                EvidenceKind::ExternalSource.as_str(),
                ClaimDomain::ExternalReport.as_str(),
                Status::Supported,
                vec![valid_selector("run-beta")],
            );
            assert_ne!(admitted_a.contribution(), admitted_b.contribution());
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted_a),
                aligned_candidate("edge-a", &admitted_b),
            ];
            input.admitted_contributions = vec![admitted_a, admitted_b];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn duplicate_edge_ids_among_non_admitted_candidates_are_rejected() {
            let expected = expected();
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                policy_ineligible_candidate("edge-a"),
                decision_absent_candidate("edge-a"),
            ];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn one_admitted_and_one_non_admitted_sharing_an_edge_id_are_rejected() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted),
                policy_ineligible_candidate("edge-a"),
            ];
            input.admitted_contributions = vec![admitted];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn multiple_duplicated_edge_ids_report_the_lexically_first() {
            let expected = expected();
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                policy_ineligible_candidate("edge-z"),
                policy_ineligible_candidate("edge-a"),
                policy_ineligible_candidate("edge-z"),
                policy_ineligible_candidate("edge-a"),
            ];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn duplicate_edge_id_outranks_trace_mismatch() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut trace_broken = aligned_candidate("edge-b", &admitted);
            trace_broken.edge_id = "edge-b".into();
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                trace_broken,
                policy_ineligible_candidate("edge-c"),
                policy_ineligible_candidate("edge-c"),
            ];
            input.admitted_contributions = vec![admitted];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-c".into()
                }
            );
        }

        #[test]
        fn duplicate_edge_id_is_detected_before_candidate_map_insertion() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted),
                aligned_candidate("edge-a", &admitted),
            ];
            input.admitted_contributions = vec![admitted];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );
        }
    }

    mod admitted_candidate_trace {
        use super::*;

        fn trace_broken_input(
            mutate: impl FnOnce(&mut AdmittedCandidateCompositionInputV0),
        ) -> (ExpectedIdentitiesV0, AdmittedAuditCompositionInputV0) {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut candidate = aligned_candidate("edge-a", &admitted);
            mutate(&mut candidate);
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![candidate];
            input.admitted_contributions = vec![admitted];
            (expected, input)
        }

        #[test]
        fn each_of_the_eight_trace_fields_is_checked() {
            let admitted = staged_admitted("edge-a", "origin-group-alpha");

            // Field 1 is special: mutating it changes the candidate's own
            // edge ID, which is also the reported identity.
            let (expected, input) =
                trace_broken_input(|candidate| candidate.edge_id = "edge-drifted".into());
            assert_eq!(
                compose_failure(&expected, input),
                Failure::AdmittedCandidateTraceMismatch {
                    edge_id: "edge-drifted".into()
                }
            );

            let cases: Vec<fn(&mut AdmittedCandidateCompositionInputV0)> = vec![
                |candidate| candidate.source_evidence_id = "evidence-drifted".into(),
                |candidate| candidate.target_claim_id = "claim-drifted".into(),
                |candidate| candidate.evidence_kind = Some("ExecutionEvidence".into()),
                |candidate| candidate.claim_domain = Some("Interpretation".into()),
                |candidate| candidate.candidate_ceiling = Some(Status::Settled),
                |candidate| {
                    candidate.candidate_contribution =
                        Some(stage_contribution_identity_v0_for_tests(
                            "claim-shared",
                            "evidence-shared",
                            "edge-a",
                            "scope:shared",
                            &"e".repeat(64),
                        ))
                },
                |candidate| {
                    candidate.candidate_namespace = Some(stage_namespace_v0_for_tests(
                        "claim-drifted",
                        "scope:shared",
                    ))
                },
            ];
            for mutate in cases {
                let (expected, input) = trace_broken_input(mutate);
                assert_eq!(
                    compose_failure(&expected, input),
                    Failure::AdmittedCandidateTraceMismatch {
                        edge_id: "edge-a".into()
                    }
                );
            }

            let mut missing = aligned_candidate("edge-a", &admitted);
            missing.candidate_contribution = None;
            let (expected, input) = trace_broken_input(|candidate| *candidate = missing);
            assert_eq!(
                compose_failure(&expected, input),
                Failure::AdmittedCandidateTraceMismatch {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn first_malformed_candidate_is_reported_in_exact_edge_id_order() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-alpha");
            let admitted_b = staged_admitted("edge-b", "origin-group-alpha");
            let mut broken_a = aligned_candidate("edge-a", &admitted_a);
            broken_a.source_evidence_id = "drifted".into();
            let mut broken_b = aligned_candidate("edge-b", &admitted_b);
            broken_b.source_evidence_id = "drifted".into();
            let mut input = honest_input(&expected);
            // Vector order must not select the outcome.
            input.candidate_audits = vec![broken_b, broken_a];
            input.admitted_contributions = vec![admitted_a, admitted_b];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::AdmittedCandidateTraceMismatch {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn trace_mismatch_is_detected_before_top_level_duplicate_detection() {
            // A trace-mismatched admitted candidate plus a duplicated
            // top-level identity: stage 10 precedes every map stage.
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut broken = aligned_candidate("edge-a", &admitted);
            broken.claim_domain = Some("Interpretation".into());
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![broken];
            input.admitted_contributions = vec![admitted.clone(), admitted];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::AdmittedCandidateTraceMismatch {
                    edge_id: "edge-a".into()
                }
            );
        }
    }

    mod admitted_alignment {
        use super::*;

        #[test]
        fn repeated_candidate_identity_is_globally_rejected_before_insertion() {
            // `DuplicateCandidateAdmission` guards stage 11, but a repeated
            // candidate identity cannot reach it through valid earlier
            // stages: trace alignment binds each candidate edge ID to the
            // identity's `justification_edge_id`, so two trace-aligned
            // candidates carrying one identity necessarily share an edge ID
            // (stage 9), and carrying it under a second edge ID is a trace
            // mismatch (stage 10). The stage-11 check remains as
            // defense-in-depth; both constructible routes reject globally.
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");

            let mut same_edge = honest_input(&expected);
            same_edge.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted),
                aligned_candidate("edge-a", &admitted),
            ];
            same_edge.admitted_contributions = vec![admitted.clone()];
            assert_eq!(
                compose_failure(&expected, same_edge),
                Failure::DuplicateCandidateEdgeId {
                    edge_id: "edge-a".into()
                }
            );

            let mut cross_edge = honest_input(&expected);
            cross_edge.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted),
                aligned_candidate("edge-b", &admitted),
            ];
            cross_edge.admitted_contributions = vec![admitted.clone()];
            assert_eq!(
                compose_failure(&expected, cross_edge),
                Failure::AdmittedCandidateTraceMismatch {
                    edge_id: "edge-b".into()
                }
            );
        }

        #[test]
        fn duplicate_top_level_admission_is_rejected() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![aligned_candidate("edge-a", &admitted)];
            input.admitted_contributions = vec![admitted.clone(), admitted];
            assert!(matches!(
                compose_failure(&expected, input),
                Failure::DuplicateTopLevelAdmission { .. }
            ));
        }

        #[test]
        fn candidate_only_identity_reports_exact_set_difference() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![aligned_candidate("edge-a", &admitted)];
            match compose_failure(&expected, input) {
                Failure::AdmittedSetMismatch {
                    candidate_only,
                    top_level_only,
                } => {
                    assert_eq!(candidate_only, vec![admitted.contribution().clone()]);
                    assert!(top_level_only.is_empty());
                }
                other => panic!("expected admitted set mismatch, got {other:?}"),
            }
        }

        #[test]
        fn top_level_only_identity_reports_exact_set_difference() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let mut input = honest_input(&expected);
            input.admitted_contributions = vec![admitted.clone()];
            match compose_failure(&expected, input) {
                Failure::AdmittedSetMismatch {
                    candidate_only,
                    top_level_only,
                } => {
                    assert!(candidate_only.is_empty());
                    assert_eq!(top_level_only, vec![admitted.contribution().clone()]);
                }
                other => panic!("expected admitted set mismatch, got {other:?}"),
            }
        }

        #[test]
        fn same_identity_with_different_value_reports_first_in_identity_order() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-alpha");
            let admitted_b = staged_admitted("edge-b", "origin-group-alpha");
            let drifted_b = stage_admitted_contribution_v0_for_tests(
                ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                ORIGIN_ADMISSION_POLICY_ID_V0,
                admitted_b.contribution().clone(),
                admitted_b.namespace().clone(),
                "origin-group-drifted",
                EvidenceKind::ExternalSource.as_str(),
                ClaimDomain::ExternalReport.as_str(),
                Status::Supported,
                vec![valid_selector("run-alpha")],
            );
            let drifted_a = stage_admitted_contribution_v0_for_tests(
                ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                ORIGIN_ADMISSION_POLICY_ID_V0,
                admitted_a.contribution().clone(),
                admitted_a.namespace().clone(),
                "origin-group-drifted",
                EvidenceKind::ExternalSource.as_str(),
                ClaimDomain::ExternalReport.as_str(),
                Status::Supported,
                vec![valid_selector("run-alpha")],
            );
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted_a),
                aligned_candidate("edge-b", &admitted_b),
            ];
            input.admitted_contributions = vec![drifted_a, drifted_b];
            let expected_first = std::cmp::min(
                admitted_a.contribution().clone(),
                admitted_b.contribution().clone(),
            );
            assert_eq!(
                compose_failure(&expected, input),
                Failure::AdmittedValueMismatch {
                    contribution: expected_first
                }
            );
        }
    }

    mod completion_shape_and_disposition {
        use super::*;

        fn incomplete_input(
            expected: &ExpectedIdentitiesV0,
            selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
            candidates: Vec<AdmittedCandidateCompositionInputV0>,
        ) -> AdmittedAuditCompositionInputV0 {
            let mut input = honest_input(expected);
            input.completion = AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
                unavailable_binding_selectors: selectors,
            };
            input.candidate_audits = candidates;
            input
        }

        #[test]
        fn complete_forbids_origin_incomplete_candidates() {
            let expected = expected();
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![origin_incomplete_candidate("edge-a")];
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamCompletionDispositionMismatch {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn incomplete_forbids_terminal_dispositions_and_admissions() {
            let admitted = staged_admitted("edge-c", "origin-group-alpha");
            for candidate in [
                decision_absent_candidate("edge-a"),
                not_admitted_candidate("edge-b", &admitted),
                aligned_candidate("edge-c", &admitted),
            ] {
                let expected = expected();
                let edge_id = candidate.edge_id.clone();
                let mut input = incomplete_input(
                    &expected,
                    vec![valid_selector("run-absent")],
                    vec![candidate],
                );
                if edge_id == "edge-c" {
                    input.admitted_contributions = vec![admitted.clone()];
                }
                assert_eq!(
                    compose_failure(&expected, input),
                    Failure::UpstreamCompletionDispositionMismatch { edge_id }
                );
            }
        }

        #[test]
        fn first_inconsistent_candidate_is_reported_in_exact_edge_id_order() {
            let expected = expected();
            let input = incomplete_input(
                &expected,
                vec![valid_selector("run-absent")],
                vec![
                    decision_absent_candidate("edge-b"),
                    decision_absent_candidate("edge-a"),
                ],
            );
            assert_eq!(
                compose_failure(&expected, input),
                Failure::UpstreamCompletionDispositionMismatch {
                    edge_id: "edge-a".into()
                }
            );
        }

        #[test]
        fn incomplete_with_only_permitted_dispositions_is_accepted() {
            let expected = expected();
            let input = incomplete_input(
                &expected,
                vec![valid_selector("run-absent")],
                vec![
                    origin_incomplete_candidate("edge-a"),
                    policy_ineligible_candidate("edge-b"),
                ],
            );
            let audit = compose_support_contribution_audit_v0(&expected, input)
                .expect("permitted dispositions map to incomplete");
            assert!(matches!(
                audit.completion(),
                SupportContributionAuditCompletionV0::AdmittedContributionIncomplete { .. }
            ));
        }

        #[test]
        fn empty_vector_incomplete_is_rejected_even_with_only_policy_ineligible() {
            let expected = expected();
            let input = incomplete_input(
                &expected,
                Vec::new(),
                vec![policy_ineligible_candidate("edge-a")],
            );
            assert_eq!(
                compose_failure(&expected, input),
                Failure::EmptyUnavailableBindingSelectors
            );
        }

        #[test]
        fn empty_vector_incomplete_is_rejected_with_an_empty_candidate_universe() {
            let expected = expected();
            let input = incomplete_input(&expected, Vec::new(), Vec::new());
            assert_eq!(
                compose_failure(&expected, input),
                Failure::EmptyUnavailableBindingSelectors
            );
        }

        #[test]
        fn empty_vector_incomplete_outranks_any_disposition_violation() {
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let expected = expected();
            let input = incomplete_input(
                &expected,
                Vec::new(),
                vec![not_admitted_candidate("edge-a", &admitted)],
            );
            assert_eq!(
                compose_failure(&expected, input),
                Failure::EmptyUnavailableBindingSelectors
            );
        }

        #[test]
        fn non_empty_vector_incomplete_with_only_policy_ineligible_is_accepted() {
            let expected = expected();
            let input = incomplete_input(
                &expected,
                vec![valid_selector("run-absent")],
                vec![policy_ineligible_candidate("edge-a")],
            );
            let audit = compose_support_contribution_audit_v0(&expected, input)
                .expect("non-empty incomplete with policy-ineligible candidates is valid");
            assert!(matches!(
                audit.completion(),
                SupportContributionAuditCompletionV0::AdmittedContributionIncomplete { .. }
            ));
            assert!(audit.support_contributions().is_empty());
        }
    }

    mod projection {
        use super::*;

        #[test]
        fn one_aligned_admitted_contribution_projects_one_support_contribution() {
            let expected = expected();
            let admitted = staged_admitted("edge-a", "origin-group-alpha");
            let audit =
                compose_support_contribution_audit_v0(&expected, one_admitted_input(&expected))
                    .expect("one aligned admitted contribution composes");
            assert_eq!(
                audit.completion(),
                &SupportContributionAuditCompletionV0::Complete
            );
            assert_eq!(audit.support_contributions().len(), 1);

            let support = &audit.support_contributions()[0];
            assert_eq!(support.policy_id(), SUPPORT_CONTRIBUTION_POLICY_ID_V0);
            assert_eq!(
                support.admitted_contribution_policy_id(),
                ADMITTED_CONTRIBUTION_POLICY_ID_V0
            );
            assert_eq!(
                support.origin_admission_policy_id(),
                ORIGIN_ADMISSION_POLICY_ID_V0
            );
            assert_eq!(support.contribution(), admitted.contribution());
            assert_eq!(support.namespace(), admitted.namespace());
            assert_eq!(support.origin_group(), "origin-group-alpha");
            assert_eq!(
                support.evidence_kind(),
                EvidenceKind::ExternalSource.as_str()
            );
            assert_eq!(support.claim_domain(), ClaimDomain::ExternalReport.as_str());
            assert_eq!(support.support_ceiling(), Status::Supported);
            assert_eq!(
                support.supporting_origin_candidate_selectors(),
                admitted.supporting_origin_candidate_selectors()
            );
        }

        #[test]
        fn same_origin_distinct_contributions_remain_two_support_inputs() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-shared");
            let admitted_b = staged_admitted("edge-b", "origin-group-shared");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-b", &admitted_b),
                aligned_candidate("edge-a", &admitted_a),
            ];
            input.admitted_contributions = vec![admitted_b.clone(), admitted_a.clone()];
            let audit = compose_support_contribution_audit_v0(&expected, input)
                .expect("two aligned admitted contributions compose");
            assert_eq!(audit.support_contributions().len(), 2);
            assert!(audit
                .support_contributions()
                .iter()
                .all(|support| support.origin_group() == "origin-group-shared"));
            // Exact `ContributionIdentityV0` order, not vector order.
            let first = std::cmp::min(admitted_a.contribution(), admitted_b.contribution());
            assert_eq!(audit.support_contributions()[0].contribution(), first);
        }

        #[test]
        fn distinct_origin_contributions_remain_two_support_inputs() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-alpha");
            let admitted_b = staged_admitted("edge-b", "origin-group-beta");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted_a),
                aligned_candidate("edge-b", &admitted_b),
            ];
            input.admitted_contributions = vec![admitted_a, admitted_b];
            let audit = compose_support_contribution_audit_v0(&expected, input)
                .expect("distinct origins compose");
            assert_eq!(audit.support_contributions().len(), 2);
            let groups: Vec<&str> = audit
                .support_contributions()
                .iter()
                .map(SupportContributionV0::origin_group)
                .collect();
            assert!(groups.contains(&"origin-group-alpha"));
            assert!(groups.contains(&"origin-group-beta"));
        }

        #[test]
        fn composition_is_deterministic_under_vector_permutation() {
            let expected = expected();
            let admitted_a = staged_admitted("edge-a", "origin-group-alpha");
            let admitted_b = staged_admitted("edge-b", "origin-group-beta");
            let mut forward = honest_input(&expected);
            forward.candidate_audits = vec![
                aligned_candidate("edge-a", &admitted_a),
                aligned_candidate("edge-b", &admitted_b),
                policy_ineligible_candidate("edge-c"),
            ];
            forward.admitted_contributions = vec![admitted_a.clone(), admitted_b.clone()];

            let mut permuted = honest_input(&expected);
            permuted.candidate_audits = vec![
                policy_ineligible_candidate("edge-c"),
                aligned_candidate("edge-b", &admitted_b),
                aligned_candidate("edge-a", &admitted_a),
            ];
            permuted.admitted_contributions = vec![admitted_b.clone(), admitted_a.clone()];

            let first = compose_support_contribution_audit_v0(&expected, forward)
                .expect("forward composition succeeds");
            let second = compose_support_contribution_audit_v0(&expected, permuted)
                .expect("permuted composition succeeds");
            assert_eq!(first, second);
            assert_eq!(first.canonical_bytes(), second.canonical_bytes());
        }
    }

    mod invariants {
        use super::*;
        use crate::origin_binding_verifier::ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0;

        use SupportContributionInvariantReasonV0 as Reason;

        // Three frozen checks are unreachable-by-construction through every
        // permitted staging path, because the pinned upstream constructors
        // cannot produce their drifted inputs: `ArtifactAlgorithmMismatch`
        // (the identity constructor pins sha256), `NamespacePolicyMismatch`
        // (the namespace constructor pins the origin-admission policy), and
        // `DuplicateCandidateAdmission` (trace alignment binds candidate edge
        // IDs to identity edge IDs, so a repeated candidate identity is
        // always caught at stage 9 or 10 first). They remain in the closed
        // vocabularies as defense-in-depth; the contract forbids widening the
        // origin-binding module to stage them.
        //
        // Every other frozen reason is staged and asserted below.

        fn invariant_failure(admitted: AdmittedContributionV0) -> Reason {
            let expected = expected();
            let mut input = honest_input(&expected);
            // The candidate must mirror the (possibly drifted) identity edge
            // ID so earlier trace alignment still passes.
            let edge_id = admitted.contribution().justification_edge_id().to_owned();
            input.candidate_audits = vec![aligned_candidate(&edge_id, &admitted)];
            input.admitted_contributions = vec![admitted.clone()];
            match compose_failure(&expected, input) {
                Failure::ContributionInvariantMismatch { reason, .. } => reason,
                other => panic!("expected invariant mismatch, got {other:?}"),
            }
        }

        fn invariant_valid(admitted: AdmittedContributionV0) {
            let expected = expected();
            let mut input = honest_input(&expected);
            let edge_id = admitted.contribution().justification_edge_id().to_owned();
            input.candidate_audits = vec![aligned_candidate(&edge_id, &admitted)];
            input.admitted_contributions = vec![admitted.clone()];
            compose_support_contribution_audit_v0(&expected, input)
                .expect("boundary-valid contribution composes");
        }

        fn identity_with(
            target: &str,
            evidence: &str,
            edge: &str,
            scope: &str,
            digest: &str,
        ) -> ContributionIdentityV0 {
            stage_contribution_identity_v0_for_tests(target, evidence, edge, scope, digest)
        }

        fn admitted_with_identity(
            contribution: ContributionIdentityV0,
            namespace: OriginComparisonNamespaceV0,
        ) -> AdmittedContributionV0 {
            stage_admitted_contribution_v0_for_tests(
                ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                ORIGIN_ADMISSION_POLICY_ID_V0,
                contribution,
                namespace,
                "origin-group-alpha",
                EvidenceKind::ExternalSource.as_str(),
                ClaimDomain::ExternalReport.as_str(),
                Status::Supported,
                vec![valid_selector("run-alpha")],
            )
        }

        #[test]
        fn policy_lane_invariants_are_checked_in_order() {
            let base = staged_admitted("edge-a", "origin-group-alpha");
            let drifted =
                |policy: &str, origin: &str, kind: &str, domain: &str, ceiling: Status| {
                    stage_admitted_contribution_v0_for_tests(
                        policy,
                        origin,
                        base.contribution().clone(),
                        base.namespace().clone(),
                        "origin-group-alpha",
                        kind,
                        domain,
                        ceiling,
                        vec![valid_selector("run-alpha")],
                    )
                };
            assert_eq!(
                invariant_failure(drifted(
                    "magpie-admitted-contribution-v1",
                    ORIGIN_ADMISSION_POLICY_ID_V0,
                    "ExternalSource",
                    "ExternalReport",
                    Status::Supported,
                )),
                Reason::AdmittedPolicyMismatch
            );
            assert_eq!(
                invariant_failure(drifted(
                    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                    "magpie-origin-admission-v1",
                    "ExternalSource",
                    "ExternalReport",
                    Status::Supported,
                )),
                Reason::OriginPolicyMismatch
            );
            assert_eq!(
                invariant_failure(drifted(
                    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                    ORIGIN_ADMISSION_POLICY_ID_V0,
                    "ExecutionEvidence",
                    "ExternalReport",
                    Status::Supported,
                )),
                Reason::EvidenceKindMismatch
            );
            assert_eq!(
                invariant_failure(drifted(
                    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                    ORIGIN_ADMISSION_POLICY_ID_V0,
                    "ExternalSource",
                    "Interpretation",
                    Status::Supported,
                )),
                Reason::ClaimDomainMismatch
            );
            assert_eq!(
                invariant_failure(drifted(
                    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                    ORIGIN_ADMISSION_POLICY_ID_V0,
                    "ExternalSource",
                    "ExternalReport",
                    Status::Settled,
                )),
                Reason::CandidateCeilingMismatch
            );
        }

        #[test]
        fn replay_reference_boundaries_are_exact_for_all_four_fields() {
            let digest = "c".repeat(64);
            let cases: [(&str, &str, &str, &str, Reason); 4] = [
                (
                    "",
                    "evidence",
                    "edge-a",
                    "scope",
                    Reason::InvalidTargetClaimReference,
                ),
                (
                    "claim",
                    "",
                    "edge-a",
                    "scope",
                    Reason::InvalidSourceEvidenceReference,
                ),
                (
                    "claim",
                    "evidence",
                    "",
                    "scope",
                    Reason::InvalidJustificationEdgeReference,
                ),
                (
                    "claim",
                    "evidence",
                    "edge-a",
                    "",
                    Reason::InvalidScopeReference,
                ),
            ];
            for (target, evidence, edge, scope, reason) in cases {
                let namespace = stage_namespace_v0_for_tests(target, scope);
                assert_eq!(
                    invariant_failure(admitted_with_identity(
                        identity_with(target, evidence, edge, scope, &digest),
                        namespace,
                    )),
                    reason
                );
            }

            let long = "x".repeat(1_025);
            let namespace = stage_namespace_v0_for_tests(&long, "scope");
            assert_eq!(
                invariant_failure(admitted_with_identity(
                    identity_with(&long, "evidence", "edge-a", "scope", &digest),
                    namespace,
                )),
                Reason::InvalidTargetClaimReference
            );

            // One byte, exactly 1,024 bytes, and non-ASCII UTF-8 within the
            // byte bound are all valid; a multibyte string whose byte length
            // exceeds 1,024 is invalid.
            for target in ["x", &"x".repeat(1_024), "clé"] {
                let namespace = stage_namespace_v0_for_tests(target, "scope");
                invariant_valid(admitted_with_identity(
                    identity_with(target, "evidence", "edge-a", "scope", &digest),
                    namespace,
                ));
            }
            let multibyte = "é".repeat(600);
            assert!(multibyte.len() > 1_024);
            let namespace = stage_namespace_v0_for_tests(&multibyte, "scope");
            assert_eq!(
                invariant_failure(admitted_with_identity(
                    identity_with(&multibyte, "evidence", "edge-a", "scope", &digest),
                    namespace,
                )),
                Reason::InvalidTargetClaimReference
            );
        }

        #[test]
        fn equal_malformed_contribution_and_namespace_values_are_still_rejected() {
            let digest = "c".repeat(64);
            let namespace = stage_namespace_v0_for_tests("", "");
            assert_eq!(
                invariant_failure(admitted_with_identity(
                    identity_with("", "evidence", "edge-a", "", &digest),
                    namespace,
                )),
                Reason::InvalidTargetClaimReference
            );
        }

        #[test]
        fn replay_reference_failures_outrank_artifact_and_namespace_failures() {
            // Empty target reference plus uppercase digest plus namespace
            // target/scope drift: the reference reason wins by precedence.
            let namespace = stage_namespace_v0_for_tests("claim-drifted", "scope-drifted");
            assert_eq!(
                invariant_failure(admitted_with_identity(
                    identity_with("", "evidence", "edge-a", "scope", &"C".repeat(64)),
                    namespace,
                )),
                Reason::InvalidTargetClaimReference
            );
        }

        #[test]
        fn artifact_digest_must_be_exact_lowercase_hex_64() {
            let namespace = stage_namespace_v0_for_tests("claim", "scope");
            let mixed_case = format!("{}{}", "a".repeat(32), "B".repeat(32));
            let prefixed = format!("0x{}", "a".repeat(62));
            let whitespace = format!("{} ", "a".repeat(63));
            for digest in [
                "",
                &"a".repeat(63),
                &"a".repeat(65),
                &"A".repeat(64),
                &mixed_case,
                &"g".repeat(64),
                &prefixed,
                &whitespace,
            ] {
                assert_eq!(
                    invariant_failure(admitted_with_identity(
                        identity_with("claim", "evidence", "edge-a", "scope", digest),
                        namespace.clone(),
                    )),
                    Reason::InvalidArtifactDigest,
                    "digest {digest:?} must be rejected"
                );
            }
            invariant_valid(admitted_with_identity(
                identity_with("claim", "evidence", "edge-a", "scope", &"f".repeat(64)),
                namespace.clone(),
            ));
        }

        #[test]
        fn namespace_target_and_scope_equality_is_independently_mandatory() {
            let base = staged_admitted("edge-a", "origin-group-alpha");
            let drifted = |namespace: OriginComparisonNamespaceV0| {
                stage_admitted_contribution_v0_for_tests(
                    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
                    ORIGIN_ADMISSION_POLICY_ID_V0,
                    base.contribution().clone(),
                    namespace,
                    "origin-group-alpha",
                    EvidenceKind::ExternalSource.as_str(),
                    ClaimDomain::ExternalReport.as_str(),
                    Status::Supported,
                    vec![valid_selector("run-alpha")],
                )
            };
            assert_eq!(
                invariant_failure(drifted(stage_namespace_v0_for_tests(
                    "claim-other",
                    "scope:shared",
                ))),
                Reason::NamespaceTargetMismatch
            );
            assert_eq!(
                invariant_failure(drifted(stage_namespace_v0_for_tests(
                    "claim-shared",
                    "scope:other",
                ))),
                Reason::NamespaceScopeMismatch
            );
        }

        #[test]
        fn origin_group_grammar_boundaries_are_exact() {
            let long = "g".repeat(129);
            for group in [
                "",
                long.as_str(),
                "gröupe",
                "-starts-with-punctuation",
                "has whitespace",
                "has@unsupported",
            ] {
                assert_eq!(
                    invariant_failure(staged_admitted("edge-a", group)),
                    Reason::InvalidOriginGroup,
                    "group {group:?} must be rejected"
                );
            }
            invariant_valid(staged_admitted("edge-a", "g"));
            invariant_valid(staged_admitted("edge-a", &"g".repeat(128)));
            invariant_valid(staged_admitted("edge-a", "g0._:/-ok"));
        }

        fn selector_with(
            kind: &str,
            root: &str,
            algorithm: &str,
            profile: &str,
            run_id: &str,
        ) -> ArtifactProvenanceAnchorSelectorV0 {
            ArtifactProvenanceAnchorSelectorV0::new(kind, root, algorithm, profile, run_id)
        }

        fn admitted_with_selectors(
            selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
        ) -> AdmittedContributionV0 {
            staged_admitted_with_selectors("edge-a", "origin-group-alpha", selectors)
        }

        #[test]
        fn an_empty_supporting_selector_vector_is_missing_not_invalid() {
            assert_eq!(
                invariant_failure(admitted_with_selectors(Vec::new())),
                Reason::MissingSupportingOriginSelector
            );
        }

        #[test]
        fn selector_bundle_kinds_are_exactly_the_two_origin_binding_kinds() {
            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![selector_with(
                    "magpie-other-bundle-v0",
                    &"a".repeat(64),
                    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                    ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                    "run-alpha",
                )])),
                Reason::InvalidSupportingOriginSelector
            );
            // The inner artifact-provenance acquisition kind is not the
            // outer origin-binding acquisition kind.
            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![selector_with(
                    "magpie-artifact-acquisition-v0",
                    &"a".repeat(64),
                    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                    ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                    "run-alpha",
                )])),
                Reason::InvalidSupportingOriginSelector
            );
            invariant_valid(admitted_with_selectors(vec![valid_selector("run-alpha")]));
            invariant_valid(admitted_with_selectors(vec![selector_with(
                ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
                &"b".repeat(64),
                ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                "run-alpha",
            )]));
        }

        #[test]
        fn selector_witness_fields_are_exact() {
            for root in [
                "",
                &"a".repeat(63),
                &"a".repeat(65),
                &"A".repeat(64),
                &"z".repeat(64),
            ] {
                assert_eq!(
                    invariant_failure(admitted_with_selectors(vec![selector_with(
                        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                        root,
                        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                        "run-alpha",
                    )])),
                    Reason::InvalidSupportingOriginSelector,
                    "witness root {root:?} must be rejected"
                );
            }
            for (algorithm, profile) in [
                ("sha512", ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0),
                (ORIGIN_BINDING_WITNESS_ALGORITHM_V0, "other-profile-v0"),
            ] {
                assert_eq!(
                    invariant_failure(admitted_with_selectors(vec![selector_with(
                        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                        &"a".repeat(64),
                        algorithm,
                        profile,
                        "run-alpha",
                    )])),
                    Reason::InvalidSupportingOriginSelector
                );
            }
        }

        #[test]
        fn selector_run_id_grammar_boundaries_are_exact() {
            let long = "r".repeat(129);
            for run_id in [
                "",
                long.as_str(),
                ".punctuation-first",
                "has whitespace",
                "has@unsupported",
                "rön-id",
            ] {
                assert_eq!(
                    invariant_failure(admitted_with_selectors(vec![selector_with(
                        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                        &"a".repeat(64),
                        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                        run_id,
                    )])),
                    Reason::InvalidSupportingOriginSelector,
                    "run ID {run_id:?} must be rejected"
                );
            }
            for run_id in ["r", &"r".repeat(128), "r0._:/-ok"] {
                invariant_valid(admitted_with_selectors(vec![selector_with(
                    ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                    &"a".repeat(64),
                    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                    ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                    run_id,
                )]));
            }
        }

        #[test]
        fn selector_vector_must_be_strictly_increasing_without_duplicates() {
            let alpha = valid_selector("run-alpha");
            let beta = valid_selector("run-beta");
            invariant_valid(admitted_with_selectors(vec![alpha.clone()]));
            invariant_valid(admitted_with_selectors(vec![alpha.clone(), beta.clone()]));

            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![alpha.clone(), alpha.clone()])),
                Reason::NonCanonicalSupportingOriginSelectorOrder
            );
            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![beta.clone(), alpha.clone()])),
                Reason::NonCanonicalSupportingOriginSelectorOrder
            );
        }

        #[test]
        fn individual_selector_validity_precedes_vector_canonicality() {
            let invalid = selector_with(
                "magpie-other-bundle-v0",
                &"a".repeat(64),
                ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                "run-alpha",
            );
            // An individually invalid selector in an otherwise ordered
            // vector reports the invalid selector.
            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![
                    valid_selector("run-alpha"),
                    invalid.clone(),
                ])),
                Reason::InvalidSupportingOriginSelector
            );
            // An individually invalid selector in a non-canonical vector
            // still reports the invalid selector, not the order failure.
            assert_eq!(
                invariant_failure(admitted_with_selectors(vec![
                    valid_selector("run-beta"),
                    valid_selector("run-alpha"),
                    invalid,
                ])),
                Reason::InvalidSupportingOriginSelector
            );
        }

        #[test]
        fn first_failing_contribution_is_reported_in_identity_order() {
            let expected = expected();
            let bad_a = staged_admitted("edge-a", "");
            let bad_b = staged_admitted("edge-b", "");
            let mut input = honest_input(&expected);
            input.candidate_audits = vec![
                aligned_candidate("edge-b", &bad_b),
                aligned_candidate("edge-a", &bad_a),
            ];
            input.admitted_contributions = vec![bad_b.clone(), bad_a.clone()];
            let expected_first =
                std::cmp::min(bad_a.contribution().clone(), bad_b.contribution().clone());
            match compose_failure(&expected, input) {
                Failure::ContributionInvariantMismatch {
                    contribution,
                    reason,
                } => {
                    assert_eq!(contribution, expected_first);
                    assert_eq!(reason, Reason::InvalidOriginGroup);
                }
                other => panic!("expected invariant mismatch, got {other:?}"),
            }
        }
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
