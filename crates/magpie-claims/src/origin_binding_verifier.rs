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

impl StandingReplaySnapshot {
    /// Resolve one exact origin-binding assertion from this replay and closure.
    pub fn resolve_origin_binding_context_v0(
        &self,
        binding_selector: &ArtifactProvenanceAnchorSelectorV0,
        closure: &ResolutionContentClosureV0,
    ) -> OriginBindingContextTraceV0 {
        let supplied = match closure.foreign_bundle(binding_selector) {
            Some(bytes) => bytes,
            None => return OriginBindingContextTraceV0::BindingBundleUnavailable,
        };
        if supplied.len() > MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0 {
            return OriginBindingContextTraceV0::BindingBundleTooLarge;
        }

        let text = match decode_bundle_text(supplied) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let envelope = match parse_binding_envelope(text) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let parsed = match validate_binding_shape(envelope) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        if let Err(trace) = validate_binding_semantics(&parsed) {
            return trace;
        }

        let canonical = encode_binding(&parsed);
        if canonical != supplied {
            return OriginBindingContextTraceV0::NonCanonicalEncoding;
        }

        let expected_kind = match parsed.family {
            OriginBindingFamilyV0::Acquisition => ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            OriginBindingFamilyV0::Derivation => ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        };
        if binding_selector.bundle_kind() != expected_kind {
            return OriginBindingContextTraceV0::BindingBundleKindMismatch;
        }
        if binding_selector.witness_algorithm() != ORIGIN_BINDING_WITNESS_ALGORITHM_V0 {
            return OriginBindingContextTraceV0::BindingWitnessAlgorithmMismatch;
        }
        if binding_selector.canonicalization_profile() != ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0
        {
            return OriginBindingContextTraceV0::BindingCanonicalizationProfileMismatch;
        }
        if binding_selector.run_id() != parsed.run_id {
            return OriginBindingContextTraceV0::BindingRunIdMismatch;
        }

        let computed_witness_root = sha256_hex(&canonical);
        if binding_selector.witness_root() != computed_witness_root {
            return OriginBindingContextTraceV0::BindingWitnessRootMismatch;
        }
        let identity = selector_identity(binding_selector);
        let occurrences = self.anchors().occurrences(&identity);
        if occurrences.is_empty() {
            return OriginBindingContextTraceV0::BindingAnchorAbsent;
        }

        let target = match self
            .standing()
            .typed_claim(&parsed.contribution.target_claim_id)
        {
            Some(value) => value,
            None => return OriginBindingContextTraceV0::TargetClaimAbsent,
        };
        let evidence = match self
            .standing()
            .typed_evidence(&parsed.contribution.source_evidence_id)
        {
            Some(value) => value,
            None => return OriginBindingContextTraceV0::SourceEvidenceAbsent,
        };
        let edge = match self
            .standing()
            .justification_edge(&parsed.contribution.justification_edge_id)
        {
            Some(value) => value,
            None => return OriginBindingContextTraceV0::JustificationEdgeAbsent,
        };
        if edge.source_id != parsed.contribution.source_evidence_id {
            return OriginBindingContextTraceV0::EdgeSourceMismatch;
        }
        if edge.target_id != parsed.contribution.target_claim_id {
            return OriginBindingContextTraceV0::EdgeTargetMismatch;
        }
        if target.scope_ref != parsed.contribution.scope_ref
            || evidence.scope_ref != parsed.contribution.scope_ref
            || edge.scope_ref != parsed.contribution.scope_ref
        {
            return OriginBindingContextTraceV0::ScopeMismatch;
        }

        let acquisition_receipt = match resolve_artifact_acquisition_from_closure_v0(
            self,
            &parsed.acquisition_selector,
            closure,
        ) {
            ArtifactProvenanceContextTraceV0::MatchedAcquisition(receipt) => receipt,
            trace => {
                return OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved { trace }
            }
        };

        let contribution = ContributionIdentityV0 {
            target_claim_id: parsed.contribution.target_claim_id.clone(),
            source_evidence_id: parsed.contribution.source_evidence_id.clone(),
            justification_edge_id: parsed.contribution.justification_edge_id.clone(),
            scope_ref: parsed.contribution.scope_ref.clone(),
            artifact_algorithm: parsed.contribution.artifact.algorithm.clone(),
            artifact_digest: parsed.contribution.artifact.digest.clone(),
        };
        let comparison_namespace = OriginComparisonNamespaceV0 {
            origin_admission_policy_id: parsed.origin_admission_policy_id.clone(),
            target_claim_id: parsed.contribution.target_claim_id.clone(),
            scope_ref: parsed.contribution.scope_ref.clone(),
        };

        let provenance = match &parsed.derivation_selector {
            None => {
                if acquisition_receipt.artifact_algorithm()
                    != parsed.contribution.artifact.algorithm
                    || acquisition_receipt.artifact_digest() != parsed.contribution.artifact.digest
                {
                    return OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromContributionArtifact;
                }
                OriginBindingProvenanceV0::Acquisition {
                    acquisition_receipt,
                }
            }
            Some(derivation_selector) => {
                let derivation_receipt = match resolve_artifact_derivation_from_closure_v0(
                    self,
                    derivation_selector,
                    closure,
                ) {
                    ArtifactProvenanceContextTraceV0::MatchedDerivation(receipt) => receipt,
                    trace => {
                        return OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
                            trace,
                        }
                    }
                };
                if acquisition_receipt.artifact_algorithm()
                    != derivation_receipt.parent_artifact_algorithm()
                    || acquisition_receipt.artifact_digest()
                        != derivation_receipt.parent_artifact_digest()
                {
                    return OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromDerivationParent;
                }
                if derivation_receipt.derived_artifact_algorithm()
                    != parsed.contribution.artifact.algorithm
                    || derivation_receipt.derived_artifact_digest()
                        != parsed.contribution.artifact.digest
                {
                    return OriginBindingContextTraceV0::DerivationOutputDiffersFromContributionArtifact;
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
            contribution,
            comparison_namespace,
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

        match receipt.family {
            OriginBindingFamilyV0::Acquisition => {
                OriginBindingContextTraceV0::MatchedAcquisition(receipt)
            }
            OriginBindingFamilyV0::Derivation => {
                OriginBindingContextTraceV0::MatchedDerivation(receipt)
            }
        }
    }
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
    if value.is_empty() || value.len() > 1_024 {
        return Err(OriginBindingContextTraceV0::InvalidReplayReferenceLength);
    }
    Ok(())
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
