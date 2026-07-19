use magpie_claims::policy::{
    refutation_ceiling, support_ceiling, support_context_requirement, ClaimDomain, EvidenceKind,
};
use magpie_claims::{
    replay_origin_admission_context_v0, ArtifactProvenanceAnchorSelectorV0,
    ExternalReportCorroborationLaneV0, OriginAdmissionReplayContextV0,
    ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, StandingBlockerV3, StandingPolicyContextV3,
    StandingPolicyRuleV3, StandingResolutionV3, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0, MAGPIE_CLAIMS_POLICY_V3_ID,
    ORIGIN_ADMISSION_POLICY_ID_V0, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_ACQUISITION_SCHEMA_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [74; 32];
const TRUSTED_KIND: &str = "human-review";
const TRUSTED_REFERENCE: &str = "authority:origin-review-v0";
const CLAIM_ID: &str = "claim-standing-v3";
const SCOPE: &str = "scope:standing-v3";
const CORROBORATED_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/standing-policy-v3-corroboration-v0/corroborated-two-groups.json"
);
const INSUFFICIENT_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/standing-policy-v3-corroboration-v0/insufficient-one-group.json"
);
const INCOMPLETE_FIXTURE: &[u8] = include_bytes!(
    "../../../fixtures/standing-policy-v3-corroboration-v0/support-audit-incomplete.json"
);

#[derive(Clone, Debug)]
struct GraphContribution {
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    scope: String,
    content_hash: String,
    claim_metadata: String,
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

#[derive(Clone, Debug)]
struct CompleteSpec<'a> {
    label: &'a str,
    claim_id: &'a str,
    scope: &'a str,
    artifact_bytes: &'a [u8],
    origin_group: &'a str,
    claim_metadata: &'a str,
}

impl<'a> CompleteSpec<'a> {
    fn external(label: &'a str, origin_group: &'a str) -> Self {
        Self {
            label,
            claim_id: CLAIM_ID,
            scope: SCOPE,
            artifact_bytes: label.as_bytes(),
            origin_group,
            claim_metadata: r#"{"claim_domain":"ExternalReport"}"#,
        }
    }
}

#[derive(Clone, Debug)]
struct Material {
    graph: GraphContribution,
    acquisition: AcquisitionMaterial,
    binding: BindingMaterial,
}

struct ResolvedCase {
    context: OriginAdmissionReplayContextV0,
    closure: ResolutionContentClosureV0,
    materials: Vec<Material>,
}

impl ResolvedCase {
    fn resolution(&self, claim_id: &str) -> StandingResolutionV3 {
        self.context
            .resolved_standing_with_trace_v3(claim_id, &self.closure)
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn typed_claim_payload(graph: &GraphContribution) -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: graph.claim_id.clone(),
        statement: format!("Exact external report for {}.", graph.claim_id),
        scope_ref: graph.scope.clone(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: graph.claim_metadata.clone(),
    }
}

fn evidence_payload(graph: &GraphContribution) -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: graph.evidence_id.clone(),
        evidence_kind: "ExternalSource".into(),
        summary: format!("Exact evidence for {}.", graph.evidence_id),
        scope_ref: graph.scope.clone(),
        actor_class: "SourceImporter".into(),
        content_hash: graph.content_hash.clone(),
        metadata_json: "{}".into(),
    }
}

fn edge_payload(graph: &GraphContribution) -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: graph.edge_id.clone(),
        edge_kind: "supports".into(),
        source_id: graph.evidence_id.clone(),
        target_id: graph.claim_id.clone(),
        scope_ref: graph.scope.clone(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate relationship only.".into(),
        metadata_json: "{}".into(),
    }
}

fn graph_payloads(graph: &GraphContribution) -> [Payload; 3] {
    [
        typed_claim_payload(graph),
        evidence_payload(graph),
        edge_payload(graph),
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
    acquisition: &AcquisitionMaterial,
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
        artifact_digest = acquisition.artifact_digest,
        acquisition_kind = acquisition.selector.bundle_kind(),
        acquisition_root = acquisition.selector.witness_root(),
        acquisition_algorithm = acquisition.selector.witness_algorithm(),
        acquisition_profile = acquisition.selector.canonicalization_profile(),
        acquisition_run_id = acquisition.selector.run_id(),
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

fn material(spec: &CompleteSpec<'_>) -> Material {
    let acquisition = acquisition_material(spec.label, spec.artifact_bytes);
    let graph = GraphContribution {
        claim_id: spec.claim_id.to_owned(),
        evidence_id: format!("evidence-{}", spec.label),
        edge_id: format!("edge-{}", spec.label),
        scope: spec.scope.to_owned(),
        content_hash: acquisition.artifact_digest.clone(),
        claim_metadata: spec.claim_metadata.to_owned(),
    };
    let binding = binding_material(spec.label, &graph, &acquisition, spec.origin_group);
    Material {
        graph,
        acquisition,
        binding,
    }
}

fn replay_context(
    payloads: Vec<Payload>,
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
    note: &str,
) -> OriginAdmissionReplayContextV0 {
    let store = MemStore::new();
    {
        let mut timestamp = 0_u64;
        let mut writer = LogWriter::open_with_clock(
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
                .append(Provenance::new("standing-v3-integration", "graph"), payload)
                .unwrap();
        }
        for selector in anchors {
            writer
                .append(
                    Provenance::new("standing-v3-integration", "same-replay-anchor"),
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
        writer
            .append(
                Provenance::new("standing-v3-integration", "fixture-note"),
                Payload::Note {
                    text: note.to_owned(),
                },
            )
            .unwrap();
    }
    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    replay_origin_admission_context_v0(&reader).unwrap()
}

fn construct_closure(materials: &[Material]) -> ResolutionContentClosureV0 {
    let artifacts = materials
        .iter()
        .map(|material| {
            ResolutionArtifactObjectInputV0::new(
                ResolutionArtifactObjectKeyV0::new("sha256", &material.acquisition.artifact_digest),
                &material.acquisition.artifact_bytes,
            )
        })
        .collect::<Vec<_>>();
    let mut bundles = Vec::new();
    for material in materials {
        bundles.push(ResolutionForeignBundleObjectInputV0::new(
            material.binding.selector.clone(),
            &material.binding.bytes,
        ));
        bundles.push(ResolutionForeignBundleObjectInputV0::new(
            material.acquisition.selector.clone(),
            &material.acquisition.bundle_bytes,
        ));
    }
    ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap()
}

fn complete_case(specs: &[CompleteSpec<'_>], note: &str) -> ResolvedCase {
    let materials = specs.iter().map(material).collect::<Vec<_>>();
    let payloads = materials
        .iter()
        .flat_map(|material| graph_payloads(&material.graph))
        .collect::<Vec<_>>();
    let anchors = materials
        .iter()
        .flat_map(|material| {
            [
                material.binding.selector.clone(),
                material.acquisition.selector.clone(),
            ]
        })
        .collect::<Vec<_>>();
    let context = replay_context(payloads, &anchors, note);
    let closure = construct_closure(&materials);
    ResolvedCase {
        context,
        closure,
        materials,
    }
}

fn corroborated_fixture_case() -> ResolvedCase {
    complete_case(
        &[
            CompleteSpec::external("fixture-alpha", "origin-group-alpha"),
            CompleteSpec::external("fixture-beta", "origin-group-beta"),
        ],
        "standing v3 corroborated fixture",
    )
}

fn insufficient_fixture_case() -> ResolvedCase {
    complete_case(
        &[
            CompleteSpec::external("fixture-one-a", "origin-group-one"),
            CompleteSpec::external("fixture-one-b", "origin-group-one"),
        ],
        "standing v3 insufficient fixture",
    )
}

fn incomplete_fixture_case() -> ResolvedCase {
    let spec = CompleteSpec::external("fixture-incomplete", "origin-group-present");
    let present = material(&spec);
    let absent_binding = binding_material(
        "fixture-incomplete-absent",
        &present.graph,
        &present.acquisition,
        "origin-group-absent",
    );
    let payloads = graph_payloads(&present.graph).to_vec();
    let anchors = vec![
        present.binding.selector.clone(),
        present.acquisition.selector.clone(),
        absent_binding.selector,
    ];
    let context = replay_context(payloads, &anchors, "standing v3 incomplete fixture");
    let materials = vec![present];
    let closure = construct_closure(&materials);
    ResolvedCase {
        context,
        closure,
        materials,
    }
}

fn group_names(lane: &ExternalReportCorroborationLaneV0) -> Vec<&str> {
    lane.groups()
        .iter()
        .map(|group| group.origin_group())
        .collect()
}

#[test]
fn exact_public_happy_path_supports_but_never_settles() {
    let case = corroborated_fixture_case();
    let resolution = case.resolution(CLAIM_ID);
    assert_eq!(MAGPIE_CLAIMS_POLICY_V3_ID, "magpie-claims-standing-v3");
    assert_eq!(
        EXTERNAL_REPORT_CORROBORATION_MIN_DISTINCT_ORIGIN_GROUPS_V0,
        2
    );
    assert_eq!(resolution.policy_id(), MAGPIE_CLAIMS_POLICY_V3_ID);
    assert_eq!(resolution.claim_id(), CLAIM_ID);
    assert_eq!(resolution.governed_standing(), Some(Status::Supported));
    assert_ne!(resolution.governed_standing(), Some(Status::Settled));
    assert_eq!(
        case.context.resolved_standing_v3(CLAIM_ID, &case.closure),
        resolution.governed_standing()
    );
    assert_eq!(
        resolution.application().unwrap().rule(),
        &StandingPolicyRuleV3::ExternalReportCorroborationV0
    );
    assert_eq!(
        resolution.application().unwrap().context(),
        &StandingPolicyContextV3::Corroborated {
            distinct_group_count: 2,
        }
    );
    assert_eq!(
        resolution.application().unwrap().achieved_standing(),
        Some(Status::Supported)
    );
    assert_eq!(
        group_names(resolution.lane().unwrap()),
        ["origin-group-alpha", "origin-group-beta"]
    );
    assert!(resolution.blockers().is_empty());
    assert!(resolution.resolution_failure().is_none());
}

#[test]
fn more_than_two_groups_are_all_retained_and_repeated_resolution_is_identical() {
    let case = complete_case(
        &[
            CompleteSpec::external("three-z", "origin-zeta"),
            CompleteSpec::external("three-a", "origin-alpha"),
            CompleteSpec::external("three-b", "origin-beta"),
        ],
        "standing v3 three groups",
    );
    let first = case.resolution(CLAIM_ID);
    let second = case.resolution(CLAIM_ID);
    assert_eq!(first, second);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(
        group_names(first.lane().unwrap()),
        ["origin-alpha", "origin-beta", "origin-zeta"]
    );
    assert_eq!(
        first.application().unwrap().context(),
        &StandingPolicyContextV3::Corroborated {
            distinct_group_count: 3,
        }
    );
}

#[test]
fn same_origin_multiplicity_is_visible_but_counts_once() {
    let case = insufficient_fixture_case();
    let resolution = case.resolution(CLAIM_ID);
    let lane = resolution.lane().unwrap();
    assert_eq!(lane.groups().len(), 1);
    assert_eq!(lane.groups()[0].contributions().len(), 2);
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    assert!(resolution.application().is_none());
    assert_eq!(
        resolution.blockers(),
        [StandingBlockerV3::InsufficientDistinctOriginGroups {
            distinct_group_count: 1,
        }]
    );
}

#[test]
fn many_same_origin_contributions_remain_visible_and_count_once() {
    let case = complete_case(
        &[
            CompleteSpec::external("many-a", "origin-many"),
            CompleteSpec::external("many-b", "origin-many"),
            CompleteSpec::external("many-c", "origin-many"),
            CompleteSpec::external("many-d", "origin-many"),
            CompleteSpec::external("many-e", "origin-many"),
        ],
        "standing v3 many contributions in one group",
    );
    let resolution = case.resolution(CLAIM_ID);
    let lane = resolution.lane().unwrap();
    assert_eq!(lane.groups().len(), 1);
    assert_eq!(lane.groups()[0].origin_group(), "origin-many");
    assert_eq!(lane.groups()[0].contributions().len(), 5);
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    assert!(resolution.application().is_none());
    assert_eq!(
        resolution.blockers(),
        [StandingBlockerV3::InsufficientDistinctOriginGroups {
            distinct_group_count: 1,
        }]
    );
}

#[test]
fn one_selected_contribution_is_one_group_and_preserves_inherited_standing() {
    let case = complete_case(
        &[CompleteSpec::external("single", "origin-single")],
        "standing v3 single contribution",
    );
    let resolution = case.resolution(CLAIM_ID);
    assert_eq!(resolution.lane().unwrap().groups().len(), 1);
    assert_eq!(
        resolution.lane().unwrap().groups()[0].contributions().len(),
        1
    );
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    assert!(resolution.application().is_none());
    assert_eq!(
        resolution.blockers(),
        [StandingBlockerV3::InsufficientDistinctOriginGroups {
            distinct_group_count: 1,
        }]
    );
}

#[test]
fn case_distinct_group_keys_and_shared_digest_still_count_exactly() {
    let shared = b"shared external artifact";
    let mut upper = CompleteSpec::external("case-upper", "Origin-Group");
    upper.artifact_bytes = shared;
    let mut lower = CompleteSpec::external("case-lower", "origin-group");
    lower.artifact_bytes = shared;
    let case = complete_case(&[upper, lower], "standing v3 exact group strings");
    let resolution = case.resolution(CLAIM_ID);
    assert_eq!(
        group_names(resolution.lane().unwrap()),
        ["Origin-Group", "origin-group"]
    );
    let digests = resolution
        .lane()
        .unwrap()
        .groups()
        .iter()
        .flat_map(|group| group.contributions())
        .map(|support| support.contribution().artifact_digest())
        .collect::<Vec<_>>();
    assert_eq!(digests.len(), 2);
    assert_eq!(digests[0], digests[1]);
    assert_eq!(resolution.governed_standing(), Some(Status::Supported));
}

#[test]
fn foreign_target_is_ordered_ignored_material_and_never_counts() {
    let mut foreign = CompleteSpec::external("foreign", "origin-foreign");
    foreign.claim_id = "claim-foreign";
    foreign.scope = "scope:foreign";
    let case = complete_case(
        &[
            foreign,
            CompleteSpec::external("main-beta", "origin-beta"),
            CompleteSpec::external("main-alpha", "origin-alpha"),
        ],
        "standing v3 ignored target",
    );
    let resolution = case.resolution(CLAIM_ID);
    assert_eq!(resolution.ignored_contributions().len(), 1);
    assert_eq!(
        resolution.ignored_contributions()[0]
            .contribution()
            .target_claim_id(),
        "claim-foreign"
    );
    assert_eq!(resolution.lane().unwrap().groups().len(), 2);
    assert_eq!(resolution.governed_standing(), Some(Status::Supported));

    let foreign_resolution = case.resolution("claim-foreign");
    assert_eq!(foreign_resolution.lane().unwrap().groups().len(), 1);
    assert_eq!(foreign_resolution.ignored_contributions().len(), 2);
    assert_eq!(
        foreign_resolution.governed_standing(),
        Some(Status::Conjectured)
    );
    assert_ne!(
        resolution.canonical_bytes(),
        foreign_resolution.canonical_bytes()
    );
}

#[test]
fn same_textual_group_in_different_claim_namespaces_never_combines() {
    let mut foreign = CompleteSpec::external("shared-foreign", "origin-shared");
    foreign.claim_id = "claim-foreign";
    foreign.scope = "scope:foreign";
    let case = complete_case(
        &[
            CompleteSpec::external("shared-requested", "origin-shared"),
            foreign,
        ],
        "standing v3 same group text across claim namespaces",
    );

    let requested = case.resolution(CLAIM_ID);
    let foreign = case.resolution("claim-foreign");
    for (resolution, claim_id) in [(&requested, CLAIM_ID), (&foreign, "claim-foreign")] {
        let lane = resolution.lane().unwrap();
        assert_eq!(lane.namespace().target_claim_id(), claim_id);
        assert_eq!(group_names(lane), ["origin-shared"]);
        assert_eq!(lane.groups()[0].contributions().len(), 1);
        assert_eq!(resolution.ignored_contributions().len(), 1);
        assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
        assert!(resolution.application().is_none());
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count: 1,
            }]
        );
    }
    assert_ne!(requested.canonical_bytes(), foreign.canonical_bytes());
}

#[test]
fn absent_malformed_and_wrong_domain_requests_never_promote() {
    let case = corroborated_fixture_case();
    let absent = case.resolution("missing-claim");
    assert_eq!(absent.governed_standing(), None);
    assert!(absent.application().is_none());
    assert!(absent.lane().is_none());

    let empty = case.resolution("");
    assert_eq!(empty.claim_id(), "");
    assert_eq!(empty.governed_standing(), None);
    assert!(empty.application().is_none());
    assert!(empty.lane().is_none());
    assert_eq!(empty.ignored_contributions().len(), 2);
    assert_eq!(
        empty.blockers(),
        [StandingBlockerV3::InsufficientDistinctOriginGroups {
            distinct_group_count: 0,
        }]
    );
    assert_eq!(case.context.resolved_standing_v3("", &case.closure), None);

    for (label, metadata) in [
        ("wrong-domain", r#"{"claim_domain":"Interpretation"}"#),
        ("malformed-domain", "{"),
        ("unknown-domain", r#"{"claim_domain":"Unknown"}"#),
    ] {
        let mut spec = CompleteSpec::external(label, "origin-one");
        spec.claim_id = label;
        spec.scope = "scope:invalid-domain";
        spec.claim_metadata = metadata;
        let case = complete_case(&[spec], "standing v3 invalid requested domain");
        let resolution = case.resolution(label);
        assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
        assert!(resolution.application().is_none());
        assert!(resolution.lane().is_none());
        assert_eq!(
            resolution.blockers(),
            [StandingBlockerV3::InsufficientDistinctOriginGroups {
                distinct_group_count: 0,
            }]
        );
    }
}

#[test]
fn incomplete_support_audit_has_no_evaluation_or_standing_effect() {
    let case = incomplete_fixture_case();
    let resolution = case.resolution(CLAIM_ID);
    assert_eq!(resolution.governed_standing(), Some(Status::Conjectured));
    assert!(resolution.lane().is_none());
    assert!(resolution.application().is_none());
    assert!(resolution.ignored_contributions().is_empty());
    assert_eq!(
        resolution.blockers(),
        [StandingBlockerV3::SupportAuditIncomplete]
    );
}

#[test]
fn v3_query_preserves_every_existing_authority_and_policy_surface() {
    let case = corroborated_fixture_case();
    let material = &case.materials[0];
    let snapshot = case.context.snapshot();
    let snapshot_before = snapshot.canonical_bytes();
    let standing_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    let v1_before = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    let v2_before = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    let deterministic_before = snapshot.resolve_deterministic_verifier_context_v0(
        CLAIM_ID,
        &material.graph.evidence_id,
        &material.graph.edge_id,
    );
    let origin_binding_before =
        snapshot.resolve_origin_binding_context_v0(&material.binding.selector, &case.closure);
    let origin_before = case
        .context
        .resolve_origin_admission_audit_v0(&case.closure)
        .canonical_bytes();
    let admitted_before = case
        .context
        .resolve_admitted_contribution_audit_v0(&case.closure)
        .canonical_bytes();
    let support_before = case
        .context
        .resolve_support_contribution_audit_v0(&case.closure)
        .canonical_bytes();
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
        let _ = case.resolution(CLAIM_ID);
    }

    assert_eq!(snapshot_before, snapshot.canonical_bytes());
    assert_eq!(standing_before, snapshot.standing().canonical_bytes());
    assert_eq!(
        v0_before,
        snapshot.standing().resolved_standing_with_trace(CLAIM_ID)
    );
    assert_eq!(
        v1_before,
        snapshot.resolved_standing_with_trace_v1(CLAIM_ID)
    );
    assert_eq!(
        v2_before,
        snapshot.resolved_standing_with_trace_v2(CLAIM_ID)
    );
    assert_eq!(
        deterministic_before,
        snapshot.resolve_deterministic_verifier_context_v0(
            CLAIM_ID,
            &material.graph.evidence_id,
            &material.graph.edge_id,
        )
    );
    assert_eq!(
        origin_binding_before,
        snapshot.resolve_origin_binding_context_v0(&material.binding.selector, &case.closure)
    );
    assert_eq!(
        origin_before,
        case.context
            .resolve_origin_admission_audit_v0(&case.closure)
            .canonical_bytes()
    );
    assert_eq!(
        admitted_before,
        case.context
            .resolve_admitted_contribution_audit_v0(&case.closure)
            .canonical_bytes()
    );
    assert_eq!(
        support_before,
        case.context
            .resolve_support_contribution_audit_v0(&case.closure)
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
}

#[test]
fn canonical_output_is_compact_typed_audit_material_only() {
    let resolution = corroborated_fixture_case().resolution(CLAIM_ID);
    let bytes = resolution.canonical_bytes();
    assert_eq!(bytes.first(), Some(&b'{'));
    assert_eq!(bytes.last(), Some(&b'}'));
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!bytes.ends_with(b"\n"));
    assert!(!bytes.ends_with(b"\r"));
    assert_eq!(bytes, resolution.clone().canonical_bytes());
}

#[test]
fn reachable_fixtures_match_the_public_resolver_and_generic_json_shape() {
    fn assert_fixture(
        fixture: &[u8],
        resolution: StandingResolutionV3,
        expected_len: usize,
        expected_sha256: &str,
    ) -> serde_json::Value {
        assert_eq!(resolution.canonical_bytes(), fixture);
        assert_eq!(fixture.len(), expected_len);
        assert_eq!(sha256_hex(fixture), expected_sha256);
        assert_eq!(fixture.first(), Some(&b'{'));
        assert_eq!(fixture.last(), Some(&b'}'));
        assert!(!fixture.starts_with(&[0xef, 0xbb, 0xbf]));
        assert!(!fixture.ends_with(b"\n"));
        assert!(!fixture.ends_with(b"\r"));
        let value: serde_json::Value = serde_json::from_slice(fixture).unwrap();
        assert_eq!(value["policy_id"], MAGPIE_CLAIMS_POLICY_V3_ID);
        value
    }

    let corroborated = assert_fixture(
        CORROBORATED_FIXTURE,
        corroborated_fixture_case().resolution(CLAIM_ID),
        6_777,
        "2a4eb437e8a39574e7abfc1d32d380f2b68487ca6b42513e5af054bb9e286132",
    );
    assert_eq!(corroborated["governed_standing"], "Supported");
    assert_eq!(corroborated["lane"]["groups"].as_array().unwrap().len(), 2);
    assert_eq!(
        corroborated["application"]["rule"],
        "external_report_corroboration_v0"
    );
    assert_eq!(
        corroborated["application"]["context"]["distinct_group_count"],
        2
    );

    let insufficient = assert_fixture(
        INSUFFICIENT_FIXTURE,
        insufficient_fixture_case().resolution(CLAIM_ID),
        6_665,
        "5bf0902c0c64718c12c798517675ce23777bca885e2ada7fa05adbb7dfadbe9a",
    );
    assert_eq!(insufficient["governed_standing"], "Conjectured");
    assert_eq!(insufficient["lane"]["groups"].as_array().unwrap().len(), 1);
    assert!(insufficient["application"].is_null());
    assert_eq!(
        insufficient["blockers"][0]["kind"],
        "insufficient_distinct_origin_groups"
    );

    let incomplete = assert_fixture(
        INCOMPLETE_FIXTURE,
        incomplete_fixture_case().resolution(CLAIM_ID),
        2_300,
        "12ae03f449a153f4665e8180126e5f0f7a559c968581559fb8c5cc654760e6e5",
    );
    assert_eq!(incomplete["governed_standing"], "Conjectured");
    assert!(incomplete["lane"].is_null());
    assert!(incomplete["application"].is_null());
    assert_eq!(
        incomplete["blockers"][0]["kind"],
        "support_audit_incomplete"
    );
    assert_eq!(
        incomplete["support_contribution_audit"]["completion"]["outcome"],
        "admitted_contribution_incomplete"
    );
}
