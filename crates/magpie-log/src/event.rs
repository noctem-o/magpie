use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::hashing::ContentHash;

/// Who/what produced an event. Provenance is first-class — it is signed and
/// chained along with the payload, never bolted on afterwards.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub agent: String,
    pub source: String,
}

impl Provenance {
    pub fn new(agent: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            agent: agent.into(),
            source: source.into(),
        }
    }
}

/// The epistemic-status spectrum. Evidence moves a claim along this axis; every
/// transition is its own logged event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Open,
    Conjectured,
    Supported,
    Settled,
    Refuted,
}

/// The content of an event. Heterogeneous by design. The first real inhabitants
/// are epistemic claims (generalized from the manuscript's
/// proven/conjectured/open/refuted tracking), plus a generic `Note` escape hatch
/// so the log is not boxed in early.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Payload {
    /// The chain's self-description, always and only at seq 0. Pins the
    /// canonicalization profile and the (hex) verifying key, so a chain
    /// carries a record of *how to verify it* — while trust in the key itself
    /// still comes from outside.
    Genesis {
        canonicalization_profile: String,
        verifying_key: String,
    },
    ClaimAsserted {
        claim_id: String,
        statement: String,
        status: Status,
    },
    EvidenceRecorded {
        claim_id: String,
        summary: String,
    },
    ClaimStatusChanged {
        claim_id: String,
        from: Status,
        to: Status,
        reason: String,
    },
    Note {
        text: String,
    },
    /// An anchor: commits an externally sealed evidence segment (a deadbolt
    /// witness/evidence bundle) into the chain by its root. The chain
    /// guarantees order, signature, and inclusion; the segment's *contents*
    /// verify locally against `witness_root` using the sealing subsystem's
    /// own verifier. Anything not anchored is not part of the record.
    ///
    /// `canonicalization_profile` names the FOREIGN profile that produced the
    /// root (e.g. `phase5-interim-jcs-like-v1`) — deliberately not assumed to
    /// be magpie-core-v1. Kind-agnostic on purpose: a new deadbolt bundle
    /// kind must never require a Magpie format change.
    SegmentAnchored {
        bundle_kind: String,
        witness_root: String,
        witness_algorithm: String,
        canonicalization_profile: String,
        run_id: String,
    },
    /// ADR-0002 typed claim assertion. Registers a scoped claim node; it does
    /// not settle truth by itself. Standing remains a derived projection.
    ClaimAssertedV2 {
        claim_id: String,
        statement: String,
        scope_ref: String,
        actor_class: String,
        content_hash: String,
        metadata_json: String,
    },
    /// ADR-0002 typed evidence registration. Evidence is inert until the
    /// standing projection connects it through explicit justification edges.
    EvidenceRegistered {
        evidence_id: String,
        evidence_kind: String,
        summary: String,
        scope_ref: String,
        actor_class: String,
        content_hash: String,
        metadata_json: String,
    },
    /// ADR-0002 typed justification edge. Edge semantics are replayed by
    /// projections; the log only records the closed vocabulary and fields.
    JustificationEdgeRecorded {
        edge_id: String,
        edge_kind: String,
        source_id: String,
        target_id: String,
        scope_ref: String,
        actor_class: String,
        rationale: String,
        metadata_json: String,
    },
}

impl Payload {
    pub(crate) fn validate(&self) -> Result<(), &'static str> {
        match self {
            Payload::Genesis { .. }
            | Payload::ClaimAsserted { .. }
            | Payload::EvidenceRecorded { .. }
            | Payload::ClaimStatusChanged { .. }
            | Payload::Note { .. } => Ok(()),
            Payload::SegmentAnchored { witness_root, .. } => {
                if is_lowercase_hex_64(witness_root) {
                    Ok(())
                } else {
                    Err("payload.witness_root must be 64 lowercase hex chars")
                }
            }
            Payload::ClaimAssertedV2 {
                claim_id,
                statement,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json: _,
            } => {
                require_non_empty(claim_id, "payload.claim_id must be non-empty")?;
                require_non_empty(statement, "payload.statement must be non-empty")?;
                require_non_empty(scope_ref, "payload.scope_ref must be non-empty")?;
                if !is_actor_class(actor_class) {
                    return Err("payload.actor_class must be a known ADR-0002 actor class");
                }
                if !is_empty_or_lowercase_hex_64(content_hash) {
                    return Err("payload.content_hash must be empty or 64 lowercase hex chars");
                }
                Ok(())
            }
            Payload::EvidenceRegistered {
                evidence_id,
                evidence_kind,
                summary,
                scope_ref,
                actor_class,
                content_hash,
                metadata_json: _,
            } => {
                require_non_empty(evidence_id, "payload.evidence_id must be non-empty")?;
                require_non_empty(summary, "payload.summary must be non-empty")?;
                require_non_empty(scope_ref, "payload.scope_ref must be non-empty")?;
                if !is_evidence_kind(evidence_kind) {
                    return Err("payload.evidence_kind must be a known ADR-0002 evidence kind");
                }
                if !is_actor_class(actor_class) {
                    return Err("payload.actor_class must be a known ADR-0002 actor class");
                }
                if !is_empty_or_lowercase_hex_64(content_hash) {
                    return Err("payload.content_hash must be empty or 64 lowercase hex chars");
                }
                Ok(())
            }
            Payload::JustificationEdgeRecorded {
                edge_id,
                edge_kind,
                source_id,
                target_id,
                scope_ref,
                actor_class,
                rationale,
                metadata_json: _,
            } => {
                require_non_empty(edge_id, "payload.edge_id must be non-empty")?;
                require_non_empty(source_id, "payload.source_id must be non-empty")?;
                require_non_empty(target_id, "payload.target_id must be non-empty")?;
                require_non_empty(scope_ref, "payload.scope_ref must be non-empty")?;
                require_non_empty(rationale, "payload.rationale must be non-empty")?;
                if !is_edge_kind(edge_kind) {
                    return Err("payload.edge_kind must be a known ADR-0002 edge kind");
                }
                if !is_actor_class(actor_class) {
                    return Err("payload.actor_class must be a known ADR-0002 actor class");
                }
                Ok(())
            }
        }
    }
}

fn require_non_empty(value: &str, error: &'static str) -> Result<(), &'static str> {
    if is_non_empty(value) {
        Ok(())
    } else {
        Err(error)
    }
}

fn is_non_empty(value: &str) -> bool {
    !value.is_empty()
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_empty_or_lowercase_hex_64(value: &str) -> bool {
    value.is_empty() || is_lowercase_hex_64(value)
}

fn is_actor_class(value: &str) -> bool {
    matches!(
        value,
        "HumanRoot"
            | "AgentProposer"
            | "AutomatedVerifier"
            | "DeadboltAnchorer"
            | "LensWitness"
            | "SourceImporter"
    )
}

fn is_evidence_kind(value: &str) -> bool {
    matches!(
        value,
        "DeterministicVerification"
            | "HumanRatification"
            | "DeadboltAnchor"
            | "ExecutionEvidence"
            | "BehavioralEvaluation"
            | "ExternalSource"
            | "ModelSelfReport"
            | "LensReadout"
    )
}

fn is_edge_kind(value: &str) -> bool {
    matches!(
        value,
        "supports" | "derived_from" | "contradicts" | "supersedes" | "invalidates" | "ratifies"
    )
}

/// Everything that is hashed and signed: position, time, the chain link, who
/// produced it, and what it says.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCore {
    pub seq: u64,
    pub timestamp_nanos: u64,
    pub prev_hash: ContentHash,
    pub provenance: Provenance,
    pub payload: Payload,
}

impl EventCore {
    /// The single seam for canonical encoding.
    ///
    /// Encodes under the `magpie-core-v1` profile (see `canonical.rs` and
    /// `docs/FORMAT.md`): fixed field order, big-endian integers,
    /// length-prefixed UTF-8, tagged enums, magic-prefixed. Injective by
    /// construction and reimplementable from the spec alone. The invariant —
    /// "hash over the canonical bytes of the core" — lives here; changing the
    /// encoding means a new profile name and a new chain.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        crate::canonical::core_bytes(self)
    }

    pub fn hash(&self) -> ContentHash {
        ContentHash::of(&self.canonical_bytes())
    }
}

/// An Ed25519 signature, carried as 64 bytes and rendered as hex in the log.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sig([u8; 64]);

impl Sig {
    pub fn new(bytes: [u8; 64]) -> Self {
        Sig(bytes)
    }
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Debug for Sig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sig({}…)", &self.to_hex()[..8])
    }
}

impl Serialize for Sig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Sig {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom("signature must be 64 bytes"));
        }
        let mut out = [0u8; 64];
        out.copy_from_slice(&bytes);
        Ok(Sig(out))
    }
}

/// A stored log record: the signed/chained core, its chain hash, and the
/// signature over that hash.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedEvent {
    pub core: EventCore,
    pub hash: ContentHash,
    pub signature: Sig,
}
