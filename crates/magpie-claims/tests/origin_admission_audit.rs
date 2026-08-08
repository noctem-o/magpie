use magpie_claims::policy::{refutation_ceiling, support_ceiling, ClaimDomain, EvidenceKind};
use magpie_claims::{
    replay_origin_admission_context_v0, AdmittedOriginV0, ArtifactProvenanceAnchorSelectorV0,
    OriginAdmissionAuditCompletionV0, OriginAdmissionAuditV0, OriginAdmissionCandidateAuditV0,
    OriginAdmissionCandidateDispositionV0, OriginAdmissionDecisionV0,
    OriginAdmissionReplayContextV0, OriginBindingConflictV0, OriginBindingContextTraceV0,
    ResolutionArtifactObjectInputV0, ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0, ORIGIN_ADMISSION_AUDIT_SCHEMA_V0,
    ORIGIN_ADMISSION_POLICY_ID_V0, ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0,
    ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_ACQUISITION_SCHEMA_V0, ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const ACQUISITION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-bundle.json");
const ACQUISITION_ARTIFACT: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-artifact.txt");
const ADMITTED_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/origin-admission-audit-v0/admitted.json");
const DUPLICATE_COLLAPSE_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/origin-admission-audit-v0/duplicate-collapse.json");
const CONFLICT_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/origin-admission-audit-v0/conflict.json");
const SCOPED_UNRESOLVED_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/origin-admission-audit-v0/scoped-unresolved.json");
const INCOMPLETE_UNIVERSE_AUDIT_FIXTURE: &[u8] =
    include_bytes!("../../../fixtures/origin-admission-audit-v0/incomplete-universe.json");

const ACQUISITION_ROOT: &str = "4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e";
const ACQUISITION_DIGEST: &str = "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076";
const SELECTED_POLICY: &str = "magpie-origin-admission-v0";
const TRUSTED_KIND: &str = "human-review";
const TRUSTED_REFERENCE: &str = "authority:origin-review-v0";
const SEED: [u8; 32] = [49; 32];

#[derive(Clone, Debug)]
struct ContributionSpec {
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    scope: String,
}

impl ContributionSpec {
    fn new(label: &str, scope: &str) -> Self {
        Self {
            claim_id: format!("claim-{label}"),
            evidence_id: format!("evidence-{label}"),
            edge_id: format!("edge-{label}"),
            scope: scope.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
struct BindingMaterial {
    selector: ArtifactProvenanceAnchorSelectorV0,
    bytes: Vec<u8>,
}

struct AuditCase {
    context: OriginAdmissionReplayContextV0,
    closure: ResolutionContentClosureV0,
    audit: OriginAdmissionAuditV0,
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    ArtifactProvenanceAnchorSelectorV0::new(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        ACQUISITION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-acq-0001",
    )
}

fn acquisition_bundle_bytes(acquisition_id: &str, observed_locator: &str, run_id: &str) -> Vec<u8> {
    format!(
        concat!(
            "{{\"schema\":\"magpie-artifact-acquisition-v0\",",
            "\"acquisition_id\":\"{acquisition_id}\",",
            "\"acquisition_profile\":\"magpie-import-v0\",",
            "\"artifact\":{{\"algorithm\":\"sha256\",",
            "\"digest\":\"{artifact_digest}\"}},",
            "\"observed_locator\":\"{observed_locator}\",",
            "\"run_id\":\"{run_id}\"}}"
        ),
        acquisition_id = acquisition_id,
        artifact_digest = ACQUISITION_DIGEST,
        observed_locator = observed_locator,
        run_id = run_id,
    )
    .into_bytes()
}

fn acquisition_selector_for_bytes(
    bytes: &[u8],
    run_id: &str,
) -> ArtifactProvenanceAnchorSelectorV0 {
    ArtifactProvenanceAnchorSelectorV0::new(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        sha256_hex(bytes),
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        run_id,
    )
}

fn binding_material(
    label: &str,
    contribution: &ContributionSpec,
    provenance_selector: &ArtifactProvenanceAnchorSelectorV0,
    policy_id: &str,
    origin_group: &str,
    authority_kind: &str,
    authority_reference: &str,
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
            "\"bundle_kind\":\"{provenance_kind}\",",
            "\"witness_root\":\"{provenance_root}\",",
            "\"witness_algorithm\":\"{provenance_algorithm}\",",
            "\"canonicalization_profile\":\"{provenance_profile}\",",
            "\"run_id\":\"{provenance_run_id}\"}},",
            "\"origin_group\":\"{origin_group}\",",
            "\"authority\":{{\"kind\":\"{authority_kind}\",",
            "\"reference\":\"{authority_reference}\"}},",
            "\"run_id\":\"{run_id}\"}}"
        ),
        schema = ORIGIN_BINDING_ACQUISITION_SCHEMA_V0,
        binding_id = binding_id,
        policy_id = policy_id,
        claim_id = contribution.claim_id,
        evidence_id = contribution.evidence_id,
        edge_id = contribution.edge_id,
        scope = contribution.scope,
        artifact_digest = ACQUISITION_DIGEST,
        provenance_kind = provenance_selector.bundle_kind(),
        provenance_root = provenance_selector.witness_root(),
        provenance_algorithm = provenance_selector.witness_algorithm(),
        provenance_profile = provenance_selector.canonicalization_profile(),
        provenance_run_id = provenance_selector.run_id(),
        origin_group = origin_group,
        authority_kind = authority_kind,
        authority_reference = authority_reference,
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

fn append_contribution(writer: &mut LogWriter<MemStore>, contribution: &ContributionSpec) {
    writer
        .append(
            Provenance::new("origin-admission-audit-test", "legacy-control"),
            Payload::ClaimAsserted {
                claim_id: contribution.claim_id.clone(),
                statement: "one origin-admission audit claim".into(),
                status: Status::Conjectured,
            },
        )
        .unwrap();
    writer
        .append(
            Provenance::new("origin-admission-audit-test", "typed-claim"),
            Payload::ClaimAssertedV2 {
                claim_id: contribution.claim_id.clone(),
                statement: "one origin-admission audit claim".into(),
                scope_ref: contribution.scope.clone(),
                actor_class: "AgentProposer".into(),
                content_hash: String::new(),
                metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
            },
        )
        .unwrap();
    writer
        .append(
            Provenance::new("origin-admission-audit-test", "typed-evidence"),
            Payload::EvidenceRegistered {
                evidence_id: contribution.evidence_id.clone(),
                evidence_kind: "ExternalSource".into(),
                summary: "one exact external contribution".into(),
                scope_ref: contribution.scope.clone(),
                actor_class: "SourceImporter".into(),
                content_hash: String::new(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
    writer
        .append(
            Provenance::new("origin-admission-audit-test", "typed-edge"),
            Payload::JustificationEdgeRecorded {
                edge_id: contribution.edge_id.clone(),
                edge_kind: "supports".into(),
                source_id: contribution.evidence_id.clone(),
                target_id: contribution.claim_id.clone(),
                scope_ref: contribution.scope.clone(),
                actor_class: "HumanRoot".into(),
                rationale: "candidate relationship only".into(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
}

fn replay_context(
    contributions: &[ContributionSpec],
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
        for contribution in contributions {
            append_contribution(&mut writer, contribution);
        }
        for selector in anchors {
            writer
                .append(
                    Provenance::new("origin-admission-audit-test", "same-replay-anchor"),
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
                    Provenance::new("origin-admission-audit-test", "ignored-note"),
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

fn complete_case(
    contributions: &[ContributionSpec],
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
    bindings: &[BindingMaterial],
    extra_bundles: &[(ArtifactProvenanceAnchorSelectorV0, Vec<u8>)],
    ignored_note: Option<&str>,
) -> AuditCase {
    let acquisition = acquisition_selector();
    let mut bundles = bindings
        .iter()
        .map(|binding| (binding.selector.clone(), binding.bytes.clone()))
        .collect::<Vec<_>>();
    bundles.push((acquisition, ACQUISITION_BUNDLE.to_vec()));
    bundles.extend_from_slice(extra_bundles);
    let closure = construct_closure(
        &[(ACQUISITION_DIGEST.to_owned(), ACQUISITION_ARTIFACT.to_vec())],
        &bundles,
    );
    let context = replay_context(contributions, anchors, ignored_note);
    let audit = context.resolve_origin_admission_audit_v0(&closure);
    AuditCase {
        context,
        closure,
        audit,
    }
}

fn candidate<'a>(
    audit: &'a OriginAdmissionAuditV0,
    selector: &ArtifactProvenanceAnchorSelectorV0,
) -> &'a OriginAdmissionCandidateAuditV0 {
    audit
        .candidate_audits()
        .iter()
        .find(|candidate| candidate.selector() == selector)
        .expect("candidate selector must be present")
}

fn admitted(decision: &OriginAdmissionDecisionV0) -> &AdmittedOriginV0 {
    match decision {
        OriginAdmissionDecisionV0::OriginAdmitted { admitted_origin } => admitted_origin,
        _ => panic!("expected admitted-origin decision"),
    }
}

fn conflict(decision: &OriginAdmissionDecisionV0) -> &OriginBindingConflictV0 {
    match decision {
        OriginAdmissionDecisionV0::ConflictingBindings { conflict } => conflict,
        _ => panic!("expected conflict decision"),
    }
}

fn assert_complete(audit: &OriginAdmissionAuditV0) {
    assert_eq!(
        audit.completion(),
        &OriginAdmissionAuditCompletionV0::Complete
    );
}

fn admitted_fixture_case() -> (AuditCase, BindingMaterial) {
    let contribution = ContributionSpec::new("fixture-admitted", "scope:fixture-admitted");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "fixture-admitted",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [binding.selector.clone(), acquisition];
    (
        complete_case(
            std::slice::from_ref(&contribution),
            &anchors,
            std::slice::from_ref(&binding),
            &[],
            Some("fixture admitted history"),
        ),
        binding,
    )
}

fn duplicate_fixture_case() -> (AuditCase, Vec<BindingMaterial>) {
    let contribution = ContributionSpec::new("fixture-duplicate", "scope:fixture-duplicate");
    let acquisition = acquisition_selector();
    let bindings = vec![
        binding_material(
            "fixture-duplicate-a",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-shared",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "fixture-duplicate-b",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-shared",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[1].selector.clone(),
        acquisition,
        bindings[0].selector.clone(),
    ];
    (
        complete_case(
            std::slice::from_ref(&contribution),
            &anchors,
            &bindings,
            &[],
            Some("fixture duplicate history"),
        ),
        bindings,
    )
}

fn conflict_fixture_case() -> (AuditCase, Vec<BindingMaterial>) {
    let contribution = ContributionSpec::new("fixture-conflict", "scope:fixture-conflict");
    let acquisition = acquisition_selector();
    let bindings = vec![
        binding_material(
            "fixture-conflict-z",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-zulu",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "fixture-conflict-a",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        acquisition,
    ];
    (
        complete_case(
            std::slice::from_ref(&contribution),
            &anchors,
            &bindings,
            &[],
            Some("fixture conflict history"),
        ),
        bindings,
    )
}

fn scoped_unresolved_fixture_case() -> (AuditCase, Vec<BindingMaterial>) {
    let contribution = ContributionSpec::new("fixture-unresolved", "scope:fixture-unresolved");
    let acquisition = acquisition_selector();
    let unavailable_bundle = acquisition_bundle_bytes(
        "acq-fixture-unresolved",
        "file:fixture-unresolved.txt",
        "run-acq-fixture-unresolved",
    );
    let unavailable_selector =
        acquisition_selector_for_bytes(&unavailable_bundle, "run-acq-fixture-unresolved");
    let bindings = vec![
        binding_material(
            "fixture-unresolved-matched",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "fixture-unresolved-missing",
            &contribution,
            &unavailable_selector,
            SELECTED_POLICY,
            "origin-group-beta",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        acquisition,
    ];
    (
        complete_case(
            std::slice::from_ref(&contribution),
            &anchors,
            &bindings,
            &[],
            Some("fixture unresolved history"),
        ),
        bindings,
    )
}

fn incomplete_fixture_case() -> (AuditCase, Vec<BindingMaterial>) {
    let contribution = ContributionSpec::new("fixture-incomplete", "scope:fixture-incomplete");
    let acquisition = acquisition_selector();
    let bindings = vec![
        binding_material(
            "fixture-incomplete-present",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "fixture-incomplete-absent",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-beta",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        acquisition.clone(),
    ];
    let closure = construct_closure(
        &[(ACQUISITION_DIGEST.to_owned(), ACQUISITION_ARTIFACT.to_vec())],
        &[
            (bindings[0].selector.clone(), bindings[0].bytes.clone()),
            (acquisition, ACQUISITION_BUNDLE.to_vec()),
        ],
    );
    let context = replay_context(
        std::slice::from_ref(&contribution),
        &anchors,
        Some("fixture incomplete history"),
    );
    let audit = context.resolve_origin_admission_audit_v0(&closure);
    (
        AuditCase {
            context,
            closure,
            audit,
        },
        bindings,
    )
}

#[test]
fn complete_trusted_admission_exposes_the_exact_public_audit_surface() {
    let (case, binding) = admitted_fixture_case();
    let audit = &case.audit;

    assert_eq!(
        ORIGIN_ADMISSION_AUDIT_SCHEMA_V0,
        "magpie-origin-admission-audit-v0"
    );
    assert_eq!(
        ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0,
        "magpie-origin-admission-audit-json-v0"
    );
    assert_eq!(ORIGIN_ADMISSION_POLICY_ID_V0, SELECTED_POLICY);
    assert_eq!(ORIGIN_ADMISSION_TRUSTED_AUTHORITY_KIND_V0, TRUSTED_KIND);
    assert_eq!(
        ORIGIN_ADMISSION_TRUSTED_AUTHORITY_REFERENCE_V0,
        TRUSTED_REFERENCE
    );
    assert_eq!(audit.schema(), ORIGIN_ADMISSION_AUDIT_SCHEMA_V0);
    assert_eq!(
        audit.canonicalization_profile(),
        ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(audit.policy_id(), ORIGIN_ADMISSION_POLICY_ID_V0);
    assert_eq!(
        audit.verified_prefix_identity(),
        case.context.verified_prefix_identity()
    );
    assert_eq!(audit.closure_identity(), case.closure.identity());
    assert_complete(audit);
    assert_eq!(audit.candidate_audits().len(), 1);
    let candidate = candidate(audit, &binding.selector);
    assert_eq!(candidate.anchor_occurrences().len(), 1);
    assert!(candidate.trace().matched_acquisition().is_some());
    assert_eq!(
        candidate.disposition(),
        &OriginAdmissionCandidateDispositionV0::TrustedMatched {
            origin_group: "origin-group-alpha".into(),
        }
    );
    assert_eq!(
        candidate
            .validated_contribution()
            .unwrap()
            .target_claim_id(),
        "claim-fixture-admitted"
    );
    assert_eq!(
        candidate
            .validated_namespace()
            .unwrap()
            .origin_admission_policy_id(),
        SELECTED_POLICY
    );
    assert_eq!(audit.contribution_decisions().len(), 1);
    let admitted = admitted(&audit.contribution_decisions()[0]);
    assert_eq!(admitted.policy_id(), SELECTED_POLICY);
    assert_eq!(admitted.origin_group(), "origin-group-alpha");
    assert_eq!(
        admitted.supporting_candidate_selectors(),
        std::slice::from_ref(&binding.selector)
    );
    assert_eq!(
        admitted.namespace().target_claim_id(),
        admitted.contribution().target_claim_id()
    );
}

#[test]
fn global_incomplete_universe_retains_every_candidate_and_emits_no_decisions() {
    let (case, bindings) = incomplete_fixture_case();
    assert_eq!(case.audit.candidate_audits().len(), 2);
    assert!(matches!(
        candidate(&case.audit, &bindings[0].selector).disposition(),
        OriginAdmissionCandidateDispositionV0::TrustedMatched { .. }
    ));
    assert_eq!(
        candidate(&case.audit, &bindings[1].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::BindingBytesUnavailable
    );
    assert_eq!(
        candidate(&case.audit, &bindings[1].selector).trace(),
        &OriginBindingContextTraceV0::BindingBundleUnavailable
    );
    let unavailable = match case.audit.completion() {
        OriginAdmissionAuditCompletionV0::IncompleteCandidateUniverse {
            unavailable_binding_selectors,
        } => unavailable_binding_selectors,
        _ => panic!("expected incomplete candidate universe"),
    };
    assert_eq!(unavailable, std::slice::from_ref(&bindings[1].selector));
    assert!(case.audit.contribution_decisions().is_empty());
}

#[test]
fn unanchored_closure_bundle_never_becomes_a_candidate_or_fold_input() {
    let contribution = ContributionSpec::new("extra-bundle", "scope:extra-bundle");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "extra-bundle",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let extra_selector = ArtifactProvenanceAnchorSelectorV0::new(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        "e".repeat(64),
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        "run-unanchored-extra",
    );
    let anchors = [binding.selector.clone(), acquisition];
    let without_extra = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        std::slice::from_ref(&binding),
        &[],
        None,
    );
    let with_extra = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        std::slice::from_ref(&binding),
        &[(extra_selector.clone(), b"unanchored closure bytes".to_vec())],
        None,
    );

    assert_eq!(without_extra.audit.candidate_audits().len(), 1);
    assert_eq!(with_extra.audit.candidate_audits().len(), 1);
    assert!(with_extra
        .audit
        .candidate_audits()
        .iter()
        .all(|candidate| candidate.selector() != &extra_selector));
    assert_eq!(
        without_extra.audit.candidate_audits(),
        with_extra.audit.candidate_audits()
    );
    assert_eq!(
        without_extra.audit.contribution_decisions(),
        with_extra.audit.contribution_decisions()
    );
    assert_ne!(
        without_extra.audit.closure_identity(),
        with_extra.audit.closure_identity()
    );
}

#[test]
fn repeated_exact_anchor_occurrences_remain_one_candidate_and_one_fold_input() {
    let contribution = ContributionSpec::new("repeated", "scope:repeated");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "repeated",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [
        binding.selector.clone(),
        acquisition,
        binding.selector.clone(),
        binding.selector.clone(),
    ];
    let case = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        std::slice::from_ref(&binding),
        &[],
        None,
    );
    assert_eq!(case.audit.candidate_audits().len(), 1);
    let candidate = candidate(&case.audit, &binding.selector);
    assert_eq!(candidate.anchor_occurrences().len(), 3);
    assert_eq!(
        candidate
            .anchor_occurrences()
            .iter()
            .map(|occurrence| occurrence.sequence)
            .collect::<Vec<_>>(),
        vec![5, 7, 8]
    );
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    assert_eq!(
        admitted(&case.audit.contribution_decisions()[0]).supporting_candidate_selectors(),
        std::slice::from_ref(&binding.selector)
    );
}

#[test]
fn duplicate_trusted_assignments_collapse_without_amplification() {
    let (case, bindings) = duplicate_fixture_case();
    assert_complete(&case.audit);
    assert_eq!(case.audit.candidate_audits().len(), 2);
    assert!(case.audit.candidate_audits().iter().all(|candidate| {
        matches!(
            candidate.disposition(),
            OriginAdmissionCandidateDispositionV0::TrustedMatched { .. }
        )
    }));
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    let admitted = admitted(&case.audit.contribution_decisions()[0]);
    assert_eq!(admitted.origin_group(), "origin-group-shared");
    let expected = bindings
        .iter()
        .map(|binding| binding.selector.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(admitted.supporting_candidate_selectors(), expected);
}

#[test]
fn trusted_conflict_is_terminal_and_admits_no_group() {
    let (case, _bindings) = conflict_fixture_case();
    assert_complete(&case.audit);
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    let conflict = conflict(&case.audit.contribution_decisions()[0]);
    assert_eq!(
        conflict.origin_groups(),
        ["origin-group-alpha", "origin-group-zulu"]
    );
    assert_eq!(conflict.matched_candidate_selectors().len(), 2);
    assert!(conflict.unresolved_candidate_selectors().is_empty());
}

#[test]
fn untrusted_and_policy_mismatched_bindings_are_visible_but_non_vetoing() {
    let contribution = ContributionSpec::new("non-veto", "scope:non-veto");
    let acquisition = acquisition_selector();
    let bindings = vec![
        binding_material(
            "non-veto-trusted",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "non-veto-untrusted",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-beta",
            "model-review",
            "authority:model-review-v0",
        ),
        binding_material(
            "non-veto-policy",
            &contribution,
            &acquisition,
            "magpie-origin-admission-other-v0",
            "origin-group-gamma",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        bindings[2].selector.clone(),
        acquisition,
    ];
    let case = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        &bindings,
        &[],
        None,
    );
    assert_eq!(
        candidate(&case.audit, &bindings[1].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::AuthorityUntrusted {
            authority_kind: "model-review".into(),
            authority_reference: "authority:model-review-v0".into(),
        }
    );
    assert_eq!(
        candidate(&case.audit, &bindings[2].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::PolicyMismatch {
            claimed_policy_id: "magpie-origin-admission-other-v0".into(),
        }
    );
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    assert_eq!(
        admitted(&case.audit.contribution_decisions()[0]).origin_group(),
        "origin-group-alpha"
    );
}

#[test]
fn same_key_eligible_unresolved_blocks_admission_without_entering_group_partition() {
    let (case, bindings) = scoped_unresolved_fixture_case();
    assert_complete(&case.audit);
    let unresolved_candidate = bindings
        .iter()
        .find(|binding| {
            matches!(
                candidate(&case.audit, &binding.selector).disposition(),
                OriginAdmissionCandidateDispositionV0::TrustedEligibleUnresolved { .. }
            )
        })
        .unwrap();
    assert!(matches!(
        candidate(&case.audit, &unresolved_candidate.selector).trace(),
        OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved { .. }
    ));
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    match &case.audit.contribution_decisions()[0] {
        OriginAdmissionDecisionV0::EligibleBindingUnresolved {
            matched_candidate_selectors,
            unresolved_candidate_selectors,
            ..
        } => {
            assert_eq!(matched_candidate_selectors.len(), 1);
            assert_eq!(
                unresolved_candidate_selectors,
                std::slice::from_ref(&unresolved_candidate.selector)
            );
        }
        _ => panic!("expected eligible unresolved decision"),
    }
}

#[test]
fn known_conflict_precedes_same_key_unresolved_and_retains_the_blocker() {
    let contribution = ContributionSpec::new("conflict-unresolved", "scope:conflict-unresolved");
    let acquisition = acquisition_selector();
    let unavailable_bundle = acquisition_bundle_bytes(
        "acq-conflict-unresolved",
        "file:conflict-unresolved.txt",
        "run-acq-conflict-unresolved",
    );
    let unavailable_selector =
        acquisition_selector_for_bytes(&unavailable_bundle, "run-acq-conflict-unresolved");
    let bindings = vec![
        binding_material(
            "conflict-unresolved-a",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "conflict-unresolved-b",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-beta",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "conflict-unresolved-missing",
            &contribution,
            &unavailable_selector,
            SELECTED_POLICY,
            "origin-group-gamma",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = vec![
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        bindings[2].selector.clone(),
        acquisition,
    ];
    let case = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        &bindings,
        &[],
        None,
    );
    let conflict = conflict(&case.audit.contribution_decisions()[0]);
    assert_eq!(
        conflict.origin_groups(),
        ["origin-group-alpha", "origin-group-beta"]
    );
    assert_eq!(
        conflict.unresolved_candidate_selectors(),
        std::slice::from_ref(&bindings[2].selector)
    );
}

#[test]
fn scoped_unresolved_isolation_allows_an_unrelated_key_to_be_admitted() {
    let contribution_a = ContributionSpec::new("scope-a", "scope:scope-a");
    let contribution_b = ContributionSpec::new("scope-b", "scope:scope-b");
    let acquisition = acquisition_selector();
    let unavailable_bundle =
        acquisition_bundle_bytes("acq-scope-a", "file:scope-a.txt", "run-acq-scope-a");
    let unavailable_selector =
        acquisition_selector_for_bytes(&unavailable_bundle, "run-acq-scope-a");
    let unresolved = binding_material(
        "scope-a-unresolved",
        &contribution_a,
        &unavailable_selector,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let matched = binding_material(
        "scope-b-matched",
        &contribution_b,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-beta",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let bindings = [unresolved.clone(), matched.clone()];
    let anchors = [
        unresolved.selector.clone(),
        matched.selector.clone(),
        acquisition,
    ];
    let case = complete_case(
        &[contribution_a, contribution_b],
        &anchors,
        &bindings,
        &[],
        None,
    );

    assert_eq!(case.audit.contribution_decisions().len(), 2);
    assert!(case.audit.contribution_decisions().iter().any(|decision| {
        matches!(
            decision,
            OriginAdmissionDecisionV0::EligibleBindingUnresolved {
                contribution,
                ..
            } if contribution.target_claim_id() == "claim-scope-a"
        )
    }));
    assert!(case.audit.contribution_decisions().iter().any(|decision| {
        matches!(
            decision,
            OriginAdmissionDecisionV0::OriginAdmitted {
                admitted_origin,
            } if admitted_origin.contribution().target_claim_id() == "claim-scope-b"
                && admitted_origin.origin_group() == "origin-group-beta"
        )
    }));
}

#[test]
fn definitive_failures_and_disposition_precedence_are_exact_and_non_blocking() {
    let contribution = ContributionSpec::new("precedence", "scope:precedence");
    let acquisition = acquisition_selector();
    let malformed_selector = acquisition_selector_for_bytes(b"{", "run-acq-malformed-precedence");
    let unavailable_bundle = acquisition_bundle_bytes(
        "acq-unavailable-precedence",
        "file:unavailable-precedence.txt",
        "run-acq-unavailable-precedence",
    );
    let unavailable_selector =
        acquisition_selector_for_bytes(&unavailable_bundle, "run-acq-unavailable-precedence");
    let bindings = vec![
        binding_material(
            "precedence-trusted",
            &contribution,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-alpha",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "precedence-definitive",
            &contribution,
            &malformed_selector,
            SELECTED_POLICY,
            "origin-group-beta",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "precedence-wrong-policy-definitive",
            &contribution,
            &malformed_selector,
            "magpie-origin-admission-other-v0",
            "origin-group-gamma",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "precedence-wrong-policy-availability",
            &contribution,
            &unavailable_selector,
            "magpie-origin-admission-other-v0",
            "origin-group-delta",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "precedence-untrusted-availability",
            &contribution,
            &unavailable_selector,
            SELECTED_POLICY,
            "origin-group-epsilon",
            "model-review",
            "authority:model-review-v0",
        ),
    ];
    let anchors = bindings
        .iter()
        .map(|binding| binding.selector.clone())
        .chain(std::iter::once(acquisition.clone()))
        .collect::<Vec<_>>();
    let case = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        &bindings,
        &[(malformed_selector, b"{".to_vec())],
        None,
    );

    assert_eq!(
        candidate(&case.audit, &bindings[1].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::DefinitivelyRejected
    );
    assert!(candidate(&case.audit, &bindings[1].selector)
        .validated_contribution()
        .is_some());
    assert_eq!(
        candidate(&case.audit, &bindings[2].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::DefinitivelyRejected
    );
    assert_eq!(
        candidate(&case.audit, &bindings[3].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::PolicyMismatch {
            claimed_policy_id: "magpie-origin-admission-other-v0".into(),
        }
    );
    assert_eq!(
        candidate(&case.audit, &bindings[4].selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::AuthorityUntrusted {
            authority_kind: "model-review".into(),
            authority_reference: "authority:model-review-v0".into(),
        }
    );
    assert_eq!(case.audit.contribution_decisions().len(), 1);
    assert_eq!(
        admitted(&case.audit.contribution_decisions()[0]).origin_group(),
        "origin-group-alpha"
    );
}

#[test]
fn acquisition_artifact_unavailability_is_eligible_but_digest_failure_is_definitive() {
    let contribution = ContributionSpec::new("availability", "scope:availability");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "availability",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [binding.selector.clone(), acquisition.clone()];
    let context = replay_context(std::slice::from_ref(&contribution), &anchors, None);
    let artifact_unavailable_closure = construct_closure(
        &[],
        &[
            (binding.selector.clone(), binding.bytes.clone()),
            (acquisition.clone(), ACQUISITION_BUNDLE.to_vec()),
        ],
    );
    let unavailable = context.resolve_origin_admission_audit_v0(&artifact_unavailable_closure);
    assert_eq!(
        candidate(&unavailable, &binding.selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::TrustedEligibleUnresolved {
            claimed_origin_group: "origin-group-alpha".into(),
        }
    );

    let digest_mismatch_closure = construct_closure(
        &[(ACQUISITION_DIGEST.to_owned(), b"wrong artifact".to_vec())],
        &[
            (binding.selector.clone(), binding.bytes.clone()),
            (acquisition, ACQUISITION_BUNDLE.to_vec()),
        ],
    );
    let mismatch = context.resolve_origin_admission_audit_v0(&digest_mismatch_closure);
    assert_eq!(
        candidate(&mismatch, &binding.selector).disposition(),
        &OriginAdmissionCandidateDispositionV0::DefinitivelyRejected
    );
    assert!(mismatch.contribution_decisions().is_empty());
}

#[test]
fn identical_group_text_in_different_namespaces_produces_separate_decisions() {
    let contribution_a = ContributionSpec::new("namespace-a", "scope:namespace-a");
    let contribution_b = ContributionSpec::new("namespace-b", "scope:namespace-b");
    let acquisition = acquisition_selector();
    let bindings = [
        binding_material(
            "namespace-a",
            &contribution_a,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-shared-text",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
        binding_material(
            "namespace-b",
            &contribution_b,
            &acquisition,
            SELECTED_POLICY,
            "origin-group-shared-text",
            TRUSTED_KIND,
            TRUSTED_REFERENCE,
        ),
    ];
    let anchors = [
        bindings[0].selector.clone(),
        bindings[1].selector.clone(),
        acquisition,
    ];
    let case = complete_case(
        &[contribution_a, contribution_b],
        &anchors,
        &bindings,
        &[],
        None,
    );
    assert_eq!(case.audit.contribution_decisions().len(), 2);
    let admitted = case
        .audit
        .contribution_decisions()
        .iter()
        .map(admitted)
        .collect::<Vec<_>>();
    assert!(admitted
        .iter()
        .all(|origin| origin.origin_group() == "origin-group-shared-text"));
    assert_ne!(admitted[0].namespace(), admitted[1].namespace());
}

#[test]
fn deterministic_input_closure_order_and_identity_laws_are_exact() {
    let contribution = ContributionSpec::new("deterministic", "scope:deterministic");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "deterministic",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [binding.selector.clone(), acquisition.clone()];
    let context = replay_context(std::slice::from_ref(&contribution), &anchors, None);
    let artifacts = [(ACQUISITION_DIGEST.to_owned(), ACQUISITION_ARTIFACT.to_vec())];
    let forward_bundles = [
        (binding.selector.clone(), binding.bytes.clone()),
        (acquisition.clone(), ACQUISITION_BUNDLE.to_vec()),
    ];
    let reverse_bundles = [
        (acquisition.clone(), ACQUISITION_BUNDLE.to_vec()),
        (binding.selector.clone(), binding.bytes.clone()),
    ];
    let forward = construct_closure(&artifacts, &forward_bundles);
    let reverse = construct_closure(&artifacts, &reverse_bundles);
    assert_eq!(forward.identity(), reverse.identity());

    let first = context.resolve_origin_admission_audit_v0(&forward);
    let second = context.resolve_origin_admission_audit_v0(&forward);
    let reordered = context.resolve_origin_admission_audit_v0(&reverse);
    assert_eq!(first, second);
    assert_eq!(first, reordered);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(first.canonical_bytes(), reordered.canonical_bytes());

    let extra_selector = ArtifactProvenanceAnchorSelectorV0::new(
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
            (acquisition, ACQUISITION_BUNDLE.to_vec()),
            (extra_selector, b"different closure identity".to_vec()),
        ],
    );
    let different_audit = context.resolve_origin_admission_audit_v0(&different);
    assert_ne!(forward.identity(), different.identity());
    assert_ne!(first.closure_identity(), different_audit.closure_identity());
    assert_ne!(first.canonical_bytes(), different_audit.canonical_bytes());
}

#[test]
fn different_ignored_signed_history_changes_prefix_identity_not_snapshot_bytes() {
    let contribution = ContributionSpec::new("history", "scope:history");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "history",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [binding.selector.clone(), acquisition.clone()];
    let context_a = replay_context(
        std::slice::from_ref(&contribution),
        &anchors,
        Some("ignored history A"),
    );
    let context_b = replay_context(
        std::slice::from_ref(&contribution),
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
        &[(ACQUISITION_DIGEST.to_owned(), ACQUISITION_ARTIFACT.to_vec())],
        &[
            (binding.selector, binding.bytes),
            (acquisition, ACQUISITION_BUNDLE.to_vec()),
        ],
    );
    let audit_a = context_a.resolve_origin_admission_audit_v0(&closure);
    let audit_b = context_b.resolve_origin_admission_audit_v0(&closure);
    assert_ne!(
        audit_a.verified_prefix_identity(),
        audit_b.verified_prefix_identity()
    );
    assert_ne!(audit_a.canonical_bytes(), audit_b.canonical_bytes());
}

#[test]
fn audit_execution_is_byte_for_byte_standing_and_verifier_inert() {
    let contribution = ContributionSpec::new("inert", "scope:inert");
    let acquisition = acquisition_selector();
    let binding = binding_material(
        "inert",
        &contribution,
        &acquisition,
        SELECTED_POLICY,
        "origin-group-alpha",
        TRUSTED_KIND,
        TRUSTED_REFERENCE,
    );
    let anchors = [binding.selector.clone(), acquisition.clone()];
    let case = complete_case(
        std::slice::from_ref(&contribution),
        &anchors,
        std::slice::from_ref(&binding),
        &[],
        None,
    );
    let snapshot = case.context.snapshot();
    let snapshot_before = snapshot.canonical_bytes();
    let standing_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace(&contribution.claim_id);
    let v1_before = snapshot.resolved_standing_with_trace_v1(&contribution.claim_id);
    let v2_before = snapshot.resolved_standing_with_trace_v2(&contribution.claim_id);
    let deterministic_before = snapshot.resolve_deterministic_verifier_context_v0(
        &contribution.claim_id,
        &contribution.evidence_id,
        &contribution.edge_id,
    );
    let origin_binding_before =
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &case.closure);
    let ceilings_before = EvidenceKind::ALL
        .into_iter()
        .flat_map(|kind| {
            ClaimDomain::ALL.into_iter().map(move |domain| {
                (
                    kind.as_str(),
                    domain.as_str(),
                    support_ceiling(kind, domain),
                    refutation_ceiling(kind, domain),
                )
            })
        })
        .collect::<Vec<_>>();

    for _ in 0..3 {
        let audit = case
            .context
            .resolve_origin_admission_audit_v0(&case.closure);
        assert_eq!(audit, case.audit);
    }

    assert_eq!(snapshot_before, snapshot.canonical_bytes());
    assert_eq!(standing_before, snapshot.standing().canonical_bytes());
    assert_eq!(
        v0_before,
        snapshot
            .standing()
            .resolved_standing_with_trace(&contribution.claim_id)
    );
    assert_eq!(
        v1_before,
        snapshot.resolved_standing_with_trace_v1(&contribution.claim_id)
    );
    assert_eq!(
        v2_before,
        snapshot.resolved_standing_with_trace_v2(&contribution.claim_id)
    );
    assert_eq!(
        deterministic_before,
        snapshot.resolve_deterministic_verifier_context_v0(
            &contribution.claim_id,
            &contribution.evidence_id,
            &contribution.edge_id,
        )
    );
    let origin_binding_after =
        snapshot.resolve_origin_binding_context_v0(&binding.selector, &case.closure);
    assert_eq!(origin_binding_before, origin_binding_after);
    assert_eq!(
        origin_binding_before.canonical_bytes(),
        origin_binding_after.canonical_bytes()
    );
    let ceilings_after = EvidenceKind::ALL
        .into_iter()
        .flat_map(|kind| {
            ClaimDomain::ALL.into_iter().map(move |domain| {
                (
                    kind.as_str(),
                    domain.as_str(),
                    support_ceiling(kind, domain),
                    refutation_ceiling(kind, domain),
                )
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(ceilings_before, ceilings_after);
    assert_eq!(v0_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v1_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v2_before.governed_standing, Some(Status::Conjectured));
}

#[test]
fn serialized_audit_output_is_deterministic_non_authoritative_and_contains_no_raw_objects() {
    let (case, _binding) = admitted_fixture_case();
    let cloned = case.audit.clone();
    let bytes = case.audit.canonical_bytes();
    assert_eq!(bytes, cloned.canonical_bytes());
    assert_eq!(bytes.first(), Some(&b'{'));
    assert_eq!(bytes.last(), Some(&b'}'));
    assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!bytes.ends_with(b"\n"));
    assert!(!bytes.ends_with(b"\r"));
    assert!(!bytes
        .windows(ACQUISITION_ARTIFACT.len())
        .any(|window| window == ACQUISITION_ARTIFACT));
    assert!(!bytes
        .windows(ACQUISITION_BUNDLE.len())
        .any(|window| window == ACQUISITION_BUNDLE));
}

fn assert_literal_fixture(
    fixture: &[u8],
    audit: OriginAdmissionAuditV0,
    expected_len: usize,
    expected_sha256: &str,
) -> serde_json::Value {
    let production_bytes = audit.canonical_bytes();
    assert_eq!(production_bytes, fixture);
    assert_eq!(fixture.len(), expected_len);
    assert_eq!(sha256_hex(fixture), expected_sha256);
    assert_eq!(fixture.first(), Some(&b'{'));
    assert_eq!(fixture.last(), Some(&b'}'));
    assert!(!fixture.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!fixture.ends_with(b"\n"));
    assert!(!fixture.ends_with(b"\r"));

    let value: serde_json::Value = serde_json::from_slice(fixture).unwrap();
    assert_eq!(value["schema"], ORIGIN_ADMISSION_AUDIT_SCHEMA_V0);
    assert_eq!(
        value["canonicalization_profile"],
        ORIGIN_ADMISSION_AUDIT_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(value["policy_id"], ORIGIN_ADMISSION_POLICY_ID_V0);
    value
}

fn candidate_run_ids(value: &serde_json::Value) -> Vec<&str> {
    value["candidate_audits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["selector"]["run_id"].as_str().unwrap())
        .collect()
}

fn candidate_dispositions(value: &serde_json::Value) -> Vec<&str> {
    value["candidate_audits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["disposition"]["outcome"].as_str().unwrap())
        .collect()
}

fn decision_outcomes(value: &serde_json::Value) -> Vec<&str> {
    value["contribution_decisions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decision| decision["outcome"].as_str().unwrap())
        .collect()
}

#[test]
fn normative_fixtures_match_production_and_independent_expected_structure() {
    let admitted = assert_literal_fixture(
        ADMITTED_AUDIT_FIXTURE,
        admitted_fixture_case().0.audit,
        6303,
        "b10ca460595b9402d52e483d4ff0aa592ca6aef4996316f446cfb29cd28b8a8f",
    );
    assert_eq!(admitted["completion"]["outcome"], "complete");
    assert_eq!(
        candidate_run_ids(&admitted),
        ["run-origin-binding-fixture-admitted"]
    );
    assert_eq!(candidate_dispositions(&admitted), ["trusted_matched"]);
    assert_eq!(decision_outcomes(&admitted), ["origin_admitted"]);
    assert_eq!(
        admitted["contribution_decisions"][0]["details"]["admitted_origin"]["origin_group"],
        "origin-group-alpha"
    );
    assert_eq!(
        admitted["contribution_decisions"][0]["details"]["admitted_origin"]
            ["supporting_candidate_selectors"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let duplicate = assert_literal_fixture(
        DUPLICATE_COLLAPSE_AUDIT_FIXTURE,
        duplicate_fixture_case().0.audit,
        11433,
        "a2b4ee3559598ab970f74e841dadc2898fbdb7dd91ee4170182c1da735175131",
    );
    assert_eq!(duplicate["completion"]["outcome"], "complete");
    assert_eq!(
        candidate_run_ids(&duplicate),
        [
            "run-origin-binding-fixture-duplicate-a",
            "run-origin-binding-fixture-duplicate-b",
        ]
    );
    assert_eq!(
        candidate_dispositions(&duplicate),
        ["trusted_matched", "trusted_matched"]
    );
    assert_eq!(decision_outcomes(&duplicate), ["origin_admitted"]);
    assert_eq!(
        duplicate["contribution_decisions"][0]["details"]["admitted_origin"]["origin_group"],
        "origin-group-shared"
    );
    assert_eq!(
        duplicate["contribution_decisions"][0]["details"]["admitted_origin"]
            ["supporting_candidate_selectors"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let conflict = assert_literal_fixture(
        CONFLICT_AUDIT_FIXTURE,
        conflict_fixture_case().0.audit,
        11436,
        "a20515271e5479c01e1a952ff1607826fa4e5de89bc4957fcef5d2ef48d92494",
    );
    assert_eq!(conflict["completion"]["outcome"], "complete");
    assert_eq!(
        candidate_run_ids(&conflict),
        [
            "run-origin-binding-fixture-conflict-a",
            "run-origin-binding-fixture-conflict-z",
        ]
    );
    assert_eq!(
        candidate_dispositions(&conflict),
        ["trusted_matched", "trusted_matched"]
    );
    assert_eq!(decision_outcomes(&conflict), ["conflicting_bindings"]);
    assert_eq!(
        conflict["contribution_decisions"][0]["details"]["conflict"]["origin_groups"],
        serde_json::json!(["origin-group-alpha", "origin-group-zulu"])
    );
    assert_eq!(
        conflict["contribution_decisions"][0]["details"]["conflict"]["matched_candidate_selectors"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let unresolved = assert_literal_fixture(
        SCOPED_UNRESOLVED_AUDIT_FIXTURE,
        scoped_unresolved_fixture_case().0.audit,
        8190,
        "f2d31a951324a49c8694eb0ca5e6c914f0e31601962e920eefc97f16d45f7b41",
    );
    assert_eq!(unresolved["completion"]["outcome"], "complete");
    assert_eq!(
        candidate_run_ids(&unresolved),
        [
            "run-origin-binding-fixture-unresolved-matched",
            "run-origin-binding-fixture-unresolved-missing",
        ]
    );
    assert_eq!(
        candidate_dispositions(&unresolved),
        ["trusted_matched", "trusted_eligible_unresolved"]
    );
    assert_eq!(
        decision_outcomes(&unresolved),
        ["eligible_binding_unresolved"]
    );
    assert_eq!(
        unresolved["contribution_decisions"][0]["details"]["matched_candidate_selectors"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        unresolved["contribution_decisions"][0]["details"]["unresolved_candidate_selectors"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    let incomplete = assert_literal_fixture(
        INCOMPLETE_UNIVERSE_AUDIT_FIXTURE,
        incomplete_fixture_case().0.audit,
        6783,
        "9a5ea4489be404055193578cc88e395c1e50d34953403e99a2c7db2a0eeb7a3f",
    );
    assert_eq!(
        incomplete["completion"]["outcome"],
        "incomplete_candidate_universe"
    );
    assert_eq!(
        candidate_run_ids(&incomplete),
        [
            "run-origin-binding-fixture-incomplete-present",
            "run-origin-binding-fixture-incomplete-absent",
        ]
    );
    assert_eq!(
        candidate_dispositions(&incomplete),
        ["trusted_matched", "binding_bytes_unavailable"]
    );
    assert!(decision_outcomes(&incomplete).is_empty());
    assert_eq!(
        incomplete["completion"]["details"]["unavailable_binding_selectors"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(incomplete["candidate_audits"][1]["validated_contribution"].is_null());
    assert!(incomplete["candidate_audits"][1]["validated_namespace"].is_null());
}
