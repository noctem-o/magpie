//! Closed detached v2 receipt, per docs/design/detached-standing-receipt-v0.md.
//! Retained inputs are independent of detached bytes. Only trusted rederivation
//! constructs a produced result; hashes alone never certify a computation.

use magpie_log::{
    prepare_portable_history_v0, signature_profile::V_SIG_PROFILE_ID, ContentHash,
    HistoryCheckpointV0, HistoryExpectationEvaluationV0, HistoryExpectationOutcomeV0,
    HistoryExpectationRelationV0, Payload, PortableHistoryOperationalErrorV0,
    PortableHistoryVerificationErrorV0, PortableRejection, Projection, VerifiedReplayEvent,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::replay_snapshot::replay_portable_standing_context;
use crate::{StandingResolutionV2, MAGPIE_CLAIMS_POLICY_V2_ID};

pub const STANDING_RECEIPT_SCHEMA_V0: &str = "magpie-standing-receipt-v0";
pub const STANDING_RECEIPT_ENCODING_PROFILE_V0: &str = "magpie-standing-receipt-json-v0";
pub const STANDING_RECEIPT_HISTORY_PROFILE_V0: &str = "magpie-standing-receipt-history-v2-v0";
pub const STANDING_RECEIPT_CONTEXT_DOMAIN_V0: &str = "magpie-standing-receipt-context-v0";
pub const STANDING_RECEIPT_RESULT_DOMAIN_V0: &str = "magpie-standing-receipt-result-v0";

/// Caller-supplied identity, checked against the exact retained byte image.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingReceiptHistoryIdentityV0 {
    pub sha256: ContentHash,
    pub byte_count: u64,
}

impl StandingReceiptHistoryIdentityV0 {
    /// Compute request material only; this does not verify history.
    pub fn of(history: &[u8]) -> Result<Self, StandingReceiptOperationalErrorV0> {
        Ok(Self {
            sha256: ContentHash::of(history),
            byte_count: u64::try_from(history.len())
                .map_err(|_| StandingReceiptOperationalErrorV0::HistoryByteCountOverflow)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingReceiptCheckpointV0 {
    pub event_count: u64,
    pub commitment_sha256: ContentHash,
}

/// No default, zero-checkpoint substitution, or relation fallback.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StandingReceiptExpectationV0 {
    None,
    Exact {
        checkpoint: StandingReceiptCheckpointV0,
    },
    ContainsCheckpoint {
        checkpoint: StandingReceiptCheckpointV0,
    },
}

/// Closed G/I_G wire shape. Construction supplies coordinates, not authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingReceiptContextV0 {
    pub profile: String,
    pub history: StandingReceiptHistoryIdentityV0,
    pub verifying_key: String,
    pub expectation: StandingReceiptExpectationV0,
    pub claim_id: String,
}

/// Explicit policy selection is validated but not duplicated into I_G.
#[derive(Clone, Debug)]
pub struct StandingReceiptRequestV0 {
    pub context: StandingReceiptContextV0,
    pub policy_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingReceiptVerifiedPrefixV0 {
    pub event_count: u64,
    pub tip_sha256: ContentHash,
}

/// Inert typed output for read/serialization. Fabricating or cloning this value
/// cannot construct a produced receipt. No detached parser is needed for checking.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StandingReceiptWireV0 {
    pub schema: &'static str,
    pub encoding_profile: &'static str,
    pub context: StandingReceiptContextV0,
    pub context_sha256: String,
    pub verified_prefix: StandingReceiptVerifiedPrefixV0,
    pub outcome: StandingResolutionV2,
    pub result_sha256: String,
}

#[derive(Serialize)]
struct ResultBinding<'a> {
    context_sha256: &'a str,
    verified_prefix: &'a StandingReceiptVerifiedPrefixV0,
    outcome: &'a StandingResolutionV2,
}

/// Trusted production/check completion; read-only, with private construction.
/// A receipt conveys only the exact named computation, never permission or truth.
///
/// ```compile_fail
/// use magpie_claims::{ProducedStandingReceiptV0, StandingReceiptWireV0};
/// fn forge(wire: StandingReceiptWireV0) -> ProducedStandingReceiptV0 {
///     ProducedStandingReceiptV0 { wire, bytes: Vec::new() }
/// }
/// ```
/// ```compile_fail
/// let _: magpie_claims::ProducedStandingReceiptV0 = serde_json::from_str("{}").unwrap();
/// ```
/// ```compile_fail
/// use magpie_claims::{ProducedStandingReceiptV0, StandingResolutionV2};
/// fn substitute(receipt: &mut ProducedStandingReceiptV0, outcome: StandingResolutionV2) {
///     receipt.wire().outcome = outcome;
/// }
/// ```
/// ```compile_fail
/// fn write(receipt: &mut magpie_claims::ProducedStandingReceiptV0) {
///     receipt.append_record(b"raw");
/// }
/// ```
/// ```compile_fail
/// use magpie_claims::{ProducedStandingReceiptV0, StandingReceiptWireV0};
/// fn convert(wire: StandingReceiptWireV0) -> ProducedStandingReceiptV0 { wire.into() }
/// ```
#[derive(Debug)]
pub struct ProducedStandingReceiptV0 {
    wire: StandingReceiptWireV0,
    bytes: Vec<u8>,
}

impl ProducedStandingReceiptV0 {
    pub fn wire(&self) -> &StandingReceiptWireV0 {
        &self.wire
    }
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn context_sha256(&self) -> &str {
        &self.wire.context_sha256
    }
    pub fn result_sha256(&self) -> &str {
        &self.wire.result_sha256
    }
}

/// Failures to complete evaluation; none is a native standing outcome.
#[derive(Debug)]
pub enum StandingReceiptOperationalErrorV0 {
    HistoryByteCountOverflow,
    Verification(PortableHistoryOperationalErrorV0),
    Serialization(serde_json::Error),
}

impl std::fmt::Display for StandingReceiptOperationalErrorV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "standing receipt could not complete: {self:?}")
    }
}
impl std::error::Error for StandingReceiptOperationalErrorV0 {}

/// Request/history refusal and operational failure retain distinct typed causes.
/// Native v2 semantic failures instead remain inside a successfully produced outcome.
#[derive(Debug)]
pub enum StandingReceiptErrorV0 {
    UnsupportedProfile,
    UnsupportedPolicy,
    HistoryDigestMismatch,
    HistoryByteCountMismatch,
    HistoryRejected(PortableRejection),
    ExpectationNotSatisfied(HistoryExpectationEvaluationV0),
    EmptyClaimSelector,
    MissingTypedClaim,
    AmbiguousTypedClaim,
    DetachedBytesMismatch,
    Operational(StandingReceiptOperationalErrorV0),
}

impl std::fmt::Display for StandingReceiptErrorV0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "standing receipt: {self:?}")
    }
}
impl std::error::Error for StandingReceiptErrorV0 {}

impl From<PortableHistoryVerificationErrorV0> for StandingReceiptErrorV0 {
    fn from(error: PortableHistoryVerificationErrorV0) -> Self {
        match error {
            PortableHistoryVerificationErrorV0::Rejected(reason) => Self::HistoryRejected(reason),
            PortableHistoryVerificationErrorV0::Operational(error) => {
                Self::Operational(StandingReceiptOperationalErrorV0::Verification(error))
            }
        }
    }
}

impl From<StandingReceiptOperationalErrorV0> for StandingReceiptErrorV0 {
    fn from(error: StandingReceiptOperationalErrorV0) -> Self {
        Self::Operational(error)
    }
}

struct Selector<'a> {
    claim_id: &'a str,
    // Only zero, one, and multiple matter; saturate without introducing a ceiling.
    matches: u8,
}

impl Projection for Selector<'_> {
    fn apply(&mut self, event: &VerifiedReplayEvent<'_>) {
        if let Payload::ClaimAssertedV2 { claim_id, .. } = &event.event().core.payload {
            if claim_id == self.claim_id {
                self.matches = self.matches.saturating_add(1);
            }
        }
    }
}

/// Produce one receipt from one exact retained image and explicit coordinates.
/// No path, clock, ambient lookup, inherited detached outcome, or writer is used.
/// The caller must acquire all bytes before calling; failed acquisition must not
/// be represented by substituting an empty packet. Allocator abort/interruption
/// returns no result, never a semantic standing failure.
pub fn produce_standing_receipt_v0(
    request: &StandingReceiptRequestV0,
    history_bytes: &[u8],
) -> Result<ProducedStandingReceiptV0, StandingReceiptErrorV0> {
    let context = &request.context;
    if context.profile != STANDING_RECEIPT_HISTORY_PROFILE_V0 {
        return Err(StandingReceiptErrorV0::UnsupportedProfile);
    }
    if request.policy_id != MAGPIE_CLAIMS_POLICY_V2_ID {
        return Err(StandingReceiptErrorV0::UnsupportedPolicy);
    }
    let prepared = prepare_portable_history_v0(V_SIG_PROFILE_ID, &context.verifying_key)?;
    let actual = StandingReceiptHistoryIdentityV0::of(history_bytes)?;
    if actual.byte_count != context.history.byte_count {
        return Err(StandingReceiptErrorV0::HistoryByteCountMismatch);
    }
    if actual.sha256 != context.history.sha256 {
        return Err(StandingReceiptErrorV0::HistoryDigestMismatch);
    }
    let history = prepared.verify(history_bytes)?;
    let comparison = match &context.expectation {
        StandingReceiptExpectationV0::None => None,
        StandingReceiptExpectationV0::Exact { checkpoint } => {
            Some((checkpoint, HistoryExpectationRelationV0::Exact))
        }
        StandingReceiptExpectationV0::ContainsCheckpoint { checkpoint } => {
            Some((checkpoint, HistoryExpectationRelationV0::ContainsCheckpoint))
        }
    };
    if let Some((checkpoint, relation)) = comparison {
        let evaluation = history.evaluate_expectation(
            HistoryCheckpointV0::new(checkpoint.event_count, checkpoint.commitment_sha256),
            relation,
        );
        if evaluation.outcome() != HistoryExpectationOutcomeV0::Satisfied {
            return Err(StandingReceiptErrorV0::ExpectationNotSatisfied(evaluation));
        }
    }
    if context.claim_id.is_empty() {
        return Err(StandingReceiptErrorV0::EmptyClaimSelector);
    }
    let mut selector = Selector {
        claim_id: &context.claim_id,
        matches: 0,
    };
    history.replay_into(&mut selector);
    match selector.matches {
        0 => return Err(StandingReceiptErrorV0::MissingTypedClaim),
        1 => {}
        _ => return Err(StandingReceiptErrorV0::AmbiguousTypedClaim),
    }
    let snapshot = replay_portable_standing_context(&history);
    let outcome = snapshot.resolved_standing_with_trace_v2(&context.claim_id);
    let prefix = StandingReceiptVerifiedPrefixV0 {
        event_count: history.event_count(),
        tip_sha256: history.tip(),
    };
    let wire = encode_wire(context.clone(), prefix, outcome)?;
    let bytes = serialize(&wire)?;
    Ok(ProducedStandingReceiptV0 { wire, bytes })
}

/// Accept detached bytes only by full trusted rederivation and exact byte match.
/// Detached bytes never select inputs and are never parsed/normalized here.
pub fn check_standing_receipt_v0(
    request: &StandingReceiptRequestV0,
    history_bytes: &[u8],
    detached_bytes: &[u8],
) -> Result<ProducedStandingReceiptV0, StandingReceiptErrorV0> {
    let produced = produce_standing_receipt_v0(request, history_bytes)?;
    if produced.canonical_bytes() != detached_bytes {
        return Err(StandingReceiptErrorV0::DetachedBytesMismatch);
    }
    Ok(produced)
}

fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, StandingReceiptOperationalErrorV0> {
    serde_json::to_vec(value).map_err(StandingReceiptOperationalErrorV0::Serialization)
}

fn binding_hash(domain: &str, bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(domain.as_bytes());
    digest.update([0]);
    digest.update(bytes);
    hex::encode(digest.finalize())
}

// Encoding alone constructs only inert wire data, never the produced capability.
fn encode_wire(
    context: StandingReceiptContextV0,
    verified_prefix: StandingReceiptVerifiedPrefixV0,
    outcome: StandingResolutionV2,
) -> Result<StandingReceiptWireV0, StandingReceiptOperationalErrorV0> {
    let context_sha256 = binding_hash(STANDING_RECEIPT_CONTEXT_DOMAIN_V0, &serialize(&context)?);
    let result_sha256 = binding_hash(
        STANDING_RECEIPT_RESULT_DOMAIN_V0,
        &serialize(&ResultBinding {
            context_sha256: &context_sha256,
            verified_prefix: &verified_prefix,
            outcome: &outcome,
        })?,
    );
    Ok(StandingReceiptWireV0 {
        schema: STANDING_RECEIPT_SCHEMA_V0,
        encoding_profile: STANDING_RECEIPT_ENCODING_PROFILE_V0,
        context,
        context_sha256,
        verified_prefix,
        outcome,
        result_sha256,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StandingCurrentness, StandingResolutionFailureV2, StandingTraceReason};
    use magpie_log::Status;

    // Encoding-only contract values: never passed off as portable-verified history.
    const FIXTURE: &[u8] = br#"{"schema":"magpie-standing-receipt-v0","encoding_profile":"magpie-standing-receipt-json-v0","context":{"profile":"magpie-standing-receipt-history-v2-v0","history":{"sha256":"0000000000000000000000000000000000000000000000000000000000000000","byte_count":1},"verifying_key":"1111111111111111111111111111111111111111111111111111111111111111","expectation":{"kind":"exact","checkpoint":{"event_count":2,"commitment_sha256":"2222222222222222222222222222222222222222222222222222222222222222"}},"claim_id":"claim-fixture"},"context_sha256":"5026700c2a766d6b1f434552c6ba3c4759180d86475be872ed13c69916b161cb","verified_prefix":{"event_count":2,"tip_sha256":"2222222222222222222222222222222222222222222222222222222222222222"},"outcome":{"claim_id":"claim-fixture","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[],"blockers":[]},"result_sha256":"3d12b224a3fffbfa3da2047d2ffe78bb266275cae4731a3f1df2bcc33d96bcff"}"#;
    const CONTEXT_HASH: &str = "5026700c2a766d6b1f434552c6ba3c4759180d86475be872ed13c69916b161cb";
    const RESULT_HASH: &str = "3d12b224a3fffbfa3da2047d2ffe78bb266275cae4731a3f1df2bcc33d96bcff";

    fn fixture_wire() -> StandingReceiptWireV0 {
        let tip: ContentHash = serde_json::from_str(&format!("\"{}\"", "2".repeat(64))).unwrap();
        let context = StandingReceiptContextV0 {
            profile: STANDING_RECEIPT_HISTORY_PROFILE_V0.into(),
            history: StandingReceiptHistoryIdentityV0 {
                sha256: ContentHash::ZERO,
                byte_count: 1,
            },
            verifying_key: "1".repeat(64),
            expectation: StandingReceiptExpectationV0::Exact {
                checkpoint: StandingReceiptCheckpointV0 {
                    event_count: 2,
                    commitment_sha256: tip,
                },
            },
            claim_id: "claim-fixture".into(),
        };
        let prefix = StandingReceiptVerifiedPrefixV0 {
            event_count: 2,
            tip_sha256: tip,
        };
        let outcome = StandingResolutionV2 {
            claim_id: "claim-fixture".into(),
            governed_standing: Some(Status::Conjectured),
            legacy_raw_standing: Some(Status::Conjectured),
            currentness: StandingCurrentness::Unknown,
            policy_id: MAGPIE_CLAIMS_POLICY_V2_ID,
            resolution_failure: None,
            trace: Vec::new(),
            blockers: Vec::new(),
        };
        encode_wire(context, prefix, outcome).unwrap()
    }

    #[test]
    fn normative_encoding_vector_exact_bytes_lengths_and_bindings() {
        let wire = fixture_wire();
        assert_eq!(serialize(&wire.context).unwrap().len(), 413);
        assert_eq!(wire.context_sha256, CONTEXT_HASH);
        let binding = serialize(&ResultBinding {
            context_sha256: &wire.context_sha256,
            verified_prefix: &wire.verified_prefix,
            outcome: &wire.outcome,
        })
        .unwrap();
        assert_eq!(binding.len(), 399);
        assert_eq!(wire.result_sha256, RESULT_HASH);
        let bytes = serialize(&wire).unwrap();
        assert_eq!(bytes.len(), 997);
        assert_eq!(bytes, FIXTURE);
    }

    #[test]
    fn native_preservation_and_typed_failure_are_encoded_without_simplification() {
        // Refuted/Supported inheritance and composition failures are not currently
        // reachable from the public resolver on legitimate history. This exercises
        // only inert encoding; existing v2 unit tests pin actual combination rules.
        let fixture = fixture_wire();
        for status in [
            None,
            Some(Status::Open),
            Some(Status::Conjectured),
            Some(Status::Supported),
            Some(Status::Settled),
            Some(Status::Refuted),
        ] {
            let mut outcome = fixture.outcome.clone();
            outcome.governed_standing = status;
            let wire = encode_wire(
                fixture.context.clone(),
                fixture.verified_prefix.clone(),
                outcome.clone(),
            )
            .unwrap();
            assert_eq!(wire.outcome.canonical_bytes(), outcome.canonical_bytes());
            if status != Some(Status::Conjectured) {
                assert_ne!(wire.result_sha256, RESULT_HASH);
            }
        }
        for failure in [
            StandingResolutionFailureV2::InheritedV1TraceLengthMismatch {
                v0_trace_len: 1,
                v1_trace_len: 0,
            },
            StandingResolutionFailureV2::InheritedV1CandidateMismatch { index: 0 },
        ] {
            let mut outcome = fixture.outcome.clone();
            outcome.resolution_failure = Some(failure);
            outcome
                .blockers
                .push(StandingTraceReason::RequiresVerifierContext);
            let wire = encode_wire(
                fixture.context.clone(),
                fixture.verified_prefix.clone(),
                outcome.clone(),
            )
            .unwrap();
            assert_eq!(wire.outcome.canonical_bytes(), outcome.canonical_bytes());
            assert_ne!(wire.result_sha256, RESULT_HASH);
            assert!(String::from_utf8(serialize(&wire).unwrap())
                .unwrap()
                .contains("\"resolution_failure\""));
        }
    }

    #[test]
    fn compact_strings_and_binding_domains_are_frozen() {
        let mut wire = fixture_wire();
        wire.context.claim_id = "quote\"\\\u{08}\t\n\u{0c}\r\u{01}é".into();
        let bytes = serialize(&wire.context).unwrap();
        assert!(String::from_utf8(bytes)
            .unwrap()
            .ends_with(r#""claim_id":"quote\"\\\b\t\n\f\r\u0001é"}"#));
        assert_ne!(
            binding_hash(
                "magpie-standing-receipt-context-v1",
                &serialize(&fixture_wire().context).unwrap()
            ),
            CONTEXT_HASH
        );
        assert_ne!(
            binding_hash(STANDING_RECEIPT_CONTEXT_DOMAIN_V0, b"{}"),
            CONTEXT_HASH
        );
    }

    #[test]
    fn serialization_inability_is_operational_not_standing() {
        struct Fails;
        impl Serialize for Fails {
            fn serialize<S: serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom(
                    "injected serialization inability",
                ))
            }
        }
        assert!(matches!(
            serialize(&Fails),
            Err(StandingReceiptOperationalErrorV0::Serialization(_))
        ));
    }
}
