use magpie_claims::policy::{refutation_ceiling, support_ceiling, ClaimDomain, EvidenceKind};
use magpie_claims::{
    replay_origin_admission_context_v0, replay_standing_context,
    ClaimInlinePredicateEdgeLaneEdgeKindV0, ClaimInlinePredicateEdgeLaneFailureV0,
    ClaimInlinePredicateEdgeLaneIneligibleReasonV0, ClaimInlinePredicateEdgeLaneKindV0,
    ClaimInlinePredicateEdgeLaneOutcomeKindV0, ClaimInlinePredicateEdgeLaneOutcomeV0,
    ClaimInlinePredicateEdgeLaneRelationV0, ClaimInlineSubjectResolutionFailureV0,
    DeterministicVerifierContextTraceV0, InlinePredicateAttestationBindingFailureV0,
    ResolutionContentClosureV0, Sha256ClaimInlineBytesPredicateOutcomeKindV0,
    StandingReplaySnapshot,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [63; 32];
const CLAIM_ID: &str = "claim-c";
const EVIDENCE_ID: &str = "evidence-e";
const SCOPE: &str = "scope-s";
const SUPPORT_EDGE_ID: &str = "edge-support";
const REFUTE_EDGE_ID: &str = "edge-refute";
const CLAIM_SCHEMA: &str = "magpie-machine-predicate-inline-bytes-v0";
const ATTESTATION_SCHEMA: &str = "magpie-inline-predicate-attestation-v0";
const PREDICATE_ID: &str = "sha256_claim_inline_bytes_equals_v0";
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
    Provenance::new("test", "claim-inline-predicate-edge-lane")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap()
}

fn canonical_claim_statement(expected_sha256: &str, subject_hex: &str) -> String {
    format!("{CLAIM_SCHEMA}:{PREDICATE_ID}:{expected_sha256}:{subject_hex}")
}

fn claim_metadata(expected_sha256: &str, subject_hex: &str) -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{expected_sha256}","subject_hex":"{subject_hex}"}}}}"#
    )
}

fn attestation_metadata(claim_id: &str, scope_ref: &str) -> String {
    format!(
        r#"{{"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":{},"scope_ref":{}}}}}"#,
        json_string(claim_id),
        json_string(scope_ref),
    )
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
    target_id: String,
    scope_ref: String,
    actor_class: String,
    rationale: String,
    metadata_json: String,
}

#[derive(Clone)]
struct Fixture {
    include_claim: bool,
    claim_id: String,
    claim_statement: String,
    claim_scope_ref: String,
    claim_actor_class: String,
    claim_content_hash: String,
    claim_metadata_json: String,
    evidence: Vec<EvidenceSpec>,
    edges: Vec<EdgeSpec>,
}

impl Fixture {
    fn valid(expected_sha256: &str, edge_kind: &str, edge_id: &str) -> Self {
        Self::valid_for(
            CLAIM_ID,
            EVIDENCE_ID,
            edge_id,
            SCOPE,
            expected_sha256,
            edge_kind,
        )
    }

    fn valid_for(
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
        scope_ref: &str,
        expected_sha256: &str,
        edge_kind: &str,
    ) -> Self {
        let subject_hex = "616263";
        let statement = canonical_claim_statement(expected_sha256, subject_hex);
        Self {
            include_claim: true,
            claim_id: claim_id.to_owned(),
            claim_statement: statement.clone(),
            claim_scope_ref: scope_ref.to_owned(),
            claim_actor_class: "AgentProposer".to_owned(),
            claim_content_hash: sha256_hex(statement.as_bytes()),
            claim_metadata_json: claim_metadata(expected_sha256, subject_hex),
            evidence: vec![EvidenceSpec {
                evidence_id: evidence_id.to_owned(),
                evidence_kind: "DeterministicVerification".to_owned(),
                summary: "routing attestation".to_owned(),
                scope_ref: scope_ref.to_owned(),
                actor_class: "AutomatedVerifier".to_owned(),
                content_hash: String::new(),
                metadata_json: attestation_metadata(claim_id, scope_ref),
            }],
            edges: vec![EdgeSpec {
                edge_id: edge_id.to_owned(),
                edge_kind: edge_kind.to_owned(),
                source_id: evidence_id.to_owned(),
                target_id: claim_id.to_owned(),
                scope_ref: scope_ref.to_owned(),
                actor_class: "HumanRoot".to_owned(),
                rationale: "exact selected lane".to_owned(),
                metadata_json: "{}".to_owned(),
            }],
        }
    }

    fn records(&self) -> Vec<Vec<u8>> {
        let store = MemStore::new();
        {
            let mut log = writer(store.clone());
            if self.include_claim {
                log.append(
                    provenance(),
                    Payload::ClaimAssertedV2 {
                        claim_id: self.claim_id.clone(),
                        statement: self.claim_statement.clone(),
                        scope_ref: self.claim_scope_ref.clone(),
                        actor_class: self.claim_actor_class.clone(),
                        content_hash: self.claim_content_hash.clone(),
                        metadata_json: self.claim_metadata_json.clone(),
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
                        target_id: edge.target_id.clone(),
                        scope_ref: edge.scope_ref.clone(),
                        actor_class: edge.actor_class.clone(),
                        rationale: edge.rationale.clone(),
                        metadata_json: edge.metadata_json.clone(),
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

    fn outcome(
        &self,
        claim_id: &str,
        evidence_id: &str,
        edge_id: &str,
    ) -> ClaimInlinePredicateEdgeLaneOutcomeV0 {
        self.snapshot()
            .resolve_claim_inline_predicate_edge_lane_v0(claim_id, evidence_id, edge_id)
    }
}

fn assert_failure(
    outcome: &ClaimInlinePredicateEdgeLaneOutcomeV0,
    expected: ClaimInlinePredicateEdgeLaneFailureV0,
) {
    assert_eq!(
        outcome.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::ResolutionFailed
    );
    assert_eq!(outcome.failure(), Some(&expected));
    assert!(outcome.receipt().is_none());
    assert!(outcome.ineligible_reason().is_none());
}

fn assert_vector(
    outcome: &ClaimInlinePredicateEdgeLaneOutcomeV0,
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
    assert!(!bytes.contains(&b' '));
}

#[test]
fn claim_inline_predicate_edge_lane_all_four_matrix_cells_are_exact() {
    let support = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        SUPPORT_EDGE_ID,
    );
    assert_eq!(
        support.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
    assert!(support.ineligible_reason().is_none());
    assert!(support.failure().is_none());
    let receipt = support.receipt().unwrap();
    assert_eq!(
        receipt.schema(),
        "magpie-claim-inline-predicate-edge-lane-v0"
    );
    assert_eq!(receipt.lane(), ClaimInlinePredicateEdgeLaneKindV0::Support);
    assert_eq!(receipt.predicate_id(), PREDICATE_ID);
    assert_eq!(receipt.claim_id(), CLAIM_ID);
    assert_eq!(receipt.evidence_id(), EVIDENCE_ID);
    assert_eq!(receipt.edge_id(), SUPPORT_EDGE_ID);
    assert_eq!(receipt.scope_ref(), SCOPE);
    assert_eq!(
        receipt.predicate_relation(),
        ClaimInlinePredicateEdgeLaneRelationV0::DigestEqual
    );
    assert_eq!(
        receipt.edge_kind(),
        ClaimInlinePredicateEdgeLaneEdgeKindV0::Supports
    );
    assert_eq!(receipt.candidate_ceiling(), Status::Settled);
    assert!(std::ptr::eq(
        support.requested_claim_id().as_ptr(),
        receipt.claim_id().as_ptr()
    ));
    assert!(std::ptr::eq(
        support.requested_evidence_id().as_ptr(),
        receipt.evidence_id().as_ptr()
    ));
    assert!(std::ptr::eq(
        support.requested_edge_id().as_ptr(),
        receipt.edge_id().as_ptr()
    ));

    let unequal_support = Fixture::valid(ZERO_DIGEST, "supports", SUPPORT_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        SUPPORT_EDGE_ID,
    );
    assert_eq!(
        unequal_support.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::Ineligible
    );
    assert_eq!(
        unequal_support.ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestUnequalDoesNotSupport)
    );
    assert!(unequal_support.receipt().is_none());
    assert!(unequal_support.failure().is_none());

    let equal_refute = Fixture::valid(ABC_DIGEST, "contradicts", REFUTE_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        REFUTE_EDGE_ID,
    );
    assert_eq!(
        equal_refute.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::Ineligible
    );
    assert_eq!(
        equal_refute.ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
    );

    let refutation = Fixture::valid(ZERO_DIGEST, "contradicts", REFUTE_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        REFUTE_EDGE_ID,
    );
    assert_eq!(
        refutation.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::RefutationEligible
    );
    let receipt = refutation.receipt().unwrap();
    assert_eq!(
        receipt.lane(),
        ClaimInlinePredicateEdgeLaneKindV0::Refutation
    );
    assert_eq!(
        receipt.predicate_relation(),
        ClaimInlinePredicateEdgeLaneRelationV0::DigestUnequal
    );
    assert_eq!(
        receipt.edge_kind(),
        ClaimInlinePredicateEdgeLaneEdgeKindV0::Contradicts
    );
    assert_eq!(receipt.candidate_ceiling(), Status::Refuted);
}

#[test]
fn claim_inline_predicate_edge_lane_six_ratified_runtime_vectors_are_exact() {
    let support = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        SUPPORT_EDGE_ID,
    );
    assert_vector(
        &support,
        r#"{"outcome":"support_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"support","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","scope_ref":"scope-s","predicate_relation":"digest_equal","edge_kind":"supports","candidate_ceiling":"settled"}}}"#,
        362,
        "3d1e241b92e8b66c7e42613231a68e4df990c910d48627415ad29db8dc67c085",
    );

    let refutation = Fixture::valid(ZERO_DIGEST, "contradicts", REFUTE_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        REFUTE_EDGE_ID,
    );
    assert_vector(
        &refutation,
        r#"{"outcome":"refutation_eligible","details":{"receipt":{"schema":"magpie-claim-inline-predicate-edge-lane-v0","lane":"refutation","predicate_id":"sha256_claim_inline_bytes_equals_v0","claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","scope_ref":"scope-s","predicate_relation":"digest_unequal","edge_kind":"contradicts","candidate_ceiling":"refuted"}}}"#,
        372,
        "2849f779e12e756fce866294053d89cae9413254c5edab78221e82ac2d3b3c06",
    );

    let unequal_support = Fixture::valid(ZERO_DIGEST, "supports", SUPPORT_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        SUPPORT_EDGE_ID,
    );
    assert_vector(
        &unequal_support,
        r#"{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-support","reason":"digest_unequal_does_not_support"}}"#,
        152,
        "793302a14a991b45ff6a5c6c15c04d936664c29c3685f25d0e509f26c8e983b8",
    );

    let equal_refute = Fixture::valid(ABC_DIGEST, "contradicts", REFUTE_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        REFUTE_EDGE_ID,
    );
    assert_vector(
        &equal_refute,
        r#"{"outcome":"ineligible","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-refute","reason":"digest_equal_does_not_refute"}}"#,
        148,
        "2f320d25d7b5d3f3c5c45185cf33abb025d2a6f7a93f5721d157c3318eac469c",
    );

    let missing_edge = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID).outcome(
        CLAIM_ID,
        EVIDENCE_ID,
        "edge-missing",
    );
    assert_vector(
        &missing_edge,
        r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-c","evidence_id":"evidence-e","edge_id":"edge-missing","reason":"missing_edge"}}"#,
        140,
        "6ad4b261939e8289b1ee67157e74a37423888e5baf33204802f178e7c4033bc9",
    );

    let mut missing_claim = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    missing_claim.include_claim = false;
    let nested = missing_claim.outcome("claim-missing", EVIDENCE_ID, "edge-e");
    assert_vector(
        &nested,
        r#"{"outcome":"resolution_failed","details":{"claim_id":"claim-missing","evidence_id":"evidence-e","edge_id":"edge-e","reason":"attestation_binding_failed","binding_reason":"claim_predicate_resolution_failed","claim_reason":"missing_claim"}}"#,
        238,
        "942b5ec8232429b7301ba489d7aedf214ad32a77be751d59d80bc0707432a62b",
    );
}

#[test]
fn claim_inline_predicate_edge_lane_emits_runtime_bytes_for_secondary_hashing() {
    if std::env::var_os("MAGPIE_EDGE_LANE_EMIT_VECTORS").is_none() {
        return;
    }

    let support_fixture = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    let refutation_fixture = Fixture::valid(ZERO_DIGEST, "contradicts", REFUTE_EDGE_ID);
    let unequal_support_fixture = Fixture::valid(ZERO_DIGEST, "supports", SUPPORT_EDGE_ID);
    let equal_refute_fixture = Fixture::valid(ABC_DIGEST, "contradicts", REFUTE_EDGE_ID);
    let support_snapshot = support_fixture.snapshot();
    let unequal_snapshot = unequal_support_fixture.snapshot();

    let mut missing_claim_fixture = support_fixture.clone();
    missing_claim_fixture.include_claim = false;
    let vectors = [
        (
            "0063_support_eligible",
            support_fixture
                .outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID)
                .canonical_bytes(),
        ),
        (
            "0063_refutation_eligible",
            refutation_fixture
                .outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID)
                .canonical_bytes(),
        ),
        (
            "0063_digest_unequal_does_not_support",
            unequal_support_fixture
                .outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID)
                .canonical_bytes(),
        ),
        (
            "0063_digest_equal_does_not_refute",
            equal_refute_fixture
                .outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID)
                .canonical_bytes(),
        ),
        (
            "0063_missing_edge",
            support_fixture
                .outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing")
                .canonical_bytes(),
        ),
        (
            "0063_nested_missing_claim",
            missing_claim_fixture
                .outcome("claim-missing", EVIDENCE_ID, "edge-e")
                .canonical_bytes(),
        ),
        (
            "0059_digest_equal",
            support_snapshot
                .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID)
                .canonical_bytes(),
        ),
        (
            "0059_digest_unequal",
            unequal_snapshot
                .evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID)
                .canonical_bytes(),
        ),
        (
            "0059_missing_claim",
            support_snapshot
                .evaluate_sha256_claim_inline_bytes_equals_v0("claim-missing")
                .canonical_bytes(),
        ),
        (
            "0061_bound",
            support_snapshot
                .resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID)
                .canonical_bytes(),
        ),
        (
            "0061_missing_evidence",
            support_snapshot
                .resolve_inline_predicate_attestation_v0(CLAIM_ID, "evidence-missing")
                .canonical_bytes(),
        ),
        (
            "0061_nested_missing_claim",
            support_snapshot
                .resolve_inline_predicate_attestation_v0("claim-missing", EVIDENCE_ID)
                .canonical_bytes(),
        ),
    ];
    for (name, bytes) in vectors {
        println!("MAGPIE_VECTOR|{name}|{}", hex::encode(bytes));
    }
}

#[test]
fn claim_inline_predicate_edge_lane_exact_top_level_failures_and_edge_kinds() {
    let base = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    assert_failure(
        &base.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
    );

    for unsupported in ["ratifies", "derived_from", "invalidates", "supersedes"] {
        let fixture = Fixture::valid(ABC_DIGEST, unsupported, SUPPORT_EDGE_ID);
        assert_failure(
            &fixture.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
            ClaimInlinePredicateEdgeLaneFailureV0::UnsupportedEdgeKind,
        );
    }

    let mut source = base.clone();
    source.edges[0].source_id = "evidence-other".into();
    assert_failure(
        &source.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        ClaimInlinePredicateEdgeLaneFailureV0::EdgeSourceBindingMismatch,
    );

    let mut target = base.clone();
    target.edges[0].target_id = "claim-other".into();
    assert_failure(
        &target.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        ClaimInlinePredicateEdgeLaneFailureV0::EdgeTargetBindingMismatch,
    );

    let mut scope = base;
    scope.edges[0].scope_ref = "scope-other".into();
    assert_failure(
        &scope.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        ClaimInlinePredicateEdgeLaneFailureV0::EdgeScopeBindingMismatch,
    );
}

#[test]
fn claim_inline_predicate_edge_lane_failure_precedence_is_frozen() {
    use ClaimInlinePredicateEdgeLaneFailureV0 as Failure;
    use InlinePredicateAttestationBindingFailureV0 as Binding;

    let mut malformed_claim = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    malformed_claim.claim_metadata_json = "{".into();
    malformed_claim.edges.clear();
    assert_failure(
        &malformed_claim.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        Failure::AttestationBindingFailed(Binding::ClaimPredicateResolutionFailed(
            ClaimInlineSubjectResolutionFailureV0::MalformedMetadata,
        )),
    );

    let mut missing_evidence = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    missing_evidence.evidence.clear();
    missing_evidence.edges.clear();
    assert_failure(
        &missing_evidence.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        Failure::AttestationBindingFailed(Binding::MissingEvidence),
    );

    let mut wrong_kind = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    wrong_kind.evidence[0].evidence_kind = "ExecutionEvidence".into();
    wrong_kind.evidence[0].metadata_json = "{".into();
    wrong_kind.edges.clear();
    assert_failure(
        &wrong_kind.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        Failure::AttestationBindingFailed(Binding::WrongEvidenceKind),
    );

    let mut malformed_attestation = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    malformed_attestation.evidence[0].metadata_json = "{".into();
    malformed_attestation.edges.clear();
    assert_failure(
        &malformed_attestation.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        Failure::AttestationBindingFailed(Binding::MalformedAttestationMetadata),
    );

    let mut missing_selected = Fixture::valid(ABC_DIGEST, "ratifies", "edge-other");
    missing_selected.edges[0].source_id = "wrong-source".into();
    missing_selected.edges[0].target_id = "wrong-target".into();
    missing_selected.edges[0].scope_ref = "wrong-scope".into();
    assert_failure(
        &missing_selected.outcome(CLAIM_ID, EVIDENCE_ID, "edge-missing"),
        Failure::MissingEdge,
    );

    let mut unsupported = Fixture::valid(ABC_DIGEST, "ratifies", SUPPORT_EDGE_ID);
    unsupported.edges[0].source_id = "wrong-source".into();
    unsupported.edges[0].target_id = "wrong-target".into();
    unsupported.edges[0].scope_ref = "wrong-scope".into();
    assert_failure(
        &unsupported.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        Failure::UnsupportedEdgeKind,
    );

    let mut source = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    source.edges[0].source_id = "wrong-source".into();
    source.edges[0].target_id = "wrong-target".into();
    source.edges[0].scope_ref = "wrong-scope".into();
    assert_failure(
        &source.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        Failure::EdgeSourceBindingMismatch,
    );

    let mut target = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    target.edges[0].target_id = "wrong-target".into();
    target.edges[0].scope_ref = "wrong-scope".into();
    assert_failure(
        &target.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        Failure::EdgeTargetBindingMismatch,
    );

    let mut scope = Fixture::valid(ZERO_DIGEST, "supports", SUPPORT_EDGE_ID);
    scope.edges[0].scope_ref = "wrong-scope".into();
    assert_failure(
        &scope.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        Failure::EdgeScopeBindingMismatch,
    );
}

#[test]
fn claim_inline_predicate_edge_lane_representative_nested_failures_are_preserved() {
    use ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed;
    use InlinePredicateAttestationBindingFailureV0 as Binding;

    let mut claim = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    claim.claim_metadata_json = "{".into();
    assert_failure(
        &claim.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::ClaimPredicateResolutionFailed(
            ClaimInlineSubjectResolutionFailureV0::MalformedMetadata,
        )),
    );

    let mut evidence = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    evidence.evidence.clear();
    assert_failure(
        &evidence.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::MissingEvidence),
    );

    let mut parser = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    parser.evidence[0].metadata_json = r#"{"inline_predicate_attestation":{},"unknown":0}"#.into();
    assert_failure(
        &parser.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::UnknownAttestationMetadataKey),
    );

    let mut predicate = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    predicate.evidence[0].metadata_json = attestation_metadata(CLAIM_ID, SCOPE)
        .replace(PREDICATE_ID, "sha256_claim_inline_bytes_equals_v1");
    assert_failure(
        &predicate.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::PredicateBindingMismatch),
    );

    let mut claim_binding = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    claim_binding.evidence[0].metadata_json = attestation_metadata("claim-other", SCOPE);
    assert_failure(
        &claim_binding.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::ClaimBindingMismatch),
    );

    let mut evidence_scope = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    evidence_scope.evidence[0].metadata_json = attestation_metadata(CLAIM_ID, "scope-other");
    assert_failure(
        &evidence_scope.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::EvidenceScopeBindingMismatch),
    );

    let mut claim_scope = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    claim_scope.claim_scope_ref = "scope-claim".into();
    assert_failure(
        &claim_scope.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        AttestationBindingFailed(Binding::ClaimScopeBindingMismatch),
    );
}

#[test]
fn claim_inline_predicate_edge_lane_exact_ids_have_no_normalization_or_namespace_rules() {
    let cases = [
        (" claim ", " evidence ", " edge ", " scope "),
        ("Claim-C", "Evidence-E", "Edge-E", "Scope-S"),
        ("claim:raw", "evidence:raw", "edge:raw", "scope:raw"),
        (
            "claim-e\u{301}",
            "evidence-e\u{301}",
            "edge-e\u{301}",
            "scope-e\u{301}",
        ),
    ];
    for (claim_id, evidence_id, edge_id, scope_ref) in cases {
        let fixture = Fixture::valid_for(
            claim_id,
            evidence_id,
            edge_id,
            scope_ref,
            ABC_DIGEST,
            "supports",
        );
        let outcome = fixture.outcome(claim_id, evidence_id, edge_id);
        assert_eq!(
            outcome.kind(),
            ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
        );
        assert_eq!(outcome.requested_claim_id(), claim_id);
        assert_eq!(outcome.requested_evidence_id(), evidence_id);
        assert_eq!(outcome.requested_edge_id(), edge_id);
    }

    let whitespace = Fixture::valid_for(
        " claim ",
        " evidence ",
        " edge ",
        " scope ",
        ABC_DIGEST,
        "supports",
    );
    assert_failure(
        &whitespace.outcome("claim", "evidence", "edge"),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::MissingClaim,
            ),
        ),
    );

    let exact_case = Fixture::valid_for(
        "Claim-C",
        "Evidence-E",
        "Edge-E",
        SCOPE,
        ABC_DIGEST,
        "supports",
    );
    assert_failure(
        &exact_case.outcome("claim-c", "Evidence-E", "Edge-E"),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::MissingClaim,
            ),
        ),
    );
    assert_failure(
        &exact_case.outcome("Claim-C", "evidence-e", "Edge-E"),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::MissingEvidence,
        ),
    );
    assert_failure(
        &exact_case.outcome("Claim-C", "Evidence-E", "edge-e"),
        ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
    );

    let decomposed = Fixture::valid_for(
        "claim-e\u{301}",
        "evidence-e\u{301}",
        "edge-e\u{301}",
        SCOPE,
        ABC_DIGEST,
        "supports",
    );
    assert_failure(
        &decomposed.outcome("claim-é", "evidence-e\u{301}", "edge-e\u{301}"),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::MissingClaim,
            ),
        ),
    );
    assert_failure(
        &decomposed.outcome("claim-e\u{301}", "evidence-é", "edge-e\u{301}"),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::MissingEvidence,
        ),
    );
    assert_failure(
        &decomposed.outcome("claim-e\u{301}", "evidence-e\u{301}", "edge-é"),
        ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
    );

    let mut escaped = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    escaped.evidence[0].metadata_json = r#"{"inline_predicate_attestation":{"schema":"magpie-inline-predicate-attestation-v\u0030","predicate_id":"sha256_claim_inline_bytes_equals_v\u0030","subject_claim_id":"claim-\u0063","scope_ref":"scope-\u0073"}}"#.into();
    assert_eq!(
        escaped
            .outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID)
            .kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
}

#[test]
fn claim_inline_predicate_edge_lane_failed_ids_and_json_encoding_are_exact() {
    let fixture = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    let claim_id = "q\"\\\u{0008}\t\n\u{000c}\r\u{0001}/é/e\u{0301}";
    let evidence_id = "e\"\\\u{0002}/普通话";
    let edge_id = "x\"\\\u{0003}/edge";
    let outcome = fixture.outcome(claim_id, evidence_id, edge_id);
    assert_eq!(outcome.requested_claim_id(), claim_id);
    assert_eq!(outcome.requested_evidence_id(), evidence_id);
    assert_eq!(outcome.requested_edge_id(), edge_id);
    let json = String::from_utf8(outcome.canonical_bytes()).unwrap();
    assert!(json.contains(r#"\b\t\n\f\r\u0001/é/e"#));
    assert!(json.contains(r#"\u0002/普通话"#));
    assert!(json.contains(r#"\u0003/edge"#));
    assert!(!json.contains("\\/"));
    assert!(!json.starts_with('\u{feff}'));
    assert!(!json.ends_with('\n'));

    let precomposed = fixture.outcome("é", EVIDENCE_ID, SUPPORT_EDGE_ID);
    let decomposed = fixture.outcome("e\u{0301}", EVIDENCE_ID, SUPPORT_EDGE_ID);
    assert_ne!(precomposed.canonical_bytes(), decomposed.canonical_bytes());
    assert!(String::from_utf8(precomposed.canonical_bytes())
        .unwrap()
        .contains('é'));
}

#[test]
fn claim_inline_predicate_edge_lane_exact_edge_selection_and_first_write_wins() {
    let mut exact = Fixture::valid(ABC_DIGEST, "supports", "edge-good");
    let mut bad = exact.edges[0].clone();
    bad.edge_id = "edge-bad".into();
    bad.edge_kind = "ratifies".into();
    bad.source_id = "wrong-source".into();
    exact.edges.push(bad);
    assert_eq!(
        exact.outcome(CLAIM_ID, EVIDENCE_ID, "edge-good").kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
    assert_failure(
        &exact.outcome(CLAIM_ID, EVIDENCE_ID, "edge-bad"),
        ClaimInlinePredicateEdgeLaneFailureV0::UnsupportedEdgeKind,
    );
    assert_failure(
        &exact.outcome(CLAIM_ID, EVIDENCE_ID, "edge-absent"),
        ClaimInlinePredicateEdgeLaneFailureV0::MissingEdge,
    );

    let mut duplicate = Fixture::valid(ABC_DIGEST, "supports", "edge-duplicate");
    let mut second = duplicate.edges[0].clone();
    second.edge_kind = "contradicts".into();
    duplicate.edges.push(second);
    assert_eq!(
        duplicate
            .outcome(CLAIM_ID, EVIDENCE_ID, "edge-duplicate")
            .kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );

    let mut reversed = Fixture::valid(ABC_DIGEST, "contradicts", "edge-duplicate");
    let mut second = reversed.edges[0].clone();
    second.edge_kind = "supports".into();
    reversed.edges.push(second);
    assert_eq!(
        reversed
            .outcome(CLAIM_ID, EVIDENCE_ID, "edge-duplicate")
            .ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
    );
}

fn assert_same_receipt(
    left: &ClaimInlinePredicateEdgeLaneOutcomeV0,
    right: &ClaimInlinePredicateEdgeLaneOutcomeV0,
) {
    assert_eq!(left.kind(), right.kind());
    let left = left.receipt().unwrap();
    let right = right.receipt().unwrap();
    assert_eq!(left.schema(), right.schema());
    assert_eq!(left.lane(), right.lane());
    assert_eq!(left.predicate_id(), right.predicate_id());
    assert_eq!(left.claim_id(), right.claim_id());
    assert_eq!(left.evidence_id(), right.evidence_id());
    assert_eq!(left.edge_id(), right.edge_id());
    assert_eq!(left.scope_ref(), right.scope_ref());
    assert_eq!(left.predicate_relation(), right.predicate_relation());
    assert_eq!(left.edge_kind(), right.edge_kind());
    assert_eq!(left.candidate_ceiling(), right.candidate_ceiling());
}

#[test]
fn claim_inline_predicate_edge_lane_ignored_fields_are_byte_invariant() {
    let base = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    let expected = base.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID);

    let mut variants = Vec::new();
    let mut value = base.clone();
    value.claim_actor_class = "HumanRoot".into();
    variants.push(value);
    let mut value = base.clone();
    value.evidence[0].content_hash = "f".repeat(64);
    variants.push(value);
    let mut value = base.clone();
    value.evidence[0].actor_class = "SourceImporter".into();
    variants.push(value);
    let mut value = base.clone();
    value.evidence[0].summary = "different summary".into();
    variants.push(value);
    let mut value = base.clone();
    value.edges[0].actor_class = "AgentProposer".into();
    variants.push(value);
    let mut value = base.clone();
    value.edges[0].rationale = "different rationale".into();
    variants.push(value);
    let mut value = base;
    value.edges[0].metadata_json = r#"{"ignored":"different"}"#.into();
    variants.push(value);

    for variant in variants {
        let outcome = variant.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID);
        assert_same_receipt(&expected, &outcome);
        assert_eq!(outcome.canonical_bytes(), expected.canonical_bytes());
    }
}

#[test]
fn claim_inline_predicate_edge_lane_mutual_exclusion_and_repetition_do_not_amplify() {
    let mut equal = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    let mut contradicts = equal.edges[0].clone();
    contradicts.edge_id = REFUTE_EDGE_ID.into();
    contradicts.edge_kind = "contradicts".into();
    equal.edges.push(contradicts);
    assert_eq!(
        equal.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID).kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
    assert_eq!(
        equal
            .outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID)
            .ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
    );

    let mut unequal = Fixture::valid(ZERO_DIGEST, "contradicts", REFUTE_EDGE_ID);
    let mut supports = unequal.edges[0].clone();
    supports.edge_id = SUPPORT_EDGE_ID.into();
    supports.edge_kind = "supports".into();
    unequal.edges.push(supports);
    assert_eq!(
        unequal
            .outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID)
            .kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::RefutationEligible
    );
    assert_eq!(
        unequal
            .outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID)
            .ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestUnequalDoesNotSupport)
    );

    let mut repeated = Fixture::valid(ABC_DIGEST, "supports", "edge-a");
    let mut second_evidence = repeated.evidence[0].clone();
    second_evidence.evidence_id = "evidence-b".into();
    repeated.evidence.push(second_evidence);
    let mut second_edge = repeated.edges[0].clone();
    second_edge.edge_id = "edge-b".into();
    second_edge.source_id = "evidence-b".into();
    repeated.edges.push(second_edge);
    let first = repeated.outcome(CLAIM_ID, EVIDENCE_ID, "edge-a");
    let second = repeated.outcome(CLAIM_ID, "evidence-b", "edge-b");
    assert_eq!(
        first.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
    assert_eq!(
        second.kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
    );
    assert_ne!(first.canonical_bytes(), second.canonical_bytes());

    let mut same_evidence = Fixture::valid(ABC_DIGEST, "supports", "edge-one");
    let mut another_edge = same_evidence.edges[0].clone();
    another_edge.edge_id = "edge-two".into();
    same_evidence.edges.push(another_edge);
    let one = same_evidence.outcome(CLAIM_ID, EVIDENCE_ID, "edge-one");
    let two = same_evidence.outcome(CLAIM_ID, EVIDENCE_ID, "edge-two");
    assert_eq!(one.kind(), two.kind());
    assert_ne!(one.canonical_bytes(), two.canonical_bytes());
}

#[test]
fn claim_inline_predicate_edge_lane_ticket_0056_substitution_is_closed() {
    let base = Fixture::valid(ABC_DIGEST, "contradicts", REFUTE_EDGE_ID);
    let outcome = base.outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID);
    assert_eq!(
        outcome.ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
    );

    let mut smuggled = base.clone();
    smuggled.evidence[0].metadata_json = smuggled.evidence[0].metadata_json.replace(
        "}}",
        r#","subject_hex":"deadbeef","relation":"digest_unequal"}}"#,
    );
    assert_failure(
        &smuggled.outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::UnknownAttestationField,
        ),
    );

    let mut unrelated = base;
    unrelated.evidence[0].summary = "arbitrary bytes deadbeef".into();
    unrelated.evidence[0].content_hash = "0".repeat(64);
    let unrelated_outcome = unrelated.outcome(CLAIM_ID, EVIDENCE_ID, REFUTE_EDGE_ID);
    assert_eq!(
        unrelated_outcome.canonical_bytes(),
        outcome.canonical_bytes()
    );
}

#[test]
fn claim_inline_predicate_edge_lane_ticket_0059_and_0061_vectors_are_inert() {
    let equal_snapshot = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID).snapshot();
    let equal = equal_snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(
        equal.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestEqual
    );
    assert_eq!(equal.canonical_bytes().len(), 639);
    assert_eq!(
        sha256_hex(&equal.canonical_bytes()),
        "d0c6a0f771600c34aad02264f722f915bb9093ac28c4489d1369585c402a6f5a"
    );

    let unequal_snapshot = Fixture::valid(ZERO_DIGEST, "supports", SUPPORT_EDGE_ID).snapshot();
    let unequal = unequal_snapshot.evaluate_sha256_claim_inline_bytes_equals_v0(CLAIM_ID);
    assert_eq!(
        unequal.kind(),
        Sha256ClaimInlineBytesPredicateOutcomeKindV0::DigestUnequal
    );
    assert_eq!(unequal.canonical_bytes().len(), 641);
    assert_eq!(
        sha256_hex(&unequal.canonical_bytes()),
        "cced955ed7822888e3d2e0a3f91263ec905d2c629d5649003eaabd3360f81291"
    );

    let missing_claim =
        equal_snapshot.evaluate_sha256_claim_inline_bytes_equals_v0("claim-missing");
    assert_eq!(missing_claim.canonical_bytes().len(), 95);
    assert_eq!(
        sha256_hex(&missing_claim.canonical_bytes()),
        "b99b96af9b35c96e6d65bea4a0fdd364670abefd21e163009737a9ef4d91969d"
    );

    let bound = equal_snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, EVIDENCE_ID);
    assert_eq!(bound.canonical_bytes().len(), 216);
    assert_eq!(
        sha256_hex(&bound.canonical_bytes()),
        "9ecd2e4c3c4f291fb8d4c5c6a4ceac590a5cc5a516c088766979596705d86bc5"
    );

    let missing_evidence =
        equal_snapshot.resolve_inline_predicate_attestation_v0(CLAIM_ID, "evidence-missing");
    assert_eq!(missing_evidence.canonical_bytes().len(), 125);
    assert_eq!(
        sha256_hex(&missing_evidence.canonical_bytes()),
        "968bf85313eef6e2700fb9e3780bfa1b9f88c31f1d6a58898901843b20a10fe6"
    );

    let nested =
        equal_snapshot.resolve_inline_predicate_attestation_v0("claim-missing", EVIDENCE_ID);
    assert_eq!(nested.canonical_bytes().len(), 173);
    assert_eq!(
        sha256_hex(&nested.canonical_bytes()),
        "2a341009683cc4bc9db94a5d631898a0ea5b1efa7701e8e23ac0f93831b91190"
    );
}

#[test]
fn claim_inline_predicate_edge_lane_historical_metadata_families_still_refuse() {
    let statement = format!("sha256_bytes_equals_v0:{ABC_DIGEST}");
    let mut historical = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    historical.claim_statement = statement.clone();
    historical.claim_content_hash = sha256_hex(statement.as_bytes());
    historical.claim_metadata_json = format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{ABC_DIGEST}"}}}}"#
    );
    historical.evidence[0].metadata_json = format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
    );
    let snapshot = historical.snapshot();
    assert_failure(
        &snapshot.resolve_claim_inline_predicate_edge_lane_v0(
            CLAIM_ID,
            EVIDENCE_ID,
            SUPPORT_EDGE_ID,
        ),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::ClaimPredicateResolutionFailed(
                ClaimInlineSubjectResolutionFailureV0::UnknownClaimMetadataKey,
            ),
        ),
    );
    assert!(matches!(
        snapshot.resolve_deterministic_verifier_context_v0(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        Some(DeterministicVerifierContextTraceV0::Matched(_))
    ));

    let mut inline_with_old_evidence = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    inline_with_old_evidence.evidence[0].metadata_json =
        historical.evidence[0].metadata_json.clone();
    assert_failure(
        &inline_with_old_evidence.outcome(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID),
        ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
            InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey,
        ),
    );
}

#[test]
fn claim_inline_predicate_edge_lane_is_snapshot_standing_policy_and_support_audit_inert() {
    let fixture = Fixture::valid(ABC_DIGEST, "supports", SUPPORT_EDGE_ID);
    let reader = LogReader::open(
        MemStore::from_records(fixture.records()),
        signing_key().verifying_key(),
    );
    let context = replay_origin_admission_context_v0(&reader).unwrap();
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
    let support_audit_before = context
        .resolve_support_contribution_audit_v0(&closure)
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
    );

    assert_eq!(
        context
            .snapshot()
            .resolve_claim_inline_predicate_edge_lane_v0(CLAIM_ID, EVIDENCE_ID, SUPPORT_EDGE_ID,)
            .kind(),
        ClaimInlinePredicateEdgeLaneOutcomeKindV0::SupportEligible
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
        context
            .resolve_support_contribution_audit_v0(&closure)
            .canonical_bytes(),
        support_audit_before
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
        ),
        policy_before
    );
}
