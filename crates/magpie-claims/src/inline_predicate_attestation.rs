//! Routing-only inline predicate attestation binding.
//!
//! The authority-bearing outcome and receipt are resolver-created audit values.
//! Callers may inspect and serialize values they obtained, but cannot construct,
//! deserialize, reclassify, mutate, or feed them back into an authority path.
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! let _ = InlinePredicateAttestationBindingOutcomeV0 {};
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingReceiptV0;
//!
//! let _ = InlinePredicateAttestationBindingReceiptV0 {
//!     schema: String::new(),
//!     predicate_id: String::new(),
//!     claim_id: String::new(),
//!     evidence_id: String::new(),
//!     scope_ref: String::new(),
//! };
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! let _: InlinePredicateAttestationBindingOutcomeV0 =
//!     serde_json::from_str(r#"{"outcome":"bound"}"#).unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingReceiptV0;
//!
//! let _: InlinePredicateAttestationBindingReceiptV0 =
//!     serde_json::from_str("{}").unwrap();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! let _ = InlinePredicateAttestationBindingOutcomeV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingReceiptV0;
//!
//! let _ = InlinePredicateAttestationBindingReceiptV0::default();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeKindV0,
//!     InlinePredicateAttestationBindingOutcomeV0,
//! };
//!
//! let _: InlinePredicateAttestationBindingOutcomeV0 =
//!     InlinePredicateAttestationBindingOutcomeKindV0::Bound.into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingFailureV0,
//!     InlinePredicateAttestationBindingOutcomeV0,
//! };
//!
//! let _: InlinePredicateAttestationBindingOutcomeV0 =
//!     InlinePredicateAttestationBindingFailureV0::MissingEvidence.into();
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeV0,
//!     InlinePredicateAttestationBindingReceiptV0,
//! };
//!
//! fn cannot_reclassify(
//!     receipt: InlinePredicateAttestationBindingReceiptV0,
//! ) -> InlinePredicateAttestationBindingOutcomeV0 {
//!     receipt.into()
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeV0,
//!     InlinePredicateAttestationBindingReceiptV0,
//! };
//!
//! fn no_public_bound_variant(
//!     receipt: InlinePredicateAttestationBindingReceiptV0,
//! ) -> InlinePredicateAttestationBindingOutcomeV0 {
//!     InlinePredicateAttestationBindingOutcomeV0::Bound(receipt)
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingFailureV0,
//!     InlinePredicateAttestationBindingOutcomeV0,
//! };
//!
//! fn no_public_failure_payload() -> InlinePredicateAttestationBindingOutcomeV0 {
//!     InlinePredicateAttestationBindingOutcomeV0::ResolutionFailed {
//!         requested_claim_id: "claim".into(),
//!         requested_evidence_id: "evidence".into(),
//!         failure: InlinePredicateAttestationBindingFailureV0::MissingEvidence,
//!     }
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! fn cannot_replace_claim_id(outcome: &mut InlinePredicateAttestationBindingOutcomeV0) {
//!     outcome.requested_claim_id = "replacement".into();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! fn cannot_replace_evidence_id(outcome: &mut InlinePredicateAttestationBindingOutcomeV0) {
//!     outcome.requested_evidence_id = "replacement".into();
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeV0,
//!     InlinePredicateAttestationBindingReceiptV0,
//! };
//!
//! fn no_consuming_receipt(
//!     outcome: InlinePredicateAttestationBindingOutcomeV0,
//! ) -> InlinePredicateAttestationBindingReceiptV0 {
//!     outcome.into_receipt()
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! fn opaque_outcome_has_no_debug(outcome: &InlinePredicateAttestationBindingOutcomeV0) {
//!     let _ = format!("{outcome:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingReceiptV0;
//!
//! fn opaque_receipt_has_no_debug(receipt: &InlinePredicateAttestationBindingReceiptV0) {
//!     let _ = format!("{receipt:?}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingOutcomeV0;
//!
//! fn opaque_outcome_has_no_display(outcome: &InlinePredicateAttestationBindingOutcomeV0) {
//!     let _ = format!("{outcome}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationBindingReceiptV0;
//!
//! fn opaque_receipt_has_no_display(receipt: &InlinePredicateAttestationBindingReceiptV0) {
//!     let _ = format!("{receipt}");
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingOutcomeV0, StandingReplaySnapshot,
//! };
//!
//! fn cannot_reinject_outcome(
//!     snapshot: &StandingReplaySnapshot,
//!     outcome: &InlinePredicateAttestationBindingOutcomeV0,
//! ) {
//!     let _ = snapshot.resolve_inline_predicate_attestation_v0("claim", "evidence", outcome);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::{
//!     InlinePredicateAttestationBindingReceiptV0, StandingReplaySnapshot,
//! };
//!
//! fn cannot_reinject_receipt(
//!     snapshot: &StandingReplaySnapshot,
//!     receipt: &InlinePredicateAttestationBindingReceiptV0,
//! ) {
//!     let _ = snapshot.resolve_inline_predicate_attestation_v0("claim", "evidence", receipt);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//!
//! fn serialized_bytes_are_not_authority(snapshot: &StandingReplaySnapshot, bytes: &[u8]) {
//!     let _ = snapshot.resolve_inline_predicate_attestation_v0("claim", "evidence", bytes);
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingView;
//!
//! fn standing_view_is_not_a_resolver(view: &StandingView) {
//!     let _ = view.resolve_inline_predicate_attestation_v0("claim", "evidence");
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
//!     let _ = selected.resolve_inline_predicate_attestation_v0(
//!         "claim",
//!         "evidence",
//!         other,
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::StandingReplaySnapshot;
//!
//! fn no_public_raw_material_resolver(snapshot: &StandingReplaySnapshot) {
//!     let _ = snapshot.resolve_inline_predicate_attestation_raw_v0(
//!         "claim",
//!         "evidence",
//!         "{}",
//!     );
//! }
//! ```
//!
//! ```compile_fail
//! use magpie_claims::InlinePredicateAttestationV0;
//!
//! let _ = InlinePredicateAttestationV0 {};
//! ```

use serde::{Serialize, Serializer};

use crate::claim_inline_sha256_predicate::{
    decoded_json_string_eq, decoded_utf8_len_through_limit, key_has_equal_predecessor,
    materialize_json_string, resolve_claim_inline_subject_v0, validate_json_document,
    JsonStringToken, ObjectMemberCursor, ResolvedClaimInlineSubjectV0, MAX_PREDICATE_ID_BYTES_V0,
};
use crate::replay_snapshot::StandingReplaySnapshot;
use crate::ClaimInlineSubjectResolutionFailureV0;

const INLINE_PREDICATE_ATTESTATION_SCHEMA_V0: &str = "magpie-inline-predicate-attestation-v0";
const DETERMINISTIC_VERIFICATION_EVIDENCE_KIND_V0: &str = "DeterministicVerification";
const OUTCOME_CANONICALIZATION_PROFILE_V0: &str =
    "magpie-inline-predicate-attestation-binding-outcome-json-v0";

/// Polarity-neutral classification of an inline predicate attestation route.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlinePredicateAttestationBindingOutcomeKindV0 {
    Bound,
    ResolutionFailed,
}

/// Closed binding failure vocabulary in its frozen first-failure order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlinePredicateAttestationBindingFailureV0 {
    ClaimPredicateResolutionFailed(ClaimInlineSubjectResolutionFailureV0),
    MissingEvidence,
    WrongEvidenceKind,
    MalformedAttestationMetadata,
    DuplicateAttestationMetadataKey,
    UnknownAttestationMetadataKey,
    MissingInlinePredicateAttestation,
    WrongTypeInlinePredicateAttestation,
    DuplicateAttestationField,
    UnknownAttestationField,
    MissingSchema,
    WrongTypeSchema,
    UnknownSchema,
    MissingPredicateId,
    WrongTypePredicateId,
    EmptyPredicateId,
    PredicateIdTooLong,
    InvalidPredicateId,
    MissingSubjectClaimId,
    WrongTypeSubjectClaimId,
    EmptySubjectClaimId,
    MissingScopeRef,
    WrongTypeScopeRef,
    EmptyScopeRef,
    PredicateBindingMismatch,
    ClaimBindingMismatch,
    EvidenceScopeBindingMismatch,
    ClaimScopeBindingMismatch,
}

/// Exact routing receipt for a successfully bound inline predicate attestation.
///
/// Construction is private. Field declaration order is the canonical receipt
/// JSON order.
#[derive(Clone, Serialize)]
pub struct InlinePredicateAttestationBindingReceiptV0 {
    schema: String,
    predicate_id: String,
    claim_id: String,
    evidence_id: String,
    scope_ref: String,
}

impl InlinePredicateAttestationBindingReceiptV0 {
    pub fn schema(&self) -> &str {
        &self.schema
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

    pub fn scope_ref(&self) -> &str {
        &self.scope_ref
    }
}

#[derive(Clone)]
enum InlinePredicateAttestationBindingOutcomeDataV0 {
    Bound(InlinePredicateAttestationBindingReceiptV0),
    ResolutionFailed {
        requested_claim_id: String,
        requested_evidence_id: String,
        failure: InlinePredicateAttestationBindingFailureV0,
    },
}

/// Opaque resolver-created inline predicate attestation binding outcome.
#[derive(Clone)]
pub struct InlinePredicateAttestationBindingOutcomeV0 {
    data: InlinePredicateAttestationBindingOutcomeDataV0,
}

impl InlinePredicateAttestationBindingOutcomeV0 {
    pub fn kind(&self) -> InlinePredicateAttestationBindingOutcomeKindV0 {
        match self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(_) => {
                InlinePredicateAttestationBindingOutcomeKindV0::Bound
            }
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed { .. } => {
                InlinePredicateAttestationBindingOutcomeKindV0::ResolutionFailed
            }
        }
    }

    pub fn requested_claim_id(&self) -> &str {
        match &self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(receipt) => receipt.claim_id(),
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                ..
            } => requested_claim_id,
        }
    }

    pub fn requested_evidence_id(&self) -> &str {
        match &self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(receipt) => receipt.evidence_id(),
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                requested_evidence_id,
                ..
            } => requested_evidence_id,
        }
    }

    pub fn receipt(&self) -> Option<&InlinePredicateAttestationBindingReceiptV0> {
        match &self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(receipt) => Some(receipt),
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed { .. } => None,
        }
    }

    pub fn failure(&self) -> Option<&InlinePredicateAttestationBindingFailureV0> {
        match &self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                failure, ..
            } => Some(failure),
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(_) => None,
        }
    }

    /// Deterministic one-way audit bytes under
    /// `magpie-inline-predicate-attestation-binding-outcome-json-v0`.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let _canonicalization_profile = OUTCOME_CANONICALIZATION_PROFILE_V0;
        serde_json::to_vec(self)
            .expect("InlinePredicateAttestationBindingOutcomeV0 is always serializable")
    }

    fn bound(receipt: InlinePredicateAttestationBindingReceiptV0) -> Self {
        Self {
            data: InlinePredicateAttestationBindingOutcomeDataV0::Bound(receipt),
        }
    }

    fn resolution_failed(
        requested_claim_id: String,
        requested_evidence_id: String,
        failure: InlinePredicateAttestationBindingFailureV0,
    ) -> Self {
        Self {
            data: InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                failure,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "outcome", content = "details", rename_all = "snake_case")]
enum InlinePredicateAttestationBindingOutcomeWireV0<'a> {
    Bound {
        receipt: &'a InlinePredicateAttestationBindingReceiptV0,
    },
    ResolutionFailed(InlinePredicateAttestationBindingFailureDetailsWireV0<'a>),
}

#[derive(Serialize)]
#[serde(untagged)]
enum InlinePredicateAttestationBindingFailureDetailsWireV0<'a> {
    Ordinary(InlinePredicateAttestationBindingOrdinaryFailureWireV0<'a>),
    ClaimPredicate(InlinePredicateAttestationBindingClaimFailureWireV0<'a>),
}

#[derive(Serialize)]
struct InlinePredicateAttestationBindingOrdinaryFailureWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    reason: &'a InlinePredicateAttestationBindingFailureV0,
}

#[derive(Serialize)]
struct InlinePredicateAttestationBindingClaimFailureWireV0<'a> {
    claim_id: &'a str,
    evidence_id: &'a str,
    reason: InlinePredicateAttestationBindingClaimFailureReasonWireV0,
    claim_reason: ClaimInlineSubjectResolutionFailureV0,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum InlinePredicateAttestationBindingClaimFailureReasonWireV0 {
    ClaimPredicateResolutionFailed,
}

impl Serialize for InlinePredicateAttestationBindingOutcomeV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire = match &self.data {
            InlinePredicateAttestationBindingOutcomeDataV0::Bound(receipt) => {
                InlinePredicateAttestationBindingOutcomeWireV0::Bound { receipt }
            }
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                failure:
                    InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                        claim_reason,
                    ),
            } => InlinePredicateAttestationBindingOutcomeWireV0::ResolutionFailed(
                InlinePredicateAttestationBindingFailureDetailsWireV0::ClaimPredicate(
                    InlinePredicateAttestationBindingClaimFailureWireV0 {
                        claim_id: requested_claim_id,
                        evidence_id: requested_evidence_id,
                        reason:
                            InlinePredicateAttestationBindingClaimFailureReasonWireV0::ClaimPredicateResolutionFailed,
                        claim_reason: *claim_reason,
                    },
                ),
            ),
            InlinePredicateAttestationBindingOutcomeDataV0::ResolutionFailed {
                requested_claim_id,
                requested_evidence_id,
                failure,
            } => InlinePredicateAttestationBindingOutcomeWireV0::ResolutionFailed(
                InlinePredicateAttestationBindingFailureDetailsWireV0::Ordinary(
                    InlinePredicateAttestationBindingOrdinaryFailureWireV0 {
                        claim_id: requested_claim_id,
                        evidence_id: requested_evidence_id,
                        reason: failure,
                    },
                ),
            ),
        };
        wire.serialize(serializer)
    }
}

#[derive(Clone, Copy, Default)]
struct AttestationStringFieldTokenV0 {
    seen: bool,
    wrong_type: bool,
    token: Option<JsonStringToken>,
}

impl AttestationStringFieldTokenV0 {
    fn observe(&mut self, input: &str, value_start: usize, value_end: usize) {
        self.seen = true;
        if input.as_bytes()[value_start] == b'"' {
            if self.token.is_none() {
                self.token = Some(JsonStringToken::new(value_start, value_end));
            }
        } else {
            self.wrong_type = true;
        }
    }
}

#[derive(Clone, Copy, Default)]
struct AttestationFieldTokensV0 {
    duplicate_key: bool,
    unknown_key: bool,
    schema: AttestationStringFieldTokenV0,
    predicate_id: AttestationStringFieldTokenV0,
    subject_claim_id: AttestationStringFieldTokenV0,
    scope_ref: AttestationStringFieldTokenV0,
}

#[derive(Clone, Copy, Default)]
struct AttestationEnvelopeFieldTokenV0 {
    seen: bool,
    wrong_type: bool,
    fields: Option<AttestationFieldTokensV0>,
}

#[derive(Clone, Copy, Default)]
struct AttestationEnvelopeTokensV0 {
    duplicate_key: bool,
    unknown_key: bool,
    attestation: AttestationEnvelopeFieldTokenV0,
}

struct ParsedInlinePredicateAttestationV0 {
    predicate_id: String,
    subject_claim_id: JsonStringToken,
    scope_ref: JsonStringToken,
}

fn collect_attestation_envelope_tokens(input: &str) -> Option<AttestationEnvelopeTokensV0> {
    let (document_start, document_end) = validate_json_document(input)?;
    if input.as_bytes()[document_start] != b'{' {
        return None;
    }

    let outer_close = document_end - 1;
    let mut envelope = AttestationEnvelopeTokensV0::default();
    let mut members = ObjectMemberCursor::new(input, document_start, outer_close);
    while let Some(member) = members.next() {
        if key_has_equal_predecessor(input, document_start, outer_close, member.key) {
            envelope.duplicate_key = true;
        }

        if decoded_json_string_eq(input, member.key, "inline_predicate_attestation") {
            envelope.attestation.seen = true;
            if input.as_bytes()[member.value_start] == b'{' {
                if envelope.attestation.fields.is_none() {
                    envelope.attestation.fields = Some(collect_attestation_field_tokens(
                        input,
                        member.value_start,
                        member.value_end - 1,
                    ));
                }
            } else {
                envelope.attestation.wrong_type = true;
            }
        } else {
            envelope.unknown_key = true;
        }
    }
    Some(envelope)
}

fn collect_attestation_field_tokens(
    input: &str,
    open: usize,
    close: usize,
) -> AttestationFieldTokensV0 {
    let mut fields = AttestationFieldTokensV0::default();
    let mut members = ObjectMemberCursor::new(input, open, close);
    while let Some(member) = members.next() {
        if key_has_equal_predecessor(input, open, close, member.key) {
            fields.duplicate_key = true;
        }

        if decoded_json_string_eq(input, member.key, "schema") {
            fields
                .schema
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "predicate_id") {
            fields
                .predicate_id
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "subject_claim_id") {
            fields
                .subject_claim_id
                .observe(input, member.value_start, member.value_end);
        } else if decoded_json_string_eq(input, member.key, "scope_ref") {
            fields
                .scope_ref
                .observe(input, member.value_start, member.value_end);
        } else {
            fields.unknown_key = true;
        }
    }
    fields
}

fn require_attestation_string_field(
    field: AttestationStringFieldTokenV0,
    missing: InlinePredicateAttestationBindingFailureV0,
    wrong_type: InlinePredicateAttestationBindingFailureV0,
) -> Result<JsonStringToken, InlinePredicateAttestationBindingFailureV0> {
    if !field.seen {
        Err(missing)
    } else if field.wrong_type {
        Err(wrong_type)
    } else {
        Ok(field.token.expect("seen string field has a borrowed token"))
    }
}

fn parse_inline_predicate_attestation_v0(
    metadata_json: &str,
) -> Result<ParsedInlinePredicateAttestationV0, InlinePredicateAttestationBindingFailureV0> {
    let envelope = collect_attestation_envelope_tokens(metadata_json)
        .ok_or(InlinePredicateAttestationBindingFailureV0::MalformedAttestationMetadata)?;

    if envelope.duplicate_key {
        return Err(InlinePredicateAttestationBindingFailureV0::DuplicateAttestationMetadataKey);
    }
    if envelope.unknown_key {
        return Err(InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey);
    }
    if !envelope.attestation.seen {
        return Err(InlinePredicateAttestationBindingFailureV0::MissingInlinePredicateAttestation);
    }
    if envelope.attestation.wrong_type {
        return Err(
            InlinePredicateAttestationBindingFailureV0::WrongTypeInlinePredicateAttestation,
        );
    }

    let fields = envelope
        .attestation
        .fields
        .expect("seen object attestation has collected fixed fields");
    if fields.duplicate_key {
        return Err(InlinePredicateAttestationBindingFailureV0::DuplicateAttestationField);
    }
    if fields.unknown_key {
        return Err(InlinePredicateAttestationBindingFailureV0::UnknownAttestationField);
    }

    let schema = require_attestation_string_field(
        fields.schema,
        InlinePredicateAttestationBindingFailureV0::MissingSchema,
        InlinePredicateAttestationBindingFailureV0::WrongTypeSchema,
    )?;
    if !decoded_json_string_eq(
        metadata_json,
        schema,
        INLINE_PREDICATE_ATTESTATION_SCHEMA_V0,
    ) {
        return Err(InlinePredicateAttestationBindingFailureV0::UnknownSchema);
    }

    let predicate = require_attestation_string_field(
        fields.predicate_id,
        InlinePredicateAttestationBindingFailureV0::MissingPredicateId,
        InlinePredicateAttestationBindingFailureV0::WrongTypePredicateId,
    )?;
    if predicate.is_empty() {
        return Err(InlinePredicateAttestationBindingFailureV0::EmptyPredicateId);
    }
    let predicate_len =
        decoded_utf8_len_through_limit(metadata_json, predicate, MAX_PREDICATE_ID_BYTES_V0)
            .map_err(|()| InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong)?;
    let predicate_id = materialize_json_string(metadata_json, predicate, predicate_len);
    if !predicate_id
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(InlinePredicateAttestationBindingFailureV0::InvalidPredicateId);
    }

    let subject_claim_id = require_attestation_string_field(
        fields.subject_claim_id,
        InlinePredicateAttestationBindingFailureV0::MissingSubjectClaimId,
        InlinePredicateAttestationBindingFailureV0::WrongTypeSubjectClaimId,
    )?;
    if subject_claim_id.is_empty() {
        return Err(InlinePredicateAttestationBindingFailureV0::EmptySubjectClaimId);
    }

    let scope_ref = require_attestation_string_field(
        fields.scope_ref,
        InlinePredicateAttestationBindingFailureV0::MissingScopeRef,
        InlinePredicateAttestationBindingFailureV0::WrongTypeScopeRef,
    )?;
    if scope_ref.is_empty() {
        return Err(InlinePredicateAttestationBindingFailureV0::EmptyScopeRef);
    }

    Ok(ParsedInlinePredicateAttestationV0 {
        predicate_id,
        subject_claim_id,
        scope_ref,
    })
}

pub(crate) struct ResolvedInlinePredicateAttestationBindingV0 {
    predicate_id: String,
    claim_id: String,
    evidence_id: String,
    scope_ref: String,
}

impl ResolvedInlinePredicateAttestationBindingV0 {
    pub(crate) fn predicate_id(&self) -> &str {
        &self.predicate_id
    }

    pub(crate) fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub(crate) fn evidence_id(&self) -> &str {
        &self.evidence_id
    }

    pub(crate) fn scope_ref(&self) -> &str {
        &self.scope_ref
    }
}

pub(crate) fn resolve_inline_predicate_attestation_after_claim_v0(
    snapshot: &StandingReplaySnapshot,
    resolved_claim: &ResolvedClaimInlineSubjectV0,
    evidence_id: &str,
) -> Result<ResolvedInlinePredicateAttestationBindingV0, InlinePredicateAttestationBindingFailureV0>
{
    let evidence = snapshot
        .standing()
        .typed_evidence(evidence_id)
        .ok_or(InlinePredicateAttestationBindingFailureV0::MissingEvidence)?;
    if evidence.evidence_kind.as_bytes() != DETERMINISTIC_VERIFICATION_EVIDENCE_KIND_V0.as_bytes() {
        return Err(InlinePredicateAttestationBindingFailureV0::WrongEvidenceKind);
    }

    let attestation = parse_inline_predicate_attestation_v0(&evidence.metadata_json)?;

    if attestation.predicate_id.as_bytes() != resolved_claim.predicate_id().as_bytes() {
        return Err(InlinePredicateAttestationBindingFailureV0::PredicateBindingMismatch);
    }
    if !decoded_json_string_eq(
        &evidence.metadata_json,
        attestation.subject_claim_id,
        resolved_claim.claim_id(),
    ) {
        return Err(InlinePredicateAttestationBindingFailureV0::ClaimBindingMismatch);
    }
    if !decoded_json_string_eq(
        &evidence.metadata_json,
        attestation.scope_ref,
        &evidence.scope_ref,
    ) {
        return Err(InlinePredicateAttestationBindingFailureV0::EvidenceScopeBindingMismatch);
    }
    if !decoded_json_string_eq(
        &evidence.metadata_json,
        attestation.scope_ref,
        resolved_claim.scope_ref(),
    ) {
        return Err(InlinePredicateAttestationBindingFailureV0::ClaimScopeBindingMismatch);
    }

    Ok(ResolvedInlinePredicateAttestationBindingV0 {
        predicate_id: resolved_claim.predicate_id().to_owned(),
        claim_id: resolved_claim.claim_id().to_owned(),
        evidence_id: evidence_id.to_owned(),
        scope_ref: evidence.scope_ref.clone(),
    })
}

impl StandingReplaySnapshot {
    /// Bind one replayed deterministic-verification evidence node to one
    /// internally resolved claim-inline predicate without evaluating its digest
    /// relation or affecting standing.
    pub fn resolve_inline_predicate_attestation_v0(
        &self,
        claim_id: &str,
        evidence_id: &str,
    ) -> InlinePredicateAttestationBindingOutcomeV0 {
        let resolution_failed = |failure| {
            InlinePredicateAttestationBindingOutcomeV0::resolution_failed(
                claim_id.to_owned(),
                evidence_id.to_owned(),
                failure,
            )
        };

        let resolved_claim = match resolve_claim_inline_subject_v0(self, claim_id) {
            Ok(resolved_claim) => resolved_claim,
            Err(failure) => {
                return resolution_failed(
                    InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                        failure,
                    ),
                );
            }
        };

        let binding = match resolve_inline_predicate_attestation_after_claim_v0(
            self,
            &resolved_claim,
            evidence_id,
        ) {
            Ok(binding) => binding,
            Err(failure) => return resolution_failed(failure),
        };

        InlinePredicateAttestationBindingOutcomeV0::bound(
            InlinePredicateAttestationBindingReceiptV0 {
                schema: INLINE_PREDICATE_ATTESTATION_SCHEMA_V0.to_owned(),
                predicate_id: binding.predicate_id,
                claim_id: binding.claim_id,
                evidence_id: binding.evidence_id,
                scope_ref: binding.scope_ref,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_METADATA: &str = r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#;

    fn parse_failure(metadata: &str) -> InlinePredicateAttestationBindingFailureV0 {
        match parse_inline_predicate_attestation_v0(metadata) {
            Ok(_) => panic!("metadata unexpectedly parsed"),
            Err(failure) => failure,
        }
    }

    #[test]
    fn all_failure_reason_encodings_and_order_are_frozen() {
        use InlinePredicateAttestationBindingFailureV0::*;

        let ordinary = [
            (MissingEvidence, "missing_evidence"),
            (WrongEvidenceKind, "wrong_evidence_kind"),
            (
                MalformedAttestationMetadata,
                "malformed_attestation_metadata",
            ),
            (
                DuplicateAttestationMetadataKey,
                "duplicate_attestation_metadata_key",
            ),
            (
                UnknownAttestationMetadataKey,
                "unknown_attestation_metadata_key",
            ),
            (
                MissingInlinePredicateAttestation,
                "missing_inline_predicate_attestation",
            ),
            (
                WrongTypeInlinePredicateAttestation,
                "wrong_type_inline_predicate_attestation",
            ),
            (DuplicateAttestationField, "duplicate_attestation_field"),
            (UnknownAttestationField, "unknown_attestation_field"),
            (MissingSchema, "missing_schema"),
            (WrongTypeSchema, "wrong_type_schema"),
            (UnknownSchema, "unknown_schema"),
            (MissingPredicateId, "missing_predicate_id"),
            (WrongTypePredicateId, "wrong_type_predicate_id"),
            (EmptyPredicateId, "empty_predicate_id"),
            (PredicateIdTooLong, "predicate_id_too_long"),
            (InvalidPredicateId, "invalid_predicate_id"),
            (MissingSubjectClaimId, "missing_subject_claim_id"),
            (WrongTypeSubjectClaimId, "wrong_type_subject_claim_id"),
            (EmptySubjectClaimId, "empty_subject_claim_id"),
            (MissingScopeRef, "missing_scope_ref"),
            (WrongTypeScopeRef, "wrong_type_scope_ref"),
            (EmptyScopeRef, "empty_scope_ref"),
            (PredicateBindingMismatch, "predicate_binding_mismatch"),
            (ClaimBindingMismatch, "claim_binding_mismatch"),
            (
                EvidenceScopeBindingMismatch,
                "evidence_scope_binding_mismatch",
            ),
            (ClaimScopeBindingMismatch, "claim_scope_binding_mismatch"),
        ];

        assert_eq!(ordinary.len() + 1, 28);
        for (reason, encoding) in ordinary {
            assert_eq!(
                serde_json::to_string(&reason).unwrap(),
                format!(r#""{encoding}""#)
            );
        }
    }

    #[test]
    fn every_nested_claim_failure_is_retained_and_serialized_without_flattening() {
        use ClaimInlineSubjectResolutionFailureV0::*;

        let claim_failures = [
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
        ];
        assert_eq!(claim_failures.len(), 33);

        for claim_failure in claim_failures {
            let failure =
                InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                    claim_failure,
                );
            let outcome = InlinePredicateAttestationBindingOutcomeV0::resolution_failed(
                "claim".to_owned(),
                "evidence".to_owned(),
                failure,
            );
            assert_eq!(outcome.failure(), Some(&failure));
            let claim_reason = serde_json::to_string(&claim_failure).unwrap();
            let expected = format!(
                r#"{{"outcome":"resolution_failed","details":{{"claim_id":"claim","evidence_id":"evidence","reason":"claim_predicate_resolution_failed","claim_reason":{claim_reason}}}}}"#
            );
            assert_eq!(
                outcome.canonical_bytes(),
                expected.as_bytes(),
                "{claim_reason}"
            );
        }
    }

    #[test]
    fn exact_valid_metadata_and_escaped_equivalents_parse() {
        let direct = parse_inline_predicate_attestation_v0(VALID_METADATA).unwrap();
        assert_eq!(direct.predicate_id, "sha256_claim_inline_bytes_equals_v0");

        let escaped = r#"{"inline_predicate_\u0061ttestation":{"sch\u0065ma":"magpie-inline-predicate-attestation-v\u0030","pred\u0069cate_id":"sha256_claim_inline_bytes_equals_v\u0030","subject_claim_\u0069d":"claim-\u0063","scope_\u0072ef":"scope-\u0073"}}"#;
        let parsed = parse_inline_predicate_attestation_v0(escaped).unwrap();
        assert_eq!(parsed.predicate_id, "sha256_claim_inline_bytes_equals_v0");
        assert!(decoded_json_string_eq(
            escaped,
            parsed.subject_claim_id,
            "claim-c"
        ));
        assert!(decoded_json_string_eq(escaped, parsed.scope_ref, "scope-s"));
    }

    #[test]
    fn malformed_complete_documents_fail_before_semantics() {
        for metadata in [
            "",
            "{",
            "{} trailing",
            "[]",
            "null",
            "0",
            r#"{"unknown":[1,]}"#,
            r#"{"inline_predicate_attestation":{"schema":"\x"}}"#,
            r#"{"inline_predicate_attestation":{"schema":"\uD800"}}"#,
            r#"{"inline_predicate_attestation":{"schema":"\uDC00"}}"#,
            r#"{"inline_predicate_attestation":{"schema":01}}"#,
            r#"{"inline_predicate_attestation":{"schema":true false}}"#,
        ] {
            assert_eq!(
                parse_failure(metadata),
                InlinePredicateAttestationBindingFailureV0::MalformedAttestationMetadata,
                "{metadata:?}"
            );
        }
    }

    #[test]
    fn outer_and_inner_structural_precedence_is_source_order_independent() {
        let duplicate_outer = format!(
            r#"{{"unknown":0,"inline_predicate_attestation":{{}},"inline_predicate_\u0061ttestation":{VALID_METADATA}}}"#
        );
        assert_eq!(
            parse_failure(&duplicate_outer),
            InlinePredicateAttestationBindingFailureV0::DuplicateAttestationMetadataKey
        );

        assert_eq!(
            parse_failure(r#"{"unknown":0}"#),
            InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey
        );
        assert_eq!(
            parse_failure("{}"),
            InlinePredicateAttestationBindingFailureV0::MissingInlinePredicateAttestation
        );
        assert_eq!(
            parse_failure(r#"{"inline_predicate_attestation":null}"#),
            InlinePredicateAttestationBindingFailureV0::WrongTypeInlinePredicateAttestation
        );

        let duplicate_inner =
            r#"{"inline_predicate_attestation":{"unknown":0,"schema":"x","sch\u0065ma":"y"}}"#;
        assert_eq!(
            parse_failure(duplicate_inner),
            InlinePredicateAttestationBindingFailureV0::DuplicateAttestationField
        );
        assert_eq!(
            parse_failure(r#"{"inline_predicate_attestation":{"unknown":0}}"#),
            InlinePredicateAttestationBindingFailureV0::UnknownAttestationField
        );
    }

    #[test]
    fn smuggled_authority_fields_are_unknown() {
        for key in [
            "byte",
            "digest",
            "relation",
            "outcome",
            "receipt",
            "edge",
            "polarity",
            "contribution",
            "policy",
            "standing",
        ] {
            let metadata = format!(
                r#"{{"inline_predicate_attestation":{{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":"scope-s","{key}":null}}}}"#
            );
            assert_eq!(
                parse_failure(&metadata),
                InlinePredicateAttestationBindingFailureV0::UnknownAttestationField,
                "{key}"
            );
        }
    }

    #[test]
    fn predicate_decoded_byte_bound_precedes_character_validation() {
        let with_predicate = |predicate_json: &str| {
            format!(
                r#"{{"inline_predicate_attestation":{{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":{predicate_json},"subject_claim_id":"claim-c","scope_ref":"scope-s"}}}}"#
            )
        };

        for length in [63usize, 64] {
            let predicate = "a".repeat(length);
            assert!(
                parse_inline_predicate_attestation_v0(&with_predicate(&format!(
                    r#""{predicate}""#
                )))
                .is_ok()
            );
        }
        let overlong = "a".repeat(65);
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{overlong}""#))),
            InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong
        );

        let escaped_65 = r#"\u0061"#.repeat(65);
        let escaped_64 = r#"\u0061"#.repeat(64);
        assert!(
            parse_inline_predicate_attestation_v0(&with_predicate(&format!(r#""{escaped_64}""#)))
                .is_ok()
        );
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{escaped_65}""#))),
            InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong
        );

        let multibyte_64 = "é".repeat(32);
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{multibyte_64}""#))),
            InlinePredicateAttestationBindingFailureV0::InvalidPredicateId
        );
        let multibyte_65 = format!("{}a", "é".repeat(32));
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{multibyte_65}""#))),
            InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong
        );
        let escaped_multibyte_65 = format!("{}a", r#"\u00e9"#.repeat(32));
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{escaped_multibyte_65}""#))),
            InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong
        );

        let invalid_and_overlong = format!("-{}", "a".repeat(64));
        assert_eq!(
            parse_failure(&with_predicate(&format!(r#""{invalid_and_overlong}""#))),
            InlinePredicateAttestationBindingFailureV0::PredicateIdTooLong
        );
    }

    #[test]
    fn long_unbounded_fields_have_only_the_ratified_semantic_failures() {
        let long = "x".repeat(100_000);
        let long_schema = format!(
            r#"{{"inline_predicate_attestation":{{"schema":"{long}","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":"scope-s"}}}}"#
        );
        assert_eq!(
            parse_failure(&long_schema),
            InlinePredicateAttestationBindingFailureV0::UnknownSchema
        );

        let long_claim = format!(
            r#"{{"inline_predicate_attestation":{{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"{long}","scope_ref":"scope-s"}}}}"#
        );
        assert!(parse_inline_predicate_attestation_v0(&long_claim).is_ok());

        let long_scope = format!(
            r#"{{"inline_predicate_attestation":{{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":"{long}"}}}}"#
        );
        assert!(parse_inline_predicate_attestation_v0(&long_scope).is_ok());
    }

    #[test]
    fn canonical_wire_views_pin_field_order_and_nested_shape() {
        let bound = InlinePredicateAttestationBindingOutcomeV0::bound(
            InlinePredicateAttestationBindingReceiptV0 {
                schema: INLINE_PREDICATE_ATTESTATION_SCHEMA_V0.to_owned(),
                predicate_id: "sha256_claim_inline_bytes_equals_v0".to_owned(),
                claim_id: "claim-c".to_owned(),
                evidence_id: "evidence-e".to_owned(),
                scope_ref: "scope-s".to_owned(),
            },
        );
        assert_eq!(
            String::from_utf8(bound.canonical_bytes()).unwrap(),
            r#"{"outcome":"bound","details":{"receipt":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","scope_ref":"scope-s"}}}"#
        );

        let ordinary = InlinePredicateAttestationBindingOutcomeV0::resolution_failed(
            "claim-c".to_owned(),
            "evidence-missing".to_owned(),
            InlinePredicateAttestationBindingFailureV0::MissingEvidence,
        );
        assert_eq!(
            String::from_utf8(ordinary.canonical_bytes()).unwrap(),
            r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-missing","reason":"missing_evidence"}}"#
        );

        let nested = InlinePredicateAttestationBindingOutcomeV0::resolution_failed(
            "claim-missing".to_owned(),
            "evidence-e".to_owned(),
            InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::MissingClaim,
            ),
        );
        assert_eq!(
            String::from_utf8(nested.canonical_bytes()).unwrap(),
            r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}"#
        );
    }
}
