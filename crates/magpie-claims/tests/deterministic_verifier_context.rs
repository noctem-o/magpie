use std::{cell::Cell, rc::Rc};

use magpie_claims::{
    replay_standing_context, DeterministicVerifierContextTraceV0, StandingReplaySnapshot,
    MAX_WITNESS_BYTES_V0,
};
use magpie_log::{
    LogError, LogReader, LogStore, LogWriter, MemStore, Payload, Provenance, Sig, SignedEvent,
    SigningKey, Status,
};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [83; 32];
const CLAIM_ID: &str = "claim-machine";
const EVIDENCE_ID: &str = "evidence-machine";
const EDGE_ID: &str = "edge-machine";
const SCOPE: &str = "scope:machine";
const ABC_DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut timestamp = 0;
    LogWriter::open_with_clock(
        store,
        signing_key(),
        Box::new(move || {
            timestamp += 1;
            timestamp
        }),
    )
    .unwrap()
}

fn provenance() -> Provenance {
    Provenance::new("test", "deterministic-verifier-context")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn predicate_metadata(expected: &str) -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{expected}"}}}}"#
    )
}

fn witness_metadata(witness_hex: &str) -> String {
    format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"{witness_hex}"}}}}"#
    )
}

#[derive(Clone)]
struct Fixture {
    include_legacy_claim: bool,
    include_typed_claim: bool,
    include_evidence: bool,
    include_edge: bool,
    statement: String,
    claim_content_hash: String,
    claim_metadata: String,
    claim_actor_class: String,
    evidence_kind: String,
    evidence_scope: String,
    evidence_metadata: String,
    evidence_actor_class: String,
    edge_kind: String,
    edge_source: String,
    edge_target: String,
    edge_scope: String,
}

impl Fixture {
    fn abc() -> Self {
        let statement = format!("sha256_bytes_equals_v0:{ABC_DIGEST}");
        let claim_content_hash = sha256_hex(statement.as_bytes());
        Self {
            include_legacy_claim: true,
            include_typed_claim: true,
            include_evidence: true,
            include_edge: true,
            statement,
            claim_content_hash,
            claim_metadata: predicate_metadata(ABC_DIGEST),
            claim_actor_class: "AgentProposer".into(),
            evidence_kind: "DeterministicVerification".into(),
            evidence_scope: SCOPE.into(),
            evidence_metadata: witness_metadata("616263"),
            evidence_actor_class: "SourceImporter".into(),
            edge_kind: "supports".into(),
            edge_source: EVIDENCE_ID.into(),
            edge_target: CLAIM_ID.into(),
            edge_scope: SCOPE.into(),
        }
    }

    fn with_witness(bytes: &[u8]) -> Self {
        let expected = sha256_hex(bytes);
        let statement = format!("sha256_bytes_equals_v0:{expected}");
        Self {
            statement: statement.clone(),
            claim_content_hash: sha256_hex(statement.as_bytes()),
            claim_metadata: predicate_metadata(&expected),
            evidence_metadata: witness_metadata(&hex::encode(bytes)),
            ..Self::abc()
        }
    }

    fn records(&self) -> Vec<Vec<u8>> {
        let store = MemStore::new();
        {
            let mut log = writer(store.clone());
            if self.include_legacy_claim {
                log.append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: CLAIM_ID.into(),
                        statement: self.statement.clone(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            }
            if self.include_typed_claim {
                log.append(
                    provenance(),
                    Payload::ClaimAssertedV2 {
                        claim_id: CLAIM_ID.into(),
                        statement: self.statement.clone(),
                        scope_ref: SCOPE.into(),
                        actor_class: self.claim_actor_class.clone(),
                        content_hash: self.claim_content_hash.clone(),
                        metadata_json: self.claim_metadata.clone(),
                    },
                )
                .unwrap();
            }
            if self.include_evidence {
                log.append(
                    provenance(),
                    Payload::EvidenceRegistered {
                        evidence_id: EVIDENCE_ID.into(),
                        evidence_kind: self.evidence_kind.clone(),
                        summary: "inline witness".into(),
                        scope_ref: self.evidence_scope.clone(),
                        actor_class: self.evidence_actor_class.clone(),
                        content_hash: String::new(),
                        metadata_json: self.evidence_metadata.clone(),
                    },
                )
                .unwrap();
            }
            if self.include_edge {
                log.append(
                    provenance(),
                    Payload::JustificationEdgeRecorded {
                        edge_id: EDGE_ID.into(),
                        edge_kind: self.edge_kind.clone(),
                        source_id: self.edge_source.clone(),
                        target_id: self.edge_target.clone(),
                        scope_ref: self.edge_scope.clone(),
                        actor_class: "HumanRoot".into(),
                        rationale: "candidate relationship".into(),
                        metadata_json: "{}".into(),
                    },
                )
                .unwrap();
            }
        }
        store.records()
    }

    fn snapshot(&self) -> StandingReplaySnapshot {
        let reader = LogReader::open(
            MemStore::from_records(self.records()),
            signing_key().verifying_key(),
        );
        replay_standing_context(&reader).unwrap()
    }

    fn outcome(&self) -> Option<DeterministicVerifierContextTraceV0> {
        self.snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
    }
}

fn assert_outcome(
    mut fixture: Fixture,
    claim_metadata: Option<&str>,
    evidence_metadata: Option<&str>,
    expected: DeterministicVerifierContextTraceV0,
) {
    if let Some(value) = claim_metadata {
        fixture.claim_metadata = value.into();
    }
    if let Some(value) = evidence_metadata {
        fixture.evidence_metadata = value.into();
    }
    assert_eq!(fixture.outcome(), Some(expected));
}

#[test]
fn sha256_witness_exact_match_yields_standing_inert_context() {
    let fixture = Fixture::abc();
    assert_eq!(
        sha256_hex(fixture.statement.as_bytes()),
        "70cea2ac9ed9d373b422ef3543e45ddaf289fa4e64a8aed726fc6b13df797421"
    );
    let snapshot = fixture.snapshot();
    let trace = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap();
    let receipt = trace.matched_receipt().unwrap();
    assert_eq!(receipt.predicate_schema(), "magpie-machine-predicate-v0");
    assert_eq!(receipt.predicate_id(), "sha256_bytes_equals_v0");
    assert_eq!(receipt.claim_id(), CLAIM_ID);
    assert_eq!(receipt.evidence_id(), EVIDENCE_ID);
    assert_eq!(receipt.edge_id(), EDGE_ID);
    assert_eq!(receipt.scope_ref(), SCOPE);
    assert_eq!(receipt.canonical_statement(), fixture.statement);
    assert_eq!(receipt.claim_content_hash(), fixture.claim_content_hash);
    assert_eq!(receipt.expected_sha256(), ABC_DIGEST);
    assert_eq!(receipt.computed_sha256(), ABC_DIGEST);
    assert_eq!(receipt.witness_len(), 3);
    assert_eq!(
        snapshot.standing().resolved_standing(CLAIM_ID),
        Some(Status::Conjectured)
    );
}

#[test]
fn empty_witness_exact_match_is_allowed() {
    assert!(matches!(
        Fixture::with_witness(b"").outcome(),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));
}

#[test]
fn witness_4096_byte_boundary_matches() {
    assert!(matches!(
        Fixture::with_witness(&vec![b'x'; MAX_WITNESS_BYTES_V0]).outcome(),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));
}

#[test]
fn deterministic_context_replays_byte_identically() {
    let fixture = Fixture::abc();
    let first = fixture.snapshot();
    let second = fixture.snapshot();
    let a = first
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap();
    let b = second
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap();
    assert_eq!(a, b);
    assert_eq!(a.canonical_bytes(), b.canonical_bytes());
}

#[test]
fn context_query_does_not_mutate_snapshot_bytes() {
    let snapshot = Fixture::abc().snapshot();
    let before = snapshot.canonical_bytes();
    let _ = snapshot.resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID);
    assert_eq!(before, snapshot.canonical_bytes());
}

#[test]
fn deterministic_context_does_not_change_v0_or_v1_standing() {
    let snapshot = Fixture::abc().snapshot();
    let before_v0 = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    let before_v1 = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    let _ = snapshot.resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID);
    assert_eq!(
        before_v0,
        snapshot.standing().resolved_standing_with_trace(CLAIM_ID)
    );
    assert_eq!(
        before_v1,
        snapshot.resolved_standing_with_trace_v1(CLAIM_ID)
    );
    assert_eq!(before_v0.governed_standing, Some(Status::Conjectured));
    assert_eq!(before_v1.governed_standing, Some(Status::Conjectured));
}

#[test]
fn deterministic_context_does_not_change_v0_or_v1_canonical_bytes() {
    let snapshot = Fixture::abc().snapshot();
    let v0 = snapshot
        .standing()
        .resolved_standing_with_trace(CLAIM_ID)
        .canonical_bytes();
    let v1 = snapshot
        .resolved_standing_with_trace_v1(CLAIM_ID)
        .canonical_bytes();
    let _ = snapshot.resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID);
    assert_eq!(
        v0,
        snapshot
            .standing()
            .resolved_standing_with_trace(CLAIM_ID)
            .canonical_bytes()
    );
    assert_eq!(
        v1,
        snapshot
            .resolved_standing_with_trace_v1(CLAIM_ID)
            .canonical_bytes()
    );
}

#[test]
fn automated_verifier_actor_class_does_not_create_authority() {
    let mut fixture = Fixture::abc();
    fixture.evidence_actor_class = "AutomatedVerifier".into();
    assert!(matches!(
        fixture.outcome(),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));
}

#[test]
fn bare_deterministic_verification_label_does_not_match() {
    let mut fixture = Fixture::abc();
    fixture.evidence_metadata = "{}".into();
    assert_eq!(
        fixture.outcome(),
        Some(DeterministicVerifierContextTraceV0::MissingVerificationWitness)
    );
}

#[test]
fn verified_boolean_is_rejected_as_unknown_witness_key() {
    let metadata =
        witness_metadata("616263").replace("\"witness_hex\"", "\"verified\":true,\"witness_hex\"");
    assert_outcome(
        Fixture::abc(),
        None,
        Some(&metadata),
        DeterministicVerifierContextTraceV0::UnknownWitnessKey,
    );
}

#[test]
fn statement_and_content_hash_failures_precede_witness_parsing() {
    for (name, statement, hash, expected) in [
        (
            "arbitrary_statement_cannot_inherit_machine_predicate_support",
            "arbitrary prose".to_string(),
            sha256_hex(b"arbitrary prose"),
            DeterministicVerifierContextTraceV0::StatementPredicateMismatch,
        ),
        (
            "statement_whitespace_variant_fails_closed",
            format!(" sha256_bytes_equals_v0:{ABC_DIGEST}"),
            sha256_hex(format!(" sha256_bytes_equals_v0:{ABC_DIGEST}").as_bytes()),
            DeterministicVerifierContextTraceV0::StatementPredicateMismatch,
        ),
        (
            "statement_uppercase_digest_variant_fails_closed",
            format!("sha256_bytes_equals_v0:{}", ABC_DIGEST.to_uppercase()),
            sha256_hex(format!("sha256_bytes_equals_v0:{}", ABC_DIGEST.to_uppercase()).as_bytes()),
            DeterministicVerifierContextTraceV0::StatementPredicateMismatch,
        ),
        (
            "statement_trailing_newline_fails_closed",
            format!("sha256_bytes_equals_v0:{ABC_DIGEST}\n"),
            sha256_hex(format!("sha256_bytes_equals_v0:{ABC_DIGEST}\n").as_bytes()),
            DeterministicVerifierContextTraceV0::StatementPredicateMismatch,
        ),
    ] {
        let mut fixture = Fixture::abc();
        fixture.statement = statement;
        fixture.claim_content_hash = hash;
        fixture.evidence_metadata = "not json".into();
        assert_eq!(fixture.outcome(), Some(expected), "{name}");
    }
    let mut missing = Fixture::abc();
    missing.claim_content_hash.clear();
    missing.evidence_metadata = "not json".into();
    assert_eq!(
        missing.outcome(),
        Some(DeterministicVerifierContextTraceV0::MissingClaimContentHash)
    );
    let mut mismatch = Fixture::abc();
    mismatch.claim_content_hash = "0".repeat(64);
    mismatch.evidence_metadata = "not json".into();
    assert_eq!(
        mismatch.outcome(),
        Some(DeterministicVerifierContextTraceV0::ClaimContentHashMismatch)
    );
}

#[test]
fn claim_predicate_parser_outcomes_are_closed() {
    let base = predicate_metadata(ABC_DIGEST);
    let cases = [
        (
            r#"{"claim_domain":"ExactMachineCheckable"}"#.to_string(),
            DeterministicVerifierContextTraceV0::MissingMachinePredicate,
        ),
        (
            r#"{"claim_domain":"ExactMachineCheckable","machine_predicate":3}"#.to_string(),
            DeterministicVerifierContextTraceV0::MalformedMachinePredicate,
        ),
        (
            base.replace("\"schema\":", "\"schema\":\"x\",\"schema\":"),
            DeterministicVerifierContextTraceV0::DuplicatePredicateKey,
        ),
        (
            base.replace(
                "\"machine_predicate\":",
                "\"extra\":0,\"machine_predicate\":",
            ),
            DeterministicVerifierContextTraceV0::UnknownPredicateKey,
        ),
        (
            base.replace("\"expected_sha256\":", "\"extra\":0,\"expected_sha256\":"),
            DeterministicVerifierContextTraceV0::UnknownPredicateKey,
        ),
        (
            base.replace("magpie-machine-predicate-v0", "unknown"),
            DeterministicVerifierContextTraceV0::UnknownPredicateSchema,
        ),
        (
            base.replace("sha256_bytes_equals_v0", "unknown_predicate"),
            DeterministicVerifierContextTraceV0::UnknownPredicateId,
        ),
        (
            predicate_metadata("aa"),
            DeterministicVerifierContextTraceV0::InvalidExpectedDigest,
        ),
        (
            predicate_metadata(&"A".repeat(64)),
            DeterministicVerifierContextTraceV0::InvalidExpectedDigest,
        ),
        (
            predicate_metadata(&"g".repeat(64)),
            DeterministicVerifierContextTraceV0::InvalidExpectedDigest,
        ),
        (
            base.replace(&format!("\"{ABC_DIGEST}\""), "true"),
            DeterministicVerifierContextTraceV0::MalformedMachinePredicate,
        ),
    ];
    for (metadata, expected) in cases {
        assert_outcome(Fixture::abc(), Some(&metadata), None, expected);
    }
}

#[test]
fn witness_parser_and_binding_outcomes_are_closed() {
    let base = witness_metadata("616263");
    let cases = [
        (
            "{}".to_string(),
            DeterministicVerifierContextTraceV0::MissingVerificationWitness,
        ),
        (
            r#"{"verification_witness":3}"#.to_string(),
            DeterministicVerifierContextTraceV0::MalformedVerificationWitness,
        ),
        (
            base.replace(
                "\"verification_witness\":",
                "\"verification_witness\":{},\"verification_witness\":",
            ),
            DeterministicVerifierContextTraceV0::DuplicateWitnessKey,
        ),
        (
            base.replace("\"schema\":", "\"schema\":\"x\",\"schema\":"),
            DeterministicVerifierContextTraceV0::DuplicateWitnessKey,
        ),
        (
            base.replace(
                "\"verification_witness\":",
                "\"extra\":0,\"verification_witness\":",
            ),
            DeterministicVerifierContextTraceV0::UnknownWitnessKey,
        ),
        (
            base.replace("\"witness_hex\":", "\"extra\":0,\"witness_hex\":"),
            DeterministicVerifierContextTraceV0::UnknownWitnessKey,
        ),
        (
            base.replace("magpie-verification-witness-v0", "unknown"),
            DeterministicVerifierContextTraceV0::UnknownWitnessSchema,
        ),
        (
            base.replace("sha256_bytes_equals_v0", "unknown"),
            DeterministicVerifierContextTraceV0::PredicateBindingMismatch,
        ),
        (
            base.replace(CLAIM_ID, "other-claim"),
            DeterministicVerifierContextTraceV0::ClaimBindingMismatch,
        ),
        (
            base.replace(SCOPE, "other-scope"),
            DeterministicVerifierContextTraceV0::ScopeBindingMismatch,
        ),
        (
            witness_metadata("zz"),
            DeterministicVerifierContextTraceV0::InvalidWitnessHex,
        ),
        (
            witness_metadata("AB"),
            DeterministicVerifierContextTraceV0::InvalidWitnessHex,
        ),
        (
            witness_metadata("a"),
            DeterministicVerifierContextTraceV0::InvalidWitnessHex,
        ),
        (
            witness_metadata("0x61"),
            DeterministicVerifierContextTraceV0::InvalidWitnessHex,
        ),
        (
            witness_metadata("61 62"),
            DeterministicVerifierContextTraceV0::InvalidWitnessHex,
        ),
        (
            base.replace("\"witness_hex\":\"616263\"", "\"witness_hex\":true"),
            DeterministicVerifierContextTraceV0::MalformedVerificationWitness,
        ),
        (
            format!("{base} null"),
            DeterministicVerifierContextTraceV0::MalformedVerificationWitness,
        ),
        (
            witness_metadata("00"),
            DeterministicVerifierContextTraceV0::DigestMismatch,
        ),
    ];
    for (metadata, expected) in cases {
        assert_outcome(Fixture::abc(), None, Some(&metadata), expected);
    }
}

#[test]
fn witness_4097_bytes_is_rejected() {
    assert_outcome(
        Fixture::abc(),
        None,
        Some(&witness_metadata(&"00".repeat(MAX_WITNESS_BYTES_V0 + 1))),
        DeterministicVerifierContextTraceV0::WitnessTooLarge,
    );
}

#[test]
fn candidate_boundary_returns_no_context_attempt() {
    let cases = [
        {
            let mut f = Fixture::abc();
            f.evidence_kind = "ExternalSource".into();
            f
        },
        {
            let mut f = Fixture::abc();
            f.claim_metadata = f
                .claim_metadata
                .replace("ExactMachineCheckable", "Interpretation");
            f
        },
        {
            let mut f = Fixture::abc();
            f.edge_kind = "contradicts".into();
            f
        },
        {
            let mut f = Fixture::abc();
            f.include_legacy_claim = false;
            f.include_typed_claim = false;
            f
        },
        {
            let mut f = Fixture::abc();
            f.include_evidence = false;
            f
        },
        {
            let mut f = Fixture::abc();
            f.include_edge = false;
            f
        },
        {
            let mut f = Fixture::abc();
            f.evidence_scope = "scope:other".into();
            f
        },
    ];
    for (index, fixture) in cases.into_iter().enumerate() {
        assert_eq!(fixture.outcome(), None, "candidate boundary case {index}");
    }
}

#[test]
fn deterministic_failure_precedence_is_key_order_independent() {
    let predicate_unknown_missing = r#"{"claim_domain":"ExactMachineCheckable","machine_predicate":{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","unknown":0}}"#;
    assert_outcome(
        Fixture::abc(),
        Some(predicate_unknown_missing),
        None,
        DeterministicVerifierContextTraceV0::UnknownPredicateKey,
    );
    let predicate_duplicate_unknown = r#"{"unknown":0,"claim_domain":"ExactMachineCheckable","machine_predicate":{"schema":"x","schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}"#;
    assert_outcome(
        Fixture::abc(),
        Some(predicate_duplicate_unknown),
        None,
        DeterministicVerifierContextTraceV0::DuplicatePredicateKey,
    );
    let both_wrong = predicate_metadata(ABC_DIGEST)
        .replace("magpie-machine-predicate-v0", "bad")
        .replace("sha256_bytes_equals_v0", "also_bad");
    assert_outcome(
        Fixture::abc(),
        Some(&both_wrong),
        None,
        DeterministicVerifierContextTraceV0::UnknownPredicateSchema,
    );
    let witness = witness_metadata("00").replace(CLAIM_ID, "wrong");
    assert_outcome(
        Fixture::abc(),
        None,
        Some(&witness),
        DeterministicVerifierContextTraceV0::ClaimBindingMismatch,
    );
}

#[test]
fn matched_trace_has_literal_stable_audit_json_and_field_order() {
    let trace = Fixture::abc().outcome().unwrap();
    let expected = r#"{"outcome":"matched","receipt":{"predicate_schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","claim_id":"claim-machine","evidence_id":"evidence-machine","edge_id":"edge-machine","scope_ref":"scope:machine","canonical_statement":"sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","claim_content_hash":"70cea2ac9ed9d373b422ef3543e45ddaf289fa4e64a8aed726fc6b13df797421","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","witness_len":3}}"#;
    assert_eq!(trace.canonical_bytes(), expected.as_bytes());
}

#[test]
fn every_unit_failure_has_literal_snake_case_serialization() {
    use DeterministicVerifierContextTraceV0::*;
    let cases = [
        (MissingMachinePredicate, "missing_machine_predicate"),
        (MalformedMachinePredicate, "malformed_machine_predicate"),
        (DuplicatePredicateKey, "duplicate_predicate_key"),
        (UnknownPredicateKey, "unknown_predicate_key"),
        (UnknownPredicateSchema, "unknown_predicate_schema"),
        (UnknownPredicateId, "unknown_predicate_id"),
        (InvalidExpectedDigest, "invalid_expected_digest"),
        (StatementPredicateMismatch, "statement_predicate_mismatch"),
        (MissingClaimContentHash, "missing_claim_content_hash"),
        (ClaimContentHashMismatch, "claim_content_hash_mismatch"),
        (MissingVerificationWitness, "missing_verification_witness"),
        (
            MalformedVerificationWitness,
            "malformed_verification_witness",
        ),
        (DuplicateWitnessKey, "duplicate_witness_key"),
        (UnknownWitnessKey, "unknown_witness_key"),
        (UnknownWitnessSchema, "unknown_witness_schema"),
        (ClaimBindingMismatch, "claim_binding_mismatch"),
        (ScopeBindingMismatch, "scope_binding_mismatch"),
        (PredicateBindingMismatch, "predicate_binding_mismatch"),
        (InvalidWitnessHex, "invalid_witness_hex"),
        (WitnessTooLarge, "witness_too_large"),
        (DigestMismatch, "digest_mismatch"),
    ];
    for (trace, outcome) in cases {
        assert_eq!(
            trace.canonical_bytes(),
            format!(r#"{{"outcome":"{outcome}"}}"#).as_bytes()
        );
    }
}

struct ChangingStore {
    first: Vec<Vec<u8>>,
    second: Vec<Vec<u8>>,
    reads: Rc<Cell<usize>>,
}
impl LogStore for ChangingStore {
    fn append_record(&mut self, _: &[u8]) -> Result<(), LogError> {
        panic!("read only")
    }
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        let n = self.reads.get();
        self.reads.set(n + 1);
        if n == 0 {
            Ok(self.first.clone())
        } else {
            Ok(self.second.clone())
        }
    }
}

#[test]
fn changing_store_cannot_mix_predicate_and_witness_snapshots() {
    let first = Fixture::abc().records();
    let second = Fixture::with_witness(b"different").records();
    let reads = Rc::new(Cell::new(0));
    let reader = LogReader::open(
        ChangingStore {
            first,
            second,
            reads: Rc::clone(&reads),
        },
        signing_key().verifying_key(),
    );
    let snapshot = replay_standing_context(&reader).unwrap();
    assert_eq!(reads.get(), 1);
    assert!(matches!(
        snapshot.resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));
}

#[test]
fn verification_failure_returns_no_snapshot_or_partial_context() {
    let mut records = Fixture::abc().records();
    let mut event: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    event.signature = Sig::new([0; 64]);
    *records.last_mut().unwrap() = serde_json::to_vec(&event).unwrap();
    let reader = LogReader::open(
        MemStore::from_records(records),
        signing_key().verifying_key(),
    );
    assert!(matches!(
        replay_standing_context(&reader),
        Err(LogError::BadSignature { .. })
    ));
}
