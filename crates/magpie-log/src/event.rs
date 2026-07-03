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
}

impl Payload {
    pub(crate) fn validate(&self) -> Result<(), &'static str> {
        match self {
            Payload::SegmentAnchored { witness_root, .. } => {
                if is_lowercase_hex_64(witness_root) {
                    Ok(())
                } else {
                    Err("payload.witness_root must be 64 lowercase hex chars")
                }
            }
            _ => Ok(()),
        }
    }
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
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
