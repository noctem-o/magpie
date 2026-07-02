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
        Self { agent: agent.into(), source: source.into() }
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
    ClaimAsserted { claim_id: String, statement: String, status: Status },
    EvidenceRecorded { claim_id: String, summary: String },
    ClaimStatusChanged { claim_id: String, from: Status, to: Status, reason: String },
    Note { text: String },
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
    /// The single place canonical encoding lives.
    ///
    /// NOTE — first thing to harden: JSON is not a canonical-bytes format by
    /// spec. `serde_json` field order is stable in practice (struct fields in
    /// declaration order; no `HashMap`s or floats in this type), which is enough
    /// for a single-codebase chain. Before trusting the chain across machines or
    /// crate versions, swap this for a true canonical codec (RFC 8785 JCS, or a
    /// fixed length-prefixed binary format). The invariant — "hash over the
    /// canonical bytes of the core" — stays; only this function changes.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("EventCore is always serializable")
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
