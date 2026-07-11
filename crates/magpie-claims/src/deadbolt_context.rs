use std::collections::BTreeMap;
use std::fmt;

use magpie_log::{Payload, Projection, SignedEvent};
use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::policy::{ClaimDomain, EvidenceKind};

/// Exact schema identifier for structured Deadbolt occurrence predicates and references.
pub const DEADBOLT_OCCURRENCE_SCHEMA: &str = "segment_anchored_v1";

/// The five opaque fields that identify one `SegmentAnchored` payload exactly.
///
/// Equality is byte-for-byte over the Rust strings. No field is trimmed,
/// normalized, case-folded, aliased, or reinterpreted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct DeadboltAnchorIdentity {
    pub bundle_kind: String,
    pub witness_root: String,
    pub witness_algorithm: String,
    pub canonicalization_profile: String,
    pub run_id: String,
}

/// One replayed occurrence of an exact Deadbolt anchor identity.
///
/// Sequence, event hash, and provenance are audit material. They do not
/// participate in identity matching and confer no authority by themselves.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DeadboltAnchorOccurrence {
    pub identity: DeadboltAnchorIdentity,
    pub sequence: u64,
    pub event_hash: String,
    pub provenance_agent: String,
    pub provenance_source: String,
}

/// Replay-derived index of exact `SegmentAnchored` occurrences.
///
/// `len` counts distinct five-field identities. Repeated identical anchors
/// remain available through `occurrences` in sequence order without becoming
/// distinct identities or stronger epistemic evidence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeadboltAnchorIndex {
    occurrences_by_identity: BTreeMap<DeadboltAnchorIdentity, Vec<DeadboltAnchorOccurrence>>,
}

impl DeadboltAnchorIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.occurrences_by_identity.len()
    }

    pub fn is_empty(&self) -> bool {
        self.occurrences_by_identity.is_empty()
    }

    pub fn occurrences(&self, identity: &DeadboltAnchorIdentity) -> &[DeadboltAnchorOccurrence] {
        self.occurrences_by_identity
            .get(identity)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn first_occurrence(
        &self,
        identity: &DeadboltAnchorIdentity,
    ) -> Option<&DeadboltAnchorOccurrence> {
        self.occurrences(identity).first()
    }

    /// Deterministic derived bytes for replay-equivalence checks.
    ///
    /// These bytes are not L0 canonical encoding and carry no standing.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        #[derive(Serialize)]
        struct Entry<'a> {
            identity: &'a DeadboltAnchorIdentity,
            occurrences: &'a [DeadboltAnchorOccurrence],
        }

        let entries: Vec<Entry<'_>> = self
            .occurrences_by_identity
            .iter()
            .map(|(identity, occurrences)| Entry {
                identity,
                occurrences,
            })
            .collect();
        serde_json::to_vec(&entries).expect("DeadboltAnchorIndex is always serializable")
    }
}

impl Projection for DeadboltAnchorIndex {
    fn apply(&mut self, event: &SignedEvent) {
        match &event.core.payload {
            Payload::SegmentAnchored {
                bundle_kind,
                witness_root,
                witness_algorithm,
                canonicalization_profile,
                run_id,
            } => {
                let identity = DeadboltAnchorIdentity {
                    bundle_kind: bundle_kind.clone(),
                    witness_root: witness_root.clone(),
                    witness_algorithm: witness_algorithm.clone(),
                    canonicalization_profile: canonicalization_profile.clone(),
                    run_id: run_id.clone(),
                };
                let occurrence = DeadboltAnchorOccurrence {
                    identity: identity.clone(),
                    sequence: event.core.seq,
                    event_hash: event.hash.to_hex(),
                    provenance_agent: event.core.provenance.agent.clone(),
                    provenance_source: event.core.provenance.source.clone(),
                };
                let occurrences = self.occurrences_by_identity.entry(identity).or_default();
                let insertion_index = occurrences
                    .partition_point(|existing| existing.sequence <= occurrence.sequence);
                occurrences.insert(insertion_index, occurrence);
            }
            Payload::Genesis { .. }
            | Payload::ClaimAsserted { .. }
            | Payload::EvidenceRecorded { .. }
            | Payload::ClaimStatusChanged { .. }
            | Payload::Note { .. }
            | Payload::ClaimAssertedV2 { .. }
            | Payload::EvidenceRegistered { .. }
            | Payload::JustificationEdgeRecorded { .. } => {}
        }
    }
}

/// Closed outcome of exact Deadbolt occurrence-context resolution.
///
/// `Matched` means only that the structured claim predicate and typed evidence
/// reference identify an exact anchor retained from verified Magpie replay.
/// It is not admission, aggregation, achieved standing, or a foreign bundle
/// interpretation verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeadboltOccurrenceContextResolution {
    Matched(DeadboltAnchorOccurrence),
    WrongEvidenceKind,
    WrongClaimDomain,
    MissingClaimPredicate,
    MalformedClaimPredicate,
    MissingEvidenceReference,
    MalformedEvidenceReference,
    PredicateReferenceMismatch,
    AnchorNotFound,
}

/// Resolve exact Deadbolt occurrence context from closed policy inputs.
///
/// No actor class, prose, caller-provided verifier flag, ambient state, or
/// foreign Deadbolt runtime participates in this pure match.
pub fn resolve_deadbolt_occurrence_context(
    evidence_kind: EvidenceKind,
    claim_domain: ClaimDomain,
    claim_metadata_json: &str,
    evidence_metadata_json: &str,
    anchors: &DeadboltAnchorIndex,
) -> DeadboltOccurrenceContextResolution {
    if evidence_kind != EvidenceKind::DeadboltAnchor {
        return DeadboltOccurrenceContextResolution::WrongEvidenceKind;
    }
    if claim_domain != ClaimDomain::OccurrenceInclusion {
        return DeadboltOccurrenceContextResolution::WrongClaimDomain;
    }

    let claim_metadata: ClaimOccurrenceMetadata = match serde_json::from_str(claim_metadata_json) {
        Ok(metadata) => metadata,
        Err(_) => return DeadboltOccurrenceContextResolution::MalformedClaimPredicate,
    };
    let claim_identity = match claim_metadata.deadbolt_occurrence {
        Some(predicate) => match predicate.into_identity() {
            Some(identity) => identity,
            None => return DeadboltOccurrenceContextResolution::MalformedClaimPredicate,
        },
        None => return DeadboltOccurrenceContextResolution::MissingClaimPredicate,
    };

    let evidence_metadata: EvidenceAnchorMetadata =
        match serde_json::from_str(evidence_metadata_json) {
            Ok(metadata) => metadata,
            Err(_) => return DeadboltOccurrenceContextResolution::MalformedEvidenceReference,
        };
    let evidence_identity = match evidence_metadata.deadbolt_anchor_ref {
        Some(reference) => match reference.into_identity() {
            Some(identity) => identity,
            None => return DeadboltOccurrenceContextResolution::MalformedEvidenceReference,
        },
        None => return DeadboltOccurrenceContextResolution::MissingEvidenceReference,
    };

    if claim_identity != evidence_identity {
        return DeadboltOccurrenceContextResolution::PredicateReferenceMismatch;
    }

    match anchors.first_occurrence(&claim_identity) {
        Some(occurrence) => DeadboltOccurrenceContextResolution::Matched(occurrence.clone()),
        None => DeadboltOccurrenceContextResolution::AnchorNotFound,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnchorIdentityMetadata {
    schema: String,
    bundle_kind: String,
    witness_root: String,
    witness_algorithm: String,
    canonicalization_profile: String,
    run_id: String,
}

impl AnchorIdentityMetadata {
    fn into_identity(self) -> Option<DeadboltAnchorIdentity> {
        if self.schema != DEADBOLT_OCCURRENCE_SCHEMA
            || self.bundle_kind.is_empty()
            || !is_lowercase_hex_64(&self.witness_root)
            || self.witness_algorithm.is_empty()
            || self.canonicalization_profile.is_empty()
            || self.run_id.is_empty()
        {
            return None;
        }

        Some(DeadboltAnchorIdentity {
            bundle_kind: self.bundle_kind,
            witness_root: self.witness_root,
            witness_algorithm: self.witness_algorithm,
            canonicalization_profile: self.canonicalization_profile,
            run_id: self.run_id,
        })
    }
}

struct ClaimOccurrenceMetadata {
    deadbolt_occurrence: Option<AnchorIdentityMetadata>,
}

impl<'de> Deserialize<'de> for ClaimOccurrenceMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClaimMetadataVisitor;

        impl<'de> Visitor<'de> for ClaimMetadataVisitor {
            type Value = ClaimOccurrenceMetadata;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON object with at most one deadbolt_occurrence")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut deadbolt_occurrence = None;
                let mut seen = false;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "deadbolt_occurrence" {
                        if seen {
                            return Err(serde::de::Error::duplicate_field("deadbolt_occurrence"));
                        }
                        seen = true;
                        deadbolt_occurrence = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(ClaimOccurrenceMetadata {
                    deadbolt_occurrence,
                })
            }
        }

        deserializer.deserialize_map(ClaimMetadataVisitor)
    }
}

struct EvidenceAnchorMetadata {
    deadbolt_anchor_ref: Option<AnchorIdentityMetadata>,
}

impl<'de> Deserialize<'de> for EvidenceAnchorMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EvidenceMetadataVisitor;

        impl<'de> Visitor<'de> for EvidenceMetadataVisitor {
            type Value = EvidenceAnchorMetadata;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a JSON object with at most one deadbolt_anchor_ref")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut deadbolt_anchor_ref = None;
                let mut seen = false;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "deadbolt_anchor_ref" {
                        if seen {
                            return Err(serde::de::Error::duplicate_field("deadbolt_anchor_ref"));
                        }
                        seen = true;
                        deadbolt_anchor_ref = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                Ok(EvidenceAnchorMetadata {
                    deadbolt_anchor_ref,
                })
            }
        }

        deserializer.deserialize_map(EvidenceMetadataVisitor)
    }
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
