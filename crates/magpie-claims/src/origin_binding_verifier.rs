//! Standing-inert origin-binding verification from one verified replay and closure.
//!
//! Origin-binding receipts cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::OriginBindingReceiptV0;
//! let _receipt = OriginBindingReceiptV0 { verifier_profile: String::new() };
//! ```
//!
//! Origin-binding receipts cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginBindingReceiptV0;
//! let _: OriginBindingReceiptV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Contribution identities cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::ContributionIdentityV0;
//! let _: ContributionIdentityV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Comparison namespaces cannot be deserialized:
//!
//! ```compile_fail
//! use magpie_claims::OriginComparisonNamespaceV0;
//! let _: OriginComparisonNamespaceV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! A standalone `StandingView` has no origin-binding resolver:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ArtifactProvenanceAnchorSelectorV0, ResolutionContentClosureV0, StandingView,
//! };
//! let view = StandingView::new();
//! let selector = ArtifactProvenanceAnchorSelectorV0::new("", "", "", "", "");
//! let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
//! let _ = view.resolve_origin_binding_context_v0(&selector, &closure);
//! ```
//!
//! No free public resolver accepts a caller-created anchor index:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     resolve_origin_binding_context_v0, ArtifactProvenanceAnchorSelectorV0,
//!     DeadboltAnchorIndex, ResolutionContentClosureV0,
//! };
//! let anchors = DeadboltAnchorIndex::new();
//! let selector = ArtifactProvenanceAnchorSelectorV0::new("", "", "", "", "");
//! let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
//! let _ = resolve_origin_binding_context_v0(&anchors, &selector, &closure);
//! ```
//!
//! Serialized or cloned receipt material is not accepted as resolver authority:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ArtifactProvenanceAnchorSelectorV0, OriginBindingReceiptV0,
//!     ResolutionContentClosureV0, StandingReplaySnapshot,
//! };
//! fn substitute(
//!     snapshot: &StandingReplaySnapshot,
//!     selector: &ArtifactProvenanceAnchorSelectorV0,
//!     closure: &ResolutionContentClosureV0,
//!     receipt: OriginBindingReceiptV0,
//! ) {
//!     let _ = snapshot.resolve_origin_binding_context_v0(selector, closure, receipt);
//! }
//! ```

use std::collections::BTreeSet;
use std::fmt;

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use sha2::{Digest, Sha256};

use crate::artifact_provenance_verifier::{
    resolve_artifact_acquisition_from_closure_v0, resolve_artifact_derivation_from_closure_v0,
    ArtifactAcquisitionReceiptV0, ArtifactDerivationReceiptV0, ArtifactProvenanceAnchorSelectorV0,
    ArtifactProvenanceContextTraceV0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_DERIVATION_BUNDLE_KIND_V0, ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
    ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
};
use crate::deadbolt_context::{DeadboltAnchorIdentity, DeadboltAnchorOccurrence};
use crate::origin_admission_audit::ORIGIN_ADMISSION_POLICY_ID_V0;
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::resolution_content_closure::ResolutionContentClosureV0;

pub const ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0: &str = "magpie-origin-binding-acquisition-v0";

pub const ORIGIN_BINDING_ACQUISITION_SCHEMA_V0: &str = "magpie-origin-binding-acquisition-v0";

pub const ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0: &str = "magpie-origin-binding-derivation-v0";

pub const ORIGIN_BINDING_DERIVATION_SCHEMA_V0: &str = "magpie-origin-binding-derivation-v0";

pub const ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0: &str = "magpie-origin-binding-json-v0";

pub const ORIGIN_BINDING_VERIFIER_PROFILE_V0: &str = "magpie-origin-binding-verifier-v0";

pub const ORIGIN_BINDING_WITNESS_ALGORITHM_V0: &str = "sha256";

pub const ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0: &str = "sha256";

pub const MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0: usize = 16_384;

/// Exact subject of one verified origin-binding governance assertion.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ContributionIdentityV0 {
    target_claim_id: String,
    source_evidence_id: String,
    justification_edge_id: String,
    scope_ref: String,
    artifact_algorithm: String,
    artifact_digest: String,
}

impl ContributionIdentityV0 {
    pub(crate) fn from_admitted_contribution_graph_v0(
        target_claim_id: &str,
        source_evidence_id: &str,
        justification_edge_id: &str,
        scope_ref: &str,
        artifact_digest: &str,
    ) -> Self {
        Self {
            target_claim_id: target_claim_id.to_owned(),
            source_evidence_id: source_evidence_id.to_owned(),
            justification_edge_id: justification_edge_id.to_owned(),
            scope_ref: scope_ref.to_owned(),
            artifact_algorithm: ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0.to_owned(),
            artifact_digest: artifact_digest.to_owned(),
        }
    }

    pub fn target_claim_id(&self) -> &str {
        &self.target_claim_id
    }

    pub fn source_evidence_id(&self) -> &str {
        &self.source_evidence_id
    }

    pub fn justification_edge_id(&self) -> &str {
        &self.justification_edge_id
    }

    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }

    pub fn artifact_algorithm(&self) -> &str {
        &self.artifact_algorithm
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

/// Derived namespace in which opaque origin-group keys may later be compared.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OriginComparisonNamespaceV0 {
    origin_admission_policy_id: String,
    target_claim_id: String,
    scope_ref: String,
}

impl OriginComparisonNamespaceV0 {
    pub(crate) fn for_admitted_contribution_v0(target_claim_id: &str, scope_ref: &str) -> Self {
        Self {
            origin_admission_policy_id: ORIGIN_ADMISSION_POLICY_ID_V0.to_owned(),
            target_claim_id: target_claim_id.to_owned(),
            scope_ref: scope_ref.to_owned(),
        }
    }

    pub fn origin_admission_policy_id(&self) -> &str {
        &self.origin_admission_policy_id
    }

    pub fn target_claim_id(&self) -> &str {
        &self.target_claim_id
    }

    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }
}

/// Test-only constructor for hostile policy-v3 namespace inputs.
///
/// `#[cfg(test)]` keeps this outside production builds and public API. Honest
/// production namespaces remain constructible only through origin admission.
#[cfg(test)]
pub(crate) fn stage_origin_comparison_namespace_v0_for_tests(
    origin_admission_policy_id: &str,
    target_claim_id: &str,
    scope_ref: &str,
) -> OriginComparisonNamespaceV0 {
    OriginComparisonNamespaceV0 {
        origin_admission_policy_id: origin_admission_policy_id.to_owned(),
        target_claim_id: target_claim_id.to_owned(),
        scope_ref: scope_ref.to_owned(),
    }
}

/// Closed origin-binding wire family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginBindingFamilyV0 {
    Acquisition,
    Derivation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "family", rename_all = "snake_case")]
enum OriginBindingProvenanceV0 {
    Acquisition {
        acquisition_receipt: ArtifactAcquisitionReceiptV0,
    },
    Derivation {
        derivation_selector: ArtifactProvenanceAnchorSelectorV0,
        acquisition_receipt: ArtifactAcquisitionReceiptV0,
        derivation_receipt: Box<ArtifactDerivationReceiptV0>,
    },
}

/// Private-construction audit receipt for one exact verified governance assertion.
///
/// A receipt records claimed authority content. It does not trust that authority,
/// admit an origin group, create support, aggregate evidence, or affect standing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OriginBindingReceiptV0 {
    verifier_profile: String,
    binding_selector: ArtifactProvenanceAnchorSelectorV0,
    family: OriginBindingFamilyV0,
    schema: String,
    binding_id: String,
    origin_admission_policy_id: String,
    contribution: ContributionIdentityV0,
    comparison_namespace: OriginComparisonNamespaceV0,
    acquisition_selector: ArtifactProvenanceAnchorSelectorV0,
    provenance: OriginBindingProvenanceV0,
    origin_group: String,
    authority_kind: String,
    authority_reference: String,
    binding_run_id: String,
    bundle_byte_len: u64,
    computed_witness_root: String,
    occurrences: Vec<DeadboltAnchorOccurrence>,
}

impl OriginBindingReceiptV0 {
    pub fn verifier_profile(&self) -> &str {
        &self.verifier_profile
    }

    pub fn binding_selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.binding_selector
    }

    pub fn family(&self) -> OriginBindingFamilyV0 {
        self.family
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn binding_id(&self) -> &str {
        &self.binding_id
    }

    pub fn origin_admission_policy_id(&self) -> &str {
        &self.origin_admission_policy_id
    }

    pub fn contribution(&self) -> &ContributionIdentityV0 {
        &self.contribution
    }

    pub fn comparison_namespace(&self) -> &OriginComparisonNamespaceV0 {
        &self.comparison_namespace
    }

    pub fn acquisition_selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.acquisition_selector
    }

    pub fn derivation_selector(&self) -> Option<&ArtifactProvenanceAnchorSelectorV0> {
        match &self.provenance {
            OriginBindingProvenanceV0::Acquisition { .. } => None,
            OriginBindingProvenanceV0::Derivation {
                derivation_selector,
                ..
            } => Some(derivation_selector),
        }
    }

    pub fn origin_group(&self) -> &str {
        &self.origin_group
    }

    pub fn authority_kind(&self) -> &str {
        &self.authority_kind
    }

    pub fn authority_reference(&self) -> &str {
        &self.authority_reference
    }

    pub fn binding_run_id(&self) -> &str {
        &self.binding_run_id
    }

    pub fn bundle_byte_len(&self) -> u64 {
        self.bundle_byte_len
    }

    pub fn computed_witness_root(&self) -> &str {
        &self.computed_witness_root
    }

    pub fn occurrences(&self) -> &[DeadboltAnchorOccurrence] {
        &self.occurrences
    }

    pub fn acquisition_receipt(&self) -> &ArtifactAcquisitionReceiptV0 {
        match &self.provenance {
            OriginBindingProvenanceV0::Acquisition {
                acquisition_receipt,
            }
            | OriginBindingProvenanceV0::Derivation {
                acquisition_receipt,
                ..
            } => acquisition_receipt,
        }
    }

    pub fn derivation_receipt(&self) -> Option<&ArtifactDerivationReceiptV0> {
        match &self.provenance {
            OriginBindingProvenanceV0::Acquisition { .. } => None,
            OriginBindingProvenanceV0::Derivation {
                derivation_receipt, ..
            } => Some(derivation_receipt.as_ref()),
        }
    }
}

/// Closed deterministic audit trace for origin-binding verifier v0.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum OriginBindingContextTraceV0 {
    BindingBundleUnavailable,
    BindingBundleTooLarge,
    InvalidUtf8,
    Utf8BomPresent,
    InvalidJson,
    TopLevelNotObject,
    DuplicateKey,
    MissingField,
    UnknownField,
    WrongJsonType,
    SchemaMismatch,
    InvalidProtocolIdentifier,
    InvalidReplayReferenceLength,
    UnsupportedArtifactAlgorithm,
    InvalidArtifactDigest,
    InvalidAcquisitionSelector,
    InvalidDerivationSelector,
    NonCanonicalEncoding,
    BindingBundleKindMismatch,
    BindingWitnessAlgorithmMismatch,
    BindingCanonicalizationProfileMismatch,
    BindingRunIdMismatch,
    BindingWitnessRootMismatch,
    BindingAnchorAbsent,
    TargetClaimAbsent,
    SourceEvidenceAbsent,
    JustificationEdgeAbsent,
    EdgeSourceMismatch,
    EdgeTargetMismatch,
    ScopeMismatch,
    AcquisitionPrerequisiteUnresolved {
        trace: ArtifactProvenanceContextTraceV0,
    },
    DerivationPrerequisiteUnresolved {
        trace: ArtifactProvenanceContextTraceV0,
    },
    AcquisitionArtifactDiffersFromContributionArtifact,
    AcquisitionArtifactDiffersFromDerivationParent,
    DerivationOutputDiffersFromContributionArtifact,
    MatchedAcquisition(OriginBindingReceiptV0),
    MatchedDerivation(OriginBindingReceiptV0),
}

impl OriginBindingContextTraceV0 {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("origin-binding traces are always serializable")
    }

    pub fn matched_receipt(&self) -> Option<&OriginBindingReceiptV0> {
        match self {
            Self::MatchedAcquisition(receipt) | Self::MatchedDerivation(receipt) => Some(receipt),
            _ => None,
        }
    }

    pub fn matched_acquisition(&self) -> Option<&OriginBindingReceiptV0> {
        match self {
            Self::MatchedAcquisition(receipt) => Some(receipt),
            _ => None,
        }
    }

    pub fn matched_derivation(&self) -> Option<&OriginBindingReceiptV0> {
        match self {
            Self::MatchedDerivation(receipt) => Some(receipt),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OriginBindingEvaluationClassV0 {
    BindingBytesUnavailable,
    DefinitivelyRejected,
    AvailabilityOnly,
    Matched,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ValidatedOriginBindingSubjectV0 {
    contribution: ContributionIdentityV0,
    namespace: OriginComparisonNamespaceV0,
    claimed_policy_id: String,
    claimed_origin_group: String,
    authority_kind: String,
    authority_reference: String,
}

impl ValidatedOriginBindingSubjectV0 {
    pub(crate) fn contribution(&self) -> &ContributionIdentityV0 {
        &self.contribution
    }

    pub(crate) fn namespace(&self) -> &OriginComparisonNamespaceV0 {
        &self.namespace
    }

    pub(crate) fn claimed_policy_id(&self) -> &str {
        &self.claimed_policy_id
    }

    pub(crate) fn claimed_origin_group(&self) -> &str {
        &self.claimed_origin_group
    }

    pub(crate) fn authority_kind(&self) -> &str {
        &self.authority_kind
    }

    pub(crate) fn authority_reference(&self) -> &str {
        &self.authority_reference
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OriginBindingEvaluationV0 {
    trace: OriginBindingContextTraceV0,
    class: OriginBindingEvaluationClassV0,
    validated_subject: Option<ValidatedOriginBindingSubjectV0>,
}

impl OriginBindingEvaluationV0 {
    fn binding_bytes_unavailable() -> Self {
        Self {
            trace: OriginBindingContextTraceV0::BindingBundleUnavailable,
            class: OriginBindingEvaluationClassV0::BindingBytesUnavailable,
            validated_subject: None,
        }
    }

    fn definitively_rejected(
        trace: OriginBindingContextTraceV0,
        validated_subject: Option<ValidatedOriginBindingSubjectV0>,
    ) -> Self {
        Self {
            trace,
            class: OriginBindingEvaluationClassV0::DefinitivelyRejected,
            validated_subject,
        }
    }

    fn availability_only(
        trace: OriginBindingContextTraceV0,
        validated_subject: ValidatedOriginBindingSubjectV0,
    ) -> Self {
        Self {
            trace,
            class: OriginBindingEvaluationClassV0::AvailabilityOnly,
            validated_subject: Some(validated_subject),
        }
    }

    fn matched(
        trace: OriginBindingContextTraceV0,
        validated_subject: ValidatedOriginBindingSubjectV0,
    ) -> Self {
        Self {
            trace,
            class: OriginBindingEvaluationClassV0::Matched,
            validated_subject: Some(validated_subject),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        OriginBindingContextTraceV0,
        OriginBindingEvaluationClassV0,
        Option<ValidatedOriginBindingSubjectV0>,
    ) {
        (self.trace, self.class, self.validated_subject)
    }

    fn into_trace(self) -> OriginBindingContextTraceV0 {
        self.trace
    }
}

impl StandingReplaySnapshot {
    /// Resolve one exact origin-binding assertion from this replay and closure.
    pub fn resolve_origin_binding_context_v0(
        &self,
        binding_selector: &ArtifactProvenanceAnchorSelectorV0,
        closure: &ResolutionContentClosureV0,
    ) -> OriginBindingContextTraceV0 {
        evaluate_origin_binding_context_v0(self, binding_selector, closure).into_trace()
    }
}

pub(crate) fn evaluate_origin_binding_context_v0(
    snapshot: &StandingReplaySnapshot,
    binding_selector: &ArtifactProvenanceAnchorSelectorV0,
    closure: &ResolutionContentClosureV0,
) -> OriginBindingEvaluationV0 {
    let supplied = match closure.foreign_bundle(binding_selector) {
        Some(bytes) => bytes,
        None => return OriginBindingEvaluationV0::binding_bytes_unavailable(),
    };
    if supplied.len() > MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0 {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingBundleTooLarge,
            None,
        );
    }

    let text = match decode_bundle_text(supplied) {
        Ok(value) => value,
        Err(trace) => return OriginBindingEvaluationV0::definitively_rejected(trace, None),
    };
    let envelope = match parse_binding_envelope(text) {
        Ok(value) => value,
        Err(trace) => return OriginBindingEvaluationV0::definitively_rejected(trace, None),
    };
    let parsed = match validate_binding_shape(envelope) {
        Ok(value) => value,
        Err(trace) => return OriginBindingEvaluationV0::definitively_rejected(trace, None),
    };
    if let Err(trace) = validate_binding_semantics(&parsed) {
        return OriginBindingEvaluationV0::definitively_rejected(trace, None);
    }

    let canonical = encode_binding(&parsed);
    if canonical != supplied {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::NonCanonicalEncoding,
            None,
        );
    }

    let expected_kind = match parsed.family {
        OriginBindingFamilyV0::Acquisition => ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        OriginBindingFamilyV0::Derivation => ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
    };
    if binding_selector.bundle_kind() != expected_kind {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingBundleKindMismatch,
            None,
        );
    }
    if binding_selector.witness_algorithm() != ORIGIN_BINDING_WITNESS_ALGORITHM_V0 {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingWitnessAlgorithmMismatch,
            None,
        );
    }
    if binding_selector.canonicalization_profile() != ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0 {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingCanonicalizationProfileMismatch,
            None,
        );
    }
    if binding_selector.run_id() != parsed.run_id {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingRunIdMismatch,
            None,
        );
    }

    let computed_witness_root = sha256_hex(&canonical);
    if binding_selector.witness_root() != computed_witness_root {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingWitnessRootMismatch,
            None,
        );
    }
    let identity = selector_identity(binding_selector);
    let occurrences = snapshot.anchors().occurrences(&identity);
    if occurrences.is_empty() {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::BindingAnchorAbsent,
            None,
        );
    }

    let target = match snapshot
        .standing()
        .typed_claim(&parsed.contribution.target_claim_id)
    {
        Some(value) => value,
        None => {
            return OriginBindingEvaluationV0::definitively_rejected(
                OriginBindingContextTraceV0::TargetClaimAbsent,
                None,
            )
        }
    };
    let evidence = match snapshot
        .standing()
        .typed_evidence(&parsed.contribution.source_evidence_id)
    {
        Some(value) => value,
        None => {
            return OriginBindingEvaluationV0::definitively_rejected(
                OriginBindingContextTraceV0::SourceEvidenceAbsent,
                None,
            )
        }
    };
    let edge = match snapshot
        .standing()
        .justification_edge(&parsed.contribution.justification_edge_id)
    {
        Some(value) => value,
        None => {
            return OriginBindingEvaluationV0::definitively_rejected(
                OriginBindingContextTraceV0::JustificationEdgeAbsent,
                None,
            )
        }
    };
    if edge.source_id != parsed.contribution.source_evidence_id {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::EdgeSourceMismatch,
            None,
        );
    }
    if edge.target_id != parsed.contribution.target_claim_id {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::EdgeTargetMismatch,
            None,
        );
    }
    if target.scope_ref != parsed.contribution.scope_ref
        || evidence.scope_ref != parsed.contribution.scope_ref
        || edge.scope_ref != parsed.contribution.scope_ref
    {
        return OriginBindingEvaluationV0::definitively_rejected(
            OriginBindingContextTraceV0::ScopeMismatch,
            None,
        );
    }

    let validated_subject = ValidatedOriginBindingSubjectV0 {
        contribution: ContributionIdentityV0 {
            target_claim_id: parsed.contribution.target_claim_id.clone(),
            source_evidence_id: parsed.contribution.source_evidence_id.clone(),
            justification_edge_id: parsed.contribution.justification_edge_id.clone(),
            scope_ref: parsed.contribution.scope_ref.clone(),
            artifact_algorithm: parsed.contribution.artifact.algorithm.clone(),
            artifact_digest: parsed.contribution.artifact.digest.clone(),
        },
        namespace: OriginComparisonNamespaceV0 {
            origin_admission_policy_id: parsed.origin_admission_policy_id.clone(),
            target_claim_id: parsed.contribution.target_claim_id.clone(),
            scope_ref: parsed.contribution.scope_ref.clone(),
        },
        claimed_policy_id: parsed.origin_admission_policy_id.clone(),
        claimed_origin_group: parsed.origin_group.clone(),
        authority_kind: parsed.authority.kind.clone(),
        authority_reference: parsed.authority.reference.clone(),
    };

    let acquisition_receipt = match resolve_artifact_acquisition_from_closure_v0(
        snapshot,
        &parsed.acquisition_selector,
        closure,
    ) {
        ArtifactProvenanceContextTraceV0::MatchedAcquisition(receipt) => receipt,
        trace => {
            let availability_only = artifact_provenance_trace_is_availability_only(&trace);
            let public_trace =
                OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved { trace };
            return if availability_only {
                OriginBindingEvaluationV0::availability_only(public_trace, validated_subject)
            } else {
                OriginBindingEvaluationV0::definitively_rejected(
                    public_trace,
                    Some(validated_subject),
                )
            };
        }
    };

    let provenance = match &parsed.derivation_selector {
        None => {
            if acquisition_receipt.artifact_algorithm() != parsed.contribution.artifact.algorithm
                || acquisition_receipt.artifact_digest() != parsed.contribution.artifact.digest
            {
                return OriginBindingEvaluationV0::definitively_rejected(
                    OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromContributionArtifact,
                    Some(validated_subject),
                );
            }
            OriginBindingProvenanceV0::Acquisition {
                acquisition_receipt,
            }
        }
        Some(derivation_selector) => {
            let derivation_receipt = match resolve_artifact_derivation_from_closure_v0(
                snapshot,
                derivation_selector,
                closure,
            ) {
                ArtifactProvenanceContextTraceV0::MatchedDerivation(receipt) => receipt,
                trace => {
                    let availability_only = artifact_provenance_trace_is_availability_only(&trace);
                    let public_trace =
                        OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved { trace };
                    return if availability_only {
                        OriginBindingEvaluationV0::availability_only(
                            public_trace,
                            validated_subject,
                        )
                    } else {
                        OriginBindingEvaluationV0::definitively_rejected(
                            public_trace,
                            Some(validated_subject),
                        )
                    };
                }
            };
            if acquisition_receipt.artifact_algorithm()
                != derivation_receipt.parent_artifact_algorithm()
                || acquisition_receipt.artifact_digest()
                    != derivation_receipt.parent_artifact_digest()
            {
                return OriginBindingEvaluationV0::definitively_rejected(
                    OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromDerivationParent,
                    Some(validated_subject),
                );
            }
            if derivation_receipt.derived_artifact_algorithm()
                != parsed.contribution.artifact.algorithm
                || derivation_receipt.derived_artifact_digest()
                    != parsed.contribution.artifact.digest
            {
                return OriginBindingEvaluationV0::definitively_rejected(
                    OriginBindingContextTraceV0::DerivationOutputDiffersFromContributionArtifact,
                    Some(validated_subject),
                );
            }
            OriginBindingProvenanceV0::Derivation {
                derivation_selector: derivation_selector.clone(),
                acquisition_receipt,
                derivation_receipt: Box::new(derivation_receipt),
            }
        }
    };

    let receipt = OriginBindingReceiptV0 {
        verifier_profile: ORIGIN_BINDING_VERIFIER_PROFILE_V0.to_owned(),
        binding_selector: binding_selector.clone(),
        family: parsed.family,
        schema: parsed.schema,
        binding_id: parsed.binding_id,
        origin_admission_policy_id: parsed.origin_admission_policy_id,
        contribution: validated_subject.contribution.clone(),
        comparison_namespace: validated_subject.namespace.clone(),
        acquisition_selector: parsed.acquisition_selector,
        provenance,
        origin_group: parsed.origin_group,
        authority_kind: parsed.authority.kind,
        authority_reference: parsed.authority.reference,
        binding_run_id: parsed.run_id,
        bundle_byte_len: supplied.len() as u64,
        computed_witness_root,
        occurrences: occurrences.to_vec(),
    };

    assert_eq!(receipt.contribution(), validated_subject.contribution());
    assert_eq!(
        receipt.comparison_namespace(),
        validated_subject.namespace()
    );
    assert_eq!(
        receipt.origin_admission_policy_id(),
        validated_subject.claimed_policy_id()
    );
    assert_eq!(
        receipt.origin_group(),
        validated_subject.claimed_origin_group()
    );
    assert_eq!(receipt.authority_kind(), validated_subject.authority_kind());
    assert_eq!(
        receipt.authority_reference(),
        validated_subject.authority_reference()
    );

    let trace = match receipt.family {
        OriginBindingFamilyV0::Acquisition => {
            OriginBindingContextTraceV0::MatchedAcquisition(receipt)
        }
        OriginBindingFamilyV0::Derivation => {
            OriginBindingContextTraceV0::MatchedDerivation(receipt)
        }
    };
    OriginBindingEvaluationV0::matched(trace, validated_subject)
}

fn artifact_provenance_trace_is_availability_only(
    trace: &ArtifactProvenanceContextTraceV0,
) -> bool {
    matches!(
        trace,
        ArtifactProvenanceContextTraceV0::BundleUnavailable
            | ArtifactProvenanceContextTraceV0::ArtifactUnavailable
            | ArtifactProvenanceContextTraceV0::ParentArtifactUnavailable
            | ArtifactProvenanceContextTraceV0::DerivedArtifactUnavailable
    )
}

fn selector_identity(selector: &ArtifactProvenanceAnchorSelectorV0) -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: selector.bundle_kind().to_owned(),
        witness_root: selector.witness_root().to_owned(),
        witness_algorithm: selector.witness_algorithm().to_owned(),
        canonicalization_profile: selector.canonicalization_profile().to_owned(),
        run_id: selector.run_id().to_owned(),
    }
}

#[allow(clippy::result_large_err)]
fn decode_bundle_text(input: &[u8]) -> Result<&str, OriginBindingContextTraceV0> {
    let text = std::str::from_utf8(input).map_err(|_| OriginBindingContextTraceV0::InvalidUtf8)?;
    if input.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(OriginBindingContextTraceV0::Utf8BomPresent);
    }
    Ok(text)
}

fn sha256_hex(input: &[u8]) -> String {
    hex::encode(Sha256::digest(input))
}

#[derive(Default)]
struct ObjectFlags {
    duplicate: bool,
    unknown: bool,
}

#[derive(Default)]
struct StringSlot {
    value: Option<String>,
    wrong_type: bool,
    seen: bool,
}

impl StringSlot {
    fn record(&mut self, value: LooseString, flags: &mut ObjectFlags) {
        self.seen = true;
        flags.duplicate |= value.duplicate;
        match value.value {
            Some(value) => self.value = Some(value),
            None => self.wrong_type = true,
        }
    }

    fn required(self) -> Result<String, ShapeFailure> {
        if !self.seen {
            return Err(ShapeFailure::Missing);
        }
        if self.wrong_type {
            return Err(ShapeFailure::WrongType);
        }
        self.value.ok_or(ShapeFailure::WrongType)
    }
}

enum ShapeFailure {
    Missing,
    WrongType,
}

impl ShapeFailure {
    fn trace(self) -> OriginBindingContextTraceV0 {
        match self {
            Self::Missing => OriginBindingContextTraceV0::MissingField,
            Self::WrongType => OriginBindingContextTraceV0::WrongJsonType,
        }
    }
}

#[derive(Default)]
struct DiscardedJson {
    duplicate: bool,
}

impl<'de> Deserialize<'de> for DiscardedJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DiscardVisitor;

        impl<'de> Visitor<'de> for DiscardVisitor {
            type Value = DiscardedJson;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any JSON value")
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut duplicate = false;
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    duplicate |= value.duplicate;
                }
                Ok(DiscardedJson { duplicate })
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut keys = BTreeSet::new();
                let mut duplicate = false;
                while let Some(key) = map.next_key::<String>()? {
                    duplicate |= !keys.insert(key);
                    duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                }
                Ok(DiscardedJson { duplicate })
            }
        }

        deserializer.deserialize_any(DiscardVisitor)
    }
}

#[derive(Default)]
struct LooseString {
    value: Option<String>,
    duplicate: bool,
}

impl<'de> Deserialize<'de> for LooseString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LooseStringVisitor;

        impl<'de> Visitor<'de> for LooseStringVisitor {
            type Value = LooseString;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("any JSON value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(LooseString {
                    value: Some(value.to_owned()),
                    duplicate: false,
                })
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(LooseString {
                    value: Some(value),
                    duplicate: false,
                })
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut duplicate = false;
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    duplicate |= value.duplicate;
                }
                Ok(LooseString {
                    value: None,
                    duplicate,
                })
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut keys = BTreeSet::new();
                let mut duplicate = false;
                while let Some(key) = map.next_key::<String>()? {
                    duplicate |= !keys.insert(key);
                    duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                }
                Ok(LooseString {
                    value: None,
                    duplicate,
                })
            }
        }

        deserializer.deserialize_any(LooseStringVisitor)
    }
}

trait FixedObject {
    fn flags(&self) -> &ObjectFlags;
    fn is_object(&self) -> bool;
}

#[derive(Default)]
struct ObjectSlot<T> {
    value: Option<T>,
    seen: bool,
}

impl<T: FixedObject> ObjectSlot<T> {
    fn record(&mut self, value: T, flags: &mut ObjectFlags) {
        self.seen = true;
        flags.duplicate |= value.flags().duplicate;
        flags.unknown |= value.flags().unknown;
        self.value = Some(value);
    }

    fn required(self) -> Result<T, ShapeFailure> {
        if !self.seen {
            return Err(ShapeFailure::Missing);
        }
        let value = self.value.ok_or(ShapeFailure::WrongType)?;
        if !value.is_object() {
            return Err(ShapeFailure::WrongType);
        }
        Ok(value)
    }
}

#[derive(Default)]
struct ArtifactFields {
    flags: ObjectFlags,
    algorithm: StringSlot,
    digest: StringSlot,
    is_object: bool,
}

impl FixedObject for ArtifactFields {
    fn flags(&self) -> &ObjectFlags {
        &self.flags
    }

    fn is_object(&self) -> bool {
        self.is_object
    }
}

impl<'de> Deserialize<'de> for ArtifactFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ArtifactVisitor;

        impl<'de> Visitor<'de> for ArtifactVisitor {
            type Value = ArtifactFields;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an artifact identity object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = ArtifactFields {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "algorithm" => out.algorithm.record(map.next_value()?, &mut out.flags),
                        "digest" => out.digest.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            out.flags.duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                        }
                    }
                }
                Ok(out)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = ArtifactFields::default();
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    out.flags.duplicate |= value.duplicate;
                }
                Ok(out)
            }
        }

        deserializer.deserialize_any(ArtifactVisitor)
    }
}

#[derive(Default)]
struct SelectorFields {
    flags: ObjectFlags,
    bundle_kind: StringSlot,
    witness_root: StringSlot,
    witness_algorithm: StringSlot,
    canonicalization_profile: StringSlot,
    run_id: StringSlot,
    is_object: bool,
}

impl FixedObject for SelectorFields {
    fn flags(&self) -> &ObjectFlags {
        &self.flags
    }

    fn is_object(&self) -> bool {
        self.is_object
    }
}

impl<'de> Deserialize<'de> for SelectorFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SelectorVisitor;

        impl<'de> Visitor<'de> for SelectorVisitor {
            type Value = SelectorFields;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an artifact-provenance selector object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = SelectorFields {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "bundle_kind" => out.bundle_kind.record(map.next_value()?, &mut out.flags),
                        "witness_root" => {
                            out.witness_root.record(map.next_value()?, &mut out.flags)
                        }
                        "witness_algorithm" => out
                            .witness_algorithm
                            .record(map.next_value()?, &mut out.flags),
                        "canonicalization_profile" => out
                            .canonicalization_profile
                            .record(map.next_value()?, &mut out.flags),
                        "run_id" => out.run_id.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            out.flags.duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                        }
                    }
                }
                Ok(out)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = SelectorFields::default();
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    out.flags.duplicate |= value.duplicate;
                }
                Ok(out)
            }
        }

        deserializer.deserialize_any(SelectorVisitor)
    }
}

#[derive(Default)]
struct ContributionFields {
    flags: ObjectFlags,
    target_claim_id: StringSlot,
    source_evidence_id: StringSlot,
    justification_edge_id: StringSlot,
    scope_ref: StringSlot,
    artifact: ObjectSlot<ArtifactFields>,
    is_object: bool,
}

impl FixedObject for ContributionFields {
    fn flags(&self) -> &ObjectFlags {
        &self.flags
    }

    fn is_object(&self) -> bool {
        self.is_object
    }
}

impl<'de> Deserialize<'de> for ContributionFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ContributionVisitor;

        impl<'de> Visitor<'de> for ContributionVisitor {
            type Value = ContributionFields;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a contribution object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = ContributionFields {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "target_claim_id" => out
                            .target_claim_id
                            .record(map.next_value()?, &mut out.flags),
                        "source_evidence_id" => out
                            .source_evidence_id
                            .record(map.next_value()?, &mut out.flags),
                        "justification_edge_id" => out
                            .justification_edge_id
                            .record(map.next_value()?, &mut out.flags),
                        "scope_ref" => out.scope_ref.record(map.next_value()?, &mut out.flags),
                        "artifact" => out.artifact.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            out.flags.duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                        }
                    }
                }
                Ok(out)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = ContributionFields::default();
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    out.flags.duplicate |= value.duplicate;
                }
                Ok(out)
            }
        }

        deserializer.deserialize_any(ContributionVisitor)
    }
}

#[derive(Default)]
struct AuthorityFields {
    flags: ObjectFlags,
    kind: StringSlot,
    reference: StringSlot,
    is_object: bool,
}

impl FixedObject for AuthorityFields {
    fn flags(&self) -> &ObjectFlags {
        &self.flags
    }

    fn is_object(&self) -> bool {
        self.is_object
    }
}

impl<'de> Deserialize<'de> for AuthorityFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AuthorityVisitor;

        impl<'de> Visitor<'de> for AuthorityVisitor {
            type Value = AuthorityFields;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an authority claim object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = AuthorityFields {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "kind" => out.kind.record(map.next_value()?, &mut out.flags),
                        "reference" => out.reference.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            out.flags.duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                        }
                    }
                }
                Ok(out)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = AuthorityFields::default();
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    out.flags.duplicate |= value.duplicate;
                }
                Ok(out)
            }
        }

        deserializer.deserialize_any(AuthorityVisitor)
    }
}

#[derive(Default)]
struct BindingEnvelope {
    flags: ObjectFlags,
    schema: StringSlot,
    binding_id: StringSlot,
    origin_admission_policy_id: StringSlot,
    contribution: ObjectSlot<ContributionFields>,
    acquisition_selector: ObjectSlot<SelectorFields>,
    derivation_selector: ObjectSlot<SelectorFields>,
    origin_group: StringSlot,
    authority: ObjectSlot<AuthorityFields>,
    run_id: StringSlot,
    is_object: bool,
}

impl<'de> Deserialize<'de> for BindingEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BindingVisitor;

        impl<'de> Visitor<'de> for BindingVisitor {
            type Value = BindingEnvelope;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an origin-binding bundle object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = BindingEnvelope {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "schema" => out.schema.record(map.next_value()?, &mut out.flags),
                        "binding_id" => out.binding_id.record(map.next_value()?, &mut out.flags),
                        "origin_admission_policy_id" => out
                            .origin_admission_policy_id
                            .record(map.next_value()?, &mut out.flags),
                        "contribution" => {
                            out.contribution.record(map.next_value()?, &mut out.flags)
                        }
                        "acquisition_selector" => out
                            .acquisition_selector
                            .record(map.next_value()?, &mut out.flags),
                        "derivation_selector" => out
                            .derivation_selector
                            .record(map.next_value()?, &mut out.flags),
                        "origin_group" => {
                            out.origin_group.record(map.next_value()?, &mut out.flags)
                        }
                        "authority" => out.authority.record(map.next_value()?, &mut out.flags),
                        "run_id" => out.run_id.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            out.flags.duplicate |= map.next_value::<DiscardedJson>()?.duplicate;
                        }
                    }
                }
                Ok(out)
            }

            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Default::default())
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = BindingEnvelope::default();
                while let Some(value) = seq.next_element::<DiscardedJson>()? {
                    out.flags.duplicate |= value.duplicate;
                }
                Ok(out)
            }
        }

        deserializer.deserialize_any(BindingVisitor)
    }
}

#[allow(clippy::result_large_err)]
fn parse_binding_envelope(input: &str) -> Result<BindingEnvelope, OriginBindingContextTraceV0> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let envelope = BindingEnvelope::deserialize(&mut deserializer)
        .map_err(|_| OriginBindingContextTraceV0::InvalidJson)?;
    deserializer
        .end()
        .map_err(|_| OriginBindingContextTraceV0::InvalidJson)?;
    if !envelope.is_object {
        return Err(OriginBindingContextTraceV0::TopLevelNotObject);
    }
    Ok(envelope)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedArtifactIdentity {
    algorithm: String,
    digest: String,
}

struct ParsedContribution {
    target_claim_id: String,
    source_evidence_id: String,
    justification_edge_id: String,
    scope_ref: String,
    artifact: ParsedArtifactIdentity,
}

struct ParsedAuthority {
    kind: String,
    reference: String,
}

struct ParsedBinding {
    family: OriginBindingFamilyV0,
    schema: String,
    binding_id: String,
    origin_admission_policy_id: String,
    contribution: ParsedContribution,
    acquisition_selector: ArtifactProvenanceAnchorSelectorV0,
    derivation_selector: Option<ArtifactProvenanceAnchorSelectorV0>,
    origin_group: String,
    authority: ParsedAuthority,
    run_id: String,
}

#[allow(clippy::result_large_err)]
fn validate_artifact_shape(
    fields: ArtifactFields,
) -> Result<ParsedArtifactIdentity, OriginBindingContextTraceV0> {
    let algorithm = fields.algorithm.required().map_err(ShapeFailure::trace)?;
    let digest = fields.digest.required().map_err(ShapeFailure::trace)?;
    Ok(ParsedArtifactIdentity { algorithm, digest })
}

#[allow(clippy::result_large_err)]
fn validate_selector_shape(
    fields: SelectorFields,
) -> Result<ArtifactProvenanceAnchorSelectorV0, OriginBindingContextTraceV0> {
    let bundle_kind = fields.bundle_kind.required().map_err(ShapeFailure::trace)?;
    let witness_root = fields
        .witness_root
        .required()
        .map_err(ShapeFailure::trace)?;
    let witness_algorithm = fields
        .witness_algorithm
        .required()
        .map_err(ShapeFailure::trace)?;
    let canonicalization_profile = fields
        .canonicalization_profile
        .required()
        .map_err(ShapeFailure::trace)?;
    let run_id = fields.run_id.required().map_err(ShapeFailure::trace)?;
    Ok(ArtifactProvenanceAnchorSelectorV0::new(
        bundle_kind,
        witness_root,
        witness_algorithm,
        canonicalization_profile,
        run_id,
    ))
}

#[allow(clippy::result_large_err)]
fn validate_contribution_shape(
    fields: ContributionFields,
) -> Result<ParsedContribution, OriginBindingContextTraceV0> {
    let target_claim_id = fields
        .target_claim_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let source_evidence_id = fields
        .source_evidence_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let justification_edge_id = fields
        .justification_edge_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let scope_ref = fields.scope_ref.required().map_err(ShapeFailure::trace)?;
    let artifact =
        validate_artifact_shape(fields.artifact.required().map_err(ShapeFailure::trace)?)?;
    Ok(ParsedContribution {
        target_claim_id,
        source_evidence_id,
        justification_edge_id,
        scope_ref,
        artifact,
    })
}

#[allow(clippy::result_large_err)]
fn validate_authority_shape(
    fields: AuthorityFields,
) -> Result<ParsedAuthority, OriginBindingContextTraceV0> {
    let kind = fields.kind.required().map_err(ShapeFailure::trace)?;
    let reference = fields.reference.required().map_err(ShapeFailure::trace)?;
    Ok(ParsedAuthority { kind, reference })
}

#[allow(clippy::result_large_err)]
fn validate_binding_shape(
    envelope: BindingEnvelope,
) -> Result<ParsedBinding, OriginBindingContextTraceV0> {
    if envelope.flags.duplicate {
        return Err(OriginBindingContextTraceV0::DuplicateKey);
    }

    let BindingEnvelope {
        mut flags,
        schema,
        binding_id,
        origin_admission_policy_id,
        contribution,
        acquisition_selector,
        derivation_selector,
        origin_group,
        authority,
        run_id,
        ..
    } = envelope;

    let schema = schema.required().map_err(ShapeFailure::trace)?;
    let family = match schema.as_str() {
        ORIGIN_BINDING_ACQUISITION_SCHEMA_V0 => OriginBindingFamilyV0::Acquisition,
        ORIGIN_BINDING_DERIVATION_SCHEMA_V0 => OriginBindingFamilyV0::Derivation,
        _ => return Err(OriginBindingContextTraceV0::SchemaMismatch),
    };
    let binding_id = binding_id.required().map_err(ShapeFailure::trace)?;
    let origin_admission_policy_id = origin_admission_policy_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let contribution =
        validate_contribution_shape(contribution.required().map_err(ShapeFailure::trace)?)?;
    let acquisition_selector = validate_selector_shape(
        acquisition_selector
            .required()
            .map_err(ShapeFailure::trace)?,
    )?;
    let derivation_selector = match family {
        OriginBindingFamilyV0::Acquisition => {
            if derivation_selector.seen {
                flags.unknown = true;
            }
            None
        }
        OriginBindingFamilyV0::Derivation => Some(validate_selector_shape(
            derivation_selector
                .required()
                .map_err(ShapeFailure::trace)?,
        )?),
    };
    let origin_group = origin_group.required().map_err(ShapeFailure::trace)?;
    let authority = validate_authority_shape(authority.required().map_err(ShapeFailure::trace)?)?;
    let run_id = run_id.required().map_err(ShapeFailure::trace)?;
    if flags.unknown {
        return Err(OriginBindingContextTraceV0::UnknownField);
    }

    Ok(ParsedBinding {
        family,
        schema,
        binding_id,
        origin_admission_policy_id,
        contribution,
        acquisition_selector,
        derivation_selector,
        origin_group,
        authority,
        run_id,
    })
}

#[allow(clippy::result_large_err)]
fn validate_binding_semantics(parsed: &ParsedBinding) -> Result<(), OriginBindingContextTraceV0> {
    validate_protocol_identifier(&parsed.binding_id)?;
    validate_protocol_identifier(&parsed.origin_admission_policy_id)?;
    validate_replay_reference(&parsed.contribution.target_claim_id)?;
    validate_replay_reference(&parsed.contribution.source_evidence_id)?;
    validate_replay_reference(&parsed.contribution.justification_edge_id)?;
    validate_replay_reference(&parsed.contribution.scope_ref)?;
    validate_artifact_semantics(&parsed.contribution.artifact)?;
    if !valid_provenance_selector(
        &parsed.acquisition_selector,
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ) {
        return Err(OriginBindingContextTraceV0::InvalidAcquisitionSelector);
    }
    if let Some(selector) = &parsed.derivation_selector {
        if !valid_provenance_selector(selector, ARTIFACT_DERIVATION_BUNDLE_KIND_V0) {
            return Err(OriginBindingContextTraceV0::InvalidDerivationSelector);
        }
    }
    validate_protocol_identifier(&parsed.origin_group)?;
    validate_protocol_identifier(&parsed.authority.kind)?;
    validate_protocol_identifier(&parsed.authority.reference)?;
    validate_protocol_identifier(&parsed.run_id)?;
    Ok(())
}

#[allow(clippy::result_large_err)]
fn validate_protocol_identifier(value: &str) -> Result<(), OriginBindingContextTraceV0> {
    if !valid_identifier(value) {
        return Err(OriginBindingContextTraceV0::InvalidProtocolIdentifier);
    }
    Ok(())
}

fn valid_identifier(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && bytes[0].is_ascii_alphanumeric()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'/' | b'-')
        })
}

#[allow(clippy::result_large_err)]
fn validate_replay_reference(value: &str) -> Result<(), OriginBindingContextTraceV0> {
    if !valid_origin_binding_replay_reference_v0(value) {
        return Err(OriginBindingContextTraceV0::InvalidReplayReferenceLength);
    }
    Ok(())
}

/// Exact origin-binding replay-reference grammar (`1..=1024` bytes), shared
/// with the support-contribution audit's inherited-reference checks.
pub(crate) fn valid_origin_binding_replay_reference_v0(value: &str) -> bool {
    !value.is_empty() && value.len() <= 1_024
}

/// Exact origin-binding protocol-identifier grammar for origin groups.
pub(crate) fn valid_origin_group_v0(value: &str) -> bool {
    valid_identifier(value)
}

/// Exact origin-binding artifact-digest grammar (64 lowercase hex bytes).
pub(crate) fn valid_origin_binding_artifact_digest_v0(value: &str) -> bool {
    is_lowercase_hex_64(value)
}

/// Exact structural grammar for the two outer origin-binding selector kinds.
///
/// This validation is structural and protocol-exact. It does not re-read
/// bundle bytes, recompute the witness root, or confer standalone authority.
pub(crate) fn valid_origin_binding_selector_v0(
    selector: &ArtifactProvenanceAnchorSelectorV0,
) -> bool {
    let bundle_kind = selector.bundle_kind();

    (bundle_kind == ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0
        || bundle_kind == ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0)
        && is_lowercase_hex_64(selector.witness_root())
        && selector.witness_algorithm() == ORIGIN_BINDING_WITNESS_ALGORITHM_V0
        && selector.canonicalization_profile() == ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0
        && valid_identifier(selector.run_id())
}

#[allow(clippy::result_large_err)]
fn validate_artifact_semantics(
    artifact: &ParsedArtifactIdentity,
) -> Result<(), OriginBindingContextTraceV0> {
    if artifact.algorithm != ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0 {
        return Err(OriginBindingContextTraceV0::UnsupportedArtifactAlgorithm);
    }
    if !is_lowercase_hex_64(&artifact.digest) {
        return Err(OriginBindingContextTraceV0::InvalidArtifactDigest);
    }
    Ok(())
}

fn valid_provenance_selector(
    selector: &ArtifactProvenanceAnchorSelectorV0,
    expected_bundle_kind: &str,
) -> bool {
    selector.bundle_kind() == expected_bundle_kind
        && is_lowercase_hex_64(selector.witness_root())
        && selector.witness_algorithm() == ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0
        && selector.canonicalization_profile() == ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0
        && valid_identifier(selector.run_id())
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn encode_binding(parsed: &ParsedBinding) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"{\"schema\":");
    push_json_string(&mut out, &parsed.schema);
    out.extend_from_slice(b",\"binding_id\":");
    push_json_string(&mut out, &parsed.binding_id);
    out.extend_from_slice(b",\"origin_admission_policy_id\":");
    push_json_string(&mut out, &parsed.origin_admission_policy_id);
    out.extend_from_slice(b",\"contribution\":{\"target_claim_id\":");
    push_json_string(&mut out, &parsed.contribution.target_claim_id);
    out.extend_from_slice(b",\"source_evidence_id\":");
    push_json_string(&mut out, &parsed.contribution.source_evidence_id);
    out.extend_from_slice(b",\"justification_edge_id\":");
    push_json_string(&mut out, &parsed.contribution.justification_edge_id);
    out.extend_from_slice(b",\"scope_ref\":");
    push_json_string(&mut out, &parsed.contribution.scope_ref);
    out.extend_from_slice(b",\"artifact\":{\"algorithm\":");
    push_json_string(&mut out, &parsed.contribution.artifact.algorithm);
    out.extend_from_slice(b",\"digest\":");
    push_json_string(&mut out, &parsed.contribution.artifact.digest);
    out.extend_from_slice(b"}},\"acquisition_selector\":");
    push_selector(&mut out, &parsed.acquisition_selector);
    if let Some(selector) = &parsed.derivation_selector {
        out.extend_from_slice(b",\"derivation_selector\":");
        push_selector(&mut out, selector);
    }
    out.extend_from_slice(b",\"origin_group\":");
    push_json_string(&mut out, &parsed.origin_group);
    out.extend_from_slice(b",\"authority\":{\"kind\":");
    push_json_string(&mut out, &parsed.authority.kind);
    out.extend_from_slice(b",\"reference\":");
    push_json_string(&mut out, &parsed.authority.reference);
    out.extend_from_slice(b"},\"run_id\":");
    push_json_string(&mut out, &parsed.run_id);
    out.push(b'}');
    out
}

fn push_selector(out: &mut Vec<u8>, selector: &ArtifactProvenanceAnchorSelectorV0) {
    out.extend_from_slice(b"{\"bundle_kind\":");
    push_json_string(out, selector.bundle_kind());
    out.extend_from_slice(b",\"witness_root\":");
    push_json_string(out, selector.witness_root());
    out.extend_from_slice(b",\"witness_algorithm\":");
    push_json_string(out, selector.witness_algorithm());
    out.extend_from_slice(b",\"canonicalization_profile\":");
    push_json_string(out, selector.canonicalization_profile());
    out.extend_from_slice(b",\"run_id\":");
    push_json_string(out, selector.run_id());
    out.push(b'}');
}

fn push_json_string(out: &mut Vec<u8>, value: &str) {
    const LOWER_HEX: &[u8; 16] = b"0123456789abcdef";

    out.push(b'"');
    for character in value.chars() {
        match character {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\u{0008}' => out.extend_from_slice(b"\\b"),
            '\u{0009}' => out.extend_from_slice(b"\\t"),
            '\u{000a}' => out.extend_from_slice(b"\\n"),
            '\u{000c}' => out.extend_from_slice(b"\\f"),
            '\u{000d}' => out.extend_from_slice(b"\\r"),
            '\u{0000}'..='\u{001f}' => {
                let byte = character as u8;
                out.extend_from_slice(b"\\u00");
                out.push(LOWER_HEX[(byte >> 4) as usize]);
                out.push(LOWER_HEX[(byte & 0x0f) as usize]);
            }
            _ => {
                let mut encoded = [0; 4];
                out.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
            }
        }
    }
    out.push(b'"');
}

#[cfg(test)]
mod evaluation_regression_tests {
    use super::*;
    use crate::replay_standing_context;
    use crate::resolution_content_closure::{
        ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0,
        ResolutionForeignBundleObjectInputV0,
    };
    use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};

    const ACQUISITION_BINDING: &[u8] =
        include_bytes!("../../../fixtures/origin-binding-v0/acquisition-binding.json");
    const DERIVATION_BINDING: &[u8] =
        include_bytes!("../../../fixtures/origin-binding-v0/derivation-binding.json");
    const PARENT_ACQUISITION_BUNDLE: &[u8] = include_bytes!(
        "../../../fixtures/origin-binding-v0/derivation-parent-acquisition-bundle.json"
    );
    const ACQUISITION_BUNDLE: &[u8] =
        include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-bundle.json");
    const DERIVATION_BUNDLE: &[u8] =
        include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-bundle.json");
    const ACQUISITION_ARTIFACT: &[u8] =
        include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-artifact.txt");
    const DERIVATION_PARENT: &[u8] =
        include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-parent.txt");
    const DERIVATION_DERIVED: &[u8] =
        include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-derived.txt");

    const ACQUISITION_BINDING_ROOT: &str =
        "4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7";
    const DERIVATION_BINDING_ROOT: &str =
        "24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29";
    const ACQUISITION_ROOT: &str =
        "4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e";
    const PARENT_ACQUISITION_ROOT: &str =
        "94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce";
    const DERIVATION_ROOT: &str =
        "472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd";
    const ACQUISITION_DIGEST: &str =
        "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076";
    const PARENT_DIGEST: &str = "b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060";
    const DERIVED_DIGEST: &str = "1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005";
    const SCOPE: &str = "scope:origin-example";
    const SEED: [u8; 32] = [45; 32];

    fn selector(
        kind: &str,
        root: &str,
        algorithm: &str,
        profile: &str,
        run_id: &str,
    ) -> ArtifactProvenanceAnchorSelectorV0 {
        ArtifactProvenanceAnchorSelectorV0::new(kind, root, algorithm, profile, run_id)
    }

    fn acquisition_binding_selector() -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            ACQUISITION_BINDING_ROOT,
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-origin-binding-acq-0001",
        )
    }

    fn derivation_binding_selector() -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
            DERIVATION_BINDING_ROOT,
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-origin-binding-der-0001",
        )
    }

    fn acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
            ACQUISITION_ROOT,
            ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
            ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
            "run-acq-0001",
        )
    }

    fn parent_acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
            PARENT_ACQUISITION_ROOT,
            ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
            ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
            "run-acq-alpha-0001",
        )
    }

    fn derivation_selector() -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
            DERIVATION_ROOT,
            ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
            ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
            "run-der-0001",
        )
    }

    fn replace_once(bytes: &[u8], from: &str, to: &str) -> Vec<u8> {
        let text = std::str::from_utf8(bytes).unwrap();
        assert_eq!(text.matches(from).count(), 1);
        text.replacen(from, to, 1).into_bytes()
    }

    fn selector_for_bundle(
        kind: &str,
        run_id: &str,
        bytes: &[u8],
    ) -> ArtifactProvenanceAnchorSelectorV0 {
        selector(
            kind,
            &sha256_hex(bytes),
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            run_id,
        )
    }

    fn closure(
        artifacts: &[(&str, &[u8])],
        bundles: &[(&ArtifactProvenanceAnchorSelectorV0, &[u8])],
    ) -> ResolutionContentClosureV0 {
        let artifact_inputs = artifacts
            .iter()
            .map(|(digest, bytes)| {
                ResolutionArtifactObjectInputV0::new(
                    ResolutionArtifactObjectKeyV0::new("sha256", *digest),
                    bytes,
                )
            })
            .collect::<Vec<_>>();
        let bundle_inputs = bundles
            .iter()
            .map(|(selector, bytes)| {
                ResolutionForeignBundleObjectInputV0::new((*selector).clone(), bytes)
            })
            .collect::<Vec<_>>();
        ResolutionContentClosureV0::construct(&artifact_inputs, &bundle_inputs).unwrap()
    }

    fn snapshot(
        evidence_id: &str,
        edge_id: &str,
        include_claim: bool,
        anchors: &[ArtifactProvenanceAnchorSelectorV0],
    ) -> StandingReplaySnapshot {
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
            writer
                .append(
                    Provenance::new("origin-binding-test", "legacy-control"),
                    Payload::ClaimAsserted {
                        claim_id: "claim-origin-0001".into(),
                        statement: "one origin-bound claim".into(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            if include_claim {
                writer
                    .append(
                        Provenance::new("origin-binding-test", "claim"),
                        Payload::ClaimAssertedV2 {
                            claim_id: "claim-origin-0001".into(),
                            statement: "one origin-bound claim".into(),
                            scope_ref: SCOPE.into(),
                            actor_class: "AgentProposer".into(),
                            content_hash: String::new(),
                            metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
                        },
                    )
                    .unwrap();
            }
            writer
                .append(
                    Provenance::new("origin-binding-test", "evidence"),
                    Payload::EvidenceRegistered {
                        evidence_id: evidence_id.into(),
                        evidence_kind: "ExternalSource".into(),
                        summary: "one source observation".into(),
                        scope_ref: SCOPE.into(),
                        actor_class: "SourceImporter".into(),
                        content_hash: String::new(),
                        metadata_json: "{}".into(),
                    },
                )
                .unwrap();
            writer
                .append(
                    Provenance::new("origin-binding-test", "edge"),
                    Payload::JustificationEdgeRecorded {
                        edge_id: edge_id.into(),
                        edge_kind: "supports".into(),
                        source_id: evidence_id.into(),
                        target_id: "claim-origin-0001".into(),
                        scope_ref: SCOPE.into(),
                        actor_class: "HumanRoot".into(),
                        rationale: "candidate relationship only".into(),
                        metadata_json: "{}".into(),
                    },
                )
                .unwrap();
            for anchor in anchors {
                writer
                    .append(
                        Provenance::new("origin-binding-test", "same-replay-anchor"),
                        Payload::SegmentAnchored {
                            bundle_kind: anchor.bundle_kind().into(),
                            witness_root: anchor.witness_root().into(),
                            witness_algorithm: anchor.witness_algorithm().into(),
                            canonicalization_profile: anchor.canonicalization_profile().into(),
                            run_id: anchor.run_id().into(),
                        },
                    )
                    .unwrap();
            }
        }
        let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
        replay_standing_context(&reader).unwrap()
    }

    fn assert_public_private_equivalence(
        snapshot: &StandingReplaySnapshot,
        selector: &ArtifactProvenanceAnchorSelectorV0,
        closure: &ResolutionContentClosureV0,
        class: OriginBindingEvaluationClassV0,
        has_validated_subject: bool,
        expected_len: usize,
        expected_sha256: &str,
    ) {
        let evaluation = evaluate_origin_binding_context_v0(snapshot, selector, closure);
        let public = snapshot.resolve_origin_binding_context_v0(selector, closure);
        let (private_trace, actual_class, validated_subject) = evaluation.into_parts();
        assert_eq!(public, private_trace);
        assert_eq!(public.canonical_bytes(), private_trace.canonical_bytes());
        assert_eq!(actual_class, class);
        assert_eq!(validated_subject.is_some(), has_validated_subject);
        let bytes = public.canonical_bytes();
        assert_eq!(bytes.len(), expected_len);
        assert_eq!(sha256_hex(&bytes), expected_sha256);
    }

    #[test]
    fn public_resolver_matches_private_evaluator_and_preserves_pinned_trace_bytes() {
        let binding = acquisition_binding_selector();
        let acquisition = acquisition_selector();
        let acquisition_snapshot = snapshot(
            "evidence-origin-acq-0001",
            "edge-origin-acq-0001",
            true,
            &[binding.clone(), acquisition.clone()],
        );
        let acquisition_closure = closure(
            &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
            &[
                (&binding, ACQUISITION_BINDING),
                (&acquisition, ACQUISITION_BUNDLE),
            ],
        );
        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &acquisition_closure,
            OriginBindingEvaluationClassV0::Matched,
            true,
            3_365,
            "7276c9d33671123e3dfb53a88ffb8ac9bc6c572a5ca500de9f9151052657d178",
        );

        let derivation_binding = derivation_binding_selector();
        let parent_acquisition = parent_acquisition_selector();
        let derivation = derivation_selector();
        let derivation_snapshot = snapshot(
            "evidence-origin-der-0001",
            "edge-origin-der-0001",
            true,
            &[
                derivation_binding.clone(),
                parent_acquisition.clone(),
                derivation.clone(),
            ],
        );
        let derivation_closure = closure(
            &[
                (PARENT_DIGEST, DERIVATION_PARENT),
                (DERIVED_DIGEST, DERIVATION_DERIVED),
            ],
            &[
                (&derivation_binding, DERIVATION_BINDING),
                (&parent_acquisition, PARENT_ACQUISITION_BUNDLE),
                (&derivation, DERIVATION_BUNDLE),
            ],
        );
        assert_public_private_equivalence(
            &derivation_snapshot,
            &derivation_binding,
            &derivation_closure,
            OriginBindingEvaluationClassV0::Matched,
            true,
            5_184,
            "9b5907b9349100618ffe969e6fb7a9a6bae5feb89e2167691133ead778058492",
        );

        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(&[], &[]),
            OriginBindingEvaluationClassV0::BindingBytesUnavailable,
            false,
            40,
            "0cb46ca223e0ca6384a844a9a62b3cdf7b1a24346df57c19b6f20241bb2b5aba",
        );
        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(&[], &[(&binding, b"{")]),
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            false,
            26,
            "3967aa23f8172bce1d40a4508e6f10f800b0163d1e2788223c6b35951f50abe6",
        );

        let wrong_root = selector(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            &"0".repeat(64),
            ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
            ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
            "run-origin-binding-acq-0001",
        );
        assert_public_private_equivalence(
            &snapshot(
                "evidence-origin-acq-0001",
                "edge-origin-acq-0001",
                true,
                &[],
            ),
            &wrong_root,
            &closure(&[], &[(&wrong_root, ACQUISITION_BINDING)]),
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            false,
            43,
            "a3a75ac579aecf7686c0b2ca527d2b60e90be21fbf9204dd2c1ffd08eb92d05e",
        );

        assert_public_private_equivalence(
            &snapshot(
                "evidence-origin-acq-0001",
                "edge-origin-acq-0001",
                false,
                &[binding.clone(), acquisition.clone()],
            ),
            &binding,
            &acquisition_closure,
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            false,
            33,
            "a56dbea879ce8100f5a6b46c35b6df94c1b10d927f909a79cd90f699d33971c4",
        );

        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(&[], &[(&binding, ACQUISITION_BINDING)]),
            OriginBindingEvaluationClassV0::AvailabilityOnly,
            true,
            102,
            "2d7091908057c02ddc393a831fd8d64d3008d1b6069b9be56c280e85d62606f1",
        );
        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(
                &[],
                &[
                    (&binding, ACQUISITION_BINDING),
                    (&acquisition, ACQUISITION_BUNDLE),
                ],
            ),
            OriginBindingEvaluationClassV0::AvailabilityOnly,
            true,
            104,
            "d839b29d487a7632da555632ebeb332514576ce4069dd5c8a4591177fdd8c70e",
        );
        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(
                &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
                &[(&binding, ACQUISITION_BINDING), (&acquisition, b"{")],
            ),
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            true,
            96,
            "ac66a8184b8cfa88280c528d8e142d679b89d9d4fd4e7feec3e53b4898baa4c9",
        );
        assert_public_private_equivalence(
            &acquisition_snapshot,
            &binding,
            &closure(
                &[(ACQUISITION_DIGEST, b"wrong artifact")],
                &[
                    (&binding, ACQUISITION_BINDING),
                    (&acquisition, ACQUISITION_BUNDLE),
                ],
            ),
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            true,
            108,
            "e186eb56197666c9c53ff2e213103645bd308f614bdde94986cb1088df425208",
        );

        let coherence_bytes = replace_once(ACQUISITION_BINDING, ACQUISITION_DIGEST, DERIVED_DIGEST);
        let coherence_binding = selector_for_bundle(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            "run-origin-binding-acq-0001",
            &coherence_bytes,
        );
        let coherence_snapshot = snapshot(
            "evidence-origin-acq-0001",
            "edge-origin-acq-0001",
            true,
            &[coherence_binding.clone(), acquisition.clone()],
        );
        let coherence_closure = closure(
            &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
            &[
                (&coherence_binding, &coherence_bytes),
                (&acquisition, ACQUISITION_BUNDLE),
            ],
        );
        assert_public_private_equivalence(
            &coherence_snapshot,
            &coherence_binding,
            &coherence_closure,
            OriginBindingEvaluationClassV0::DefinitivelyRejected,
            true,
            69,
            "6f06897a73fe5818e57c5dd4ec40970158a62bc6dd2c5e175f2d72363428a191",
        );
    }
}
