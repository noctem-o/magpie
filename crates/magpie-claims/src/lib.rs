//! # magpie-claims — the epistemic claim store, as a projection
//!
//! [`ClaimsView`] folds log events into per-claim state: a statement, its
//! current position on the epistemic spectrum, accumulated evidence, and a
//! transition count. It is *derived*: drop it and rebuild it from the log at
//! any time. It reads through a `LogReader` and cannot write. That asymmetry
//! is enforced by the type system.

pub mod policy;

mod admitted_contribution_audit;
mod artifact_provenance_verifier;
mod deadbolt_context;
mod deterministic_verifier_context;
mod origin_admission_audit;
mod origin_admission_replay;
mod origin_binding_verifier;
mod replay_snapshot;
mod resolution_content_closure;
mod standing;
mod standing_v1;
mod standing_v2;

use std::collections::BTreeMap;

use magpie_log::{Payload, Projection, SignedEvent, Status};
use serde::Serialize;

pub use admitted_contribution_audit::{
    AdmittedContributionAuditCompletionV0, AdmittedContributionAuditV0,
    AdmittedContributionCandidateAuditV0, AdmittedContributionCandidateDispositionV0,
    AdmittedContributionCandidateReasonV0, AdmittedContributionV0,
    ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0,
    ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0, ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0,
    ADMITTED_CONTRIBUTION_POLICY_ID_V0,
};

pub use artifact_provenance_verifier::{
    ArtifactAcquisitionReceiptV0, ArtifactDerivationReceiptV0, ArtifactProvenanceAnchorSelectorV0,
    ArtifactProvenanceContextTraceV0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_ACQUISITION_SCHEMA_V0, ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
    ARTIFACT_DERIVATION_SCHEMA_V0, ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0,
    ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0, MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0,
};

pub use deadbolt_context::{
    resolve_deadbolt_occurrence_context, DeadboltAnchorIdentity, DeadboltAnchorIndex,
    DeadboltAnchorOccurrence, DeadboltOccurrenceContextResolution, DEADBOLT_OCCURRENCE_SCHEMA,
};

pub use deterministic_verifier_context::{
    DeterministicVerifierContextTraceV0, DeterministicVerifierReceiptV0,
    MACHINE_PREDICATE_SCHEMA_V0, MAX_WITNESS_BYTES_V0, SHA256_BYTES_EQUALS_PREDICATE_V0,
    SHA256_BYTES_EQUALS_STATEMENT_PREFIX_V0, VERIFICATION_WITNESS_SCHEMA_V0,
};

pub use replay_snapshot::{replay_standing_context, StandingReplaySnapshot};

pub use origin_admission_audit::{
    AdmittedOriginV0, OriginAdmissionAuditCompletionV0, OriginAdmissionAuditV0,
    OriginAdmissionCandidateAuditV0, OriginAdmissionCandidateDispositionV0,
    OriginAdmissionDecisionV0, OriginBindingConflictV0,
    ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0, ORIGIN_ADMISSION_AUDIT_SCHEMA_V0,
    ORIGIN_ADMISSION_POLICY_ID_V0, ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0,
    ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0,
};

pub use origin_admission_replay::{
    replay_origin_admission_context_v0, OriginAdmissionReplayContextV0, VerifiedLogPrefixIdentityV0,
};

pub use origin_binding_verifier::{
    ContributionIdentityV0, OriginBindingContextTraceV0, OriginBindingFamilyV0,
    OriginBindingReceiptV0, OriginComparisonNamespaceV0, MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0,
    ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0, ORIGIN_BINDING_ACQUISITION_SCHEMA_V0,
    ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0, ORIGIN_BINDING_DERIVATION_SCHEMA_V0,
    ORIGIN_BINDING_VERIFIER_PROFILE_V0, ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};

pub use resolution_content_closure::{
    ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0,
    ResolutionContentClosureConstructionErrorV0, ResolutionContentClosureIdentityV0,
    ResolutionContentClosureKeyFieldV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0,
    RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0,
    RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0, RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0,
};

pub use standing::{
    StandingClaim, StandingCurrentness, StandingResolution, StandingTraceEntry,
    StandingTraceReason, StandingView, TypedClaimNode, TypedEvidenceNode, TypedJustificationEdge,
    MAGPIE_CLAIMS_POLICY_ID,
};

pub use standing_v1::{
    DeadboltOccurrenceContextTrace, StandingPolicyApplication, StandingPolicyRule,
    StandingResolutionV1, StandingTraceEntryV1, MAGPIE_CLAIMS_POLICY_V1_ID,
};

pub use standing_v2::{
    StandingPolicyApplicationV2, StandingPolicyContextV2, StandingPolicyRuleV2,
    StandingResolutionFailureV2, StandingResolutionV2, StandingTraceEntryV2,
    MAGPIE_CLAIMS_POLICY_V2_ID,
};

/// A single tracked claim and its current epistemic state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Claim {
    pub statement: String,
    pub status: Status,
    pub evidence: Vec<String>,
    pub transitions: u64,
}

/// The derived claim store. `BTreeMap` (not `HashMap`) so serialization is
/// deterministic — which is what lets us assert byte-identical regeneration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ClaimsView {
    pub claims: BTreeMap<String, Claim>,
}

impl ClaimsView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, claim_id: &str) -> Option<&Claim> {
        self.claims.get(claim_id)
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    /// Deterministic bytes for the "is this regenerable?" check.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("ClaimsView is always serializable")
    }
}

impl Projection for ClaimsView {
    fn apply(&mut self, event: &SignedEvent) {
        match &event.core.payload {
            Payload::ClaimAsserted {
                claim_id,
                statement,
                status,
            } => {
                self.claims
                    .entry(claim_id.clone())
                    .or_insert_with(|| Claim {
                        statement: statement.clone(),
                        status: *status,
                        evidence: Vec::new(),
                        transitions: 0,
                    });
            }
            Payload::EvidenceRecorded { claim_id, summary } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.evidence.push(summary.clone());
                }
            }
            Payload::ClaimStatusChanged { claim_id, to, .. } => {
                if let Some(claim) = self.claims.get_mut(claim_id) {
                    claim.status = *to;
                    claim.transitions += 1;
                }
            }
            Payload::Note { .. } => {}
            Payload::Genesis { .. } => {}
            // Anchors commit execution evidence, not epistemic claims; the
            // claim store deliberately ignores them. (The episodic store is
            // where they surface as timeline rows.)
            Payload::SegmentAnchored { .. } => {}
            // ADR-0002 typed events are interpreted by StandingView, not the
            // legacy ClaimsView. Keep this compatibility projection explicit.
            Payload::ClaimAssertedV2 { .. } => {}
            Payload::EvidenceRegistered { .. } => {}
            Payload::JustificationEdgeRecorded { .. } => {}
        }
    }
}
