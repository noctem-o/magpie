//! Explicit policy-v3 external-report corroboration over one verified replay context.
//!
//! Policy v3 consumes only an internally derived policy-v2 resolution and an
//! internally derived support-contribution audit from the same
//! [`OriginAdmissionReplayContextV0`](crate::OriginAdmissionReplayContextV0)
//! and immutable closure. One crate-private pure composer implements the
//! complete policy. There is no public constructor, free resolver, runtime
//! policy selector, caller-provided audit, or second standing engine.
//!
//! External-source corroboration can produce `Supported`, never `Settled`.
//! Distinct admitted origin groups are corroboration separation only; they are
//! not proof of truth or statistical independence.
//!
//! Declaration order of every field and variant below is canonical JSON byte
//! order. Do not reorder.
//!
//! Public v3 structs cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::CorroborationGroupV0;
//! let _ = CorroborationGroupV0 {
//!     origin_group: String::new(),
//!     contributions: Vec::new(),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     CorroborationGroupV0, ExternalReportCorroborationLaneV0,
//!     OriginComparisonNamespaceV0,
//! };
//! fn construct(
//!     namespace: OriginComparisonNamespaceV0,
//!     groups: Vec<CorroborationGroupV0>,
//! ) {
//!     let _ = ExternalReportCorroborationLaneV0 { namespace, groups };
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     StandingPolicyApplicationV3, StandingPolicyContextV3, StandingPolicyRuleV3,
//! };
//! use magpie_log::Status;
//! let _ = StandingPolicyApplicationV3 {
//!     rule: StandingPolicyRuleV3::ExternalReportCorroborationV0,
//!     context: StandingPolicyContextV3::Corroborated { distinct_group_count: 2 },
//!     achieved_standing: Some(Status::Supported),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ExternalReportCorroborationLaneV0, ResolutionContentClosureIdentityV0,
//!     StandingBlockerV3, StandingCurrentness, StandingPolicyApplicationV3,
//!     StandingResolutionFailureV3, StandingResolutionV2, StandingResolutionV3,
//!     SupportContributionAuditV0, SupportContributionV0, VerifiedLogPrefixIdentityV0,
//! };
//! fn construct(
//!     currentness: StandingCurrentness,
//!     verified_prefix_identity: VerifiedLogPrefixIdentityV0,
//!     closure_identity: ResolutionContentClosureIdentityV0,
//!     inherited_v2: StandingResolutionV2,
//!     support_contribution_audit: SupportContributionAuditV0,
//!     resolution_failure: Option<StandingResolutionFailureV3>,
//!     lane: Option<ExternalReportCorroborationLaneV0>,
//!     application: Option<StandingPolicyApplicationV3>,
//!     ignored_contributions: Vec<SupportContributionV0>,
//!     blockers: Vec<StandingBlockerV3>,
//! ) {
//!     let _ = StandingResolutionV3 {
//!         claim_id: String::new(),
//!         governed_standing: None,
//!         legacy_raw_standing: None,
//!         currentness,
//!         policy_id: "not-authority",
//!         verified_prefix_identity,
//!         closure_identity,
//!         inherited_v2,
//!         support_contribution_audit,
//!         resolution_failure,
//!         lane,
//!         application,
//!         ignored_contributions,
//!         blockers,
//!     };
//! }
//! ```
//!
//! V3 values cannot be deserialized into authority-bearing types:
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyRuleV3;
//! let _ = serde_json::from_str::<StandingPolicyRuleV3>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::CorroborationGroupV0;
//! let _ = serde_json::from_str::<CorroborationGroupV0>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ExternalReportCorroborationLaneV0;
//! let _ = serde_json::from_str::<ExternalReportCorroborationLaneV0>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyContextV3;
//! let _ = serde_json::from_str::<StandingPolicyContextV3>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingBlockerV3;
//! let _ = serde_json::from_str::<StandingBlockerV3>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV3;
//! let _ = serde_json::from_str::<StandingPolicyApplicationV3>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionFailureV3;
//! let _ = serde_json::from_str::<StandingResolutionFailureV3>("");
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV3;
//! let _ = serde_json::from_str::<StandingResolutionV3>("");
//! ```
//!
//! V3 types have no `Default` implementation:
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyRuleV3;
//! let _ = StandingPolicyRuleV3::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::CorroborationGroupV0;
//! let _ = CorroborationGroupV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ExternalReportCorroborationLaneV0;
//! let _ = ExternalReportCorroborationLaneV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyContextV3;
//! let _ = StandingPolicyContextV3::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingBlockerV3;
//! let _ = StandingBlockerV3::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingPolicyApplicationV3;
//! let _ = StandingPolicyApplicationV3::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionFailureV3;
//! let _ = StandingResolutionFailureV3::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingResolutionV3;
//! let _ = StandingResolutionV3::default();
//! ```
//!
//! Each public resolver accepts only the exact requested claim and closure.
//! Every forbidden authority input is pinned independently so one rejected
//! input cannot hide another input becoming accepted.
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionAuditV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     audit: SupportContributionAuditV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, audit);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     contribution: SupportContributionV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, contribution);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     contributions: Vec<SupportContributionV0>,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, contributions);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     origin_group: &str,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, origin_group);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ExternalReportCorroborationLaneV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     lane: ExternalReportCorroborationLaneV0,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, lane);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     lane_id: &str,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, lane_id);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     threshold: usize,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, threshold);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     policy_id: &str,
//! ) {
//!     let _ = context.resolved_standing_with_trace_v3(claim_id, closure, policy_id);
//! }
//! ```
//!
//! The scalar resolver independently rejects the same authority inputs:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionAuditV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     audit: SupportContributionAuditV0,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, audit);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     contribution: SupportContributionV0,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, contribution);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
//!     SupportContributionV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     contributions: Vec<SupportContributionV0>,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, contributions);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     origin_group: &str,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, origin_group);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ExternalReportCorroborationLaneV0, OriginAdmissionReplayContextV0,
//!     ResolutionContentClosureV0,
//! };
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     lane: ExternalReportCorroborationLaneV0,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, lane);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     lane_id: &str,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, lane_id);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     threshold: usize,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, threshold);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{OriginAdmissionReplayContextV0, ResolutionContentClosureV0};
//! fn substitute(
//!     context: &OriginAdmissionReplayContextV0,
//!     claim_id: &str,
//!     closure: &ResolutionContentClosureV0,
//!     policy_id: &str,
//! ) {
//!     let _ = context.resolved_standing_v3(claim_id, closure, policy_id);
//! }
//! ```
//!
//! A standalone standing view or replay snapshot has no v3 resolver:
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingView};
//! fn resolve(view: &StandingView, closure: &ResolutionContentClosureV0) {
//!     let _ = view.resolved_standing_with_trace_v3("claim", closure);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{ResolutionContentClosureV0, StandingReplaySnapshot};
//! fn resolve(snapshot: &StandingReplaySnapshot, closure: &ResolutionContentClosureV0) {
//!     let _ = snapshot.resolved_standing_with_trace_v3("claim", closure);
//! }
//! ```

use std::collections::BTreeMap;

use magpie_log::Status;
use serde::Serialize;

use crate::admitted_contribution_audit::ADMITTED_CONTRIBUTION_POLICY_ID_V0;
use crate::origin_admission_audit::ORIGIN_ADMISSION_POLICY_ID_V0;
use crate::origin_admission_replay::VerifiedLogPrefixIdentityV0;
use crate::origin_binding_verifier::{ContributionIdentityV0, OriginComparisonNamespaceV0};
use crate::policy::{ClaimDomain, EvidenceKind};
use crate::resolution_content_closure::ResolutionContentClosureIdentityV0;
use crate::standing::StandingCurrentness;
use crate::standing_v2::{StandingResolutionV2, MAGPIE_CLAIMS_POLICY_V2_ID};
use crate::support_contribution_audit::{
    SupportContributionAuditCompletionV0, SupportContributionAuditV0, SupportContributionV0,
    SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0, SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0,
    SUPPORT_CONTRIBUTION_POLICY_ID_V0,
};

/// Fixed identity for the explicitly selected governed-standing v3 policy.
pub const MAGPIE_CLAIMS_POLICY_V3_ID: &str = "magpie-claims-standing-v3";

/// Fixed minimum distinct admitted origin groups for the sole v3 rule.
pub const EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0: usize = 2;

/// Closed rule vocabulary for policy v3.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingPolicyRuleV3 {
    ExternalReportCorroborationV0,
}

/// Every retained support contribution assigned to one exact origin group.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CorroborationGroupV0 {
    origin_group: String,
    contributions: Vec<SupportContributionV0>,
}

impl CorroborationGroupV0 {
    pub fn origin_group(&self) -> &str {
        &self.origin_group
    }

    pub fn contributions(&self) -> &[SupportContributionV0] {
        &self.contributions
    }
}

/// The one exact permitted external-report corroboration lane.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExternalReportCorroborationLaneV0 {
    namespace: OriginComparisonNamespaceV0,
    groups: Vec<CorroborationGroupV0>,
}

impl ExternalReportCorroborationLaneV0 {
    pub fn namespace(&self) -> &OriginComparisonNamespaceV0 {
        &self.namespace
    }

    pub fn groups(&self) -> &[CorroborationGroupV0] {
        &self.groups
    }
}

/// Closed successful application context for policy v3.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingPolicyContextV3 {
    Corroborated { distinct_group_count: usize },
}

/// Closed non-failure blocker vocabulary in deterministic declaration order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingBlockerV3 {
    InsufficientDistinctOriginGroups { distinct_group_count: usize },
    SupportAuditIncomplete,
    SupportAuditRejected,
    InheritedV2ResolutionFailed,
}

/// The sole policy-v3 application shape.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingPolicyApplicationV3 {
    rule: StandingPolicyRuleV3,
    context: StandingPolicyContextV3,
    achieved_standing: Option<Status>,
}

impl StandingPolicyApplicationV3 {
    pub fn rule(&self) -> &StandingPolicyRuleV3 {
        &self.rule
    }

    pub fn context(&self) -> &StandingPolicyContextV3 {
        &self.context
    }

    pub fn achieved_standing(&self) -> Option<Status> {
        self.achieved_standing
    }
}

/// Closed outer-composition failure vocabulary in exact first-failure order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingResolutionFailureV3 {
    InheritedClaimMismatch,
    InheritedPolicyMismatch,
    SupportAuditSchemaMismatch,
    SupportAuditCanonicalizationProfileMismatch,
    SupportAuditPolicyMismatch,
    SupportAuditAdmittedPolicyMismatch,
    SupportAuditOriginPolicyMismatch,
    VerifiedPrefixIdentityMismatch,
    ClosureIdentityMismatch,
    LaneInvariantMismatch {
        contribution: ContributionIdentityV0,
    },
    TargetScopeMismatch {
        contribution: ContributionIdentityV0,
    },
    DuplicateContributionIdentity {
        contribution: ContributionIdentityV0,
    },
}

/// Deterministic governed-standing explanation under explicit policy v3.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingResolutionV3 {
    claim_id: String,
    governed_standing: Option<Status>,
    legacy_raw_standing: Option<Status>,
    currentness: StandingCurrentness,
    policy_id: &'static str,
    verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    closure_identity: ResolutionContentClosureIdentityV0,
    inherited_v2: StandingResolutionV2,
    support_contribution_audit: SupportContributionAuditV0,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution_failure: Option<StandingResolutionFailureV3>,
    lane: Option<ExternalReportCorroborationLaneV0>,
    application: Option<StandingPolicyApplicationV3>,
    ignored_contributions: Vec<SupportContributionV0>,
    blockers: Vec<StandingBlockerV3>,
}

impl StandingResolutionV3 {
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn governed_standing(&self) -> Option<Status> {
        self.governed_standing
    }

    pub fn legacy_raw_standing(&self) -> Option<Status> {
        self.legacy_raw_standing
    }

    pub fn currentness(&self) -> StandingCurrentness {
        self.currentness
    }

    pub fn policy_id(&self) -> &str {
        self.policy_id
    }

    pub fn verified_prefix_identity(&self) -> &VerifiedLogPrefixIdentityV0 {
        &self.verified_prefix_identity
    }

    pub fn closure_identity(&self) -> &ResolutionContentClosureIdentityV0 {
        &self.closure_identity
    }

    pub fn inherited_v2(&self) -> &StandingResolutionV2 {
        &self.inherited_v2
    }

    pub fn support_contribution_audit(&self) -> &SupportContributionAuditV0 {
        &self.support_contribution_audit
    }

    pub fn resolution_failure(&self) -> Option<&StandingResolutionFailureV3> {
        self.resolution_failure.as_ref()
    }

    pub fn lane(&self) -> Option<&ExternalReportCorroborationLaneV0> {
        self.lane.as_ref()
    }

    pub fn application(&self) -> Option<&StandingPolicyApplicationV3> {
        self.application.as_ref()
    }

    pub fn ignored_contributions(&self) -> &[SupportContributionV0] {
        &self.ignored_contributions
    }

    pub fn blockers(&self) -> &[StandingBlockerV3] {
        &self.blockers
    }

    /// Deterministic policy-pinned derived audit bytes.
    ///
    /// These bytes are not L0 canonical encoding and are never accepted as
    /// authority by a resolver.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("StandingResolutionV3 is always serializable")
    }
}

/// Sole test-only mutation of the otherwise write-once v3 policy identifier.
///
/// This stages the otherwise unreachable
/// `StandingResolutionFailureV4::InheritedPolicyMismatch` path. It must remain
/// guarded by `#[cfg(test)]`; production construction never accepts a
/// caller-selected policy identifier.
#[cfg(test)]
pub(crate) fn stage_standing_resolution_v3_policy_id_for_tests(
    resolution: &mut StandingResolutionV3,
    policy_id: &'static str,
) {
    resolution.policy_id = policy_id;
}

/// Sealed crate-private input to the one pure v3 composition path.
///
/// Production constructs this only after deriving both nested inputs from one
/// private-construction replay context and the same closure.
pub(crate) struct StandingV3CompositionInputV0 {
    requested_claim_id: String,
    requested_claim_scope: Option<String>,
    expected_verified_prefix_identity: VerifiedLogPrefixIdentityV0,
    expected_closure_identity: ResolutionContentClosureIdentityV0,
    inherited_v2: StandingResolutionV2,
    support_contribution_audit: SupportContributionAuditV0,
}

impl StandingV3CompositionInputV0 {
    pub(crate) fn new(
        requested_claim_id: &str,
        requested_claim_scope: Option<String>,
        expected_verified_prefix_identity: VerifiedLogPrefixIdentityV0,
        expected_closure_identity: ResolutionContentClosureIdentityV0,
        inherited_v2: StandingResolutionV2,
        support_contribution_audit: SupportContributionAuditV0,
    ) -> Self {
        Self {
            requested_claim_id: requested_claim_id.to_owned(),
            requested_claim_scope,
            expected_verified_prefix_identity,
            expected_closure_identity,
            inherited_v2,
            support_contribution_audit,
        }
    }
}

/// The single pure policy-v3 composition path used by production and hostile tests.
pub(crate) fn compose_standing_resolution_v3(
    input: StandingV3CompositionInputV0,
) -> StandingResolutionV3 {
    use StandingResolutionFailureV3 as Failure;

    if input.inherited_v2.claim_id != input.requested_claim_id {
        return failed_resolution(input, Failure::InheritedClaimMismatch);
    }
    if input.inherited_v2.policy_id != MAGPIE_CLAIMS_POLICY_V2_ID {
        return failed_resolution(input, Failure::InheritedPolicyMismatch);
    }

    if input.support_contribution_audit.schema() != SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0 {
        return failed_resolution(input, Failure::SupportAuditSchemaMismatch);
    }
    if input.support_contribution_audit.canonicalization_profile()
        != SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
    {
        return failed_resolution(input, Failure::SupportAuditCanonicalizationProfileMismatch);
    }
    if input.support_contribution_audit.policy_id() != SUPPORT_CONTRIBUTION_POLICY_ID_V0 {
        return failed_resolution(input, Failure::SupportAuditPolicyMismatch);
    }
    if input
        .support_contribution_audit
        .admitted_contribution_policy_id()
        != ADMITTED_CONTRIBUTION_POLICY_ID_V0
    {
        return failed_resolution(input, Failure::SupportAuditAdmittedPolicyMismatch);
    }
    if input
        .support_contribution_audit
        .origin_admission_policy_id()
        != ORIGIN_ADMISSION_POLICY_ID_V0
    {
        return failed_resolution(input, Failure::SupportAuditOriginPolicyMismatch);
    }
    if input.support_contribution_audit.verified_prefix_identity()
        != &input.expected_verified_prefix_identity
    {
        return failed_resolution(input, Failure::VerifiedPrefixIdentityMismatch);
    }
    if input.support_contribution_audit.closure_identity() != &input.expected_closure_identity {
        return failed_resolution(input, Failure::ClosureIdentityMismatch);
    }

    let inherited_v2_failed = input.inherited_v2.resolution_failure.is_some();
    let completion = input.support_contribution_audit.completion().clone();

    let (mut selected, mut ignored) = match completion {
        SupportContributionAuditCompletionV0::Complete => input
            .support_contribution_audit
            .support_contributions()
            .iter()
            .cloned()
            .partition::<Vec<_>, _>(|contribution| {
                contribution.contribution().target_claim_id() == input.requested_claim_id
            }),
        SupportContributionAuditCompletionV0::AdmittedContributionIncomplete { .. }
        | SupportContributionAuditCompletionV0::UpstreamCompositionRejected { .. } => {
            (Vec::new(), Vec::new())
        }
    };

    selected.sort_by(|left, right| left.contribution().cmp(right.contribution()));
    ignored.sort_by(|left, right| left.contribution().cmp(right.contribution()));

    if matches!(completion, SupportContributionAuditCompletionV0::Complete) {
        if let Some(contribution) = selected
            .iter()
            .find(|contribution| !valid_lane_membership(contribution))
        {
            return failed_resolution(
                input,
                Failure::LaneInvariantMismatch {
                    contribution: contribution.contribution().clone(),
                },
            );
        }

        if let Some(contribution) = selected.iter().find(|contribution| {
            !valid_target_scope_binding(
                contribution,
                &input.requested_claim_id,
                input.requested_claim_scope.as_deref(),
            )
        }) {
            return failed_resolution(
                input,
                Failure::TargetScopeMismatch {
                    contribution: contribution.contribution().clone(),
                },
            );
        }

        let mut counts: BTreeMap<ContributionIdentityV0, usize> = BTreeMap::new();
        for contribution in &selected {
            *counts
                .entry(contribution.contribution().clone())
                .or_default() += 1;
        }
        if let Some((contribution, _)) = counts.iter().find(|(_, count)| **count > 1) {
            return failed_resolution(
                input,
                Failure::DuplicateContributionIdentity {
                    contribution: contribution.clone(),
                },
            );
        }
    }

    let lane = build_lane(selected);
    let distinct_group_count = lane.as_ref().map_or(0, |lane| lane.groups.len());

    let mut blockers = Vec::new();
    match &completion {
        SupportContributionAuditCompletionV0::Complete
            if distinct_group_count
                < EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0 =>
        {
            blockers.push(StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count,
            });
        }
        SupportContributionAuditCompletionV0::AdmittedContributionIncomplete { .. } => {
            blockers.push(StandingBlockerV3::SupportAuditIncomplete);
        }
        SupportContributionAuditCompletionV0::UpstreamCompositionRejected { .. } => {
            blockers.push(StandingBlockerV3::SupportAuditRejected);
        }
        SupportContributionAuditCompletionV0::Complete => {}
    }
    if inherited_v2_failed {
        blockers.push(StandingBlockerV3::InheritedV2ResolutionFailed);
    }

    let corroborated = matches!(completion, SupportContributionAuditCompletionV0::Complete)
        && distinct_group_count >= EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0;
    let application =
        (corroborated && !inherited_v2_failed).then_some(StandingPolicyApplicationV3 {
            rule: StandingPolicyRuleV3::ExternalReportCorroborationV0,
            context: StandingPolicyContextV3::Corroborated {
                distinct_group_count,
            },
            achieved_standing: Some(Status::Supported),
        });

    let governed_standing = if inherited_v2_failed {
        input.inherited_v2.governed_standing
    } else {
        combine_standing(input.inherited_v2.governed_standing, application.is_some())
    };

    StandingResolutionV3 {
        claim_id: input.requested_claim_id,
        governed_standing,
        legacy_raw_standing: input.inherited_v2.legacy_raw_standing,
        currentness: input.inherited_v2.currentness,
        policy_id: MAGPIE_CLAIMS_POLICY_V3_ID,
        verified_prefix_identity: input.expected_verified_prefix_identity,
        closure_identity: input.expected_closure_identity,
        inherited_v2: input.inherited_v2,
        support_contribution_audit: input.support_contribution_audit,
        resolution_failure: None,
        lane,
        application,
        ignored_contributions: ignored,
        blockers,
    }
}

fn valid_lane_membership(contribution: &SupportContributionV0) -> bool {
    contribution.policy_id() == SUPPORT_CONTRIBUTION_POLICY_ID_V0
        && contribution.admitted_contribution_policy_id() == ADMITTED_CONTRIBUTION_POLICY_ID_V0
        && contribution.origin_admission_policy_id() == ORIGIN_ADMISSION_POLICY_ID_V0
        && contribution.namespace().origin_admission_policy_id() == ORIGIN_ADMISSION_POLICY_ID_V0
        && contribution.evidence_kind() == EvidenceKind::ExternalSource.as_str()
        && contribution.claim_domain() == ClaimDomain::ExternalReport.as_str()
        && contribution.support_ceiling() == Status::Supported
}

fn valid_target_scope_binding(
    contribution: &SupportContributionV0,
    requested_claim_id: &str,
    requested_claim_scope: Option<&str>,
) -> bool {
    contribution.namespace().target_claim_id() == requested_claim_id
        && requested_claim_scope.is_some_and(|scope| contribution.namespace().scope_ref() == scope)
        && contribution.contribution().scope_ref() == contribution.namespace().scope_ref()
}

fn build_lane(selected: Vec<SupportContributionV0>) -> Option<ExternalReportCorroborationLaneV0> {
    let namespace = selected.first()?.namespace().clone();
    let mut grouped: BTreeMap<String, Vec<SupportContributionV0>> = BTreeMap::new();
    for contribution in selected {
        grouped
            .entry(contribution.origin_group().to_owned())
            .or_default()
            .push(contribution);
    }
    let groups = grouped
        .into_iter()
        .map(|(origin_group, contributions)| CorroborationGroupV0 {
            origin_group,
            contributions,
        })
        .collect();
    Some(ExternalReportCorroborationLaneV0 { namespace, groups })
}

fn combine_standing(inherited: Option<Status>, corroborated: bool) -> Option<Status> {
    // Fail-closed status evolution (A-005/RQ-005): every current `Status`
    // variant is named explicitly, so adding a future variant stops
    // compilation here and forces an explicit policy decision. Only the
    // Boolean dimension may be wildcarded, once the status is fully selected.
    // Unlike v2, successful corroboration promotes an absent inherited
    // standing to `Supported`.
    match (inherited, corroborated) {
        (Some(Status::Settled), _) => Some(Status::Settled),
        (Some(Status::Refuted), _) => Some(Status::Refuted),
        (Some(Status::Supported), _) => Some(Status::Supported),
        (None | Some(Status::Open | Status::Conjectured), true) => Some(Status::Supported),
        (inherited @ (None | Some(Status::Open | Status::Conjectured)), false) => inherited,
    }
}

fn failed_resolution(
    input: StandingV3CompositionInputV0,
    failure: StandingResolutionFailureV3,
) -> StandingResolutionV3 {
    let inherited_identity_invalid = matches!(
        failure,
        StandingResolutionFailureV3::InheritedClaimMismatch
            | StandingResolutionFailureV3::InheritedPolicyMismatch
    );
    let (governed_standing, legacy_raw_standing, currentness) = if inherited_identity_invalid {
        (None, None, StandingCurrentness::Unknown)
    } else {
        (
            input.inherited_v2.governed_standing,
            input.inherited_v2.legacy_raw_standing,
            input.inherited_v2.currentness,
        )
    };

    StandingResolutionV3 {
        claim_id: input.requested_claim_id,
        governed_standing,
        legacy_raw_standing,
        currentness,
        policy_id: MAGPIE_CLAIMS_POLICY_V3_ID,
        verified_prefix_identity: input.expected_verified_prefix_identity,
        closure_identity: input.expected_closure_identity,
        inherited_v2: input.inherited_v2,
        support_contribution_audit: input.support_contribution_audit,
        resolution_failure: Some(failure),
        lane: None,
        application: None,
        ignored_contributions: Vec::new(),
        blockers: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admitted_contribution_audit::{
        stage_contribution_identity_v0_for_tests, stage_namespace_v0_for_tests,
    };
    use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;
    use crate::origin_admission_replay::replay_origin_admission_context_v0;
    use crate::origin_binding_verifier::{
        stage_origin_comparison_namespace_v0_for_tests, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0, ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
    };
    use crate::resolution_content_closure::{
        ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
    };
    use crate::standing_v2::StandingResolutionFailureV2;
    use crate::support_contribution_audit::{
        stage_support_contribution_audit_v0_for_tests, stage_support_contribution_v0_for_tests,
        SupportContributionCompositionFailureV0,
    };
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey};
    use sha2::{Digest, Sha256};

    const SEED: [u8; 32] = [73; 32];
    const CLAIM_ID: &str = "claim-v3";
    const SCOPE: &str = "scope:v3";
    const REJECTED_FIXTURE: &[u8] = include_bytes!(
        "../../../fixtures/standing-policy-v3-corroboration-v0/support-audit-rejected.json"
    );
    const INHERITED_FAILURE_FIXTURE: &[u8] = include_bytes!(
        "../../../fixtures/standing-policy-v3-corroboration-v0/inherited-v2-failure.json"
    );
    const OUTER_FAILURE_FIXTURE: &[u8] =
        include_bytes!("../../../fixtures/standing-policy-v3-corroboration-v0/outer-failure.json");
    const OUTER_LANE_FIXTURE: &[u8] = include_bytes!(
        "../../../fixtures/standing-policy-v3-corroboration-v0/outer-failure-lane-invariant.json"
    );
    const OUTER_TARGET_FIXTURE: &[u8] = include_bytes!(
        "../../../fixtures/standing-policy-v3-corroboration-v0/outer-failure-target-scope.json"
    );

    #[derive(Clone)]
    struct Expected {
        prefix: VerifiedLogPrefixIdentityV0,
        closure: ResolutionContentClosureIdentityV0,
    }

    #[derive(Clone)]
    struct SupportSpec {
        policy_id: String,
        admitted_policy_id: String,
        origin_policy_id: String,
        contribution: ContributionIdentityV0,
        namespace: OriginComparisonNamespaceV0,
        origin_group: String,
        evidence_kind: String,
        claim_domain: String,
        support_ceiling: Status,
        selectors: Vec<ArtifactProvenanceAnchorSelectorV0>,
    }

    impl SupportSpec {
        fn honest(label: &str, origin_group: &str) -> Self {
            Self {
                policy_id: SUPPORT_CONTRIBUTION_POLICY_ID_V0.to_owned(),
                admitted_policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
                origin_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                contribution: identity(CLAIM_ID, label, SCOPE),
                namespace: stage_namespace_v0_for_tests(CLAIM_ID, SCOPE),
                origin_group: origin_group.to_owned(),
                evidence_kind: EvidenceKind::ExternalSource.as_str().to_owned(),
                claim_domain: ClaimDomain::ExternalReport.as_str().to_owned(),
                support_ceiling: Status::Supported,
                selectors: Vec::new(),
            }
        }

        fn build(self) -> SupportContributionV0 {
            stage_support_contribution_v0_for_tests(
                &self.policy_id,
                &self.admitted_policy_id,
                &self.origin_policy_id,
                self.contribution,
                self.namespace,
                &self.origin_group,
                &self.evidence_kind,
                &self.claim_domain,
                self.support_ceiling,
                self.selectors,
            )
        }
    }

    #[derive(Clone)]
    struct AuditSpec {
        schema: String,
        canonicalization_profile: String,
        policy_id: String,
        admitted_policy_id: String,
        origin_policy_id: String,
        prefix: VerifiedLogPrefixIdentityV0,
        closure: ResolutionContentClosureIdentityV0,
        completion: SupportContributionAuditCompletionV0,
        contributions: Vec<SupportContributionV0>,
    }

    impl AuditSpec {
        fn honest(expected: &Expected, contributions: Vec<SupportContributionV0>) -> Self {
            Self {
                schema: SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0.to_owned(),
                canonicalization_profile: SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
                    .to_owned(),
                policy_id: SUPPORT_CONTRIBUTION_POLICY_ID_V0.to_owned(),
                admitted_policy_id: ADMITTED_CONTRIBUTION_POLICY_ID_V0.to_owned(),
                origin_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
                prefix: expected.prefix.clone(),
                closure: expected.closure.clone(),
                completion: SupportContributionAuditCompletionV0::Complete,
                contributions,
            }
        }

        fn build(self) -> SupportContributionAuditV0 {
            stage_support_contribution_audit_v0_for_tests(
                &self.schema,
                &self.canonicalization_profile,
                &self.policy_id,
                &self.admitted_policy_id,
                &self.origin_policy_id,
                self.prefix,
                self.closure,
                self.completion,
                self.contributions,
            )
        }
    }

    fn replay_identity(note_count: usize) -> VerifiedLogPrefixIdentityV0 {
        let store = MemStore::new();
        {
            let mut timestamp = 0_u64;
            let mut writer = LogWriter::open_with_clock(
                store.clone(),
                SigningKey::from_bytes(&SEED),
                Box::new(move || {
                    timestamp += 1;
                    timestamp
                }),
            )
            .unwrap();
            for index in 0..note_count {
                writer
                    .append(
                        Provenance::new("standing-v3-test", "identity"),
                        Payload::Note {
                            text: format!("identity-{index}"),
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
        let object = ResolutionArtifactObjectInputV0::new(
            ResolutionArtifactObjectKeyV0::new("sha256", "a".repeat(64)),
            b"different-v3-closure",
        );
        ResolutionContentClosureV0::construct(&[object], &[])
            .unwrap()
            .identity()
            .clone()
    }

    fn expected() -> Expected {
        Expected {
            prefix: replay_identity(0),
            closure: closure_identity(),
        }
    }

    fn identity(target: &str, label: &str, scope: &str) -> ContributionIdentityV0 {
        stage_contribution_identity_v0_for_tests(
            target,
            &format!("evidence-{label}"),
            &format!("edge-{label}"),
            scope,
            &format!("{:0<64}", label),
        )
    }

    fn inherited(status: Option<Status>) -> StandingResolutionV2 {
        StandingResolutionV2 {
            claim_id: CLAIM_ID.to_owned(),
            governed_standing: status,
            legacy_raw_standing: Some(Status::Open),
            currentness: StandingCurrentness::Unknown,
            policy_id: MAGPIE_CLAIMS_POLICY_V2_ID,
            resolution_failure: None,
            trace: Vec::new(),
            blockers: Vec::new(),
        }
    }

    fn compose(
        expected: &Expected,
        inherited_v2: StandingResolutionV2,
        audit: SupportContributionAuditV0,
    ) -> StandingResolutionV3 {
        compose_standing_resolution_v3(StandingV3CompositionInputV0::new(
            CLAIM_ID,
            Some(SCOPE.to_owned()),
            expected.prefix.clone(),
            expected.closure.clone(),
            inherited_v2,
            audit,
        ))
    }

    fn compose_specs(
        expected: &Expected,
        inherited_v2: StandingResolutionV2,
        audit: AuditSpec,
    ) -> StandingResolutionV3 {
        compose(expected, inherited_v2, audit.build())
    }

    fn two_groups() -> Vec<SupportContributionV0> {
        vec![
            SupportSpec::honest("alpha", "origin-alpha").build(),
            SupportSpec::honest("beta", "origin-beta").build(),
        ]
    }

    fn rejected_fixture_resolution() -> StandingResolutionV3 {
        let expected = expected();
        let mut audit = AuditSpec::honest(&expected, Vec::new());
        audit.completion = SupportContributionAuditCompletionV0::UpstreamCompositionRejected {
            failure: SupportContributionCompositionFailureV0::UpstreamSchemaMismatch,
        };
        compose_specs(&expected, inherited(Some(Status::Conjectured)), audit)
    }

    fn inherited_failure_fixture_resolution() -> StandingResolutionV3 {
        let expected = expected();
        let mut inherited = inherited(Some(Status::Conjectured));
        inherited.resolution_failure =
            Some(StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 0 });
        compose_specs(
            &expected,
            inherited,
            AuditSpec::honest(&expected, two_groups()),
        )
    }

    fn outer_global_fixture_resolution() -> StandingResolutionV3 {
        let expected = expected();
        let mut audit = AuditSpec::honest(&expected, Vec::new());
        audit.policy_id = "foreign-support-policy".to_owned();
        compose_specs(&expected, inherited(Some(Status::Conjectured)), audit)
    }

    fn outer_lane_fixture_resolution() -> StandingResolutionV3 {
        let expected = expected();
        let mut support = SupportSpec::honest("fixture-lane", "origin-alpha");
        support.origin_policy_id = "foreign-origin-policy".to_owned();
        compose_specs(
            &expected,
            inherited(Some(Status::Conjectured)),
            AuditSpec::honest(&expected, vec![support.build()]),
        )
    }

    fn outer_target_fixture_resolution() -> StandingResolutionV3 {
        let expected = expected();
        let mut support = SupportSpec::honest("fixture-target", "origin-alpha");
        support.namespace = stage_origin_comparison_namespace_v0_for_tests(
            ORIGIN_ADMISSION_POLICY_ID_V0,
            CLAIM_ID,
            "scope:foreign",
        );
        compose_specs(
            &expected,
            inherited(Some(Status::Conjectured)),
            AuditSpec::honest(&expected, vec![support.build()]),
        )
    }

    #[test]
    fn constants_and_closed_enum_serde_shapes_are_exact() {
        assert_eq!(MAGPIE_CLAIMS_POLICY_V3_ID, "magpie-claims-standing-v3");
        assert_eq!(
            EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0,
            2
        );
        assert_eq!(
            serde_json::to_string(&StandingPolicyRuleV3::ExternalReportCorroborationV0).unwrap(),
            "\"external_report_corroboration_v0\""
        );
        assert_eq!(
            serde_json::to_string(&StandingPolicyContextV3::Corroborated {
                distinct_group_count: 3,
            })
            .unwrap(),
            r#"{"kind":"corroborated","distinct_group_count":3}"#
        );

        let blockers = [
            (
                StandingBlockerV3::InsufficientDistinctOriginGroups {
                    distinct_group_count: 1,
                },
                r#"{"kind":"insufficient_distinct_origin_groups","distinct_group_count":1}"#,
            ),
            (
                StandingBlockerV3::SupportAuditIncomplete,
                r#"{"kind":"support_audit_incomplete"}"#,
            ),
            (
                StandingBlockerV3::SupportAuditRejected,
                r#"{"kind":"support_audit_rejected"}"#,
            ),
            (
                StandingBlockerV3::InheritedV2ResolutionFailed,
                r#"{"kind":"inherited_v2_resolution_failed"}"#,
            ),
        ];
        for (blocker, expected) in blockers {
            assert_eq!(serde_json::to_string(&blocker).unwrap(), expected);
        }

        let contribution = identity(CLAIM_ID, "serde", SCOPE);
        let failures = [
            (
                StandingResolutionFailureV3::InheritedClaimMismatch,
                r#"{"kind":"inherited_claim_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::InheritedPolicyMismatch,
                r#"{"kind":"inherited_policy_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::SupportAuditSchemaMismatch,
                r#"{"kind":"support_audit_schema_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::SupportAuditCanonicalizationProfileMismatch,
                r#"{"kind":"support_audit_canonicalization_profile_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::SupportAuditPolicyMismatch,
                r#"{"kind":"support_audit_policy_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::SupportAuditAdmittedPolicyMismatch,
                r#"{"kind":"support_audit_admitted_policy_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::SupportAuditOriginPolicyMismatch,
                r#"{"kind":"support_audit_origin_policy_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::VerifiedPrefixIdentityMismatch,
                r#"{"kind":"verified_prefix_identity_mismatch"}"#.to_owned(),
            ),
            (
                StandingResolutionFailureV3::ClosureIdentityMismatch,
                r#"{"kind":"closure_identity_mismatch"}"#.to_owned(),
            ),
        ];
        for (failure, expected) in failures {
            assert_eq!(serde_json::to_string(&failure).unwrap(), expected);
        }
        for (failure, kind) in [
            (
                StandingResolutionFailureV3::LaneInvariantMismatch {
                    contribution: contribution.clone(),
                },
                "lane_invariant_mismatch",
            ),
            (
                StandingResolutionFailureV3::TargetScopeMismatch {
                    contribution: contribution.clone(),
                },
                "target_scope_mismatch",
            ),
            (
                StandingResolutionFailureV3::DuplicateContributionIdentity {
                    contribution: contribution.clone(),
                },
                "duplicate_contribution_identity",
            ),
        ] {
            let value = serde_json::to_value(failure).unwrap();
            assert_eq!(value["kind"], kind);
            assert_eq!(value["contribution"]["justification_edge_id"], "edge-serde");
        }
    }

    #[test]
    fn every_outer_identity_failure_and_first_failure_order_are_exact() {
        let expected = expected();
        let honest = AuditSpec::honest(&expected, Vec::new());
        let mut cases = Vec::new();

        let mut schema = honest.clone();
        schema.schema = "wrong-schema".to_owned();
        cases.push((
            schema,
            StandingResolutionFailureV3::SupportAuditSchemaMismatch,
        ));
        let mut profile = honest.clone();
        profile.canonicalization_profile = "wrong-profile".to_owned();
        cases.push((
            profile,
            StandingResolutionFailureV3::SupportAuditCanonicalizationProfileMismatch,
        ));
        let mut policy = honest.clone();
        policy.policy_id = "wrong-support-policy".to_owned();
        cases.push((
            policy,
            StandingResolutionFailureV3::SupportAuditPolicyMismatch,
        ));
        let mut admitted = honest.clone();
        admitted.admitted_policy_id = "wrong-admitted-policy".to_owned();
        cases.push((
            admitted,
            StandingResolutionFailureV3::SupportAuditAdmittedPolicyMismatch,
        ));
        let mut origin = honest.clone();
        origin.origin_policy_id = "wrong-origin-policy".to_owned();
        cases.push((
            origin,
            StandingResolutionFailureV3::SupportAuditOriginPolicyMismatch,
        ));
        let mut prefix = honest.clone();
        prefix.prefix = replay_identity(1);
        cases.push((
            prefix,
            StandingResolutionFailureV3::VerifiedPrefixIdentityMismatch,
        ));
        let mut closure = honest.clone();
        closure.closure = different_closure_identity();
        cases.push((
            closure,
            StandingResolutionFailureV3::ClosureIdentityMismatch,
        ));

        for (audit, failure) in cases {
            let resolution = compose_specs(&expected, inherited(None), audit);
            assert_eq!(resolution.resolution_failure(), Some(&failure));
            assert!(resolution.lane().is_none());
            assert!(resolution.application().is_none());
            assert!(resolution.ignored_contributions().is_empty());
            assert!(resolution.blockers().is_empty());
            assert_eq!(resolution.verified_prefix_identity(), &expected.prefix);
            assert_eq!(resolution.closure_identity(), &expected.closure);
        }

        let mut several = honest;
        several.schema = "wrong-schema".to_owned();
        several.canonicalization_profile = "wrong-profile".to_owned();
        several.policy_id = "wrong-policy".to_owned();
        assert_eq!(
            compose_specs(&expected, inherited(None), several).resolution_failure(),
            Some(&StandingResolutionFailureV3::SupportAuditSchemaMismatch)
        );
    }

    #[test]
    fn adjacent_outer_failure_precedence_is_frozen_end_to_end() {
        let expected = expected();
        let honest = AuditSpec::honest(&expected, Vec::new());

        let mut inherited_claim = inherited(None);
        inherited_claim.claim_id = "foreign".to_owned();
        inherited_claim.policy_id = "foreign-policy";
        assert_eq!(
            compose_specs(&expected, inherited_claim, honest.clone()).resolution_failure(),
            Some(&StandingResolutionFailureV3::InheritedClaimMismatch)
        );

        let mut inherited_policy = inherited(None);
        inherited_policy.policy_id = "foreign-policy";
        let mut schema = honest.clone();
        schema.schema = "wrong".to_owned();
        assert_eq!(
            compose_specs(&expected, inherited_policy, schema).resolution_failure(),
            Some(&StandingResolutionFailureV3::InheritedPolicyMismatch)
        );

        let mut pairs = Vec::new();
        let mut schema_profile = honest.clone();
        schema_profile.schema = "wrong".to_owned();
        schema_profile.canonicalization_profile = "wrong".to_owned();
        pairs.push((
            schema_profile,
            StandingResolutionFailureV3::SupportAuditSchemaMismatch,
        ));
        let mut profile_policy = honest.clone();
        profile_policy.canonicalization_profile = "wrong".to_owned();
        profile_policy.policy_id = "wrong".to_owned();
        pairs.push((
            profile_policy,
            StandingResolutionFailureV3::SupportAuditCanonicalizationProfileMismatch,
        ));
        let mut policy_admitted = honest.clone();
        policy_admitted.policy_id = "wrong".to_owned();
        policy_admitted.admitted_policy_id = "wrong".to_owned();
        pairs.push((
            policy_admitted,
            StandingResolutionFailureV3::SupportAuditPolicyMismatch,
        ));
        let mut admitted_origin = honest.clone();
        admitted_origin.admitted_policy_id = "wrong".to_owned();
        admitted_origin.origin_policy_id = "wrong".to_owned();
        pairs.push((
            admitted_origin,
            StandingResolutionFailureV3::SupportAuditAdmittedPolicyMismatch,
        ));
        let mut origin_prefix = honest.clone();
        origin_prefix.origin_policy_id = "wrong".to_owned();
        origin_prefix.prefix = replay_identity(1);
        pairs.push((
            origin_prefix,
            StandingResolutionFailureV3::SupportAuditOriginPolicyMismatch,
        ));
        let mut prefix_closure = honest;
        prefix_closure.prefix = replay_identity(1);
        prefix_closure.closure = different_closure_identity();
        pairs.push((
            prefix_closure,
            StandingResolutionFailureV3::VerifiedPrefixIdentityMismatch,
        ));

        for (audit, failure) in pairs {
            assert_eq!(
                compose_specs(&expected, inherited(None), audit).resolution_failure(),
                Some(&failure)
            );
        }
    }

    #[test]
    fn inherited_identity_mismatch_clears_top_level_authority_and_claim_wins() {
        let expected = expected();
        for status in [Status::Settled, Status::Refuted, Status::Supported] {
            let mut foreign_claim = inherited(Some(status));
            foreign_claim.claim_id = "foreign-claim".to_owned();
            foreign_claim.policy_id = "foreign-policy";
            foreign_claim.legacy_raw_standing = Some(Status::Settled);
            foreign_claim.currentness = StandingCurrentness::Current;
            let nested = foreign_claim.clone();
            let audit = AuditSpec::honest(&expected, Vec::new()).build();
            let nested_audit = audit.clone();
            let resolution = compose(&expected, foreign_claim, audit);
            assert_eq!(
                resolution.resolution_failure(),
                Some(&StandingResolutionFailureV3::InheritedClaimMismatch)
            );
            assert_eq!(resolution.claim_id(), CLAIM_ID);
            assert_eq!(resolution.governed_standing(), None);
            assert_eq!(resolution.legacy_raw_standing(), None);
            assert_eq!(resolution.currentness(), StandingCurrentness::Unknown);
            assert_eq!(resolution.inherited_v2(), &nested);
            assert_eq!(resolution.support_contribution_audit(), &nested_audit);

            let mut foreign_policy = inherited(Some(status));
            foreign_policy.policy_id = "foreign-policy";
            foreign_policy.legacy_raw_standing = Some(Status::Refuted);
            foreign_policy.currentness = StandingCurrentness::Current;
            let resolution = compose_specs(
                &expected,
                foreign_policy,
                AuditSpec::honest(&expected, Vec::new()),
            );
            assert_eq!(
                resolution.resolution_failure(),
                Some(&StandingResolutionFailureV3::InheritedPolicyMismatch)
            );
            assert_eq!(resolution.governed_standing(), None);
            assert_eq!(resolution.legacy_raw_standing(), None);
            assert_eq!(resolution.currentness(), StandingCurrentness::Unknown);
        }
    }

    #[test]
    fn later_outer_failure_preserves_valid_inherited_values_without_promotion() {
        let expected = expected();
        for status in [
            Some(Status::Settled),
            Some(Status::Refuted),
            Some(Status::Supported),
            None,
        ] {
            let mut audit = AuditSpec::honest(&expected, two_groups());
            audit.schema = "wrong-schema".to_owned();
            let mut inherited = inherited(status);
            inherited.legacy_raw_standing = Some(Status::Refuted);
            inherited.currentness = StandingCurrentness::Current;
            let resolution = compose_specs(&expected, inherited, audit);
            assert_eq!(resolution.governed_standing(), status);
            assert_eq!(resolution.legacy_raw_standing(), Some(Status::Refuted));
            assert_eq!(resolution.currentness(), StandingCurrentness::Current);
            assert!(resolution.application().is_none());
        }
    }

    #[test]
    fn every_later_outer_failure_class_preserves_inherited_supported() {
        fn assert_preserved(expected: &Expected, audit: AuditSpec) {
            let mut inherited = inherited(Some(Status::Supported));
            inherited.legacy_raw_standing = Some(Status::Refuted);
            inherited.currentness = StandingCurrentness::Current;
            let resolution = compose_specs(expected, inherited, audit);
            assert!(resolution.resolution_failure().is_some());
            assert_eq!(resolution.governed_standing(), Some(Status::Supported));
            assert_eq!(resolution.legacy_raw_standing(), Some(Status::Refuted));
            assert_eq!(resolution.currentness(), StandingCurrentness::Current);
            assert!(resolution.lane().is_none());
            assert!(resolution.application().is_none());
        }

        let expected = expected();
        let honest = AuditSpec::honest(&expected, Vec::new());
        let mut audits = Vec::new();
        let mut schema = honest.clone();
        schema.schema = "wrong".to_owned();
        audits.push(schema);
        let mut profile = honest.clone();
        profile.canonicalization_profile = "wrong".to_owned();
        audits.push(profile);
        let mut policy = honest.clone();
        policy.policy_id = "wrong".to_owned();
        audits.push(policy);
        let mut admitted = honest.clone();
        admitted.admitted_policy_id = "wrong".to_owned();
        audits.push(admitted);
        let mut origin = honest.clone();
        origin.origin_policy_id = "wrong".to_owned();
        audits.push(origin);
        let mut prefix = honest.clone();
        prefix.prefix = replay_identity(1);
        audits.push(prefix);
        let mut closure = honest;
        closure.closure = different_closure_identity();
        audits.push(closure);
        for audit in audits {
            assert_preserved(&expected, audit);
        }

        let mut lane = SupportSpec::honest("later-lane", "origin");
        lane.policy_id = "wrong".to_owned();
        assert_preserved(&expected, AuditSpec::honest(&expected, vec![lane.build()]));
        let mut target = SupportSpec::honest("later-target", "origin");
        target.namespace = stage_namespace_v0_for_tests("foreign", SCOPE);
        assert_preserved(
            &expected,
            AuditSpec::honest(&expected, vec![target.build()]),
        );
        let duplicate = SupportSpec::honest("later-duplicate", "origin");
        assert_preserved(
            &expected,
            AuditSpec::honest(
                &expected,
                vec![duplicate.clone().build(), duplicate.build()],
            ),
        );
    }

    #[test]
    fn lane_invariant_stage_is_complete_and_identity_ordered() {
        let expected = expected();
        let mut cases = Vec::new();

        let mut support_policy = SupportSpec::honest("one", "group");
        support_policy.policy_id = "wrong-support-policy".to_owned();
        cases.push(support_policy);
        let mut admitted_policy = SupportSpec::honest("one", "group");
        admitted_policy.admitted_policy_id = "wrong-admitted-policy".to_owned();
        cases.push(admitted_policy);
        let mut origin_policy = SupportSpec::honest("one", "group");
        origin_policy.origin_policy_id = "wrong-origin-policy".to_owned();
        cases.push(origin_policy);
        let mut namespace_policy = SupportSpec::honest("one", "group");
        namespace_policy.namespace =
            stage_origin_comparison_namespace_v0_for_tests("wrong-origin-policy", CLAIM_ID, SCOPE);
        cases.push(namespace_policy);
        let mut evidence = SupportSpec::honest("one", "group");
        evidence.evidence_kind = EvidenceKind::ModelSelfReport.as_str().to_owned();
        cases.push(evidence);
        let mut domain = SupportSpec::honest("one", "group");
        domain.claim_domain = ClaimDomain::Interpretation.as_str().to_owned();
        cases.push(domain);
        let mut ceiling = SupportSpec::honest("one", "group");
        ceiling.support_ceiling = Status::Settled;
        cases.push(ceiling);

        for spec in cases {
            let identity = spec.contribution.clone();
            let resolution = compose_specs(
                &expected,
                inherited(None),
                AuditSpec::honest(&expected, vec![spec.build()]),
            );
            assert_eq!(
                resolution.resolution_failure(),
                Some(&StandingResolutionFailureV3::LaneInvariantMismatch {
                    contribution: identity,
                })
            );
        }

        let mut invalid_a = SupportSpec::honest("a", "group");
        invalid_a.policy_id = "wrong".to_owned();
        let identity_a = invalid_a.contribution.clone();
        let mut invalid_z = SupportSpec::honest("z", "group");
        invalid_z.origin_policy_id = "wrong".to_owned();
        for contributions in [
            vec![invalid_z.clone().build(), invalid_a.clone().build()],
            vec![invalid_a.clone().build(), invalid_z.clone().build()],
        ] {
            assert_eq!(
                compose_specs(
                    &expected,
                    inherited(None),
                    AuditSpec::honest(&expected, contributions),
                )
                .resolution_failure(),
                Some(&StandingResolutionFailureV3::LaneInvariantMismatch {
                    contribution: identity_a.clone(),
                })
            );
        }
    }

    #[test]
    fn target_and_scope_drift_are_separate_and_lane_class_wins_globally() {
        let expected = expected();
        let mut target = SupportSpec::honest("target", "group");
        target.namespace = stage_origin_comparison_namespace_v0_for_tests(
            ORIGIN_ADMISSION_POLICY_ID_V0,
            "foreign-claim",
            SCOPE,
        );
        let mut namespace_scope = SupportSpec::honest("namespace-scope", "group");
        namespace_scope.namespace = stage_origin_comparison_namespace_v0_for_tests(
            ORIGIN_ADMISSION_POLICY_ID_V0,
            CLAIM_ID,
            "scope:foreign",
        );
        let mut contribution_scope = SupportSpec::honest("contribution-scope", "group");
        contribution_scope.contribution = identity(CLAIM_ID, "contribution-scope", "scope:foreign");

        for spec in [target, namespace_scope, contribution_scope] {
            let identity = spec.contribution.clone();
            let resolution = compose_specs(
                &expected,
                inherited(None),
                AuditSpec::honest(&expected, vec![spec.build()]),
            );
            assert_eq!(
                resolution.resolution_failure(),
                Some(&StandingResolutionFailureV3::TargetScopeMismatch {
                    contribution: identity,
                })
            );
        }

        let mut smaller_target = SupportSpec::honest("a", "group");
        smaller_target.namespace = stage_origin_comparison_namespace_v0_for_tests(
            ORIGIN_ADMISSION_POLICY_ID_V0,
            "foreign-claim",
            SCOPE,
        );
        let mut larger_lane = SupportSpec::honest("z", "group");
        larger_lane.policy_id = "wrong".to_owned();
        let larger_identity = larger_lane.contribution.clone();
        for contributions in [
            vec![smaller_target.clone().build(), larger_lane.clone().build()],
            vec![larger_lane.clone().build(), smaller_target.clone().build()],
        ] {
            assert_eq!(
                compose_specs(
                    &expected,
                    inherited(None),
                    AuditSpec::honest(&expected, contributions),
                )
                .resolution_failure(),
                Some(&StandingResolutionFailureV3::LaneInvariantMismatch {
                    contribution: larger_identity.clone(),
                })
            );
        }
    }

    #[test]
    fn duplicate_identity_never_amplifies_and_is_permutation_invariant() {
        let expected = expected();
        let first = SupportSpec::honest("duplicate", "origin-alpha");
        let identity = first.contribution.clone();
        let mut second = first.clone();
        second.origin_group = "origin-beta".to_owned();

        for contributions in [
            vec![first.clone().build(), second.clone().build()],
            vec![second.clone().build(), first.clone().build()],
        ] {
            let resolution = compose_specs(
                &expected,
                inherited(None),
                AuditSpec::honest(&expected, contributions),
            );
            assert_eq!(
                resolution.resolution_failure(),
                Some(
                    &StandingResolutionFailureV3::DuplicateContributionIdentity {
                        contribution: identity.clone(),
                    }
                )
            );
            assert!(resolution.lane().is_none());
            assert!(resolution.application().is_none());
            assert_eq!(resolution.governed_standing(), None);
        }

        let same_group = vec![first.clone().build(), first.clone().build()];
        assert_eq!(
            compose_specs(
                &expected,
                inherited(None),
                AuditSpec::honest(&expected, same_group),
            )
            .resolution_failure(),
            Some(
                &StandingResolutionFailureV3::DuplicateContributionIdentity {
                    contribution: identity,
                }
            )
        );
    }

    #[test]
    fn first_duplicate_key_is_identity_ordered_across_permutations() {
        let expected = expected();
        let a = SupportSpec::honest("a-duplicate", "origin-a");
        let z = SupportSpec::honest("z-duplicate", "origin-z");
        let first = std::cmp::min(a.contribution.clone(), z.contribution.clone());
        for supports in [
            vec![
                z.clone().build(),
                a.clone().build(),
                z.clone().build(),
                a.clone().build(),
            ],
            vec![
                a.clone().build(),
                z.clone().build(),
                a.clone().build(),
                z.clone().build(),
            ],
        ] {
            let resolution = compose_specs(
                &expected,
                inherited(None),
                AuditSpec::honest(&expected, supports),
            );
            assert_eq!(
                resolution.resolution_failure(),
                Some(
                    &StandingResolutionFailureV3::DuplicateContributionIdentity {
                        contribution: first.clone(),
                    }
                )
            );
        }
    }

    #[test]
    fn foreign_malformed_support_is_ignored_before_lane_validation() {
        let expected = expected();
        let mut foreign = SupportSpec::honest("foreign-malformed", "foreign");
        foreign.contribution = identity("foreign-claim", "foreign-malformed", "scope:foreign");
        foreign.namespace = stage_origin_comparison_namespace_v0_for_tests(
            "foreign-origin-policy",
            "wrong-namespace-target",
            "scope:wrong",
        );
        foreign.policy_id = "foreign-support-policy".to_owned();
        let resolution = compose_specs(
            &expected,
            inherited(None),
            AuditSpec::honest(
                &expected,
                vec![
                    foreign.build(),
                    SupportSpec::honest("alpha", "origin-alpha").build(),
                    SupportSpec::honest("beta", "origin-beta").build(),
                ],
            ),
        );
        assert!(resolution.resolution_failure().is_none());
        assert_eq!(resolution.ignored_contributions().len(), 1);
        assert_eq!(resolution.governed_standing(), Some(Status::Supported));
    }

    #[test]
    fn missing_replayed_scope_rejects_any_staged_selected_contribution() {
        let expected = expected();
        let audit = AuditSpec::honest(
            &expected,
            vec![SupportSpec::honest("missing-scope", "origin").build()],
        )
        .build();
        let identity = audit.support_contributions()[0].contribution().clone();
        let resolution = compose_standing_resolution_v3(StandingV3CompositionInputV0::new(
            CLAIM_ID,
            None,
            expected.prefix,
            expected.closure,
            inherited(None),
            audit,
        ));
        assert_eq!(
            resolution.resolution_failure(),
            Some(&StandingResolutionFailureV3::TargetScopeMismatch {
                contribution: identity,
            })
        );
    }

    #[test]
    fn same_digest_and_selector_material_never_vetoes_distinct_groups() {
        let expected = expected();
        let shared_digest = "a".repeat(64);
        let shared_selector = ArtifactProvenanceAnchorSelectorV0::new(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            "b".repeat(64),
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-shared-selector",
        );
        let mut alpha = SupportSpec::honest("shared-alpha", "origin-alpha");
        alpha.contribution = stage_contribution_identity_v0_for_tests(
            CLAIM_ID,
            "evidence-alpha",
            "edge-alpha",
            SCOPE,
            &shared_digest,
        );
        alpha.selectors = vec![shared_selector.clone()];
        let mut beta = SupportSpec::honest("shared-beta", "origin-beta");
        beta.contribution = stage_contribution_identity_v0_for_tests(
            CLAIM_ID,
            "evidence-beta",
            "edge-beta",
            SCOPE,
            &shared_digest,
        );
        beta.selectors = vec![shared_selector];
        let resolution = compose_specs(
            &expected,
            inherited(None),
            AuditSpec::honest(&expected, vec![beta.build(), alpha.build()]),
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Supported));
        assert_eq!(resolution.lane().unwrap().groups().len(), 2);
    }

    #[test]
    fn grouping_ignored_order_and_distinct_group_count_are_structural() {
        let expected = expected();
        let mut foreign_z = SupportSpec::honest("foreign-z", "ignored");
        foreign_z.contribution = identity("foreign-claim", "z", "scope:foreign");
        foreign_z.namespace = stage_namespace_v0_for_tests("foreign-claim", "scope:foreign");
        let mut foreign_a = SupportSpec::honest("foreign-a", "ignored");
        foreign_a.contribution = identity("foreign-claim", "a", "scope:foreign");
        foreign_a.namespace = stage_namespace_v0_for_tests("foreign-claim", "scope:foreign");
        let contributions = vec![
            SupportSpec::honest("z", "origin-beta").build(),
            foreign_z.build(),
            SupportSpec::honest("b", "origin-alpha").build(),
            foreign_a.build(),
            SupportSpec::honest("a", "origin-alpha").build(),
            SupportSpec::honest("c", "Origin-Alpha").build(),
        ];
        let resolution = compose_specs(
            &expected,
            inherited(None),
            AuditSpec::honest(&expected, contributions),
        );
        let lane = resolution.lane().unwrap();
        assert_eq!(
            lane.groups()
                .iter()
                .map(CorroborationGroupV0::origin_group)
                .collect::<Vec<_>>(),
            ["Origin-Alpha", "origin-alpha", "origin-beta"]
        );
        assert_eq!(lane.groups()[1].contributions().len(), 2);
        assert_eq!(
            lane.groups()[1]
                .contributions()
                .iter()
                .map(|value| value.contribution().justification_edge_id())
                .collect::<Vec<_>>(),
            ["edge-a", "edge-b"]
        );
        assert_eq!(
            resolution
                .ignored_contributions()
                .iter()
                .map(|value| value.contribution().justification_edge_id())
                .collect::<Vec<_>>(),
            ["edge-a", "edge-z"]
        );
        assert_eq!(
            resolution.application().unwrap().context(),
            &StandingPolicyContextV3::Corroborated {
                distinct_group_count: 3,
            }
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Supported));
    }

    #[test]
    fn completion_blockers_and_inherited_failure_order_are_exact() {
        let expected = expected();

        let mut incomplete_supported = AuditSpec::honest(&expected, two_groups());
        incomplete_supported.completion =
            SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                unavailable_binding_selectors: Vec::new(),
            };
        let resolution = compose_specs(
            &expected,
            inherited(Some(Status::Supported)),
            incomplete_supported,
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Supported));
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::SupportAuditIncomplete]
        );
        assert!(resolution.lane().is_none());
        assert!(resolution.application().is_none());
        assert!(resolution.ignored_contributions().is_empty());

        let mut rejected_supported = AuditSpec::honest(&expected, two_groups());
        rejected_supported.completion =
            SupportContributionAuditCompletionV0::UpstreamCompositionRejected {
                failure: SupportContributionCompositionFailureV0::UpstreamSchemaMismatch,
            };
        let resolution = compose_specs(
            &expected,
            inherited(Some(Status::Supported)),
            rejected_supported,
        );
        assert_eq!(resolution.governed_standing(), Some(Status::Supported));
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::SupportAuditRejected]
        );
        assert!(resolution.lane().is_none());
        assert!(resolution.application().is_none());
        assert!(resolution.ignored_contributions().is_empty());

        let mut inherited_failed = inherited(Some(Status::Conjectured));
        inherited_failed.resolution_failure =
            Some(StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 0 });

        let mut incomplete = AuditSpec::honest(&expected, two_groups());
        incomplete.completion =
            SupportContributionAuditCompletionV0::AdmittedContributionIncomplete {
                unavailable_binding_selectors: Vec::new(),
            };
        let resolution = compose_specs(&expected, inherited_failed.clone(), incomplete);
        assert_eq!(
            resolution.blockers(),
            [
                StandingBlockerV3::SupportAuditIncomplete,
                StandingBlockerV3::InheritedV2ResolutionFailed,
            ]
        );
        assert!(resolution.lane().is_none());
        assert!(resolution.application().is_none());
        assert!(resolution.ignored_contributions().is_empty());
        assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));

        let mut rejected = AuditSpec::honest(&expected, two_groups());
        rejected.completion = SupportContributionAuditCompletionV0::UpstreamCompositionRejected {
            failure: SupportContributionCompositionFailureV0::UpstreamSchemaMismatch,
        };
        let resolution = compose_specs(&expected, inherited_failed.clone(), rejected);
        assert_eq!(
            resolution.blockers(),
            [
                StandingBlockerV3::SupportAuditRejected,
                StandingBlockerV3::InheritedV2ResolutionFailed,
            ]
        );
        assert!(resolution.lane().is_none());
        assert!(resolution.application().is_none());

        let resolution = compose_specs(
            &expected,
            inherited_failed,
            AuditSpec::honest(&expected, two_groups()),
        );
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::InheritedV2ResolutionFailed]
        );
        assert!(resolution.lane().is_some());
        assert!(resolution.application().is_none());
        assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    }

    #[test]
    fn standing_precedence_and_never_settled_law_are_exact() {
        let expected = expected();
        for (inherited, expected_standing) in [
            (Some(Status::Settled), Some(Status::Settled)),
            (Some(Status::Refuted), Some(Status::Refuted)),
            (Some(Status::Supported), Some(Status::Supported)),
            (Some(Status::Conjectured), Some(Status::Supported)),
            (None, Some(Status::Supported)),
        ] {
            let resolution = compose_specs(
                &expected,
                self::inherited(inherited),
                AuditSpec::honest(&expected, two_groups()),
            );
            assert_eq!(resolution.governed_standing(), expected_standing);
            assert_eq!(
                resolution.application().unwrap().achieved_standing(),
                Some(Status::Supported)
            );
            assert_ne!(
                resolution.application().unwrap().achieved_standing(),
                Some(Status::Settled)
            );
        }

        let one_group = compose_specs(
            &expected,
            inherited(None),
            AuditSpec::honest(
                &expected,
                vec![SupportSpec::honest("one", "origin-one").build()],
            ),
        );
        assert_eq!(one_group.governed_standing(), None);
        assert!(one_group.application().is_none());
        assert_eq!(
            one_group.blockers(),
            [StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count: 1,
            }]
        );
    }

    #[test]
    fn legacy_raw_values_neither_settle_nor_veto_corroboration() {
        let expected = expected();
        for raw in [Status::Settled, Status::Refuted] {
            let mut inherited = inherited(Some(Status::Conjectured));
            inherited.legacy_raw_standing = Some(raw);
            let resolution = compose_specs(
                &expected,
                inherited,
                AuditSpec::honest(&expected, two_groups()),
            );
            assert_eq!(resolution.legacy_raw_standing(), Some(raw));
            assert_eq!(resolution.governed_standing(), Some(Status::Supported));
            assert_ne!(resolution.governed_standing(), Some(Status::Settled));
        }
    }

    #[test]
    fn canonical_bytes_are_ordered_compact_and_repeatable() {
        let expected = expected();
        let resolution = compose_specs(
            &expected,
            inherited(None),
            AuditSpec::honest(&expected, Vec::new()),
        );
        assert_eq!(resolution.governed_standing(), None);
        assert!(resolution.lane().is_none());
        assert!(resolution.application().is_none());
        assert!(resolution.ignored_contributions().is_empty());
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count: 0,
            }]
        );
        let bytes = resolution.canonical_bytes();
        assert_eq!(bytes, resolution.clone().canonical_bytes());
        assert_eq!(bytes.first(), Some(&b'{'));
        assert_eq!(bytes.last(), Some(&b'}'));
        assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
        assert!(!bytes.ends_with(b"\n"));
        let text = String::from_utf8(bytes).unwrap();
        assert!(!text.contains("\"resolution_failure\""));
        assert!(text.contains("\"lane\":null,\"application\":null"));
        let ordered_fields = [
            "\"claim_id\"",
            "\"governed_standing\"",
            "\"legacy_raw_standing\"",
            "\"currentness\"",
            "\"policy_id\"",
            "\"verified_prefix_identity\"",
            "\"closure_identity\"",
            "\"inherited_v2\"",
            "\"support_contribution_audit\"",
            "\"lane\"",
            "\"application\"",
            "\"ignored_contributions\"",
            "\"blockers\"",
        ];
        let mut cursor = 0;
        for field in ordered_fields {
            let relative = text[cursor..].find(field).unwrap();
            cursor += relative + field.len();
        }
    }

    #[test]
    fn defensive_fixtures_match_the_single_composer_and_generic_json_shape() {
        fn assert_fixture(
            fixture: &[u8],
            resolution: StandingResolutionV3,
            expected_len: usize,
            expected_sha256: &str,
        ) -> serde_json::Value {
            assert_eq!(resolution.canonical_bytes(), fixture);
            assert_eq!(fixture.len(), expected_len);
            assert_eq!(hex::encode(Sha256::digest(fixture)), expected_sha256);
            assert_eq!(fixture.first(), Some(&b'{'));
            assert_eq!(fixture.last(), Some(&b'}'));
            assert!(!fixture.starts_with(&[0xef, 0xbb, 0xbf]));
            assert!(!fixture.ends_with(b"\n"));
            assert!(!fixture.ends_with(b"\r"));
            serde_json::from_slice(fixture).unwrap()
        }

        let rejected = assert_fixture(
            REJECTED_FIXTURE,
            rejected_fixture_resolution(),
            1_668,
            "49281b375cc57c09ec4c8345f31c1b241ebc841d70819f08d9729d79f1cad897",
        );
        assert_eq!(rejected["blockers"][0]["kind"], "support_audit_rejected");
        assert!(rejected["lane"].is_null());
        assert!(rejected["application"].is_null());

        let inherited = assert_fixture(
            INHERITED_FAILURE_FIXTURE,
            inherited_failure_fixture_resolution(),
            4_788,
            "d71f54f2646372f71a5714b55c7492bb15dcd7b5a71483e52578040e684a1931",
        );
        assert_eq!(
            inherited["blockers"][0]["kind"],
            "inherited_v2_resolution_failed"
        );
        assert_eq!(inherited["lane"]["groups"].as_array().unwrap().len(), 2);
        assert!(inherited["application"].is_null());
        assert_eq!(
            inherited["inherited_v2"]["resolution_failure"]["kind"],
            "inherited_v1_candidate_mismatch"
        );

        let global = assert_fixture(
            OUTER_FAILURE_FIXTURE,
            outer_global_fixture_resolution(),
            1_607,
            "ad5dce19f5f3a165d2e7acb6736e0032a278fbddcb0caa0a13ec981aa72a3458",
        );
        assert_eq!(
            global["resolution_failure"]["kind"],
            "support_audit_policy_mismatch"
        );
        assert!(global["lane"].is_null());

        let lane = assert_fixture(
            OUTER_LANE_FIXTURE,
            outer_lane_fixture_resolution(),
            2_614,
            "6a10c3559313cef0ed7c16c227759f016c8261d31f86e47d0cc91793ad90527c",
        );
        assert_eq!(
            lane["resolution_failure"]["kind"],
            "lane_invariant_mismatch"
        );
        assert_eq!(
            lane["resolution_failure"]["contribution"]["justification_edge_id"],
            "edge-fixture-lane"
        );

        let target = assert_fixture(
            OUTER_TARGET_FIXTURE,
            outer_target_fixture_resolution(),
            2_630,
            "032ce9ce6faa3971882cc0605ab69df97d2c2145f8f6c63b407d32b7e723dc6e",
        );
        assert_eq!(
            target["resolution_failure"]["kind"],
            "target_scope_mismatch"
        );
        assert_eq!(
            target["resolution_failure"]["contribution"]["justification_edge_id"],
            "edge-fixture-target"
        );
    }

    #[test]
    fn combination_truth_table_is_exact_for_every_current_status() {
        // Compatibility law: the complete 12-case
        // `Option<Status> x corroborated` matrix. Every current variant
        // appears explicitly; a future variant is a compile error in
        // `combine_standing`, never a silent promotion.
        let cases = [
            (None, false, None),
            (None, true, Some(Status::Supported)),
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
        for (inherited, corroborated, expected) in cases {
            assert_eq!(
                combine_standing(inherited, corroborated),
                expected,
                "v3 combination changed for inherited={inherited:?} corroborated={corroborated}"
            );
        }
    }

    #[test]
    fn combination_dominance_and_none_laws_are_exact() {
        for corroborated in [false, true] {
            // Settlement remains dominant over corroboration.
            assert_eq!(
                combine_standing(Some(Status::Settled), corroborated),
                Some(Status::Settled)
            );
            // Governed refutation remains dominant over corroboration.
            assert_eq!(
                combine_standing(Some(Status::Refuted), corroborated),
                Some(Status::Refuted)
            );
            // Existing support is preserved with either Boolean value.
            assert_eq!(
                combine_standing(Some(Status::Supported), corroborated),
                Some(Status::Supported)
            );
        }
        // Unlike v2, successful corroboration promotes an absent inherited
        // standing to `Supported`; without corroboration, `None` is kept.
        assert_eq!(combine_standing(None, false), None);
        assert_eq!(combine_standing(None, true), Some(Status::Supported));
    }
}
