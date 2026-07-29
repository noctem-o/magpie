use magpie_claims::{
    replay_origin_admission_context_v0, ClaimInlinePredicateEdgeLaneFailureV0,
    ClaimInlinePredicateEdgeLaneIneligibleReasonV0, InlinePredicateAttestationBindingFailureV0,
    OriginAdmissionReplayContextV0, ResolutionContentClosureV0, StandingBlockerV3,
    StandingCurrentness, StandingPolicyContextV4, StandingPolicyRuleV4,
    StandingTraceClassificationV4, MAGPIE_CLAIMS_POLICY_V3_ID, MAGPIE_CLAIMS_POLICY_V4_ID,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [65; 32];
const CLAIM_ID: &str = "claim-c";
const OTHER_CLAIM_ID: &str = "claim-other";
const SCOPE: &str = "scope-s";
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
    Provenance::new("test", "standing-v4")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn canonical_claim_statement(expected_sha256: &str) -> String {
    format!("{CLAIM_SCHEMA}:{PREDICATE_ID}:{expected_sha256}:616263")
}

fn claim_metadata(expected_sha256: &str) -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate_inline_bytes":{{"schema":"{CLAIM_SCHEMA}","predicate_id":"{PREDICATE_ID}","expected_sha256":"{expected_sha256}","subject_hex":"616263"}}}}"#
    )
}

fn attestation_metadata(claim_id: &str, scope_ref: &str) -> String {
    format!(
        r#"{{"inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":{},"scope_ref":{}}}}}"#,
        serde_json::to_string(claim_id).unwrap(),
        serde_json::to_string(scope_ref).unwrap(),
    )
}

#[derive(Clone)]
struct EvidenceSpec {
    evidence_id: String,
    evidence_kind: String,
    summary: String,
    scope_ref: String,
    content_hash: String,
    metadata_json: String,
}

impl EvidenceSpec {
    fn bound(evidence_id: &str) -> Self {
        Self {
            evidence_id: evidence_id.to_owned(),
            evidence_kind: "DeterministicVerification".to_owned(),
            summary: "claim-inline routing attestation".to_owned(),
            scope_ref: SCOPE.to_owned(),
            content_hash: String::new(),
            metadata_json: attestation_metadata(CLAIM_ID, SCOPE),
        }
    }
}

#[derive(Clone)]
struct EdgeSpec {
    edge_id: String,
    edge_kind: String,
    source_id: String,
    target_id: String,
    scope_ref: String,
}

impl EdgeSpec {
    fn new(edge_id: &str, edge_kind: &str, source_id: &str, target_id: &str) -> Self {
        Self {
            edge_id: edge_id.to_owned(),
            edge_kind: edge_kind.to_owned(),
            source_id: source_id.to_owned(),
            target_id: target_id.to_owned(),
            scope_ref: SCOPE.to_owned(),
        }
    }
}

#[derive(Clone)]
struct Fixture {
    expected_sha256: String,
    legacy_status: Option<Status>,
    evidence: Vec<EvidenceSpec>,
    edges: Vec<EdgeSpec>,
}

impl Fixture {
    fn valid_unequal() -> Self {
        Self {
            expected_sha256: ZERO_DIGEST.to_owned(),
            legacy_status: None,
            evidence: vec![EvidenceSpec::bound("evidence-a")],
            edges: vec![EdgeSpec::new(
                "edge-a",
                "contradicts",
                "evidence-a",
                CLAIM_ID,
            )],
        }
    }

    fn records(&self) -> Vec<Vec<u8>> {
        let store = MemStore::new();
        {
            let mut log = writer(store.clone());
            let statement = canonical_claim_statement(&self.expected_sha256);
            if let Some(status) = self.legacy_status {
                log.append(
                    provenance(),
                    Payload::ClaimAsserted {
                        claim_id: CLAIM_ID.to_owned(),
                        statement: statement.clone(),
                        status,
                    },
                )
                .unwrap();
            }
            log.append(
                provenance(),
                Payload::ClaimAssertedV2 {
                    claim_id: CLAIM_ID.to_owned(),
                    statement: statement.clone(),
                    scope_ref: SCOPE.to_owned(),
                    actor_class: "AgentProposer".to_owned(),
                    content_hash: sha256_hex(statement.as_bytes()),
                    metadata_json: claim_metadata(&self.expected_sha256),
                },
            )
            .unwrap();
            for evidence in &self.evidence {
                log.append(
                    provenance(),
                    Payload::EvidenceRegistered {
                        evidence_id: evidence.evidence_id.clone(),
                        evidence_kind: evidence.evidence_kind.clone(),
                        summary: evidence.summary.clone(),
                        scope_ref: evidence.scope_ref.clone(),
                        actor_class: "AutomatedVerifier".to_owned(),
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
                        actor_class: "HumanRoot".to_owned(),
                        rationale: "standing-v4 test edge".to_owned(),
                        metadata_json: "{}".to_owned(),
                    },
                )
                .unwrap();
            }
        }
        store.records()
    }

    fn context(&self) -> OriginAdmissionReplayContextV0 {
        let reader = LogReader::open(
            MemStore::from_records(self.records()),
            signing_key().verifying_key(),
        );
        replay_origin_admission_context_v0(&reader).unwrap()
    }
}

fn closure() -> ResolutionContentClosureV0 {
    ResolutionContentClosureV0::construct(&[], &[]).unwrap()
}

#[test]
fn public_v4_resolver_derives_sound_refutation_from_real_replay_and_closure() {
    let context = Fixture::valid_unequal().context();
    let closure = closure();
    let inherited = context.resolved_standing_with_trace_v3(CLAIM_ID, &closure);
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure);

    assert_eq!(resolution.policy_id(), MAGPIE_CLAIMS_POLICY_V4_ID);
    assert_eq!(resolution.claim_id(), CLAIM_ID);
    assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
    assert_eq!(
        context.resolved_standing_v4(CLAIM_ID, &closure),
        resolution.governed_standing()
    );
    assert_eq!(
        resolution.inherited_v3().canonical_bytes(),
        inherited.canonical_bytes()
    );
    assert_eq!(
        resolution.inherited_v3().policy_id(),
        MAGPIE_CLAIMS_POLICY_V3_ID
    );
    assert_eq!(
        resolution.verified_prefix_identity(),
        context.verified_prefix_identity()
    );
    assert_eq!(resolution.closure_identity(), closure.identity());
    assert_eq!(
        resolution.inherited_v3().verified_prefix_identity(),
        context.verified_prefix_identity()
    );
    assert_eq!(
        resolution.inherited_v3().closure_identity(),
        closure.identity()
    );
    assert!(resolution.resolution_failure().is_none());
    assert_eq!(resolution.candidate_trace().len(), 1);
    let entry = &resolution.candidate_trace()[0];
    assert_eq!(entry.edge_id(), "edge-a");
    assert_eq!(entry.evidence_id(), "evidence-a");
    assert_eq!(
        entry.classification(),
        StandingTraceClassificationV4::RefutationEligible
    );
    assert!(entry.ineligible_reason().is_none());
    assert!(entry.failure().is_none());
    assert_eq!(resolution.applications().len(), 1);
    assert_eq!(
        resolution.applications()[0].rule(),
        &StandingPolicyRuleV4::Sha256ClaimInlineBytesDirectRefutationV0
    );
    assert_eq!(
        resolution.applications()[0].context(),
        &StandingPolicyContextV4::UncontestedRefutationEligible
    );
    assert_eq!(
        resolution.applications()[0].achieved_standing(),
        Some(Status::Refuted)
    );
    assert!(resolution.blockers().is_empty());
}

#[test]
fn digest_equal_contradicts_is_ineligible_and_preserves_inherited_standing() {
    let mut fixture = Fixture::valid_unequal();
    fixture.expected_sha256 = ABC_DIGEST.to_owned();
    let context = fixture.context();
    let closure = closure();
    let inherited = context.resolved_standing_with_trace_v3(CLAIM_ID, &closure);
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure);

    assert_eq!(
        resolution.governed_standing(),
        inherited.governed_standing()
    );
    assert!(resolution.applications().is_empty());
    assert_eq!(resolution.candidate_trace().len(), 1);
    assert_eq!(
        resolution.candidate_trace()[0].classification(),
        StandingTraceClassificationV4::Ineligible
    );
    assert_eq!(
        resolution.candidate_trace()[0].ineligible_reason(),
        Some(ClaimInlinePredicateEdgeLaneIneligibleReasonV0::DigestEqualDoesNotRefute)
    );
    assert!(resolution.candidate_trace()[0].failure().is_none());
}

#[test]
fn negative_candidate_filter_and_raw_string_order_are_exact() {
    let mut fixture = Fixture::valid_unequal();
    fixture.evidence = vec![
        EvidenceSpec::bound("evidence-10"),
        EvidenceSpec::bound("evidence-a"),
        EvidenceSpec::bound("evidence-z"),
        EvidenceSpec::bound("evidence-support"),
        EvidenceSpec::bound("evidence-unknown"),
        EvidenceSpec::bound("evidence-other"),
    ];
    fixture.edges = vec![
        EdgeSpec::new("edge-z", "contradicts", "evidence-z", CLAIM_ID),
        EdgeSpec::new("edge-support", "supports", "evidence-support", CLAIM_ID),
        EdgeSpec::new("edge-a", "contradicts", "evidence-a", CLAIM_ID),
        EdgeSpec::new("edge-unknown", "ratifies", "evidence-unknown", CLAIM_ID),
        EdgeSpec::new(
            "edge-other",
            "contradicts",
            "evidence-other",
            OTHER_CLAIM_ID,
        ),
        EdgeSpec::new("edge-10", "contradicts", "evidence-10", CLAIM_ID),
    ];
    let context = fixture.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());

    assert_eq!(
        resolution
            .candidate_trace()
            .iter()
            .map(|entry| entry.edge_id())
            .collect::<Vec<_>>(),
        ["edge-10", "edge-a", "edge-z"]
    );
    assert!(resolution.candidate_trace().iter().all(|entry| {
        entry.classification() == StandingTraceClassificationV4::RefutationEligible
    }));
    assert_eq!(resolution.applications().len(), 1);
}

#[test]
fn first_write_wins_duplicate_edge_ids_and_distinct_ids_do_not_amplify() {
    let mut duplicate_ignored = Fixture::valid_unequal();
    duplicate_ignored.edges = vec![
        EdgeSpec::new("edge-dup", "supports", "evidence-a", CLAIM_ID),
        EdgeSpec::new("edge-dup", "contradicts", "evidence-a", CLAIM_ID),
    ];
    let context = duplicate_ignored.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert!(resolution.candidate_trace().is_empty());
    assert!(resolution.applications().is_empty());

    let mut distinct = Fixture::valid_unequal();
    distinct.edges = vec![
        EdgeSpec::new("edge-z", "contradicts", "evidence-a", CLAIM_ID),
        EdgeSpec::new("edge-a", "contradicts", "evidence-a", CLAIM_ID),
    ];
    let context = distinct.context();
    let closure = closure();
    let first = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure);
    let second = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure);
    assert_eq!(first.candidate_trace().len(), 2);
    assert_eq!(first.applications().len(), 1);
    assert_eq!(first.governed_standing(), Some(Status::Refuted));
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
}

#[test]
fn candidate_failures_are_independent_and_do_not_short_circuit() {
    for malformed_metadata in [None, Some(r#"{"inline_predicate_attestation":0}"#)] {
        let mut fixture = Fixture::valid_unequal();

        // `None` deliberately leaves `evidence-broken` unregistered so the
        // candidate fails with `MissingEvidence`.
        if let Some(metadata) = malformed_metadata {
            let mut broken = EvidenceSpec::bound("evidence-broken");
            broken.metadata_json = metadata.to_owned();
            fixture.evidence.push(broken);
        }
        fixture.edges = vec![
            EdgeSpec::new("edge-a", "contradicts", "evidence-a", CLAIM_ID),
            EdgeSpec::new("edge-z", "contradicts", "evidence-broken", CLAIM_ID),
        ];
        let context = fixture.context();
        let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
        assert_eq!(resolution.candidate_trace().len(), 2);
        assert_eq!(
            resolution.candidate_trace()[0].classification(),
            StandingTraceClassificationV4::RefutationEligible
        );
        assert_eq!(
            resolution.candidate_trace()[1].classification(),
            StandingTraceClassificationV4::ResolutionFailed
        );
        let expected_binding = if malformed_metadata.is_some() {
            InlinePredicateAttestationBindingFailureV0::WrongTypeInlinePredicateAttestation
        } else {
            InlinePredicateAttestationBindingFailureV0::MissingEvidence
        };
        assert_eq!(
            resolution.candidate_trace()[1].failure(),
            Some(
                &ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(expected_binding)
            )
        );
        assert_eq!(resolution.applications().len(), 1);
        assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
    }
}

#[test]
fn ticket_0056_substitution_routes_remain_closed() {
    let mut equal = Fixture::valid_unequal();
    equal.expected_sha256 = ABC_DIGEST.to_owned();
    equal.evidence[0].summary = "unrelated bytes: ffffffff".to_owned();
    equal.evidence[0].content_hash = "d".repeat(64);
    let context = equal.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert_eq!(
        resolution.candidate_trace()[0].classification(),
        StandingTraceClassificationV4::Ineligible
    );
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));

    let mut historical = Fixture::valid_unequal();
    historical.evidence[0].metadata_json = format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
    );
    let context = historical.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert_eq!(
        resolution.candidate_trace()[0].classification(),
        StandingTraceClassificationV4::ResolutionFailed
    );
    assert_eq!(
        resolution.candidate_trace()[0].failure(),
        Some(
            &ClaimInlinePredicateEdgeLaneFailureV0::AttestationBindingFailed(
                InlinePredicateAttestationBindingFailureV0::UnknownAttestationMetadataKey,
            )
        )
    );
    assert!(resolution.applications().is_empty());

    let mut evidence_selected_subject = Fixture::valid_unequal();
    evidence_selected_subject.evidence[0].metadata_json = format!(
        r#"{{"subject_hex":"646566","expected_sha256":"{}","inline_predicate_attestation":{{"schema":"{ATTESTATION_SCHEMA}","predicate_id":"{PREDICATE_ID}","subject_claim_id":"{CLAIM_ID}","scope_ref":"{SCOPE}"}}}}"#,
        sha256_hex(b"def"),
    );
    let context = evidence_selected_subject.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert_eq!(
        resolution.candidate_trace()[0].classification(),
        StandingTraceClassificationV4::ResolutionFailed
    );
    assert!(resolution.applications().is_empty());
}

#[test]
fn legacy_raw_refuted_is_audit_only_for_both_absence_and_eligibility() {
    let mut no_candidate = Fixture::valid_unequal();
    no_candidate.legacy_status = Some(Status::Refuted);
    no_candidate.edges.clear();
    let context = no_candidate.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert_eq!(resolution.legacy_raw_standing(), Some(Status::Refuted));
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    assert!(resolution.applications().is_empty());

    let mut eligible = Fixture::valid_unequal();
    eligible.legacy_status = Some(Status::Refuted);
    let context = eligible.context();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure());
    assert_eq!(resolution.legacy_raw_standing(), Some(Status::Refuted));
    assert_eq!(resolution.governed_standing(), Some(Status::Refuted));
    assert_eq!(resolution.applications().len(), 1);
}

#[test]
fn real_public_resolution_preserves_complete_extension_safe_v3_audit() {
    let context = Fixture::valid_unequal().context();
    let closure = closure();
    let resolution = context.resolved_standing_with_trace_v4(CLAIM_ID, &closure);
    assert!(resolution.inherited_v3().resolution_failure().is_none());
    assert_eq!(
        resolution.inherited_v3().blockers(),
        [StandingBlockerV3::InsufficientDistinctOriginGroups {
            distinct_group_count: 0,
        }]
    );
    assert_eq!(
        resolution.inherited_v3().governed_standing(),
        Some(Status::Conjectured)
    );
    assert_eq!(
        resolution.inherited_v3().currentness(),
        StandingCurrentness::Unknown
    );
}
