//! # magpie-core-v1 — the canonical byte encoding
//!
//! The single place where "the bytes that get hashed" are defined. The full
//! normative spec lives in `docs/FORMAT.md`; this file is its implementation.
//!
//! Design rules:
//! - **Injective by construction.** Fixed field order, big-endian integers,
//!   length-prefixed UTF-8 strings, one tag byte per enum variant. No escaping,
//!   no number formatting, no key sorting — none of JSON's ambiguity surface.
//! - **Domain-separated at both layers.** The preimage starts with a fixed
//!   magic (`magpie-core-v1`), so a content hash can never collide with a hash
//!   of non-Magpie material. Signatures are made over `magpie-sig-v1 || hash`,
//!   so they can never be replayed onto anything else signed by the same key.
//! - **Strings are hashed as their exact UTF-8 bytes.** No Unicode
//!   normalization is applied — two strings that render identically but differ
//!   in code points are different content, deliberately.
//! - **The store is not the preimage.** Records on disk stay JSON-lines for
//!   greppability; these bytes are computed, hashed, and discarded.
//!
//! Changing anything here is a **format break**: it requires a new profile
//! name, and a chain only ever carries one profile (declared in its genesis
//! event). That friction is the point.

use crate::event::{EventCore, Payload, Status};

/// The profile identifier this implementation encodes. Declared in every
/// chain's genesis event; verified on open.
pub const CANONICALIZATION_PROFILE: &str = "magpie-core-v1";

/// Fixed magic prefixed to every canonical core (hash-domain separation).
pub(crate) const CORE_MAGIC: &[u8] = b"magpie-core-v1";

/// Context prefixed to the content hash before signing (signature-domain
/// separation): the signed message is `SIG_DOMAIN || hash`.
pub(crate) const SIG_DOMAIN: &[u8] = b"magpie-sig-v1";

fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn put_u8(out: &mut Vec<u8>, v: u8) {
    out.push(v);
}

/// u64 big-endian byte length, then the UTF-8 bytes, exactly as held.
fn put_str(out: &mut Vec<u8>, s: &str) {
    put_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}

/// Status tags follow the spectrum's order — the encoding is meaningful.
fn status_tag(s: Status) -> u8 {
    match s {
        Status::Open => 0,
        Status::Conjectured => 1,
        Status::Supported => 2,
        Status::Settled => 3,
        Status::Refuted => 4,
    }
}

/// The canonical bytes of an [`EventCore`] under `magpie-core-v1`.
pub(crate) fn core_bytes(core: &EventCore) -> Vec<u8> {
    let mut out = Vec::with_capacity(160);
    out.extend_from_slice(CORE_MAGIC);
    put_u64(&mut out, core.seq);
    put_u64(&mut out, core.timestamp_nanos);
    out.extend_from_slice(core.prev_hash.as_bytes());
    put_str(&mut out, &core.provenance.agent);
    put_str(&mut out, &core.provenance.source);
    match &core.payload {
        Payload::Genesis { canonicalization_profile, verifying_key } => {
            put_u8(&mut out, 0);
            put_str(&mut out, canonicalization_profile);
            put_str(&mut out, verifying_key);
        }
        Payload::ClaimAsserted { claim_id, statement, status } => {
            put_u8(&mut out, 1);
            put_str(&mut out, claim_id);
            put_str(&mut out, statement);
            put_u8(&mut out, status_tag(*status));
        }
        Payload::EvidenceRecorded { claim_id, summary } => {
            put_u8(&mut out, 2);
            put_str(&mut out, claim_id);
            put_str(&mut out, summary);
        }
        Payload::ClaimStatusChanged { claim_id, from, to, reason } => {
            put_u8(&mut out, 3);
            put_str(&mut out, claim_id);
            put_u8(&mut out, status_tag(*from));
            put_u8(&mut out, status_tag(*to));
            put_str(&mut out, reason);
        }
        Payload::Note { text } => {
            put_u8(&mut out, 4);
            put_str(&mut out, text);
        }
    }
    out
}
