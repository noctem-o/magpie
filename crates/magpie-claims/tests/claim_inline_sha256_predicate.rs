use magpie_claims::policy::{
    refutation_ceiling, support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
};
use magpie_claims::{
    replay_origin_admission_context_v0, replay_standing_context,
    ClaimInlineSubjectResolutionFailureV0, DeterministicVerifierContextTraceV0,
    OriginAdmissionReplayContextV0, ResolutionContentClosureV0,
    Sha256ClaimInlineBytesPredicateOutcomeKindV0, Sha256ClaimInlineBytesPredicateOutcomeV0,
    StandingReplaySnapshot, MAX_INLINE_SUBJECT_BYTES_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [95; 32];
const CLAIM_ID: &str = "claim-c";
const SCOPE: &str = "scope-s";
const EVIDENCE_ID: &str = "evidence-unrelated";
const EDGE_ID: &str = "edge-contradicts";
const SCHEMA: &str = "magpie-machine-predicate-inline-bytes-v0";
const PREDICATE_ID: &str = "sha256_claim_inline_bytes_equals_v0";
const ABC_DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const EMPTY_DIGEST: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const ZERO_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut timestamp = 0u64;
    LogWriter::<MemStore>::open_with_clock(
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
    Provenance::new("test", "claim-inline-sha256-predicate")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn canonical_statement(expected_sha256: &str, subject_hex: &str) -> String {
    format!("{SCHEMA}:{PREDICATE_ID}:{expected_sha256}:{subject_hex}")
}

fn inline_metadata(expected_sha256: &str, subject_hex: &str) -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{expected_sha256}","subject_hex":"{subject_hex}"}}}}"#
    )
}

fn descriptor_json(
    schema_value: &str,
    predicate_value: &str,
    expected_value: &str,
    subject_value: &str,
) -> String {
    format!(
        r#"{{"schema":{schema_value},"predicate_id":{predicate_value},"expected_sha256":{expected_value},"subject_hex":{subject_value}}}"#
    )
}

fn envelope_json(domain_value: &str, descriptor_value: &str) -> String {
    format!(
        r#"{{"claim_domain":{domain_value},"machine_predicate_inline_bytes":{descriptor_value}}}"#
    )
}

fn valid_descriptor_json() -> String {
    descriptor_json(
        &format!(r#""{SCHEMA}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{ABC_DIGEST}""#),
        r#""616263""#,
    )
}

#[derive(Clone)]
struct EvidenceSpec {
    evidence_kind: String,
    metadata_json: String,
}

#[derive(Clone)]
struct EdgeSpec {
    edge_kind: String,
}

#[derive(Clone)]
struct Fixture {
    include_legacy_claim: bool,
    include_typed_claim: bool,
    legacy_statement: String,
    typed_statement: String,
    claim_content_hash: String,
    claim_metadata: String,
    evidence: Option<EvidenceSpec>,
    edge: Option<EdgeSpec>,
}

impl Fixture {
    fn claim(expected_sha256: &str, subject_hex: &str) -> Self {
        let statement = canonical_statement(expected_sha256, subject_hex);
        Self {
            include_legacy_claim: false,
            include_typed_claim: true,
            legacy_statement: statement.clone(),
            typed_statement: statement.clone(),
            claim_content_hash: sha256_hex(statement.as_bytes()),
            claim_metadata: inline_metadata(expected_sha256, subject_hex),
            evidence: None,
            edge: None,
        }
    }

    fn abc_equal() -> Self {
        Self::claim(ABC_DIGEST, "616263")
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
                        statement: self.legacy_statement.clone(),
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
                        statement: self.typed_statement.clone(),
                        scope_ref: SCOPE.into(),
                        actor_class: "AgentProposer".into(),
                        content_hash: self.claim_content_hash.clone(),
                        metadata_json: self.claim_metadata.clone(),
                    },
                )
                .unwrap();
            }
            if let Some(evidence) = &self.evidence {
                log.append(
                    provenance(),
                    Payload::EvidenceRegistered {
                        evidence_id: EVIDENCE_ID.into(),
                        evidence_kind: evidence.evidence_kind.clone(),
                        summary: "unrelated byte-bearing evidence".into(),
                        scope_ref: SCOPE.into(),
                        actor_class: "SourceImporter".into(),
                        content_hash: String::new(),
                        metadata_json: evidence.metadata_json.clone(),
                    },
                )
                .unwrap();
            }
            if let Some(edge) = &self.edge {
                log.append(
                    provenance(),
                    Payload::JustificationEdgeRecorded {
                        edge_id: EDGE_ID.into(),
                        edge_kind: edge.edge_kind.clone(),
                        source_id: EVIDENCE_ID.into(),
                        target_id: CLAIM_ID.into(),
                        scope_ref: SCOPE.into(),
                        actor_class: "HumanRoot".into(),
                        rationale: "unrelated candidate relationship".into(),
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

    fn origin_context(&self) -> OriginAdmissionReplayContextV0 {
        let reader = LogReader::open(
            MemStore::from_records(self.records()),
            signing_key().verifying_key(),
        );
        replay_origin_admission_context_v0(&reader).unwrap()
    }

    fn outcome(&self) -> Sha256ClaimInlineBytesPredicateOutcomeV0 {
        self.snapshot()
            .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID)
    }
}

fn empty_snapshot() -> StandingReplaySnapshot {
    let fixture = Fixture {
        include_legacy_claim: false,
        include_typed_claim: false,
        legacy_statement: String::new(),
        typed_statement: String::new(),
        claim_content_hash: String::new(),
        claim_metadata: String::new(),
        evidence: None,
        edge: None,
    };
    fixture.snapshot()
}

fn assert_failure(
    outcome: &Sha256ClaimInlineBytesPredicateOutcomeV0,
    expected: ClaimInlineSubjectResolutionFailureV0,
) {
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::ResolutionFailed
    );
    assert_eq!(outcome.failure(), Some(&expected));
    assert!(outcome.receipt().is_none());
}

#[test]
fn abc_matching_digest_is_equal_with_exact_receipt() {
    let outcome = Fixture::abc_equal().outcome();
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );
    assert_eq!(outcome.requested_claim_id(), CLAIM_ID);
    assert!(outcome.failure().is_none());

    let receipt = outcome.receipt().unwrap();
    assert_eq!(receipt.predicate_schema(), SCHEMA);
    assert_eq!(receipt.predicate_id(), PREDICATE_ID);
    assert_eq!(receipt.claim_id(), CLAIM_ID);
    assert_eq!(receipt.scope_ref(), SCOPE);
    assert_eq!(
        receipt.canonical_statement(),
        canonical_statement(ABC_DIGEST, "616263")
    );
    assert_eq!(
        receipt.claim_content_hash(),
        sha256_hex(receipt.canonical_statement().as_bytes())
    );
    assert_eq!(receipt.expected_sha256(), ABC_DIGEST);
    assert_eq!(receipt.computed_sha256(), ABC_DIGEST);
}

#[test]
fn abc_different_valid_digest_is_unequal_not_resolution_failure() {
    let outcome = Fixture::claim(ZERO_DIGEST, "616263").outcome();
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestUnequal
    );
    assert!(outcome.failure().is_none());
    let receipt = outcome.receipt().unwrap();
    assert_eq!(receipt.expected_sha256(), ZERO_DIGEST);
    assert_eq!(receipt.computed_sha256(), ABC_DIGEST);
}

#[test]
fn empty_subject_supports_both_terminal_relations() {
    let equal = Fixture::claim(EMPTY_DIGEST, "").outcome();
    assert_eq!(
        equal.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );
    assert!(equal
        .receipt()
        .unwrap()
        .canonical_statement()
        .ends_with(':'));
    assert_eq!(
        equal.receipt().unwrap().claim_content_hash(),
        "afc617e9136762ccb1218ef474511467bad0d20bff01d8981e4cb983c18d67aa"
    );

    let unequal = Fixture::claim(ZERO_DIGEST, "").outcome();
    assert_eq!(
        unequal.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestUnequal
    );
    assert_eq!(unequal.receipt().unwrap().computed_sha256(), EMPTY_DIGEST);
}

#[test]
fn maximum_decoded_subject_boundary_is_accepted() {
    let subject = vec![0xa5; MAX_INLINE_SUBJECT_BYTES_V0];
    let subject_hex = hex::encode(&subject);
    let expected = sha256_hex(&subject);
    let outcome = Fixture::claim(&expected, &subject_hex).outcome();
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );
    assert_eq!(outcome.receipt().unwrap().expected_sha256(), expected);
}

#[test]
fn repeated_evaluation_and_replay_are_byte_identical() {
    let fixture = Fixture::abc_equal();
    let snapshot = fixture.snapshot();
    let first = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    let second = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    let replayed = fixture
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);

    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(first.canonical_bytes(), replayed.canonical_bytes());
}

#[test]
fn same_snapshot_claim_lookup_failures_and_bindings_are_exact() {
    let missing = empty_snapshot().evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_failure(
        &missing,
        ClaimInlineSubjectResolutionFailureV0::MissingClaim,
    );

    let mut legacy_only = Fixture::abc_equal();
    legacy_only.include_legacy_claim = true;
    legacy_only.include_typed_claim = false;
    assert_failure(
        &legacy_only.outcome(),
        ClaimInlineSubjectResolutionFailureV0::MissingTypedClaim,
    );

    let mut table_mismatch = Fixture::abc_equal();
    table_mismatch.include_legacy_claim = true;
    table_mismatch.legacy_statement = "different legacy statement".into();
    assert_failure(
        &table_mismatch.outcome(),
        ClaimInlineSubjectResolutionFailureV0::StatementBindingMismatch,
    );

    let mut descriptor_mismatch = Fixture::abc_equal();
    descriptor_mismatch.typed_statement = "not the descriptor statement".into();
    assert_failure(
        &descriptor_mismatch.outcome(),
        ClaimInlineSubjectResolutionFailureV0::StatementBindingMismatch,
    );

    let mut missing_hash = Fixture::abc_equal();
    missing_hash.claim_content_hash.clear();
    assert_failure(
        &missing_hash.outcome(),
        ClaimInlineSubjectResolutionFailureV0::MissingClaimContentHash,
    );

    let mut wrong_hash = Fixture::abc_equal();
    wrong_hash.claim_content_hash = ZERO_DIGEST.into();
    assert_failure(
        &wrong_hash.outcome(),
        ClaimInlineSubjectResolutionFailureV0::ClaimContentHashMismatch,
    );
}

#[test]
fn every_reachable_parser_failure_reason_is_selected_exactly() {
    use ClaimInlineSubjectResolutionFailureV0::*;

    let valid_descriptor = valid_descriptor_json();
    let quoted_domain = r#""ExactMachineCheckable""#;
    let overlong_predicate = "a".repeat(65);
    let overlong_subject = "0".repeat(MAX_INLINE_SUBJECT_BYTES_V0 * 2 + 2);

    let cases: Vec<(&str, String, ClaimInlineSubjectResolutionFailureV0)> = vec![
        ("malformed", "{".into(), MalformedMetadata),
        (
            "missing domain",
            format!(r#"{{"machine_predicate_inline_bytes":{valid_descriptor}}}"#),
            MissingClaimDomain,
        ),
        (
            "wrong domain type",
            envelope_json("0", &valid_descriptor),
            WrongTypeClaimDomain,
        ),
        (
            "duplicate outer",
            format!(
                r#"{{"claim_domain":"ExactMachineCheckable","cla\u0069m_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{valid_descriptor}}}"#
            ),
            DuplicateClaimMetadataKey,
        ),
        (
            "unknown domain",
            envelope_json(r#""UnknownDomain""#, &valid_descriptor),
            UnknownClaimDomain,
        ),
        (
            "domain mismatch",
            envelope_json(r#""Interpretation""#, &valid_descriptor),
            ClaimDomainMismatch,
        ),
        (
            "unknown outer",
            format!(
                r#"{{"claim_domain":"ExactMachineCheckable","unknown":0,"machine_predicate_inline_bytes":{valid_descriptor}}}"#
            ),
            UnknownClaimMetadataKey,
        ),
        (
            "missing descriptor",
            format!(r#"{{"claim_domain":{quoted_domain}}}"#),
            MissingInlineSubjectDescriptor,
        ),
        (
            "wrong descriptor type",
            envelope_json(quoted_domain, "0"),
            WrongTypeInlineSubjectDescriptor,
        ),
        (
            "duplicate descriptor",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"schema":"{SCHEMA}","sch\u0065ma":"{SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            DuplicateDescriptorKey,
        ),
        (
            "unknown descriptor",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"schema":"{SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263","unknown":0}}"#
                ),
            ),
            UnknownDescriptorKey,
        ),
        (
            "missing schema",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            MissingSchema,
        ),
        (
            "wrong schema type",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    "0",
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            WrongTypeSchema,
        ),
        (
            "unknown schema",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    r#""magpie-machine-predicate-v0""#,
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            UnknownSchema,
        ),
        (
            "missing predicate",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"schema":"{SCHEMA}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            MissingPredicateId,
        ),
        (
            "wrong predicate type",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    "0",
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            WrongTypePredicateId,
        ),
        (
            "empty predicate",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    r#""""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            EmptyPredicateId,
        ),
        (
            "long predicate",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{overlong_predicate}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            PredicateIdTooLong,
        ),
        (
            "invalid predicate",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    r#""invalid-id""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            InvalidPredicateId,
        ),
        (
            "unknown predicate",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    r#""sha256_bytes_equals_v0""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            UnknownPredicateId,
        ),
        (
            "missing expected",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"schema":"{SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_hex":"616263"}}"#
                ),
            ),
            MissingExpectedSha256,
        ),
        (
            "wrong expected type",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    "0",
                    r#""616263""#,
                ),
            ),
            WrongTypeExpectedSha256,
        ),
        (
            "invalid expected",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    r#""ABC""#,
                    r#""616263""#,
                ),
            ),
            InvalidExpectedSha256,
        ),
        (
            "missing subject",
            envelope_json(
                quoted_domain,
                &format!(
                    r#"{{"schema":"{SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}"}}"#
                ),
            ),
            MissingSubjectHex,
        ),
        (
            "wrong subject type",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    "0",
                ),
            ),
            WrongTypeSubjectHex,
        ),
        (
            "long subject",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    &format!(r#""{overlong_subject}""#),
                ),
            ),
            SubjectHexTooLong,
        ),
        (
            "invalid subject",
            envelope_json(
                quoted_domain,
                &descriptor_json(
                    &format!(r#""{SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""ABC""#,
                ),
            ),
            InvalidSubjectHex,
        ),
    ];

    for (name, metadata, expected) in cases {
        let mut fixture = Fixture::abc_equal();
        fixture.claim_metadata = metadata;
        let outcome = fixture.outcome();
        assert_eq!(outcome.failure(), Some(&expected), "{name}");
        assert_eq!(outcome.requested_claim_id(), CLAIM_ID, "{name}");
    }
}

fn assert_canonical_vector(
    outcome: &Sha256ClaimInlineBytesPredicateOutcomeV0,
    literal: &str,
    expected_len: usize,
    expected_sha256: &str,
) {
    let bytes = outcome.canonical_bytes();
    assert_eq!(bytes, literal.as_bytes());
    assert_eq!(serde_json::to_vec(outcome).unwrap(), bytes);
    assert_eq!(outcome.clone().canonical_bytes(), bytes);
    assert_eq!(bytes.len(), expected_len);
    assert_eq!(sha256_hex(&bytes), expected_sha256);
    assert_eq!(bytes.first(), Some(&b'{'));
    assert_eq!(bytes.last(), Some(&b'}'));
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!bytes.ends_with(b"\n"));
    assert!(!bytes.ends_with(b"\r\n"));
}

#[test]
fn all_three_canonical_vectors_match_exact_literal_bytes() {
    let equal_literal = r#"{"outcome":"digest_equal","details":{"receipt":{"predicate_schema":"magpie-machine-predicate-inline-bytes-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","scope_ref":"scope-s","canonical_statement":"magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad:616263","claim_content_hash":"aa1b8f1c339bc8e53c7db9d432e85073cc1495284279ba1b2474b74d26381223","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}}"#;
    assert_canonical_vector(
        &Fixture::abc_equal().outcome(),
        equal_literal,
        639,
        "d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a",
    );

    let unequal_literal = r#"{"outcome":"digest_unequal","details":{"receipt":{"predicate_schema":"magpie-machine-predicate-inline-bytes-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","scope_ref":"scope-s","canonical_statement":"magpie-machine-predicate-inline-bytes-v0:sha256_claim_inline_bytes_equals_v0:0000000000000000000000000000000000000000000000000000000000000000:616263","claim_content_hash":"1d5c96e14bb5116ef1dcf2481fac95a8b134cfc7434dc6054e69f919ca15ff93","expected_sha256":"0000000000000000000000000000000000000000000000000000000000000000","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"}}}"#;
    assert_canonical_vector(
        &Fixture::claim(ZERO_DIGEST, "616263").outcome(),
        unequal_literal,
        641,
        "cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291",
    );

    let failed = empty_snapshot().evaluate_sha256_claim_inline_bytes_equals_v0("claim-missing");
    let failed_literal = r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","reason":"missing_claim"}}"#;
    assert_canonical_vector(
        &failed,
        failed_literal,
        95,
        "b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d",
    );
}

#[test]
fn failure_serialization_uses_the_exact_json_escaping_law() {
    let query = "q\"\\\u{0008}\t\n\u{000c}\r\u{0001}/é/e\u{0301}";
    let outcome = empty_snapshot().evaluate_sha256_claim_inline_bytes_equals_v0(query);
    assert_eq!(outcome.requested_claim_id(), query);
    let expected = "{\"outcome\":\"resolution_failed\",\"details\":{\"claim_id\":\"q\\\"\\\\\\b\\t\\n\\f\\r\\u0001/é/e\u{0301}\",\"reason\":\"missing_claim\"}}";
    assert_eq!(outcome.canonical_bytes(), expected.as_bytes());
    assert!(!expected.contains("\\/"));

    let composed = empty_snapshot().evaluate_sha256_claim_inline_bytes_equals_v0("é");
    let decomposed = empty_snapshot().evaluate_sha256_claim_inline_bytes_equals_v0("e\u{0301}");
    assert_ne!(composed.canonical_bytes(), decomposed.canonical_bytes());
    assert!(String::from_utf8(composed.canonical_bytes())
        .unwrap()
        .contains("é"));
}

#[test]
fn failure_attribution_preserves_every_requested_query_id_without_a_bound() {
    let snapshot = empty_snapshot();
    let claim_a = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0("claim-a");
    let claim_b = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0("claim-b");
    assert_eq!(
        claim_a.failure(),
        Some(&ClaimInlineSubjectResolutionFailureV0::MissingClaim)
    );
    assert_eq!(claim_a.failure(), claim_b.failure());
    assert_ne!(claim_a.requested_claim_id(), claim_b.requested_claim_id());
    assert_ne!(claim_a.canonical_bytes(), claim_b.canonical_bytes());

    let long = "λ".repeat(10_000);
    for query in ["", "quote\"slash\\", "普通话", long.as_str()] {
        let outcome = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(query);
        assert_eq!(outcome.requested_claim_id(), query);
        assert_failure(
            &outcome,
            ClaimInlineSubjectResolutionFailureV0::MissingClaim,
        );
    }
}

#[test]
fn malformed_trailing_and_non_object_metadata_all_fail_as_malformed() {
    for metadata in [
        "{",
        "{} trailing",
        "[]",
        "null",
        r#"{"claim_domain":"ExactMachineCheckable"}{}"#,
        r#"{"claim_domain":"ExactMachineCheckable","unknown":[1,]}"#,
    ] {
        let mut fixture = Fixture::abc_equal();
        fixture.claim_metadata = metadata.into();
        assert_failure(
            &fixture.outcome(),
            ClaimInlineSubjectResolutionFailureV0::MalformedMetadata,
        );
    }
}

#[test]
fn digest_and_subject_spellings_are_strict_and_fail_before_comparison() {
    let expected_cases = [
        "A".repeat(64),
        format!("0x{}", "0".repeat(64)),
        format!(" {}", "0".repeat(64)),
        "0".repeat(63),
        "0".repeat(65),
    ];
    for expected in expected_cases {
        let mut fixture = Fixture::abc_equal();
        fixture.claim_metadata = inline_metadata(&expected, "XYZ");
        assert_failure(
            &fixture.outcome(),
            ClaimInlineSubjectResolutionFailureV0::InvalidExpectedSha256,
        );
    }

    let subject_cases = [
        "0".to_owned(),
        "AA".to_owned(),
        "0g".to_owned(),
        "0x00".to_owned(),
        " ".to_owned(),
    ];
    for subject in subject_cases {
        let mut fixture = Fixture::abc_equal();
        fixture.claim_metadata = inline_metadata(ABC_DIGEST, &subject);
        assert_failure(
            &fixture.outcome(),
            ClaimInlineSubjectResolutionFailureV0::InvalidSubjectHex,
        );
    }
}

#[test]
fn frozen_first_failure_order_ignores_source_order_and_double_faults() {
    use ClaimInlineSubjectResolutionFailureV0::*;

    let cases = [
        (
            r#"{"unknown":0} trailing"#,
            MalformedMetadata,
            "malformed before structural",
        ),
        (
            r#"{"unknown":0,"unknown":1}"#,
            MissingClaimDomain,
            "missing domain before duplicate",
        ),
        (
            r#"{"claim_domain":0,"claim_domain":"Nope"}"#,
            WrongTypeClaimDomain,
            "wrong domain type before duplicate",
        ),
        (
            r#"{"claim_domain":"Nope","unknown":0}"#,
            UnknownClaimDomain,
            "unknown domain before unknown outer",
        ),
        (
            r#"{"claim_domain":"Interpretation","unknown":0}"#,
            ClaimDomainMismatch,
            "domain mismatch before unknown outer",
        ),
        (
            r#"{"claim_domain":"ExactMachineCheckable","unknown":0}"#,
            UnknownClaimMetadataKey,
            "unknown outer before missing descriptor",
        ),
    ];
    for (metadata, expected, name) in cases {
        let mut fixture = Fixture::abc_equal();
        fixture.claim_metadata = metadata.into();
        assert_eq!(fixture.outcome().failure(), Some(&expected), "{name}");
    }

    let subject_first = format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"subject_hex":"XYZ","expected_sha256":"XYZ","predicate_id":"{PREDICATE_ID}","schema":"{SCHEMA}"}}}}"#
    );
    let mut fixture = Fixture::abc_equal();
    fixture.claim_metadata = subject_first;
    assert_eq!(
        fixture.outcome().failure(),
        Some(&InvalidExpectedSha256),
        "expected digest must win even when subject appears first"
    );
}

#[test]
fn ticket_0056_unrelated_evidence_and_contradiction_cannot_substitute_subject_bytes() {
    let mut attacked = Fixture::abc_equal();
    attacked.evidence = Some(EvidenceSpec {
        evidence_kind: "DeterministicVerification".into(),
        metadata_json: format!(
            r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"deadbeef"}}}}"#
        ),
    });
    attacked.edge = Some(EdgeSpec {
        edge_kind: "contradicts".into(),
    });

    let attacked_outcome = attacked.outcome();
    assert_eq!(
        attacked_outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );

    let mut altered = attacked.clone();
    altered.evidence.as_mut().unwrap().metadata_json = r#"{"arbitrary_bytes":"ffffffff"}"#.into();
    altered.edge = None;
    let altered_outcome = altered.outcome();

    let clean_outcome = Fixture::abc_equal().outcome();
    assert_eq!(
        attacked_outcome.canonical_bytes(),
        altered_outcome.canonical_bytes()
    );
    assert_eq!(
        attacked_outcome.canonical_bytes(),
        clean_outcome.canonical_bytes()
    );
}

fn historical_v0_fixture() -> Fixture {
    let statement = format!("sha256_bytes_equals_v0:{ABC_DIGEST}");
    Fixture {
        include_legacy_claim: false,
        include_typed_claim: true,
        legacy_statement: statement.clone(),
        typed_statement: statement.clone(),
        claim_content_hash: sha256_hex(statement.as_bytes()),
        claim_metadata: format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{ABC_DIGEST}"}}}}"#
        ),
        evidence: Some(EvidenceSpec {
            evidence_kind: "DeterministicVerification".into(),
            metadata_json: format!(
                r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
            ),
        }),
        edge: Some(EdgeSpec {
            edge_kind: "supports".into(),
        }),
    }
}

fn inline_fixture_with_historical_evidence() -> Fixture {
    let mut fixture = Fixture::abc_equal();
    fixture.evidence = Some(EvidenceSpec {
        evidence_kind: "DeterministicVerification".into(),
        metadata_json: format!(
            r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
        ),
    });
    fixture.edge = Some(EdgeSpec {
        edge_kind: "supports".into(),
    });
    fixture
}

#[test]
fn old_and_new_metadata_families_fail_closed_in_both_directions() {
    let historical = historical_v0_fixture();
    let new_outcome = historical
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_failure(
        &new_outcome,
        ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey,
    );

    let inline = inline_fixture_with_historical_evidence();
    assert_eq!(
        inline
            .snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::UnknownPredicateKey)
    );
}

#[test]
fn historical_evidence_local_v0_trace_and_bytes_are_unchanged() {
    let snapshot = historical_v0_fixture().snapshot();
    let before = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap();
    assert!(matches!(
        before,
        DeterministicVerifierContextTraceV0::Matched(_)
    ));
    let before_bytes = before.canonical_bytes();

    let inline_outcome = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_failure(
        &inline_outcome,
        ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey,
    );

    let after = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap();
    assert_eq!(after.canonical_bytes(), before_bytes);
}

#[test]
fn evaluation_is_snapshot_and_standing_v0_v1_v2_inert() {
    let snapshot = Fixture::abc_equal().snapshot();
    let snapshot_before = snapshot.canonical_bytes();
    let standing_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace(CLAIM_ID)
        .canonical_bytes();
    let v1_before = snapshot
        .resolved_standing_with_trace_v1(CLAIM_ID)
        .canonical_bytes();
    let v2_before = snapshot
        .resolved_standing_with_trace_v2(CLAIM_ID)
        .canonical_bytes();

    let outcome = snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );

    assert_eq!(snapshot.canonical_bytes(), snapshot_before);
    assert_eq!(snapshot.standing().canonical_bytes(), standing_before);
    assert_eq!(
        snapshot
            .standing()
            .resolved_standing_with_trace(CLAIM_ID)
            .canonical_bytes(),
        v0_before
    );
    assert_eq!(
        snapshot
            .resolved_standing_with_trace_v1(CLAIM_ID)
            .canonical_bytes(),
        v1_before
    );
    assert_eq!(
        snapshot
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
        v2_before
    );
}

#[test]
fn evaluation_is_standing_v3_and_policy_matrix_inert() {
    let context = Fixture::abc_equal().origin_context();
    let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
    let snapshot_before = context.snapshot().canonical_bytes();
    let v3_before = context
        .resolved_standing_with_trace_v3(CLAIM_ID, &closure)
        .canonical_bytes();
    let policy_before = (
        support_ceiling(
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExactMachineCheckable,
        ),
        refutation_ceiling(
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExactMachineCheckable,
        ),
        support_context_requirement(
            EvidenceKind::DeterministicVerification,
            ClaimDomain::ExactMachineCheckable,
        ),
    );

    let outcome = context
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(
        outcome.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );

    assert_eq!(context.snapshot().canonical_bytes(), snapshot_before);
    assert_eq!(
        context
            .resolved_standing_with_trace_v3(CLAIM_ID, &closure)
            .canonical_bytes(),
        v3_before
    );
    assert_eq!(
        (
            support_ceiling(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
            ),
            refutation_ceiling(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
            ),
            support_context_requirement(
                EvidenceKind::DeterministicVerification,
                ClaimDomain::ExactMachineCheckable,
            ),
        ),
        policy_before
    );
}

#[test]
fn predicate_identity_and_schema_version_boundaries_are_exact() {
    let historical_schema = envelope_json(
        r#""ExactMachineCheckable""#,
        &descriptor_json(
            r#""magpie-machine-predicate-v0""#,
            &format!(r#""{PREDICATE_ID}""#),
            &format!(r#""{ABC_DIGEST}""#),
            r#""616263""#,
        ),
    );
    let mut fixture = Fixture::abc_equal();
    fixture.claim_metadata = historical_schema;
    assert_failure(
        &fixture.outcome(),
        ClaimInlineSubjectResolutionFailureV0::UnknownSchema,
    );

    let historical_identity = envelope_json(
        r#""ExactMachineCheckable""#,
        &descriptor_json(
            &format!(r#""{SCHEMA}""#),
            r#""sha256_bytes_equals_v0""#,
            &format!(r#""{ABC_DIGEST}""#),
            r#""616263""#,
        ),
    );
    fixture.claim_metadata = historical_identity;
    assert_failure(
        &fixture.outcome(),
        ClaimInlineSubjectResolutionFailureV0::UnknownPredicateId,
    );
}
