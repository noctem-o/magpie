use magpie_claims::policy::{
    refutation_ceiling, support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
};
use magpie_claims::{
    replay_origin_admission_context_v0, replay_standing_context,
    ClaimInlineSubjectResolutionFailureV0, DeterministicVerifierContextTraceV0,
    InlinePredicateAttestationBindingFailureV0, InlinePredicateAttestationBindingOutcomeKindV0,
    InlinePredicateAttestationBindingOutcomeV0, OriginAdmissionReplayContextV0,
    ResolutionContentClosureV0, Sha256ClaimInlineBytesPredicateOutcomeKindV0,
    StandingReplaySnapshot, MAX_INLINE_SUBJECT_BYTES_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [61; 32];
const CLAIM_ID: &str = "claim-c";
const EVIDENCE_ID: &str = "evidence-e";
const SECOND_EVIDENCE_ID: &str = "evidence-second";
const EDGE_ID: &str = "edge-j";
const SCOPE: &str = "scope-s";
const CLAIM_SCHEMA: &str = "magpie-machine-predicate-inline-bytes-v0";
const PREDICATE_ID: &str = "sha256_claim_inline_bytes_equals_v0";
const ATTESTATION_SCHEMA: &str = "magpie-inline-predicate-attestation-v0";
const ABC_DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const ZERO_DIGEST: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut timestamp = 0u64;
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
    Provenance::new("test", "inline-predicate-attestation")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn canonical_claim_statement(expected_sha256: &str, subject_hex: &str) -> String {
    format!("{CLAIM_SCHEMA}:{PREDICATE_ID}:{expected_sha256}:{subject_hex}")
}

fn claim_metadata(expected_sha256: &str, subject_hex: &str) -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{expected_sha256}","subject_hex":"{subject_hex}"}}}}"#
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

fn claim_envelope_json(domain_value: &str, descriptor_value: &str) -> String {
    format!(
        r#"{{"claim_domain":{domain_value},"machine_predicate_inline_bytes":{descriptor_value}}}"#
    )
}

fn attestation_metadata(
    schema_value: &str,
    predicate_value: &str,
    claim_value: &str,
    scope_value: &str,
) -> String {
    format!(
        r#"{{"inline_predicate_attestation":{{"schema":{schema_value},"predicate_id":{predicate_value},"subject_claim_id":{claim_value},"scope_ref":{scope_value}}}}}"#
    )
}

fn valid_attestation_for(claim_id: &str, scope_ref: &str) -> String {
    attestation_metadata(
        &format!(r#""{ATTESTATION_SCHEMA}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{claim_id}""#),
        &format!(r#""{scope_ref}""#),
    )
}

#[derive(Clone)]
struct TypedClaimSpec {
    statement: String,
    scope_ref: String,
    content_hash: String,
    metadata_json: String,
}

#[derive(Clone)]
struct EvidenceSpec {
    evidence_id: String,
    evidence_kind: String,
    summary: String,
    scope_ref: String,
    actor_class: String,
    content_hash: String,
    metadata_json: String,
}

#[derive(Clone)]
struct EdgeSpec {
    edge_id: String,
    edge_kind: String,
    source_id: String,
}

#[derive(Clone)]
struct Fixture {
    legacy_statement: Option<String>,
    typed_claim: Option<TypedClaimSpec>,
    evidence: Vec<EvidenceSpec>,
    edges: Vec<EdgeSpec>,
}

impl Fixture {
    fn valid_with_digest(expected_sha256: &str) -> Self {
        let subject_hex = "616263";
        let statement = canonical_claim_statement(expected_sha256, subject_hex);
        Self {
            legacy_statement: None,
            typed_claim: Some(TypedClaimSpec {
                statement: statement.clone(),
                scope_ref: SCOPE.into(),
                content_hash: sha256_hex(statement.as_bytes()),
                metadata_json: claim_metadata(expected_sha256, subject_hex),
            }),
            evidence: vec![EvidenceSpec {
                evidence_id: EVIDENCE_ID.into(),
                evidence_kind: "DeterministicVerification".into(),
                summary: "routing evidence".into(),
                scope_ref: SCOPE.into(),
                actor_class: "AutomatedVerifier".into(),
                content_hash: String::new(),
                metadata_json: valid_attestation_for(CLAIM_ID, SCOPE),
            }],
            edges: Vec::new(),
        }
    }

    fn valid() -> Self {
        Self::valid_with_digest(ABC_DIGEST)
    }

    fn records(&self) -> Vec<Vec<u8>> {
        let store = MemStore::new();
        {
            let mut log = writer(store.clone());
            if let Some(statement) = &self.legacy_statement {
                log.append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: CLAIM_ID.into(),
                        statement: statement.clone(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            }
            if let Some(claim) = &self.typed_claim {
                log.append(
                    provenance(),
                    Payload::ClaimAssertedV2 {
                        claim_id: CLAIM_ID.into(),
                        statement: claim.statement.clone(),
                        scope_ref: claim.scope_ref.clone(),
                        actor_class: "AgentProposer".into(),
                        content_hash: claim.content_hash.clone(),
                        metadata_json: claim.metadata_json.clone(),
                    },
                )
                .unwrap();
            }
            for evidence in &self.evidence {
                log.append(
                    provenance(),
                    Payload::EvidenceRegistered {
                        evidence_id: evidence.evidence_id.clone(),
                        evidence_kind: evidence.evidence_kind.clone(),
                        summary: evidence.summary.clone(),
                        scope_ref: evidence.scope_ref.clone(),
                        actor_class: evidence.actor_class.clone(),
                        content_hash: evidence.content_hash.clone(),
                        metadata_json: evidence.metadata_json.clone(),
                    },
                )
                .unwrap();
            }
            for edge in &self.edges {
                log.append(
                    provenance(),
                    Payload::JustificationEdgeRecorded {
                        edge_id: edge.edge_id.clone(),
                        edge_kind: edge.edge_kind.clone(),
                        source_id: edge.source_id.clone(),
                        target_id: CLAIM_ID.into(),
                        scope_ref: SCOPE.into(),
                        actor_class: "HumanRoot".into(),
                        rationale: "must remain outside routing authority".into(),
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

    fn outcome(&self) -> InlinePredicateAttestationBindingOutcomeV0 {
        self.snapshot()
            .resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID)
    }
}

fn assert_failure(
    outcome: &InlinePredicateAttestationBindingOutcomeV0,
    expected: InlinePredicateAttestationBindingFailureV0,
) {
    assert_eq!(
        outcome.kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::ResolutionFailed
    );
    assert_eq!(outcome.failure(), Some(&expected));
    assert!(outcome.receipt().is_none());
}

fn with_evidence_metadata(metadata_json: impl Into<String>) -> Fixture {
    let mut fixture = Fixture::valid();
    fixture.evidence[0].metadata_json = metadata_json.into();
    fixture
}

#[test]
fn exact_valid_attestation_is_bound_with_only_the_five_field_receipt() {
    let fixture = Fixture::valid();
    let snapshot = fixture.snapshot();
    let first = snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID);
    let second = snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID);

    assert_eq!(
        first.kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert!(first.failure().is_none());
    let receipt = first.receipt().unwrap();
    assert_eq!(receipt.schema(), ATTESTATION_SCHEMA);
    assert_eq!(receipt.predicate_id(), PREDICATE_ID);
    assert_eq!(receipt.claim_id(), CLAIM_ID);
    assert_eq!(receipt.evidence_id(), EVIDENCE_ID);
    assert_eq!(receipt.scope_ref(), SCOPE);
    assert!(std::ptr::eq(
        first.requested_claim_id().as_ptr(),
        receipt.claim_id().as_ptr()
    ));
    assert!(std::ptr::eq(
        first.requested_evidence_id().as_ptr(),
        receipt.evidence_id().as_ptr()
    ));
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(first.canonical_bytes(), fixture.outcome().canonical_bytes());
}

#[test]
fn source_order_and_escaped_exact_spellings_do_not_change_bound_bytes() {
    let direct = Fixture::valid().outcome();
    let escaped = r#"{"inline_predicate_\u0061ttestation":{"scope_\u0072ef":"scope-\u0073","subject_claim_\u0069d":"claim-\u0063","pred\u0069cate_id":"sha256_claim_inline_bytes_equals_v\u0030","sch\u0065ma":"magpie-inline-predicate-attestation-v\u0030"}}"#;
    let escaped_outcome = with_evidence_metadata(escaped).outcome();
    let reordered = r#"{"inline_predicate_attestation":{"scope_ref":"scope-s","subject_claim_id":"claim-c","predicate_id":"sha256_claim_inline_bytes_equals_v0","schema":"magpie-inline-predicate-attestation-v0"}}"#;
    let reordered_outcome = with_evidence_metadata(reordered).outcome();

    assert_eq!(
        escaped_outcome.kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert_eq!(escaped_outcome.canonical_bytes(), direct.canonical_bytes());
    assert_eq!(
        reordered_outcome.canonical_bytes(),
        direct.canonical_bytes()
    );
}

#[test]
fn digest_equal_and_digest_unequal_claims_are_equally_eligible_for_bound() {
    let equal_fixture = Fixture::valid_with_digest(ABC_DIGEST);
    let unequal_fixture = Fixture::valid_with_digest(ZERO_DIGEST);

    assert_eq!(
        equal_fixture
            .snapshot()
            .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID)
            .kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );
    assert_eq!(
        unequal_fixture
            .snapshot()
            .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID)
            .kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestUnequal
    );
    assert_eq!(
        equal_fixture.outcome().kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert_eq!(
        unequal_fixture.outcome().kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert_eq!(
        equal_fixture.outcome().canonical_bytes(),
        unequal_fixture.outcome().canonical_bytes()
    );
}

#[test]
fn every_outer_failure_is_reachable_with_exact_classification() {
    use InlinePredicateAttestationBindingFailureV0::*;

    let mut missing_claim = Fixture::valid();
    missing_claim.typed_claim = None;
    assert_failure(
        &missing_claim.outcome(),
        ClaimPredicateResolutionFailed(ClaimInlineSubjectResolutionFailureV0::MissingClaim),
    );

    let mut missing_evidence = Fixture::valid();
    missing_evidence.evidence.clear();
    assert_failure(&missing_evidence.outcome(), MissingEvidence);

    let mut wrong_kind = Fixture::valid();
    wrong_kind.evidence[0].evidence_kind = "ExecutionEvidence".into();
    assert_failure(&wrong_kind.outcome(), WrongEvidenceKind);

    let cases = [
        ("{", MalformedAttestationMetadata),
        (
            r#"{"inline_predicate_attestation":{},"inline_predicate_\u0061ttestation":{}}"#,
            DuplicateAttestationMetadataKey,
        ),
        (
            r#"{"inline_predicate_attestation":{},"verification_witness":{}}"#,
            UnknownAttestationMetadataKey,
        ),
        ("{}", MissingInlinePredicateAttestation),
        (
            r#"{"inline_predicate_attestation":null}"#,
            WrongTypeInlinePredicateAttestation,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"x","sch\u0065ma":"y"}}"#,
            DuplicateAttestationField,
        ),
        (
            r#"{"inline_predicate_attestation":{"unknown":null}}"#,
            UnknownAttestationField,
        ),
        (
            r#"{"inline_predicate_attestation":{"predicate_id":"x","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            MissingSchema,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":null,"predicate_id":"x","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            WrongTypeSchema,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"unknown","predicate_id":"x","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            UnknownSchema,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            MissingPredicateId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":null,"subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            WrongTypePredicateId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            EmptyPredicateId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            PredicateIdTooLong,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"invalid-id","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            InvalidPredicateId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","scope_ref":"scope-s"}}"#,
            MissingSubjectClaimId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":null,"scope_ref":"scope-s"}}"#,
            WrongTypeSubjectClaimId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"","scope_ref":"scope-s"}}"#,
            EmptySubjectClaimId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c"}}"#,
            MissingScopeRef,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":null}}"#,
            WrongTypeScopeRef,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":""}}"#,
            EmptyScopeRef,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v1","subject_claim_id":"claim-c","scope_ref":"scope-s"}}"#,
            PredicateBindingMismatch,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-other","scope_ref":"scope-s"}}"#,
            ClaimBindingMismatch,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-c","scope_ref":"scope-other"}}"#,
            EvidenceScopeBindingMismatch,
        ),
    ];
    for (metadata, expected) in cases {
        assert_failure(&with_evidence_metadata(metadata).outcome(), expected);
    }

    let mut claim_scope_mismatch = Fixture::valid();
    claim_scope_mismatch.typed_claim.as_mut().unwrap().scope_ref = "scope-claim".into();
    assert_failure(&claim_scope_mismatch.outcome(), ClaimScopeBindingMismatch);
}

#[test]
fn frozen_failure_precedence_wins_over_all_later_double_faults() {
    use InlinePredicateAttestationBindingFailureV0::*;

    let mut claim_before_evidence = Fixture::valid();
    claim_before_evidence
        .typed_claim
        .as_mut()
        .unwrap()
        .metadata_json = "{".into();
    claim_before_evidence.evidence.clear();
    assert_failure(
        &claim_before_evidence.outcome(),
        ClaimPredicateResolutionFailed(ClaimInlineSubjectResolutionFailureV0::MalformedMetadata),
    );

    let mut missing_before_other_evidence_kind = Fixture::valid();
    missing_before_other_evidence_kind.evidence[0].evidence_id = "different-evidence".into();
    missing_before_other_evidence_kind.evidence[0].evidence_kind = "ExecutionEvidence".into();
    assert_failure(
        &missing_before_other_evidence_kind.outcome(),
        MissingEvidence,
    );

    let mut kind_before_metadata = Fixture::valid();
    kind_before_metadata.evidence[0].evidence_kind = "ExecutionEvidence".into();
    kind_before_metadata.evidence[0].metadata_json = "{".into();
    assert_failure(&kind_before_metadata.outcome(), WrongEvidenceKind);

    let cases = [
        (
            r#"{"unknown":[1,],"inline_predicate_attestation":{},"inline_predicate_attestation":{}}"#,
            MalformedAttestationMetadata,
        ),
        (
            r#"{"unknown":0,"inline_predicate_attestation":{},"inline_predicate_attestation":{}}"#,
            DuplicateAttestationMetadataKey,
        ),
        (r#"{"unknown":0}"#, UnknownAttestationMetadataKey),
        (
            r#"{"inline_predicate_attestation":{"unknown":0,"schema":"x","schema":"y"}}"#,
            DuplicateAttestationField,
        ),
        (
            r#"{"inline_predicate_attestation":{"unknown":0}}"#,
            UnknownAttestationField,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"wrong","predicate_id":"","subject_claim_id":"","scope_ref":""}}"#,
            UnknownSchema,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"","subject_claim_id":"","scope_ref":""}}"#,
            EmptyPredicateId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"","scope_ref":""}}"#,
            EmptySubjectClaimId,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v1","subject_claim_id":"claim-other","scope_ref":"scope-other"}}"#,
            PredicateBindingMismatch,
        ),
        (
            r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","subject_claim_id":"claim-other","scope_ref":"scope-other"}}"#,
            ClaimBindingMismatch,
        ),
    ];
    for (metadata, expected) in cases {
        assert_failure(&with_evidence_metadata(metadata).outcome(), expected);
    }

    let mut evidence_before_claim_scope =
        with_evidence_metadata(valid_attestation_for(CLAIM_ID, "scope-attestation"));
    evidence_before_claim_scope.evidence[0].scope_ref = "scope-evidence".into();
    evidence_before_claim_scope
        .typed_claim
        .as_mut()
        .unwrap()
        .scope_ref = "scope-claim".into();
    assert_failure(
        &evidence_before_claim_scope.outcome(),
        EvidenceScopeBindingMismatch,
    );
}

#[test]
fn hostile_json_and_smuggled_fields_fail_closed() {
    use InlinePredicateAttestationBindingFailureV0::*;

    for metadata in [
        "",
        "[]",
        "null",
        "1",
        "{} trailing",
        r#"{"inline_predicate_attestation":[1,]}"#,
        r#"{"inline_predicate_attestation":{"schema":"\q"}}"#,
        r#"{"inline_predicate_attestation":{"schema":"\uD800"}}"#,
        r#"{"inline_predicate_attestation":{"schema":"\uDC00"}}"#,
        r#"{"inline_predicate_attestation":{"schema":1.}}"#,
        r#"{"inline_predicate_attestation":{"schema":tru}}"#,
    ] {
        assert_failure(
            &with_evidence_metadata(metadata).outcome(),
            MalformedAttestationMetadata,
        );
    }

    for key in [
        "byte",
        "digest",
        "relation",
        "edge",
        "polarity",
        "outcome",
        "policy",
        "standing",
        "contribution",
        "verification_witness",
    ] {
        let metadata = format!(
            r#"{{"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","{key}":null}}}}"#
        );
        assert_failure(
            &with_evidence_metadata(metadata).outcome(),
            UnknownAttestationField,
        );
    }

    let valid_complex_unknown_outer = format!(
        r#"{{"unknown":[-12.5e+7,true,null,{{"nested":"\uD83D\uDE03"}}],"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}"}}}}"#
    );
    assert_failure(
        &with_evidence_metadata(valid_complex_unknown_outer).outcome(),
        UnknownAttestationMetadataKey,
    );
    let valid_complex_unknown_inner = format!(
        r#"{{"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","unknown":[-0,1.0e-2,false,null,{{}}]}}}}"#
    );
    assert_failure(
        &with_evidence_metadata(valid_complex_unknown_inner).outcome(),
        UnknownAttestationField,
    );

    let post_unescape_outer_duplicate =
        r#"{"unknown😃":0,"unknown\uD83D\uDE03":1,"inline_predicate_attestation":{}}"#;
    assert_failure(
        &with_evidence_metadata(post_unescape_outer_duplicate).outcome(),
        DuplicateAttestationMetadataKey,
    );
    let post_unescape_inner_duplicate = format!(
        r#"{{"inline_predicate_attestation":{{"unknown😃":0,"unknown\uD83D\uDE03":1,"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}"}}}}"#
    );
    assert_failure(
        &with_evidence_metadata(post_unescape_inner_duplicate).outcome(),
        DuplicateAttestationField,
    );
}

#[test]
fn predicate_length_boundaries_and_long_borrowed_mismatches_keep_frozen_failures() {
    use InlinePredicateAttestationBindingFailureV0::*;

    for length in [63usize, 64] {
        let predicate = "a".repeat(length);
        let fixture = with_evidence_metadata(attestation_metadata(
            &format!(r#""{ATTESTATION_SCHEMA}""#),
            &format!(r#""{predicate}""#),
            &format!(r#""{CLAIM_ID}""#),
            &format!(r#""{SCOPE}""#),
        ));
        assert_failure(&fixture.outcome(), PredicateBindingMismatch);
    }
    for predicate_json in [
        format!(r#""{}""#, "a".repeat(65)),
        format!(r#""{}""#, r#"\u0061"#.repeat(65)),
        format!(r#""{}a""#, "é".repeat(32)),
    ] {
        let fixture = with_evidence_metadata(attestation_metadata(
            &format!(r#""{ATTESTATION_SCHEMA}""#),
            &predicate_json,
            &format!(r#""{CLAIM_ID}""#),
            &format!(r#""{SCOPE}""#),
        ));
        assert_failure(&fixture.outcome(), PredicateIdTooLong);
    }
    for predicate_json in [
        format!(r#""{}""#, "é".repeat(32)),
        format!(r#""{}""#, r#"\u00e9"#.repeat(32)),
    ] {
        let fixture = with_evidence_metadata(attestation_metadata(
            &format!(r#""{ATTESTATION_SCHEMA}""#),
            &predicate_json,
            &format!(r#""{CLAIM_ID}""#),
            &format!(r#""{SCOPE}""#),
        ));
        assert_failure(&fixture.outcome(), InvalidPredicateId);
    }

    let long = "x".repeat(100_000);
    let schema = with_evidence_metadata(attestation_metadata(
        &format!(r#""{long}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{CLAIM_ID}""#),
        &format!(r#""{SCOPE}""#),
    ));
    assert_failure(&schema.outcome(), UnknownSchema);
    let claim = with_evidence_metadata(attestation_metadata(
        &format!(r#""{ATTESTATION_SCHEMA}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{long}""#),
        &format!(r#""{SCOPE}""#),
    ));
    assert_failure(&claim.outcome(), ClaimBindingMismatch);
    let scope = with_evidence_metadata(attestation_metadata(
        &format!(r#""{ATTESTATION_SCHEMA}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{CLAIM_ID}""#),
        &format!(r#""{long}""#),
    ));
    assert_failure(&scope.outcome(), EvidenceScopeBindingMismatch);
}

fn historical_fixture(evidence_metadata: String) -> Fixture {
    let statement = format!("sha256_bytes_equals_v0:{ABC_DIGEST}");
    Fixture {
        legacy_statement: None,
        typed_claim: Some(TypedClaimSpec {
            statement: statement.clone(),
            scope_ref: SCOPE.into(),
            content_hash: sha256_hex(statement.as_bytes()),
            metadata_json: format!(
                r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{ABC_DIGEST}"}}}}"#
            ),
        }),
        evidence: vec![EvidenceSpec {
            evidence_id: EVIDENCE_ID.into(),
            evidence_kind: "DeterministicVerification".into(),
            summary: "historical witness".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AutomatedVerifier".into(),
            content_hash: String::new(),
            metadata_json: evidence_metadata,
        }],
        edges: vec![EdgeSpec {
            edge_id: EDGE_ID.into(),
            edge_kind: "supports".into(),
            source_id: EVIDENCE_ID.into(),
        }],
    }
}

fn historical_witness_metadata() -> String {
    format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
    )
}

#[test]
fn metadata_families_refuse_cross_dispatch_in_both_directions() {
    let new_against_old_witness = with_evidence_metadata(historical_witness_metadata());
    assert_failure(
        &new_against_old_witness.outcome(),
        InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey,
    );

    let historical = historical_fixture(historical_witness_metadata());
    assert!(matches!(
        historical
            .snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));

    let old_against_new_attestation = historical_fixture(valid_attestation_for(CLAIM_ID, SCOPE));
    assert_eq!(
        old_against_new_attestation
            .snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::UnknownWitnessKey)
    );

    let both = format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}},"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}"}}}}"#
    );
    assert_failure(
        &with_evidence_metadata(both.clone()).outcome(),
        InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey,
    );
    assert_eq!(
        historical_fixture(both)
            .snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::UnknownWitnessKey)
    );

    assert_failure(
        &with_evidence_metadata("{}").outcome(),
        InlinePredicateAttestationBindingFailureV0::MissingInlinePredicateAttestation,
    );
    assert_eq!(
        historical_fixture("{}".into())
            .snapshot()
            .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::MissingVerificationWitness)
    );
}

#[test]
fn edges_and_evidence_non_authority_fields_do_not_change_binding_bytes() {
    let clean = Fixture::valid().outcome().canonical_bytes();
    for edge_kinds in [
        Vec::<&str>::new(),
        vec!["supports"],
        vec!["contradicts"],
        vec!["supports", "contradicts"],
    ] {
        let mut fixture = Fixture::valid();
        fixture.edges = edge_kinds
            .iter()
            .enumerate()
            .map(|(index, edge_kind)| EdgeSpec {
                edge_id: format!("edge-{index}"),
                edge_kind: (*edge_kind).into(),
                source_id: EVIDENCE_ID.into(),
            })
            .collect();
        assert_eq!(fixture.outcome().canonical_bytes(), clean);
    }

    let variations = [
        ("summary-zero", "AutomatedVerifier", ""),
        ("summary-a", "HumanRoot", ABC_DIGEST),
        ("summary-b", "SourceImporter", ZERO_DIGEST),
    ];
    for (summary, actor_class, content_hash) in variations {
        let mut fixture = Fixture::valid();
        fixture.evidence[0].summary = summary.into();
        fixture.evidence[0].actor_class = actor_class.into();
        fixture.evidence[0].content_hash = content_hash.into();
        assert_eq!(fixture.outcome().canonical_bytes(), clean);
    }
}

#[test]
fn substitutions_multiple_evidence_and_all_scope_orders_are_exact() {
    let substitution = with_evidence_metadata(valid_attestation_for("claim-other", SCOPE));
    assert_failure(
        &substitution.outcome(),
        InlinePredicateAttestationBindingFailureV0::ClaimBindingMismatch,
    );

    let mut multiple = Fixture::valid();
    let mut second = multiple.evidence[0].clone();
    second.evidence_id = SECOND_EVIDENCE_ID.into();
    multiple.evidence.push(second);
    let snapshot = multiple.snapshot();
    let first = snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID);
    let second = snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, SECOND_EVIDENCE_ID);
    assert_eq!(
        first.kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert_eq!(
        second.kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );
    assert_eq!(second.receipt().unwrap().evidence_id(), SECOND_EVIDENCE_ID);
    assert_ne!(first.canonical_bytes(), second.canonical_bytes());

    let cases = [
        (
            "scope-attestation",
            "scope-evidence",
            "scope-claim",
            InlinePredicateAttestationBindingFailureV0::EvidenceScopeBindingMismatch,
        ),
        (
            "scope-attestation",
            "scope-attestation",
            "scope-claim",
            InlinePredicateAttestationBindingFailureV0::ClaimScopeBindingMismatch,
        ),
        (
            "scope-attestation",
            "scope-evidence",
            "scope-attestation",
            InlinePredicateAttestationBindingFailureV0::EvidenceScopeBindingMismatch,
        ),
        (
            "scope-attestation",
            "scope-other",
            "scope-other",
            InlinePredicateAttestationBindingFailureV0::EvidenceScopeBindingMismatch,
        ),
    ];
    for (attestation_scope, evidence_scope, claim_scope, expected) in cases {
        let mut fixture =
            with_evidence_metadata(valid_attestation_for(CLAIM_ID, attestation_scope));
        fixture.evidence[0].scope_ref = evidence_scope.into();
        fixture.typed_claim.as_mut().unwrap().scope_ref = claim_scope.into();
        assert_failure(&fixture.outcome(), expected);
    }
}

fn assert_vector(
    outcome: &InlinePredicateAttestationBindingOutcomeV0,
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
}

#[test]
fn all_three_attestation_vectors_and_ticket_0059_vectors_are_byte_exact() {
    let bound_literal = r#"{"outcome":"bound","details":{"receipt":{"schema":"magpie-inline-predicate-attestation-v0","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","scope_ref":"scope-s"}}}"#;
    assert_vector(
        &Fixture::valid().outcome(),
        bound_literal,
        216,
        "9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5",
    );

    let mut missing = Fixture::valid();
    missing.evidence.clear();
    let missing_outcome = missing
        .snapshot()
        .resolve_inline_predicate_attestation_v0(CLAIM_ID, "evidence-missing");
    let missing_literal = r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-missing","reason":"missing_evidence"}}"#;
    assert_vector(
        &missing_outcome,
        missing_literal,
        125,
        "968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6",
    );

    let nested_outcome = missing
        .snapshot()
        .resolve_inline_predicate_attestation_v0("claim-missing", EVIDENCE_ID);
    let nested_literal = r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}"#;
    assert_vector(
        &nested_outcome,
        nested_literal,
        173,
        "2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190",
    );

    let equal = Fixture::valid()
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(equal.canonical_bytes().len(), 639);
    assert_eq!(
        sha256_hex(&equal.canonical_bytes()),
        "d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a"
    );
    let unequal = Fixture::valid_with_digest(ZERO_DIGEST)
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(unequal.canonical_bytes().len(), 641);
    assert_eq!(
        sha256_hex(&unequal.canonical_bytes()),
        "cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291"
    );
    let missing_claim = missing
        .snapshot()
        .evaluate_sha256_claim_inline_bytes_equals_v0("claim-missing");
    assert_eq!(missing_claim.canonical_bytes().len(), 95);
    assert_eq!(
        sha256_hex(&missing_claim.canonical_bytes()),
        "b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d"
    );
}

#[test]
fn failed_query_attribution_and_json_escaping_preserve_both_exact_ids() {
    let snapshot = Fixture::valid().snapshot();
    let claim_query = "q\"\\\u{0008}\t\n\u{000c}\r\u{0001}/é/e\u{0301}";
    let evidence_query = "e\"\\\u{0008}\t\n\u{000c}\r\u{0002}/普通话";
    let outcome = snapshot.resolve_inline_predicate_attestation_v0(claim_query, evidence_query);
    assert_eq!(outcome.requested_claim_id(), claim_query);
    assert_eq!(outcome.requested_evidence_id(), evidence_query);
    assert_failure(
        &outcome,
        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
            ClaimInlineSubjectResolutionFailureV0::MissingClaim,
        ),
    );
    let json = String::from_utf8(outcome.canonical_bytes()).unwrap();
    let expected_claim_escape = format!("{}e\u{0301}", r#"\b\t\n\f\r\u0001/é/"#);
    assert!(json.contains(&expected_claim_escape));
    assert!(json.contains(r#"\b\t\n\f\r\u0002/普通话"#));
    assert!(!json.contains("\\/"));

    let long_claim = "λ".repeat(10_000);
    let long_evidence = " evidence ".repeat(10_000);
    let long = snapshot.resolve_inline_predicate_attestation_v0(&long_claim, &long_evidence);
    assert_eq!(long.requested_claim_id(), long_claim);
    assert_eq!(long.requested_evidence_id(), long_evidence);
}

#[test]
fn resolver_is_snapshot_policy_and_standing_inert() {
    let fixture = Fixture::valid();
    let context = fixture.origin_context();
    let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
    let snapshot_before = context.snapshot().canonical_bytes();
    let standing_before = context.snapshot().standing().canonical_bytes();
    let v0_before = context
        .snapshot()
        .standing()
        .resolved_standing_with_trace(CLAIM_ID)
        .canonical_bytes();
    let v1_before = context
        .snapshot()
        .resolved_standing_with_trace_v1(CLAIM_ID)
        .canonical_bytes();
    let v2_before = context
        .snapshot()
        .resolved_standing_with_trace_v2(CLAIM_ID)
        .canonical_bytes();
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

    assert_eq!(
        context
            .snapshot()
            .resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID)
            .kind(),
        InlinePredicateAttestationBindingOutcomeKindV0::Bound
    );

    assert_eq!(context.snapshot().canonical_bytes(), snapshot_before);
    assert_eq!(
        context.snapshot().standing().canonical_bytes(),
        standing_before
    );
    assert_eq!(
        context
            .snapshot()
            .standing()
            .resolved_standing_with_trace(CLAIM_ID)
            .canonical_bytes(),
        v0_before
    );
    assert_eq!(
        context
            .snapshot()
            .resolved_standing_with_trace_v1(CLAIM_ID)
            .canonical_bytes(),
        v1_before
    );
    assert_eq!(
        context
            .snapshot()
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
        v2_before
    );
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
fn historical_verifier_bytes_are_unchanged_by_attempting_the_new_route() {
    let snapshot = historical_fixture(historical_witness_metadata()).snapshot();
    let before = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap()
        .canonical_bytes();
    assert_failure(
        &snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID),
        InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
            ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey,
        ),
    );
    let after = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, EDGE_ID)
        .unwrap()
        .canonical_bytes();
    assert_eq!(after, before);
}

#[test]
fn nested_claim_failures_are_wrapped_without_flattening_or_reordering() {
    use ClaimInlineSubjectResolutionFailureV0 as ClaimFailure;
    use InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed;

    let mut missing_typed = Fixture::valid();
    let statement = missing_typed
        .typed_claim
        .as_ref()
        .unwrap()
        .statement
        .clone();
    missing_typed.legacy_statement = Some(statement);
    missing_typed.typed_claim = None;
    assert_failure(
        &missing_typed.outcome(),
        ClaimPredicateResolutionFailed(ClaimFailure::MissingTypedClaim),
    );

    let valid_descriptor = descriptor_json(
        &format!(r#""{CLAIM_SCHEMA}""#),
        &format!(r#""{PREDICATE_ID}""#),
        &format!(r#""{ABC_DIGEST}""#),
        r#""616263""#,
    );
    let overlong_predicate = "a".repeat(65);
    let overlong_subject = "0".repeat(MAX_INLINE_SUBJECT_BYTES_V0 * 2 + 2);
    let parser_cases: Vec<(String, ClaimFailure)> = vec![
        ("{".into(), ClaimFailure::MalformedMetadata),
        (
            format!(r#"{{"machine_predicate_inline_bytes":{valid_descriptor}}}"#),
            ClaimFailure::MissingClaimDomain,
        ),
        (
            claim_envelope_json("0", &valid_descriptor),
            ClaimFailure::WrongTypeClaimDomain,
        ),
        (
            format!(
                r#"{{"claim_domain":"ExactMachineCheckable","cla\u0069m_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{valid_descriptor}}}"#
            ),
            ClaimFailure::DuplicateClaimMetadataKey,
        ),
        (
            claim_envelope_json(r#""UnknownDomain""#, &valid_descriptor),
            ClaimFailure::UnknownClaimDomain,
        ),
        (
            claim_envelope_json(r#""Interpretation""#, &valid_descriptor),
            ClaimFailure::ClaimDomainMismatch,
        ),
        (
            format!(
                r#"{{"claim_domain":"ExactMachineCheckable","unknown":0,"machine_predicate_inline_bytes":{valid_descriptor}}}"#
            ),
            ClaimFailure::UnknownClaimMetadataKey,
        ),
        (
            r#"{"claim_domain":"ExactMachineCheckable"}"#.into(),
            ClaimFailure::MissingInlineSubjectDescriptor,
        ),
        (
            claim_envelope_json(r#""ExactMachineCheckable""#, "0"),
            ClaimFailure::WrongTypeInlineSubjectDescriptor,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"schema":"{CLAIM_SCHEMA}","schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            ClaimFailure::DuplicateDescriptorKey,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263","unknown":0}}"#
                ),
            ),
            ClaimFailure::UnknownDescriptorKey,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            ClaimFailure::MissingSchema,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    "0",
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::WrongTypeSchema,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    r#""unknown""#,
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::UnknownSchema,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"schema":"{CLAIM_SCHEMA}","expected_sha256":"{ABC_DIGEST}","subject_hex":"616263"}}"#
                ),
            ),
            ClaimFailure::MissingPredicateId,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    "0",
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::WrongTypePredicateId,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    r#""""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::EmptyPredicateId,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{overlong_predicate}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::PredicateIdTooLong,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    r#""invalid-id""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::InvalidPredicateId,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    r#""sha256_bytes_equals_v0""#,
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""616263""#,
                ),
            ),
            ClaimFailure::UnknownPredicateId,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_hex":"616263"}}"#
                ),
            ),
            ClaimFailure::MissingExpectedSha256,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    "0",
                    r#""616263""#,
                ),
            ),
            ClaimFailure::WrongTypeExpectedSha256,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    r#""invalid""#,
                    r#""616263""#,
                ),
            ),
            ClaimFailure::InvalidExpectedSha256,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &format!(
                    r#"{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{ABC_DIGEST}"}}"#
                ),
            ),
            ClaimFailure::MissingSubjectHex,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    "0",
                ),
            ),
            ClaimFailure::WrongTypeSubjectHex,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    &format!(r#""{overlong_subject}""#),
                ),
            ),
            ClaimFailure::SubjectHexTooLong,
        ),
        (
            claim_envelope_json(
                r#""ExactMachineCheckable""#,
                &descriptor_json(
                    &format!(r#""{CLAIM_SCHEMA}""#),
                    &format!(r#""{PREDICATE_ID}""#),
                    &format!(r#""{ABC_DIGEST}""#),
                    r#""XYZ""#,
                ),
            ),
            ClaimFailure::InvalidSubjectHex,
        ),
    ];
    assert_eq!(parser_cases.len(), 27);
    for (metadata, expected) in parser_cases {
        let mut fixture = Fixture::valid();
        fixture.typed_claim.as_mut().unwrap().metadata_json = metadata;
        assert_failure(&fixture.outcome(), ClaimPredicateResolutionFailed(expected));
    }

    let mut statement = Fixture::valid();
    statement.typed_claim.as_mut().unwrap().statement = "wrong".into();
    assert_failure(
        &statement.outcome(),
        ClaimPredicateResolutionFailed(ClaimFailure::StatementBindingMismatch),
    );

    let mut missing_hash = Fixture::valid();
    missing_hash
        .typed_claim
        .as_mut()
        .unwrap()
        .content_hash
        .clear();
    assert_failure(
        &missing_hash.outcome(),
        ClaimPredicateResolutionFailed(ClaimFailure::MissingClaimContentHash),
    );

    let mut wrong_hash = Fixture::valid();
    wrong_hash.typed_claim.as_mut().unwrap().content_hash = ZERO_DIGEST.into();
    assert_failure(
        &wrong_hash.outcome(),
        ClaimPredicateResolutionFailed(ClaimFailure::ClaimContentHashMismatch),
    );
}
