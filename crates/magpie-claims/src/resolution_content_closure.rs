//! Standing-inert construction of one immutable resolution content closure.
//!
//! Closure identities cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::ResolutionContentClosureIdentityV0;
//! let _identity = ResolutionContentClosureIdentityV0 {
//!     schema: String::new(),
//! };
//! ```
//!
//! Closures cannot be constructed with struct literals:
//!
//! ```compile_fail
//! use magpie_claims::ResolutionContentClosureV0;
//! let _closure = ResolutionContentClosureV0 {};
//! ```
//!
//! Deserializing a closure identity is not supported:
//!
//! ```compile_fail
//! use magpie_claims::ResolutionContentClosureIdentityV0;
//! let _: ResolutionContentClosureIdentityV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! Deserializing a closure is not supported:
//!
//! ```compile_fail
//! use magpie_claims::ResolutionContentClosureV0;
//! let _: ResolutionContentClosureV0 = serde_json::from_str("{}").unwrap();
//! ```
//!
//! No constructor accepts only a caller-created identity:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionContentClosureIdentityV0, ResolutionContentClosureV0,
//! };
//! fn substitute(identity: &ResolutionContentClosureIdentityV0) {
//!     let _ = ResolutionContentClosureV0::construct_from_identity(identity);
//! }
//! ```
//!
//! Constructed closures expose no mutable insertion method:
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
//! };
//! let mut closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
//! let key = ResolutionArtifactObjectKeyV0::new("sha256", "digest");
//! closure.insert_artifact(key, b"payload".to_vec());
//! ```

use std::collections::BTreeMap;
use std::fmt;

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::artifact_provenance_verifier::ArtifactProvenanceAnchorSelectorV0;

pub const RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0: &str = "magpie-resolution-content-closure-v0";

pub const RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-resolution-content-closure-json-v0";

pub const RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0: &str = "sha256";

pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0: usize = 1_024;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0: usize = 1_024;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0: usize = 2_048;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0: usize = 1_024;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0: usize = 67_108_864;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0: usize = 1_048_576;

pub const MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0: u64 = 536_870_912;

/// Untrusted exact lookup key for one artifact object.
///
/// Construction is intentionally infallible and performs no algorithm or
/// digest validation. The key does not prove that supplied bytes match it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ResolutionArtifactObjectKeyV0 {
    algorithm: String,
    digest: String,
}

impl ResolutionArtifactObjectKeyV0 {
    pub fn new(algorithm: impl Into<String>, digest: impl Into<String>) -> Self {
        Self {
            algorithm: algorithm.into(),
            digest: digest.into(),
        }
    }

    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// One borrowed artifact key/byte occurrence supplied to construction.
pub struct ResolutionArtifactObjectInputV0<'a> {
    key: ResolutionArtifactObjectKeyV0,
    bytes: &'a [u8],
}

impl<'a> ResolutionArtifactObjectInputV0<'a> {
    pub fn new(key: ResolutionArtifactObjectKeyV0, bytes: &'a [u8]) -> Self {
        Self { key, bytes }
    }

    pub fn key(&self) -> &ResolutionArtifactObjectKeyV0 {
        &self.key
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }
}

impl fmt::Debug for ResolutionArtifactObjectInputV0<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResolutionArtifactObjectInputV0")
            .field("key", &self.key)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

/// One borrowed foreign-bundle selector/byte occurrence supplied to construction.
pub struct ResolutionForeignBundleObjectInputV0<'a> {
    selector: ArtifactProvenanceAnchorSelectorV0,
    bytes: &'a [u8],
}

impl<'a> ResolutionForeignBundleObjectInputV0<'a> {
    pub fn new(selector: ArtifactProvenanceAnchorSelectorV0, bytes: &'a [u8]) -> Self {
        Self { selector, bytes }
    }

    pub fn selector(&self) -> &ArtifactProvenanceAnchorSelectorV0 {
        &self.selector
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }
}

impl fmt::Debug for ResolutionForeignBundleObjectInputV0<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResolutionForeignBundleObjectInputV0")
            .field("selector", &self.selector)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

/// Fixed key-field category and aggregate failure precedence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionContentClosureKeyFieldV0 {
    ArtifactAlgorithm,
    ArtifactDigest,
    BundleKind,
    WitnessRoot,
    WitnessAlgorithm,
    CanonicalizationProfile,
    RunId,
}

/// Closed deterministic construction failure for resolution content closure v0.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
pub enum ResolutionContentClosureConstructionErrorV0 {
    TotalEntryLimitExceeded {
        supplied_entries: u64,
        maximum_entries: u64,
    },
    ArtifactEntryLimitExceeded {
        supplied_artifact_entries: u64,
        maximum_entries: u64,
    },
    ForeignBundleEntryLimitExceeded {
        supplied_foreign_bundle_entries: u64,
        maximum_entries: u64,
    },
    KeyFieldTooLarge {
        field: ResolutionContentClosureKeyFieldV0,
        offending_entries: u64,
        maximum_observed_bytes: u64,
        maximum_bytes: u64,
    },
    ArtifactObjectTooLarge {
        key: ResolutionArtifactObjectKeyV0,
        maximum_observed_bytes_for_key: u64,
        maximum_bytes: u64,
    },
    ForeignBundleObjectTooLarge {
        selector: ArtifactProvenanceAnchorSelectorV0,
        maximum_observed_bytes_for_key: u64,
        maximum_bytes: u64,
    },
    TotalSuppliedBytesExceeded {
        supplied_object_bytes: u64,
        maximum_bytes: u64,
    },
    ConflictingArtifactObject {
        key: ResolutionArtifactObjectKeyV0,
    },
    ConflictingForeignBundleObject {
        selector: ArtifactProvenanceAnchorSelectorV0,
    },
}

/// Exact four-field identity of one successfully constructed closure.
///
/// The identity has no public constructor and no Deserialize implementation.
/// Serialized identity-shaped material is audit output only.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ResolutionContentClosureIdentityV0 {
    schema: String,
    canonicalization_profile: String,
    digest_algorithm: String,
    manifest_sha256: String,
}

impl ResolutionContentClosureIdentityV0 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn canonicalization_profile(&self) -> &str {
        &self.canonicalization_profile
    }

    pub fn digest_algorithm(&self) -> &str {
        &self.digest_algorithm
    }

    pub fn manifest_sha256(&self) -> &str {
        &self.manifest_sha256
    }
}

/// Immutable, standing-inert availability closure over two exact key spaces.
pub struct ResolutionContentClosureV0 {
    artifact_objects: BTreeMap<ResolutionArtifactObjectKeyV0, Vec<u8>>,
    foreign_bundle_objects: BTreeMap<ArtifactProvenanceAnchorSelectorV0, Vec<u8>>,
    canonical_manifest_bytes: Vec<u8>,
    identity: ResolutionContentClosureIdentityV0,
    supplied_object_bytes: u64,
    retained_object_bytes: u64,
}

impl ResolutionContentClosureV0 {
    /// Construct one immutable closure from explicit borrowed key/byte inputs.
    ///
    /// The complete deterministic first-failure precedence is implemented
    /// before any object bytes are copied into the returned closure.
    #[allow(clippy::manual_saturating_arithmetic)] // Keep the impossible checked-add branch explicit.
    #[allow(clippy::result_large_err)] // The exact public enum retains complete typed audit keys.
    pub fn construct<'a>(
        artifact_entries: &[ResolutionArtifactObjectInputV0<'a>],
        foreign_bundle_entries: &[ResolutionForeignBundleObjectInputV0<'a>],
    ) -> Result<Self, ResolutionContentClosureConstructionErrorV0> {
        let artifact_entry_count = usize_to_u64_saturated(artifact_entries.len());
        let foreign_bundle_entry_count = usize_to_u64_saturated(foreign_bundle_entries.len());
        let total_entry_count = artifact_entry_count
            .checked_add(foreign_bundle_entry_count)
            .unwrap_or(u64::MAX);

        if total_entry_count
            > usize_to_u64_saturated(MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0)
        {
            return Err(
                ResolutionContentClosureConstructionErrorV0::TotalEntryLimitExceeded {
                    supplied_entries: total_entry_count,
                    maximum_entries: usize_to_u64_saturated(
                        MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0,
                    ),
                },
            );
        }

        if artifact_entry_count
            > usize_to_u64_saturated(MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0)
        {
            return Err(
                ResolutionContentClosureConstructionErrorV0::ArtifactEntryLimitExceeded {
                    supplied_artifact_entries: artifact_entry_count,
                    maximum_entries: usize_to_u64_saturated(
                        MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0,
                    ),
                },
            );
        }

        if foreign_bundle_entry_count
            > usize_to_u64_saturated(MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0)
        {
            return Err(
                ResolutionContentClosureConstructionErrorV0::ForeignBundleEntryLimitExceeded {
                    supplied_foreign_bundle_entries: foreign_bundle_entry_count,
                    maximum_entries: usize_to_u64_saturated(
                        MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0,
                    ),
                },
            );
        }

        if let Some(error) = first_key_field_failure(artifact_entries, foreign_bundle_entries) {
            return Err(error);
        }
        if let Some(error) = first_artifact_object_size_failure(artifact_entries) {
            return Err(error);
        }
        if let Some(error) = first_foreign_bundle_object_size_failure(foreign_bundle_entries) {
            return Err(error);
        }

        let supplied_object_bytes =
            checked_supplied_object_bytes(artifact_entries, foreign_bundle_entries)?;

        if let Some(key) = first_conflicting_artifact_key(artifact_entries) {
            return Err(
                ResolutionContentClosureConstructionErrorV0::ConflictingArtifactObject { key },
            );
        }
        if let Some(selector) = first_conflicting_foreign_bundle_selector(foreign_bundle_entries) {
            return Err(
                ResolutionContentClosureConstructionErrorV0::ConflictingForeignBundleObject {
                    selector,
                },
            );
        }

        let mut artifact_objects = BTreeMap::new();
        for input in artifact_entries {
            artifact_objects
                .entry(input.key().clone())
                .or_insert_with(|| input.bytes().to_vec());
        }

        let mut foreign_bundle_objects = BTreeMap::new();
        for input in foreign_bundle_entries {
            foreign_bundle_objects
                .entry(input.selector().clone())
                .or_insert_with(|| input.bytes().to_vec());
        }

        let retained_object_bytes =
            checked_retained_object_bytes(&artifact_objects, &foreign_bundle_objects)?;
        if retained_object_bytes > supplied_object_bytes
            || supplied_object_bytes > MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0
        {
            return Err(
                ResolutionContentClosureConstructionErrorV0::TotalSuppliedBytesExceeded {
                    supplied_object_bytes,
                    maximum_bytes: MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0,
                },
            );
        }

        let canonical_manifest_bytes = encode_manifest(&artifact_objects, &foreign_bundle_objects);
        let identity = ResolutionContentClosureIdentityV0 {
            schema: RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0.to_owned(),
            canonicalization_profile: RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0
                .to_owned(),
            digest_algorithm: RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0.to_owned(),
            manifest_sha256: sha256_hex(&canonical_manifest_bytes),
        };

        Ok(Self {
            artifact_objects,
            foreign_bundle_objects,
            canonical_manifest_bytes,
            identity,
            supplied_object_bytes,
            retained_object_bytes,
        })
    }

    pub fn identity(&self) -> &ResolutionContentClosureIdentityV0 {
        &self.identity
    }

    pub fn canonical_manifest_bytes(&self) -> &[u8] {
        &self.canonical_manifest_bytes
    }

    pub fn artifact_count(&self) -> usize {
        self.artifact_objects.len()
    }

    pub fn foreign_bundle_count(&self) -> usize {
        self.foreign_bundle_objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.artifact_objects.is_empty() && self.foreign_bundle_objects.is_empty()
    }

    pub fn supplied_object_bytes(&self) -> u64 {
        self.supplied_object_bytes
    }

    pub fn retained_object_bytes(&self) -> u64 {
        self.retained_object_bytes
    }

    pub fn artifact(&self, key: &ResolutionArtifactObjectKeyV0) -> Option<&[u8]> {
        self.artifact_objects.get(key).map(Vec::as_slice)
    }

    pub fn foreign_bundle(&self, selector: &ArtifactProvenanceAnchorSelectorV0) -> Option<&[u8]> {
        self.foreign_bundle_objects.get(selector).map(Vec::as_slice)
    }
}

impl fmt::Debug for ResolutionContentClosureV0 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResolutionContentClosureV0")
            .field("identity", &self.identity)
            .field("artifact_count", &self.artifact_objects.len())
            .field("foreign_bundle_count", &self.foreign_bundle_objects.len())
            .field("supplied_object_bytes", &self.supplied_object_bytes)
            .field("retained_object_bytes", &self.retained_object_bytes)
            .finish()
    }
}

fn usize_to_u64_saturated(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn first_key_field_failure(
    artifact_entries: &[ResolutionArtifactObjectInputV0<'_>],
    foreign_bundle_entries: &[ResolutionForeignBundleObjectInputV0<'_>],
) -> Option<ResolutionContentClosureConstructionErrorV0> {
    oversized_key_field(
        ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm,
        artifact_entries.iter().map(|input| input.key().algorithm()),
    )
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::ArtifactDigest,
            artifact_entries.iter().map(|input| input.key().digest()),
        )
    })
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::BundleKind,
            foreign_bundle_entries
                .iter()
                .map(|input| input.selector().bundle_kind()),
        )
    })
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::WitnessRoot,
            foreign_bundle_entries
                .iter()
                .map(|input| input.selector().witness_root()),
        )
    })
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::WitnessAlgorithm,
            foreign_bundle_entries
                .iter()
                .map(|input| input.selector().witness_algorithm()),
        )
    })
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::CanonicalizationProfile,
            foreign_bundle_entries
                .iter()
                .map(|input| input.selector().canonicalization_profile()),
        )
    })
    .or_else(|| {
        oversized_key_field(
            ResolutionContentClosureKeyFieldV0::RunId,
            foreign_bundle_entries
                .iter()
                .map(|input| input.selector().run_id()),
        )
    })
}

fn oversized_key_field<'a>(
    field: ResolutionContentClosureKeyFieldV0,
    values: impl Iterator<Item = &'a str>,
) -> Option<ResolutionContentClosureConstructionErrorV0> {
    let mut offending_entries = 0_u64;
    let mut maximum_observed_bytes = 0_u64;
    for value in values {
        if value.len() > MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0 {
            offending_entries = offending_entries.saturating_add(1);
            maximum_observed_bytes =
                maximum_observed_bytes.max(usize_to_u64_saturated(value.len()));
        }
    }

    (offending_entries > 0).then_some(
        ResolutionContentClosureConstructionErrorV0::KeyFieldTooLarge {
            field,
            offending_entries,
            maximum_observed_bytes,
            maximum_bytes: usize_to_u64_saturated(
                MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0,
            ),
        },
    )
}

fn first_artifact_object_size_failure(
    entries: &[ResolutionArtifactObjectInputV0<'_>],
) -> Option<ResolutionContentClosureConstructionErrorV0> {
    let mut selected: Option<(&ResolutionArtifactObjectKeyV0, u64)> = None;

    for input in entries {
        if input.byte_len() <= MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0 {
            continue;
        }
        let length = usize_to_u64_saturated(input.byte_len());
        match selected {
            None => selected = Some((input.key(), length)),
            Some((key, _)) if input.key() < key => selected = Some((input.key(), length)),
            Some((key, maximum)) if input.key() == key => {
                selected = Some((key, maximum.max(length)));
            }
            Some(_) => {}
        }
    }

    selected.map(|(key, maximum_observed_bytes_for_key)| {
        ResolutionContentClosureConstructionErrorV0::ArtifactObjectTooLarge {
            key: key.clone(),
            maximum_observed_bytes_for_key,
            maximum_bytes: usize_to_u64_saturated(
                MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0,
            ),
        }
    })
}

fn first_foreign_bundle_object_size_failure(
    entries: &[ResolutionForeignBundleObjectInputV0<'_>],
) -> Option<ResolutionContentClosureConstructionErrorV0> {
    let mut selected: Option<(&ArtifactProvenanceAnchorSelectorV0, u64)> = None;

    for input in entries {
        if input.byte_len() <= MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0 {
            continue;
        }
        let length = usize_to_u64_saturated(input.byte_len());
        match selected {
            None => selected = Some((input.selector(), length)),
            Some((selector, _)) if input.selector() < selector => {
                selected = Some((input.selector(), length));
            }
            Some((selector, maximum)) if input.selector() == selector => {
                selected = Some((selector, maximum.max(length)));
            }
            Some(_) => {}
        }
    }

    selected.map(|(selector, maximum_observed_bytes_for_key)| {
        ResolutionContentClosureConstructionErrorV0::ForeignBundleObjectTooLarge {
            selector: selector.clone(),
            maximum_observed_bytes_for_key,
            maximum_bytes: usize_to_u64_saturated(
                MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0,
            ),
        }
    })
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn checked_supplied_object_bytes(
    artifact_entries: &[ResolutionArtifactObjectInputV0<'_>],
    foreign_bundle_entries: &[ResolutionForeignBundleObjectInputV0<'_>],
) -> Result<u64, ResolutionContentClosureConstructionErrorV0> {
    let mut total = 0_u64;
    for length in artifact_entries
        .iter()
        .map(ResolutionArtifactObjectInputV0::byte_len)
        .chain(
            foreign_bundle_entries
                .iter()
                .map(ResolutionForeignBundleObjectInputV0::byte_len),
        )
    {
        let length = match u64::try_from(length) {
            Ok(value) => value,
            Err(_) => return Err(total_supplied_bytes_exceeded(u64::MAX)),
        };
        total = match total.checked_add(length) {
            Some(value) => value,
            None => return Err(total_supplied_bytes_exceeded(u64::MAX)),
        };
    }

    if total > MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0 {
        return Err(total_supplied_bytes_exceeded(total));
    }
    Ok(total)
}

fn total_supplied_bytes_exceeded(
    supplied_object_bytes: u64,
) -> ResolutionContentClosureConstructionErrorV0 {
    ResolutionContentClosureConstructionErrorV0::TotalSuppliedBytesExceeded {
        supplied_object_bytes,
        maximum_bytes: MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0,
    }
}

fn first_conflicting_artifact_key(
    entries: &[ResolutionArtifactObjectInputV0<'_>],
) -> Option<ResolutionArtifactObjectKeyV0> {
    let mut seen: BTreeMap<&ResolutionArtifactObjectKeyV0, &[u8]> = BTreeMap::new();
    let mut conflicting: Option<&ResolutionArtifactObjectKeyV0> = None;

    for input in entries {
        match seen.get(input.key()) {
            Some(existing) if *existing != input.bytes() => {
                if conflicting.is_none_or(|key| input.key() < key) {
                    conflicting = Some(input.key());
                }
            }
            Some(_) => {}
            None => {
                seen.insert(input.key(), input.bytes());
            }
        }
    }

    conflicting.cloned()
}

fn first_conflicting_foreign_bundle_selector(
    entries: &[ResolutionForeignBundleObjectInputV0<'_>],
) -> Option<ArtifactProvenanceAnchorSelectorV0> {
    let mut seen: BTreeMap<&ArtifactProvenanceAnchorSelectorV0, &[u8]> = BTreeMap::new();
    let mut conflicting: Option<&ArtifactProvenanceAnchorSelectorV0> = None;

    for input in entries {
        match seen.get(input.selector()) {
            Some(existing) if *existing != input.bytes() => {
                if conflicting.is_none_or(|selector| input.selector() < selector) {
                    conflicting = Some(input.selector());
                }
            }
            Some(_) => {}
            None => {
                seen.insert(input.selector(), input.bytes());
            }
        }
    }

    conflicting.cloned()
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn checked_retained_object_bytes(
    artifact_objects: &BTreeMap<ResolutionArtifactObjectKeyV0, Vec<u8>>,
    foreign_bundle_objects: &BTreeMap<ArtifactProvenanceAnchorSelectorV0, Vec<u8>>,
) -> Result<u64, ResolutionContentClosureConstructionErrorV0> {
    let mut total = 0_u64;
    for bytes in artifact_objects
        .values()
        .chain(foreign_bundle_objects.values())
    {
        let length = match u64::try_from(bytes.len()) {
            Ok(value) => value,
            Err(_) => return Err(total_supplied_bytes_exceeded(u64::MAX)),
        };
        total = match total.checked_add(length) {
            Some(value) => value,
            None => return Err(total_supplied_bytes_exceeded(u64::MAX)),
        };
    }
    Ok(total)
}

fn encode_manifest(
    artifact_objects: &BTreeMap<ResolutionArtifactObjectKeyV0, Vec<u8>>,
    foreign_bundle_objects: &BTreeMap<ArtifactProvenanceAnchorSelectorV0, Vec<u8>>,
) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(b"{\"schema\":");
    push_json_string(&mut output, RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0);
    output.extend_from_slice(b",\"artifact_objects\":[");

    for (index, (key, bytes)) in artifact_objects.iter().enumerate() {
        if index > 0 {
            output.push(b',');
        }
        output.extend_from_slice(b"{\"key\":{\"algorithm\":");
        push_json_string(&mut output, key.algorithm());
        output.extend_from_slice(b",\"digest\":");
        push_json_string(&mut output, key.digest());
        output.extend_from_slice(b"},\"byte_length\":");
        push_u64(&mut output, usize_to_u64_saturated(bytes.len()));
        output.extend_from_slice(b",\"content_sha256\":");
        push_json_string(&mut output, &sha256_hex(bytes));
        output.push(b'}');
    }

    output.extend_from_slice(b"],\"foreign_bundle_objects\":[");
    for (index, (selector, bytes)) in foreign_bundle_objects.iter().enumerate() {
        if index > 0 {
            output.push(b',');
        }
        output.extend_from_slice(b"{\"key\":{\"bundle_kind\":");
        push_json_string(&mut output, selector.bundle_kind());
        output.extend_from_slice(b",\"witness_root\":");
        push_json_string(&mut output, selector.witness_root());
        output.extend_from_slice(b",\"witness_algorithm\":");
        push_json_string(&mut output, selector.witness_algorithm());
        output.extend_from_slice(b",\"canonicalization_profile\":");
        push_json_string(&mut output, selector.canonicalization_profile());
        output.extend_from_slice(b",\"run_id\":");
        push_json_string(&mut output, selector.run_id());
        output.extend_from_slice(b"},\"byte_length\":");
        push_u64(&mut output, usize_to_u64_saturated(bytes.len()));
        output.extend_from_slice(b",\"content_sha256\":");
        push_json_string(&mut output, &sha256_hex(bytes));
        output.push(b'}');
    }
    output.extend_from_slice(b"]}");
    output
}

fn push_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(value.to_string().as_bytes());
}

fn push_json_string(output: &mut Vec<u8>, value: &str) {
    const LOWER_HEX: &[u8; 16] = b"0123456789abcdef";

    output.push(b'"');
    for character in value.chars() {
        match character {
            '"' => output.extend_from_slice(b"\\\""),
            '\\' => output.extend_from_slice(b"\\\\"),
            '\u{0008}' => output.extend_from_slice(b"\\b"),
            '\u{0009}' => output.extend_from_slice(b"\\t"),
            '\u{000a}' => output.extend_from_slice(b"\\n"),
            '\u{000c}' => output.extend_from_slice(b"\\f"),
            '\u{000d}' => output.extend_from_slice(b"\\r"),
            '\u{0000}'..='\u{001f}' => {
                let byte = character as u8;
                output.extend_from_slice(b"\\u00");
                output.push(LOWER_HEX[(byte >> 4) as usize]);
                output.push(LOWER_HEX[(byte & 0x0f) as usize]);
            }
            _ => {
                let mut encoded = [0; 4];
                output.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
            }
        }
    }
    output.push(b'"');
}

fn sha256_hex(input: &[u8]) -> String {
    hex::encode(Sha256::digest(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_identity_equality_uses_all_four_fields() {
        let identity = ResolutionContentClosureIdentityV0 {
            schema: RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0.to_owned(),
            canonicalization_profile: RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0
                .to_owned(),
            digest_algorithm: RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0.to_owned(),
            manifest_sha256: "a".repeat(64),
        };

        let mut changed_schema = identity.clone();
        changed_schema.schema = "other-schema".to_owned();
        let mut changed_profile = identity.clone();
        changed_profile.canonicalization_profile = "other-profile".to_owned();
        let mut changed_algorithm = identity.clone();
        changed_algorithm.digest_algorithm = "other-algorithm".to_owned();

        assert_ne!(identity, changed_schema);
        assert_ne!(identity, changed_profile);
        assert_ne!(identity, changed_algorithm);
    }
}
