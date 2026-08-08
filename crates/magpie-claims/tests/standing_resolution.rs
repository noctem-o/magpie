use magpie_claims::{
    policy::{
        support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
        SupportContextRequirement,
    },
    StandingClaim, StandingCurrentness, StandingResolution, StandingTraceReason, StandingView,
    TypedClaimNode, TypedEvidenceNode, TypedJustificationEdge, MAGPIE_CLAIMS_POLICY_ID,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};

const CLAIM_ID: &str = "claim-1";
const EVIDENCE_ID: &str = "evidence-1";
const EDGE_ID: &str = "edge-1";
const SCOPE: &str = "scope:one";
const SEED: [u8; 32] = [23u8; 32];

fn candidate_view(claim_metadata: &str, evidence_kind: &str) -> StandingView {
    let mut view = StandingView::new();
    view.claims.insert(
        CLAIM_ID.into(),
        StandingClaim {
            statement: "A governed claim.".into(),
            asserted_initial_status: Status::Conjectured,
            standing: Status::Conjectured,
            legacy_evidence: Vec::new(),
            legacy_transitions: 0,
        },
    );
    view.typed_claims.insert(
        CLAIM_ID.into(),
        TypedClaimNode {
            statement: "A governed claim.".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AgentProposer".into(),
            content_hash: String::new(),
            metadata_json: claim_metadata.into(),
        },
    );
    view.typed_evidence.insert(
        EVIDENCE_ID.into(),
        TypedEvidenceNode {
            evidence_kind: evidence_kind.into(),
            summary: "Candidate support.".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AutomatedVerifier".into(),
            content_hash: String::new(),
            metadata_json: "{}".into(),
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
            rationale: "Candidate support edge.".into(),
            metadata_json: "{}".into(),
        },
    );
    view
}

fn only_edge(resolution: &StandingResolution) -> &magpie_claims::StandingTraceEntry {
    assert_eq!(resolution.trace.len(), 1);
    &resolution.trace[0]
}

fn assert_fail_closed_metadata(metadata: &str, expected: StandingTraceReason) {
    let resolution =
        candidate_view(metadata, "ExternalSource").resolved_standing_with_trace(CLAIM_ID);

    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert_eq!(resolution.legacy_raw_standing, Some(Status::Conjectured));
    assert_eq!(only_edge(&resolution).candidate_ceiling, None);
    assert_eq!(only_edge(&resolution).reasons, vec![expected]);
    assert!(resolution.blockers.contains(&expected));
}

#[test]
fn malformed_metadata_refuses_candidate_support() {
    assert_fail_closed_metadata("{", StandingTraceReason::MalformedMetadata);
}

#[test]
fn missing_claim_domain_refuses_candidate_support() {
    assert_fail_closed_metadata("{}", StandingTraceReason::MissingClaimDomain);
}

#[test]
fn unknown_claim_domain_refuses_candidate_support() {
    assert_fail_closed_metadata(
        r#"{"claim_domain":"HumanOverride"}"#,
        StandingTraceReason::UnknownClaimDomain,
    );
}

#[test]
fn wrong_type_claim_domain_refuses_candidate_support() {
    assert_fail_closed_metadata(
        r#"{"claim_domain":7}"#,
        StandingTraceReason::WrongTypeClaimDomain,
    );
}

#[test]
fn duplicate_metadata_keys_refuse_candidate_support() {
    assert_fail_closed_metadata(
        r#"{"claim_domain":"ExternalReport","claim_domain":"ExternalReport"}"#,
        StandingTraceReason::DuplicateMetadataKey,
    );
}

#[test]
fn human_ratification_requires_admission_and_contributes_nothing() {
    let resolution = candidate_view(r#"{"claim_domain":"HumanJudgment"}"#, "HumanRatification")
        .resolved_standing_with_trace(CLAIM_ID);
    let entry = only_edge(&resolution);

    assert_eq!(entry.candidate_ceiling, Some(Status::Supported));
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert!(entry
        .reasons
        .contains(&StandingTraceReason::RequiresAdmission));
    assert!(entry
        .reasons
        .contains(&StandingTraceReason::CeilingIsCandidateOnly));
}

#[test]
fn bare_deadbolt_anchor_requires_verifier_context_and_does_not_settle() {
    let resolution = candidate_view(
        r#"{"claim_domain":"Occurrence/Inclusion"}"#,
        "DeadboltAnchor",
    )
    .resolved_standing_with_trace(CLAIM_ID);
    let entry = only_edge(&resolution);

    assert_eq!(entry.candidate_ceiling, Some(Status::Settled));
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert!(entry
        .reasons
        .contains(&StandingTraceReason::RequiresVerifierContext));
}

#[test]
fn deterministic_verification_candidate_does_not_settle_without_context() {
    let resolution = candidate_view(
        r#"{"claim_domain":"ExactMachineCheckable"}"#,
        "DeterministicVerification",
    )
    .resolved_standing_with_trace(CLAIM_ID);
    let entry = only_edge(&resolution);

    assert_eq!(entry.candidate_ceiling, Some(Status::Settled));
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert!(entry
        .reasons
        .contains(&StandingTraceReason::RequiresVerifierContext));
}

#[test]
fn support_ceiling_is_reported_as_candidate_only() {
    let resolution = candidate_view(
        r#"{"claim_domain":"ExternalReport","unrelated":{"standing":"Settled"}}"#,
        "ExternalSource",
    )
    .resolved_standing_with_trace(CLAIM_ID);
    let entry = only_edge(&resolution);

    assert_eq!(resolution.policy_id, MAGPIE_CLAIMS_POLICY_ID);
    assert_eq!(resolution.currentness, StandingCurrentness::Unknown);
    assert_eq!(entry.candidate_ceiling, Some(Status::Supported));
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert_eq!(
        entry.reasons,
        vec![
            StandingTraceReason::AcceptedCandidate,
            StandingTraceReason::CeilingIsCandidateOnly,
        ]
    );
    assert_eq!(
        resolution.canonical_bytes(),
        br#"{"claim_id":"claim-1","governed_standing":"Conjectured","legacy_raw_standing":"Conjectured","currentness":"unknown","policy_id":"magpie-claims-standing-v0","trace":[{"edge_id":"edge-1","source_id":"evidence-1","target_id":"claim-1","evidence_kind":"ExternalSource","claim_domain":"ExternalReport","candidate_ceiling":"Supported","reasons":["accepted_candidate","ceiling_is_candidate_only"]}],"blockers":["ceiling_is_candidate_only"]}"#
            .to_vec()
    );
}

#[test]
fn legacy_status_change_is_visible_but_quarantined() {
    let mut view = candidate_view(r#"{"claim_domain":"ExternalReport"}"#, "ExternalSource");
    let claim = view.claims.get_mut(CLAIM_ID).unwrap();
    claim.standing = Status::Settled;
    claim.legacy_transitions = 1;

    let resolution = view.resolved_standing_with_trace(CLAIM_ID);

    assert_eq!(resolution.legacy_raw_standing, Some(Status::Settled));
    assert_eq!(resolution.governed_standing, Some(Status::Conjectured));
    assert_eq!(view.resolved_standing(CLAIM_ID), Some(Status::Conjectured));
    assert_eq!(resolution.trace[0].edge_id, None);
    assert_eq!(
        resolution.trace[0].reasons,
        vec![StandingTraceReason::LegacyRawStatusQuarantined]
    );
}

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn replay_payloads(payloads: Vec<Payload>) -> StandingResolution {
    let store = MemStore::new();
    {
        let mut tick = 0u64;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                tick += 1;
                tick
            }),
        )
        .unwrap();
        for payload in payloads {
            writer
                .append(Provenance::new("test", "standing-resolution"), payload)
                .unwrap();
        }
    }

    let reader = LogReader::open(store, signing_key().verifying_key());
    let mut view = StandingView::new();
    reader.replay(&mut view).unwrap();
    view.resolved_standing_with_trace(CLAIM_ID)
}

fn claim_payload() -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: CLAIM_ID.into(),
        statement: "A governed claim.".into(),
        scope_ref: SCOPE.into(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
    }
}

fn evidence_payload(id: &str) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: id.into(),
        evidence_kind: "ExternalSource".into(),
        summary: "Independent candidate support.".into(),
        scope_ref: SCOPE.into(),
        actor_class: "AutomatedVerifier".into(),
        content_hash: String::new(),
        metadata_json: "{}".into(),
    }
}

fn edge_payload(id: &str, source_id: &str) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: id.into(),
        edge_kind: "supports".into(),
        source_id: source_id.into(),
        target_id: CLAIM_ID.into(),
        scope_ref: SCOPE.into(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate support.".into(),
        metadata_json: "{}".into(),
    }
}

#[test]
fn trace_is_deterministic_under_verified_replay() {
    let payloads = vec![
        claim_payload(),
        evidence_payload("evidence-a"),
        edge_payload("edge-a", "evidence-a"),
    ];

    let first = replay_payloads(payloads.clone());
    let second = replay_payloads(payloads);

    assert_eq!(first, second);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
}

#[test]
fn independent_event_permutation_produces_identical_resolution() {
    let forward = replay_payloads(vec![
        claim_payload(),
        evidence_payload("evidence-a"),
        evidence_payload("evidence-b"),
        edge_payload("edge-a", "evidence-a"),
        edge_payload("edge-b", "evidence-b"),
    ]);
    let reversed = replay_payloads(vec![
        edge_payload("edge-b", "evidence-b"),
        evidence_payload("evidence-b"),
        edge_payload("edge-a", "evidence-a"),
        evidence_payload("evidence-a"),
        claim_payload(),
    ]);

    assert_eq!(forward, reversed);
    assert_eq!(
        forward
            .trace
            .iter()
            .filter_map(|entry| entry.edge_id.as_deref())
            .collect::<Vec<_>>(),
        vec!["edge-a", "edge-b"]
    );
}

#[test]
fn every_relevant_ignored_edge_has_a_closed_reason() {
    let mut view = candidate_view(
        r#"{"claim_domain":"OperationalObservation"}"#,
        "ExecutionEvidence",
    );
    view.justification_edges.clear();
    view.typed_evidence.insert(
        "unknown-evidence".into(),
        TypedEvidenceNode {
            evidence_kind: "HumanOverride".into(),
            summary: "Unknown kind.".into(),
            scope_ref: SCOPE.into(),
            actor_class: "AgentProposer".into(),
            content_hash: String::new(),
            metadata_json: "{}".into(),
        },
    );

    for (edge_id, edge_kind, source_id, scope_ref) in [
        ("edge-a", "derived_from", EVIDENCE_ID, SCOPE),
        ("edge-b", "supports", "missing", SCOPE),
        ("edge-c", "supports", EVIDENCE_ID, "scope:other"),
        ("edge-d", "supports", "unknown-evidence", SCOPE),
    ] {
        view.justification_edges.insert(
            edge_id.into(),
            TypedJustificationEdge {
                edge_kind: edge_kind.into(),
                source_id: source_id.into(),
                target_id: CLAIM_ID.into(),
                scope_ref: scope_ref.into(),
                actor_class: "HumanRoot".into(),
                rationale: "Rejected edge.".into(),
                metadata_json: "{}".into(),
            },
        );
    }

    let resolution = view.resolved_standing_with_trace(CLAIM_ID);

    assert_eq!(resolution.trace.len(), 4);
    assert!(resolution
        .trace
        .iter()
        .all(|entry| entry.edge_id.is_some() && !entry.reasons.is_empty()));
    assert_eq!(
        resolution
            .trace
            .iter()
            .map(|entry| entry.reasons[0])
            .collect::<Vec<_>>(),
        vec![
            StandingTraceReason::UnsupportedEdgeKindForV0,
            StandingTraceReason::MissingSourceEvidence,
            StandingTraceReason::ScopeMismatch,
            StandingTraceReason::UnknownEvidenceKind,
        ]
    );
}

#[test]
fn runtime_candidate_lookup_matches_full_support_matrix() {
    for kind in EvidenceKind::ALL {
        for domain in ClaimDomain::ALL {
            let metadata = format!(r#"{{"claim_domain":"{}"}}"#, domain.as_str());
            let resolution =
                candidate_view(&metadata, kind.as_str()).resolved_standing_with_trace(CLAIM_ID);
            let entry = only_edge(&resolution);

            assert_eq!(
                entry.candidate_ceiling,
                support_ceiling(kind, domain),
                "{kind} x {domain}"
            );
            assert_eq!(resolution.governed_standing, Some(Status::Conjectured));

            let expected_context_reason = match support_context_requirement(kind, domain) {
                SupportContextRequirement::NoSupportContribution => {
                    Some(StandingTraceReason::NoSupportCeiling)
                }
                SupportContextRequirement::HumanAdmission => {
                    Some(StandingTraceReason::RequiresAdmission)
                }
                SupportContextRequirement::DeterministicVerifierContext
                | SupportContextRequirement::DeadboltVerifierContext => {
                    Some(StandingTraceReason::RequiresVerifierContext)
                }
                SupportContextRequirement::NoPrivilegedContext => None,
            };
            assert_eq!(
                entry.reasons.iter().copied().find(|reason| {
                    matches!(
                        reason,
                        StandingTraceReason::NoSupportCeiling
                            | StandingTraceReason::RequiresAdmission
                            | StandingTraceReason::RequiresVerifierContext
                    )
                }),
                expected_context_reason,
                "{kind} x {domain}"
            );
        }
    }
}
