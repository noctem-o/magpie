use magpie_claims::{
    replay_standing_context, DeadboltAnchorIdentity, DeadboltOccurrenceContextTrace, StandingClaim,
    StandingPolicyRule, StandingReplaySnapshot, StandingTraceReason, StandingView, TypedClaimNode,
    TypedEvidenceNode, TypedJustificationEdge, DEADBOLT_OCCURRENCE_SCHEMA, MAGPIE_CLAIMS_POLICY_ID,
    MAGPIE_CLAIMS_POLICY_V1_ID,
};
use magpie_log::{
    LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status, VerifyingKey,
};
use serde_json::{json, Value};

const SEED: [u8; 32] = [61u8; 32];
const CLAIM_ID: &str = "claim-policy-v1";
const EVIDENCE_ID: &str = "evidence-policy-v1";
const EDGE_ID: &str = "edge-policy-v1";
const SCOPE: &str = "scope:policy-v1";
const WITNESS_ROOT: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/anchor-log.jsonl"
);
const FIXTURE_VERIFYING_KEY_HEX: &str =
    "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn identity() -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: WITNESS_ROOT.into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "kernel-decision-witness-v1".into(),
        run_id: "policy-v1-run".into(),
    }
}

fn identity_json(identity: &DeadboltAnchorIdentity) -> Value {
    json!({
        "schema": DEADBOLT_OCCURRENCE_SCHEMA,
        "bundle_kind": identity.bundle_kind,
        "witness_root": identity.witness_root,
        "witness_algorithm": identity.witness_algorithm,
        "canonicalization_profile": identity.canonicalization_profile,
        "run_id": identity.run_id,
    })
}

fn claim_metadata(identity: &DeadboltAnchorIdentity) -> String {
    json!({
        "claim_domain": "Occurrence/Inclusion",
        "deadbolt_occurrence": identity_json(identity),
    })
    .to_string()
}

fn evidence_metadata(identity: &DeadboltAnchorIdentity) -> String {
    json!({"deadbolt_anchor_ref": identity_json(identity)}).to_string()
}

fn claim_payload(metadata_json: &str) -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: CLAIM_ID.into(),
        statement: "The exact structured SegmentAnchored identity occurs.".into(),
        scope_ref: SCOPE.into(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: metadata_json.into(),
    }
}

fn evidence_payload(
    evidence_id: &str,
    evidence_kind: &str,
    scope_ref: &str,
    metadata_json: &str,
) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: evidence_id.into(),
        evidence_kind: evidence_kind.into(),
        summary: "Exact occurrence reference.".into(),
        scope_ref: scope_ref.into(),
        actor_class: "DeadboltAnchorer".into(),
        content_hash: String::new(),
        metadata_json: metadata_json.into(),
    }
}

fn edge_payload(edge_id: &str, edge_kind: &str, source_id: &str, scope_ref: &str) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: edge_id.into(),
        edge_kind: edge_kind.into(),
        source_id: source_id.into(),
        target_id: CLAIM_ID.into(),
        scope_ref: scope_ref.into(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate exact occurrence support.".into(),
        metadata_json: "{}".into(),
    }
}

fn anchor_payload(identity: &DeadboltAnchorIdentity) -> Payload {
    Payload::SegmentAnchored {
        bundle_kind: identity.bundle_kind.clone(),
        witness_root: identity.witness_root.clone(),
        witness_algorithm: identity.witness_algorithm.clone(),
        canonicalization_profile: identity.canonicalization_profile.clone(),
        run_id: identity.run_id.clone(),
    }
}

fn positive_payloads() -> Vec<Payload> {
    let identity = identity();
    vec![
        claim_payload(&claim_metadata(&identity)),
        evidence_payload(
            EVIDENCE_ID,
            "DeadboltAnchor",
            SCOPE,
            &evidence_metadata(&identity),
        ),
        edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
        anchor_payload(&identity),
    ]
}

fn replay(payloads: Vec<Payload>) -> StandingReplaySnapshot {
    let store = MemStore::new();
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for payload in payloads {
            writer
                .append(Provenance::new("test", "deadbolt-standing-v1"), payload)
                .unwrap();
        }
    }
    let reader = LogReader::open(store, signing_key().verifying_key());
    replay_standing_context(&reader).unwrap()
}

fn context_for(
    claim_metadata_json: &str,
    evidence_metadata_json: &str,
    anchored_identity: &DeadboltAnchorIdentity,
) -> DeadboltOccurrenceContextTrace {
    let snapshot = replay(vec![
        claim_payload(claim_metadata_json),
        evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, evidence_metadata_json),
        edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
        anchor_payload(anchored_identity),
    ]);
    let resolution = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert_eq!(resolution.trace.len(), 1);
    let application = resolution.trace[0]
        .application
        .as_ref()
        .expect("eligible baseline must attempt the v1 rule");
    assert_eq!(application.achieved_standing, None);
    application.context.clone()
}

#[test]
fn deadbolt_occurrence_standing_v1_exact_match_settles_only_v1() {
    let snapshot = replay(positive_payloads());
    let v0 = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    let v1 = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);

    assert_eq!(v0.policy_id, MAGPIE_CLAIMS_POLICY_ID);
    assert_eq!(v1.policy_id, MAGPIE_CLAIMS_POLICY_V1_ID);
    assert_eq!(v0.governed_standing, Some(Status::Conjectured));
    assert_eq!(
        snapshot.standing().resolved_standing(CLAIM_ID),
        Some(Status::Conjectured)
    );
    assert_eq!(v1.governed_standing, Some(Status::Settled));
    assert_eq!(
        snapshot.resolved_standing_v1(CLAIM_ID),
        Some(Status::Settled)
    );
    assert!(v1.blockers.is_empty());
    assert_eq!(v1.trace.len(), 1);

    let entry = &v1.trace[0];
    assert_eq!(entry.candidate.edge_id.as_deref(), Some(EDGE_ID));
    assert_eq!(entry.candidate.source_id.as_deref(), Some(EVIDENCE_ID));
    let application = entry.application.as_ref().unwrap();
    assert_eq!(
        application.rule,
        StandingPolicyRule::DeadboltOccurrenceInclusionV1
    );
    assert_eq!(application.achieved_standing, Some(Status::Settled));
    let DeadboltOccurrenceContextTrace::Matched { occurrence } = &application.context else {
        panic!("exact match must retain the matched occurrence")
    };
    assert_eq!(occurrence.identity, identity());
    assert_eq!(occurrence.sequence, 4);
    assert_eq!(
        occurrence.event_hash,
        "ad01ff4e5cd122a9f2ab0435a07af2fb9d90a526cdad01ffa826693bc43ff4b2"
    );
}

#[test]
fn deadbolt_occurrence_standing_v1_reports_every_context_failure_exactly() {
    let anchored = identity();
    let valid_claim = claim_metadata(&anchored);
    let valid_evidence = evidence_metadata(&anchored);

    let malformed_claim = json!({
        "claim_domain": "Occurrence/Inclusion",
        "deadbolt_occurrence": {
            "schema": "wrong-schema",
            "bundle_kind": anchored.bundle_kind,
            "witness_root": anchored.witness_root,
            "witness_algorithm": anchored.witness_algorithm,
            "canonicalization_profile": anchored.canonicalization_profile,
            "run_id": anchored.run_id,
        }
    })
    .to_string();
    let malformed_evidence = json!({
        "deadbolt_anchor_ref": {
            "schema": DEADBOLT_OCCURRENCE_SCHEMA,
            "bundle_kind": anchored.bundle_kind,
            "witness_root": "not-lowercase-hex",
            "witness_algorithm": anchored.witness_algorithm,
            "canonicalization_profile": anchored.canonicalization_profile,
            "run_id": anchored.run_id,
        }
    })
    .to_string();

    for (claim, evidence, expected) in [
        (
            r#"{"claim_domain":"Occurrence/Inclusion"}"#.to_owned(),
            valid_evidence.clone(),
            DeadboltOccurrenceContextTrace::MissingClaimPredicate,
        ),
        (
            malformed_claim,
            valid_evidence.clone(),
            DeadboltOccurrenceContextTrace::MalformedClaimPredicate,
        ),
        (
            valid_claim.clone(),
            "{}".to_owned(),
            DeadboltOccurrenceContextTrace::MissingEvidenceReference,
        ),
        (
            valid_claim.clone(),
            malformed_evidence,
            DeadboltOccurrenceContextTrace::MalformedEvidenceReference,
        ),
    ] {
        assert_eq!(context_for(&claim, &evidence, &anchored), expected);
    }

    for field in [
        "witness_root",
        "bundle_kind",
        "witness_algorithm",
        "canonicalization_profile",
        "run_id",
    ] {
        let mut changed = anchored.clone();
        match field {
            "witness_root" => changed.witness_root = "f".repeat(64),
            "bundle_kind" => changed.bundle_kind.push_str("-other"),
            "witness_algorithm" => changed.witness_algorithm = "sha512".into(),
            "canonicalization_profile" => changed.canonicalization_profile.push_str("-other"),
            "run_id" => changed.run_id.push_str("-other"),
            _ => unreachable!(),
        }
        assert_eq!(
            context_for(&valid_claim, &evidence_metadata(&changed), &anchored),
            DeadboltOccurrenceContextTrace::PredicateReferenceMismatch,
            "{field} disagreement must be explicit"
        );
    }

    let mut unanchored = anchored.clone();
    unanchored.run_id = "unanchored-run".into();
    assert_eq!(
        context_for(
            &claim_metadata(&unanchored),
            &evidence_metadata(&unanchored),
            &anchored,
        ),
        DeadboltOccurrenceContextTrace::AnchorNotFound
    );
}

#[test]
fn deadbolt_occurrence_standing_v1_rejects_every_ineligible_policy_boundary() {
    let anchored = identity();
    let evidence = evidence_metadata(&anchored);
    let mut cases = Vec::new();
    for domain in [
        "Interpretation",
        "ExactMachineCheckable",
        "OperationalObservation",
    ] {
        cases.push(vec![
            claim_payload(
                &json!({"claim_domain": domain, "deadbolt_occurrence": identity_json(&anchored)})
                    .to_string(),
            ),
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ]);
    }
    for kind in ["DeterministicVerification", "ExternalSource"] {
        cases.push(vec![
            claim_payload(&claim_metadata(&anchored)),
            evidence_payload(EVIDENCE_ID, kind, SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ]);
    }
    cases.extend([
        vec![
            claim_payload(&claim_metadata(&anchored)),
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "contradicts", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            claim_payload(&claim_metadata(&anchored)),
            edge_payload(EDGE_ID, "supports", "missing-evidence", SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            claim_payload(&claim_metadata(&anchored)),
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", "scope:other", &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            Payload::ClaimAsserted {
                claim_id: CLAIM_ID.into(),
                statement: "Legacy target only.".into(),
                status: Status::Conjectured,
            },
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            claim_payload("{"),
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
        vec![
            claim_payload(r#"{"claim_domain":"UnknownDomain"}"#),
            evidence_payload(EVIDENCE_ID, "DeadboltAnchor", SCOPE, &evidence),
            edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            anchor_payload(&anchored),
        ],
    ]);

    for payloads in cases {
        let snapshot = replay(payloads);
        let baseline = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
        let v1 = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
        assert_eq!(v1.governed_standing, baseline.governed_standing);
        assert_eq!(v1.blockers, baseline.blockers);
        assert!(v1.trace.iter().all(|entry| entry.application.is_none()));
    }
}

#[test]
fn deadbolt_occurrence_standing_v1_unknown_evidence_kind_is_ineligible_and_unreplayable() {
    let mut view = StandingView::new();
    view.claims.insert(
        CLAIM_ID.into(),
        StandingClaim {
            statement: "Unknown-kind boundary.".into(),
            asserted_initial_status: Status::Conjectured,
            standing: Status::Conjectured,
            legacy_evidence: Vec::new(),
            legacy_transitions: 0,
        },
    );
    view.typed_claims.insert(
        CLAIM_ID.into(),
        TypedClaimNode {
            statement: "Unknown-kind boundary.".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AgentProposer".into(),
            content_hash: String::new(),
            metadata_json: claim_metadata(&identity()),
        },
    );
    view.typed_evidence.insert(
        EVIDENCE_ID.into(),
        TypedEvidenceNode {
            evidence_kind: "UnknownEvidenceKind".into(),
            summary: "Unknown kind.".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AgentProposer".into(),
            content_hash: String::new(),
            metadata_json: evidence_metadata(&identity()),
        },
    );
    view.justification_edges.insert(
        EDGE_ID.into(),
        TypedJustificationEdge {
            edge_kind: "supports".into(),
            source_id: EVIDENCE_ID.into(),
            target_id: CLAIM_ID.into(),
            scope_ref: SCOPE.into(),
            actor_class: "HumanRoot".into(),
            rationale: "Unknown kind.".into(),
            metadata_json: "{}".into(),
        },
    );
    let baseline = view.resolved_standing_with_trace(CLAIM_ID);
    assert_eq!(
        baseline.trace[0].reasons,
        vec![StandingTraceReason::UnknownEvidenceKind]
    );

    let store = MemStore::new();
    let mut writer = LogWriter::open(store, signing_key()).unwrap();
    let result = writer.append(
        Provenance::new("test", "unknown-kind"),
        evidence_payload(
            EVIDENCE_ID,
            "UnknownEvidenceKind",
            SCOPE,
            &evidence_metadata(&identity()),
        ),
    );
    assert!(
        result.is_err(),
        "unknown kinds cannot enter an accepted replay snapshot"
    );
}

#[test]
fn deadbolt_occurrence_standing_v1_authority_confusion_cannot_settle() {
    let anchored = identity();
    let store = MemStore::new();
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        writer
            .append(
                Provenance::new("test", "authority-confusion"),
                Payload::ClaimAssertedV2 {
                    claim_id: CLAIM_ID.into(),
                    statement: format!("Caller says occurrence {WITNESS_ROOT} is verified."),
                    scope_ref: SCOPE.into(),
                    actor_class: "HumanRoot".into(),
                    content_hash: WITNESS_ROOT.into(),
                    metadata_json:
                        r#"{"claim_domain":"Occurrence/Inclusion","verified":true,"admitted":true}"#
                            .into(),
                },
            )
            .unwrap();
        writer
            .append(
                Provenance::new("test", "authority-confusion"),
                Payload::EvidenceRegistered {
                    evidence_id: EVIDENCE_ID.into(),
                    evidence_kind: "DeadboltAnchor".into(),
                    summary: "verified and admitted".into(),
                    scope_ref: SCOPE.into(),
                    actor_class: "DeadboltAnchorer".into(),
                    content_hash: WITNESS_ROOT.into(),
                    metadata_json: r#"{"verified":true,"admitted":true}"#.into(),
                },
            )
            .unwrap();
        writer
            .append(
                Provenance::new("test", "authority-confusion"),
                edge_payload(EDGE_ID, "supports", EVIDENCE_ID, SCOPE),
            )
            .unwrap();
        writer
            .append(
                Provenance::new("test", "authority-confusion"),
                anchor_payload(&anchored),
            )
            .unwrap();
    }
    let reader = LogReader::open(store, signing_key().verifying_key());
    let snapshot = replay_standing_context(&reader).unwrap();
    let resolution = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);

    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    let application = resolution.trace[0].application.as_ref().unwrap();
    assert_eq!(application.achieved_standing, None);
    assert_eq!(
        application.context,
        DeadboltOccurrenceContextTrace::MissingClaimPredicate
    );
}

#[test]
fn deadbolt_occurrence_standing_v1_unresolved_path_does_not_veto_direct_proof() {
    let anchored = identity();
    let snapshot = replay(vec![
        claim_payload(&claim_metadata(&anchored)),
        evidence_payload(
            "evidence-matched",
            "DeadboltAnchor",
            SCOPE,
            &evidence_metadata(&anchored),
        ),
        edge_payload("edge-matched", "supports", "evidence-matched", SCOPE),
        evidence_payload("evidence-unresolved", "DeadboltAnchor", SCOPE, "{}"),
        edge_payload("edge-unresolved", "supports", "evidence-unresolved", SCOPE),
        anchor_payload(&anchored),
    ]);
    let resolution = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);

    assert_eq!(resolution.governed_standing, Some(Status::Settled));
    assert_eq!(resolution.trace.len(), 2);
    let matched = &resolution.trace[0];
    let unresolved = &resolution.trace[1];
    assert_eq!(matched.candidate.edge_id.as_deref(), Some("edge-matched"));
    assert_eq!(
        matched.application.as_ref().unwrap().achieved_standing,
        Some(Status::Settled)
    );
    assert_eq!(
        unresolved.candidate.edge_id.as_deref(),
        Some("edge-unresolved")
    );
    assert_eq!(
        unresolved.application.as_ref().unwrap().context,
        DeadboltOccurrenceContextTrace::MissingEvidenceReference
    );
    assert!(resolution
        .blockers
        .contains(&StandingTraceReason::RequiresVerifierContext));
    assert!(resolution
        .blockers
        .contains(&StandingTraceReason::CeilingIsCandidateOnly));
}

#[test]
fn deadbolt_occurrence_standing_v1_duplicate_anchor_uses_earliest_without_amplification() {
    let anchored = identity();
    let mut payloads = positive_payloads();
    payloads.push(anchor_payload(&anchored));
    let snapshot = replay(payloads);
    let resolution = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);

    assert_eq!(snapshot.anchors().occurrences(&anchored).len(), 2);
    assert_eq!(resolution.governed_standing, Some(Status::Settled));
    assert_eq!(resolution.trace.len(), 1);
    let application = resolution.trace[0].application.as_ref().unwrap();
    let DeadboltOccurrenceContextTrace::Matched { occurrence } = &application.context else {
        panic!("duplicate identity must still match")
    };
    assert_eq!(occurrence.sequence, 4);
    assert_eq!(application.achieved_standing, Some(Status::Settled));
}

#[test]
fn deadbolt_occurrence_standing_v1_regenerates_and_pins_canonical_bytes() {
    let store = MemStore::new();
    {
        let mut timestamp = 0u64;
        let mut writer = LogWriter::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for payload in positive_payloads() {
            writer
                .append(Provenance::new("test", "deadbolt-standing-v1"), payload)
                .unwrap();
        }
    }
    let reader = LogReader::open(store, signing_key().verifying_key());
    let first = replay_standing_context(&reader).unwrap();
    let second = replay_standing_context(&reader).unwrap();
    let first_resolution = first.resolved_standing_with_trace_v1(CLAIM_ID);
    let second_resolution = second.resolved_standing_with_trace_v1(CLAIM_ID);

    assert_eq!(first, second);
    assert_eq!(first_resolution, second_resolution);
    assert_eq!(
        first_resolution.canonical_bytes(),
        second_resolution.canonical_bytes()
    );
    assert_eq!(
        String::from_utf8(first_resolution.canonical_bytes()).unwrap(),
        r#"{"claim_id":"claim-policy-v1","governed_standing":"Settled","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v1","trace":[{"candidate":{"edge_id":"edge-policy-v1","source_id":"evidence-policy-v1","target_id":"claim-policy-v1","evidence_kind":"DeadboltAnchor","claim_domain":"Occurrence/Inclusion","candidate_ceiling":"Settled","reasons":["accepted_candidate","requires_verifier_context","ceiling_is_candidate_only"]},"application":{"rule":"deadbolt_occurrence_inclusion_v1","context":{"outcome":"matched","occurrence":{"identity":{"bundle_kind":"kernel-decision-witness","witness_root":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","witness_algorithm":"sha256","canonicalization_profile":"kernel-decision-witness-v1","run_id":"policy-v1-run"},"sequence":4,"event_hash":"ad01ff4e5cd122a9f2ab0435a07af2fb9d90a526cdad01ffa826693bc43ff4b2","provenance_agent":"test","provenance_source":"deadbolt-standing-v1"}},"achieved_standing":"Settled"}}],"blockers":[]}"#
    );
    assert_eq!(
        first.standing().resolved_standing(CLAIM_ID),
        Some(Status::Conjectured)
    );
}

fn fixture_store() -> MemStore {
    let data = std::fs::read(FIXTURE_PATH).expect("portable anchor fixture must be committed");
    MemStore::from_records(
        data.split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect(),
    )
}

fn fixture_verifying_key() -> VerifyingKey {
    VerifyingKey::from_bytes(&decode_hex_32(FIXTURE_VERIFYING_KEY_HEX))
        .expect("fixture verifying key must be valid")
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    let mut output = [0u8; 32];
    for (index, slot) in output.iter_mut().enumerate() {
        *slot = (hex_nibble(value.as_bytes()[index * 2]) << 4)
            | hex_nibble(value.as_bytes()[index * 2 + 1]);
    }
    output
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("fixture hex must be lowercase"),
    }
}

#[test]
fn deadbolt_occurrence_standing_v1_anchor_only_fixture_creates_no_proposition() {
    let reader = LogReader::open(fixture_store(), fixture_verifying_key());
    let snapshot = replay_standing_context(&reader).unwrap();
    let resolution = snapshot.resolved_standing_with_trace_v1("portable-run-0001");

    assert_eq!(snapshot.event_count(), 2);
    assert_eq!(snapshot.anchors().len(), 1);
    assert!(snapshot.standing().is_structurally_empty());
    assert_eq!(snapshot.resolved_standing_v1("portable-run-0001"), None);
    assert_eq!(resolution.governed_standing, None);
    assert!(resolution
        .trace
        .iter()
        .all(|entry| entry.application.is_none()));
}
