use magpie_claims::{
    replay_standing_context, ArtifactProvenanceAnchorSelectorV0, ArtifactProvenanceContextTraceV0,
    OriginBindingContextTraceV0, OriginBindingFamilyV0, ResolutionArtifactObjectInputV0,
    ResolutionArtifactObjectKeyV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, StandingReplaySnapshot,
    ARTIFACT_ACQUISITION_BUNDLE_KIND_V0, ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0, ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_ACQUISITION_SCHEMA_V0, ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0,
    ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0, ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
    ORIGIN_BINDING_DERIVATION_SCHEMA_V0, ORIGIN_BINDING_VERIFIER_PROFILE_V0,
    ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const ACQUISITION_BINDING: &[u8] =
    include_bytes!("../../../fixtures/origin-binding-v0/acquisition-binding.json");
const DERIVATION_BINDING: &[u8] =
    include_bytes!("../../../fixtures/origin-binding-v0/derivation-binding.json");
const PARENT_ACQUISITION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/origin-binding-v0/derivation-parent-acquisition-bundle.json");
const ACQUISITION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-bundle.json");
const DERIVATION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-bundle.json");
const ACQUISITION_ARTIFACT: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-artifact.txt");
const DERIVATION_PARENT: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-parent.txt");
const DERIVATION_DERIVED: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-derived.txt");

const ACQUISITION_BINDING_ROOT: &str =
    "4f4bbaab87c1cfdc4dfe3b8c0d8b11a04bdc637de1e8e1481001b09543f0a4d7";
const DERIVATION_BINDING_ROOT: &str =
    "24aa77f2f601e70f6d5d6044f0b1274394f164276df07296be16ab59953a5a29";
const ACQUISITION_ROOT: &str = "4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e";
const PARENT_ACQUISITION_ROOT: &str =
    "94593dd609cb44b07c2f3dd5a72226ff4c02ad4b9f4378a84d4432c433a1f7ce";
const DERIVATION_ROOT: &str = "472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd";
const ACQUISITION_DIGEST: &str = "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076";
const PARENT_DIGEST: &str = "b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060";
const DERIVED_DIGEST: &str = "1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005";
const SCOPE: &str = "scope:origin-example";
const SEED: [u8; 32] = [45; 32];

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn selector(
    kind: &str,
    root: &str,
    algorithm: &str,
    profile: &str,
    run_id: &str,
) -> ArtifactProvenanceAnchorSelectorV0 {
    ArtifactProvenanceAnchorSelectorV0::new(kind, root, algorithm, profile, run_id)
}

fn selector_for_bundle(
    kind: &str,
    run_id: &str,
    bytes: &[u8],
) -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        kind,
        &sha256_hex(bytes),
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        run_id,
    )
}

fn acquisition_binding_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        ACQUISITION_BINDING_ROOT,
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        "run-origin-binding-acq-0001",
    )
}

fn derivation_binding_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        DERIVATION_BINDING_ROOT,
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        "run-origin-binding-der-0001",
    )
}

fn acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        ACQUISITION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-acq-0001",
    )
}

fn parent_acquisition_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        PARENT_ACQUISITION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-acq-alpha-0001",
    )
}

fn derivation_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
        DERIVATION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-der-0001",
    )
}

fn closure(
    artifacts: &[(&str, &[u8])],
    bundles: &[(&ArtifactProvenanceAnchorSelectorV0, &[u8])],
) -> ResolutionContentClosureV0 {
    let artifact_inputs: Vec<_> = artifacts
        .iter()
        .map(|(digest, bytes)| {
            ResolutionArtifactObjectInputV0::new(
                ResolutionArtifactObjectKeyV0::new("sha256", *digest),
                bytes,
            )
        })
        .collect();
    let bundle_inputs: Vec<_> = bundles
        .iter()
        .map(|(selector, bytes)| {
            ResolutionForeignBundleObjectInputV0::new((*selector).clone(), bytes)
        })
        .collect();
    ResolutionContentClosureV0::construct(&artifact_inputs, &bundle_inputs).unwrap()
}

fn acquisition_closure() -> ResolutionContentClosureV0 {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[
            (&binding, ACQUISITION_BINDING),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    )
}

fn derivation_closure() -> ResolutionContentClosureV0 {
    let binding = derivation_binding_selector();
    let acquisition = parent_acquisition_selector();
    let derivation = derivation_selector();
    closure(
        &[
            (PARENT_DIGEST, DERIVATION_PARENT),
            (DERIVED_DIGEST, DERIVATION_DERIVED),
        ],
        &[
            (&binding, DERIVATION_BINDING),
            (&acquisition, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    )
}

#[derive(Clone)]
struct ReplaySpec {
    include_legacy_claim: bool,
    include_claim: bool,
    include_evidence: bool,
    include_edge: bool,
    claim_id: String,
    evidence_id: String,
    edge_id: String,
    claim_scope: String,
    evidence_scope: String,
    edge_scope: String,
    edge_source: String,
    edge_target: String,
    extra_claim_id: Option<String>,
    extra_evidence_id: Option<String>,
}

impl ReplaySpec {
    fn acquisition() -> Self {
        Self {
            include_legacy_claim: true,
            include_claim: true,
            include_evidence: true,
            include_edge: true,
            claim_id: "claim-origin-0001".into(),
            evidence_id: "evidence-origin-acq-0001".into(),
            edge_id: "edge-origin-acq-0001".into(),
            claim_scope: SCOPE.into(),
            evidence_scope: SCOPE.into(),
            edge_scope: SCOPE.into(),
            edge_source: "evidence-origin-acq-0001".into(),
            edge_target: "claim-origin-0001".into(),
            extra_claim_id: None,
            extra_evidence_id: None,
        }
    }

    fn derivation() -> Self {
        Self {
            evidence_id: "evidence-origin-der-0001".into(),
            edge_id: "edge-origin-der-0001".into(),
            edge_source: "evidence-origin-der-0001".into(),
            ..Self::acquisition()
        }
    }
}

fn append_claim(writer: &mut LogWriter<MemStore>, claim_id: &str, scope: &str) {
    writer
        .append(
            Provenance::new("origin-binding-test", "claim"),
            Payload::ClaimAssertedV2 {
                claim_id: claim_id.into(),
                statement: "one origin-bound claim".into(),
                scope_ref: scope.into(),
                actor_class: "AgentProposer".into(),
                content_hash: String::new(),
                metadata_json: r#"{"claim_domain":"ExternalReport"}"#.into(),
            },
        )
        .unwrap();
}

fn append_evidence(writer: &mut LogWriter<MemStore>, evidence_id: &str, scope: &str) {
    writer
        .append(
            Provenance::new("origin-binding-test", "evidence"),
            Payload::EvidenceRegistered {
                evidence_id: evidence_id.into(),
                evidence_kind: "ExternalSource".into(),
                summary: "one source observation".into(),
                scope_ref: scope.into(),
                actor_class: "SourceImporter".into(),
                content_hash: String::new(),
                metadata_json: "{}".into(),
            },
        )
        .unwrap();
}

fn snapshot(
    spec: &ReplaySpec,
    anchors: &[ArtifactProvenanceAnchorSelectorV0],
) -> StandingReplaySnapshot {
    let store = MemStore::new();
    {
        let mut timestamp = 0;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            SigningKey::from_bytes(&SEED),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        if spec.include_legacy_claim {
            writer
                .append(
                    Provenance::new("origin-binding-test", "legacy-control"),
                    Payload::ClaimAsserted {
                        claim_id: spec.claim_id.clone(),
                        statement: "one origin-bound claim".into(),
                        status: Status::Conjectured,
                    },
                )
                .unwrap();
        }
        if spec.include_claim {
            append_claim(&mut writer, &spec.claim_id, &spec.claim_scope);
        }
        if spec.include_evidence {
            append_evidence(&mut writer, &spec.evidence_id, &spec.evidence_scope);
        }
        if let Some(claim_id) = &spec.extra_claim_id {
            append_claim(&mut writer, claim_id, &spec.claim_scope);
        }
        if let Some(evidence_id) = &spec.extra_evidence_id {
            append_evidence(&mut writer, evidence_id, &spec.evidence_scope);
        }
        if spec.include_edge {
            writer
                .append(
                    Provenance::new("origin-binding-test", "edge"),
                    Payload::JustificationEdgeRecorded {
                        edge_id: spec.edge_id.clone(),
                        edge_kind: "supports".into(),
                        source_id: spec.edge_source.clone(),
                        target_id: spec.edge_target.clone(),
                        scope_ref: spec.edge_scope.clone(),
                        actor_class: "HumanRoot".into(),
                        rationale: "candidate relationship only".into(),
                        metadata_json: "{}".into(),
                    },
                )
                .unwrap();
        }
        for anchor in anchors {
            writer
                .append(
                    Provenance::new("origin-binding-test", "same-replay-anchor"),
                    Payload::SegmentAnchored {
                        bundle_kind: anchor.bundle_kind().into(),
                        witness_root: anchor.witness_root().into(),
                        witness_algorithm: anchor.witness_algorithm().into(),
                        canonicalization_profile: anchor.canonicalization_profile().into(),
                        run_id: anchor.run_id().into(),
                    },
                )
                .unwrap();
        }
    }
    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    replay_standing_context(&reader).unwrap()
}

fn outcome(
    snapshot: &StandingReplaySnapshot,
    selector: &ArtifactProvenanceAnchorSelectorV0,
    closure: &ResolutionContentClosureV0,
) -> OriginBindingContextTraceV0 {
    snapshot.resolve_origin_binding_context_v0(selector, closure)
}

fn replace_once(bytes: &[u8], from: &str, to: &str) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).unwrap();
    assert_eq!(text.matches(from).count(), 1, "replacement must be exact");
    text.replacen(from, to, 1).into_bytes()
}

fn binding_outcome_for_bytes(bytes: &[u8]) -> OriginBindingContextTraceV0 {
    let binding = acquisition_binding_selector();
    let closure = closure(&[], &[(&binding, bytes)]);
    outcome(
        &snapshot(&ReplaySpec::acquisition(), &[]),
        &binding,
        &closure,
    )
}

#[test]
fn acquisition_vector_matches_and_retains_complete_audit_receipt() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[binding.clone(), acquisition.clone()],
    );
    let trace = outcome(&snapshot, &binding, &acquisition_closure());
    assert!(trace.matched_derivation().is_none());
    let receipt = trace.matched_acquisition().unwrap();

    assert_eq!(
        receipt.verifier_profile(),
        ORIGIN_BINDING_VERIFIER_PROFILE_V0
    );
    assert_eq!(receipt.binding_selector(), &binding);
    assert_eq!(receipt.family(), OriginBindingFamilyV0::Acquisition);
    assert_eq!(receipt.schema(), ORIGIN_BINDING_ACQUISITION_SCHEMA_V0);
    assert_eq!(receipt.binding_id(), "origin-binding-acq-0001");
    assert_eq!(
        receipt.origin_admission_policy_id(),
        "magpie-origin-admission-v0"
    );
    assert_eq!(
        receipt.contribution().target_claim_id(),
        "claim-origin-0001"
    );
    assert_eq!(
        receipt.contribution().source_evidence_id(),
        "evidence-origin-acq-0001"
    );
    assert_eq!(
        receipt.contribution().justification_edge_id(),
        "edge-origin-acq-0001"
    );
    assert_eq!(receipt.contribution().scope_ref(), SCOPE);
    assert_eq!(receipt.contribution().artifact_algorithm(), "sha256");
    assert_eq!(receipt.contribution().artifact_digest(), ACQUISITION_DIGEST);
    assert_eq!(
        receipt.comparison_namespace().origin_admission_policy_id(),
        "magpie-origin-admission-v0"
    );
    assert_eq!(
        receipt.comparison_namespace().target_claim_id(),
        "claim-origin-0001"
    );
    assert_eq!(receipt.comparison_namespace().scope_ref(), SCOPE);
    assert_eq!(receipt.acquisition_selector(), &acquisition);
    assert!(receipt.derivation_selector().is_none());
    assert_eq!(receipt.origin_group(), "origin-group-report-0001");
    assert_eq!(receipt.authority_kind(), "human-review");
    assert_eq!(receipt.authority_reference(), "authority:origin-review-v0");
    assert_eq!(receipt.binding_run_id(), "run-origin-binding-acq-0001");
    assert_eq!(receipt.bundle_byte_len(), 870);
    assert_eq!(receipt.computed_witness_root(), ACQUISITION_BINDING_ROOT);
    assert_eq!(receipt.occurrences().len(), 1);
    assert_eq!(
        receipt.occurrences()[0].identity.bundle_kind,
        binding.bundle_kind()
    );
    assert_eq!(
        receipt.occurrences()[0].identity.witness_root,
        binding.witness_root()
    );
    assert_eq!(receipt.occurrences()[0].sequence, 5);
    assert_eq!(
        receipt.acquisition_receipt().artifact_digest(),
        receipt.contribution().artifact_digest()
    );
    assert_eq!(receipt.acquisition_receipt().selector(), &acquisition);
    assert!(receipt.derivation_receipt().is_none());
}

#[test]
fn derivation_vector_matches_and_retains_both_verified_prerequisites() {
    let binding = derivation_binding_selector();
    let acquisition = parent_acquisition_selector();
    let derivation = derivation_selector();
    let snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[binding.clone(), acquisition.clone(), derivation.clone()],
    );
    let trace = outcome(&snapshot, &binding, &derivation_closure());
    assert!(trace.matched_acquisition().is_none());
    let receipt = trace.matched_derivation().unwrap();

    assert_eq!(receipt.family(), OriginBindingFamilyV0::Derivation);
    assert_eq!(receipt.schema(), ORIGIN_BINDING_DERIVATION_SCHEMA_V0);
    assert_eq!(receipt.binding_id(), "origin-binding-der-0001");
    assert_eq!(receipt.binding_selector(), &binding);
    assert_eq!(receipt.acquisition_selector(), &acquisition);
    assert_eq!(receipt.derivation_selector(), Some(&derivation));
    assert_eq!(
        receipt.contribution().source_evidence_id(),
        "evidence-origin-der-0001"
    );
    assert_eq!(receipt.contribution().artifact_digest(), DERIVED_DIGEST);
    assert_eq!(receipt.bundle_byte_len(), 1_144);
    assert_eq!(receipt.computed_witness_root(), DERIVATION_BINDING_ROOT);
    assert_eq!(receipt.occurrences().len(), 1);
    assert_eq!(receipt.occurrences()[0].sequence, 5);
    assert_eq!(
        receipt.acquisition_receipt().artifact_digest(),
        PARENT_DIGEST
    );
    let derivation_receipt = receipt.derivation_receipt().unwrap();
    assert_eq!(derivation_receipt.selector(), &derivation);
    assert_eq!(
        receipt.acquisition_receipt().artifact_digest(),
        derivation_receipt.parent_artifact_digest()
    );
    assert_eq!(
        derivation_receipt.derived_artifact_digest(),
        receipt.contribution().artifact_digest()
    );
}

#[test]
fn closure_availability_and_same_replay_anchor_boundaries_are_exact() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let spec = ReplaySpec::acquisition();
    let anchored = snapshot(&spec, &[binding.clone(), acquisition.clone()]);
    let empty = closure(&[], &[]);
    assert_eq!(
        outcome(&anchored, &binding, &empty),
        OriginBindingContextTraceV0::BindingBundleUnavailable
    );

    let bundle_only = closure(&[], &[(&binding, ACQUISITION_BINDING)]);
    assert_eq!(
        outcome(&snapshot(&spec, &[]), &binding, &bundle_only),
        OriginBindingContextTraceV0::BindingAnchorAbsent
    );
    assert_eq!(
        outcome(&anchored, &binding, &bundle_only),
        OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::BundleUnavailable,
        }
    );

    let der_binding = derivation_binding_selector();
    let parent = parent_acquisition_selector();
    let der_snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[der_binding.clone(), parent.clone()],
    );
    let der_without_derivation = closure(
        &[(PARENT_DIGEST, DERIVATION_PARENT)],
        &[
            (&der_binding, DERIVATION_BINDING),
            (&parent, PARENT_ACQUISITION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(&der_snapshot, &der_binding, &der_without_derivation),
        OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::BundleUnavailable,
        }
    );
}

#[test]
fn parser_validity_and_duplicate_precedence_are_closed() {
    let mut too_large = vec![0xff; MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0 + 1];
    assert_eq!(
        binding_outcome_for_bytes(&too_large),
        OriginBindingContextTraceV0::BindingBundleTooLarge
    );
    too_large.clear();
    assert_eq!(
        binding_outcome_for_bytes(&[0xff]),
        OriginBindingContextTraceV0::InvalidUtf8
    );
    let mut bom = vec![0xef, 0xbb, 0xbf];
    bom.extend_from_slice(ACQUISITION_BINDING);
    assert_eq!(
        binding_outcome_for_bytes(&bom),
        OriginBindingContextTraceV0::Utf8BomPresent
    );
    assert_eq!(
        binding_outcome_for_bytes(b"{"),
        OriginBindingContextTraceV0::InvalidJson
    );
    assert_eq!(
        binding_outcome_for_bytes(b"{}{}"),
        OriginBindingContextTraceV0::InvalidJson
    );
    assert_eq!(
        binding_outcome_for_bytes(b"[]"),
        OriginBindingContextTraceV0::TopLevelNotObject
    );

    let duplicate_top = replace_once(
        ACQUISITION_BINDING,
        r#"{"schema":"magpie-origin-binding-acquisition-v0","#,
        r#"{"schema":"magpie-origin-binding-acquisition-v0","schema":"magpie-origin-binding-acquisition-v0","#,
    );
    let duplicate_contribution = replace_once(
        ACQUISITION_BINDING,
        r#""target_claim_id":"claim-origin-0001","#,
        r#""target_claim_id":"claim-origin-0001","target_claim_id":"claim-origin-0001","#,
    );
    let duplicate_authority = replace_once(
        ACQUISITION_BINDING,
        r#""authority":{"kind":"human-review","#,
        r#""authority":{"kind":"human-review","kind":"human-review","#,
    );
    let duplicate_unknown = replace_once(
        ACQUISITION_BINDING,
        r#""run_id":"run-origin-binding-acq-0001"}"#,
        r#""run_id":"run-origin-binding-acq-0001","future":{"x":1,"x":2}}"#,
    );
    let duplicate_plus_missing = replace_once(
        &duplicate_top,
        r#""binding_id":"origin-binding-acq-0001","#,
        "",
    );
    let duplicate_wrong_type = replace_once(
        ACQUISITION_BINDING,
        r#""binding_id":"origin-binding-acq-0001""#,
        r#""binding_id":{"x":1,"x":2}"#,
    );
    for bytes in [
        duplicate_top,
        duplicate_contribution,
        duplicate_authority,
        duplicate_unknown,
        duplicate_plus_missing,
        duplicate_wrong_type,
    ] {
        assert_eq!(
            binding_outcome_for_bytes(&bytes),
            OriginBindingContextTraceV0::DuplicateKey
        );
    }
}

#[test]
fn fixed_shape_and_semantic_failures_remain_distinct() {
    let cases = [
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""binding_id":"origin-binding-acq-0001","#,
                "",
            ),
            OriginBindingContextTraceV0::MissingField,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""binding_id":"origin-binding-acq-0001""#,
                r#""binding_id":7"#,
            ),
            OriginBindingContextTraceV0::WrongJsonType,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""schema":"magpie-origin-binding-acquisition-v0""#,
                r#""schema":"magpie-origin-binding-v9""#,
            ),
            OriginBindingContextTraceV0::SchemaMismatch,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""run_id":"run-origin-binding-acq-0001"}"#,
                r#""run_id":"run-origin-binding-acq-0001","trusted":true}"#,
            ),
            OriginBindingContextTraceV0::UnknownField,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""binding_id":"origin-binding-acq-0001""#,
                r#""binding_id":"bad identifier!""#,
            ),
            OriginBindingContextTraceV0::InvalidProtocolIdentifier,
        ),
        (
            replace_once(ACQUISITION_BINDING, "claim-origin-0001", ""),
            OriginBindingContextTraceV0::InvalidReplayReferenceLength,
        ),
        (
            replace_once(ACQUISITION_BINDING, SCOPE, ""),
            OriginBindingContextTraceV0::InvalidReplayReferenceLength,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                r#""artifact":{"algorithm":"sha256""#,
                r#""artifact":{"algorithm":"sha512""#,
            ),
            OriginBindingContextTraceV0::UnsupportedArtifactAlgorithm,
        ),
        (
            replace_once(ACQUISITION_BINDING, ACQUISITION_DIGEST, "ABC"),
            OriginBindingContextTraceV0::InvalidArtifactDigest,
        ),
        (
            replace_once(
                ACQUISITION_BINDING,
                ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
                ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
            ),
            OriginBindingContextTraceV0::InvalidAcquisitionSelector,
        ),
        (
            replace_once(
                DERIVATION_BINDING,
                ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
                ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
            ),
            OriginBindingContextTraceV0::InvalidDerivationSelector,
        ),
    ];

    for (bytes, expected) in cases {
        assert_eq!(binding_outcome_for_bytes(&bytes), expected);
    }
}

#[test]
fn every_valid_but_noncanonical_encoding_is_rejected_before_hashing() {
    let text = std::str::from_utf8(ACQUISITION_BINDING).unwrap();
    let reordered = text.replacen(
        r#"{"schema":"magpie-origin-binding-acquisition-v0","binding_id":"origin-binding-acq-0001""#,
        r#"{"binding_id":"origin-binding-acq-0001","schema":"magpie-origin-binding-acquisition-v0""#,
        1,
    );
    let cases = [
        format!(" {text}"),
        reordered,
        format!("{text}\n"),
        text.replacen("claim-origin-0001", r"claim-origin-\u002d0001", 1),
        text.replacen(SCOPE, r"scope:origin\/example", 1),
        text.replacen("claim-origin-0001", r"claim-origin-\u002D0001", 1),
        text.replacen("claim-origin-0001", r"claim-origin-\ud83d\ude00", 1),
    ];
    for bytes in cases {
        assert_eq!(
            binding_outcome_for_bytes(bytes.as_bytes()),
            OriginBindingContextTraceV0::NonCanonicalEncoding,
            "hostile bytes: {bytes}"
        );
    }
}

#[test]
fn binding_selector_failure_order_precedes_root_and_anchor_lookup() {
    let cases = [
        (
            selector(
                ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
                "0",
                "bad",
                "bad",
                "bad",
            ),
            OriginBindingContextTraceV0::BindingBundleKindMismatch,
        ),
        (
            selector(
                ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                "0",
                "sha512",
                "bad",
                "bad",
            ),
            OriginBindingContextTraceV0::BindingWitnessAlgorithmMismatch,
        ),
        (
            selector(
                ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                "0",
                ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                "jcs",
                "bad",
            ),
            OriginBindingContextTraceV0::BindingCanonicalizationProfileMismatch,
        ),
        (
            selector(
                ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                "0",
                ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                "wrong-run",
            ),
            OriginBindingContextTraceV0::BindingRunIdMismatch,
        ),
        (
            selector(
                ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
                &"0".repeat(64),
                ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
                ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
                "run-origin-binding-acq-0001",
            ),
            OriginBindingContextTraceV0::BindingWitnessRootMismatch,
        ),
    ];

    let snapshot = snapshot(&ReplaySpec::acquisition(), &[]);
    for (selector, expected) in cases {
        let closure = closure(&[], &[(&selector, ACQUISITION_BINDING)]);
        assert_eq!(outcome(&snapshot, &selector, &closure), expected);
    }
    let exact = acquisition_binding_selector();
    let closure = closure(&[], &[(&exact, ACQUISITION_BINDING)]);
    assert_eq!(
        outcome(&snapshot, &exact, &closure),
        OriginBindingContextTraceV0::BindingAnchorAbsent
    );
}

#[test]
fn contribution_revalidation_has_fixed_failure_order() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let closure = acquisition_closure();
    let anchors = [binding.clone(), acquisition];

    let mut target_absent = ReplaySpec::acquisition();
    target_absent.include_claim = false;
    assert_eq!(
        outcome(&snapshot(&target_absent, &anchors), &binding, &closure),
        OriginBindingContextTraceV0::TargetClaimAbsent
    );

    let mut evidence_absent = ReplaySpec::acquisition();
    evidence_absent.include_evidence = false;
    assert_eq!(
        outcome(&snapshot(&evidence_absent, &anchors), &binding, &closure),
        OriginBindingContextTraceV0::SourceEvidenceAbsent
    );

    let mut edge_absent = ReplaySpec::acquisition();
    edge_absent.include_edge = false;
    assert_eq!(
        outcome(&snapshot(&edge_absent, &anchors), &binding, &closure),
        OriginBindingContextTraceV0::JustificationEdgeAbsent
    );

    let mut source_mismatch = ReplaySpec::acquisition();
    source_mismatch.edge_source = "evidence-unrelated".into();
    source_mismatch.extra_evidence_id = Some("evidence-unrelated".into());
    assert_eq!(
        outcome(&snapshot(&source_mismatch, &anchors), &binding, &closure),
        OriginBindingContextTraceV0::EdgeSourceMismatch
    );

    let mut target_mismatch = ReplaySpec::acquisition();
    target_mismatch.edge_target = "claim-unrelated".into();
    target_mismatch.extra_claim_id = Some("claim-unrelated".into());
    assert_eq!(
        outcome(&snapshot(&target_mismatch, &anchors), &binding, &closure),
        OriginBindingContextTraceV0::EdgeTargetMismatch
    );

    for mismatch in [
        {
            let mut value = ReplaySpec::acquisition();
            value.claim_scope = "scope:other".into();
            value
        },
        {
            let mut value = ReplaySpec::acquisition();
            value.evidence_scope = "scope:other".into();
            value
        },
        {
            let mut value = ReplaySpec::acquisition();
            value.edge_scope = "scope:other".into();
            value
        },
    ] {
        assert_eq!(
            outcome(&snapshot(&mismatch, &anchors), &binding, &closure),
            OriginBindingContextTraceV0::ScopeMismatch
        );
    }
}

#[test]
fn individually_existing_but_unrelated_ids_do_not_manufacture_a_contribution() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let mut spec = ReplaySpec::acquisition();
    spec.edge_source = "evidence-unrelated".into();
    spec.edge_target = "claim-unrelated".into();
    spec.extra_evidence_id = Some("evidence-unrelated".into());
    spec.extra_claim_id = Some("claim-unrelated".into());
    assert_eq!(
        outcome(
            &snapshot(&spec, &[binding.clone(), acquisition]),
            &binding,
            &acquisition_closure(),
        ),
        OriginBindingContextTraceV0::EdgeSourceMismatch
    );
}

#[test]
fn acquisition_prerequisite_preserves_exact_existing_trace_variants() {
    let base_text = std::str::from_utf8(ACQUISITION_BINDING).unwrap();
    let hostile = [
        (
            base_text
                .replacen("run-acq-0001", "run-acq-other", 1)
                .into_bytes(),
            selector(
                ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
                ACQUISITION_ROOT,
                ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
                ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
                "run-acq-other",
            ),
            Some(ACQUISITION_ARTIFACT),
            ArtifactProvenanceContextTraceV0::RunIdMismatch,
        ),
        (
            base_text
                .replacen(ACQUISITION_ROOT, &"0".repeat(64), 1)
                .into_bytes(),
            selector(
                ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
                &"0".repeat(64),
                ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
                ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
                "run-acq-0001",
            ),
            Some(ACQUISITION_ARTIFACT),
            ArtifactProvenanceContextTraceV0::WitnessRootMismatch,
        ),
    ];

    for (binding_bytes, nested_selector, artifact, nested_trace) in hostile {
        let binding = selector_for_bundle(
            ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
            "run-origin-binding-acq-0001",
            &binding_bytes,
        );
        let snapshot = snapshot(
            &ReplaySpec::acquisition(),
            &[binding.clone(), nested_selector.clone()],
        );
        let artifacts = artifact
            .map(|bytes| vec![(ACQUISITION_DIGEST, bytes)])
            .unwrap_or_default();
        let closure = closure(
            &artifacts,
            &[
                (&binding, &binding_bytes),
                (&nested_selector, ACQUISITION_BUNDLE),
            ],
        );
        assert_eq!(
            outcome(&snapshot, &binding, &closure),
            OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
                trace: nested_trace,
            }
        );
    }

    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let cases = [
        (
            snapshot(&ReplaySpec::acquisition(), std::slice::from_ref(&binding)),
            closure(
                &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
                &[
                    (&binding, ACQUISITION_BINDING),
                    (&acquisition, ACQUISITION_BUNDLE),
                ],
            ),
            ArtifactProvenanceContextTraceV0::AnchorAbsent,
        ),
        (
            snapshot(
                &ReplaySpec::acquisition(),
                &[binding.clone(), acquisition.clone()],
            ),
            closure(
                &[],
                &[
                    (&binding, ACQUISITION_BINDING),
                    (&acquisition, ACQUISITION_BUNDLE),
                ],
            ),
            ArtifactProvenanceContextTraceV0::ArtifactUnavailable,
        ),
        (
            snapshot(
                &ReplaySpec::acquisition(),
                &[binding.clone(), acquisition.clone()],
            ),
            closure(
                &[(ACQUISITION_DIGEST, b"wrong artifact")],
                &[
                    (&binding, ACQUISITION_BINDING),
                    (&acquisition, ACQUISITION_BUNDLE),
                ],
            ),
            ArtifactProvenanceContextTraceV0::ArtifactDigestMismatch,
        ),
    ];
    for (snapshot, closure, nested_trace) in cases {
        assert_eq!(
            outcome(&snapshot, &binding, &closure),
            OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
                trace: nested_trace,
            }
        );
    }
}

#[test]
fn derivation_prerequisite_preserves_exact_existing_trace_variants() {
    let base_text = std::str::from_utf8(DERIVATION_BINDING).unwrap();
    let hostile = [
        (
            base_text
                .replacen("run-der-0001", "run-der-other", 1)
                .into_bytes(),
            selector(
                ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
                DERIVATION_ROOT,
                ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
                ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
                "run-der-other",
            ),
            ArtifactProvenanceContextTraceV0::RunIdMismatch,
        ),
        (
            base_text
                .replacen(DERIVATION_ROOT, &"0".repeat(64), 1)
                .into_bytes(),
            selector(
                ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
                &"0".repeat(64),
                ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
                ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
                "run-der-0001",
            ),
            ArtifactProvenanceContextTraceV0::WitnessRootMismatch,
        ),
    ];
    for (binding_bytes, nested_derivation, nested_trace) in hostile {
        let binding = selector_for_bundle(
            ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
            "run-origin-binding-der-0001",
            &binding_bytes,
        );
        let acquisition = parent_acquisition_selector();
        let snapshot = snapshot(
            &ReplaySpec::derivation(),
            &[
                binding.clone(),
                acquisition.clone(),
                nested_derivation.clone(),
            ],
        );
        let closure = closure(
            &[
                (PARENT_DIGEST, DERIVATION_PARENT),
                (DERIVED_DIGEST, DERIVATION_DERIVED),
            ],
            &[
                (&binding, &binding_bytes),
                (&acquisition, PARENT_ACQUISITION_BUNDLE),
                (&nested_derivation, DERIVATION_BUNDLE),
            ],
        );
        assert_eq!(
            outcome(&snapshot, &binding, &closure),
            OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
                trace: nested_trace,
            }
        );
    }

    let binding = derivation_binding_selector();
    let acquisition = parent_acquisition_selector();
    let derivation = derivation_selector();
    let unanchored_derivation = closure(
        &[
            (PARENT_DIGEST, DERIVATION_PARENT),
            (DERIVED_DIGEST, DERIVATION_DERIVED),
        ],
        &[
            (&binding, DERIVATION_BINDING),
            (&acquisition, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(
            &snapshot(
                &ReplaySpec::derivation(),
                &[binding.clone(), acquisition.clone()],
            ),
            &binding,
            &unanchored_derivation,
        ),
        OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::AnchorAbsent,
        }
    );

    let cases = [
        (
            snapshot(
                &ReplaySpec::derivation(),
                &[binding.clone(), acquisition.clone(), derivation.clone()],
            ),
            closure(
                &[(DERIVED_DIGEST, DERIVATION_DERIVED)],
                &[
                    (&binding, DERIVATION_BINDING),
                    (&acquisition, PARENT_ACQUISITION_BUNDLE),
                    (&derivation, DERIVATION_BUNDLE),
                ],
            ),
            ArtifactProvenanceContextTraceV0::ArtifactUnavailable,
        ),
        (
            snapshot(
                &ReplaySpec::derivation(),
                &[binding.clone(), acquisition.clone(), derivation.clone()],
            ),
            closure(
                &[
                    (PARENT_DIGEST, b"wrong parent"),
                    (DERIVED_DIGEST, DERIVATION_DERIVED),
                ],
                &[
                    (&binding, DERIVATION_BINDING),
                    (&acquisition, PARENT_ACQUISITION_BUNDLE),
                    (&derivation, DERIVATION_BUNDLE),
                ],
            ),
            ArtifactProvenanceContextTraceV0::ArtifactDigestMismatch,
        ),
    ];
    for (snapshot, closure, nested_trace) in cases {
        assert_eq!(
            outcome(&snapshot, &binding, &closure),
            OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
                trace: nested_trace,
            }
        );
    }

    let snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[binding.clone(), acquisition.clone(), derivation.clone()],
    );
    let missing_parent = closure(
        &[(DERIVED_DIGEST, DERIVATION_DERIVED)],
        &[
            (&binding, DERIVATION_BINDING),
            (&acquisition, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    // Acquisition resolves first, so make its exact parent bytes available before
    // asserting the derivation-specific missing-parent trace.
    assert_eq!(
        outcome(&snapshot, &binding, &missing_parent),
        OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::ArtifactUnavailable,
        }
    );
    let missing_derived = closure(
        &[(PARENT_DIGEST, DERIVATION_PARENT)],
        &[
            (&binding, DERIVATION_BINDING),
            (&acquisition, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(&snapshot, &binding, &missing_derived),
        OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::DerivedArtifactUnavailable,
        }
    );
    let wrong_derived = closure(
        &[
            (PARENT_DIGEST, DERIVATION_PARENT),
            (DERIVED_DIGEST, b"wrong derived"),
        ],
        &[
            (&binding, DERIVATION_BINDING),
            (&acquisition, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(&snapshot, &binding, &wrong_derived),
        OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::DerivedArtifactDigestMismatch,
        }
    );
}

#[test]
fn cross_statement_artifact_coherence_failures_follow_successful_prerequisites() {
    let acquisition_binding_bytes =
        replace_once(ACQUISITION_BINDING, ACQUISITION_DIGEST, DERIVED_DIGEST);
    let acquisition_binding = selector_for_bundle(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        "run-origin-binding-acq-0001",
        &acquisition_binding_bytes,
    );
    let acquisition = acquisition_selector();
    let acquisition_snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[acquisition_binding.clone(), acquisition.clone()],
    );
    assert!(acquisition_snapshot
        .resolve_artifact_acquisition_context_v0(
            &acquisition,
            Some(ACQUISITION_BUNDLE),
            Some(ACQUISITION_ARTIFACT),
        )
        .matched_acquisition()
        .is_some());
    let acquisition_mismatch = closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[
            (&acquisition_binding, &acquisition_binding_bytes),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(
            &acquisition_snapshot,
            &acquisition_binding,
            &acquisition_mismatch,
        ),
        OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromContributionArtifact
    );

    let acquisition_parent_mismatch_bytes = std::str::from_utf8(DERIVATION_BINDING)
        .unwrap()
        .replacen(PARENT_ACQUISITION_ROOT, ACQUISITION_ROOT, 1)
        .replacen("run-acq-alpha-0001", "run-acq-0001", 1)
        .into_bytes();
    let acquisition_parent_binding = selector_for_bundle(
        ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        "run-origin-binding-der-0001",
        &acquisition_parent_mismatch_bytes,
    );
    let derivation = derivation_selector();
    let parent_mismatch_snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[
            acquisition_parent_binding.clone(),
            acquisition.clone(),
            derivation.clone(),
        ],
    );
    assert!(parent_mismatch_snapshot
        .resolve_artifact_acquisition_context_v0(
            &acquisition,
            Some(ACQUISITION_BUNDLE),
            Some(ACQUISITION_ARTIFACT),
        )
        .matched_acquisition()
        .is_some());
    assert!(parent_mismatch_snapshot
        .resolve_artifact_derivation_context_v0(
            &derivation,
            Some(DERIVATION_BUNDLE),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        )
        .matched_derivation()
        .is_some());
    let parent_mismatch = closure(
        &[
            (ACQUISITION_DIGEST, ACQUISITION_ARTIFACT),
            (PARENT_DIGEST, DERIVATION_PARENT),
            (DERIVED_DIGEST, DERIVATION_DERIVED),
        ],
        &[
            (
                &acquisition_parent_binding,
                &acquisition_parent_mismatch_bytes,
            ),
            (&acquisition, ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(
            &parent_mismatch_snapshot,
            &acquisition_parent_binding,
            &parent_mismatch,
        ),
        OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromDerivationParent
    );

    let output_mismatch_bytes =
        replace_once(DERIVATION_BINDING, DERIVED_DIGEST, ACQUISITION_DIGEST);
    let output_binding = selector_for_bundle(
        ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        "run-origin-binding-der-0001",
        &output_mismatch_bytes,
    );
    let parent = parent_acquisition_selector();
    let output_snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[output_binding.clone(), parent.clone(), derivation.clone()],
    );
    assert!(output_snapshot
        .resolve_artifact_acquisition_context_v0(
            &parent,
            Some(PARENT_ACQUISITION_BUNDLE),
            Some(DERIVATION_PARENT),
        )
        .matched_acquisition()
        .is_some());
    assert!(output_snapshot
        .resolve_artifact_derivation_context_v0(
            &derivation,
            Some(DERIVATION_BUNDLE),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        )
        .matched_derivation()
        .is_some());
    let output_mismatch = closure(
        &[
            (PARENT_DIGEST, DERIVATION_PARENT),
            (DERIVED_DIGEST, DERIVATION_DERIVED),
        ],
        &[
            (&output_binding, &output_mismatch_bytes),
            (&parent, PARENT_ACQUISITION_BUNDLE),
            (&derivation, DERIVATION_BUNDLE),
        ],
    );
    assert_eq!(
        outcome(&output_snapshot, &output_binding, &output_mismatch),
        OriginBindingContextTraceV0::DerivationOutputDiffersFromContributionArtifact
    );
}

#[test]
fn multiplicity_determinism_and_closure_order_do_not_amplify_a_match() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[
            binding.clone(),
            acquisition.clone(),
            binding.clone(),
            binding.clone(),
        ],
    );
    let closure_a = closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[
            (&binding, ACQUISITION_BINDING),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    );
    let closure_b = closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[
            (&acquisition, ACQUISITION_BUNDLE),
            (&binding, ACQUISITION_BINDING),
        ],
    );
    let first = outcome(&snapshot, &binding, &closure_a);
    let second = outcome(&snapshot, &binding, &closure_a);
    let reordered = outcome(&snapshot, &binding, &closure_b);
    assert_eq!(first, second);
    assert_eq!(first, reordered);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
    assert_eq!(first.canonical_bytes(), reordered.canonical_bytes());
    let receipt = first.matched_acquisition().unwrap();
    assert_eq!(receipt.occurrences().len(), 3);
    assert_eq!(
        receipt
            .occurrences()
            .iter()
            .map(|occurrence| occurrence.sequence)
            .collect::<Vec<_>>(),
        vec![5, 7, 8]
    );
    assert_eq!(
        second.matched_acquisition(),
        reordered.matched_acquisition()
    );
}

#[test]
fn cloned_or_serialized_receipt_never_substitutes_for_replay_and_closure() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let matched_snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[binding.clone(), acquisition.clone()],
    );
    let full = acquisition_closure();
    let trace = outcome(&matched_snapshot, &binding, &full);
    let receipt = trace.matched_acquisition().unwrap().clone();
    assert!(!serde_json::to_vec(&receipt).unwrap().is_empty());

    assert_eq!(
        outcome(&snapshot(&ReplaySpec::acquisition(), &[]), &binding, &full),
        OriginBindingContextTraceV0::BindingAnchorAbsent
    );
    assert_eq!(
        outcome(&matched_snapshot, &binding, &closure(&[], &[])),
        OriginBindingContextTraceV0::BindingBundleUnavailable
    );
    let binding_only = closure(&[], &[(&binding, ACQUISITION_BINDING)]);
    assert_eq!(
        outcome(&matched_snapshot, &binding, &binding_only),
        OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
            trace: ArtifactProvenanceContextTraceV0::BundleUnavailable,
        }
    );
}

#[test]
fn origin_binding_resolution_is_byte_for_byte_standing_inert() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let snapshot = snapshot(&ReplaySpec::acquisition(), &[binding.clone(), acquisition]);
    let closure = acquisition_closure();
    let snapshot_before = snapshot.canonical_bytes();
    let view_before = snapshot.standing().canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace("claim-origin-0001");
    let v1_before = snapshot.resolved_standing_with_trace_v1("claim-origin-0001");
    let v2_before = snapshot.resolved_standing_with_trace_v2("claim-origin-0001");
    let v0_bytes = v0_before.canonical_bytes();
    let v1_bytes = v1_before.canonical_bytes();
    let v2_bytes = v2_before.canonical_bytes();

    for _ in 0..3 {
        assert!(outcome(&snapshot, &binding, &closure)
            .matched_acquisition()
            .is_some());
    }

    let v0_after = snapshot
        .standing()
        .resolved_standing_with_trace("claim-origin-0001");
    let v1_after = snapshot.resolved_standing_with_trace_v1("claim-origin-0001");
    let v2_after = snapshot.resolved_standing_with_trace_v2("claim-origin-0001");
    assert_eq!(snapshot_before, snapshot.canonical_bytes());
    assert_eq!(view_before, snapshot.standing().canonical_bytes());
    assert_eq!(v0_bytes, v0_after.canonical_bytes());
    assert_eq!(v1_bytes, v1_after.canonical_bytes());
    assert_eq!(v2_bytes, v2_after.canonical_bytes());
    assert_eq!(v0_before.governed_standing, v0_after.governed_standing);
    assert_eq!(v0_before.legacy_raw_standing, v0_after.legacy_raw_standing);
    assert_eq!(v0_before.blockers, v0_after.blockers);
    assert_eq!(v0_before.trace, v0_after.trace);
    assert_eq!(v1_before.governed_standing, v1_after.governed_standing);
    assert_eq!(v1_before.legacy_raw_standing, v1_after.legacy_raw_standing);
    assert_eq!(v1_before.blockers, v1_after.blockers);
    assert_eq!(v1_before.trace, v1_after.trace);
    assert_eq!(v2_before.governed_standing, v2_after.governed_standing);
    assert_eq!(v2_before.legacy_raw_standing, v2_after.legacy_raw_standing);
    assert_eq!(v2_before.blockers, v2_after.blockers);
    assert_eq!(v2_before.trace, v2_after.trace);
    assert_eq!(v0_after.governed_standing, Some(Status::Conjectured));
    assert_eq!(v0_after.legacy_raw_standing, Some(Status::Conjectured));
}

#[test]
fn fixture_bytes_and_roots_are_exact_portable_untrusted_material() {
    let fixtures = [
        (ACQUISITION_BINDING, 870, ACQUISITION_BINDING_ROOT),
        (DERIVATION_BINDING, 1_144, DERIVATION_BINDING_ROOT),
        (PARENT_ACQUISITION_BUNDLE, 294, PARENT_ACQUISITION_ROOT),
    ];
    for (bytes, length, root) in fixtures {
        assert_eq!(bytes.len(), length);
        assert_eq!(sha256_hex(bytes), root);
        assert_eq!(bytes.first(), Some(&b'{'));
        assert_eq!(bytes.last(), Some(&b'}'));
        assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
        assert!(!bytes.ends_with(b"\n"));
        assert!(!bytes.ends_with(b"\r"));
    }
}

#[test]
fn public_constants_and_every_unmatched_trace_tag_are_exact() {
    assert_eq!(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        "magpie-origin-binding-acquisition-v0"
    );
    assert_eq!(
        ORIGIN_BINDING_ACQUISITION_SCHEMA_V0,
        "magpie-origin-binding-acquisition-v0"
    );
    assert_eq!(
        ORIGIN_BINDING_DERIVATION_BUNDLE_KIND_V0,
        "magpie-origin-binding-derivation-v0"
    );
    assert_eq!(
        ORIGIN_BINDING_DERIVATION_SCHEMA_V0,
        "magpie-origin-binding-derivation-v0"
    );
    assert_eq!(
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        "magpie-origin-binding-json-v0"
    );
    assert_eq!(
        ORIGIN_BINDING_VERIFIER_PROFILE_V0,
        "magpie-origin-binding-verifier-v0"
    );
    assert_eq!(ORIGIN_BINDING_WITNESS_ALGORITHM_V0, "sha256");
    assert_eq!(ORIGIN_BINDING_ARTIFACT_ALGORITHM_V0, "sha256");
    assert_eq!(MAX_ORIGIN_BINDING_BUNDLE_BYTES_V0, 16_384);

    let traces = [
        (
            OriginBindingContextTraceV0::BindingBundleUnavailable,
            "binding_bundle_unavailable",
        ),
        (
            OriginBindingContextTraceV0::BindingBundleTooLarge,
            "binding_bundle_too_large",
        ),
        (OriginBindingContextTraceV0::InvalidUtf8, "invalid_utf8"),
        (
            OriginBindingContextTraceV0::Utf8BomPresent,
            "utf8_bom_present",
        ),
        (OriginBindingContextTraceV0::InvalidJson, "invalid_json"),
        (
            OriginBindingContextTraceV0::TopLevelNotObject,
            "top_level_not_object",
        ),
        (OriginBindingContextTraceV0::DuplicateKey, "duplicate_key"),
        (OriginBindingContextTraceV0::MissingField, "missing_field"),
        (OriginBindingContextTraceV0::UnknownField, "unknown_field"),
        (
            OriginBindingContextTraceV0::WrongJsonType,
            "wrong_json_type",
        ),
        (
            OriginBindingContextTraceV0::SchemaMismatch,
            "schema_mismatch",
        ),
        (
            OriginBindingContextTraceV0::InvalidProtocolIdentifier,
            "invalid_protocol_identifier",
        ),
        (
            OriginBindingContextTraceV0::InvalidReplayReferenceLength,
            "invalid_replay_reference_length",
        ),
        (
            OriginBindingContextTraceV0::UnsupportedArtifactAlgorithm,
            "unsupported_artifact_algorithm",
        ),
        (
            OriginBindingContextTraceV0::InvalidArtifactDigest,
            "invalid_artifact_digest",
        ),
        (
            OriginBindingContextTraceV0::InvalidAcquisitionSelector,
            "invalid_acquisition_selector",
        ),
        (
            OriginBindingContextTraceV0::InvalidDerivationSelector,
            "invalid_derivation_selector",
        ),
        (
            OriginBindingContextTraceV0::NonCanonicalEncoding,
            "non_canonical_encoding",
        ),
        (
            OriginBindingContextTraceV0::BindingBundleKindMismatch,
            "binding_bundle_kind_mismatch",
        ),
        (
            OriginBindingContextTraceV0::BindingWitnessAlgorithmMismatch,
            "binding_witness_algorithm_mismatch",
        ),
        (
            OriginBindingContextTraceV0::BindingCanonicalizationProfileMismatch,
            "binding_canonicalization_profile_mismatch",
        ),
        (
            OriginBindingContextTraceV0::BindingRunIdMismatch,
            "binding_run_id_mismatch",
        ),
        (
            OriginBindingContextTraceV0::BindingWitnessRootMismatch,
            "binding_witness_root_mismatch",
        ),
        (
            OriginBindingContextTraceV0::BindingAnchorAbsent,
            "binding_anchor_absent",
        ),
        (
            OriginBindingContextTraceV0::TargetClaimAbsent,
            "target_claim_absent",
        ),
        (
            OriginBindingContextTraceV0::SourceEvidenceAbsent,
            "source_evidence_absent",
        ),
        (
            OriginBindingContextTraceV0::JustificationEdgeAbsent,
            "justification_edge_absent",
        ),
        (
            OriginBindingContextTraceV0::EdgeSourceMismatch,
            "edge_source_mismatch",
        ),
        (
            OriginBindingContextTraceV0::EdgeTargetMismatch,
            "edge_target_mismatch",
        ),
        (OriginBindingContextTraceV0::ScopeMismatch, "scope_mismatch"),
        (
            OriginBindingContextTraceV0::AcquisitionPrerequisiteUnresolved {
                trace: ArtifactProvenanceContextTraceV0::BundleUnavailable,
            },
            "acquisition_prerequisite_unresolved",
        ),
        (
            OriginBindingContextTraceV0::DerivationPrerequisiteUnresolved {
                trace: ArtifactProvenanceContextTraceV0::BundleUnavailable,
            },
            "derivation_prerequisite_unresolved",
        ),
        (
            OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromContributionArtifact,
            "acquisition_artifact_differs_from_contribution_artifact",
        ),
        (
            OriginBindingContextTraceV0::AcquisitionArtifactDiffersFromDerivationParent,
            "acquisition_artifact_differs_from_derivation_parent",
        ),
        (
            OriginBindingContextTraceV0::DerivationOutputDiffersFromContributionArtifact,
            "derivation_output_differs_from_contribution_artifact",
        ),
    ];

    for (trace, expected_tag) in traces {
        let bytes = trace.canonical_bytes();
        assert_eq!(bytes, trace.canonical_bytes());
        assert!(!bytes.contains(&b' '));
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["outcome"], expected_tag);
    }
}

#[test]
fn public_trace_bytes_remain_pinned_across_private_evaluator_refactor() {
    let binding = acquisition_binding_selector();
    let acquisition = acquisition_selector();
    let acquisition_snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[binding.clone(), acquisition.clone()],
    );
    let derivation_binding = derivation_binding_selector();
    let parent_acquisition = parent_acquisition_selector();
    let derivation = derivation_selector();
    let derivation_snapshot = snapshot(
        &ReplaySpec::derivation(),
        &[
            derivation_binding.clone(),
            parent_acquisition.clone(),
            derivation.clone(),
        ],
    );

    let wrong_root_selector = selector(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        &"0".repeat(64),
        ORIGIN_BINDING_WITNESS_ALGORITHM_V0,
        ORIGIN_BINDING_CANONICALIZATION_PROFILE_V0,
        "run-origin-binding-acq-0001",
    );
    let mut target_absent = ReplaySpec::acquisition();
    target_absent.include_claim = false;
    let binding_only = closure(&[], &[(&binding, ACQUISITION_BINDING)]);
    let acquisition_bundle_only = closure(
        &[],
        &[
            (&binding, ACQUISITION_BINDING),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    );
    let malformed_nested = closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[(&binding, ACQUISITION_BINDING), (&acquisition, b"{")],
    );
    let digest_mismatch = closure(
        &[(ACQUISITION_DIGEST, b"wrong artifact")],
        &[
            (&binding, ACQUISITION_BINDING),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    );
    let coherence_binding_bytes =
        replace_once(ACQUISITION_BINDING, ACQUISITION_DIGEST, DERIVED_DIGEST);
    let coherence_binding = selector_for_bundle(
        ORIGIN_BINDING_ACQUISITION_BUNDLE_KIND_V0,
        "run-origin-binding-acq-0001",
        &coherence_binding_bytes,
    );
    let coherence_snapshot = snapshot(
        &ReplaySpec::acquisition(),
        &[coherence_binding.clone(), acquisition.clone()],
    );
    let coherence_closure = closure(
        &[(ACQUISITION_DIGEST, ACQUISITION_ARTIFACT)],
        &[
            (&coherence_binding, &coherence_binding_bytes),
            (&acquisition, ACQUISITION_BUNDLE),
        ],
    );

    let traces = [
        (
            "matched_acquisition",
            outcome(&acquisition_snapshot, &binding, &acquisition_closure()),
            3365,
            "7276c9d33671123e3dfb53a88ffb8ac9bc6c572a5ca500de9f9151052657d178",
        ),
        (
            "matched_derivation",
            outcome(
                &derivation_snapshot,
                &derivation_binding,
                &derivation_closure(),
            ),
            5184,
            "9b5907b9349100618ffe969e6fb7a9a6bae5feb89e2167691133ead778058492",
        ),
        (
            "binding_bundle_unavailable",
            outcome(&acquisition_snapshot, &binding, &closure(&[], &[])),
            40,
            "0cb46ca223e0ca6384a844a9a62b3cdf7b1a24346df57c19b6f20241bb2b5aba",
        ),
        (
            "malformed_binding",
            outcome(
                &acquisition_snapshot,
                &binding,
                &closure(&[], &[(&binding, b"{")]),
            ),
            26,
            "3967aa23f8172bce1d40a4508e6f10f800b0163d1e2788223c6b35951f50abe6",
        ),
        (
            "binding_root_mismatch",
            outcome(
                &snapshot(&ReplaySpec::acquisition(), &[]),
                &wrong_root_selector,
                &closure(&[], &[(&wrong_root_selector, ACQUISITION_BINDING)]),
            ),
            43,
            "a3a75ac579aecf7686c0b2ca527d2b60e90be21fbf9204dd2c1ffd08eb92d05e",
        ),
        (
            "replay_structure_mismatch",
            outcome(
                &snapshot(&target_absent, &[binding.clone(), acquisition.clone()]),
                &binding,
                &acquisition_closure(),
            ),
            33,
            "a56dbea879ce8100f5a6b46c35b6df94c1b10d927f909a79cd90f699d33971c4",
        ),
        (
            "nested_acquisition_bundle_unavailable",
            outcome(&acquisition_snapshot, &binding, &binding_only),
            102,
            "2d7091908057c02ddc393a831fd8d64d3008d1b6069b9be56c280e85d62606f1",
        ),
        (
            "acquisition_artifact_unavailable",
            outcome(&acquisition_snapshot, &binding, &acquisition_bundle_only),
            104,
            "d839b29d487a7632da555632ebeb332514576ce4069dd5c8a4591177fdd8c70e",
        ),
        (
            "nested_malformed_provenance",
            outcome(&acquisition_snapshot, &binding, &malformed_nested),
            96,
            "ac66a8184b8cfa88280c528d8e142d679b89d9d4fd4e7feec3e53b4898baa4c9",
        ),
        (
            "artifact_digest_mismatch",
            outcome(&acquisition_snapshot, &binding, &digest_mismatch),
            108,
            "e186eb56197666c9c53ff2e213103645bd308f614bdde94986cb1088df425208",
        ),
        (
            "artifact_coherence_failure",
            outcome(&coherence_snapshot, &coherence_binding, &coherence_closure),
            69,
            "6f06897a73fe5818e57c5dd4ec40970158a62bc6dd2c5e175f2d72363428a191",
        ),
    ];

    for (name, trace, expected_len, expected_sha256) in traces {
        let bytes = trace.canonical_bytes();
        assert_eq!(
            bytes.len(),
            expected_len,
            "{name} public trace length changed"
        );
        assert_eq!(
            sha256_hex(&bytes),
            expected_sha256,
            "{name} public trace bytes changed"
        );
    }
}
