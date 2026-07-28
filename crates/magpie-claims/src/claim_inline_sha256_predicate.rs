//! Claim-owned inline-byte SHA-256 predicate evaluation.
//!
//! The authority-bearing outcome and receipt are evaluator-created audit
//! values. Callers may inspect and serialize values they obtained, but cannot
//! construct, deserialize, reclassify, or feed them back into the evaluator.
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateOutcomeV0;
//!
//! let _ = Sha256ClaimInlineBytesPredicateOutcomeV0 {};
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateReceiptV0;
//!
//! let _ = Sha256ClaimInlineBytesPredicateReceiptV0 {
//!     predicate_schema: String::new(),
//!     predicate_id: String::new(),
//!     claim_id: String::new(),
//!     scope_ref: String::new(),
//!     canonical_statement: String::new(),
//!     claim_content_hash: String::new(),
//!     expected_sha256: String::new(),
//!     computed_sha256: String::new(),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateOutcomeV0;
//!
//! let _: Sha256ClaimInlineBytesPredicateOutcomeV0 =
//!     serde_json::from_str(r#"{"outcome":"resolution_failed"}"#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateReceiptV0;
//!
//! let _: Sha256ClaimInlineBytesPredicateReceiptV0 =
//!     serde_json::from_str("{}").unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     Sha256ClaimInlineBytesPredicateOutcomeKindV0,
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//! };
//!
//! let _: Sha256ClaimInlineBytesPredicateOutcomeV0 =
//!     Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual.into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateOutcomeV0;
//!
//! let _ = Sha256ClaimInlineBytesPredicateOutcomeV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateReceiptV0;
//!
//! let _ = Sha256ClaimInlineBytesPredicateReceiptV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlineSubjectResolutionFailureV0,
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//! };
//!
//! let _ = Sha256ClaimInlineBytesPredicateOutcomeV0::try_from(
//!     ClaimInlineSubjectResolutionFailureV0::MissingClaim,
//! );
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//!     Sha256ClaimInlineBytesPredicateReceiptV0,
//! };
//!
//! fn cannot_reclassify(
//!     receipt: Sha256ClaimInlineBytesPredicateReceiptV0,
//! ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
//!     Sha256ClaimInlineBytesPredicateOutcomeV0::DigestUnequal(receipt)
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     ClaimInlineSubjectResolutionFailureV0,
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//! };
//!
//! fn cannot_construct_failure(
//!     claim_id: String,
//! ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
//!     Sha256ClaimInlineBytesPredicateOutcomeV0::ResolutionFailed {
//!         requested_claim_id: claim_id,
//!         failure: ClaimInlineSubjectResolutionFailureV0::MissingClaim,
//!     }
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateOutcomeV0;
//!
//! fn cannot_replace_requested_id(outcome: &mut Sha256ClaimInlineBytesPredicateOutcomeV0) {
//!     outcome.requested_claim_id = "replacement".into();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     Sha256ClaimInlineBytesPredicateReceiptV0,
//!     Sha256ClaimInlineBytesPredicateOutcomeV0,
//! };
//!
//! fn no_receipt_conversion(
//!     receipt: Sha256ClaimInlineBytesPredicateReceiptV0,
//! ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
//!     receipt.into()
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     Sha256ClaimInlineBytesPredicateOutcomeV0, StandingReplaySnapshot,
//! };
//!
//! fn cannot_reinject(
//!     snapshot: &StandingReplaySnapshot,
//!     outcome: &Sha256ClaimInlineBytesPredicateOutcomeV0,
//! ) {
//!     let _ = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0("claim", outcome);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//!
//! fn serialized_bytes_are_not_authority(
//!     snapshot: &StandingReplaySnapshot,
//!     bytes: &[u8],
//! ) {
//!     let _ = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0("claim", bytes);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//!
//! fn snapshots_cannot_be_mixed(
//!     selected: &StandingReplaySnapshot,
//!     other: &StandingReplaySnapshot,
//! ) {
//!     let _ = selected.evaluate_sha256_claim_inline_bytes_equals_v0(
//!         "claim",
//!         other.standing(),
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateOutcomeV0;
//!
//! fn opaque_outcome_has_no_debug(outcome: &Sha256ClaimInlineBytesPredicateOutcomeV0) {
//!     let _ = format!("{outcome:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::Sha256ClaimInlineBytesPredicateReceiptV0;
//!
//! fn opaque_receipt_has_no_debug(receipt: &Sha256ClaimInlineBytesPredicateReceiptV0) {
//!     let _ = format!("{receipt:?}");
//! }
//! ```

use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::policy::ClaimDomain;
use crate::replay_snapshot::StandingReplaySnapshot;

const INLINE_PREDICATE_SCHEMA_V0: &str = "magpie-machine-predicate-inline-bytes-v0";
const SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0: &str = "sha256_claim_inline_bytes_equals_v0";
const OUTCOME_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-claim-inline-sha256-predicate-outcome-json-v0";
const MAX_SUBJECT_HEX_CHARACTERS_V0: usize = MAX_INLINE_SUBJECT_BYTES_V0 * 2;

/// Maximum decoded UTF-8 byte length of the compiled predicate identifier.
pub const MAX_PREDICATE_ID_BYTES_V0: usize = 64;

/// Maximum number of decoded claim-owned inline subject bytes.
pub const MAX_INLINE_SUBJECT_BYTES_V0: usize = 4096;

/// Polarity-neutral classification of a claim-inline SHA-256 evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Sha256ClaimInlineBytesPredicateOutcomeKindV0 {
    DigestEqual,
    DigestUnequal,
    ResolutionFailed,
}

/// Closed failure vocabulary in its frozen first-failure order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInlineSubjectResolutionFailureV0 {
    MissingClaim,
    MissingTypedClaim,
    MalformedMetadata,
    MissingClaimDomain,
    WrongTypeClaimDomain,
    DuplicateClaimMetadataKey,
    UnknownClaimDomain,
    ClaimDomainMismatch,
    UnknownClaimMetadataKey,
    MissingInlineSubjectDescriptor,
    WrongTypeInlineSubjectDescriptor,
    DuplicateDescriptorKey,
    UnknownDescriptorKey,
    MissingSchema,
    WrongTypeSchema,
    UnknownSchema,
    MissingPredicateId,
    WrongTypePredicateId,
    EmptyPredicateId,
    PredicateIdTooLong,
    InvalidPredicateId,
    UnknownPredicateId,
    MissingExpectedSha256,
    WrongTypeExpectedSha256,
    InvalidExpectedSha256,
    MissingSubjectHex,
    WrongTypeSubjectHex,
    SubjectHexTooLong,
    InvalidSubjectHex,
    StatementBindingMismatch,
    MissingClaimContentHash,
    ClaimContentHashMismatch,
    DecodedSubjectTooLarge,
}

/// Audit basis for a terminal digest relation.
///
/// Construction is private. Field declaration order is the canonical receipt
/// JSON order.
#[derive(Clone, Serialize)]
pub struct Sha256ClaimInlineBytesPredicateReceiptV0 {
    predicate_schema: String,
    predicate_id: String,
    claim_id: String,
    scope_ref: String,
    canonical_statement: String,
    claim_content_hash: String,
    expected_sha256: String,
    computed_sha256: String,
}

impl Sha256ClaimInlineBytesPredicateReceiptV0 {
    pub fn predicate_schema(&self) -> &str {
        &self.predicate_schema
    }

    pub fn predicate_id(&self) -> &str {
        &self.predicate_id
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
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
}

#[derive(Clone)]
enum Sha256ClaimInlineBytesPredicateOutcomeDataV0 {
    DigestEqual(Sha256ClaimInlineBytesPredicateReceiptV0),
    DigestUnequal(Sha256ClaimInlineBytesPredicateReceiptV0),
    ResolutionFailed {
        requested_claim_id: String,
        failure: ClaimInlineSubjectResolutionFailureV0,
    },
}

/// Opaque evaluator-created claim-inline SHA-256 outcome.
#[derive(Clone)]
pub struct Sha256ClaimInlineBytesPredicateOutcomeV0 {
    data: Sha256ClaimInlineBytesPredicateOutcomeDataV0,
}

impl Sha256ClaimInlineBytesPredicateOutcomeV0 {
    pub fn kind(&self) -> Sha256ClaimInlineBytesPredicateOutcomeKindV0 {
        match self.data {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(_) => {
                Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(_) => {
                Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestUnequal
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed { .. } => {
                Sha256ClaimInlineBytesPredicateOutcomeKindV0::ResolutionFailed
            }
        }
    }

    pub fn requested_claim_id(&self) -> &str {
        match &self.data {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(receipt)
            | Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(receipt) => {
                receipt.claim_id()
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                ..
            } => requested_claim_id,
        }
    }

    pub fn receipt(&self) -> Option<&Sha256ClaimInlineBytesPredicateReceiptV0> {
        match &self.data {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(receipt)
            | Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(receipt) => Some(receipt),
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed { .. } => None,
        }
    }

    pub fn failure(&self) -> Option<&ClaimInlineSubjectResolutionFailureV0> {
        match &self.data {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed { failure, .. } => {
                Some(failure)
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(_)
            | Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(_) => None,
        }
    }

    /// Deterministic one-way audit bytes under
    /// `magpie-claim-inline-sha256-predicate-outcome-json-v0`.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        // Bind this encoder to the compiled profile identity without adding
        // that identity to the contracted wire shape or receipt.
        let _canonicalization_profile = OUTCOME_CANONICALIZATION_PROFILE_V0;
        serde_json::to_vec(self)
            .expect("Sha256ClaimInlineBytesPredicateOutcomeV0 is always serializable")
    }

    fn terminal(
        equal: bool,
        receipt: Sha256ClaimInlineBytesPredicateReceiptV0,
    ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
        let data = if equal {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(receipt)
        } else {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(receipt)
        };
        Self { data }
    }

    fn resolution_failed(
        requested_claim_id: String,
        failure: ClaimInlineSubjectResolutionFailureV0,
    ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
        Self {
            data: Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                failure,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
enum Sha256ClaimInlineBytesPredicateOutcomeWireV0<'a> {
    DigestEqual {
        receipt: &'a Sha256ClaimInlineBytesPredicateReceiptV0,
    },
    DigestUnequal {
        receipt: &'a Sha256ClaimInlineBytesPredicateReceiptV0,
    },
    ResolutionFailed {
        claim_id: &'a str,
        reason: ClaimInlineSubjectResolutionFailureV0,
    },
}

impl Serialize for Sha256ClaimInlineBytesPredicateOutcomeV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire = match &self.data {
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestEqual(receipt) => {
                Sha256ClaimInlineBytesPredicateOutcomeWireV0::DigestEqual { receipt }
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::DigestUnequal(receipt) => {
                Sha256ClaimInlineBytesPredicateOutcomeWireV0::DigestUnequal { receipt }
            }
            Sha256ClaimInlineBytesPredicateOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                failure,
            } => Sha256ClaimInlineBytesPredicateOutcomeWireV0::ResolutionFailed {
                claim_id: requested_claim_id,
                reason: *failure,
            },
        };
        wire.serialize(serializer)
    }
}

struct TerminalEvaluationV0 {
    equal: bool,
    receipt: Sha256ClaimInlineBytesPredicateReceiptV0,
}

impl StandingReplaySnapshot {
    /// Evaluate the one compiled claim-owned inline-byte SHA-256 predicate.
    pub fn evaluate_sha256_claim_inline_bytes_equals_v0(
        &self,
        claim_id: &str,
    ) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
        let requested_claim_id = claim_id.to_owned();
        match evaluate_claim_inline_sha256(self, claim_id) {
            Ok(terminal) => {
                Sha256ClaimInlineBytesPredicateOutcomeV0::terminal(terminal.equal, terminal.receipt)
            }
            Err(failure) => Sha256ClaimInlineBytesPredicateOutcomeV0::resolution_failed(
                requested_claim_id,
                failure,
            ),
        }
    }
}

fn evaluate_claim_inline_sha256(
    snapshot: &StandingReplaySnapshot,
    claim_id: &str,
) -> Result<TerminalEvaluationV0, ClaimInlineSubjectResolutionFailureV0> {
    let standing_claim = snapshot
        .standing()
        .get(claim_id)
        .ok_or(ClaimInlineSubjectResolutionFailureV0::MissingClaim)?;
    let typed_claim = snapshot
        .standing()
        .typed_claim(claim_id)
        .ok_or(ClaimInlineSubjectResolutionFailureV0::MissingTypedClaim)?;

    let descriptor = resolve_inline_descriptor(&typed_claim.metadata_json)?;
    let mut canonical_statement = String::with_capacity(
        INLINE_PREDICATE_SCHEMA_V0.len()
            + descriptor.predicate_id.len()
            + descriptor.expected_sha256.len()
            + descriptor.subject_hex.len()
            + 3,
    );
    canonical_statement.push_str(INLINE_PREDICATE_SCHEMA_V0);
    canonical_statement.push(':');
    canonical_statement.push_str(&descriptor.predicate_id);
    canonical_statement.push(':');
    canonical_statement.push_str(&descriptor.expected_sha256);
    canonical_statement.push(':');
    canonical_statement.push_str(&descriptor.subject_hex);

    if standing_claim.statement.as_bytes() != typed_claim.statement.as_bytes()
        || standing_claim.statement.as_bytes() != canonical_statement.as_bytes()
    {
        return Err(ClaimInlineSubjectResolutionFailureV0::StatementBindingMismatch);
    }

    if typed_claim.content_hash.is_empty() {
        return Err(ClaimInlineSubjectResolutionFailureV0::MissingClaimContentHash);
    }
    let recomputed_content_hash = hex::encode(Sha256::digest(canonical_statement.as_bytes()));
    if typed_claim.content_hash.as_bytes() != recomputed_content_hash.as_bytes() {
        return Err(ClaimInlineSubjectResolutionFailureV0::ClaimContentHashMismatch);
    }

    let subject_bytes = decode_subject_bytes(&descriptor.subject_hex)?;
    let computed_bytes: [u8; 32] = Sha256::digest(&subject_bytes).into();
    let equal = computed_bytes == descriptor.expected_bytes;
    let computed_sha256 = hex::encode(computed_bytes);

    Ok(TerminalEvaluationV0 {
        equal,
        receipt: Sha256ClaimInlineBytesPredicateReceiptV0 {
            predicate_schema: INLINE_PREDICATE_SCHEMA_V0.to_owned(),
            predicate_id: descriptor.predicate_id,
            claim_id: claim_id.to_owned(),
            scope_ref: typed_claim.scope_ref.clone(),
            canonical_statement,
            claim_content_hash: typed_claim.content_hash.clone(),
            expected_sha256: descriptor.expected_sha256,
            computed_sha256,
        },
    })
}

fn decode_subject_bytes(
    subject_hex: &str,
) -> Result<Vec<u8>, ClaimInlineSubjectResolutionFailureV0> {
    let decoded = hex::decode(subject_hex)
        .expect("strictly validated lowercase even-length subject hex must decode");
    guard_decoded_subject_size(decoded)
}

fn guard_decoded_subject_size(
    decoded: Vec<u8>,
) -> Result<Vec<u8>, ClaimInlineSubjectResolutionFailureV0> {
    if decoded.len() > MAX_INLINE_SUBJECT_BYTES_V0 {
        Err(ClaimInlineSubjectResolutionFailureV0::DecodedSubjectTooLarge)
    } else {
        Ok(decoded)
    }
}

#[derive(Debug)]
struct ResolvedInlineDescriptorV0 {
    predicate_id: String,
    expected_sha256: String,
    expected_bytes: [u8; 32],
    subject_hex: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct JsonStringToken {
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug, Default)]
struct StringFieldToken {
    seen: bool,
    wrong_type: bool,
    token: Option<JsonStringToken>,
}

impl StringFieldToken {
    fn observe(&mut self, input: &str, value_start: usize, value_end: usize) {
        self.seen = true;
        if input.as_bytes()[value_start] == b'"' {
            if self.token.is_none() {
                self.token = Some(JsonStringToken {
                    start: value_start,
                    end: value_end,
                });
            }
        } else {
            self.wrong_type = true;
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DescriptorTokens {
    duplicate_key: bool,
    unknown_key: bool,
    schema: StringFieldToken,
    predicate_id: StringFieldToken,
    expected_sha256: StringFieldToken,
    subject_hex: StringFieldToken,
}

#[derive(Clone, Copy, Debug, Default)]
struct DescriptorFieldToken {
    seen: bool,
    wrong_type: bool,
    descriptor: Option<DescriptorTokens>,
}

#[derive(Clone, Copy, Debug, Default)]
struct EnvelopeTokens {
    duplicate_key: bool,
    unknown_key: bool,
    claim_domain: StringFieldToken,
    inline_descriptor: DescriptorFieldToken,
}

fn resolve_inline_descriptor(
    metadata_json: &str,
) -> Result<ResolvedInlineDescriptorV0, ClaimInlineSubjectResolutionFailureV0> {
    let envelope = collect_envelope_tokens(metadata_json)
        .ok_or(ClaimInlineSubjectResolutionFailureV0::MalformedMetadata)?;

    if !envelope.claim_domain.seen {
        return Err(ClaimInlineSubjectResolutionFailureV0::MissingClaimDomain);
    }
    if envelope.claim_domain.wrong_type {
        return Err(ClaimInlineSubjectResolutionFailureV0::WrongTypeClaimDomain);
    }
    if envelope.duplicate_key {
        return Err(ClaimInlineSubjectResolutionFailureV0::DuplicateClaimMetadataKey);
    }
    let claim_domain_token = envelope
        .claim_domain
        .token
        .expect("seen string field has a borrowed token");
    let claim_domain = classify_claim_domain(metadata_json, claim_domain_token)
        .ok_or(ClaimInlineSubjectResolutionFailureV0::UnknownClaimDomain)?;
    if claim_domain != ClaimDomain::ExactMachineCheckable {
        return Err(ClaimInlineSubjectResolutionFailureV0::ClaimDomainMismatch);
    }
    if envelope.unknown_key {
        return Err(ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey);
    }
    if !envelope.inline_descriptor.seen {
        return Err(ClaimInlineSubjectResolutionFailureV0::MissingInlineSubjectDescriptor);
    }
    if envelope.inline_descriptor.wrong_type {
        return Err(ClaimInlineSubjectResolutionFailureV0::WrongTypeInlineSubjectDescriptor);
    }
    let descriptor = envelope
        .inline_descriptor
        .descriptor
        .expect("seen object descriptor has collected fixed fields");
    if descriptor.duplicate_key {
        return Err(ClaimInlineSubjectResolutionFailureV0::DuplicateDescriptorKey);
    }
    if descriptor.unknown_key {
        return Err(ClaimInlineSubjectResolutionFailureV0::UnknownDescriptorKey);
    }

    let schema_token = require_string_field(
        descriptor.schema,
        ClaimInlineSubjectResolutionFailureV0::MissingSchema,
        ClaimInlineSubjectResolutionFailureV0::WrongTypeSchema,
    )?;
    if !decoded_json_string_eq(metadata_json, schema_token, INLINE_PREDICATE_SCHEMA_V0) {
        return Err(ClaimInlineSubjectResolutionFailureV0::UnknownSchema);
    }

    let predicate_token = require_string_field(
        descriptor.predicate_id,
        ClaimInlineSubjectResolutionFailureV0::MissingPredicateId,
        ClaimInlineSubjectResolutionFailureV0::WrongTypePredicateId,
    )?;
    if predicate_token.end == predicate_token.start + 2 {
        return Err(ClaimInlineSubjectResolutionFailureV0::EmptyPredicateId);
    }
    let predicate_id_len =
        decoded_utf8_len_through_limit(metadata_json, predicate_token, MAX_PREDICATE_ID_BYTES_V0)
            .map_err(|()| ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong)?;
    debug_assert_ne!(predicate_id_len, 0);
    let predicate_id = materialize_json_string(metadata_json, predicate_token, predicate_id_len);
    if !predicate_id
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(ClaimInlineSubjectResolutionFailureV0::InvalidPredicateId);
    }
    if predicate_id.as_bytes() != SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0.as_bytes() {
        return Err(ClaimInlineSubjectResolutionFailureV0::UnknownPredicateId);
    }

    let expected_token = require_string_field(
        descriptor.expected_sha256,
        ClaimInlineSubjectResolutionFailureV0::MissingExpectedSha256,
        ClaimInlineSubjectResolutionFailureV0::WrongTypeExpectedSha256,
    )?;
    if !is_exact_lowercase_hex(metadata_json, expected_token, 64) {
        return Err(ClaimInlineSubjectResolutionFailureV0::InvalidExpectedSha256);
    }
    let expected_sha256 = materialize_json_string(metadata_json, expected_token, 64);
    let mut expected_bytes = [0u8; 32];
    hex::decode_to_slice(&expected_sha256, &mut expected_bytes)
        .expect("strictly validated expected SHA-256 must decode");

    let subject_token = require_string_field(
        descriptor.subject_hex,
        ClaimInlineSubjectResolutionFailureV0::MissingSubjectHex,
        ClaimInlineSubjectResolutionFailureV0::WrongTypeSubjectHex,
    )?;
    let subject_character_count = decoded_character_count_through_limit(
        metadata_json,
        subject_token,
        MAX_SUBJECT_HEX_CHARACTERS_V0,
    )
    .map_err(|()| ClaimInlineSubjectResolutionFailureV0::SubjectHexTooLong)?;
    if subject_character_count % 2 != 0 || !is_lowercase_hex(metadata_json, subject_token) {
        return Err(ClaimInlineSubjectResolutionFailureV0::InvalidSubjectHex);
    }
    let subject_hex =
        materialize_json_string(metadata_json, subject_token, subject_character_count);

    Ok(ResolvedInlineDescriptorV0 {
        predicate_id,
        expected_sha256,
        expected_bytes,
        subject_hex,
    })
}

fn require_string_field(
    field: StringFieldToken,
    missing: ClaimInlineSubjectResolutionFailureV0,
    wrong_type: ClaimInlineSubjectResolutionFailureV0,
) -> Result<JsonStringToken, ClaimInlineSubjectResolutionFailureV0> {
    if !field.seen {
        Err(missing)
    } else if field.wrong_type {
        Err(wrong_type)
    } else {
        Ok(field.token.expect("seen string field has a borrowed token"))
    }
}

fn classify_claim_domain(input: &str, token: JsonStringToken) -> Option<ClaimDomain> {
    for domain in ClaimDomain::ALL {
        let spelling = domain.as_str();
        if decoded_json_string_eq(input, token, spelling) {
            return ClaimDomain::try_from(spelling).ok();
        }
    }
    None
}

fn is_exact_lowercase_hex(input: &str, token: JsonStringToken, exact_len: usize) -> bool {
    let mut count = 0usize;
    let mut valid = true;
    for character in DecodedJsonStringCharacters::new(input, token) {
        count += 1;
        if !matches!(character, '0'..='9' | 'a'..='f') {
            valid = false;
        }
    }
    valid && count == exact_len
}

fn is_lowercase_hex(input: &str, token: JsonStringToken) -> bool {
    DecodedJsonStringCharacters::new(input, token)
        .all(|character| matches!(character, '0'..='9' | 'a'..='f'))
}

fn decoded_utf8_len_through_limit(
    input: &str,
    token: JsonStringToken,
    limit: usize,
) -> Result<usize, ()> {
    let mut decoded_len = 0usize;
    for character in DecodedJsonStringCharacters::new(input, token) {
        decoded_len = decoded_len.checked_add(character.len_utf8()).ok_or(())?;
        if decoded_len > limit {
            return Err(());
        }
    }
    Ok(decoded_len)
}

fn decoded_character_count_through_limit(
    input: &str,
    token: JsonStringToken,
    limit: usize,
) -> Result<usize, ()> {
    let mut count = 0usize;
    for _ in DecodedJsonStringCharacters::new(input, token) {
        count = count.checked_add(1).ok_or(())?;
        if count > limit {
            return Err(());
        }
    }
    Ok(count)
}

fn materialize_json_string(input: &str, token: JsonStringToken, capacity: usize) -> String {
    let mut decoded = String::with_capacity(capacity);
    for character in DecodedJsonStringCharacters::new(input, token) {
        decoded.push(character);
    }
    decoded
}

fn decoded_json_string_eq(input: &str, token: JsonStringToken, expected: &str) -> bool {
    DecodedJsonStringCharacters::new(input, token).eq(expected.chars())
}

fn decoded_json_tokens_equal(input: &str, left: JsonStringToken, right: JsonStringToken) -> bool {
    DecodedJsonStringCharacters::new(input, left).eq(DecodedJsonStringCharacters::new(input, right))
}

struct DecodedJsonStringCharacters<'a> {
    input: &'a str,
    cursor: usize,
    content_end: usize,
}

impl<'a> DecodedJsonStringCharacters<'a> {
    fn new(input: &'a str, token: JsonStringToken) -> Self {
        Self {
            input,
            cursor: token.start + 1,
            content_end: token.end - 1,
        }
    }
}

impl Iterator for DecodedJsonStringCharacters<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.content_end {
            return None;
        }
        let bytes = self.input.as_bytes();
        if bytes[self.cursor] != b'\\' {
            let character = self.input[self.cursor..self.content_end]
                .chars()
                .next()
                .expect("validated JSON string has a scalar at the cursor");
            self.cursor += character.len_utf8();
            return Some(character);
        }

        let escape = bytes[self.cursor + 1];
        match escape {
            b'"' => {
                self.cursor += 2;
                Some('"')
            }
            b'\\' => {
                self.cursor += 2;
                Some('\\')
            }
            b'/' => {
                self.cursor += 2;
                Some('/')
            }
            b'b' => {
                self.cursor += 2;
                Some('\u{0008}')
            }
            b'f' => {
                self.cursor += 2;
                Some('\u{000c}')
            }
            b'n' => {
                self.cursor += 2;
                Some('\n')
            }
            b'r' => {
                self.cursor += 2;
                Some('\r')
            }
            b't' => {
                self.cursor += 2;
                Some('\t')
            }
            b'u' => {
                let first = parse_hex_quad(bytes, self.cursor + 2)
                    .expect("validated JSON Unicode escape must parse");
                if (0xd800..=0xdbff).contains(&first) {
                    let second = parse_hex_quad(bytes, self.cursor + 8)
                        .expect("validated JSON surrogate pair must parse");
                    self.cursor += 12;
                    let scalar = 0x1_0000
                        + (((u32::from(first) - 0xd800) << 10) | (u32::from(second) - 0xdc00));
                    char::from_u32(scalar)
                } else {
                    self.cursor += 6;
                    char::from_u32(u32::from(first))
                }
            }
            _ => unreachable!("validated JSON string has only known escapes"),
        }
    }
}

fn collect_envelope_tokens(input: &str) -> Option<EnvelopeTokens> {
    let (document_start, document_end) = validate_json_document(input)?;
    if input.as_bytes()[document_start] != b'{' {
        return None;
    }
    let outer_close = document_end - 1;
    let mut envelope = EnvelopeTokens::default();
    let mut members = ObjectMemberCursor::new(input, document_start, outer_close);
    while let Some(member) = members.next() {
        if key_has_equal_predecessor(input, document_start, outer_close, member.key) {
            envelope.duplicate_key = true;
        }

        if decoded_json_string_eq(input, member.key, "claim_domain") {
            envelope
                .claim_domain
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "machine_predicate_inline_bytes") {
            envelope.inline_descriptor.seen = true;
            if input.as_bytes()[member.value_start] == b'{' {
                if envelope.inline_descriptor.descriptor.is_none() {
                    envelope.inline_descriptor.descriptor = Some(collect_descriptor_tokens(
                        input,
                        member.value_start,
                        member.value_end - 1,
                    ));
                }
            } else {
                envelope.inline_descriptor.wrong_type = true;
            }
        } else {
            envelope.unknown_key = true;
        }
    }
    Some(envelope)
}

fn collect_descriptor_tokens(input: &str, open: usize, close: usize) -> DescriptorTokens {
    let mut descriptor = DescriptorTokens::default();
    let mut members = ObjectMemberCursor::new(input, open, close);
    while let Some(member) = members.next() {
        if key_has_equal_predecessor(input, open, close, member.key) {
            descriptor.duplicate_key = true;
        }

        if decoded_json_string_eq(input, member.key, "schema") {
            descriptor
                .schema
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "predicate_id") {
            descriptor
                .predicate_id
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "expected_sha256") {
            descriptor
                .expected_sha256
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "subject_hex") {
            descriptor
                .subject_hex
                .observe(input, member.value_start, member.value_end);
        } else {
            descriptor.unknown_key = true;
        }
    }
    descriptor
}

#[derive(Clone, Copy)]
struct ObjectMember {
    key: JsonStringToken,
    value_start: usize,
    value_end: usize,
}

struct ObjectMemberCursor<'a> {
    input: &'a str,
    cursor: usize,
    close: usize,
}

impl<'a> ObjectMemberCursor<'a> {
    fn new(input: &'a str, open: usize, close: usize) -> Self {
        Self {
            input,
            cursor: open + 1,
            close,
        }
    }

    fn next(&mut self) -> Option<ObjectMember> {
        self.cursor = skip_json_whitespace(self.input.as_bytes(), self.cursor, self.close);
        if self.cursor >= self.close {
            return None;
        }
        let key_start = self.cursor;
        let key_end =
            scan_json_string(self.input, key_start).expect("validated object key must be a string");
        let key = JsonStringToken {
            start: key_start,
            end: key_end,
        };
        self.cursor = skip_json_whitespace(self.input.as_bytes(), key_end, self.close);
        self.cursor += 1;
        self.cursor = skip_json_whitespace(self.input.as_bytes(), self.cursor, self.close);
        let value_start = self.cursor;
        let value_end = scan_json_value_end(self.input, value_start, self.close)
            .expect("validated object member must contain one value");
        self.cursor = skip_json_whitespace(self.input.as_bytes(), value_end, self.close);
        if self.cursor < self.close {
            self.cursor += 1;
        }
        Some(ObjectMember {
            key,
            value_start,
            value_end,
        })
    }
}

fn key_has_equal_predecessor(
    input: &str,
    open: usize,
    close: usize,
    current: JsonStringToken,
) -> bool {
    let mut prior_members = ObjectMemberCursor::new(input, open, close);
    while let Some(prior) = prior_members.next() {
        if prior.key.start >= current.start {
            return false;
        }
        if decoded_json_tokens_equal(input, prior.key, current) {
            return true;
        }
    }
    false
}

fn validate_json_document(input: &str) -> Option<(usize, usize)> {
    if !validate_json_strings_and_delimiters(input) {
        return None;
    }
    let bytes = input.as_bytes();
    let start = skip_json_whitespace(bytes, 0, bytes.len());
    if start == bytes.len() {
        return None;
    }
    let end = scan_json_value_end(input, start, bytes.len())?;
    if skip_json_whitespace(bytes, end, bytes.len()) != bytes.len() {
        return None;
    }
    if !validate_every_json_container(input) {
        return None;
    }
    Some((start, end))
}

fn validate_json_strings_and_delimiters(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    let mut depth = 0usize;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'"' => {
                let Some(end) = scan_json_string(input, cursor) else {
                    return false;
                };
                cursor = end;
            }
            b'{' | b'[' => {
                depth = match depth.checked_add(1) {
                    Some(depth) => depth,
                    None => return false,
                };
                cursor += 1;
            }
            closing @ (b'}' | b']') => {
                if depth == 0 {
                    return false;
                }
                let Some(opening) = opening_delimiter_at_depth(input, cursor, depth) else {
                    return false;
                };
                if !delimiters_match(opening, closing) {
                    return false;
                }
                depth -= 1;
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    depth == 0
}

fn opening_delimiter_at_depth(input: &str, stop: usize, target_depth: usize) -> Option<u8> {
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    let mut depth = 0usize;
    let mut opening = None;
    while cursor < stop {
        match bytes[cursor] {
            b'"' => cursor = scan_json_string(input, cursor)?,
            byte @ (b'{' | b'[') => {
                depth = depth.checked_add(1)?;
                if depth == target_depth {
                    opening = Some(byte);
                }
                cursor += 1;
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1)?;
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    opening
}

fn delimiters_match(opening: u8, closing: u8) -> bool {
    matches!((opening, closing), (b'{', b'}') | (b'[', b']'))
}

fn validate_every_json_container(input: &str) -> bool {
    let bytes = input.as_bytes();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'"' => {
                let Some(end) = scan_json_string(input, cursor) else {
                    return false;
                };
                cursor = end;
            }
            b'{' | b'[' => {
                let Some(close) = find_matching_json_close(input, cursor, bytes.len()) else {
                    return false;
                };
                let valid = if bytes[cursor] == b'{' {
                    validate_json_object(input, cursor, close)
                } else {
                    validate_json_array(input, cursor, close)
                };
                if !valid {
                    return false;
                }
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    true
}

fn validate_json_object(input: &str, open: usize, close: usize) -> bool {
    let bytes = input.as_bytes();
    let mut cursor = skip_json_whitespace(bytes, open + 1, close);
    if cursor == close {
        return true;
    }
    loop {
        if cursor >= close || bytes[cursor] != b'"' {
            return false;
        }
        let Some(key_end) = scan_json_string(input, cursor) else {
            return false;
        };
        cursor = skip_json_whitespace(bytes, key_end, close);
        if cursor >= close || bytes[cursor] != b':' {
            return false;
        }
        cursor = skip_json_whitespace(bytes, cursor + 1, close);
        if cursor >= close {
            return false;
        }
        let Some(value_end) = scan_json_value_end(input, cursor, close) else {
            return false;
        };
        cursor = skip_json_whitespace(bytes, value_end, close);
        if cursor == close {
            return true;
        }
        if bytes[cursor] != b',' {
            return false;
        }
        cursor = skip_json_whitespace(bytes, cursor + 1, close);
        if cursor == close {
            return false;
        }
    }
}

fn validate_json_array(input: &str, open: usize, close: usize) -> bool {
    let bytes = input.as_bytes();
    let mut cursor = skip_json_whitespace(bytes, open + 1, close);
    if cursor == close {
        return true;
    }
    loop {
        let Some(value_end) = scan_json_value_end(input, cursor, close) else {
            return false;
        };
        cursor = skip_json_whitespace(bytes, value_end, close);
        if cursor == close {
            return true;
        }
        if bytes[cursor] != b',' {
            return false;
        }
        cursor = skip_json_whitespace(bytes, cursor + 1, close);
        if cursor == close {
            return false;
        }
    }
}

fn scan_json_value_end(input: &str, start: usize, limit: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    if start >= limit {
        return None;
    }
    match bytes[start] {
        b'"' => {
            let end = scan_json_string(input, start)?;
            (end <= limit).then_some(end)
        }
        b'{' | b'[' => {
            let close = find_matching_json_close(input, start, limit)?;
            Some(close + 1)
        }
        b't' if bytes.get(start..start + 4) == Some(b"true") && start + 4 <= limit => {
            Some(start + 4)
        }
        b'f' if bytes.get(start..start + 5) == Some(b"false") && start + 5 <= limit => {
            Some(start + 5)
        }
        b'n' if bytes.get(start..start + 4) == Some(b"null") && start + 4 <= limit => {
            Some(start + 4)
        }
        b'-' | b'0'..=b'9' => scan_json_number_end(bytes, start, limit),
        _ => None,
    }
}

fn scan_json_number_end(bytes: &[u8], start: usize, limit: usize) -> Option<usize> {
    let mut cursor = start;
    if bytes[cursor] == b'-' {
        cursor += 1;
        if cursor >= limit {
            return None;
        }
    }

    match bytes[cursor] {
        b'0' => cursor += 1,
        b'1'..=b'9' => {
            cursor += 1;
            while cursor < limit && bytes[cursor].is_ascii_digit() {
                cursor += 1;
            }
        }
        _ => return None,
    }

    if cursor < limit && bytes[cursor] == b'.' {
        cursor += 1;
        let fraction_start = cursor;
        while cursor < limit && bytes[cursor].is_ascii_digit() {
            cursor += 1;
        }
        if cursor == fraction_start {
            return None;
        }
    }

    if cursor < limit && matches!(bytes[cursor], b'e' | b'E') {
        cursor += 1;
        if cursor < limit && matches!(bytes[cursor], b'+' | b'-') {
            cursor += 1;
        }
        let exponent_start = cursor;
        while cursor < limit && bytes[cursor].is_ascii_digit() {
            cursor += 1;
        }
        if cursor == exponent_start {
            return None;
        }
    }

    Some(cursor)
}

fn find_matching_json_close(input: &str, open: usize, limit: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut cursor = open;
    let mut depth = 0usize;
    while cursor < limit {
        match bytes[cursor] {
            b'"' => cursor = scan_json_string(input, cursor)?,
            b'{' | b'[' => {
                depth = depth.checked_add(1)?;
                cursor += 1;
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(cursor);
                }
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    None
}

fn scan_json_string(input: &str, start: usize) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return None;
    }
    let mut cursor = start + 1;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'"' => return Some(cursor + 1),
            b'\\' => {
                let escape = *bytes.get(cursor + 1)?;
                match escape {
                    b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {
                        cursor += 2;
                    }
                    b'u' => {
                        let first = parse_hex_quad(bytes, cursor + 2)?;
                        if (0xd800..=0xdbff).contains(&first) {
                            if bytes.get(cursor + 6) != Some(&b'\\')
                                || bytes.get(cursor + 7) != Some(&b'u')
                            {
                                return None;
                            }
                            let second = parse_hex_quad(bytes, cursor + 8)?;
                            if !(0xdc00..=0xdfff).contains(&second) {
                                return None;
                            }
                            cursor += 12;
                        } else if (0xdc00..=0xdfff).contains(&first) {
                            return None;
                        } else {
                            cursor += 6;
                        }
                    }
                    _ => return None,
                }
            }
            0x00..=0x1f => return None,
            byte if byte.is_ascii() => cursor += 1,
            _ => {
                let character = input[cursor..].chars().next()?;
                cursor += character.len_utf8();
            }
        }
    }
    None
}

fn parse_hex_quad(bytes: &[u8], start: usize) -> Option<u16> {
    let digits = bytes.get(start..start + 4)?;
    let mut value = 0u16;
    for digit in digits {
        value = value.checked_mul(16)?;
        value = value.checked_add(u16::from(hex_digit_value(*digit)?))?;
    }
    Some(value)
}

fn hex_digit_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn skip_json_whitespace(bytes: &[u8], mut cursor: usize, limit: usize) -> usize {
    while cursor < limit && matches!(bytes[cursor], b' ' | b'\t' | b'\n' | b'\r') {
        cursor += 1;
    }
    cursor
}

#[cfg(test)]
mod tests {
    use super::*;

    const ABC_DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    fn metadata(
        schema_json: &str,
        predicate_json: &str,
        expected_json: &str,
        subject_json: &str,
    ) -> String {
        format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":{schema_json},"predicate_id":{predicate_json},"expected_sha256":{expected_json},"subject_hex":{subject_json}}}}}"#
        )
    }

    fn valid_metadata_with_predicate_json(predicate_json: &str) -> String {
        metadata(
            r#""magpie-machine-predicate-inline-bytes-v0""#,
            predicate_json,
            &format!(r#""{ABC_DIGEST}""#),
            r#""616263""#,
        )
    }

    fn failure(metadata_json: &str) -> ClaimInlineSubjectResolutionFailureV0 {
        resolve_inline_descriptor(metadata_json).unwrap_err()
    }

    #[test]
    fn all_failure_encodings_are_frozen_in_declaration_order() {
        use ClaimInlineSubjectResolutionFailureV0::*;

        let cases = [
            (MissingClaim, "missing_claim"),
            (MissingTypedClaim, "missing_typed_claim"),
            (MalformedMetadata, "malformed_metadata"),
            (MissingClaimDomain, "missing_claim_domain"),
            (WrongTypeClaimDomain, "wrong_type_claim_domain"),
            (DuplicateClaimMetadataKey, "duplicate_claim_metadata_key"),
            (UnknownClaimDomain, "unknown_claim_domain"),
            (ClaimDomainMismatch, "claim_domain_mismatch"),
            (UnknownClaimMetadataKey, "unknown_claim_metadata_key"),
            (
                MissingInlineSubjectDescriptor,
                "missing_inline_subject_descriptor",
            ),
            (
                WrongTypeInlineSubjectDescriptor,
                "wrong_type_inline_subject_descriptor",
            ),
            (DuplicateDescriptorKey, "duplicate_descriptor_key"),
            (UnknownDescriptorKey, "unknown_descriptor_key"),
            (MissingSchema, "missing_schema"),
            (WrongTypeSchema, "wrong_type_schema"),
            (UnknownSchema, "unknown_schema"),
            (MissingPredicateId, "missing_predicate_id"),
            (WrongTypePredicateId, "wrong_type_predicate_id"),
            (EmptyPredicateId, "empty_predicate_id"),
            (PredicateIdTooLong, "predicate_id_too_long"),
            (InvalidPredicateId, "invalid_predicate_id"),
            (UnknownPredicateId, "unknown_predicate_id"),
            (MissingExpectedSha256, "missing_expected_sha256"),
            (WrongTypeExpectedSha256, "wrong_type_expected_sha256"),
            (InvalidExpectedSha256, "invalid_expected_sha256"),
            (MissingSubjectHex, "missing_subject_hex"),
            (WrongTypeSubjectHex, "wrong_type_subject_hex"),
            (SubjectHexTooLong, "subject_hex_too_long"),
            (InvalidSubjectHex, "invalid_subject_hex"),
            (StatementBindingMismatch, "statement_binding_mismatch"),
            (MissingClaimContentHash, "missing_claim_content_hash"),
            (ClaimContentHashMismatch, "claim_content_hash_mismatch"),
            (DecodedSubjectTooLarge, "decoded_subject_too_large"),
        ];

        assert_eq!(cases.len(), 33);
        for (reason, encoding) in cases {
            assert_eq!(
                serde_json::to_string(&reason).unwrap(),
                format!(r#""{encoding}""#)
            );
        }
    }

    #[test]
    fn json_validator_accepts_exact_json_grammar_without_a_depth_cap() {
        let valid_values = [
            "null",
            "true",
            "false",
            "0",
            "-0",
            "123",
            "-12.5e+7",
            r#""""#,
            r#""\uD83D\uDE03""#,
            "[]",
            "{}",
            r#"{"a":[1,true,null,{"b":"c"}]}"#,
        ];
        for value in valid_values {
            assert!(validate_json_document(value).is_some(), "{value}");
        }

        let invalid_values = [
            "",
            "nul",
            "True",
            "+1",
            "01",
            "1.",
            "1e",
            r#""\x""#,
            r#""\uD800""#,
            r#""\uDC00""#,
            "[}",
            "{]",
            "[1,]",
            r#"{"a":1,}"#,
            r#"{"a" 1}"#,
            "null trailing",
            "null null",
            "\u{00a0}null",
        ];
        for value in invalid_values {
            assert!(validate_json_document(value).is_none(), "{value:?}");
        }

        let depth = 512usize;
        let deeply_nested = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        assert!(validate_json_document(&deeply_nested).is_some());
    }

    #[test]
    fn decoded_string_comparison_handles_escapes_surrogates_and_no_normalization() {
        let direct = r#""predicate_id""#;
        let escaped = r#""pred\u0069cate_id""#;
        let direct_token = JsonStringToken {
            start: 0,
            end: direct.len(),
        };
        let escaped_token = JsonStringToken {
            start: 0,
            end: escaped.len(),
        };
        assert!(decoded_json_string_eq(direct, direct_token, "predicate_id"));
        assert!(decoded_json_string_eq(
            escaped,
            escaped_token,
            "predicate_id"
        ));

        let emoji_direct = r#""😃""#;
        let emoji_escaped = r#""\uD83D\uDE03""#;
        assert_eq!(
            DecodedJsonStringCharacters::new(
                emoji_direct,
                JsonStringToken {
                    start: 0,
                    end: emoji_direct.len()
                }
            )
            .collect::<String>(),
            DecodedJsonStringCharacters::new(
                emoji_escaped,
                JsonStringToken {
                    start: 0,
                    end: emoji_escaped.len()
                }
            )
            .collect::<String>()
        );

        let composed = r#""é""#;
        let decomposed = r#""e\u0301""#;
        assert_ne!(
            DecodedJsonStringCharacters::new(
                composed,
                JsonStringToken {
                    start: 0,
                    end: composed.len()
                }
            )
            .collect::<String>(),
            DecodedJsonStringCharacters::new(
                decomposed,
                JsonStringToken {
                    start: 0,
                    end: decomposed.len()
                }
            )
            .collect::<String>()
        );
    }

    #[test]
    fn duplicate_keys_are_compared_after_unescape_without_owned_key_storage() {
        let outer_duplicate = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","cla\u0069m_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{INLINE_PREDICATE_SCHEMA_V0}","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}}}"#
        );
        assert_eq!(
            failure(&outer_duplicate),
            ClaimInlineSubjectResolutionFailureV0::DuplicateClaimMetadataKey
        );

        let descriptor_duplicate = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{INLINE_PREDICATE_SCHEMA_V0}","pred\u0069cate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}}}"#
        );
        assert_eq!(
            failure(&descriptor_duplicate),
            ClaimInlineSubjectResolutionFailureV0::DuplicateDescriptorKey
        );

        let unknown_duplicate = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","unknown😃":0,"unknown\uD83D\uDE03":1,"machine_predicate_inline_bytes":{{"schema":"{INLINE_PREDICATE_SCHEMA_V0}","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}}}"#
        );
        assert_eq!(
            failure(&unknown_duplicate),
            ClaimInlineSubjectResolutionFailureV0::DuplicateClaimMetadataKey
        );
    }

    #[test]
    fn predicate_id_decoded_byte_bound_precedes_copy_and_character_validation() {
        for length in [63usize, 64] {
            let predicate = "a".repeat(length);
            assert_eq!(
                failure(&valid_metadata_with_predicate_json(&format!(
                    r#""{predicate}""#
                ))),
                ClaimInlineSubjectResolutionFailureV0::UnknownPredicateId
            );
        }
        let overlong = "a".repeat(65);
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{overlong}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );

        let invalid_and_overlong = format!("-{}", "a".repeat(64));
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{invalid_and_overlong}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );

        let escaped_64 = r#"\u0061"#.repeat(64);
        let escaped_65 = r#"\u0061"#.repeat(65);
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{escaped_64}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::UnknownPredicateId
        );
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{escaped_65}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );

        let literal_multibyte_64 = "é".repeat(32);
        let literal_multibyte_65 = format!("{}a", "é".repeat(32));
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{literal_multibyte_64}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::InvalidPredicateId
        );
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{literal_multibyte_65}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );

        let escaped_multibyte_64 = r#"\u00e9"#.repeat(32);
        let escaped_multibyte_65 = format!("{}a", r#"\u00e9"#.repeat(32));
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{escaped_multibyte_64}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::InvalidPredicateId
        );
        assert_eq!(
            failure(&valid_metadata_with_predicate_json(&format!(
                r#""{escaped_multibyte_65}""#
            ))),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );
    }

    #[test]
    fn predicate_id_bound_blocks_descriptor_resolution_before_statement_construction() {
        let overlong = "a".repeat(65);
        let metadata_json = metadata(
            r#""magpie-machine-predicate-inline-bytes-v0""#,
            &format!(r#""{overlong}""#),
            r#""not-a-digest""#,
            r#""not-hex""#,
        );
        assert_eq!(
            failure(&metadata_json),
            ClaimInlineSubjectResolutionFailureV0::PredicateIdTooLong
        );
    }

    #[test]
    fn expected_digest_precedes_subject_and_source_key_order_is_irrelevant() {
        let metadata_json = format!(
            r#"{{"machine_predicate_inline_bytes":{{"subject_hex":"XYZ","expected_sha256":"XYZ","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","schema":"{INLINE_PREDICATE_SCHEMA_V0}"}},"claim_domain":"ExactMachineCheckable"}}"#
        );
        assert_eq!(
            failure(&metadata_json),
            ClaimInlineSubjectResolutionFailureV0::InvalidExpectedSha256
        );
    }

    #[test]
    fn subject_bound_uses_logically_decoded_characters_before_materialization() {
        let escaped_subject = format!(
            "\"{}\"",
            r#"\u0036\u0031"#.repeat(MAX_INLINE_SUBJECT_BYTES_V0)
        );
        let resolved = resolve_inline_descriptor(&metadata(
            r#""magpie-machine-predicate-inline-bytes-v0""#,
            r#""sha256_claim_inline_bytes_equals_v0""#,
            &format!(r#""{ABC_DIGEST}""#),
            &escaped_subject,
        ))
        .unwrap();
        assert_eq!(resolved.subject_hex.len(), MAX_SUBJECT_HEX_CHARACTERS_V0);
        assert!(resolved
            .subject_hex
            .bytes()
            .all(|byte| matches!(byte, b'6' | b'1')));

        let overlong = format!("\"{}\"", r#"\u0030"#.repeat(8193));
        assert_eq!(
            failure(&metadata(
                r#""magpie-machine-predicate-inline-bytes-v0""#,
                r#""sha256_claim_inline_bytes_equals_v0""#,
                &format!(r#""{ABC_DIGEST}""#),
                &overlong,
            )),
            ClaimInlineSubjectResolutionFailureV0::SubjectHexTooLong
        );
    }

    #[test]
    fn whole_document_malformed_wins_before_structural_failures() {
        let malformed_suffix = r#"{"unknown":0} trailing"#;
        assert_eq!(
            failure(malformed_suffix),
            ClaimInlineSubjectResolutionFailureV0::MalformedMetadata
        );

        let malformed_nested_unknown = r#"{"claim_domain":"ExactMachineCheckable","unknown":[1,]}"#;
        assert_eq!(
            failure(malformed_nested_unknown),
            ClaimInlineSubjectResolutionFailureV0::MalformedMetadata
        );
    }

    #[test]
    fn deeply_nested_unknown_value_is_valid_json_and_has_no_hidden_depth_failure() {
        let depth = 256usize;
        let nested = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        let metadata_json = format!(
            r#"{{"claim_domain":"ExactMachineCheckable","unknown":{nested},"machine_predicate_inline_bytes":{{"schema":"{INLINE_PREDICATE_SCHEMA_V0}","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}}}"#
        );
        assert_eq!(
            failure(&metadata_json),
            ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey
        );
    }

    #[test]
    fn wrong_type_domain_precedes_outer_duplicate() {
        let metadata_json = format!(
            r#"{{"claim_domain":0,"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{INLINE_PREDICATE_SCHEMA_V0}","predicate_id":"{SHA256_CLAIM_INLINE_BYTES_EQUALS_PREDICATE_V0}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}}}"#
        );
        assert_eq!(
            failure(&metadata_json),
            ClaimInlineSubjectResolutionFailureV0::WrongTypeClaimDomain
        );
    }

    #[test]
    fn decoded_size_guard_is_a_distinct_defense_in_depth_failure() {
        assert_eq!(
            guard_decoded_subject_size(vec![0; MAX_INLINE_SUBJECT_BYTES_V0 + 1]).unwrap_err(),
            ClaimInlineSubjectResolutionFailureV0::DecodedSubjectTooLarge
        );
    }
}
