//! Standing-inert deterministic verifier context derived from one verified replay snapshot.
//!
//! A serializable or cloned receipt-shaped value is audit material. No API in
//! this module accepts it as authority. Trusted origin is established only by
//! the snapshot-only construction path.
//!
//! External callers cannot construct a receipt through a struct literal:
//!
//! ```compile_fail
//! use magpie_claims::DeterministicVerifierReceiptV0;
//! let _receipt = DeterministicVerifierReceiptV0 {
//!     predicate_schema: String::new(),
//! };
//! ```
//!
//! A standalone `StandingView` deliberately has no trusted context method:
//!
//! ```compile_fail
//! use magpie_claims::StandingView;
//! let view = StandingView::new();
//! let _ = view.resolve_deterministic_verifier_context_v0("c", "e", "edge");
//! ```
//!
//! There is no free raw-material resolver; construction is snapshot-only:
//!
//! ```compile_fail
//! let _ = magpie_claims::resolve_deterministic_verifier_context_v0(
//!     "raw claim metadata", "raw evidence metadata"
//! );
//! ```

use std::fmt;

use magpie_log::Status;
use serde::{
    de::{IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use sha2::{Digest, Sha256};

use crate::policy::{
    support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
    SupportContextRequirement,
};
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::standing::StandingTraceReason;

pub const MACHINE_PREDICATE_SCHEMA_V0: &str = "magpie-machine-predicate-v0";
pub const VERIFICATION_WITNESS_SCHEMA_V0: &str = "magpie-verification-witness-v0";
pub const SHA256_BYTES_EQUALS_PREDICATE_V0: &str = "sha256_bytes_equals_v0";
pub const SHA256_BYTES_EQUALS_STATEMENT_PREFIX_V0: &str = "sha256_bytes_equals_v0:";
pub const MAX_WITNESS_BYTES_V0: usize = 4096;

/// Successful, replay-derived audit material for the closed v0 checker.
///
/// Fields are private and there is no public constructor or `Deserialize`
/// implementation. Cloning or serializing a receipt does not confer authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DeterministicVerifierReceiptV0 {
    predicate_schema: String,
    predicate_id: String,
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    scope_ref: String,
    canonical_statement: String,
    claim_content_hash: String,
    expected_sha256: String,
    computed_sha256: String,
    witness_len: u64,
}

impl DeterministicVerifierReceiptV0 {
    pub fn predicate_schema(&self) -> &str {
        &self.predicate_schema
    }
    pub fn predicate_id(&self) -> &str {
        &self.predicate_id
    }
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }
    pub fn evidence_id(&self) -> &str {
        &self.evidence_id
    }
    pub fn edge_id(&self) -> &str {
        &self.edge_id
    }
    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }
    pub fn canonical_statement(&self) -> &str {
        &self.canonical_statement
    }
    pub fn claim_content_hash(&self) -> &str {
        &self.claim_content_hash
    }
    pub fn expected_sha256(&self) -> &str {
        &self.expected_sha256
    }
    pub fn computed_sha256(&self) -> &str {
        &self.computed_sha256
    }
    pub fn witness_len(&self) -> u64 {
        self.witness_len
    }
}

/// Closed, deterministic audit outcome. It is not standing or admission.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "outcome", content = "receipt", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)] // Public shape is contractually Matched(receipt), not boxed.
pub enum DeterministicVerifierContextTraceV0 {
    Matched(DeterministicVerifierReceiptV0),
    MissingMachinePredicate,
    MalformedMachinePredicate,
    DuplicatePredicateKey,
    UnknownPredicateKey,
    UnknownPredicateSchema,
    UnknownPredicateId,
    InvalidExpectedDigest,
    StatementPredicateMismatch,
    MissingClaimContentHash,
    ClaimContentHashMismatch,
    MissingVerificationWitness,
    MalformedVerificationWitness,
    DuplicateWitnessKey,
    UnknownWitnessKey,
    UnknownWitnessSchema,
    ClaimBindingMismatch,
    ScopeBindingMismatch,
    PredicateBindingMismatch,
    InvalidWitnessHex,
    WitnessTooLarge,
    DigestMismatch,
}

impl DeterministicVerifierContextTraceV0 {
    /// Policy-defined derived audit bytes for deterministic comparison.
    /// These are not L0 canonical encoding.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("deterministic verifier traces are always serializable")
    }

    pub fn matched_receipt(&self) -> Option<&DeterministicVerifierReceiptV0> {
        match self {
            Self::Matched(receipt) => Some(receipt),
            _ => None,
        }
    }
}

impl StandingReplaySnapshot {
    /// Derive standing-inert verifier context from this one verified replay.
    ///
    /// `None` means the exact v0 lane was not selected or replayed structural
    /// revalidation disagreed. `Some(failure)` means an eligible attempt failed.
    /// `Some(Matched)` is audit material only and is not achieved standing.
    pub fn resolve_deterministic_verifier_context_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
    ) -> Option<DeterministicVerifierContextTraceV0> {
        let baseline = self.standing().resolved_standing_with_trace(claim_id);
        let exact_reasons = [
            StandingTraceReason::AcceptedCandidate,
            StandingTraceReason::RequiresVerifierContext,
            StandingTraceReason::CeilingIsCandidateOnly,
        ];
        let mut candidates = baseline.trace.iter().filter(|entry| {
            entry.edge_id.as_deref() == Some(edge_id)
                && entry.source_id.as_deref() == Some(evidence_id)
                && entry.target_id == claim_id
                && entry.evidence_kind.as_deref()
                    == Some(EvidenceKind::DeterministicVerification.as_str())
                && entry.claim_domain.as_deref()
                    == Some(ClaimDomain::ExactMachineCheckable.as_str())
                && entry.candidate_ceiling == Some(Status::Settled)
                && entry.reasons == exact_reasons
        });
        candidates.next()?;
        if candidates.next().is_some()
            || support_ceiling(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
            ) != Some(Status::Settled)
            || support_context_requirement(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
            ) != SupportContextRequirement::DeterministicVerifierContext
        {
            return None;
        }

        let standing = self.standing();
        let claim = standing.get(claim_id)?;
        let typed_claim = standing.typed_claim(claim_id)?;
        let evidence = standing.typed_evidence(evidence_id)?;
        let edge = standing.justification_edge(edge_id)?;
        if edge.edge_kind != "supports"
            || edge.source_id != evidence_id
            || edge.target_id != claim_id
            || evidence.evidence_kind != EvidenceKind::DeterministicVerification.as_str()
            || evidence.scope_ref != edge.scope_ref
            || edge.scope_ref != typed_claim.scope_ref
        {
            return None;
        }

        let predicate = match parse_predicate(&typed_claim.metadata_json) {
            Ok(value) => value,
            Err(error) => return Some(error),
        };
        let canonical_statement = format!(
            "{SHA256_BYTES_EQUALS_STATEMENT_PREFIX_V0}{}",
            predicate.expected_sha256
        );
        if claim.statement != typed_claim.statement || typed_claim.statement != canonical_statement
        {
            return Some(DeterministicVerifierContextTraceV0::StatementPredicateMismatch);
        }
        if typed_claim.content_hash.is_empty() {
            return Some(DeterministicVerifierContextTraceV0::MissingClaimContentHash);
        }
        let computed_claim_hash = hex::encode(Sha256::digest(typed_claim.statement.as_bytes()));
        if typed_claim.content_hash != computed_claim_hash {
            return Some(DeterministicVerifierContextTraceV0::ClaimContentHashMismatch);
        }

        let witness = match parse_witness(&evidence.metadata_json) {
            Ok(value) => value,
            Err(error) => return Some(error),
        };
        if witness.schema != VERIFICATION_WITNESS_SCHEMA_V0 {
            return Some(DeterministicVerifierContextTraceV0::UnknownWitnessSchema);
        }
        if witness.predicate_id != predicate.predicate_id {
            return Some(DeterministicVerifierContextTraceV0::PredicateBindingMismatch);
        }
        if witness.subject_claim_id != claim_id {
            return Some(DeterministicVerifierContextTraceV0::ClaimBindingMismatch);
        }
        if witness.scope_ref != typed_claim.scope_ref {
            return Some(DeterministicVerifierContextTraceV0::ScopeBindingMismatch);
        }
        if witness.witness_hex.len() > MAX_WITNESS_BYTES_V0 * 2 {
            return Some(DeterministicVerifierContextTraceV0::WitnessTooLarge);
        }
        if witness.witness_hex.len() % 2 != 0
            || !witness
                .witness_hex
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Some(DeterministicVerifierContextTraceV0::InvalidWitnessHex);
        }
        let decoded = match hex::decode(&witness.witness_hex) {
            Ok(value) => value,
            Err(_) => return Some(DeterministicVerifierContextTraceV0::InvalidWitnessHex),
        };
        if decoded.len() > MAX_WITNESS_BYTES_V0 {
            return Some(DeterministicVerifierContextTraceV0::WitnessTooLarge);
        }
        let computed_digest_bytes = Sha256::digest(&decoded);
        let expected_digest_bytes = hex::decode(&predicate.expected_sha256)
            .expect("strict expected digest has already been validated");
        if computed_digest_bytes.as_slice() != expected_digest_bytes.as_slice() {
            return Some(DeterministicVerifierContextTraceV0::DigestMismatch);
        }
        let computed_sha256 = hex::encode(computed_digest_bytes);
        Some(DeterministicVerifierContextTraceV0::Matched(
            DeterministicVerifierReceiptV0 {
                predicate_schema: predicate.schema,
                predicate_id: predicate.predicate_id,
                claim_id: claim_id.to_owned(),
                evidence_id: evidence_id.to_owned(),
                edge_id: edge_id.to_owned(),
                scope_ref: typed_claim.scope_ref.clone(),
                canonical_statement,
                claim_content_hash: typed_claim.content_hash.clone(),
                expected_sha256: predicate.expected_sha256,
                computed_sha256,
                witness_len: decoded.len() as u64,
            },
        ))
    }
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
        if self.seen {
            flags.duplicate = true;
        }
        self.seen = true;
        match value {
            LooseString::String(value) => self.value = Some(value),
            LooseString::Other => self.wrong_type = true,
        }
    }
    fn take(self) -> Option<String> {
        if self.wrong_type {
            None
        } else {
            self.value
        }
    }
}

enum LooseString {
    String(String),
    Other,
}

impl<'de> Deserialize<'de> for LooseString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LooseStringVisitor;
        impl<'de> Visitor<'de> for LooseStringVisitor {
            type Value = LooseString;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("any JSON value")
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
                Ok(LooseString::String(value.into()))
            }
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
                Ok(LooseString::String(value))
            }
            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(LooseString::Other)
            }
            fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                let _ = IgnoredAny::deserialize(d)?;
                Ok(LooseString::Other)
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                while seq.next_element::<IgnoredAny>()?.is_some() {}
                Ok(LooseString::Other)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
                Ok(LooseString::Other)
            }
        }
        deserializer.deserialize_any(LooseStringVisitor)
    }
}

#[derive(Default)]
struct PredicateFields {
    flags: ObjectFlags,
    schema: StringSlot,
    predicate_id: StringSlot,
    expected_sha256: StringSlot,
    is_object: bool,
}

impl<'de> Deserialize<'de> for PredicateFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PredicateFields;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a machine predicate object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = PredicateFields {
                    is_object: true,
                    ..Default::default()
                };
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "schema" => out.schema.record(map.next_value()?, &mut out.flags),
                        "predicate_id" => {
                            out.predicate_id.record(map.next_value()?, &mut out.flags)
                        }
                        "expected_sha256" => out
                            .expected_sha256
                            .record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            map.next_value::<IgnoredAny>()?;
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
                while seq.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Default::default())
            }
        }
        deserializer.deserialize_any(V)
    }
}

#[derive(Default)]
struct PredicateEnvelope {
    flags: ObjectFlags,
    claim_domain: StringSlot,
    predicate: Option<PredicateFields>,
    predicate_seen: bool,
    predicate_wrong_type: bool,
    is_object: bool,
}

impl<'de> Deserialize<'de> for PredicateEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PredicateEnvelope;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a claim metadata object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = PredicateEnvelope {
                    is_object: true,
                    ..Default::default()
                };
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "claim_domain" => {
                            out.claim_domain.record(map.next_value()?, &mut out.flags)
                        }
                        "machine_predicate" => {
                            if out.predicate_seen {
                                out.flags.duplicate = true;
                            }
                            out.predicate_seen = true;
                            let value: PredicateFields = map.next_value()?;
                            out.flags.duplicate |= value.flags.duplicate;
                            out.flags.unknown |= value.flags.unknown;
                            out.predicate_wrong_type |= !value.is_object;
                            out.predicate = Some(value);
                        }
                        _ => {
                            out.flags.unknown = true;
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(out)
            }
        }
        deserializer.deserialize_map(V)
    }
}

struct MachinePredicate {
    schema: String,
    predicate_id: String,
    expected_sha256: String,
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn parse_predicate(input: &str) -> Result<MachinePredicate, DeterministicVerifierContextTraceV0> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let envelope = PredicateEnvelope::deserialize(&mut deserializer)
        .map_err(|_| DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    deserializer
        .end()
        .map_err(|_| DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    if !envelope.is_object {
        return Err(DeterministicVerifierContextTraceV0::MalformedMachinePredicate);
    }
    if envelope.flags.duplicate {
        return Err(DeterministicVerifierContextTraceV0::DuplicatePredicateKey);
    }
    if envelope.flags.unknown {
        return Err(DeterministicVerifierContextTraceV0::UnknownPredicateKey);
    }
    if !envelope.predicate_seen {
        return Err(DeterministicVerifierContextTraceV0::MissingMachinePredicate);
    }
    if envelope.predicate_wrong_type {
        return Err(DeterministicVerifierContextTraceV0::MalformedMachinePredicate);
    }
    let domain = envelope
        .claim_domain
        .take()
        .ok_or(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    let predicate = envelope
        .predicate
        .ok_or(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    let schema = predicate
        .schema
        .take()
        .ok_or(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    let predicate_id = predicate
        .predicate_id
        .take()
        .ok_or(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    let expected_sha256 = predicate
        .expected_sha256
        .take()
        .ok_or(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)?;
    if domain != ClaimDomain::ExactMachineCheckable.as_str() {
        return Err(DeterministicVerifierContextTraceV0::MalformedMachinePredicate);
    }
    if schema != MACHINE_PREDICATE_SCHEMA_V0 {
        return Err(DeterministicVerifierContextTraceV0::UnknownPredicateSchema);
    }
    if predicate_id != SHA256_BYTES_EQUALS_PREDICATE_V0 {
        return Err(DeterministicVerifierContextTraceV0::UnknownPredicateId);
    }
    if !is_lowercase_hex_64(&expected_sha256) {
        return Err(DeterministicVerifierContextTraceV0::InvalidExpectedDigest);
    }
    Ok(MachinePredicate {
        schema,
        predicate_id,
        expected_sha256,
    })
}

#[derive(Default)]
struct WitnessFields {
    flags: ObjectFlags,
    schema: StringSlot,
    predicate_id: StringSlot,
    subject_claim_id: StringSlot,
    scope_ref: StringSlot,
    witness_hex: StringSlot,
    is_object: bool,
}

impl<'de> Deserialize<'de> for WitnessFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = WitnessFields;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a verification witness object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = WitnessFields {
                    is_object: true,
                    ..Default::default()
                };
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "schema" => out.schema.record(map.next_value()?, &mut out.flags),
                        "predicate_id" => {
                            out.predicate_id.record(map.next_value()?, &mut out.flags)
                        }
                        "subject_claim_id" => out
                            .subject_claim_id
                            .record(map.next_value()?, &mut out.flags),
                        "scope_ref" => out.scope_ref.record(map.next_value()?, &mut out.flags),
                        "witness_hex" => out.witness_hex.record(map.next_value()?, &mut out.flags),
                        _ => {
                            out.flags.unknown = true;
                            map.next_value::<IgnoredAny>()?;
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
                while seq.next_element::<IgnoredAny>()?.is_some() {}
                Ok(Default::default())
            }
        }
        deserializer.deserialize_any(V)
    }
}

#[derive(Default)]
struct WitnessEnvelope {
    flags: ObjectFlags,
    witness: Option<WitnessFields>,
    witness_seen: bool,
    witness_wrong_type: bool,
    is_object: bool,
}

impl<'de> Deserialize<'de> for WitnessEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = WitnessEnvelope;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an evidence metadata object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut out = WitnessEnvelope {
                    is_object: true,
                    ..Default::default()
                };
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "verification_witness" => {
                            if out.witness_seen {
                                out.flags.duplicate = true;
                            }
                            out.witness_seen = true;
                            let value: WitnessFields = map.next_value()?;
                            out.flags.duplicate |= value.flags.duplicate;
                            out.flags.unknown |= value.flags.unknown;
                            out.witness_wrong_type |= !value.is_object;
                            out.witness = Some(value);
                        }
                        _ => {
                            out.flags.unknown = true;
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(out)
            }
        }
        deserializer.deserialize_map(V)
    }
}

struct VerificationWitness {
    schema: String,
    predicate_id: String,
    subject_claim_id: String,
    scope_ref: String,
    witness_hex: String,
}

#[allow(clippy::result_large_err)] // Closed private errors reuse the exact public failure vocabulary.
fn parse_witness(input: &str) -> Result<VerificationWitness, DeterministicVerifierContextTraceV0> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let envelope = WitnessEnvelope::deserialize(&mut deserializer)
        .map_err(|_| DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?;
    deserializer
        .end()
        .map_err(|_| DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?;
    if !envelope.is_object {
        return Err(DeterministicVerifierContextTraceV0::MalformedVerificationWitness);
    }
    if envelope.flags.duplicate {
        return Err(DeterministicVerifierContextTraceV0::DuplicateWitnessKey);
    }
    if envelope.flags.unknown {
        return Err(DeterministicVerifierContextTraceV0::UnknownWitnessKey);
    }
    if !envelope.witness_seen {
        return Err(DeterministicVerifierContextTraceV0::MissingVerificationWitness);
    }
    if envelope.witness_wrong_type {
        return Err(DeterministicVerifierContextTraceV0::MalformedVerificationWitness);
    }
    let witness = envelope
        .witness
        .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?;
    Ok(VerificationWitness {
        schema: witness
            .schema
            .take()
            .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?,
        predicate_id: witness
            .predicate_id
            .take()
            .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?,
        subject_claim_id: witness
            .subject_claim_id
            .take()
            .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?,
        scope_ref: witness
            .scope_ref
            .take()
            .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?,
        witness_hex: witness
            .witness_hex
            .take()
            .ok_or(DeterministicVerifierContextTraceV0::MalformedVerificationWitness)?,
    })
}

fn is_lowercase_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn duplicate_outer_predicate_key_is_observable_before_value_selection() {
        let metadata = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{}},"machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{DIGEST}"}}}}"#
        );
        assert_eq!(
            parse_predicate(&metadata).err(),
            Some(DeterministicVerifierContextTraceV0::DuplicatePredicateKey)
        );
    }

    #[test]
    fn claim_metadata_trailing_json_is_malformed_for_the_strict_parser() {
        let metadata = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{DIGEST}"}}}} null"#
        );
        assert_eq!(
            parse_predicate(&metadata).err(),
            Some(DeterministicVerifierContextTraceV0::MalformedMachinePredicate)
        );
    }
}
