use magpie_claims::policy::{
    refutation_ceiling, support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
};
use magpie_claims::{
    replay_origin_admission_context_v0, AdmittedContributionAuditCompletionV0,
    AdmittedContributionAuditV0, AdmittedContributionCandidateAuditV0,
    AdmittedContributionCandidateDispositionV0, AdmittedContributionCandidateReasonV0,
    AdmittedContributionV0, ArtifactProvenanceAnchorSelectorV0, OriginAdmissionAuditCompletionV0,
    OriginAdmissionDecisionV0, OriginAdmissionReplayContextV0, ResolutionArtifactObjectInputV0,
    ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, StandingTraceReason,
    ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0,
    ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0, ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0,
    ADMITTED_CONTRIBUTION_POLICY_ID_V0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    ORIGIN_ADMISSION_POLICY_ID_V0, ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0,
    ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_ACQUISITION_SCHEMA_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [50; 32];
const TRUSTED_KIND: &str = "human-review";
const TRUSTED_REFERENCE: &str = "authority:origin-review-v0";
const ADMITTED_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/admitted-contribution-audit-v0/admitted.json");
const DECISION_ABSENT_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/admitted-contribution-audit-v0/decision-absent.json");
const CONFLICT_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/admitted-contribution-audit-v0/conflict.json");
const ELIGIBLE_UNRESOLVED_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/admitted-contribution-audit-v0/eligible-unresolved.json");
const INCOMPLETE_ORIGIN_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/admitted-contribution-audit-v0/incomplete-origin.json");
const SAME_ORIGIN_DISTINCT_CONTRIBUTIONS_AUDIT_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/admitted-contribution-audit-v0/same-origin-distinct-contributions.json"
);

#[derive(Clone, Debug)]
struct GraphContribution {
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    scope: String,
    content_hash: String,
}

impl GraphContribution {
    fn new(
        claim_id: impl Into<String>,
        evidence_id: impl Into<String>,
        edge_id: impl Into<String>,
        scope: impl Into<String>,
        content_hash: impl Into<String>,
    ) -> Self {
        Self {
            claim_id: claim_id.into(),
            evidence_id: evidence_id.into(),
            edge_id: edge_id.into(),
            scope: scope.into(),
            content_hash: content_hash.into(),
        }
    }

    fn labelled(label: &str, digest: &str) -> Self {
        Self::new(
            format!("claim-{label}"),
            format!("evidence-{label}"),
            format!("edge-{label}"),
            format!("scope:{label}"),
            digest,
        )
    }
}

#[derive(Clone, Debug)]
struct AcquisitionMaterial {
    selector: ArtifactProvenanceAnchorSelectorV0,
    bundle_bytes: Vec<u8>,
    artifact_digest: String,
    artifact_bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct BindingMaterial {
    selector: ArtifactProvenanceAnchorSelectorV0,
    bytes: Vec<u8>,
}

struct ResolvedCase {
    context: OriginAdmissionReplayContextV0,
    closure: ResolutionContentClosureV0,
    audit: AdmittedContributionAuditV0,
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn typed_claim_payload(graph: &GraphContribution, metadata_json: &str, scope: &str) -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: graph.claim_id.clone(),
        statement: format!("Exact external report for {}.", graph.claim_id),
        scope_ref: scope.to_owned(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: metadata_json.into(),
    }
}

fn legacy_claim_payload(graph: &GraphContribution) -> Payload {
    Payload::ClaimAsserted {
        claim_id: graph.claim_id.clone(),
        statement: format!("Legacy shell for {}.", graph.claim_id),
        status: Status::Conjectured,
    }
}

fn evidence_payload(
    graph: &GraphContribution,
    evidence_kind: &str,
    scope: &str,
    content_hash: &str,
) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: graph.evidence_id.clone(),
        evidence_kind: evidence_kind.into(),
        summary: format!("Exact evidence for {}.", graph.evidence_id),
        scope_ref: scope.into(),
        actor_class: "SourceImporter".into(),
        content_hash: content_hash.into(),
        metadata_json: "{}".into(),
    }
}

fn edge_payload(graph: &GraphContribution, edge_kind: &str, scope: &str) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: graph.edge_id.clone(),
        edge_kind: edge_kind.into(),
        source_id: graph.evidence_id.clone(),
        target_id: graph.claim_id.clone(),
        scope_ref: scope.into(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate relationship only.".into(),
        metadata_json: "{}".into(),
    }
}

fn valid_graph_payloads(graph: &GraphContribution) -> Vec<Payload> {
    vec![
        typed_claim_payload(graph, r#"{"claim_domain":"ExternalReport"}"#, &graph.scope),
        evidence_payload(graph, "ExternalSource", &graph.scope, &graph.content_hash),
        edge_payload(graph, "supports", &graph.scope),
    ]
}

fn acquisition_material(label: &str, artifact_bytes: &[u8]) -> AcquisitionMaterial {
    let artifact_digest = sha256_hex(artifact_bytes);
    let acquisition_id = format!("acq-{label}");
    let run_id = format!("run-acq-{label}");
    let bundle_bytes = format!(
        concat!(
            "{{\"schema\":\"magpie-artifact-acquisition-v0\",",
            "\"acquisition_id\":\"{acquisition_id}\",",
            "\"acquisition_profile\":\"magpie-import-v0\",",
            "\"artifact\":{{\"algorithm\":\"sha256\",",
            "\"digest\":\"{artifact_digest}\"}},",
            "\"observed_locator\":\"file:{label}.txt\",",
            "\"run_id\":\"{run_id}\"}}"
        ),
        acquisition_id = acquisition_id,
        artifact_digest = artifact_digest,
        label = label,
        run_id = run_id,
    )
    .into_bytes();
    let selector = ArtifactProvenanceAnchorSelectorV0::new(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        sha256_hex(&bundle_bytes),
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        run_id,
    );
    AcquisitionMaterial {
        selector,
        bundle_bytes,
        artifact_digest,
        artifact_bytes: artifact_bytes.to_vec(),
    }
}

fn binding_material(
    label: &str,
    graph: &GraphContribution,
    artifact_digest: &str,
    acquisition_selector: &ArtifactProvenanceAnchorSelectorV0,
    origin_group: &str,
) -> BindingMaterial {
    let binding_id = format!("origin-binding-{label}");
    let run_id = format!("run-origin-binding-{label}");
    let bytes = format!(
        concat!(
            "{{\"schema\":\"{schema}\",",
            "\"binding_id\":\"{binding_id}\",",
            "\"origin_admission_policy_id\":\"{policy_id}\",",
            "\"contribution\":{{",
            "\"target_claim_id\":\"{claim_id}\",",
            "\"source_evidence_id\":\"{evidence_id}\",",
            "\"justification_edge_id\":\"{edge_id}\",",
            "\"scope_ref\":\"{scope}\",",
            "\"artifact\":{{\"algorithm\":\"sha256\",",
            "\"digest\":\"{artifact_digest}\"}}}},",
            "\"acquisition_selector\":{{",
            "\"bundle_kind\":\"{acquisition_kind}\",",
            "\"witness_root\":\"{acquisition_root}\",",
            "\"witness_algorithm\":\"{acquisition_algorithm}\",",
            "\"canonicalization_profile\":\"{acquisition_profile}\",",
            "\"run_id\":\"{acquisition_run_id}\"}},",
            "\"origin_group\":\"{origin_group}\",",
            "\"authority\":{{\"kind\":\"{authority_kind}\",",
            "\"reference\":\"{authority_reference}\"}},",
            "\"run_id\":\"{run_id}\"}}"
        ),
        schema = ORIGIN_BINDING_ACQUISITION_SCHEMA_V0,
        binding_id = binding_id,
        policy_id = ORIGIN_ADMISSION_POLICY_ID_V0,
        claim_id = graph.claim_id,
        evidence_id = graph.evidence_id,
        edge_id = graph.edge_id,
        scope = graph.scope,
        artifact_digest = artifact_digest,
        acquisition_kind = acquisition_selector.bundle_kind(),
        acquisition_root = acquisition_selector.witness_root(),
        acquisition_algorithm = acquisition_selector.witness_algorithm(),
        acquisition_profile = acquisition_selector.canonicalization_profile(),
        acquisition_run_id = acquisition_selector.run_id(),
        origin_group = origin_group,
        authority_kind = TRUSTED_KIND,
        authority_reference = TRUSTED_REFERENCE,
        run_id = run_id,
    )
    .into_bytes();
    let selector = ArtifactProvenanceAnchorSelectorV0::new(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        sha256_hex(&bytes),
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        run_id,
    );
    BindingMaterial { selector, bytes }
}

fn replay_context(
    payloads: Vec<Payload>,
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
    ignored_note: Option<&str>,
) -> OriginAdmissionReplayContextV0 {
    let store = MemStore::new();
    {
        let mut timestamp = 0_u64;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            SigningKey::from_bytes(&SEED),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for payload in payloads {
            writer
                .append(
                    Provenance::new("admitted-contribution-audit-test", "graph"),
                    payload,
                )
                .unwrap();
        }
        for selector in anchors {
            writer
                .append(
                    Provenance::new("admitted-contribution-audit-test", "same-replay-anchor"),
                    Payload::SegmentAnchored {
                        bundle_kind: selector.bundle_kind().to_owned(),
                        witness_root: selector.witness_root().to_owned(),
                        witness_algorithm: selector.witness_algorithm().to_owned(),
                        canonicalization_profile: selector.canonicalization_profile().to_owned(),
                        run_id: selector.run_id().to_owned(),
                    },
                )
                .unwrap();
        }
        if let Some(text) = ignored_note {
            writer
                .append(
                    Provenance::new("admitted-contribution-audit-test", "standing-ignored-note"),
                    Payload::Note {
                        text: text.to_owned(),
                    },
                )
                .unwrap();
        }
    }
    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    replay_origin_admission_context_v0(&reader).unwrap()
}

fn construct_closure(
    artifacts: &[(String, Vec<u8>)],
    bundles: &[(ArtifactProvenanceAnchorSelectorV0, Vec<u8>)],
) -> ResolutionContentClosureV0 {
    let artifact_inputs = artifacts
        .iter()
        .map(|(digest, bytes)| {
            ResolutionArtifactObjectInputV0::new(
                ResolutionArtifactObjectKeyV0::new("sha256", digest),
                bytes,
            )
        })
        .collect::<Vec<_>>();
    let bundle_inputs = bundles
        .iter()
        .map(|(selector, bytes)| ResolutionForeignBundleObjectInputV0::new(selector.clone(), bytes))
        .collect::<Vec<_>>();
    ResolutionContentClosureV0::construct(&artifact_inputs, &bundle_inputs).unwrap()
}

fn empty_closure() -> ResolutionContentClosureV0 {
    ResolutionContentClosureV0::construct(&[], &[]).unwrap()
}

fn resolve_case(
    payloads: Vec<Payload>,
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
    artifacts: &[(String, Vec<u8>)],
    bundles: &[(ArtifactProvenanceAnchorSelectorV0, Vec<u8>)],
    ignored_note: Option<&str>,
) -> ResolvedCase {
    let context = replay_context(payloads, anchors, ignored_note);
    let closure = construct_closure(artifacts, bundles);
    let audit = context.resolve_admitted_contribution_audit_v0(&closure);
    ResolvedCase {
        context,
        closure,
        audit,
    }
}

fn candidate_for_edge<'a>(
    audit: &'a AdmittedContributionAuditV0,
    edge_id: &str,
) -> &'a AdmittedContributionCandidateAuditV0 {
    audit
        .candidate_audits()
        .iter()
        .find(|candidate| candidate.edge_id() == edge_id)
        .expect("candidate edge must be present")
}

fn admitted_from_candidate(
    candidate: &AdmittedContributionCandidateAuditV0,
) -> &AdmittedContributionV0 {
    match candidate.disposition() {
        AdmittedContributionCandidateDispositionV0::Admitted {
            admitted_contribution,
        } => admitted_contribution,
        _ => panic!("expected admitted candidate"),
    }
}

fn assert_policy_reason(
    candidate: &AdmittedContributionCandidateAuditV0,
    expected: AdmittedContributionCandidateReasonV0,
) {
    match candidate.disposition() {
        AdmittedContributionCandidateDispositionV0::PolicyIneligible { reason } => {
            assert_eq!(reason, &expected)
        }
        other => panic!("expected policy-ineligible candidate, got {other:?}"),
    }
}

fn admitted_fixture_case() -> ResolvedCase {
    let acquisition = acquisition_material("fixture-admitted", b"fixture admitted artifact");
    let graph = GraphContribution::labelled("fixture-admitted", &acquisition.artifact_digest);
    let binding = binding_material(
        "fixture-admitted",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    resolve_case(
        valid_graph_payloads(&graph),
        &[binding.selector.clone(), acquisition.selector.clone()],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding.selector, binding.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
        Some("fixture admitted history"),
    )
}

fn decision_absent_fixture_case() -> ResolvedCase {
    let digest = sha256_hex(b"fixture decision absent artifact");
    let graph = GraphContribution::labelled("fixture-decision-absent", &digest);
    resolve_case(
        valid_graph_payloads(&graph),
        &[],
        &[],
        &[],
        Some("fixture decision absent history"),
    )
}

fn conflict_fixture_case() -> ResolvedCase {
    let acquisition = acquisition_material("fixture-conflict", b"fixture conflict artifact");
    let graph = GraphContribution::labelled("fixture-conflict", &acquisition.artifact_digest);
    let alpha = binding_material(
        "fixture-conflict-alpha",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let zulu = binding_material(
        "fixture-conflict-zulu",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-zulu",
    );
    resolve_case(
        valid_graph_payloads(&graph),
        &[
            zulu.selector.clone(),
            acquisition.selector.clone(),
            alpha.selector.clone(),
        ],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (zulu.selector, zulu.bytes),
            (alpha.selector, alpha.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
        Some("fixture conflict history"),
    )
}

fn unresolved_fixture_case() -> ResolvedCase {
    let unavailable = acquisition_material("fixture-unresolved", b"fixture unresolved artifact");
    let graph = GraphContribution::labelled("fixture-unresolved", &unavailable.artifact_digest);
    let binding = binding_material(
        "fixture-unresolved",
        &graph,
        &unavailable.artifact_digest,
        &unavailable.selector,
        "origin-group-unresolved",
    );
    resolve_case(
        valid_graph_payloads(&graph),
        std::slice::from_ref(&binding.selector),
        &[],
        &[(binding.selector.clone(), binding.bytes)],
        Some("fixture unresolved history"),
    )
}

fn incomplete_fixture_case() -> ResolvedCase {
    let acquisition = acquisition_material("fixture-incomplete", b"fixture incomplete artifact");
    let graph = GraphContribution::labelled("fixture-incomplete", &acquisition.artifact_digest);
    let present = binding_material(
        "fixture-incomplete-present",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let absent = binding_material(
        "fixture-incomplete-absent",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-beta",
    );
    resolve_case(
        valid_graph_payloads(&graph),
        &[
            present.selector.clone(),
            acquisition.selector.clone(),
            absent.selector,
        ],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (present.selector, present.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
        Some("fixture incomplete history"),
    )
}

fn same_origin_fixture_case() -> ResolvedCase {
    let acquisition =
        acquisition_material("fixture-same-origin", b"fixture shared origin artifact");
    let graph_a = GraphContribution::new(
        "claim-fixture-same-origin",
        "evidence-fixture-same-origin-a",
        "edge-fixture-same-origin-a",
        "scope:fixture-same-origin",
        &acquisition.artifact_digest,
    );
    let graph_b = GraphContribution::new(
        "claim-fixture-same-origin",
        "evidence-fixture-same-origin-b",
        "edge-fixture-same-origin-b",
        "scope:fixture-same-origin",
        &acquisition.artifact_digest,
    );
    let binding_a = binding_material(
        "fixture-same-origin-a",
        &graph_a,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-shared",
    );
    let binding_b = binding_material(
        "fixture-same-origin-b",
        &graph_b,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-shared",
    );
    let mut payloads = valid_graph_payloads(&graph_b);
    payloads.extend(valid_graph_payloads(&graph_a));
    resolve_case(
        payloads,
        &[
            binding_b.selector.clone(),
            acquisition.selector.clone(),
            binding_a.selector.clone(),
        ],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding_b.selector, binding_b.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
            (binding_a.selector, binding_a.bytes),
        ],
        Some("fixture same origin history"),
    )
}

#[test]
fn complete_trusted_admission_exposes_the_exact_public_surface() {
    let case = admitted_fixture_case();
    let audit = &case.audit;
    assert_eq!(
        ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0,
        "magpie-admitted-contribution-audit-v0"
    );
    assert_eq!(
        ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0,
        "magpie-admitted-contribution-audit-json-v0"
    );
    assert_eq!(
        ADMITTED_CONTRIBUTION_POLICY_ID_V0,
        "magpie-admitted-contribution-v0"
    );
    assert_eq!(ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0, "sha256");
    assert_eq!(ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0, TRUSTED_KIND);
    assert_eq!(
        ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0,
        TRUSTED_REFERENCE
    );
    assert_eq!(audit.schema(), ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0);
    assert_eq!(
        audit.canonicalization_profile(),
        ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(audit.policy_id(), ADMITTED_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        audit.origin_admission_policy_id(),
        ORIGIN_ADMISSION_POLICY_ID_V0
    );
    assert_eq!(
        audit.verified_prefix_identity(),
        case.context.verified_prefix_identity()
    );
    assert_eq!(audit.closure_identity(), case.closure.identity());
    assert_eq!(
        audit.completion(),
        &AdmittedContributionAuditCompletionV0::Complete
    );
    assert_eq!(audit.candidate_audits().len(), 1);
    assert_eq!(audit.admitted_contributions().len(), 1);

    let candidate = &audit.candidate_audits()[0];
    assert_eq!(candidate.evidence_kind(), Some("ExternalSource"));
    assert_eq!(candidate.claim_domain(), Some("ExternalReport"));
    assert_eq!(candidate.candidate_ceiling(), Some(Status::Supported));
    let admitted = admitted_from_candidate(candidate);
    assert_eq!(admitted.policy_id(), ADMITTED_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        admitted.origin_admission_policy_id(),
        ORIGIN_ADMISSION_POLICY_ID_V0
    );
    assert_eq!(admitted.origin_group(), "origin-group-alpha");
    assert_eq!(admitted.evidence_kind(), "ExternalSource");
    assert_eq!(admitted.claim_domain(), "ExternalReport");
    assert_eq!(admitted.candidate_ceiling(), Status::Supported);
    assert_eq!(admitted.supporting_origin_candidate_selectors().len(), 1);
    assert_eq!(admitted, &audit.admitted_contributions()[0]);
    assert_eq!(
        candidate.candidate_contribution(),
        Some(admitted.contribution())
    );
    assert_eq!(candidate.candidate_namespace(), Some(admitted.namespace()));
    assert_eq!(
        admitted.contribution().artifact_algorithm(),
        ADMITTED_CONTRIBUTION_ARTIFACT_ALGORITHM_V0
    );
}

#[test]
fn complete_candidate_universe_is_every_edge_in_exact_id_order() {
    let digest = sha256_hex(b"complete candidate universe");
    let valid = GraphContribution::labelled("universe-valid", &digest);
    let missing = GraphContribution::labelled("universe-missing-evidence", &digest);
    let malformed = GraphContribution::labelled("universe-malformed", &digest);
    let unsupported = GraphContribution::labelled("universe-unsupported", &digest);

    let mut payloads = vec![
        typed_claim_payload(&valid, r#"{"claim_domain":"ExternalReport"}"#, &valid.scope),
        evidence_payload(&valid, "ExternalSource", &valid.scope, &valid.content_hash),
        edge_payload(&valid, "supports", &valid.scope),
        typed_claim_payload(
            &missing,
            r#"{"claim_domain":"ExternalReport"}"#,
            &missing.scope,
        ),
        edge_payload(&missing, "supports", &missing.scope),
        typed_claim_payload(&malformed, "{", &malformed.scope),
        edge_payload(&malformed, "supports", &malformed.scope),
        typed_claim_payload(
            &unsupported,
            r#"{"claim_domain":"ExternalReport"}"#,
            &unsupported.scope,
        ),
        edge_payload(&unsupported, "derived_from", &unsupported.scope),
    ];
    payloads.reverse();
    let context = replay_context(payloads, &[], None);
    let closure = empty_closure();
    let audit = context.resolve_admitted_contribution_audit_v0(&closure);

    let expected = [
        malformed.edge_id.as_str(),
        missing.edge_id.as_str(),
        unsupported.edge_id.as_str(),
        valid.edge_id.as_str(),
    ];
    assert_eq!(
        audit
            .candidate_audits()
            .iter()
            .map(AdmittedContributionCandidateAuditV0::edge_id)
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(audit.candidate_audits().len(), 4);
    assert_policy_reason(
        candidate_for_edge(&audit, &unsupported.edge_id),
        AdmittedContributionCandidateReasonV0::UnsupportedEdgeKindForV0,
    );
    assert_policy_reason(
        candidate_for_edge(&audit, &malformed.edge_id),
        AdmittedContributionCandidateReasonV0::MalformedClaimMetadata,
    );
    assert_policy_reason(
        candidate_for_edge(&audit, &missing.edge_id),
        AdmittedContributionCandidateReasonV0::MissingSourceEvidence,
    );
    assert!(matches!(
        candidate_for_edge(&audit, &valid.edge_id).disposition(),
        AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent
    ));
}

#[test]
fn first_failure_precedence_and_optional_field_staging_are_exact() {
    let digest = sha256_hex(b"first failure staging");

    let missing_target = GraphContribution::labelled("stage-missing-target", &digest);
    let audit = replay_context(
        vec![
            evidence_payload(
                &missing_target,
                "ExternalSource",
                &missing_target.scope,
                &missing_target.content_hash,
            ),
            edge_payload(&missing_target, "supports", &missing_target.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &missing_target.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::MissingTargetClaim,
    );
    assert_eq!(candidate.evidence_kind(), None);
    assert_eq!(candidate.claim_domain(), None);
    assert_eq!(candidate.candidate_ceiling(), None);

    let missing_typed = GraphContribution::labelled("stage-missing-typed", &digest);
    let audit = replay_context(
        vec![
            legacy_claim_payload(&missing_typed),
            evidence_payload(
                &missing_typed,
                "ExternalSource",
                &missing_typed.scope,
                &missing_typed.content_hash,
            ),
            edge_payload(&missing_typed, "supports", &missing_typed.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    assert_policy_reason(
        candidate_for_edge(&audit, &missing_typed.edge_id),
        AdmittedContributionCandidateReasonV0::MissingTypedTargetClaim,
    );

    let unsupported_edge = GraphContribution::labelled("stage-unsupported-edge", &digest);
    let audit = replay_context(
        vec![
            typed_claim_payload(&unsupported_edge, "{", &unsupported_edge.scope),
            edge_payload(&unsupported_edge, "contradicts", &unsupported_edge.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &unsupported_edge.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::UnsupportedEdgeKindForV0,
    );
    assert_eq!(candidate.evidence_kind(), None);
    assert_eq!(candidate.claim_domain(), None);
    assert_eq!(candidate.candidate_ceiling(), None);

    for (label, metadata, expected) in [
        (
            "malformed",
            "{",
            AdmittedContributionCandidateReasonV0::MalformedClaimMetadata,
        ),
        (
            "missing-domain",
            "{}",
            AdmittedContributionCandidateReasonV0::MissingClaimDomain,
        ),
        (
            "wrong-type",
            r#"{"claim_domain":7}"#,
            AdmittedContributionCandidateReasonV0::WrongTypeClaimDomain,
        ),
        (
            "duplicate-domain",
            r#"{"claim_domain":"ExternalReport","claim_domain":"ExternalReport"}"#,
            AdmittedContributionCandidateReasonV0::DuplicateClaimDomainKey,
        ),
        (
            "unknown-domain",
            r#"{"claim_domain":"HumanOverride"}"#,
            AdmittedContributionCandidateReasonV0::UnknownClaimDomain,
        ),
    ] {
        let graph = GraphContribution::labelled(&format!("stage-{label}"), &digest);
        let audit = replay_context(
            vec![
                typed_claim_payload(&graph, metadata, &graph.scope),
                edge_payload(&graph, "supports", &graph.scope),
            ],
            &[],
            None,
        )
        .resolve_admitted_contribution_audit_v0(&empty_closure());
        let candidate = candidate_for_edge(&audit, &graph.edge_id);
        assert_policy_reason(candidate, expected);
        assert_eq!(candidate.evidence_kind(), None);
        assert_eq!(candidate.claim_domain(), None);
        assert_eq!(candidate.candidate_ceiling(), None);
    }

    let missing_evidence = GraphContribution::labelled("stage-missing-evidence", &digest);
    let audit = replay_context(
        vec![
            typed_claim_payload(
                &missing_evidence,
                r#"{"claim_domain":"ExternalReport"}"#,
                &missing_evidence.scope,
            ),
            edge_payload(&missing_evidence, "supports", &missing_evidence.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &missing_evidence.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::MissingSourceEvidence,
    );
    assert_eq!(candidate.claim_domain(), Some("ExternalReport"));
    assert_eq!(candidate.evidence_kind(), None);
    assert_eq!(candidate.candidate_ceiling(), None);

    let scope_mismatch = GraphContribution::labelled("stage-scope", &digest);
    let audit = replay_context(
        vec![
            typed_claim_payload(
                &scope_mismatch,
                r#"{"claim_domain":"ExternalReport"}"#,
                &scope_mismatch.scope,
            ),
            evidence_payload(
                &scope_mismatch,
                "ExternalSource",
                "scope:other",
                &scope_mismatch.content_hash,
            ),
            edge_payload(&scope_mismatch, "supports", &scope_mismatch.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &scope_mismatch.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::ScopeMismatch,
    );
    assert_eq!(candidate.evidence_kind(), Some("ExternalSource"));
    assert_eq!(candidate.claim_domain(), Some("ExternalReport"));
    assert_eq!(candidate.candidate_ceiling(), None);

    let unsupported_kind = GraphContribution::labelled("stage-unsupported-kind", &digest);
    let audit = replay_context(
        vec![
            typed_claim_payload(
                &unsupported_kind,
                r#"{"claim_domain":"Interpretation"}"#,
                &unsupported_kind.scope,
            ),
            evidence_payload(
                &unsupported_kind,
                "ExecutionEvidence",
                &unsupported_kind.scope,
                &unsupported_kind.content_hash,
            ),
            edge_payload(&unsupported_kind, "supports", &unsupported_kind.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &unsupported_kind.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::UnsupportedEvidenceKindForV0,
    );
    assert_eq!(candidate.evidence_kind(), Some("ExecutionEvidence"));
    assert_eq!(candidate.claim_domain(), Some("Interpretation"));
    assert_eq!(candidate.candidate_ceiling(), None);

    let unsupported_domain = GraphContribution::labelled("stage-unsupported-domain", &digest);
    let audit = replay_context(
        vec![
            typed_claim_payload(
                &unsupported_domain,
                r#"{"claim_domain":"Interpretation"}"#,
                &unsupported_domain.scope,
            ),
            evidence_payload(
                &unsupported_domain,
                "ExternalSource",
                &unsupported_domain.scope,
                &unsupported_domain.content_hash,
            ),
            edge_payload(&unsupported_domain, "supports", &unsupported_domain.scope),
        ],
        &[],
        None,
    )
    .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &unsupported_domain.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::UnsupportedClaimDomainForV0,
    );
    assert_eq!(candidate.evidence_kind(), Some("ExternalSource"));
    assert_eq!(candidate.claim_domain(), Some("Interpretation"));
    assert_eq!(candidate.candidate_ceiling(), None);

    let empty_hash = GraphContribution::labelled("stage-empty-hash", "");
    let audit = replay_context(valid_graph_payloads(&empty_hash), &[], None)
        .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &empty_hash.edge_id);
    assert_policy_reason(
        candidate,
        AdmittedContributionCandidateReasonV0::MissingEvidenceContentHash,
    );
    assert_eq!(candidate.evidence_kind(), Some("ExternalSource"));
    assert_eq!(candidate.claim_domain(), Some("ExternalReport"));
    assert_eq!(candidate.candidate_ceiling(), Some(Status::Supported));
    assert!(candidate.candidate_contribution().is_none());
    assert!(candidate.candidate_namespace().is_none());

    let origin_stage = GraphContribution::labelled("stage-origin", &digest);
    let audit = replay_context(valid_graph_payloads(&origin_stage), &[], None)
        .resolve_admitted_contribution_audit_v0(&empty_closure());
    let candidate = candidate_for_edge(&audit, &origin_stage.edge_id);
    assert!(matches!(
        candidate.disposition(),
        AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent
    ));
    assert_eq!(candidate.candidate_ceiling(), Some(Status::Supported));
    assert!(candidate.candidate_contribution().is_some());
    assert!(candidate.candidate_namespace().is_some());
}

#[test]
fn graph_content_hash_must_match_the_full_origin_contribution_key() {
    let acquisition = acquisition_material("hash-exactness", b"verified origin artifact");
    let graph = GraphContribution::labelled("hash-exactness", &"0".repeat(64));
    let binding = binding_material(
        "hash-exactness",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let case = resolve_case(
        valid_graph_payloads(&graph),
        &[binding.selector.clone(), acquisition.selector.clone()],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding.selector, binding.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
        None,
    );
    let origin_audit = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure);
    assert_eq!(origin_audit.contribution_decisions().len(), 1);
    let origin_digest = match &origin_audit.contribution_decisions()[0] {
        OriginAdmissionDecisionV0::OriginAdmitted { admitted_origin } => {
            admitted_origin.contribution().artifact_digest()
        }
        _ => panic!("origin path should admit the verified artifact"),
    };
    assert_eq!(origin_digest, acquisition.artifact_digest);

    let graph_candidate = &case.audit.candidate_audits()[0];
    assert_eq!(
        graph_candidate
            .candidate_contribution()
            .unwrap()
            .artifact_digest(),
        "0".repeat(64)
    );
    assert!(matches!(
        graph_candidate.disposition(),
        AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent
    ));
    assert!(case.audit.admitted_contributions().is_empty());
}

#[test]
fn global_incomplete_origin_suppresses_every_otherwise_admitted_contribution() {
    let case = incomplete_fixture_case();
    let unavailable = match case.audit.completion() {
        AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
            unavailable_binding_selectors,
        } => unavailable_binding_selectors,
        _ => panic!("expected global origin incompleteness"),
    };
    assert_eq!(unavailable.len(), 1);
    assert_eq!(case.audit.candidate_audits().len(), 1);
    assert!(matches!(
        case.audit.candidate_audits()[0].disposition(),
        AdmittedContributionCandidateDispositionV0::OriginAdmissionIncomplete
    ));
    assert!(case.audit.admitted_contributions().is_empty());

    let origin_audit = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure);
    let origin_unavailable = match origin_audit.completion() {
        OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
            unavailable_binding_selectors,
        } => unavailable_binding_selectors,
        _ => panic!("expected the internally composed audit to be incomplete"),
    };
    assert_eq!(unavailable, origin_unavailable);
    assert!(origin_audit.contribution_decisions().is_empty());
}

#[test]
fn earlier_policy_failures_remain_exact_when_origin_is_globally_incomplete() {
    let incomplete = acquisition_material("incomplete-with-failure", b"available artifact");
    let valid = GraphContribution::labelled("incomplete-valid", &incomplete.artifact_digest);
    let invalid = GraphContribution::labelled("incomplete-invalid", "");
    let present = binding_material(
        "incomplete-with-failure-present",
        &valid,
        &incomplete.artifact_digest,
        &incomplete.selector,
        "origin-group-alpha",
    );
    let absent = binding_material(
        "incomplete-with-failure-absent",
        &valid,
        &incomplete.artifact_digest,
        &incomplete.selector,
        "origin-group-beta",
    );
    let mut payloads = valid_graph_payloads(&valid);
    payloads.extend(valid_graph_payloads(&invalid));
    let case = resolve_case(
        payloads,
        &[
            present.selector.clone(),
            absent.selector,
            incomplete.selector.clone(),
        ],
        &[(
            incomplete.artifact_digest.clone(),
            incomplete.artifact_bytes.clone(),
        )],
        &[
            (present.selector, present.bytes),
            (incomplete.selector, incomplete.bundle_bytes),
        ],
        None,
    );
    assert!(matches!(
        candidate_for_edge(&case.audit, &valid.edge_id).disposition(),
        AdmittedContributionCandidateDispositionV0::OriginAdmissionIncomplete
    ));
    assert_policy_reason(
        candidate_for_edge(&case.audit, &invalid.edge_id),
        AdmittedContributionCandidateReasonV0::MissingEvidenceContentHash,
    );
    assert!(case.audit.admitted_contributions().is_empty());
}

#[test]
fn conflict_and_eligible_unresolved_decisions_are_preserved_exactly() {
    let conflict_case = conflict_fixture_case();
    let conflict_candidate = &conflict_case.audit.candidate_audits()[0];
    let nested_conflict = match conflict_candidate.disposition() {
        AdmittedContributionCandidateDispositionV0::OriginNotAdmitted { decision } => decision,
        _ => panic!("expected exact conflict preservation"),
    };
    match nested_conflict {
        OriginAdmissionDecisionV0::ConflictingBindings { conflict } => {
            assert_eq!(
                conflict.origin_groups(),
                ["origin-group-alpha", "origin-group-zulu"]
            );
        }
        _ => panic!("expected conflicting bindings"),
    }
    assert!(conflict_case.audit.admitted_contributions().is_empty());

    let unresolved_case = unresolved_fixture_case();
    let unresolved_candidate = &unresolved_case.audit.candidate_audits()[0];
    let nested_unresolved = match unresolved_candidate.disposition() {
        AdmittedContributionCandidateDispositionV0::OriginNotAdmitted { decision } => decision,
        _ => panic!("expected exact unresolved preservation"),
    };
    match nested_unresolved {
        OriginAdmissionDecisionV0::EligibleBindingUnresolved {
            unresolved_candidate_selectors,
            ..
        } => assert_eq!(unresolved_candidate_selectors.len(), 1),
        _ => panic!("expected eligible binding unresolved"),
    }
    assert!(unresolved_case.audit.admitted_contributions().is_empty());
}

#[test]
fn two_distinct_edges_over_one_artifact_and_group_remain_two_contributions() {
    let case = same_origin_fixture_case();
    assert_eq!(case.audit.candidate_audits().len(), 2);
    assert_eq!(case.audit.admitted_contributions().len(), 2);
    assert!(case
        .audit
        .candidate_audits()
        .iter()
        .all(|candidate| matches!(
            candidate.disposition(),
            AdmittedContributionCandidateDispositionV0::Admitted { .. }
        )));
    assert!(case
        .audit
        .admitted_contributions()
        .iter()
        .all(|contribution| contribution.origin_group() == "origin-group-shared"));
    assert_ne!(
        case.audit.admitted_contributions()[0].contribution(),
        case.audit.admitted_contributions()[1].contribution()
    );
    assert_eq!(
        case.audit.admitted_contributions()[0]
            .contribution()
            .artifact_digest(),
        case.audit.admitted_contributions()[1]
            .contribution()
            .artifact_digest()
    );
}

#[test]
fn two_distinct_artifacts_in_one_group_remain_two_contributions() {
    let acquisition_a = acquisition_material("distinct-artifact-a", b"artifact alpha");
    let acquisition_b = acquisition_material("distinct-artifact-b", b"artifact beta");
    let graph_a = GraphContribution::new(
        "claim-distinct-artifacts",
        "evidence-distinct-artifact-a",
        "edge-distinct-artifact-a",
        "scope:distinct-artifacts",
        &acquisition_a.artifact_digest,
    );
    let graph_b = GraphContribution::new(
        "claim-distinct-artifacts",
        "evidence-distinct-artifact-b",
        "edge-distinct-artifact-b",
        "scope:distinct-artifacts",
        &acquisition_b.artifact_digest,
    );
    let binding_a = binding_material(
        "distinct-artifact-a",
        &graph_a,
        &acquisition_a.artifact_digest,
        &acquisition_a.selector,
        "origin-group-shared",
    );
    let binding_b = binding_material(
        "distinct-artifact-b",
        &graph_b,
        &acquisition_b.artifact_digest,
        &acquisition_b.selector,
        "origin-group-shared",
    );
    let mut payloads = valid_graph_payloads(&graph_b);
    payloads.extend(valid_graph_payloads(&graph_a));
    let case = resolve_case(
        payloads,
        &[
            binding_b.selector.clone(),
            acquisition_a.selector.clone(),
            binding_a.selector.clone(),
            acquisition_b.selector.clone(),
        ],
        &[
            (
                acquisition_b.artifact_digest.clone(),
                acquisition_b.artifact_bytes.clone(),
            ),
            (
                acquisition_a.artifact_digest.clone(),
                acquisition_a.artifact_bytes.clone(),
            ),
        ],
        &[
            (binding_b.selector, binding_b.bytes),
            (acquisition_a.selector, acquisition_a.bundle_bytes),
            (binding_a.selector, binding_a.bytes),
            (acquisition_b.selector, acquisition_b.bundle_bytes),
        ],
        None,
    );
    assert_eq!(case.audit.admitted_contributions().len(), 2);
    assert!(case
        .audit
        .admitted_contributions()
        .iter()
        .all(|contribution| contribution.origin_group() == "origin-group-shared"));
    assert_ne!(
        case.audit.admitted_contributions()[0]
            .contribution()
            .artifact_digest(),
        case.audit.admitted_contributions()[1]
            .contribution()
            .artifact_digest()
    );
}

#[test]
fn identical_group_text_in_different_namespaces_has_no_relationship() {
    let acquisition_a = acquisition_material("namespace-a", b"namespace artifact a");
    let acquisition_b = acquisition_material("namespace-b", b"namespace artifact b");
    let graph_a = GraphContribution::labelled("namespace-a", &acquisition_a.artifact_digest);
    let graph_b = GraphContribution::labelled("namespace-b", &acquisition_b.artifact_digest);
    let binding_a = binding_material(
        "namespace-a",
        &graph_a,
        &acquisition_a.artifact_digest,
        &acquisition_a.selector,
        "origin-group-same-text",
    );
    let binding_b = binding_material(
        "namespace-b",
        &graph_b,
        &acquisition_b.artifact_digest,
        &acquisition_b.selector,
        "origin-group-same-text",
    );
    let mut payloads = valid_graph_payloads(&graph_a);
    payloads.extend(valid_graph_payloads(&graph_b));
    let case = resolve_case(
        payloads,
        &[
            binding_a.selector.clone(),
            binding_b.selector.clone(),
            acquisition_a.selector.clone(),
            acquisition_b.selector.clone(),
        ],
        &[
            (
                acquisition_a.artifact_digest.clone(),
                acquisition_a.artifact_bytes.clone(),
            ),
            (
                acquisition_b.artifact_digest.clone(),
                acquisition_b.artifact_bytes.clone(),
            ),
        ],
        &[
            (binding_a.selector, binding_a.bytes),
            (binding_b.selector, binding_b.bytes),
            (acquisition_a.selector, acquisition_a.bundle_bytes),
            (acquisition_b.selector, acquisition_b.bundle_bytes),
        ],
        None,
    );
    assert_eq!(case.audit.admitted_contributions().len(), 2);
    assert_eq!(
        case.audit.admitted_contributions()[0].origin_group(),
        case.audit.admitted_contributions()[1].origin_group()
    );
    assert_ne!(
        case.audit.admitted_contributions()[0].namespace(),
        case.audit.admitted_contributions()[1].namespace()
    );
}

#[test]
fn deterministic_input_closure_order_and_identity_laws_are_exact() {
    let acquisition = acquisition_material("deterministic", b"deterministic artifact");
    let graph = GraphContribution::labelled("deterministic", &acquisition.artifact_digest);
    let binding = binding_material(
        "deterministic",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let context = replay_context(
        valid_graph_payloads(&graph),
        &[binding.selector.clone(), acquisition.selector.clone()],
        None,
    );
    let artifacts = [(
        acquisition.artifact_digest.clone(),
        acquisition.artifact_bytes.clone(),
    )];
    let forward = construct_closure(
        &artifacts,
        &[
            (binding.selector.clone(), binding.bytes.clone()),
            (
                acquisition.selector.clone(),
                acquisition.bundle_bytes.clone(),
            ),
        ],
    );
    let reverse = construct_closure(
        &artifacts,
        &[
            (
                acquisition.selector.clone(),
                acquisition.bundle_bytes.clone(),
            ),
            (binding.selector.clone(), binding.bytes.clone()),
        ],
    );
    assert_eq!(forward.identity(), reverse.identity());
    let origin_forward = context.resolve_origin_admission_audit_v0(&forward);
    let origin_reverse = context.resolve_origin_admission_audit_v0(&reverse);
    assert_eq!(origin_forward, origin_reverse);
    assert_eq!(
        origin_forward.canonical_bytes(),
        origin_reverse.canonical_bytes()
    );

    let first = context.resolve_admitted_contribution_audit_v0(&forward);
    let repeated = context.resolve_admitted_contribution_audit_v0(&forward);
    let reordered = context.resolve_admitted_contribution_audit_v0(&reverse);
    assert_eq!(first, repeated);
    assert_eq!(first, reordered);
    assert_eq!(first.canonical_bytes(), repeated.canonical_bytes());
    assert_eq!(first.canonical_bytes(), reordered.canonical_bytes());

    let unrelated = ArtifactProvenanceAnchorSelectorV0::new(
        "unrelated-bundle-v0",
        "f".repeat(64),
        "sha256",
        "unrelated-json-v0",
        "run-unrelated",
    );
    let different = construct_closure(
        &artifacts,
        &[
            (binding.selector, binding.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
            (unrelated, b"different closure input".to_vec()),
        ],
    );
    let different_audit = context.resolve_admitted_contribution_audit_v0(&different);
    assert_ne!(forward.identity(), different.identity());
    assert_ne!(first.closure_identity(), different_audit.closure_identity());
    assert_ne!(first.canonical_bytes(), different_audit.canonical_bytes());
}

#[test]
fn different_ignored_signed_history_changes_prefix_identity_and_audit_bytes() {
    let acquisition = acquisition_material("history", b"history artifact");
    let graph = GraphContribution::labelled("history", &acquisition.artifact_digest);
    let binding = binding_material(
        "history",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let anchors = [binding.selector.clone(), acquisition.selector.clone()];
    let context_a = replay_context(
        valid_graph_payloads(&graph),
        &anchors,
        Some("ignored history A"),
    );
    let context_b = replay_context(
        valid_graph_payloads(&graph),
        &anchors,
        Some("ignored history B"),
    );
    assert_eq!(context_a.snapshot(), context_b.snapshot());
    assert_eq!(
        context_a.snapshot().canonical_bytes(),
        context_b.snapshot().canonical_bytes()
    );
    assert_ne!(
        context_a.verified_prefix_identity(),
        context_b.verified_prefix_identity()
    );
    let closure = construct_closure(
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding.selector, binding.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
    );
    let audit_a = context_a.resolve_admitted_contribution_audit_v0(&closure);
    let audit_b = context_b.resolve_admitted_contribution_audit_v0(&closure);
    assert_ne!(
        audit_a.verified_prefix_identity(),
        audit_b.verified_prefix_identity()
    );
    assert_ne!(audit_a.canonical_bytes(), audit_b.canonical_bytes());
}

#[test]
fn audit_execution_preserves_standing_origin_audit_and_policy_surfaces() {
    let acquisition = acquisition_material("inertia", b"inertia artifact");
    let graph = GraphContribution::labelled("inertia", &acquisition.artifact_digest);
    let binding = binding_material(
        "inertia",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-alpha",
    );
    let case = resolve_case(
        valid_graph_payloads(&graph),
        &[binding.selector.clone(), acquisition.selector.clone()],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding.selector.clone(), binding.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
        ],
        None,
    );
    let snapshot = case.context.snapshot();
    let snapshot_before = snapshot.canonical_bytes();
    let standing_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace(&graph.claim_id);
    let v1_before = snapshot.resolved_standing_with_trace_v1(&graph.claim_id);
    let v2_before = snapshot.resolved_standing_with_trace_v2(&graph.claim_id);
    let v0_bytes_before = v0_before.canonical_bytes();
    let v1_bytes_before = v1_before.canonical_bytes();
    let v2_bytes_before = v2_before.canonical_bytes();
    let deterministic_before = snapshot.resolve_deterministic_verifier_context_v0(
        &graph.claim_id,
        &graph.evidence_id,
        &graph.edge_id,
    );
    let origin_binding_before =
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &case.closure);
    let origin_audit_before = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure);
    let origin_audit_bytes_before = origin_audit_before.canonical_bytes();
    let policy_before = EvidenceKind::ALL
        .into_iter()
        .flat_map(|kind| {
            ClaimDomain::ALL.into_iter().map(move |domain| {
                (
                    kind,
                    domain,
                    support_ceiling(kind, domain),
                    refutation_ceiling(kind, domain),
                    support_context_requirement(kind, domain),
                )
            })
        })
        .collect::<Vec<_>>();

    for _ in 0..3 {
        assert_eq!(
            case.context
                .resolve_admitted_contribution_audit_v0(&case.closure),
            case.audit
        );
    }

    assert_eq!(snapshot_before, snapshot.canonical_bytes());
    assert_eq!(standing_before, snapshot.standing().canonical_bytes());
    assert_eq!(
        v0_before,
        snapshot
            .standing()
            .resolved_standing_with_trace(&graph.claim_id)
    );
    assert_eq!(
        v1_before,
        snapshot.resolved_standing_with_trace_v1(&graph.claim_id)
    );
    assert_eq!(
        v2_before,
        snapshot.resolved_standing_with_trace_v2(&graph.claim_id)
    );
    assert_eq!(
        v0_bytes_before,
        snapshot
            .standing()
            .resolved_standing_with_trace(&graph.claim_id)
            .canonical_bytes()
    );
    assert_eq!(
        v1_bytes_before,
        snapshot
            .resolved_standing_with_trace_v1(&graph.claim_id)
            .canonical_bytes()
    );
    assert_eq!(
        v2_bytes_before,
        snapshot
            .resolved_standing_with_trace_v2(&graph.claim_id)
            .canonical_bytes()
    );
    assert_eq!(
        deterministic_before,
        snapshot.resolve_deterministic_verifier_context_v0(
            &graph.claim_id,
            &graph.evidence_id,
            &graph.edge_id,
        )
    );
    let origin_binding_after =
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &case.closure);
    assert_eq!(origin_binding_before, origin_binding_after);
    assert_eq!(
        origin_binding_before.canonical_bytes(),
        origin_binding_after.canonical_bytes()
    );
    let origin_audit_after = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure);
    assert_eq!(origin_audit_before, origin_audit_after);
    assert_eq!(
        origin_audit_bytes_before,
        origin_audit_after.canonical_bytes()
    );
    let policy_after = EvidenceKind::ALL
        .into_iter()
        .flat_map(|kind| {
            ClaimDomain::ALL.into_iter().map(move |domain| {
                (
                    kind,
                    domain,
                    support_ceiling(kind, domain),
                    refutation_ceiling(kind, domain),
                    support_context_requirement(kind, domain),
                )
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(policy_before, policy_after);
    assert_eq!(v0_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v1_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v2_before.governed_standing, Some(Status::Conjectured));
}

#[test]
fn shared_evaluator_preserves_representative_standing_v0_bytes() {
    let digest = sha256_hex(b"standing parity");
    let graph = GraphContribution::labelled("standing-parity", &digest);

    let accepted = replay_context(valid_graph_payloads(&graph), &[], None);
    assert_eq!(
        accepted
            .snapshot()
            .standing()
            .resolved_standing_with_trace(&graph.claim_id)
            .canonical_bytes(),
        format!(
            concat!(
                "{{\"claim_id\":\"{claim}\",",
                "\"governed_standing\":\"Conjectured\",",
                "\"legacy_raw_standing\":\"Conjectured\",",
                "\"currentness\":\"unknown\",",
                "\"policy_id\":\"magpie-claims-standing-v0\",",
                "\"trace\":[{{\"edge_id\":\"{edge}\",",
                "\"source_id\":\"{evidence}\",",
                "\"target_id\":\"{claim}\",",
                "\"evidence_kind\":\"ExternalSource\",",
                "\"claim_domain\":\"ExternalReport\",",
                "\"candidate_ceiling\":\"Supported\",",
                "\"reasons\":[\"accepted_candidate\",",
                "\"ceiling_is_candidate_only\"]}}],",
                "\"blockers\":[\"ceiling_is_candidate_only\"]}}"
            ),
            claim = graph.claim_id,
            edge = graph.edge_id,
            evidence = graph.evidence_id,
        )
        .into_bytes()
    );

    let malformed = GraphContribution::labelled("standing-malformed", &digest);
    let context = replay_context(
        vec![
            typed_claim_payload(&malformed, "{", &malformed.scope),
            edge_payload(&malformed, "supports", &malformed.scope),
        ],
        &[],
        None,
    );
    let resolution = context
        .snapshot()
        .standing()
        .resolved_standing_with_trace(&malformed.claim_id);
    assert_eq!(
        resolution.trace[0].reasons,
        [StandingTraceReason::MalformedMetadata]
    );
    assert_eq!(resolution.trace[0].claim_domain, None);
    assert_eq!(resolution.trace[0].evidence_kind, None);
    assert_eq!(resolution.trace[0].candidate_ceiling, None);

    let unsupported = GraphContribution::labelled("standing-unsupported", &digest);
    let context = replay_context(
        vec![
            typed_claim_payload(
                &unsupported,
                r#"{"claim_domain":"ExternalReport"}"#,
                &unsupported.scope,
            ),
            edge_payload(&unsupported, "derived_from", &unsupported.scope),
        ],
        &[],
        None,
    );
    let resolution = context
        .snapshot()
        .standing()
        .resolved_standing_with_trace(&unsupported.claim_id);
    assert_eq!(
        resolution.trace[0].reasons,
        [StandingTraceReason::UnsupportedEdgeKindForV0]
    );
    assert_eq!(
        resolution.trace[0].claim_domain.as_deref(),
        Some("ExternalReport")
    );
    assert_eq!(resolution.trace[0].evidence_kind, None);
}

#[test]
fn serialized_output_is_compact_typed_audit_material_only() {
    let case = admitted_fixture_case();
    let bytes = case.audit.canonical_bytes();
    assert_eq!(bytes.first(), Some(&b'{'));
    assert_eq!(bytes.last(), Some(&b'}'));
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!bytes.ends_with(b"\n"));
    assert!(!bytes.ends_with(b"\r"));
    assert!(!bytes
        .windows(b"fixture admitted artifact".len())
        .any(|window| window == b"fixture admitted artifact"));
    assert_eq!(bytes, case.audit.clone().canonical_bytes());
}

fn assert_literal_fixture(
    fixture: &[u8],
    audit: AdmittedContributionAuditV0,
    expected_len: usize,
    expected_sha256: &str,
) -> serde_json::Value {
    assert_eq!(audit.canonical_bytes(), fixture);
    assert_eq!(fixture.len(), expected_len);
    assert_eq!(sha256_hex(fixture), expected_sha256);
    assert_eq!(fixture.first(), Some(&b'{'));
    assert_eq!(fixture.last(), Some(&b'}'));
    assert!(!fixture.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!fixture.ends_with(b"\n"));
    assert!(!fixture.ends_with(b"\r"));

    let value: serde_json::Value = serde_json::from_slice(fixture).unwrap();
    assert_eq!(value["schema"], ADMITTED_CONTRIBUTION_AUDIT_SCHEMA_V0);
    assert_eq!(
        value["canonicalization_profile"],
        ADMITTED_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(value["policy_id"], ADMITTED_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        value["origin_admission_policy_id"],
        ORIGIN_ADMISSION_POLICY_ID_V0
    );
    value
}

fn fixture_edge_ids(value: &serde_json::Value) -> Vec<&str> {
    value["candidate_audits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["edge_id"].as_str().unwrap())
        .collect()
}

fn fixture_dispositions(value: &serde_json::Value) -> Vec<&str> {
    value["candidate_audits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["disposition"]["outcome"].as_str().unwrap())
        .collect()
}

fn fixture_contribution_edge_ids(value: &serde_json::Value) -> Vec<&str> {
    value["admitted_contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|contribution| {
            contribution["contribution"]["justification_edge_id"]
                .as_str()
                .unwrap()
        })
        .collect()
}

fn fixture_origin_groups(value: &serde_json::Value) -> Vec<&str> {
    value["admitted_contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|contribution| contribution["origin_group"].as_str().unwrap())
        .collect()
}

fn fixture_selector_run_ids(value: &serde_json::Value) -> Vec<&str> {
    value["admitted_contributions"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|contribution| {
            contribution["supporting_origin_candidate_selectors"]
                .as_array()
                .unwrap()
                .iter()
                .map(|selector| selector["run_id"].as_str().unwrap())
        })
        .collect()
}

#[test]
fn normative_fixtures_match_production_and_independent_expected_structure() {
    let admitted = assert_literal_fixture(
        ADMITTED_AUDIT_FIXTURE,
        admitted_fixture_case().audit,
        3495,
        "886d0159fc275c8505c1d4ec634fcec7a5975f4443881e577348f890e8d00f93",
    );
    assert_eq!(admitted["completion"]["outcome"], "complete");
    assert_eq!(fixture_edge_ids(&admitted), ["edge-fixture-admitted"]);
    assert_eq!(fixture_dispositions(&admitted), ["admitted"]);
    assert_eq!(
        fixture_contribution_edge_ids(&admitted),
        ["edge-fixture-admitted"]
    );
    assert_eq!(fixture_origin_groups(&admitted), ["origin-group-alpha"]);
    assert_eq!(
        fixture_selector_run_ids(&admitted),
        ["run-origin-binding-fixture-admitted"]
    );

    let decision_absent = assert_literal_fixture(
        DECISION_ABSENT_AUDIT_FIXTURE,
        decision_absent_fixture_case().audit,
        1509,
        "0613b47aecf02f7d2e40e2e3016b39eefb26aa74d148ef88870b6bb311b9d13b",
    );
    assert_eq!(decision_absent["completion"]["outcome"], "complete");
    assert_eq!(
        fixture_edge_ids(&decision_absent),
        ["edge-fixture-decision-absent"]
    );
    assert_eq!(
        fixture_dispositions(&decision_absent),
        ["origin_decision_absent"]
    );
    assert!(fixture_contribution_edge_ids(&decision_absent).is_empty());

    let conflict = assert_literal_fixture(
        CONFLICT_AUDIT_FIXTURE,
        conflict_fixture_case().audit,
        2710,
        "c35d9c4b0c59c7140f56c994637a444205e9ae0220159939d1a45cb3c8bdf118",
    );
    assert_eq!(conflict["completion"]["outcome"], "complete");
    assert_eq!(fixture_edge_ids(&conflict), ["edge-fixture-conflict"]);
    assert_eq!(fixture_dispositions(&conflict), ["origin_not_admitted"]);
    assert_eq!(
        conflict["candidate_audits"][0]["disposition"]["details"]["decision"]["outcome"],
        "conflicting_bindings"
    );
    assert_eq!(
        conflict["candidate_audits"][0]["disposition"]["details"]["decision"]["details"]
            ["conflict"]["origin_groups"],
        serde_json::json!(["origin-group-alpha", "origin-group-zulu"])
    );
    assert_eq!(
        conflict["candidate_audits"][0]["disposition"]["details"]["decision"]["details"]
            ["conflict"]["matched_candidate_selectors"][0]["run_id"],
        "run-origin-binding-fixture-conflict-alpha"
    );
    assert_eq!(
        conflict["candidate_audits"][0]["disposition"]["details"]["decision"]["details"]
            ["conflict"]["matched_candidate_selectors"][1]["run_id"],
        "run-origin-binding-fixture-conflict-zulu"
    );
    assert!(fixture_contribution_edge_ids(&conflict).is_empty());

    let unresolved = assert_literal_fixture(
        ELIGIBLE_UNRESOLVED_AUDIT_FIXTURE,
        unresolved_fixture_case().audit,
        2394,
        "99a2d62d443dff44970dd51572e1a70e107581376fb5381a404a453a52af074b",
    );
    assert_eq!(unresolved["completion"]["outcome"], "complete");
    assert_eq!(fixture_edge_ids(&unresolved), ["edge-fixture-unresolved"]);
    assert_eq!(fixture_dispositions(&unresolved), ["origin_not_admitted"]);
    assert_eq!(
        unresolved["candidate_audits"][0]["disposition"]["details"]["decision"]["outcome"],
        "eligible_binding_unresolved"
    );
    assert_eq!(
        unresolved["candidate_audits"][0]["disposition"]["details"]["decision"]["details"]
            ["unresolved_candidate_selectors"][0]["run_id"],
        "run-origin-binding-fixture-unresolved"
    );
    assert!(fixture_contribution_edge_ids(&unresolved).is_empty());

    let incomplete = assert_literal_fixture(
        INCOMPLETE_ORIGIN_AUDIT_FIXTURE,
        incomplete_fixture_case().audit,
        1815,
        "bb115589fd97ffc88ce3175188216cd3213a7b862fad8031a8c9942cbe7d81e8",
    );
    assert_eq!(
        incomplete["completion"]["outcome"],
        "origin_admission_incomplete"
    );
    assert_eq!(
        incomplete["completion"]["details"]["unavailable_binding_selectors"][0]["run_id"],
        "run-origin-binding-fixture-incomplete-absent"
    );
    assert_eq!(fixture_edge_ids(&incomplete), ["edge-fixture-incomplete"]);
    assert_eq!(
        fixture_dispositions(&incomplete),
        ["origin_admission_incomplete"]
    );
    assert!(fixture_contribution_edge_ids(&incomplete).is_empty());

    let same_origin = assert_literal_fixture(
        SAME_ORIGIN_DISTINCT_CONTRIBUTIONS_AUDIT_FIXTURE,
        same_origin_fixture_case().audit,
        6485,
        "929dc86024af6b37918ee3e9ea74a09aa3557cff1cd00c87a115f48102501584",
    );
    assert_eq!(same_origin["completion"]["outcome"], "complete");
    assert_eq!(
        fixture_edge_ids(&same_origin),
        ["edge-fixture-same-origin-a", "edge-fixture-same-origin-b",]
    );
    assert_eq!(fixture_dispositions(&same_origin), ["admitted", "admitted"]);
    assert_eq!(
        fixture_contribution_edge_ids(&same_origin),
        ["edge-fixture-same-origin-a", "edge-fixture-same-origin-b",]
    );
    assert_eq!(
        fixture_origin_groups(&same_origin),
        ["origin-group-shared", "origin-group-shared"]
    );
    assert_eq!(
        fixture_selector_run_ids(&same_origin),
        [
            "run-origin-binding-fixture-same-origin-a",
            "run-origin-binding-fixture-same-origin-b",
        ]
    );
}

#[test]
fn candidate_and_admitted_order_are_structural_not_event_order() {
    let case = same_origin_fixture_case();
    let candidate_ids = case
        .audit
        .candidate_audits()
        .iter()
        .map(AdmittedContributionCandidateAuditV0::edge_id)
        .collect::<Vec<_>>();
    assert_eq!(
        candidate_ids,
        ["edge-fixture-same-origin-a", "edge-fixture-same-origin-b",]
    );
    let contribution_ids = case
        .audit
        .admitted_contributions()
        .iter()
        .map(|contribution| {
            (
                contribution.contribution().target_claim_id(),
                contribution.contribution().source_evidence_id(),
                contribution.contribution().justification_edge_id(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        contribution_ids,
        [
            (
                "claim-fixture-same-origin",
                "evidence-fixture-same-origin-a",
                "edge-fixture-same-origin-a",
            ),
            (
                "claim-fixture-same-origin",
                "evidence-fixture-same-origin-b",
                "edge-fixture-same-origin-b",
            ),
        ]
    );
}

#[test]
fn origin_decision_absence_is_distinct_from_policy_ineligibility() {
    let case = decision_absent_fixture_case();
    assert_eq!(
        case.audit.completion(),
        &AdmittedContributionAuditCompletionV0::Complete
    );
    assert_eq!(case.audit.candidate_audits().len(), 1);
    assert!(matches!(
        case.audit.candidate_audits()[0].disposition(),
        AdmittedContributionCandidateDispositionV0::OriginDecisionAbsent
    ));
    assert!(case.audit.candidate_audits()[0]
        .candidate_contribution()
        .is_some());
    assert!(case.audit.admitted_contributions().is_empty());
}

#[test]
fn supporting_selectors_are_inherited_without_reordering() {
    let acquisition = acquisition_material("selector-order", b"selector order artifact");
    let graph = GraphContribution::labelled("selector-order", &acquisition.artifact_digest);
    let binding_a = binding_material(
        "selector-order-a",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-shared",
    );
    let binding_b = binding_material(
        "selector-order-b",
        &graph,
        &acquisition.artifact_digest,
        &acquisition.selector,
        "origin-group-shared",
    );
    let case = resolve_case(
        valid_graph_payloads(&graph),
        &[
            binding_b.selector.clone(),
            acquisition.selector.clone(),
            binding_a.selector.clone(),
        ],
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding_b.selector, binding_b.bytes),
            (acquisition.selector, acquisition.bundle_bytes),
            (binding_a.selector, binding_a.bytes),
        ],
        None,
    );
    let origin_audit = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure);
    let expected = match &origin_audit.contribution_decisions()[0] {
        OriginAdmissionDecisionV0::OriginAdmitted { admitted_origin } => {
            admitted_origin.supporting_candidate_selectors()
        }
        _ => panic!("duplicate exact group assignments should collapse upstream"),
    };
    assert_eq!(expected.len(), 2);
    assert_eq!(
        case.audit.admitted_contributions()[0].supporting_origin_candidate_selectors(),
        expected
    );
}
