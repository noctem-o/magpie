use magpie_claims::{
    replay_standing_context, DeadboltAnchorIdentity, DeadboltOccurrenceContextTrace,
    DeterministicVerifierContextTraceV0, StandingPolicyContextV2, StandingPolicyRuleV2,
    StandingReplaySnapshot, StandingTraceReason, DEADBOLT_OCCURRENCE_SCHEMA,
    MAGPIE_CLAIMS_POLICY_ID, MAGPIE_CLAIMS_POLICY_V1_ID, MAGPIE_CLAIMS_POLICY_V2_ID,
    MAX_WITNESS_BYTES_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use serde_json::json;
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [92; 32];
const CLAIM_ID: &str = "claim-machine";
const SCOPE: &str = "scope:machine";
const DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
const STATEMENT: &str =
    "sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn sha256_hex(value: &[u8]) -> String {
    hex::encode(Sha256::digest(value))
}

fn claim_metadata() -> String {
    format!(
        r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{DIGEST}"}}}}"#
    )
}

fn witness_metadata(claim_id: &str, scope: &str, predicate: &str, witness_hex: &str) -> String {
    format!(
        r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"{predicate}","subject_claim_id":"{claim_id}","scope_ref":"{scope}","witness_hex":"{witness_hex}"}}}}"#
    )
}

#[derive(Clone)]
struct MachinePath {
    evidence_id: String,
    edge_id: String,
    evidence_kind: String,
    actor_class: String,
    metadata: String,
}

impl MachinePath {
    fn matched(suffix: &str) -> Self {
        Self {
            evidence_id: format!("evidence-{suffix}"),
            edge_id: format!("edge-{suffix}"),
            evidence_kind: "DeterministicVerification".into(),
            actor_class: "SourceImporter".into(),
            metadata: witness_metadata(CLAIM_ID, SCOPE, "sha256_bytes_equals_v0", "616263"),
        }
    }
}

#[derive(Clone)]
struct MachineFixture {
    legacy_statement: String,
    typed_statement: String,
    content_hash: String,
    claim_metadata: String,
    paths: Vec<MachinePath>,
}

impl MachineFixture {
    fn matched() -> Self {
        Self {
            legacy_statement: STATEMENT.into(),
            typed_statement: STATEMENT.into(),
            content_hash: sha256_hex(STATEMENT.as_bytes()),
            claim_metadata: claim_metadata(),
            paths: vec![MachinePath::matched("a")],
        }
    }

    fn records(&self) -> Vec<Vec<u8>> {
        let store = MemStore::new();
        {
            let mut clock = 0u64;
            let mut writer = LogWriter::open_with_clock(
                store.clone(),
                signing_key(),
                Box::new(move || {
                    clock += 1;
                    clock
                }),
            )
            .unwrap();
            writer
                .append(
                    Provenance::new("test", "standing-v2"),
                    Payload::ClaimAsserted {
                        claim_id: CLAIM_ID.into(),
                        statement: self.legacy_statement.clone(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
            writer
                .append(
                    Provenance::new("test", "standing-v2"),
                    Payload::ClaimAssertedV2 {
                        claim_id: CLAIM_ID.into(),
                        statement: self.typed_statement.clone(),
                        scope_ref: SCOPE.into(),
                        actor_class: "AgentProposer".into(),
                        content_hash: self.content_hash.clone(),
                        metadata_json: self.claim_metadata.clone(),
                    },
                )
                .unwrap();
            for path in &self.paths {
                writer
                    .append(
                        Provenance::new("test", "standing-v2"),
                        Payload::EvidenceRegistered {
                            evidence_id: path.evidence_id.clone(),
                            evidence_kind: path.evidence_kind.clone(),
                            summary: "inline witness only".into(),
                            scope_ref: SCOPE.into(),
                            actor_class: path.actor_class.clone(),
                            content_hash: String::new(),
                            metadata_json: path.metadata.clone(),
                        },
                    )
                    .unwrap();
                writer
                    .append(
                        Provenance::new("test", "standing-v2"),
                        Payload::JustificationEdgeRecorded {
                            edge_id: path.edge_id.clone(),
                            edge_kind: "supports".into(),
                            source_id: path.evidence_id.clone(),
                            target_id: CLAIM_ID.into(),
                            scope_ref: SCOPE.into(),
                            actor_class: "HumanRoot".into(),
                            rationale: "candidate direct support".into(),
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
}

fn assert_failure(fixture: MachineFixture, expected: DeterministicVerifierContextTraceV0) {
    let snapshot = fixture.snapshot();
    let resolution = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert_eq!(
        resolution.blockers,
        vec![
            StandingTraceReason::RequiresVerifierContext,
            StandingTraceReason::CeilingIsCandidateOnly,
        ]
    );
    assert_eq!(resolution.trace.len(), 1);
    let application = resolution.trace[0]
        .application
        .as_ref()
        .expect("exact candidate must attempt v2");
    assert_eq!(
        application.rule,
        StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0
    );
    assert_eq!(application.achieved_standing, None);
    assert_eq!(
        application.context,
        StandingPolicyContextV2::DeterministicVerifierV0 { trace: expected }
    );
}

#[test]
fn policy_v2_has_exact_fixed_identity() {
    assert_eq!(MAGPIE_CLAIMS_POLICY_V2_ID, "magpie-claims-standing-v2");
}

#[test]
fn v2_is_selected_only_by_explicit_snapshot_methods() {
    let snapshot = MachineFixture::matched().snapshot();
    assert_eq!(
        snapshot.resolved_standing_v2(CLAIM_ID),
        Some(Status::Supported)
    );
    assert_eq!(
        snapshot.resolved_standing_with_trace_v2(CLAIM_ID).policy_id,
        MAGPIE_CLAIMS_POLICY_V2_ID
    );
}

#[test]
fn v0_default_resolver_remains_v0() {
    let snapshot = MachineFixture::matched().snapshot();
    let resolution = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    assert_eq!(resolution.policy_id, MAGPIE_CLAIMS_POLICY_ID);
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
}

#[test]
fn v1_explicit_resolver_remains_v1() {
    let snapshot = MachineFixture::matched().snapshot();
    let resolution = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    assert_eq!(resolution.policy_id, MAGPIE_CLAIMS_POLICY_V1_ID);
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
}

#[test]
fn sha256_witness_exact_match_supports_only_under_v2() {
    let snapshot = MachineFixture::matched().snapshot();
    assert_eq!(
        snapshot.standing().resolved_standing(CLAIM_ID),
        Some(Status::Conjectured)
    );
    assert_eq!(
        snapshot.resolved_standing_v1(CLAIM_ID),
        Some(Status::Conjectured)
    );
    assert_eq!(
        snapshot.resolved_standing_v2(CLAIM_ID),
        Some(Status::Supported)
    );
}

#[test]
fn one_successful_receipt_reaches_supported_not_settled() {
    let snapshot = MachineFixture::matched().snapshot();
    let resolution = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    assert!(resolution.resolution_failure.is_none());
    assert_eq!(resolution.governed_standing, Some(Status::Supported));
    assert_ne!(resolution.governed_standing, Some(Status::Settled));
    assert_eq!(
        resolution.trace[0]
            .application
            .as_ref()
            .unwrap()
            .achieved_standing,
        Some(Status::Supported)
    );
}

#[test]
fn matched_v2_application_retains_complete_verifier_receipt() {
    let snapshot = MachineFixture::matched().snapshot();
    let resolution = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    let application = resolution.trace[0].application.as_ref().unwrap();
    assert_eq!(
        application.rule,
        StandingPolicyRuleV2::Sha256BytesEqualsDirectSupportV0
    );
    let StandingPolicyContextV2::DeterministicVerifierV0 {
        trace: DeterministicVerifierContextTraceV0::Matched(receipt),
    } = &application.context
    else {
        panic!("match must retain the receipt")
    };
    assert_eq!(receipt.claim_id(), CLAIM_ID);
    assert_eq!(receipt.evidence_id(), "evidence-a");
    assert_eq!(receipt.edge_id(), "edge-a");
    assert_eq!(receipt.scope_ref(), SCOPE);
    assert_eq!(receipt.canonical_statement(), STATEMENT);
    assert_eq!(
        receipt.claim_content_hash(),
        sha256_hex(STATEMENT.as_bytes())
    );
    assert_eq!(receipt.expected_sha256(), DIGEST);
    assert_eq!(receipt.computed_sha256(), DIGEST);
    assert_eq!(receipt.witness_len(), 3);
}

#[test]
fn successful_v2_resolution_clears_its_generic_context_blockers() {
    assert!(MachineFixture::matched()
        .snapshot()
        .resolved_standing_with_trace_v2(CLAIM_ID)
        .blockers
        .is_empty());
}

#[test]
fn sha256_witness_digest_mismatch_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata =
        witness_metadata(CLAIM_ID, SCOPE, "sha256_bytes_equals_v0", "616264");
    assert_failure(fixture, DeterministicVerifierContextTraceV0::DigestMismatch);
}

#[test]
fn wrong_claim_binding_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata =
        witness_metadata("other-claim", SCOPE, "sha256_bytes_equals_v0", "616263");
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::ClaimBindingMismatch,
    );
}

#[test]
fn wrong_scope_binding_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata =
        witness_metadata(CLAIM_ID, "scope:other", "sha256_bytes_equals_v0", "616263");
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::ScopeBindingMismatch,
    );
}

#[test]
fn wrong_predicate_binding_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata = witness_metadata(CLAIM_ID, SCOPE, "other_predicate", "616263");
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::PredicateBindingMismatch,
    );
}

#[test]
fn statement_mismatch_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.legacy_statement = "changed".into();
    fixture.typed_statement = "changed".into();
    fixture.content_hash = sha256_hex(b"changed");
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::StatementPredicateMismatch,
    );
}

#[test]
fn claim_content_hash_mismatch_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.content_hash = "0".repeat(64);
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::ClaimContentHashMismatch,
    );
}

#[test]
fn malformed_witness_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata = "{}".into();
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::MissingVerificationWitness,
    );
}

#[test]
fn oversized_witness_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata = witness_metadata(
        CLAIM_ID,
        SCOPE,
        "sha256_bytes_equals_v0",
        &"aa".repeat(MAX_WITNESS_BYTES_V0 + 1),
    );
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::WitnessTooLarge,
    );
}

#[test]
fn bare_deterministic_label_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata = "{}".into();
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::MissingVerificationWitness,
    );
}

#[test]
fn automated_verifier_actor_class_does_not_support_without_match() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].actor_class = "AutomatedVerifier".into();
    fixture.paths[0].metadata = "{}".into();
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::MissingVerificationWitness,
    );
}

#[test]
fn verified_boolean_does_not_support() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata = r#"{"verified":true}"#.into();
    assert_failure(
        fixture,
        DeterministicVerifierContextTraceV0::UnknownWitnessKey,
    );
}

#[test]
fn caller_constructed_receipt_is_never_policy_input() {
    let snapshot = MachineFixture::matched().snapshot();
    let before = snapshot.canonical_bytes();
    let _audit_copy = snapshot
        .resolve_deterministic_verifier_context_v0(CLAIM_ID, "evidence-a", "edge-a")
        .unwrap();
    assert_eq!(
        snapshot.resolved_standing_v2(CLAIM_ID),
        Some(Status::Supported)
    );
    assert_eq!(before, snapshot.canonical_bytes());
}

#[test]
fn multiple_successful_receipts_do_not_amplify() {
    let mut fixture = MachineFixture::matched();
    fixture.paths.push(MachinePath::matched("b"));
    let resolution = fixture.snapshot().resolved_standing_with_trace_v2(CLAIM_ID);
    assert_eq!(resolution.trace.len(), 2);
    assert!(resolution.trace.iter().all(|entry| entry
        .application
        .as_ref()
        .is_some_and(|application| application.achieved_standing == Some(Status::Supported))));
    assert_eq!(resolution.governed_standing, Some(Status::Supported));
    assert_ne!(resolution.governed_standing, Some(Status::Settled));
    assert!(resolution.blockers.is_empty());
}

#[test]
fn successful_path_is_not_vetoed_by_failed_path() {
    let mut fixture = MachineFixture::matched();
    let mut failed = MachinePath::matched("b");
    failed.metadata = witness_metadata(CLAIM_ID, SCOPE, "sha256_bytes_equals_v0", "616264");
    fixture.paths.push(failed);
    let resolution = fixture.snapshot().resolved_standing_with_trace_v2(CLAIM_ID);
    assert_eq!(resolution.governed_standing, Some(Status::Supported));
    assert_eq!(resolution.trace.len(), 2);
    assert_eq!(
        resolution.trace[0]
            .application
            .as_ref()
            .unwrap()
            .achieved_standing,
        Some(Status::Supported)
    );
    assert_eq!(
        resolution.trace[1]
            .application
            .as_ref()
            .unwrap()
            .achieved_standing,
        None
    );
    assert!(matches!(
        resolution.trace[1].application.as_ref().unwrap().context,
        StandingPolicyContextV2::DeterministicVerifierV0 {
            trace: DeterministicVerifierContextTraceV0::DigestMismatch
        }
    ));
    assert_eq!(
        resolution.blockers,
        vec![
            StandingTraceReason::RequiresVerifierContext,
            StandingTraceReason::CeilingIsCandidateOnly,
        ]
    );
}

fn deadbolt_snapshot() -> StandingReplaySnapshot {
    let identity = DeadboltAnchorIdentity {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "kernel-decision-witness-v1".into(),
        run_id: "policy-v2-run".into(),
    };
    let identity_json = json!({
        "schema": DEADBOLT_OCCURRENCE_SCHEMA,
        "bundle_kind": identity.bundle_kind,
        "witness_root": identity.witness_root,
        "witness_algorithm": identity.witness_algorithm,
        "canonicalization_profile": identity.canonicalization_profile,
        "run_id": identity.run_id,
    });
    let store = MemStore::new();
    {
        let mut clock = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                clock += 1;
                clock
            }),
        )
        .unwrap();
        let payloads = vec![
            Payload::ClaimAssertedV2 {
                claim_id: "claim-deadbolt".into(),
                statement: "Exact Deadbolt occurrence".into(),
                scope_ref: "scope:deadbolt".into(),
                actor_class: "AgentProposer".into(),
                content_hash: String::new(),
                metadata_json: json!({"claim_domain":"Occurrence/Inclusion","deadbolt_occurrence":identity_json.clone()}).to_string(),
            },
            Payload::EvidenceRegistered {
                evidence_id: "evidence-deadbolt".into(),
                evidence_kind: "DeadboltAnchor".into(),
                summary: "exact reference".into(),
                scope_ref: "scope:deadbolt".into(),
                actor_class: "DeadboltAnchorer".into(),
                content_hash: String::new(),
                metadata_json: json!({"deadbolt_anchor_ref":identity_json}).to_string(),
            },
            Payload::JustificationEdgeRecorded {
                edge_id: "edge-deadbolt".into(),
                edge_kind: "supports".into(),
                source_id: "evidence-deadbolt".into(),
                target_id: "claim-deadbolt".into(),
                scope_ref: "scope:deadbolt".into(),
                actor_class: "HumanRoot".into(),
                rationale: "exact occurrence".into(),
                metadata_json: "{}".into(),
            },
            Payload::SegmentAnchored {
                bundle_kind: identity.bundle_kind,
                witness_root: identity.witness_root,
                witness_algorithm: identity.witness_algorithm,
                canonicalization_profile: identity.canonicalization_profile,
                run_id: identity.run_id,
            },
        ];
        for payload in payloads {
            writer
                .append(Provenance::new("test", "standing-v2-deadbolt"), payload)
                .unwrap();
        }
    }
    let reader = LogReader::open(store, signing_key().verifying_key());
    replay_standing_context(&reader).unwrap()
}

#[test]
fn policy_v2_inherits_deadbolt_settlement_exactly() {
    let snapshot = deadbolt_snapshot();
    assert_eq!(
        snapshot.resolved_standing_v1("claim-deadbolt"),
        Some(Status::Settled)
    );
    assert_eq!(
        snapshot.resolved_standing_v2("claim-deadbolt"),
        Some(Status::Settled)
    );
}

#[test]
fn deadbolt_settled_is_not_downgraded_by_v2() {
    assert_eq!(
        deadbolt_snapshot().resolved_standing_v2("claim-deadbolt"),
        Some(Status::Settled)
    );
}

#[test]
fn deadbolt_v2_context_retains_exact_v1_trace() {
    let snapshot = deadbolt_snapshot();
    let v1 = snapshot.resolved_standing_with_trace_v1("claim-deadbolt");
    let v2 = snapshot.resolved_standing_with_trace_v2("claim-deadbolt");
    let inherited = v1.trace[0].application.as_ref().unwrap();
    let converted = v2.trace[0].application.as_ref().unwrap();
    assert_eq!(
        converted.rule,
        StandingPolicyRuleV2::DeadboltOccurrenceInclusionV1
    );
    assert_eq!(converted.achieved_standing, inherited.achieved_standing);
    assert_eq!(
        converted.context,
        StandingPolicyContextV2::DeadboltOccurrenceV1 {
            trace: inherited.context.clone()
        }
    );
    assert!(matches!(
        inherited.context,
        DeadboltOccurrenceContextTrace::Matched { .. }
    ));
}

#[test]
fn single_success_clears_only_its_generic_limitations() {
    successful_v2_resolution_clears_its_generic_context_blockers();
}

#[test]
fn single_failure_retains_generic_limitations() {
    sha256_witness_digest_mismatch_does_not_support();
}

#[test]
fn mixed_success_and_failure_retains_unresolved_limitations() {
    successful_path_is_not_vetoed_by_failed_path();
}

#[test]
fn all_successful_paths_clear_generic_limitations() {
    multiple_successful_receipts_do_not_amplify();
}

#[test]
fn non_context_v0_blockers_are_preserved() {
    let snapshot = MachineFixture::matched().snapshot();
    let missing = snapshot.resolved_standing_with_trace_v2("missing-claim");
    assert_eq!(
        missing.blockers,
        vec![
            StandingTraceReason::MissingTargetClaim,
            StandingTraceReason::MissingTypedTargetClaim,
        ]
    );
}

#[test]
fn policy_v2_replays_byte_identically() {
    let fixture = MachineFixture::matched();
    let records = fixture.records();
    let reader = LogReader::open(
        MemStore::from_records(records),
        signing_key().verifying_key(),
    );
    let first = replay_standing_context(&reader).unwrap();
    let second = replay_standing_context(&reader).unwrap();
    assert_eq!(
        first
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
        second
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes()
    );
}

#[test]
fn snapshot_bytes_do_not_change_after_v2_query() {
    let snapshot = MachineFixture::matched().snapshot();
    let before = snapshot.canonical_bytes();
    let _ = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    assert_eq!(before, snapshot.canonical_bytes());
}

#[test]
fn v2_query_does_not_mutate_v0_or_v1_results() {
    let snapshot = MachineFixture::matched().snapshot();
    let v0 = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    let v1 = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    let _ = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    assert_eq!(
        v0,
        snapshot.standing().resolved_standing_with_trace(CLAIM_ID)
    );
    assert_eq!(v1, snapshot.resolved_standing_with_trace_v1(CLAIM_ID));
}

#[test]
fn v0_literal_canonical_bytes_remain_unchanged() {
    let bytes = MachineFixture::matched()
        .snapshot()
        .standing()
        .resolved_standing_with_trace(CLAIM_ID)
        .canonical_bytes();
    assert_eq!(
        String::from_utf8(bytes).unwrap(),
        r#"{"claim_id":"claim-machine","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v0","trace":[{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]}],"blockers":["requires_verifier_context","ceiling_is_candidate_only"]}"#
    );
}

#[test]
fn v1_literal_canonical_bytes_remain_unchanged() {
    let bytes = MachineFixture::matched()
        .snapshot()
        .resolved_standing_with_trace_v1(CLAIM_ID)
        .canonical_bytes();
    assert_eq!(
        String::from_utf8(bytes).unwrap(),
        r#"{"claim_id":"claim-machine","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v1","trace":[{"candidate":{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":null}],"blockers":["requires_verifier_context","ceiling_is_candidate_only"]}"#
    );
}

#[test]
fn policy_v2_resolution_has_literal_canonical_bytes() {
    let resolution = MachineFixture::matched()
        .snapshot()
        .resolved_standing_with_trace_v2(CLAIM_ID);
    let value = String::from_utf8(resolution.canonical_bytes()).unwrap();
    assert_eq!(
        value,
        r#"{"claim_id":"claim-machine","governed_standing":"Supported","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[{"candidate":{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"sha256_bytes_equals_direct_support_v0","context":{"kind":"deterministic_verifier_v0","trace":{"outcome":"matched","receipt":{"predicate_schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","claim_id":"claim-machine","evidence_id":"evidence-a","edge_id":"edge-a","scope_ref":"scope:machine","canonical_statement":"sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","claim_content_hash":"70cea2ac9ed9d373b422ef3543e45ddaf289fa4e64a8aed726fc6b13df797421","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","witness_len":3}}},"achieved_standing":"Supported"}}],"blockers":[]}"#
    );
}

#[test]
fn policy_v2_failure_has_literal_canonical_bytes() {
    let mut fixture = MachineFixture::matched();
    fixture.paths[0].metadata =
        witness_metadata(CLAIM_ID, SCOPE, "sha256_bytes_equals_v0", "616264");
    let value = String::from_utf8(
        fixture
            .snapshot()
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
    )
    .unwrap();
    assert_eq!(
        value,
        r#"{"claim_id":"claim-machine","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[{"candidate":{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"sha256_bytes_equals_direct_support_v0","context":{"kind":"deterministic_verifier_v0","trace":{"outcome":"digest_mismatch"}},"achieved_standing":null}}],"blockers":["requires_verifier_context","ceiling_is_candidate_only"]}"#
    );
}

#[test]
fn inherited_deadbolt_v2_has_literal_canonical_bytes() {
    let value = String::from_utf8(
        deadbolt_snapshot()
            .resolved_standing_with_trace_v2("claim-deadbolt")
            .canonical_bytes(),
    )
    .unwrap();
    assert_eq!(
        value,
        r#"{"claim_id":"claim-deadbolt","governed_standing":"Settled","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[{"candidate":{"edge_id":"edge-deadbolt","source_id":"evidence-deadbolt","target_id":"claim-deadbolt","evidence_kind":"DeadboltAnchor","claim_domain":"Occurrence/Inclusion","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"deadbolt_occurrence_inclusion_v1","context":{"kind":"deadbolt_occurrence_v1","trace":{"outcome":"matched","occurrence":{"identity":{"bundle_kind":"kernel-decision-witness","witness_root":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","witness_algorithm":"sha256","canonicalization_profile":"kernel-decision-witness-v1","run_id":"policy-v2-run"},"sequence":4,"event_hash":"bfbcaf54b0a1718b3d0af231da34fba8ba44dfed47297989a469bdd528aeb5f0","provenance_agent":"test","provenance_source":"standing-v2-deadbolt"}}},"achieved_standing":"Settled"}}],"blockers":[]}"#
    );
}

#[test]
fn mixed_v2_has_literal_canonical_bytes_and_exact_blocker_order() {
    let mut fixture = MachineFixture::matched();
    let mut failed = MachinePath::matched("b");
    failed.metadata = witness_metadata(CLAIM_ID, SCOPE, "sha256_bytes_equals_v0", "616264");
    fixture.paths.push(failed);
    let value = String::from_utf8(
        fixture
            .snapshot()
            .resolved_standing_with_trace_v2(CLAIM_ID)
            .canonical_bytes(),
    )
    .unwrap();
    assert_eq!(
        value,
        r#"{"claim_id":"claim-machine","governed_standing":"Supported","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v2","trace":[{"candidate":{"edge_id":"edge-a","source_id":"evidence-a","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"sha256_bytes_equals_direct_support_v0","context":{"kind":"deterministic_verifier_v0","trace":{"outcome":"matched","receipt":{"predicate_schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","claim_id":"claim-machine","evidence_id":"evidence-a","edge_id":"edge-a","scope_ref":"scope:machine","canonical_statement":"sha256_bytes_equals_v0:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","claim_content_hash":"70cea2ac9ed9d373b422ef3543e45ddaf289fa4e64a8aed726fc6b13df797421","expected_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","computed_sha256":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","witness_len":3}}},"achieved_standing":"Supported"}},{"candidate":{"edge_id":"edge-b","source_id":"evidence-b","target_id":"claim-machine","evidence_kind":"DeterministicVerification","claim_domain":"ExactMachineCheckable","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"sha256_bytes_equals_direct_support_v0","context":{"kind":"deterministic_verifier_v0","trace":{"outcome":"digest_mismatch"}},"achieved_standing":null}}],"blockers":["requires_verifier_context","ceiling_is_candidate_only"]}"#
    );
}
