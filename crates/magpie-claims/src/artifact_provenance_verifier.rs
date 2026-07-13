//! Standing-inert artifact provenance verification from one verified replay snapshot.
//!
//! Receipts are serializable audit material only. Their private fields, lack of
//! constructors, and snapshot-only resolution path prevent callers from
//! presenting receipt-shaped values as replay authority.
//!
//! Acquisition receipts cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::ArtifactAcquisitionReceiptV0;
//! let _receipt = ArtifactAcquisitionReceiptV0 {
//!     verifier_profile: String::new(),
//! };
//! ```
//!
//! Derivation receipts cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::ArtifactDerivationReceiptV0;
//! let _receipt = ArtifactDerivationReceiptV0 {
//!     verifier_profile: String::new(),
//! };
//! ```
//!
//! A standalone `StandingView` deliberately has no artifact resolver:
//!
//! ```compile_fail
//! use magpie_claims::{ArtifactProvenanceAnchorSelectorV0, StandingView};
//! let view = StandingView::new();
//! let selector = ArtifactProvenanceAnchorSelectorV0::new("", "", "", "", "");
//! let _ = view.resolve_artifact_acquisition_context_v0(&selector, None, None);
//! ```
//!
//! There is no free resolver that accepts a caller-created anchor index:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     resolve_artifact_acquisition_context_v0, ArtifactProvenanceAnchorSelectorV0,
//!     DeadboltAnchorIndex,
//! };
//! let anchors = DeadboltAnchorIndex::new();
//! let selector = ArtifactProvenanceAnchorSelectorV0::new("", "", "", "", "");
//! let _ = resolve_artifact_acquisition_context_v0(&anchors, &selector, None, None);
//! ```
//!
//! Deserializing a receipt is not supported:
//!
//! ```compile_fail
//! use magpie_claims::ArtifactAcquisitionReceiptV0;
//! let _: ArtifactAcquisitionReceiptV0 = serde_json::from_str("{}").unwrap();
//! ```

use std::collections::BTreeSet;
use std::fmt;

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use sha2::{Digest, Sha256};

use crate::deadbolt_context::{DeadboltAnchorIdentity, DeadboltAnchorOccurrence};
use crate::replay_snapshot::StandingReplaySnapshot;

pub const ARTIFACT_ACQUISITION_BUNDLE_KIND_V0: &str = "magpie-artifact-acquisition-v0";

pub const ARTIFACT_ACQUISITION_SCHEMA_V0: &str = "magpie-artifact-acquisition-v0";

pub const ARTIFACT_DERIVATION_BUNDLE_KIND_V0: &str = "magpie-artifact-derivation-v0";

pub const ARTIFACT_DERIVATION_SCHEMA_V0: &str = "magpie-artifact-derivation-v0";

pub const ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-artifact-provenance-json-v0";

pub const ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0: &str = "magpie-artifact-provenance-verifier-v0";

pub const ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0: &str = "sha256";

pub const ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0: &str = "sha256";

pub const MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0: usize = 16_384;

/// Untrusted selector and lookup material for one artifact-provenance attempt.
///
/// Construction is intentionally infallible. Malformed and hostile values are
/// classified only by the verifier's fixed stage precedence. A selector is not
/// proof that any anchor occurred in an accepted replay.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ArtifactProvenanceAnchorSelectorV0 {
    bundle_kind: String,
    witness_root: String,
    witness_algorithm: String,
    canonicalization_profile: String,
    run_id: String,
}

impl ArtifactProvenanceAnchorSelectorV0 {
    pub fn new(
        bundle_kind: impl Into<String>,
        witness_root: impl Into<String>,
        witness_algorithm: impl Into<String>,
        canonicalization_profile: impl Into<String>,
        run_id: impl Into<String>,
    ) -> Self {
        Self {
            bundle_kind: bundle_kind.into(),
            witness_root: witness_root.into(),
            witness_algorithm: witness_algorithm.into(),
            canonicalization_profile: canonicalization_profile.into(),
            run_id: run_id.into(),
        }
    }

    pub fn bundle_kind(&self) -> &str {
        &self.bundle_kind
    }

    pub fn witness_root(&self) -> &str {
        &self.witness_root
    }

    pub fn witness_algorithm(&self) -> &str {
        &self.witness_algorithm
    }

    pub fn canonicalization_profile(&self) -> &str {
        &self.canonicalization_profile
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}

/// Successful acquisition verification derived from one accepted replay.
///
/// The receipt has no public constructor and no `Deserialize` implementation.
/// Cloning or serializing it does not confer authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArtifactAcquisitionReceiptV0 {
    verifier_profile: String,
    selector: ArtifactProvenanceAnchorSelectorV0,
    schema: String,
    acquisition_id: String,
    acquisition_profile: String,
    artifact_algorithm: String,
    artifact_digest: String,
    observed_locator: String,
    bundle_byte_len: u64,
    computed_witness_root: String,
    artifact_byte_len: u64,
    computed_artifact_sha256: String,
    occurrences: Vec<DeadboltAnchorOccurrence>,
}

impl ArtifactAcquisitionReceiptV0 {
    pub fn verifier_profile(&self) -> &str {
        &self.verifier_profile
    }

    pub fn selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.selector
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn acquisition_id(&self) -> &str {
        &self.acquisition_id
    }

    pub fn acquisition_profile(&self) -> &str {
        &self.acquisition_profile
    }

    pub fn artifact_algorithm(&self) -> &str {
        &self.artifact_algorithm
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn observed_locator(&self) -> &str {
        &self.observed_locator
    }

    pub fn bundle_byte_len(&self) -> u64 {
        self.bundle_byte_len
    }

    pub fn computed_witness_root(&self) -> &str {
        &self.computed_witness_root
    }

    pub fn artifact_byte_len(&self) -> u64 {
        self.artifact_byte_len
    }

    pub fn computed_artifact_sha256(&self) -> &str {
        &self.computed_artifact_sha256
    }

    pub fn occurrences(&self) -> &[DeadboltAnchorOccurrence] {
        &self.occurrences
    }
}

/// Successful direct-derivation verification derived from one accepted replay.
///
/// The receipt has no public constructor and no `Deserialize` implementation.
/// Cloning or serializing it does not confer authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ArtifactDerivationReceiptV0 {
    verifier_profile: String,
    selector: ArtifactProvenanceAnchorSelectorV0,
    schema: String,
    derivation_id: String,
    transformation_profile: String,
    parent_artifact_algorithm: String,
    parent_artifact_digest: String,
    derived_artifact_algorithm: String,
    derived_artifact_digest: String,
    bundle_byte_len: u64,
    computed_witness_root: String,
    parent_artifact_byte_len: u64,
    computed_parent_artifact_sha256: String,
    derived_artifact_byte_len: u64,
    computed_derived_artifact_sha256: String,
    occurrences: Vec<DeadboltAnchorOccurrence>,
}

impl ArtifactDerivationReceiptV0 {
    pub fn verifier_profile(&self) -> &str {
        &self.verifier_profile
    }

    pub fn selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.selector
    }

    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn derivation_id(&self) -> &str {
        &self.derivation_id
    }

    pub fn transformation_profile(&self) -> &str {
        &self.transformation_profile
    }

    pub fn parent_artifact_algorithm(&self) -> &str {
        &self.parent_artifact_algorithm
    }

    pub fn parent_artifact_digest(&self) -> &str {
        &self.parent_artifact_digest
    }

    pub fn derived_artifact_algorithm(&self) -> &str {
        &self.derived_artifact_algorithm
    }

    pub fn derived_artifact_digest(&self) -> &str {
        &self.derived_artifact_digest
    }

    pub fn bundle_byte_len(&self) -> u64 {
        self.bundle_byte_len
    }

    pub fn computed_witness_root(&self) -> &str {
        &self.computed_witness_root
    }

    pub fn parent_artifact_byte_len(&self) -> u64 {
        self.parent_artifact_byte_len
    }

    pub fn computed_parent_artifact_sha256(&self) -> &str {
        &self.computed_parent_artifact_sha256
    }

    pub fn derived_artifact_byte_len(&self) -> u64 {
        self.derived_artifact_byte_len
    }

    pub fn computed_derived_artifact_sha256(&self) -> &str {
        &self.computed_derived_artifact_sha256
    }

    pub fn occurrences(&self) -> &[DeadboltAnchorOccurrence] {
        &self.occurrences
    }
}

/// Closed deterministic audit outcome for artifact provenance v0.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "receipt", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)] // The public matched variants retain their typed receipts.
pub enum ArtifactProvenanceContextTraceV0 {
    BundleUnavailable,
    BundleTooLarge,
    InvalidUtf8,
    Utf8BomPresent,
    InvalidJson,
    TopLevelNotObject,
    DuplicateKey,
    MissingField,
    UnknownField,
    WrongJsonType,
    SchemaMismatch,
    InvalidIdentifier,
    InvalidObservedLocator,
    UnsupportedArtifactAlgorithm,
    InvalidArtifactDigest,
    ParentEqualsDerived,
    NonCanonicalEncoding,
    AnchorAbsent,
    BundleKindMismatch,
    WitnessAlgorithmMismatch,
    CanonicalizationProfileMismatch,
    RunIdMismatch,
    WitnessRootMismatch,
    ArtifactUnavailable,
    ArtifactDigestMismatch,
    ParentArtifactUnavailable,
    ParentArtifactDigestMismatch,
    DerivedArtifactUnavailable,
    DerivedArtifactDigestMismatch,
    MatchedAcquisition(ArtifactAcquisitionReceiptV0),
    MatchedDerivation(ArtifactDerivationReceiptV0),
}

impl ArtifactProvenanceContextTraceV0 {
    /// Deterministic policy-defined audit bytes, not Magpie L0 encoding.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("artifact provenance traces are always serializable")
    }

    pub fn matched_acquisition(&self) -> Option<&ArtifactAcquisitionReceiptV0> {
        match self {
            Self::MatchedAcquisition(receipt) => Some(receipt),
            _ => None,
        }
    }

    pub fn matched_derivation(&self) -> Option<&ArtifactDerivationReceiptV0> {
        match self {
            Self::MatchedDerivation(receipt) => Some(receipt),
            _ => None,
        }
    }
}

impl StandingReplaySnapshot {
    /// Verify one acquisition bundle against explicit bytes and this replay.
    ///
    /// All byte inputs are already-bounded immutable resolution inputs. This
    /// method performs no ambient lookup and grants no standing effect.
    pub fn resolve_artifact_acquisition_context_v0(
        &self,
        selector: &ArtifactProvenanceAnchorSelectorV0,
        bundle_bytes: Option<&[u8]>,
        artifact_bytes: Option<&[u8]>,
    ) -> ArtifactProvenanceContextTraceV0 {
        let supplied = match bundle_bytes {
            Some(bytes) => bytes,
            None => return ArtifactProvenanceContextTraceV0::BundleUnavailable,
        };
        if supplied.len() > MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0 {
            return ArtifactProvenanceContextTraceV0::BundleTooLarge;
        }

        let input = match decode_bundle_text(supplied) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let envelope = match parse_acquisition_envelope(input) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let parsed = match validate_acquisition_shape(envelope) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        if let Err(trace) = validate_acquisition_semantics(&parsed) {
            return trace;
        }

        let canonical = encode_acquisition(&parsed);
        if canonical != supplied {
            return ArtifactProvenanceContextTraceV0::NonCanonicalEncoding;
        }
        if selector.bundle_kind() != ARTIFACT_ACQUISITION_BUNDLE_KIND_V0 {
            return ArtifactProvenanceContextTraceV0::BundleKindMismatch;
        }
        if selector.witness_algorithm() != ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0 {
            return ArtifactProvenanceContextTraceV0::WitnessAlgorithmMismatch;
        }
        if selector.canonicalization_profile() != ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0 {
            return ArtifactProvenanceContextTraceV0::CanonicalizationProfileMismatch;
        }
        if parsed.run_id != selector.run_id() {
            return ArtifactProvenanceContextTraceV0::RunIdMismatch;
        }

        let computed_witness_root = sha256_hex(&canonical);
        if computed_witness_root != selector.witness_root() {
            return ArtifactProvenanceContextTraceV0::WitnessRootMismatch;
        }
        let identity = selector_identity(selector);
        let occurrences = self.anchors().occurrences(&identity);
        if occurrences.is_empty() {
            return ArtifactProvenanceContextTraceV0::AnchorAbsent;
        }

        let artifact = match artifact_bytes {
            Some(bytes) => bytes,
            None => return ArtifactProvenanceContextTraceV0::ArtifactUnavailable,
        };
        let computed_artifact_sha256 = sha256_hex(artifact);
        if computed_artifact_sha256 != parsed.artifact.digest {
            return ArtifactProvenanceContextTraceV0::ArtifactDigestMismatch;
        }

        ArtifactProvenanceContextTraceV0::MatchedAcquisition(ArtifactAcquisitionReceiptV0 {
            verifier_profile: ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0.to_owned(),
            selector: selector.clone(),
            schema: parsed.schema,
            acquisition_id: parsed.acquisition_id,
            acquisition_profile: parsed.acquisition_profile,
            artifact_algorithm: parsed.artifact.algorithm,
            artifact_digest: parsed.artifact.digest,
            observed_locator: parsed.observed_locator,
            bundle_byte_len: supplied.len() as u64,
            computed_witness_root,
            artifact_byte_len: artifact.len() as u64,
            computed_artifact_sha256,
            occurrences: occurrences.to_vec(),
        })
    }

    /// Verify one direct-derivation bundle against explicit bytes and this replay.
    ///
    /// All byte inputs are already-bounded immutable resolution inputs. This
    /// method performs no ambient lookup and grants no standing effect.
    pub fn resolve_artifact_derivation_context_v0(
        &self,
        selector: &ArtifactProvenanceAnchorSelectorV0,
        bundle_bytes: Option<&[u8]>,
        parent_artifact_bytes: Option<&[u8]>,
        derived_artifact_bytes: Option<&[u8]>,
    ) -> ArtifactProvenanceContextTraceV0 {
        let supplied = match bundle_bytes {
            Some(bytes) => bytes,
            None => return ArtifactProvenanceContextTraceV0::BundleUnavailable,
        };
        if supplied.len() > MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0 {
            return ArtifactProvenanceContextTraceV0::BundleTooLarge;
        }

        let input = match decode_bundle_text(supplied) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let envelope = match parse_derivation_envelope(input) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        let parsed = match validate_derivation_shape(envelope) {
            Ok(value) => value,
            Err(trace) => return trace,
        };
        if let Err(trace) = validate_derivation_semantics(&parsed) {
            return trace;
        }
        if parsed.parent_artifact == parsed.derived_artifact {
            return ArtifactProvenanceContextTraceV0::ParentEqualsDerived;
        }

        let canonical = encode_derivation(&parsed);
        if canonical != supplied {
            return ArtifactProvenanceContextTraceV0::NonCanonicalEncoding;
        }
        if selector.bundle_kind() != ARTIFACT_DERIVATION_BUNDLE_KIND_V0 {
            return ArtifactProvenanceContextTraceV0::BundleKindMismatch;
        }
        if selector.witness_algorithm() != ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0 {
            return ArtifactProvenanceContextTraceV0::WitnessAlgorithmMismatch;
        }
        if selector.canonicalization_profile() != ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0 {
            return ArtifactProvenanceContextTraceV0::CanonicalizationProfileMismatch;
        }
        if parsed.run_id != selector.run_id() {
            return ArtifactProvenanceContextTraceV0::RunIdMismatch;
        }

        let computed_witness_root = sha256_hex(&canonical);
        if computed_witness_root != selector.witness_root() {
            return ArtifactProvenanceContextTraceV0::WitnessRootMismatch;
        }
        let identity = selector_identity(selector);
        let occurrences = self.anchors().occurrences(&identity);
        if occurrences.is_empty() {
            return ArtifactProvenanceContextTraceV0::AnchorAbsent;
        }

        let parent = match parent_artifact_bytes {
            Some(bytes) => bytes,
            None => return ArtifactProvenanceContextTraceV0::ParentArtifactUnavailable,
        };
        let derived = match derived_artifact_bytes {
            Some(bytes) => bytes,
            None => return ArtifactProvenanceContextTraceV0::DerivedArtifactUnavailable,
        };
        let computed_parent_artifact_sha256 = sha256_hex(parent);
        if computed_parent_artifact_sha256 != parsed.parent_artifact.digest {
            return ArtifactProvenanceContextTraceV0::ParentArtifactDigestMismatch;
        }
        let computed_derived_artifact_sha256 = sha256_hex(derived);
        if computed_derived_artifact_sha256 != parsed.derived_artifact.digest {
            return ArtifactProvenanceContextTraceV0::DerivedArtifactDigestMismatch;
        }

        ArtifactProvenanceContextTraceV0::MatchedDerivation(ArtifactDerivationReceiptV0 {
            verifier_profile: ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0.to_owned(),
            selector: selector.clone(),
            schema: parsed.schema,
            derivation_id: parsed.derivation_id,
            transformation_profile: parsed.transformation_profile,
            parent_artifact_algorithm: parsed.parent_artifact.algorithm,
            parent_artifact_digest: parsed.parent_artifact.digest,
            derived_artifact_algorithm: parsed.derived_artifact.algorithm,
            derived_artifact_digest: parsed.derived_artifact.digest,
            bundle_byte_len: supplied.len() as u64,
            computed_witness_root,
            parent_artifact_byte_len: parent.len() as u64,
            computed_parent_artifact_sha256,
            derived_artifact_byte_len: derived.len() as u64,
            computed_derived_artifact_sha256,
            occurrences: occurrences.to_vec(),
        })
    }
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn decode_bundle_text(input: &[u8]) -> Result<&str, ArtifactProvenanceContextTraceV0> {
    let text =
        std::str::from_utf8(input).map_err(|_| ArtifactProvenanceContextTraceV0::InvalidUtf8)?;
    if input.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(ArtifactProvenanceContextTraceV0::Utf8BomPresent);
    }
    Ok(text)
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
    fn trace(self) -> ArtifactProvenanceContextTraceV0 {
        match self {
            Self::Missing => ArtifactProvenanceContextTraceV0::MissingField,
            Self::WrongType => ArtifactProvenanceContextTraceV0::WrongJsonType,
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
                    let value = map.next_value::<DiscardedJson>()?;
                    duplicate |= value.duplicate;
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
                    let value = map.next_value::<DiscardedJson>()?;
                    duplicate |= value.duplicate;
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

#[derive(Default)]
struct ArtifactFields {
    flags: ObjectFlags,
    algorithm: StringSlot,
    digest: StringSlot,
    is_object: bool,
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
                            let value = map.next_value::<DiscardedJson>()?;
                            out.flags.duplicate |= value.duplicate;
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
struct ArtifactSlot {
    value: Option<ArtifactFields>,
    seen: bool,
}

impl ArtifactSlot {
    fn record(&mut self, value: ArtifactFields, flags: &mut ObjectFlags) {
        self.seen = true;
        flags.duplicate |= value.flags.duplicate;
        flags.unknown |= value.flags.unknown;
        self.value = Some(value);
    }

    fn required(self) -> Result<ArtifactFields, ShapeFailure> {
        if !self.seen {
            return Err(ShapeFailure::Missing);
        }
        let value = self.value.ok_or(ShapeFailure::WrongType)?;
        if !value.is_object {
            return Err(ShapeFailure::WrongType);
        }
        Ok(value)
    }
}

#[derive(Default)]
struct AcquisitionEnvelope {
    flags: ObjectFlags,
    schema: StringSlot,
    acquisition_id: StringSlot,
    acquisition_profile: StringSlot,
    artifact: ArtifactSlot,
    observed_locator: StringSlot,
    run_id: StringSlot,
    is_object: bool,
}

impl<'de> Deserialize<'de> for AcquisitionEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AcquisitionVisitor;

        impl<'de> Visitor<'de> for AcquisitionVisitor {
            type Value = AcquisitionEnvelope;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an acquisition bundle object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = AcquisitionEnvelope {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "schema" => out.schema.record(map.next_value()?, &mut out.flags),
                        "acquisition_id" => {
                            out.acquisition_id.record(map.next_value()?, &mut out.flags)
                        }
                        "acquisition_profile" => out
                            .acquisition_profile
                            .record(map.next_value()?, &mut out.flags),
                        "artifact" => out.artifact.record(map.next_value()?, &mut out.flags),
                        "observed_locator" => out
                            .observed_locator
                            .record(map.next_value()?, &mut out.flags),
                        "run_id" => out.run_id.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            let value = map.next_value::<DiscardedJson>()?;
                            out.flags.duplicate |= value.duplicate;
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
                while seq.next_element::<DiscardedJson>()?.is_some() {}
                Ok(Default::default())
            }
        }

        deserializer.deserialize_any(AcquisitionVisitor)
    }
}

#[derive(Default)]
struct DerivationEnvelope {
    flags: ObjectFlags,
    schema: StringSlot,
    derivation_id: StringSlot,
    transformation_profile: StringSlot,
    parent_artifact: ArtifactSlot,
    derived_artifact: ArtifactSlot,
    run_id: StringSlot,
    is_object: bool,
}

impl<'de> Deserialize<'de> for DerivationEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct DerivationVisitor;

        impl<'de> Visitor<'de> for DerivationVisitor {
            type Value = DerivationEnvelope;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a direct-derivation bundle object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = DerivationEnvelope {
                    is_object: true,
                    ..Default::default()
                };
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    out.flags.duplicate |= !keys.insert(key.clone());
                    match key.as_str() {
                        "schema" => out.schema.record(map.next_value()?, &mut out.flags),
                        "derivation_id" => {
                            out.derivation_id.record(map.next_value()?, &mut out.flags)
                        }
                        "transformation_profile" => out
                            .transformation_profile
                            .record(map.next_value()?, &mut out.flags),
                        "parent_artifact" => out
                            .parent_artifact
                            .record(map.next_value()?, &mut out.flags),
                        "derived_artifact" => out
                            .derived_artifact
                            .record(map.next_value()?, &mut out.flags),
                        "run_id" => out.run_id.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            let value = map.next_value::<DiscardedJson>()?;
                            out.flags.duplicate |= value.duplicate;
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
                while seq.next_element::<DiscardedJson>()?.is_some() {}
                Ok(Default::default())
            }
        }

        deserializer.deserialize_any(DerivationVisitor)
    }
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn parse_acquisition_envelope(
    input: &str,
) -> Result<AcquisitionEnvelope, ArtifactProvenanceContextTraceV0> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let envelope = AcquisitionEnvelope::deserialize(&mut deserializer)
        .map_err(|_| ArtifactProvenanceContextTraceV0::InvalidJson)?;
    deserializer
        .end()
        .map_err(|_| ArtifactProvenanceContextTraceV0::InvalidJson)?;
    if !envelope.is_object {
        return Err(ArtifactProvenanceContextTraceV0::TopLevelNotObject);
    }
    Ok(envelope)
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn parse_derivation_envelope(
    input: &str,
) -> Result<DerivationEnvelope, ArtifactProvenanceContextTraceV0> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let envelope = DerivationEnvelope::deserialize(&mut deserializer)
        .map_err(|_| ArtifactProvenanceContextTraceV0::InvalidJson)?;
    deserializer
        .end()
        .map_err(|_| ArtifactProvenanceContextTraceV0::InvalidJson)?;
    if !envelope.is_object {
        return Err(ArtifactProvenanceContextTraceV0::TopLevelNotObject);
    }
    Ok(envelope)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedArtifactIdentity {
    algorithm: String,
    digest: String,
}

struct ParsedAcquisition {
    schema: String,
    acquisition_id: String,
    acquisition_profile: String,
    artifact: ParsedArtifactIdentity,
    observed_locator: String,
    run_id: String,
}

struct ParsedDerivation {
    schema: String,
    derivation_id: String,
    transformation_profile: String,
    parent_artifact: ParsedArtifactIdentity,
    derived_artifact: ParsedArtifactIdentity,
    run_id: String,
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_artifact_shape(
    fields: ArtifactFields,
) -> Result<ParsedArtifactIdentity, ArtifactProvenanceContextTraceV0> {
    let algorithm = fields.algorithm.required().map_err(ShapeFailure::trace)?;
    let digest = fields.digest.required().map_err(ShapeFailure::trace)?;
    Ok(ParsedArtifactIdentity { algorithm, digest })
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_acquisition_shape(
    envelope: AcquisitionEnvelope,
) -> Result<ParsedAcquisition, ArtifactProvenanceContextTraceV0> {
    if envelope.flags.duplicate {
        return Err(ArtifactProvenanceContextTraceV0::DuplicateKey);
    }
    let schema = envelope.schema.required().map_err(ShapeFailure::trace)?;
    if schema != ARTIFACT_ACQUISITION_SCHEMA_V0 {
        return Err(ArtifactProvenanceContextTraceV0::SchemaMismatch);
    }
    let acquisition_id = envelope
        .acquisition_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let acquisition_profile = envelope
        .acquisition_profile
        .required()
        .map_err(ShapeFailure::trace)?;
    let artifact =
        validate_artifact_shape(envelope.artifact.required().map_err(ShapeFailure::trace)?)?;
    let observed_locator = envelope
        .observed_locator
        .required()
        .map_err(ShapeFailure::trace)?;
    let run_id = envelope.run_id.required().map_err(ShapeFailure::trace)?;
    if envelope.flags.unknown {
        return Err(ArtifactProvenanceContextTraceV0::UnknownField);
    }
    Ok(ParsedAcquisition {
        schema,
        acquisition_id,
        acquisition_profile,
        artifact,
        observed_locator,
        run_id,
    })
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_derivation_shape(
    envelope: DerivationEnvelope,
) -> Result<ParsedDerivation, ArtifactProvenanceContextTraceV0> {
    if envelope.flags.duplicate {
        return Err(ArtifactProvenanceContextTraceV0::DuplicateKey);
    }
    let schema = envelope.schema.required().map_err(ShapeFailure::trace)?;
    if schema != ARTIFACT_DERIVATION_SCHEMA_V0 {
        return Err(ArtifactProvenanceContextTraceV0::SchemaMismatch);
    }
    let derivation_id = envelope
        .derivation_id
        .required()
        .map_err(ShapeFailure::trace)?;
    let transformation_profile = envelope
        .transformation_profile
        .required()
        .map_err(ShapeFailure::trace)?;
    let parent_artifact = validate_artifact_shape(
        envelope
            .parent_artifact
            .required()
            .map_err(ShapeFailure::trace)?,
    )?;
    let derived_artifact = validate_artifact_shape(
        envelope
            .derived_artifact
            .required()
            .map_err(ShapeFailure::trace)?,
    )?;
    let run_id = envelope.run_id.required().map_err(ShapeFailure::trace)?;
    if envelope.flags.unknown {
        return Err(ArtifactProvenanceContextTraceV0::UnknownField);
    }
    Ok(ParsedDerivation {
        schema,
        derivation_id,
        transformation_profile,
        parent_artifact,
        derived_artifact,
        run_id,
    })
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_acquisition_semantics(
    parsed: &ParsedAcquisition,
) -> Result<(), ArtifactProvenanceContextTraceV0> {
    validate_identifier(&parsed.acquisition_id)?;
    validate_identifier(&parsed.acquisition_profile)?;
    validate_artifact_semantics(&parsed.artifact)?;
    validate_observed_locator(&parsed.observed_locator)?;
    validate_identifier(&parsed.run_id)?;
    Ok(())
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_derivation_semantics(
    parsed: &ParsedDerivation,
) -> Result<(), ArtifactProvenanceContextTraceV0> {
    validate_identifier(&parsed.derivation_id)?;
    validate_identifier(&parsed.transformation_profile)?;
    validate_artifact_semantics(&parsed.parent_artifact)?;
    validate_artifact_semantics(&parsed.derived_artifact)?;
    validate_identifier(&parsed.run_id)?;
    Ok(())
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_identifier(value: &str) -> Result<(), ArtifactProvenanceContextTraceV0> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !bytes[0].is_ascii_alphanumeric()
        || !bytes[1..].iter().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'/' | b'-')
        })
    {
        return Err(ArtifactProvenanceContextTraceV0::InvalidIdentifier);
    }
    Ok(())
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_artifact_semantics(
    artifact: &ParsedArtifactIdentity,
) -> Result<(), ArtifactProvenanceContextTraceV0> {
    if artifact.algorithm != ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0 {
        return Err(ArtifactProvenanceContextTraceV0::UnsupportedArtifactAlgorithm);
    }
    if !is_lowercase_hex_64(&artifact.digest) {
        return Err(ArtifactProvenanceContextTraceV0::InvalidArtifactDigest);
    }
    Ok(())
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn validate_observed_locator(value: &str) -> Result<(), ArtifactProvenanceContextTraceV0> {
    if value.is_empty()
        || value.len() > 4096
        || value
            .chars()
            .any(|character| character <= '\u{001f}' || character == '\u{007f}')
    {
        return Err(ArtifactProvenanceContextTraceV0::InvalidObservedLocator);
    }
    Ok(())
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn encode_acquisition(parsed: &ParsedAcquisition) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"{\"schema\":");
    push_json_string(&mut out, &parsed.schema);
    out.extend_from_slice(b",\"acquisition_id\":");
    push_json_string(&mut out, &parsed.acquisition_id);
    out.extend_from_slice(b",\"acquisition_profile\":");
    push_json_string(&mut out, &parsed.acquisition_profile);
    out.extend_from_slice(b",\"artifact\":{\"algorithm\":");
    push_json_string(&mut out, &parsed.artifact.algorithm);
    out.extend_from_slice(b",\"digest\":");
    push_json_string(&mut out, &parsed.artifact.digest);
    out.extend_from_slice(b"},\"observed_locator\":");
    push_json_string(&mut out, &parsed.observed_locator);
    out.extend_from_slice(b",\"run_id\":");
    push_json_string(&mut out, &parsed.run_id);
    out.push(b'}');
    out
}

fn encode_derivation(parsed: &ParsedDerivation) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"{\"schema\":");
    push_json_string(&mut out, &parsed.schema);
    out.extend_from_slice(b",\"derivation_id\":");
    push_json_string(&mut out, &parsed.derivation_id);
    out.extend_from_slice(b",\"transformation_profile\":");
    push_json_string(&mut out, &parsed.transformation_profile);
    out.extend_from_slice(b",\"parent_artifact\":{\"algorithm\":");
    push_json_string(&mut out, &parsed.parent_artifact.algorithm);
    out.extend_from_slice(b",\"digest\":");
    push_json_string(&mut out, &parsed.parent_artifact.digest);
    out.extend_from_slice(b"},\"derived_artifact\":{\"algorithm\":");
    push_json_string(&mut out, &parsed.derived_artifact.algorithm);
    out.extend_from_slice(b",\"digest\":");
    push_json_string(&mut out, &parsed.derived_artifact.digest);
    out.extend_from_slice(b"},\"run_id\":");
    push_json_string(&mut out, &parsed.run_id);
    out.push(b'}');
    out
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
