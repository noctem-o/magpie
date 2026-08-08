use magpie_claims::policy::{
    refutation_ceiling, support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
};
use magpie_claims::{
    replay_origin_admission_context_v0, AdmittedContributionAuditCompletionV0,
    ArtifactProvenanceAnchorSelectorV0, OriginAdmissionAuditCompletionV0,
    OriginAdmissionReplayContextV0, ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0,
    ResolutionContentClosureV0, ResolutionForeignBundleObjectInputV0,
    SupportContributionAuditCompletionV0, SupportContributionAuditV0,
    ADMITTED_CONTRIBUTION_POLICY_ID_V0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    ORIGIN_ADMISSION_POLICY_ID_V0, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_ACQUISITION_SCHEMA_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0, SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0,
    SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0, SUPPORT_CONTRIBUTION_POLICY_ID_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [52; 32];
const TRUSTED_KIND: &str = "human-review";
const TRUSTED_REFERENCE: &str = "authority:origin-review-v0";

const ONE_SUPPORT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/support-contribution-audit-v0/one-support-contribution.json");
const COMPLETE_EMPTY_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/support-contribution-audit-v0/complete-empty.json");
const INCOMPLETE_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/support-contribution-audit-v0/admitted-contribution-incomplete.json"
);
const SAME_ORIGIN_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/support-contribution-audit-v0/same-origin-distinct-contributions.json"
);
const DISTINCT_ORIGIN_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/support-contribution-audit-v0/distinct-origin-distinct-contributions.json"
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
    audit: SupportContributionAuditV0,
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
                    Provenance::new("support-contribution-audit-test", "graph"),
                    payload,
                )
                .unwrap();
        }
        for selector in anchors {
            writer
                .append(
                    Provenance::new("support-contribution-audit-test", "same-replay-anchor"),
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
                    Provenance::new("support-contribution-audit-test", "standing-ignored-note"),
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

fn resolve_case(
    payloads: Vec<Payload>,
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
    artifacts: &[(String, Vec<u8>)],
    bundles: &[(ArtifactProvenanceAnchorSelectorV0, Vec<u8>)],
    ignored_note: Option<&str>,
) -> ResolvedCase {
    let context = replay_context(payloads, anchors, ignored_note);
    let closure = construct_closure(artifacts, bundles);
    let audit = context.resolve_support_contribution_audit_v0(&closure);
    ResolvedCase {
        context,
        closure,
        audit,
    }
}

fn one_support_fixture_case() -> ResolvedCase {
    let acquisition = acquisition_material("fixture-one-support", b"fixture one support artifact");
    let graph = GraphContribution::labelled("fixture-one-support", &acquisition.artifact_digest);
    let binding = binding_material(
        "fixture-one-support",
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
        Some("fixture one support history"),
    )
}

fn complete_empty_fixture_case() -> ResolvedCase {
    let digest = sha256_hex(b"fixture complete empty artifact");
    let graph = GraphContribution::labelled("fixture-complete-empty", &digest);
    resolve_case(
        valid_graph_payloads(&graph),
        &[],
        &[],
        &[],
        Some("fixture complete empty history"),
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

fn distinct_origin_fixture_case() -> ResolvedCase {
    let acquisition_a = acquisition_material("fixture-origin-a", b"fixture origin artifact a");
    let acquisition_b = acquisition_material("fixture-origin-b", b"fixture origin artifact b");
    let graph_a = GraphContribution::labelled("fixture-origin-a", &acquisition_a.artifact_digest);
    let graph_b = GraphContribution::labelled("fixture-origin-b", &acquisition_b.artifact_digest);
    let binding_a = binding_material(
        "fixture-origin-a",
        &graph_a,
        &acquisition_a.artifact_digest,
        &acquisition_a.selector,
        "origin-group-alpha",
    );
    let binding_b = binding_material(
        "fixture-origin-b",
        &graph_b,
        &acquisition_b.artifact_digest,
        &acquisition_b.selector,
        "origin-group-beta",
    );
    let mut payloads = valid_graph_payloads(&graph_b);
    payloads.extend(valid_graph_payloads(&graph_a));
    resolve_case(
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
        Some("fixture distinct origin history"),
    )
}

#[test]
fn one_support_contribution_exposes_the_exact_public_surface() {
    let case = one_support_fixture_case();
    let audit = &case.audit;
    assert_eq!(
        SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0,
        "magpie-support-contribution-audit-v0"
    );
    assert_eq!(
        SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0,
        "magpie-support-contribution-audit-json-v0"
    );
    assert_eq!(
        SUPPORT_CONTRIBUTION_POLICY_ID_V0,
        "magpie-support-contribution-v0"
    );
    assert_eq!(audit.schema(), SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0);
    assert_eq!(
        audit.canonicalization_profile(),
        SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(audit.policy_id(), SUPPORT_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        audit.admitted_contribution_policy_id(),
        ADMITTED_CONTRIBUTION_POLICY_ID_V0
    );
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
        &SupportContributionAuditCompletionV0::Complete
    );
    assert_eq!(audit.support_contributions().len(), 1);

    let support = &audit.support_contributions()[0];
    assert_eq!(support.policy_id(), SUPPORT_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        support.admitted_contribution_policy_id(),
        ADMITTED_CONTRIBUTION_POLICY_ID_V0
    );
    assert_eq!(
        support.origin_admission_policy_id(),
        ORIGIN_ADMISSION_POLICY_ID_V0
    );
    assert_eq!(support.origin_group(), "origin-group-alpha");
    assert_eq!(support.evidence_kind(), "ExternalSource");
    assert_eq!(support.claim_domain(), "ExternalReport");
    assert_eq!(support.support_ceiling(), Status::Supported);
    assert_eq!(support.supporting_origin_candidate_selectors().len(), 1);
    assert_eq!(support.contribution().artifact_algorithm(), "sha256");
    assert_eq!(support.contribution().artifact_digest().len(), 64);
}

#[test]
fn upstream_origin_constructor_is_complete_iff_the_unavailable_vector_is_empty() {
    // Every binding byte available: the Ticket 0048 constructor yields
    // `Complete` (and therefore never an empty-vector incomplete).
    let available = one_support_fixture_case();
    let origin_audit = available
        .context
        .resolve_origin_admission_audit_v0(&available.closure);
    assert_eq!(
        origin_audit.completion(),
        &OriginAdmissionAuditCompletionV0::Complete
    );

    // One unavailable binding: the constructor yields
    // `IncompleteCandidateUniverse` with a non-empty selector vector, which
    // the admitted audit inherits exactly.
    let incomplete = incomplete_fixture_case();
    let origin_audit = incomplete
        .context
        .resolve_origin_admission_audit_v0(&incomplete.closure);
    let origin_unavailable = match origin_audit.completion() {
        OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
            unavailable_binding_selectors,
        } => unavailable_binding_selectors,
        other => panic!("expected origin incompleteness, got {other:?}"),
    };
    assert!(!origin_unavailable.is_empty());
    let admitted_audit = incomplete
        .context
        .resolve_admitted_contribution_audit_v0(&incomplete.closure);
    match admitted_audit.completion() {
        AdmittedContributionAuditCompletionV0::OriginAdmissionIncomplete {
            unavailable_binding_selectors,
        } => {
            assert!(!unavailable_binding_selectors.is_empty());
            assert_eq!(unavailable_binding_selectors, origin_unavailable);
        }
        other => panic!("expected admitted incompleteness, got {other:?}"),
    }
}

#[test]
fn deterministic_closure_order_and_repeated_resolution_are_byte_identical() {
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

    let first = context.resolve_support_contribution_audit_v0(&forward);
    let repeated = context.resolve_support_contribution_audit_v0(&forward);
    let reordered = context.resolve_support_contribution_audit_v0(&reverse);
    assert_eq!(first, repeated);
    assert_eq!(first, reordered);
    assert_eq!(first.canonical_bytes(), repeated.canonical_bytes());
    assert_eq!(first.canonical_bytes(), reordered.canonical_bytes());
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
    let audit_a = context_a.resolve_support_contribution_audit_v0(&closure);
    let audit_b = context_b.resolve_support_contribution_audit_v0(&closure);
    assert_ne!(
        audit_a.verified_prefix_identity(),
        audit_b.verified_prefix_identity()
    );
    assert_ne!(audit_a.canonical_bytes(), audit_b.canonical_bytes());
}

#[test]
fn support_audit_execution_preserves_all_existing_surfaces() {
    let acquisition = acquisition_material("inertia", b"inertia artifact");
    let graph = GraphContribution::labelled("inertia", &acquisition.artifact_digest);
    let binding = binding_material(
        "inertia",
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
    let closure = construct_closure(
        &[(
            acquisition.artifact_digest.clone(),
            acquisition.artifact_bytes.clone(),
        )],
        &[
            (binding.selector.clone(), binding.bytes.clone()),
            (
                acquisition.selector.clone(),
                acquisition.bundle_bytes.clone(),
            ),
        ],
    );

    let snapshot = context.snapshot();
    let snapshot_before = snapshot.canonical_bytes();
    let standing_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace(&graph.claim_id);
    let v1_before = snapshot.resolved_standing_with_trace_v1(&graph.claim_id);
    let v2_before = snapshot.resolved_standing_with_trace_v2(&graph.claim_id);
    let deterministic_before = snapshot.resolve_deterministic_verifier_context_v0(
        &graph.claim_id,
        &graph.evidence_id,
        &graph.edge_id,
    );
    let origin_binding_before =
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &closure);
    let origin_audit_before = context.resolve_origin_admission_audit_v0(&closure);
    let admitted_before = context.resolve_admitted_contribution_audit_v0(&closure);
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
        let _ = context.resolve_support_contribution_audit_v0(&closure);
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
        deterministic_before,
        snapshot.resolve_deterministic_verifier_context_v0(
            &graph.claim_id,
            &graph.evidence_id,
            &graph.edge_id,
        )
    );
    assert_eq!(
        origin_binding_before,
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &closure)
    );
    assert_eq!(
        origin_audit_before.canonical_bytes(),
        context
            .resolve_origin_admission_audit_v0(&closure)
            .canonical_bytes()
    );
    assert_eq!(
        admitted_before.canonical_bytes(),
        context
            .resolve_admitted_contribution_audit_v0(&closure)
            .canonical_bytes()
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
fn serialized_output_is_compact_typed_audit_material_only() {
    let case = one_support_fixture_case();
    let bytes = case.audit.canonical_bytes();
    assert_eq!(bytes.first(), Some(&b'{'));
    assert_eq!(bytes.last(), Some(&b'}'));
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!bytes.ends_with(b"\n"));
    assert!(!bytes.ends_with(b"\r"));
    assert!(!bytes
        .windows(b"fixture one support artifact".len())
        .any(|window| window == b"fixture one support artifact"));
    assert_eq!(bytes, case.audit.clone().canonical_bytes());
}

fn assert_literal_fixture(
    fixture: &[u8],
    audit: SupportContributionAuditV0,
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
    assert_eq!(value["schema"], SUPPORT_CONTRIBUTION_AUDIT_SCHEMA_V0);
    assert_eq!(
        value["canonicalization_profile"],
        SUPPORT_CONTRIBUTION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(value["policy_id"], SUPPORT_CONTRIBUTION_POLICY_ID_V0);
    assert_eq!(
        value["admitted_contribution_policy_id"],
        ADMITTED_CONTRIBUTION_POLICY_ID_V0
    );
    assert_eq!(
        value["origin_admission_policy_id"],
        ORIGIN_ADMISSION_POLICY_ID_V0
    );
    value
}

fn fixture_origin_groups(value: &serde_json::Value) -> Vec<&str> {
    value["support_contributions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|contribution| contribution["origin_group"].as_str().unwrap())
        .collect()
}

fn fixture_support_edge_ids(value: &serde_json::Value) -> Vec<&str> {
    value["support_contributions"]
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

fn fixture_selector_run_ids(value: &serde_json::Value) -> Vec<&str> {
    value["support_contributions"]
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
    let one_support = assert_literal_fixture(
        ONE_SUPPORT_FIXTURE,
        one_support_fixture_case().audit,
        1_831,
        "8da9cfd4e15006098909e8221969a965a2d103451f23a791a64878ff16995485",
    );
    assert_eq!(one_support["completion"]["outcome"], "complete");
    assert_eq!(
        fixture_support_edge_ids(&one_support),
        ["edge-fixture-one-support"]
    );
    assert_eq!(fixture_origin_groups(&one_support), ["origin-group-alpha"]);
    assert_eq!(
        fixture_selector_run_ids(&one_support),
        ["run-origin-binding-fixture-one-support"]
    );

    let complete_empty = assert_literal_fixture(
        COMPLETE_EMPTY_FIXTURE,
        complete_empty_fixture_case().audit,
        732,
        "5be0f96921f5693dfdd40ac2ebb1e2b7f12e5945d2529b44932e6bc71c9e4658",
    );
    assert_eq!(complete_empty["completion"]["outcome"], "complete");
    assert!(fixture_support_edge_ids(&complete_empty).is_empty());

    let incomplete = assert_literal_fixture(
        INCOMPLETE_FIXTURE,
        incomplete_fixture_case().audit,
        1_083,
        "2286c05b2459cbb3bfe1977d7797a8a9150dd076468b454ea4fadeb73a807450",
    );
    assert_eq!(
        incomplete["completion"]["outcome"],
        "admitted_contribution_incomplete"
    );
    assert_eq!(
        incomplete["completion"]["details"]["unavailable_binding_selectors"][0]["run_id"],
        "run-origin-binding-fixture-incomplete-absent"
    );
    assert!(fixture_support_edge_ids(&incomplete).is_empty());

    let same_origin = assert_literal_fixture(
        SAME_ORIGIN_FIXTURE,
        same_origin_fixture_case().audit,
        2_946,
        "47fbb1f0c0e33b5e338d3d30ab90c8a52fbc91e89d7a0af2774491e2bb80782d",
    );
    assert_eq!(same_origin["completion"]["outcome"], "complete");
    assert_eq!(
        fixture_support_edge_ids(&same_origin),
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

    let distinct_origin = assert_literal_fixture(
        DISTINCT_ORIGIN_FIXTURE,
        distinct_origin_fixture_case().audit,
        2_889,
        "8b95d741ee7ac8ca099c094a03fe5811b4581017767a3213420f5a0ecc7a4089",
    );
    assert_eq!(distinct_origin["completion"]["outcome"], "complete");
    assert_eq!(
        fixture_origin_groups(&distinct_origin),
        ["origin-group-alpha", "origin-group-beta"]
    );
}
