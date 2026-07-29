//! Standing-inert claim-inline predicate edge-lane resolution.
//!
//! The authority-bearing outcome and receipt are resolver-created audit
//! values. Callers may inspect and serialize values they obtained, but cannot
//! construct, deserialize, mutate, deconstruct, or feed them back into an
//! authority path.
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! let _ = ClaimInlinePredicateEdgeLaneOutcomeV0 {};
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! let _ = ClaimInlinePredicateEdgeLaneReceiptV0 {
//!     schema: String::new(),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//!     ClaimInlinePredicateEdgeLaneReceiptV0,
//! };
//! fn no_public_payload(
//!     receipt: ClaimInlinePredicateEdgeLaneReceiptV0,
//! ) -> ClaimInlinePredicateEdgeLaneOutcomeV0 {
//!     ClaimInlinePredicateEdgeLaneOutcomeV0::SupportEligible(receipt)
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! let _: ClaimInlinePredicateEdgeLaneOutcomeV0 =
//!     serde_json::from_str(r#"{"outcome":"ineligible"}"#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! let _: ClaimInlinePredicateEdgeLaneReceiptV0 =
//!     serde_json::from_str("{}").unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! let _ = ClaimInlinePredicateEdgeLaneOutcomeV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! let _ = ClaimInlinePredicateEdgeLaneReceiptV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneOutcomeKindV0,
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//! };
//! let _: ClaimInlinePredicateEdgeLaneOutcomeV0 =
//!     ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible.into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneFailureV0,
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//! };
//! let _ = ClaimInlinePredicateEdgeLaneOutcomeV0::try_from(
//!     ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
//! );
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_mutation(value: &mut ClaimInlinePredicateEdgeLaneOutcomeV0) {
//!     value.requested_claim_id = "replacement".into();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_mutation(value: &mut ClaimInlinePredicateEdgeLaneReceiptV0) {
//!     value.edge_id = "replacement".into();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//!     ClaimInlinePredicateEdgeLaneReceiptV0,
//! };
//! fn no_consuming_receipt(
//!     value: ClaimInlinePredicateEdgeLaneOutcomeV0,
//! ) -> ClaimInlinePredicateEdgeLaneReceiptV0 {
//!     value.into_receipt()
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneFailureV0,
//!     ClaimInlinePredicateEdgeLaneOutcomeV0,
//! };
//! fn no_consuming_failure(
//!     value: ClaimInlinePredicateEdgeLaneOutcomeV0,
//! ) -> ClaimInlinePredicateEdgeLaneFailureV0 {
//!     value.into_failure()
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_debug(value: &ClaimInlinePredicateEdgeLaneOutcomeV0) {
//!     let _ = format!("{value:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_debug(value: &ClaimInlinePredicateEdgeLaneReceiptV0) {
//!     let _ = format!("{value:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_display(value: &ClaimInlinePredicateEdgeLaneOutcomeV0) {
//!     let _ = format!("{value}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_display(value: &ClaimInlinePredicateEdgeLaneReceiptV0) {
//!     let _ = format!("{value}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_error(value: &ClaimInlinePredicateEdgeLaneOutcomeV0) {
//!     let _: &(dyn std::error::Error + 'static) = value;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_error(value: &ClaimInlinePredicateEdgeLaneReceiptV0) {
//!     let _: &(dyn std::error::Error + 'static) = value;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_equality(left: &ClaimInlinePredicateEdgeLaneOutcomeV0, right: &ClaimInlinePredicateEdgeLaneOutcomeV0) {
//!     let _ = left == right;
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_equality(left: &ClaimInlinePredicateEdgeLaneReceiptV0, right: &ClaimInlinePredicateEdgeLaneReceiptV0) {
//!     let _ = left == right;
//! }
//! ```
//!
//! ```compile_fail
//! use std::hash::{Hash, Hasher};
//! use magpie_claims::ClaimInlinePredicateEdgeLaneOutcomeV0;
//! fn no_hash<H: Hasher>(value: &ClaimInlinePredicateEdgeLaneOutcomeV0, state: &mut H) {
//!     value.hash(state);
//! }
//! ```
//!
//! ```compile_fail
//! use std::hash::{Hash, Hasher};
//! use magpie_claims::ClaimInlinePredicateEdgeLaneReceiptV0;
//! fn no_receipt_hash<H: Hasher>(value: &ClaimInlinePredicateEdgeLaneReceiptV0, state: &mut H) {
//!     value.hash(state);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneOutcomeV0, StandingReplaySnapshot,
//! };
//! fn no_outcome_reinjection(
//!     snapshot: &StandingReplaySnapshot,
//!     value: &ClaimInlinePredicateEdgeLaneOutcomeV0,
//! ) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", value,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneReceiptV0, StandingReplaySnapshot,
//! };
//! fn no_receipt_reinjection(
//!     snapshot: &StandingReplaySnapshot,
//!     value: &ClaimInlinePredicateEdgeLaneReceiptV0,
//! ) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", value,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlinePredicateEdgeLaneRelationV0, StandingReplaySnapshot,
//! };
//! fn no_relation_reinjection(
//!     snapshot: &StandingReplaySnapshot,
//!     value: ClaimInlinePredicateEdgeLaneRelationV0,
//! ) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", value,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//! fn no_serialized_authority(snapshot: &StandingReplaySnapshot, bytes: &[u8]) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", bytes,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     StandingReplaySnapshot, TypedClaimNode, TypedEvidenceNode,
//!     TypedJustificationEdge,
//! };
//! fn no_caller_nodes(
//!     snapshot: &StandingReplaySnapshot,
//!     claim: &TypedClaimNode,
//!     evidence: &TypedEvidenceNode,
//!     edge: &TypedJustificationEdge,
//! ) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", claim, evidence, edge,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//! fn no_second_snapshot(
//!     selected: &StandingReplaySnapshot,
//!     other: &StandingReplaySnapshot,
//! ) {
//!     let _ = selected.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", other,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! let _ = magpie_claims::resolve_claim_inline_predicate_edge_lane_v0(
//!     "claim", "evidence", "edge",
//! );
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingView;
//! fn no_standing_view_method(view: &StandingView) {
//!     let _ = view.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge",
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::OriginAdmissionReplayContextV0;
//! fn no_origin_context_method(context: &OriginAdmissionReplayContextV0) {
//!     let _ = context.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge",
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//! fn no_raw_composer(snapshot: &StandingReplaySnapshot, bytes: &[u8]) {
//!     let _ = snapshot.compose_claim_inline_predicate_edge_lane_raw_v0(bytes);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{policy::ClaimDomain, StandingReplaySnapshot};
//! fn no_selected_policy(snapshot: &StandingReplaySnapshot) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", ClaimDomain::ExactMachineCheckable,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//! use magpie_log::Status;
//! fn no_selected_ceiling(snapshot: &StandingReplaySnapshot) {
//!     let _ = snapshot.resolve_claim_inline_predicate_edge_lane_v0(
//!         "claim", "evidence", "edge", Status::Settled,
//!     );
//! }
//! ```

use magpie_log::Status;
use serde::{Serialize, Serializer};

use crate::inline_predicate_attestation::{
    resolve_inline_predicate_attestation_after_claim_v0,
    ResolvedInlinePredicateAttestationBindingV0,
};
use crate::policy::{refutation_ceiling, support_ceiling, ClaimDomain, EvidenceKind};
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::InlinePredicateAttestationBindingFailureV0;

use super::{
    evaluate_resolved_claim_inline_sha256_v0, resolve_claim_inline_subject_v0,
    ResolvedClaimInlineSha256RelationV0,
};

const EDGE_LANE_SCHEMA_V0: &str = "magpie-claim-inline-predicate-edge-lane-v0";
const OUTCOME_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-claim-inline-predicate-edge-lane-outcome-json-v0";

/// Closed outcome classification for claim-inline predicate edge lanes v0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneOutcomeKindV0 {
    SupportEligible,
    RefutationEligible,
    Ineligible,
    ResolutionFailed,
}

/// Closed eligible-lane vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneKindV0 {
    Support,
    Refutation,
}

/// Closed claim-owned digest-relation vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneRelationV0 {
    DigestEqual,
    DigestUnequal,
}

/// Closed selected-edge-kind vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneEdgeKindV0 {
    Supports,
    Contradicts,
}

/// Closed non-authorizing matrix mismatch vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneIneligibleReasonV0 {
    DigestUnequalDoesNotSupport,
    DigestEqualDoesNotRefute,
}

/// Closed failure vocabulary in frozen first-failure order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlinePredicateEdgeLaneFailureV0 {
    AttestationBindingFailed(InlinePredicateAttestationBindingFailureV0),
    MissingEdge,
    UnsupportedEdgeKind,
    EdgeSourceBindingMismatch,
    EdgeTargetBindingMismatch,
    EdgeScopeBindingMismatch,
    SupportCeilingInvariantMismatch,
    RefutationCeilingInvariantMismatch,
}

/// Exact audit basis for an eligible lane.
///
/// Construction is private. Field declaration order is the canonical receipt
/// order.
#[derive(Clone)]
pub struct ClaimInlinePredicateEdgeLaneReceiptV0 {
    schema: String,
    lane: ClaimInlinePredicateEdgeLaneKindV0,
    predicate_id: String,
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    scope_ref: String,
    predicate_relation: ClaimInlinePredicateEdgeLaneRelationV0,
    edge_kind: ClaimInlinePredicateEdgeLaneEdgeKindV0,
    candidate_ceiling: Status,
}

impl ClaimInlinePredicateEdgeLaneReceiptV0 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn lane(&self) -> ClaimInlinePredicateEdgeLaneKindV0 {
        self.lane
    }

    pub fn predicate_id(&self) -> &str {
        &self.predicate_id
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }

    pub fn edge_id(&self) -> &str {
        &self.edge_id
    }

    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }

    pub fn predicate_relation(&self) -> ClaimInlinePredicateEdgeLaneRelationV0 {
        self.predicate_relation
    }

    pub fn edge_kind(&self) -> ClaimInlinePredicateEdgeLaneEdgeKindV0 {
        self.edge_kind
    }

    pub fn candidate_ceiling(&self) -> Status {
        self.candidate_ceiling
    }
}

#[derive(Clone)]
enum ClaimInlinePredicateEdgeLaneOutcomeDataV0 {
    SupportEligible(ClaimInlinePredicateEdgeLaneReceiptV0),
    RefutationEligible(ClaimInlinePredicateEdgeLaneReceiptV0),
    Ineligible {
        requested_claim_id: String,
        requested_evidence_id: String,
        requested_edge_id: String,
        reason: ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
    },
    ResolutionFailed {
        requested_claim_id: String,
        requested_evidence_id: String,
        requested_edge_id: String,
        failure: ClaimInlinePredicateEdgeLaneFailureV0,
    },
}

/// Opaque resolver-created claim-inline predicate edge-lane outcome.
#[derive(Clone)]
pub struct ClaimInlinePredicateEdgeLaneOutcomeV0 {
    data: ClaimInlinePredicateEdgeLaneOutcomeDataV0,
}

impl ClaimInlinePredicateEdgeLaneOutcomeV0 {
    pub fn kind(&self) -> ClaimInlinePredicateEdgeLaneOutcomeKindV0 {
        match self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(_) => {
                ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(_) => {
                ClaimInlinePredicateEdgeLaneOutcomeKindV0::RefutationEligible
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible { .. } => {
                ClaimInlinePredicateEdgeLaneOutcomeKindV0::Ineligible
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed { .. } => {
                ClaimInlinePredicateEdgeLaneOutcomeKindV0::ResolutionFailed
            }
        }
    }

    pub fn requested_claim_id(&self) -> &str {
        match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt) => {
                receipt.claim_id()
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible {
                requested_claim_id, ..
            }
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                ..
            } => requested_claim_id,
        }
    }

    pub fn requested_evidence_id(&self) -> &str {
        match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt) => {
                receipt.evidence_id()
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible {
                requested_evidence_id,
                ..
            }
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_evidence_id,
                ..
            } => requested_evidence_id,
        }
    }

    pub fn requested_edge_id(&self) -> &str {
        match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt) => {
                receipt.edge_id()
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible {
                requested_edge_id, ..
            }
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_edge_id,
                ..
            } => requested_edge_id,
        }
    }

    pub fn receipt(&self) -> Option<&ClaimInlinePredicateEdgeLaneReceiptV0> {
        match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt) => {
                Some(receipt)
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible { .. }
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed { .. } => None,
        }
    }

    pub fn ineligible_reason(&self) -> Option<ClaimInlinePredicateEdgeLaneIneligibleReasonV0> {
        match self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible { reason, .. } => Some(reason),
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(_)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(_)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed { .. } => None,
        }
    }

    pub fn failure(&self) -> Option<&ClaimInlinePredicateEdgeLaneFailureV0> {
        match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed { failure, .. } => {
                Some(failure)
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(_)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(_)
            | ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible { .. } => None,
        }
    }

    /// Deterministic one-way audit bytes under
    /// `magpie-claim-inline-predicate-edge-lane-outcome-json-v0`.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let _canonicalization_profile = OUTCOME_CANONICALIZATION_PROFILE_V0;
        serde_json::to_vec(self)
            .expect("ClaimInlinePredicateEdgeLaneOutcomeV0 is always serializable")
    }

    fn eligible(receipt: ClaimInlinePredicateEdgeLaneReceiptV0) -> Self {
        let data = match receipt.lane {
            ClaimInlinePredicateEdgeLaneKindV0::Support => {
                ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt)
            }
            ClaimInlinePredicateEdgeLaneKindV0::Refutation => {
                ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt)
            }
        };
        Self { data }
    }

    fn ineligible(
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
        reason: ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
    ) -> Self {
        Self {
            data: ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible {
                requested_claim_id: claim_id.to_owned(),
                requested_evidence_id: evidence_id.to_owned(),
                requested_edge_id: edge_id.to_owned(),
                reason,
            },
        }
    }

    fn resolution_failed(
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
        failure: ClaimInlinePredicateEdgeLaneFailureV0,
    ) -> Self {
        Self {
            data: ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_claim_id: claim_id.to_owned(),
                requested_evidence_id: evidence_id.to_owned(),
                requested_edge_id: edge_id.to_owned(),
                failure,
            },
        }
    }
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum CandidateCeilingWireV0 {
    Settled,
    Refuted,
}

#[derive(Serialize)]
struct ClaimInlinePredicateEdgeLaneReceiptWireV0<'a> {
    schema: &'a str,
    lane: ClaimInlinePredicateEdgeLaneKindV0,
    predicate_id: &'a str,
    claim_id: &'a str,
    evidence_id: &'a str,
    edge_id: &'a str,
    scope_ref: &'a str,
    predicate_relation: ClaimInlinePredicateEdgeLaneRelationV0,
    edge_kind: ClaimInlinePredicateEdgeLaneEdgeKindV0,
    candidate_ceiling: CandidateCeilingWireV0,
}

impl Serialize for ClaimInlinePredicateEdgeLaneReceiptV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let candidate_ceiling = match self.candidate_ceiling {
            Status::Settled => CandidateCeilingWireV0::Settled,
            Status::Refuted => CandidateCeilingWireV0::Refuted,
            _ => unreachable!("eligible receipts use only compiled Settled or Refuted ceilings"),
        };
        ClaimInlinePredicateEdgeLaneReceiptWireV0 {
            schema: &self.schema,
            lane: self.lane,
            predicate_id: &self.predicate_id,
            claim_id: &self.claim_id,
            evidence_id: &self.evidence_id,
            edge_id: &self.edge_id,
            scope_ref: &self.scope_ref,
            predicate_relation: self.predicate_relation,
            edge_kind: self.edge_kind,
            candidate_ceiling,
        }
        .serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
enum ClaimInlinePredicateEdgeLaneOutcomeWireV0<'a> {
    SupportEligible {
        receipt: &'a ClaimInlinePredicateEdgeLaneReceiptV0,
    },
    RefutationEligible {
        receipt: &'a ClaimInlinePredicateEdgeLaneReceiptV0,
    },
    Ineligible(ClaimInlinePredicateEdgeLaneIneligibleDetailsWireV0<'a>),
    ResolutionFailed(ClaimInlinePredicateEdgeLaneFailureDetailsWireV0<'a>),
}

#[derive(Serialize)]
struct ClaimInlinePredicateEdgeLaneIneligibleDetailsWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    edge_id: &'a str,
    reason: ClaimInlinePredicateEdgeLaneIneligibleReasonV0,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ClaimInlinePredicateEdgeLaneFailureDetailsWireV0<'a> {
    Ordinary(ClaimInlinePredicateEdgeLaneOrdinaryFailureDetailsWireV0<'a>),
    Binding(ClaimInlinePredicateEdgeLaneBindingFailureDetailsWireV0<'a>),
    Claim(ClaimInlinePredicateEdgeLaneClaimFailureDetailsWireV0<'a>),
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0 {
    MissingEdge,
    UnsupportedEdgeKind,
    EdgeSourceBindingMismatch,
    EdgeTargetBindingMismatch,
    EdgeScopeBindingMismatch,
    SupportCeilingInvariantMismatch,
    RefutationCeilingInvariantMismatch,
}

#[derive(Serialize)]
struct ClaimInlinePredicateEdgeLaneOrdinaryFailureDetailsWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    edge_id: &'a str,
    reason: ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ClaimInlinePredicateEdgeLaneBindingFailureReasonWireV0 {
    AttestationBindingFailed,
}

#[derive(Serialize)]
struct ClaimInlinePredicateEdgeLaneBindingFailureDetailsWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    edge_id: &'a str,
    reason: ClaimInlinePredicateEdgeLaneBindingFailureReasonWireV0,
    binding_reason: &'a InlinePredicateAttestationBindingFailureV0,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum ClaimPredicateResolutionFailedWireV0 {
    ClaimPredicateResolutionFailed,
}

#[derive(Serialize)]
struct ClaimInlinePredicateEdgeLaneClaimFailureDetailsWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    edge_id: &'a str,
    reason: ClaimInlinePredicateEdgeLaneBindingFailureReasonWireV0,
    binding_reason: ClaimPredicateResolutionFailedWireV0,
    claim_reason: super::ClaimInlineSubjectResolutionFailureV0,
}

impl Serialize for ClaimInlinePredicateEdgeLaneOutcomeV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire = match &self.data {
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::SupportEligible(receipt) => {
                ClaimInlinePredicateEdgeLaneOutcomeWireV0::SupportEligible { receipt }
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::RefutationEligible(receipt) => {
                ClaimInlinePredicateEdgeLaneOutcomeWireV0::RefutationEligible { receipt }
            }
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::Ineligible {
                requested_claim_id,
                requested_evidence_id,
                requested_edge_id,
                reason,
            } => ClaimInlinePredicateEdgeLaneOutcomeWireV0::Ineligible(
                ClaimInlinePredicateEdgeLaneIneligibleDetailsWireV0 {
                    claim_id: requested_claim_id,
                    evidence_id: requested_evidence_id,
                    edge_id: requested_edge_id,
                    reason: *reason,
                },
            ),
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                requested_edge_id,
                failure:
                    ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                            claim_reason,
                        ),
                    ),
            } => ClaimInlinePredicateEdgeLaneOutcomeWireV0::ResolutionFailed(
                ClaimInlinePredicateEdgeLaneFailureDetailsWireV0::Claim(
                    ClaimInlinePredicateEdgeLaneClaimFailureDetailsWireV0 {
                        claim_id: requested_claim_id,
                        evidence_id: requested_evidence_id,
                        edge_id: requested_edge_id,
                        reason:
                            ClaimInlinePredicateEdgeLaneBindingFailureReasonWireV0::AttestationBindingFailed,
                        binding_reason:
                            ClaimPredicateResolutionFailedWireV0::ClaimPredicateResolutionFailed,
                        claim_reason: *claim_reason,
                    },
                ),
            ),
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                requested_edge_id,
                failure:
                    ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                        binding_reason,
                    ),
            } => ClaimInlinePredicateEdgeLaneOutcomeWireV0::ResolutionFailed(
                ClaimInlinePredicateEdgeLaneFailureDetailsWireV0::Binding(
                    ClaimInlinePredicateEdgeLaneBindingFailureDetailsWireV0 {
                        claim_id: requested_claim_id,
                        evidence_id: requested_evidence_id,
                        edge_id: requested_edge_id,
                        reason:
                            ClaimInlinePredicateEdgeLaneBindingFailureReasonWireV0::AttestationBindingFailed,
                        binding_reason,
                    },
                ),
            ),
            ClaimInlinePredicateEdgeLaneOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                requested_edge_id,
                failure,
            } => {
                let reason = match failure {
                    ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::MissingEdge
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::UnsupportedEdgeKind => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::UnsupportedEdgeKind
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::EdgeSourceBindingMismatch => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::EdgeSourceBindingMismatch
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::EdgeTargetBindingMismatch => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::EdgeTargetBindingMismatch
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::EdgeScopeBindingMismatch => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::EdgeScopeBindingMismatch
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::SupportCeilingInvariantMismatch => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::SupportCeilingInvariantMismatch
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::RefutationCeilingInvariantMismatch => {
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureReasonWireV0::RefutationCeilingInvariantMismatch
                    }
                    ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(_) => {
                        unreachable!("binding failures are serialized by the preceding arms")
                    }
                };
                ClaimInlinePredicateEdgeLaneOutcomeWireV0::ResolutionFailed(
                    ClaimInlinePredicateEdgeLaneFailureDetailsWireV0::Ordinary(
                        ClaimInlinePredicateEdgeLaneOrdinaryFailureDetailsWireV0 {
                            claim_id: requested_claim_id,
                            evidence_id: requested_evidence_id,
                            edge_id: requested_edge_id,
                            reason,
                        },
                    ),
                )
            }
        };
        wire.serialize(serializer)
    }
}

enum MatrixClassificationV0 {
    Eligible {
        lane: ClaimInlinePredicateEdgeLaneKindV0,
        predicate_relation: ClaimInlinePredicateEdgeLaneRelationV0,
        edge_kind: ClaimInlinePredicateEdgeLaneEdgeKindV0,
        candidate_ceiling: Status,
    },
    Ineligible(ClaimInlinePredicateEdgeLaneIneligibleReasonV0),
}

fn classify_matrix_with_ceilings_v0<SupportCell, RefutationCell>(
    predicate_relation: ClaimInlinePredicateEdgeLaneRelationV0,
    edge_kind: ClaimInlinePredicateEdgeLaneEdgeKindV0,
    support_cell: SupportCell,
    refutation_cell: RefutationCell,
) -> Result<MatrixClassificationV0, ClaimInlinePredicateEdgeLaneFailureV0>
where
    SupportCell: FnOnce() -> Option<Status>,
    RefutationCell: FnOnce() -> Option<Status>,
{
    match (predicate_relation, edge_kind) {
        (
            ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
        ) => {
            if support_cell() != Some(Status::Settled) {
                return Err(ClaimInlinePredicateEdgeLaneFailureV0::SupportCeilingInvariantMismatch);
            }
            Ok(MatrixClassificationV0::Eligible {
                lane: ClaimInlinePredicateEdgeLaneKindV0::Support,
                predicate_relation,
                edge_kind,
                candidate_ceiling: Status::Settled,
            })
        }
        (
            ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
        ) => Ok(MatrixClassificationV0::Ineligible(
            ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestUnequalDoesNotSupport,
        )),
        (
            ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
        ) => Ok(MatrixClassificationV0::Ineligible(
            ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute,
        )),
        (
            ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
        ) => {
            if refutation_cell() != Some(Status::Refuted) {
                return Err(
                    ClaimInlinePredicateEdgeLaneFailureV0::RefutationCeilingInvariantMismatch,
                );
            }
            Ok(MatrixClassificationV0::Eligible {
                lane: ClaimInlinePredicateEdgeLaneKindV0::Refutation,
                predicate_relation,
                edge_kind,
                candidate_ceiling: Status::Refuted,
            })
        }
    }
}

fn eligible_receipt(
    binding: &ResolvedInlinePredicateAttestationBindingV0,
    edge_id: &str,
    lane: ClaimInlinePredicateEdgeLaneKindV0,
    predicate_relation: ClaimInlinePredicateEdgeLaneRelationV0,
    edge_kind: ClaimInlinePredicateEdgeLaneEdgeKindV0,
    candidate_ceiling: Status,
) -> ClaimInlinePredicateEdgeLaneReceiptV0 {
    ClaimInlinePredicateEdgeLaneReceiptV0 {
        schema: EDGE_LANE_SCHEMA_V0.to_owned(),
        lane,
        predicate_id: binding.predicate_id().to_owned(),
        claim_id: binding.claim_id().to_owned(),
        evidence_id: binding.evidence_id().to_owned(),
        edge_id: edge_id.to_owned(),
        scope_ref: binding.scope_ref().to_owned(),
        predicate_relation,
        edge_kind,
        candidate_ceiling,
    }
}

impl StandingReplaySnapshot {
    /// Resolve one exact claim-inline predicate edge lane from this replay
    /// snapshot without changing achieved standing.
    pub fn resolve_claim_inline_predicate_edge_lane_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
    ) -> ClaimInlinePredicateEdgeLaneOutcomeV0 {
        let resolved_claim = match resolve_claim_inline_subject_v0(self, claim_id) {
            Ok(resolved_claim) => resolved_claim,
            Err(failure) => {
                return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                    claim_id,
                    evidence_id,
                    edge_id,
                    ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                            failure,
                        ),
                    ),
                );
            }
        };

        let binding = match resolve_inline_predicate_attestation_after_claim_v0(
            self,
            &resolved_claim,
            evidence_id,
        ) {
            Ok(binding) => binding,
            Err(failure) => {
                return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                    claim_id,
                    evidence_id,
                    edge_id,
                    ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(failure),
                );
            }
        };

        let edge = match self.standing().justification_edge(edge_id) {
            Some(edge) => edge,
            None => {
                return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                    claim_id,
                    evidence_id,
                    edge_id,
                    ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
                );
            }
        };
        let edge_kind = match edge.edge_kind.as_str() {
            "supports" => ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
            "contradicts" => ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
            _ => {
                return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                    claim_id,
                    evidence_id,
                    edge_id,
                    ClaimInlinePredicateEdgeLaneFailureV0::UnsupportedEdgeKind,
                );
            }
        };
        if edge.source_id.as_bytes() != evidence_id.as_bytes() {
            return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                claim_id,
                evidence_id,
                edge_id,
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeSourceBindingMismatch,
            );
        }
        if edge.target_id.as_bytes() != claim_id.as_bytes() {
            return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                claim_id,
                evidence_id,
                edge_id,
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeTargetBindingMismatch,
            );
        }
        if edge.scope_ref.as_bytes() != binding.scope_ref().as_bytes() {
            return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                claim_id,
                evidence_id,
                edge_id,
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeScopeBindingMismatch,
            );
        }

        let digest_evaluation = evaluate_resolved_claim_inline_sha256_v0(&resolved_claim);
        let predicate_relation = match digest_evaluation.relation {
            ResolvedClaimInlineSha256RelationV0::DigestEqual => {
                ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual
            }
            ResolvedClaimInlineSha256RelationV0::DigestUnequal => {
                ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal
            }
        };
        let classification = match classify_matrix_with_ceilings_v0(
            predicate_relation,
            edge_kind,
            || {
                support_ceiling(
                    EvidenceKind::DeterministicVerification,
                    ClaimDomain::ExactMachineCheckable,
                )
            },
            || {
                refutation_ceiling(
                    EvidenceKind::DeterministicVerification,
                    ClaimDomain::ExactMachineCheckable,
                )
            },
        ) {
            Ok(classification) => classification,
            Err(failure) => {
                return ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                    claim_id,
                    evidence_id,
                    edge_id,
                    failure,
                );
            }
        };

        match classification {
            MatrixClassificationV0::Eligible {
                lane,
                predicate_relation,
                edge_kind,
                candidate_ceiling,
            } => ClaimInlinePredicateEdgeLaneOutcomeV0::eligible(eligible_receipt(
                &binding,
                edge_id,
                lane,
                predicate_relation,
                edge_kind,
                candidate_ceiling,
            )),
            MatrixClassificationV0::Ineligible(reason) => {
                ClaimInlinePredicateEdgeLaneOutcomeV0::ineligible(
                    claim_id,
                    evidence_id,
                    edge_id,
                    reason,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::ClaimInlineSubjectResolutionFailureV0;

    fn receipt(lane: ClaimInlinePredicateEdgeLaneKindV0) -> ClaimInlinePredicateEdgeLaneReceiptV0 {
        let (edge_id, predicate_relation, edge_kind, candidate_ceiling) = match lane {
            ClaimInlinePredicateEdgeLaneKindV0::Support => (
                "edge-support",
                ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
                Status::Settled,
            ),
            ClaimInlinePredicateEdgeLaneKindV0::Refutation => (
                "edge-refute",
                ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
                Status::Refuted,
            ),
        };
        ClaimInlinePredicateEdgeLaneReceiptV0 {
            schema: EDGE_LANE_SCHEMA_V0.to_owned(),
            lane,
            predicate_id: "sha256_claim_inline_bytes_equals_v0".to_owned(),
            claim_id: "claim-c".to_owned(),
            evidence_id: "evidence-e".to_owned(),
            edge_id: edge_id.to_owned(),
            scope_ref: "scope-s".to_owned(),
            predicate_relation,
            edge_kind,
            candidate_ceiling,
        }
    }

    #[test]
    fn private_matrix_seam_checks_only_the_corresponding_eligible_cell() {
        let support_calls = Cell::new(0);
        let refutation_calls = Cell::new(0);
        let support = classify_matrix_with_ceilings_v0(
            ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
            || {
                support_calls.set(support_calls.get() + 1);
                Some(Status::Settled)
            },
            || {
                refutation_calls.set(refutation_calls.get() + 1);
                Some(Status::Refuted)
            },
        )
        .unwrap();
        assert!(matches!(
            support,
            MatrixClassificationV0::Eligible {
                lane: ClaimInlinePredicateEdgeLaneKindV0::Support,
                candidate_ceiling: Status::Settled,
                ..
            }
        ));
        assert_eq!(support_calls.get(), 1);
        assert_eq!(refutation_calls.get(), 0);

        let support_calls = Cell::new(0);
        let refutation_calls = Cell::new(0);
        let refutation = classify_matrix_with_ceilings_v0(
            ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
            ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
            || {
                support_calls.set(support_calls.get() + 1);
                Some(Status::Settled)
            },
            || {
                refutation_calls.set(refutation_calls.get() + 1);
                Some(Status::Refuted)
            },
        )
        .unwrap();
        assert!(matches!(
            refutation,
            MatrixClassificationV0::Eligible {
                lane: ClaimInlinePredicateEdgeLaneKindV0::Refutation,
                candidate_ceiling: Status::Refuted,
                ..
            }
        ));
        assert_eq!(support_calls.get(), 0);
        assert_eq!(refutation_calls.get(), 1);

        for (relation, edge_kind, expected) in [
            (
                ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
                ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestUnequalDoesNotSupport,
            ),
            (
                ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
                ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute,
            ),
        ] {
            let classification = classify_matrix_with_ceilings_v0(
                relation,
                edge_kind,
                || panic!("an ineligible cell must not consult the support ceiling"),
                || panic!("an ineligible cell must not consult the refutation ceiling"),
            )
            .unwrap();
            assert!(matches!(
                classification,
                MatrixClassificationV0::Ineligible(reason) if reason == expected
            ));
        }
    }

    #[test]
    fn private_matrix_seam_reports_both_exact_ceiling_drift_failures() {
        assert_eq!(
            classify_matrix_with_ceilings_v0(
                ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports,
                || Some(Status::Supported),
                || panic!("wrong ceiling consulted"),
            )
            .err(),
            Some(ClaimInlinePredicateEdgeLaneFailureV0::SupportCeilingInvariantMismatch)
        );
        assert_eq!(
            classify_matrix_with_ceilings_v0(
                ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal,
                ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts,
                || panic!("wrong ceiling consulted"),
                || Some(Status::Settled),
            )
            .err(),
            Some(ClaimInlinePredicateEdgeLaneFailureV0::RefutationCeilingInvariantMismatch)
        );
    }

    #[test]
    fn every_top_level_reason_has_exact_ordered_canonical_bytes() {
        let ordinary = [
            (
                ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
                "missing_edge",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::UnsupportedEdgeKind,
                "unsupported_edge_kind",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeSourceBindingMismatch,
                "edge_source_binding_mismatch",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeTargetBindingMismatch,
                "edge_target_binding_mismatch",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::EdgeScopeBindingMismatch,
                "edge_scope_binding_mismatch",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::SupportCeilingInvariantMismatch,
                "support_ceiling_invariant_mismatch",
            ),
            (
                ClaimInlinePredicateEdgeLaneFailureV0::RefutationCeilingInvariantMismatch,
                "refutation_ceiling_invariant_mismatch",
            ),
        ];
        for (failure, spelling) in ordinary {
            let outcome = ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                "claim", "evidence", "edge", failure,
            );
            assert_eq!(outcome.failure(), Some(&failure));
            assert_eq!(
                String::from_utf8(outcome.canonical_bytes()).unwrap(),
                format!(
                    r#"{{"outcome":"resolution_failed","details":{{"claim_id":"claim","evidence_id":"evidence","edge_id":"edge","reason":"{spelling}"}}}}"#
                )
            );
        }
    }

    #[test]
    fn all_twenty_eight_binding_reasons_retain_exact_nested_spelling() {
        use InlinePredicateAttestationBindingFailureV0::*;

        let ordinary = [
            (MissingEvidence, "missing_evidence"),
            (WrongEvidenceKind, "wrong_evidence_kind"),
            (
                MalformedAttestationMetadata,
                "malformed_attestation_metadata",
            ),
            (
                DuplicateAttestationMetadataKey,
                "duplicate_attestation_metadata_key",
            ),
            (
                UnknownAttestationMetadataKey,
                "unknown_attestation_metadata_key",
            ),
            (
                MissingInlinePredicateAttestation,
                "missing_inline_predicate_attestation",
            ),
            (
                WrongTypeInlinePredicateAttestation,
                "wrong_type_inline_predicate_attestation",
            ),
            (DuplicateAttestationField, "duplicate_attestation_field"),
            (UnknownAttestationField, "unknown_attestation_field"),
            (MissingSchema, "missing_schema"),
            (WrongTypeSchema, "wrong_type_schema"),
            (UnknownSchema, "unknown_schema"),
            (MissingPredicateId, "missing_predicate_id"),
            (WrongTypePredicateId, "wrong_type_predicate_id"),
            (EmptyPredicateId, "empty_predicate_id"),
            (PredicateIdTooLong, "predicate_id_too_long"),
            (InvalidPredicateId, "invalid_predicate_id"),
            (MissingSubjectClaimId, "missing_subject_claim_id"),
            (WrongTypeSubjectClaimId, "wrong_type_subject_claim_id"),
            (EmptySubjectClaimId, "empty_subject_claim_id"),
            (MissingScopeRef, "missing_scope_ref"),
            (WrongTypeScopeRef, "wrong_type_scope_ref"),
            (EmptyScopeRef, "empty_scope_ref"),
            (PredicateBindingMismatch, "predicate_binding_mismatch"),
            (ClaimBindingMismatch, "claim_binding_mismatch"),
            (
                EvidenceScopeBindingMismatch,
                "evidence_scope_binding_mismatch",
            ),
            (ClaimScopeBindingMismatch, "claim_scope_binding_mismatch"),
        ];
        assert_eq!(ordinary.len() + 1, 28);
        for (binding, spelling) in ordinary {
            let failure = ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(binding);
            let outcome = ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                "claim", "evidence", "edge", failure,
            );
            assert_eq!(outcome.failure(), Some(&failure));
            assert_eq!(
                String::from_utf8(outcome.canonical_bytes()).unwrap(),
                format!(
                    r#"{{"outcome":"resolution_failed","details":{{"claim_id":"claim","evidence_id":"evidence","edge_id":"edge","reason":"attestation_binding_failed","binding_reason":"{spelling}"}}}}"#
                )
            );
        }
    }

    #[test]
    fn all_thirty_three_claim_reasons_retain_exact_nested_spelling() {
        use ClaimInlineSubjectResolutionFailureV0::*;

        let reasons = [
            (MissingClaim, "missing_claim"),
            (MissingTypedClaim, "missing_typed_claim"),
            (MalformedMetadata, "malformed_metadata"),
            (MissingClaimDomain, "missing_claim_domain"),
            (WrongTypeClaimDomain, "wrong_type_claim_domain"),
            (DuplicateClaimMetadataKey, "duplicate_claim_metadata_key"),
            (UnknownClaimDomain, "unknown_claim_domain"),
            (ClaimDomainMismatch, "claim_domain_mismatch"),
            (UnknownClaimMetadataKey, "unknown_claim_metadata_key"),
            (
                MissingInlineSubjectDescriptor,
                "missing_inline_subject_descriptor",
            ),
            (
                WrongTypeInlineSubjectDescriptor,
                "wrong_type_inline_subject_descriptor",
            ),
            (DuplicateDescriptorKey, "duplicate_descriptor_key"),
            (UnknownDescriptorKey, "unknown_descriptor_key"),
            (MissingSchema, "missing_schema"),
            (WrongTypeSchema, "wrong_type_schema"),
            (UnknownSchema, "unknown_schema"),
            (MissingPredicateId, "missing_predicate_id"),
            (WrongTypePredicateId, "wrong_type_predicate_id"),
            (EmptyPredicateId, "empty_predicate_id"),
            (PredicateIdTooLong, "predicate_id_too_long"),
            (InvalidPredicateId, "invalid_predicate_id"),
            (UnknownPredicateId, "unknown_predicate_id"),
            (MissingExpectedSha256, "missing_expected_sha256"),
            (WrongTypeExpectedSha256, "wrong_type_expected_sha256"),
            (InvalidExpectedSha256, "invalid_expected_sha256"),
            (MissingSubjectHex, "missing_subject_hex"),
            (WrongTypeSubjectHex, "wrong_type_subject_hex"),
            (SubjectHexTooLong, "subject_hex_too_long"),
            (InvalidSubjectHex, "invalid_subject_hex"),
            (StatementBindingMismatch, "statement_binding_mismatch"),
            (MissingClaimContentHash, "missing_claim_content_hash"),
            (ClaimContentHashMismatch, "claim_content_hash_mismatch"),
            (DecodedSubjectTooLarge, "decoded_subject_too_large"),
        ];
        assert_eq!(reasons.len(), 33);
        for (claim_reason, spelling) in reasons {
            let failure = ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                    claim_reason,
                ),
            );
            let outcome = ClaimInlinePredicateEdgeLaneOutcomeV0::resolution_failed(
                "claim", "evidence", "edge", failure,
            );
            assert_eq!(outcome.failure(), Some(&failure));
            assert_eq!(
                String::from_utf8(outcome.canonical_bytes()).unwrap(),
                format!(
                    r#"{{"outcome":"resolution_failed","details":{{"claim_id":"claim","evidence_id":"evidence","edge_id":"edge","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"{spelling}"}}}}"#
                )
            );
        }
    }

    #[test]
    fn eligible_receipt_serialization_uses_exact_lowercase_ceiling_and_order() {
        let support = ClaimInlinePredicateEdgeLaneOutcomeV0::eligible(receipt(
            ClaimInlinePredicateEdgeLaneKindV0::Support,
        ));
        assert_eq!(
            String::from_utf8(support.canonical_bytes()).unwrap(),
            r#"{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}"#
        );
        let refutation = ClaimInlinePredicateEdgeLaneOutcomeV0::eligible(receipt(
            ClaimInlinePredicateEdgeLaneKindV0::Refutation,
        ));
        assert_eq!(
            String::from_utf8(refutation.canonical_bytes()).unwrap(),
            r#"{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}"#
        );
    }
}
