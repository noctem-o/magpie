use magpie_claims::{
    replay_standing_context, ArtifactProvenanceAnchorSelectorV0, ArtifactProvenanceContextTraceV0,
    StandingReplaySnapshot, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0, ARTIFACT_ACQUISITION_SCHEMA_V0,
    ARTIFACT_DERIVATION_BUNDLE_KIND_V0, ARTIFACT_DERIVATION_SCHEMA_V0,
    ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0, ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
    ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0,
};
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, SigningKey, Status};
use sha2::{Digest, Sha256};

const ACQUISITION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-bundle.json");
const ACQUISITION_ARTIFACT: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/acquisition-artifact.txt");
const DERIVATION_BUNDLE: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-bundle.json");
const DERIVATION_PARENT: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-parent.txt");
const DERIVATION_DERIVED: &[u8] =
    include_bytes!("../../../fixtures/artifact-provenance-v0/derivation-derived.txt");
const FIXTURE_README: &[u8] = include_bytes!("../../../fixtures/artifact-provenance-v0/README.md");

const ACQUISITION_ROOT: &str = "4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e";
const ACQUISITION_DIGEST: &str = "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076";
const PARENT_DIGEST: &str = "b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060";
const DERIVED_DIGEST: &str = "1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005";
const DERIVATION_ROOT: &str = "472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd";
const SEED: [u8; 32] = [39; 32];
const INERT_CLAIM_ID: &str = "claim-artifact-context-inert";

const FORBIDDEN_MACHINE_MARKERS: [&str; 10] = [
    concat!("C:", "\\"),
    concat!("/ho", "me/"),
    concat!("/m", "nt/"),
    concat!("Tail", "scale"),
    concat!("Hypr", "land"),
    concat!("Qw", "en"),
    concat!("Ry", "zen"),
    concat!("40", "90"),
    concat!("last", "garlean"),
    concat!("noctem", "-o"),
];

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn selector(
    bundle_kind: &str,
    witness_root: &str,
    witness_algorithm: &str,
    canonicalization_profile: &str,
    run_id: &str,
) -> ArtifactProvenanceAnchorSelectorV0 {
    ArtifactProvenanceAnchorSelectorV0::new(
        bundle_kind,
        witness_root,
        witness_algorithm,
        canonicalization_profile,
        run_id,
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

fn derivation_selector() -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
        DERIVATION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-der-0001",
    )
}

fn selector_for_bundle(
    bundle_kind: &str,
    run_id: &str,
    bundle: &[u8],
) -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        bundle_kind,
        &sha256_hex(bundle),
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        run_id,
    )
}

fn snapshot_with_anchors(
    selectors: &[ArtifactProvenanceAnchorSelectorV0],
) -> StandingReplaySnapshot {
    let store = MemStore::new();
    {
        let mut timestamp = 1000;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            signing_key(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        for anchor in selectors {
            writer
                .append(
                    Provenance::new("artifact-fixture", "standing-inert-verifier-v0"),
                    Payload::SegmentAnchored {
                        bundle_kind: anchor.bundle_kind().to_owned(),
                        witness_root: anchor.witness_root().to_owned(),
                        witness_algorithm: anchor.witness_algorithm().to_owned(),
                        canonicalization_profile: anchor.canonicalization_profile().to_owned(),
                        run_id: anchor.run_id().to_owned(),
                    },
                )
                .unwrap();
        }
        writer
            .append(
                Provenance::new("artifact-fixture", "standing-inert-control"),
                Payload::ClaimAsserted {
                    claim_id: INERT_CLAIM_ID.into(),
                    statement: "artifact context must not change standing".into(),
                    status: Status::Conjectured,
                },
            )
            .unwrap();
        writer
            .append(
                Provenance::new("artifact-fixture", "standing-inert-control"),
                Payload::ClaimAssertedV2 {
                    claim_id: INERT_CLAIM_ID.into(),
                    statement: "artifact context must not change standing".into(),
                    scope_ref: "scope:artifact-context".into(),
                    actor_class: "AgentProposer".into(),
                    content_hash: String::new(),
                    metadata_json: r#"{"claim_domain":"Interpretation"}"#.into(),
                },
            )
            .unwrap();
    }
    let reader = LogReader::open(store, signing_key().verifying_key());
    replay_standing_context(&reader).unwrap()
}

fn empty_snapshot() -> StandingReplaySnapshot {
    snapshot_with_anchors(&[])
}

fn acquisition_text() -> String {
    String::from_utf8(ACQUISITION_BUNDLE.to_vec()).unwrap()
}

fn derivation_text() -> String {
    String::from_utf8(DERIVATION_BUNDLE.to_vec()).unwrap()
}

fn acquisition_outcome(
    snapshot: &StandingReplaySnapshot,
    selector: &ArtifactProvenanceAnchorSelectorV0,
    bundle: Option<&[u8]>,
    artifact: Option<&[u8]>,
) -> ArtifactProvenanceContextTraceV0 {
    snapshot.resolve_artifact_acquisition_context_v0(selector, bundle, artifact)
}

fn derivation_outcome(
    snapshot: &StandingReplaySnapshot,
    selector: &ArtifactProvenanceAnchorSelectorV0,
    bundle: Option<&[u8]>,
    parent: Option<&[u8]>,
    derived: Option<&[u8]>,
) -> ArtifactProvenanceContextTraceV0 {
    snapshot.resolve_artifact_derivation_context_v0(selector, bundle, parent, derived)
}

#[test]
fn normative_acquisition_fixture_produces_exact_receipt() {
    let selector = acquisition_selector();
    let snapshot = snapshot_with_anchors(std::slice::from_ref(&selector));
    let trace = acquisition_outcome(
        &snapshot,
        &selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    );
    assert!(trace.matched_derivation().is_none());
    let receipt = trace.matched_acquisition().unwrap();

    assert_eq!(
        receipt.verifier_profile(),
        ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0
    );
    assert_eq!(receipt.selector(), &selector);
    assert_eq!(
        receipt.selector().bundle_kind(),
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0
    );
    assert_eq!(receipt.selector().witness_root(), ACQUISITION_ROOT);
    assert_eq!(
        receipt.selector().witness_algorithm(),
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0
    );
    assert_eq!(
        receipt.selector().canonicalization_profile(),
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(receipt.selector().run_id(), "run-acq-0001");
    assert_eq!(receipt.schema(), ARTIFACT_ACQUISITION_SCHEMA_V0);
    assert_eq!(receipt.acquisition_id(), "acq-0001");
    assert_eq!(receipt.acquisition_profile(), "magpie-import-v0");
    assert_eq!(
        receipt.artifact_algorithm(),
        ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0
    );
    assert_eq!(receipt.artifact_digest(), ACQUISITION_DIGEST);
    assert_eq!(receipt.observed_locator(), "file:hello-artifact.txt");
    assert_eq!(receipt.bundle_byte_len(), 291);
    assert_eq!(receipt.computed_witness_root(), ACQUISITION_ROOT);
    assert_eq!(receipt.artifact_byte_len(), 15);
    assert_eq!(receipt.computed_artifact_sha256(), ACQUISITION_DIGEST);
    assert_eq!(receipt.occurrences().len(), 1);
    assert_eq!(receipt.occurrences()[0].sequence, 1);
    assert_eq!(
        receipt.occurrences()[0].identity.bundle_kind,
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0
    );
    assert_eq!(
        receipt.occurrences()[0].identity.witness_root,
        ACQUISITION_ROOT
    );
    assert_eq!(
        receipt.occurrences()[0].provenance_agent,
        "artifact-fixture"
    );
    assert_eq!(
        receipt.occurrences()[0].provenance_source,
        "standing-inert-verifier-v0"
    );
}

#[test]
fn normative_derivation_fixture_produces_exact_receipt() {
    let selector = derivation_selector();
    let snapshot = snapshot_with_anchors(std::slice::from_ref(&selector));
    let trace = derivation_outcome(
        &snapshot,
        &selector,
        Some(DERIVATION_BUNDLE),
        Some(DERIVATION_PARENT),
        Some(DERIVATION_DERIVED),
    );
    assert!(trace.matched_acquisition().is_none());
    let receipt = trace.matched_derivation().unwrap();

    assert_eq!(
        receipt.verifier_profile(),
        ARTIFACT_PROVENANCE_VERIFIER_PROFILE_V0
    );
    assert_eq!(receipt.selector(), &selector);
    assert_eq!(
        receipt.selector().bundle_kind(),
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0
    );
    assert_eq!(receipt.selector().witness_root(), DERIVATION_ROOT);
    assert_eq!(receipt.selector().run_id(), "run-der-0001");
    assert_eq!(receipt.schema(), ARTIFACT_DERIVATION_SCHEMA_V0);
    assert_eq!(receipt.derivation_id(), "der-0001");
    assert_eq!(receipt.transformation_profile(), "uppercase-ascii-v0");
    assert_eq!(
        receipt.parent_artifact_algorithm(),
        ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0
    );
    assert_eq!(receipt.parent_artifact_digest(), PARENT_DIGEST);
    assert_eq!(
        receipt.derived_artifact_algorithm(),
        ARTIFACT_PROVENANCE_ARTIFACT_ALGORITHM_V0
    );
    assert_eq!(receipt.derived_artifact_digest(), DERIVED_DIGEST);
    assert_eq!(receipt.bundle_byte_len(), 374);
    assert_eq!(receipt.computed_witness_root(), DERIVATION_ROOT);
    assert_eq!(receipt.parent_artifact_byte_len(), 6);
    assert_eq!(receipt.computed_parent_artifact_sha256(), PARENT_DIGEST);
    assert_eq!(receipt.derived_artifact_byte_len(), 6);
    assert_eq!(receipt.computed_derived_artifact_sha256(), DERIVED_DIGEST);
    assert_eq!(receipt.occurrences().len(), 1);
    assert_eq!(receipt.occurrences()[0].sequence, 1);
    assert_eq!(
        receipt.occurrences()[0].identity.bundle_kind,
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0
    );
    assert_eq!(
        receipt.occurrences()[0].identity.witness_root,
        DERIVATION_ROOT
    );
}

#[test]
fn repeated_identical_anchors_retain_all_ordered_occurrences_without_amplification() {
    for (selector, is_acquisition) in [
        (acquisition_selector(), true),
        (derivation_selector(), false),
    ] {
        let snapshot =
            snapshot_with_anchors(&[selector.clone(), selector.clone(), selector.clone()]);
        let trace = if is_acquisition {
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(ACQUISITION_BUNDLE),
                Some(ACQUISITION_ARTIFACT),
            )
        } else {
            derivation_outcome(
                &snapshot,
                &selector,
                Some(DERIVATION_BUNDLE),
                Some(DERIVATION_PARENT),
                Some(DERIVATION_DERIVED),
            )
        };
        let occurrences = if is_acquisition {
            trace.matched_acquisition().unwrap().occurrences()
        } else {
            trace.matched_derivation().unwrap().occurrences()
        };
        assert_eq!(occurrences.len(), 3);
        assert_eq!(
            occurrences
                .iter()
                .map(|item| item.sequence)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(
            usize::from(trace.matched_acquisition().is_some())
                + usize::from(trace.matched_derivation().is_some()),
            1
        );
    }
}

#[test]
fn same_snapshot_and_inputs_produce_equal_traces_and_canonical_bytes() {
    let acquisition_selector = acquisition_selector();
    let derivation_selector = derivation_selector();
    let snapshot =
        snapshot_with_anchors(&[acquisition_selector.clone(), derivation_selector.clone()]);

    let first_acquisition = acquisition_outcome(
        &snapshot,
        &acquisition_selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    );
    let second_acquisition = acquisition_outcome(
        &snapshot,
        &acquisition_selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    );
    assert_eq!(first_acquisition, second_acquisition);
    assert_eq!(
        first_acquisition.canonical_bytes(),
        second_acquisition.canonical_bytes()
    );

    let first_derivation = derivation_outcome(
        &snapshot,
        &derivation_selector,
        Some(DERIVATION_BUNDLE),
        Some(DERIVATION_PARENT),
        Some(DERIVATION_DERIVED),
    );
    let second_derivation = derivation_outcome(
        &snapshot,
        &derivation_selector,
        Some(DERIVATION_BUNDLE),
        Some(DERIVATION_PARENT),
        Some(DERIVATION_DERIVED),
    );
    assert_eq!(first_derivation, second_derivation);
    assert_eq!(
        first_derivation.canonical_bytes(),
        second_derivation.canonical_bytes()
    );
}

#[test]
fn both_verifiers_are_standing_inert_for_snapshot_and_v0_v1_v2_outputs() {
    let acquisition_selector = acquisition_selector();
    let derivation_selector = derivation_selector();
    let snapshot =
        snapshot_with_anchors(&[acquisition_selector.clone(), derivation_selector.clone()]);
    let snapshot_before = snapshot.canonical_bytes();
    let v0_before = snapshot
        .standing()
        .resolved_standing_with_trace(INERT_CLAIM_ID);
    let v1_before = snapshot.resolved_standing_with_trace_v1(INERT_CLAIM_ID);
    let v2_before = snapshot.resolved_standing_with_trace_v2(INERT_CLAIM_ID);

    assert!(acquisition_outcome(
        &snapshot,
        &acquisition_selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    )
    .matched_acquisition()
    .is_some());
    assert!(derivation_outcome(
        &snapshot,
        &derivation_selector,
        Some(DERIVATION_BUNDLE),
        Some(DERIVATION_PARENT),
        Some(DERIVATION_DERIVED),
    )
    .matched_derivation()
    .is_some());

    assert_eq!(snapshot_before, snapshot.canonical_bytes());
    assert_eq!(
        v0_before,
        snapshot
            .standing()
            .resolved_standing_with_trace(INERT_CLAIM_ID)
    );
    assert_eq!(
        v1_before,
        snapshot.resolved_standing_with_trace_v1(INERT_CLAIM_ID)
    );
    assert_eq!(
        v2_before,
        snapshot.resolved_standing_with_trace_v2(INERT_CLAIM_ID)
    );
    assert_eq!(v0_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v1_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v2_before.governed_standing, Some(Status::Conjectured));
}

#[test]
fn cloned_and_serialized_receipts_are_audit_only_and_not_policy_authority() {
    let acquisition_selector = acquisition_selector();
    let derivation_selector = derivation_selector();
    let matched_snapshot =
        snapshot_with_anchors(&[acquisition_selector.clone(), derivation_selector.clone()]);
    let acquisition_trace = acquisition_outcome(
        &matched_snapshot,
        &acquisition_selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    );
    let derivation_trace = derivation_outcome(
        &matched_snapshot,
        &derivation_selector,
        Some(DERIVATION_BUNDLE),
        Some(DERIVATION_PARENT),
        Some(DERIVATION_DERIVED),
    );
    let acquisition_copy = acquisition_trace.matched_acquisition().unwrap().clone();
    let derivation_copy = derivation_trace.matched_derivation().unwrap().clone();
    assert!(!serde_json::to_vec(&acquisition_copy).unwrap().is_empty());
    assert!(!serde_json::to_vec(&derivation_copy).unwrap().is_empty());

    let unanchored_snapshot = empty_snapshot();
    assert_eq!(
        acquisition_outcome(
            &unanchored_snapshot,
            &acquisition_selector,
            Some(ACQUISITION_BUNDLE),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::AnchorAbsent
    );
    assert_eq!(
        derivation_outcome(
            &unanchored_snapshot,
            &derivation_selector,
            Some(DERIVATION_BUNDLE),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        ),
        ArtifactProvenanceContextTraceV0::AnchorAbsent
    );
    assert_eq!(
        unanchored_snapshot.resolved_standing_v2(INERT_CLAIM_ID),
        Some(Status::Conjectured)
    );
}

#[test]
fn portable_fixture_bytes_lengths_hashes_newlines_and_markers_are_exact() {
    assert_eq!(ACQUISITION_BUNDLE.len(), 291);
    assert_eq!(DERIVATION_BUNDLE.len(), 374);
    assert_eq!(ACQUISITION_ARTIFACT, b"hello artifact\n");
    assert_eq!(DERIVATION_PARENT, b"alpha\n");
    assert_eq!(DERIVATION_DERIVED, b"ALPHA\n");
    assert!(!ACQUISITION_BUNDLE.ends_with(b"\n"));
    assert!(!ACQUISITION_BUNDLE.ends_with(b"\r"));
    assert!(!DERIVATION_BUNDLE.ends_with(b"\n"));
    assert!(!DERIVATION_BUNDLE.ends_with(b"\r"));
    assert_eq!(sha256_hex(ACQUISITION_ARTIFACT), ACQUISITION_DIGEST);
    assert_eq!(sha256_hex(ACQUISITION_BUNDLE), ACQUISITION_ROOT);
    assert_eq!(sha256_hex(DERIVATION_PARENT), PARENT_DIGEST);
    assert_eq!(sha256_hex(DERIVATION_DERIVED), DERIVED_DIGEST);
    assert_eq!(sha256_hex(DERIVATION_BUNDLE), DERIVATION_ROOT);

    for bytes in [
        FIXTURE_README,
        ACQUISITION_BUNDLE,
        ACQUISITION_ARTIFACT,
        DERIVATION_BUNDLE,
        DERIVATION_PARENT,
        DERIVATION_DERIVED,
    ] {
        assert!(!bytes.starts_with(&[0xef, 0xbb, 0xbf]));
        let text = std::str::from_utf8(bytes).unwrap();
        for marker in FORBIDDEN_MACHINE_MARKERS {
            assert!(
                !text.contains(marker),
                "portable fixture must not contain machine marker {marker:?}"
            );
        }
    }
}

#[test]
fn availability_size_utf8_bom_json_and_top_level_failures_are_closed() {
    let snapshot = empty_snapshot();
    let selector = acquisition_selector();
    assert_eq!(
        acquisition_outcome(&snapshot, &selector, None, Some(ACQUISITION_ARTIFACT)),
        ArtifactProvenanceContextTraceV0::BundleUnavailable
    );

    let mut over_limit_invalid_utf8 = vec![b' '; MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0 + 1];
    *over_limit_invalid_utf8.last_mut().unwrap() = 0xff;
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(&over_limit_invalid_utf8),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::BundleTooLarge
    );
    let exact_limit = vec![b' '; MAX_ARTIFACT_PROVENANCE_BUNDLE_BYTES_V0];
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(&exact_limit),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::InvalidJson
    );
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(&[0xff]),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::InvalidUtf8
    );
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(b"\xef\xbb\xbf{}"),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::Utf8BomPresent
    );

    for invalid in [
        br#"{"schema":"\q"}"#.as_slice(),
        br#"{"schema":"\ud800"}"#.as_slice(),
        br#"{"schema":"\ud800\u0041"}"#.as_slice(),
        br#"{}{}"#.as_slice(),
    ] {
        assert_eq!(
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(invalid),
                Some(ACQUISITION_ARTIFACT),
            ),
            ArtifactProvenanceContextTraceV0::InvalidJson
        );
    }
    for non_object in [br#"[]"#.as_slice(), br#""text""#.as_slice()] {
        assert_eq!(
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(non_object),
                Some(ACQUISITION_ARTIFACT),
            ),
            ArtifactProvenanceContextTraceV0::TopLevelNotObject
        );
    }
}

#[test]
fn duplicate_shape_and_schema_failures_follow_canonical_precedence() {
    let snapshot = empty_snapshot();
    let selector = acquisition_selector();
    let base = acquisition_text();

    let duplicate_top = base.replacen("\"schema\":", "\"schema\":\"duplicate\",\"schema\":", 1);
    let duplicate_nested = base.replacen(
        "\"algorithm\":",
        "\"algorithm\":\"sha256\",\"algorithm\":",
        1,
    );
    let duplicate_inside_unknown = base.replacen(
        "\"run_id\":",
        "\"unknown\":{\"x\":0,\"x\":1},\"run_id\":",
        1,
    );
    for hostile in [duplicate_top, duplicate_nested, duplicate_inside_unknown] {
        assert_eq!(
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(hostile.as_bytes()),
                Some(ACQUISITION_ARTIFACT),
            ),
            ArtifactProvenanceContextTraceV0::DuplicateKey
        );
    }

    let missing = base.replace("\"acquisition_id\":\"acq-0001\",", "");
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(missing.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::MissingField
    );
    let unknown = base.replacen("\"run_id\":", "\"unknown\":\"value\",\"run_id\":", 1);
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(unknown.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::UnknownField
    );
    let wrong_type = base.replace("\"acquisition_id\":\"acq-0001\"", "\"acquisition_id\":[]");
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(wrong_type.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::WrongJsonType
    );
    let schema_mismatch = base.replace(ARTIFACT_ACQUISITION_SCHEMA_V0, "wrong-schema");
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(schema_mismatch.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::SchemaMismatch
    );

    let duplicate_and_missing =
        missing.replacen("\"schema\":", "\"schema\":\"duplicate\",\"schema\":", 1);
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(duplicate_and_missing.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::DuplicateKey
    );
    let missing_and_unknown = missing.replacen("\"run_id\":", "\"unknown\":0,\"run_id\":", 1);
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(missing_and_unknown.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::MissingField
    );
    let schema_wrong_type = base.replace(
        &format!("\"schema\":\"{ARTIFACT_ACQUISITION_SCHEMA_V0}\""),
        "\"schema\":false",
    );
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(schema_wrong_type.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::WrongJsonType
    );
}

#[test]
fn semantic_failures_follow_exact_schema_order() {
    let snapshot = empty_snapshot();
    let selector = acquisition_selector();
    let base = acquisition_text();
    let cases = [
        (
            base.replace(
                "\"acquisition_id\":\"acq-0001\"",
                "\"acquisition_id\":\"-bad\"",
            ),
            ArtifactProvenanceContextTraceV0::InvalidIdentifier,
        ),
        (
            base.replace(
                "\"observed_locator\":\"file:hello-artifact.txt\"",
                "\"observed_locator\":\"\"",
            ),
            ArtifactProvenanceContextTraceV0::InvalidObservedLocator,
        ),
        (
            base.replace("\"algorithm\":\"sha256\"", "\"algorithm\":\"sha-256\""),
            ArtifactProvenanceContextTraceV0::UnsupportedArtifactAlgorithm,
        ),
        (
            base.replace(ACQUISITION_DIGEST, &ACQUISITION_DIGEST.to_uppercase()),
            ArtifactProvenanceContextTraceV0::InvalidArtifactDigest,
        ),
    ];
    for (hostile, expected) in cases {
        assert_eq!(
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(hostile.as_bytes()),
                Some(ACQUISITION_ARTIFACT),
            ),
            expected
        );
    }

    let invalid_identifier_and_whitespace = base
        .replace(
            "\"acquisition_id\":\"acq-0001\"",
            "\"acquisition_id\":\"-bad\"",
        )
        .replacen('{', "{ ", 1);
    assert_eq!(
        acquisition_outcome(
            &snapshot,
            &selector,
            Some(invalid_identifier_and_whitespace.as_bytes()),
            Some(ACQUISITION_ARTIFACT),
        ),
        ArtifactProvenanceContextTraceV0::InvalidIdentifier
    );

    let self_loop = derivation_text().replace(DERIVED_DIGEST, PARENT_DIGEST);
    assert_eq!(
        derivation_outcome(
            &snapshot,
            &derivation_selector(),
            Some(self_loop.as_bytes()),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        ),
        ArtifactProvenanceContextTraceV0::ParentEqualsDerived
    );
}

#[test]
fn every_pinned_alternate_json_spelling_is_noncanonical() {
    let snapshot = empty_snapshot();
    let selector = acquisition_selector();
    let base = acquisition_text();
    let reordered = format!(
        "{{\"acquisition_id\":\"acq-0001\",\"schema\":\"{ARTIFACT_ACQUISITION_SCHEMA_V0}\",\"acquisition_profile\":\"magpie-import-v0\",\"artifact\":{{\"algorithm\":\"sha256\",\"digest\":\"{ACQUISITION_DIGEST}\"}},\"observed_locator\":\"file:hello-artifact.txt\",\"run_id\":\"run-acq-0001\"}}"
    );
    let cases = [
        base.replacen('{', "{ ", 1),
        reordered,
        format!("{base}\n"),
        base.replace("file:hello-artifact.txt", r"file:\u0068ello-artifact.txt"),
        base.replace("file:hello-artifact.txt", r"file:hello\/artifact.txt"),
        base.replace("file:hello-artifact.txt", r"file:\u006Artifact"),
        base.replace("file:hello-artifact.txt", r"file:\ud83d\ude00"),
    ];
    for hostile in cases {
        assert_eq!(
            acquisition_outcome(
                &snapshot,
                &selector,
                Some(hostile.as_bytes()),
                Some(ACQUISITION_ARTIFACT),
            ),
            ArtifactProvenanceContextTraceV0::NonCanonicalEncoding,
            "hostile spelling: {hostile}"
        );
    }
}

#[test]
fn selector_root_replay_and_acquisition_artifact_precedence_is_exact() {
    let empty = empty_snapshot();
    let wrong_kind = selector(
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
        "wrong-root",
        "wrong-algorithm",
        "wrong-profile",
        "wrong-run",
    );
    assert_eq!(
        acquisition_outcome(&empty, &wrong_kind, Some(ACQUISITION_BUNDLE), None,),
        ArtifactProvenanceContextTraceV0::BundleKindMismatch
    );
    let wrong_algorithm = selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        "wrong-root",
        "sha-256",
        "wrong-profile",
        "wrong-run",
    );
    assert_eq!(
        acquisition_outcome(&empty, &wrong_algorithm, Some(ACQUISITION_BUNDLE), None,),
        ArtifactProvenanceContextTraceV0::WitnessAlgorithmMismatch
    );
    let wrong_profile = selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        "wrong-root",
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        "wrong-profile",
        "wrong-run",
    );
    assert_eq!(
        acquisition_outcome(&empty, &wrong_profile, Some(ACQUISITION_BUNDLE), None,),
        ArtifactProvenanceContextTraceV0::CanonicalizationProfileMismatch
    );
    let wrong_run = selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        "wrong-root",
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "wrong-run",
    );
    assert_eq!(
        acquisition_outcome(&empty, &wrong_run, Some(ACQUISITION_BUNDLE), None),
        ArtifactProvenanceContextTraceV0::RunIdMismatch
    );
    let wrong_root = selector(
        ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
        &"A".repeat(64),
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-acq-0001",
    );
    assert_eq!(
        acquisition_outcome(&empty, &wrong_root, Some(ACQUISITION_BUNDLE), None),
        ArtifactProvenanceContextTraceV0::WitnessRootMismatch
    );
    let exact = acquisition_selector();
    assert_eq!(
        acquisition_outcome(&empty, &exact, Some(ACQUISITION_BUNDLE), None),
        ArtifactProvenanceContextTraceV0::AnchorAbsent
    );

    let anchored = snapshot_with_anchors(std::slice::from_ref(&exact));
    assert_eq!(
        acquisition_outcome(&anchored, &exact, Some(ACQUISITION_BUNDLE), None),
        ArtifactProvenanceContextTraceV0::ArtifactUnavailable
    );
    assert_eq!(
        acquisition_outcome(
            &anchored,
            &exact,
            Some(ACQUISITION_BUNDLE),
            Some(b"wrong artifact"),
        ),
        ArtifactProvenanceContextTraceV0::ArtifactDigestMismatch
    );
}

#[test]
fn derivation_artifact_availability_and_digest_precedence_is_exact() {
    let selector = derivation_selector();
    let snapshot = snapshot_with_anchors(std::slice::from_ref(&selector));
    assert_eq!(
        derivation_outcome(&snapshot, &selector, Some(DERIVATION_BUNDLE), None, None,),
        ArtifactProvenanceContextTraceV0::ParentArtifactUnavailable
    );
    assert_eq!(
        derivation_outcome(
            &snapshot,
            &selector,
            Some(DERIVATION_BUNDLE),
            Some(DERIVATION_PARENT),
            None,
        ),
        ArtifactProvenanceContextTraceV0::DerivedArtifactUnavailable
    );

    let both_wrong_bundle = derivation_text()
        .replace(PARENT_DIGEST, &"0".repeat(64))
        .replace(DERIVED_DIGEST, &"1".repeat(64));
    let both_wrong_selector = selector_for_bundle(
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
        "run-der-0001",
        both_wrong_bundle.as_bytes(),
    );
    let both_wrong_snapshot = snapshot_with_anchors(std::slice::from_ref(&both_wrong_selector));
    assert_eq!(
        derivation_outcome(
            &both_wrong_snapshot,
            &both_wrong_selector,
            Some(both_wrong_bundle.as_bytes()),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        ),
        ArtifactProvenanceContextTraceV0::ParentArtifactDigestMismatch
    );

    let derived_wrong_bundle = derivation_text().replace(DERIVED_DIGEST, &"2".repeat(64));
    let derived_wrong_selector = selector_for_bundle(
        ARTIFACT_DERIVATION_BUNDLE_KIND_V0,
        "run-der-0001",
        derived_wrong_bundle.as_bytes(),
    );
    let derived_wrong_snapshot =
        snapshot_with_anchors(std::slice::from_ref(&derived_wrong_selector));
    assert_eq!(
        derivation_outcome(
            &derived_wrong_snapshot,
            &derived_wrong_selector,
            Some(derived_wrong_bundle.as_bytes()),
            Some(DERIVATION_PARENT),
            Some(DERIVATION_DERIVED),
        ),
        ArtifactProvenanceContextTraceV0::DerivedArtifactDigestMismatch
    );
}

#[test]
fn every_unit_trace_has_deterministic_tagged_serialization() {
    use ArtifactProvenanceContextTraceV0::*;
    let cases = [
        (BundleUnavailable, "bundle_unavailable"),
        (BundleTooLarge, "bundle_too_large"),
        (InvalidUtf8, "invalid_utf8"),
        (Utf8BomPresent, "utf8_bom_present"),
        (InvalidJson, "invalid_json"),
        (TopLevelNotObject, "top_level_not_object"),
        (DuplicateKey, "duplicate_key"),
        (MissingField, "missing_field"),
        (UnknownField, "unknown_field"),
        (WrongJsonType, "wrong_json_type"),
        (SchemaMismatch, "schema_mismatch"),
        (InvalidIdentifier, "invalid_identifier"),
        (InvalidObservedLocator, "invalid_observed_locator"),
        (
            UnsupportedArtifactAlgorithm,
            "unsupported_artifact_algorithm",
        ),
        (InvalidArtifactDigest, "invalid_artifact_digest"),
        (ParentEqualsDerived, "parent_equals_derived"),
        (NonCanonicalEncoding, "non_canonical_encoding"),
        (AnchorAbsent, "anchor_absent"),
        (BundleKindMismatch, "bundle_kind_mismatch"),
        (WitnessAlgorithmMismatch, "witness_algorithm_mismatch"),
        (
            CanonicalizationProfileMismatch,
            "canonicalization_profile_mismatch",
        ),
        (RunIdMismatch, "run_id_mismatch"),
        (WitnessRootMismatch, "witness_root_mismatch"),
        (ArtifactUnavailable, "artifact_unavailable"),
        (ArtifactDigestMismatch, "artifact_digest_mismatch"),
        (ParentArtifactUnavailable, "parent_artifact_unavailable"),
        (
            ParentArtifactDigestMismatch,
            "parent_artifact_digest_mismatch",
        ),
        (DerivedArtifactUnavailable, "derived_artifact_unavailable"),
        (
            DerivedArtifactDigestMismatch,
            "derived_artifact_digest_mismatch",
        ),
    ];
    for (trace, outcome) in cases {
        assert_eq!(
            trace.canonical_bytes(),
            format!(r#"{{"outcome":"{outcome}"}}"#).as_bytes()
        );
    }
}
