use magpie_claims::{
    replay_standing_context, ArtifactProvenanceAnchorSelectorV0, ResolutionArtifactObjectInputV0,
    ResolutionArtifactObjectKeyV0, ResolutionContentClosureConstructionErrorV0,
    ResolutionContentClosureKeyFieldV0, ResolutionContentClosureV0,
    ResolutionForeignBundleObjectInputV0, ARTIFACT_ACQUISITION_BUNDLE_KIND_V0,
    ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0, ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0,
    MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0,
    RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0,
    RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0, RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0,
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

const ACQUISITION_ROOT: &str = "4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e";
const ACQUISITION_DIGEST: &str = "51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076";
const PARENT_DIGEST: &str = "b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060";
const DERIVED_DIGEST: &str = "1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005";
const DERIVATION_ROOT: &str = "472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd";

const EMPTY_MANIFEST: &str = r#"{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[],"foreign_bundle_objects":[]}"#;
const EMPTY_MANIFEST_SHA256: &str =
    "95446014b15ce3e3cb63784da4898defad484c34ff29ddd09d662bd7a2b58092";

const ACQUISITION_MANIFEST: &str = r#"{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[{"key":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"byte_length":15,"content_sha256":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"}],"foreign_bundle_objects":[{"key":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-0001"},"byte_length":291,"content_sha256":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e"}]}"#;
const ACQUISITION_MANIFEST_SHA256: &str =
    "2253d66f473c8b0f71a47f308ef4c0dffbb6f56e522b58c54bfd5519f958a224";

const COMPLETE_MANIFEST: &str = r#"{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[{"key":{"algorithm":"sha256","digest":"1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"},"byte_length":6,"content_sha256":"1921b918b15842c7fdb115078e610263fac85f159c1d8e0ecec3d89a0faa4005"},{"key":{"algorithm":"sha256","digest":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},"byte_length":15,"content_sha256":"51bc0fc1f19104fa6e89ce50be9aa1f57c3346c1ca51ab49f5f00e14ce8f8076"},{"key":{"algorithm":"sha256","digest":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"},"byte_length":6,"content_sha256":"b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060"}],"foreign_bundle_objects":[{"key":{"bundle_kind":"magpie-artifact-acquisition-v0","witness_root":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-acq-0001"},"byte_length":291,"content_sha256":"4a5cdc953523db6bdbd8d88d3534334deb7f6f4288e434b1cfd4786b1ffb093e"},{"key":{"bundle_kind":"magpie-artifact-derivation-v0","witness_root":"472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd","witness_algorithm":"sha256","canonicalization_profile":"magpie-artifact-provenance-json-v0","run_id":"run-der-0001"},"byte_length":374,"content_sha256":"472e857153fe8c901b6c0d7d1b15bd876450b57b4743222a2fab91eb60bc76dd"}]}"#;
const COMPLETE_MANIFEST_SHA256: &str =
    "2e0f59ee647407c6e66ae7e05913788b92b0b4f85c739f4ed3bc231511a30a84";

fn artifact_key(algorithm: &str, digest: &str) -> ResolutionArtifactObjectKeyV0 {
    ResolutionArtifactObjectKeyV0::new(algorithm, digest)
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

fn simple_selector(name: &str) -> ArtifactProvenanceAnchorSelectorV0 {
    selector(
        &format!("bundle-{name}"),
        &format!("root-{name}"),
        "sha256",
        "profile-v0",
        &format!("run-{name}"),
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
        "magpie-artifact-derivation-v0",
        DERIVATION_ROOT,
        ARTIFACT_PROVENANCE_WITNESS_ALGORITHM_V0,
        ARTIFACT_PROVENANCE_CANONICALIZATION_PROFILE_V0,
        "run-der-0001",
    )
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn assert_success_invariant(closure: &ResolutionContentClosureV0) {
    assert!(closure.retained_object_bytes() <= closure.supplied_object_bytes());
    assert!(
        closure.supplied_object_bytes() <= MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0
    );
}

fn assert_identity(closure: &ResolutionContentClosureV0, expected_manifest_sha256: &str) {
    assert_eq!(
        closure.identity().schema(),
        RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0
    );
    assert_eq!(
        closure.identity().canonicalization_profile(),
        RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0
    );
    assert_eq!(
        closure.identity().digest_algorithm(),
        RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0
    );
    assert_eq!(
        closure.identity().manifest_sha256(),
        expected_manifest_sha256
    );
}

fn assert_manifest_envelope(closure: &ResolutionContentClosureV0) {
    let manifest = closure.canonical_manifest_bytes();
    assert_eq!(manifest.first(), Some(&b'{'));
    assert_eq!(manifest.last(), Some(&b'}'));
    assert!(!manifest.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!manifest.ends_with(b"\n"));
    assert!(!manifest.ends_with(b"\r"));
}

fn assert_equivalent_closures(
    left: &ResolutionContentClosureV0,
    right: &ResolutionContentClosureV0,
    artifact_keys: &[ResolutionArtifactObjectKeyV0],
    bundle_keys: &[ArtifactProvenanceAnchorSelectorV0],
) {
    assert_eq!(
        left.canonical_manifest_bytes(),
        right.canonical_manifest_bytes()
    );
    assert_eq!(left.identity(), right.identity());
    assert_eq!(left.artifact_count(), right.artifact_count());
    assert_eq!(left.foreign_bundle_count(), right.foreign_bundle_count());
    assert_eq!(left.supplied_object_bytes(), right.supplied_object_bytes());
    assert_eq!(left.retained_object_bytes(), right.retained_object_bytes());
    for key in artifact_keys {
        assert_eq!(left.artifact(key), right.artifact(key));
    }
    for key in bundle_keys {
        assert_eq!(left.foreign_bundle(key), right.foreign_bundle(key));
    }
    assert_success_invariant(left);
    assert_success_invariant(right);
}

#[test]
fn exact_public_constants_are_pinned() {
    assert_eq!(
        RESOLUTION_CONTENT_CLOSURE_SCHEMA_V0,
        "magpie-resolution-content-closure-v0"
    );
    assert_eq!(
        RESOLUTION_CONTENT_CLOSURE_CANONICALIZATION_PROFILE_V0,
        "magpie-resolution-content-closure-json-v0"
    );
    assert_eq!(RESOLUTION_CONTENT_CLOSURE_DIGEST_ALGORITHM_V0, "sha256");
    assert_eq!(MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_ENTRIES_V0, 1_024);
    assert_eq!(
        MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_ENTRIES_V0,
        1_024
    );
    assert_eq!(MAX_RESOLUTION_CONTENT_CLOSURE_TOTAL_ENTRIES_V0, 2_048);
    assert_eq!(MAX_RESOLUTION_CONTENT_CLOSURE_KEY_FIELD_BYTES_V0, 1_024);
    assert_eq!(
        MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0,
        67_108_864
    );
    assert_eq!(
        MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0,
        1_048_576
    );
    assert_eq!(
        MAX_RESOLUTION_CONTENT_CLOSURE_SUPPLIED_OBJECT_BYTES_V0,
        536_870_912
    );
}

#[test]
fn count_failures_and_total_count_precedence_are_exact() {
    let empty = b"";
    let artifacts: Vec<_> = (0..1_024)
        .map(|_| ResolutionArtifactObjectInputV0::new(artifact_key("a", "a"), empty))
        .collect();
    let hostile_bundle_kind = "x".repeat(1_025);
    let bundles: Vec<_> = (0..1_025)
        .map(|index| {
            let bundle_kind = if index == 0 {
                hostile_bundle_kind.as_str()
            } else {
                "bundle"
            };
            ResolutionForeignBundleObjectInputV0::new(
                selector(bundle_kind, "root", "algorithm", "profile", "run"),
                empty,
            )
        })
        .collect();
    assert_eq!(
        ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::TotalEntryLimitExceeded {
            supplied_entries: 2_049,
            maximum_entries: 2_048,
        }
    );

    let artifacts: Vec<_> = (0..1_025)
        .map(|_| ResolutionArtifactObjectInputV0::new(artifact_key("a", "a"), empty))
        .collect();
    assert_eq!(
        ResolutionContentClosureV0::construct(&artifacts, &[]).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::ArtifactEntryLimitExceeded {
            supplied_artifact_entries: 1_025,
            maximum_entries: 1_024,
        }
    );

    let bundles: Vec<_> = (0..1_025)
        .map(|_| ResolutionForeignBundleObjectInputV0::new(simple_selector("same"), empty))
        .collect();
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &bundles).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::ForeignBundleEntryLimitExceeded {
            supplied_foreign_bundle_entries: 1_025,
            maximum_entries: 1_024,
        }
    );
}

fn key_field_error(
    field: ResolutionContentClosureKeyFieldV0,
    value: &str,
) -> ResolutionContentClosureConstructionErrorV0 {
    let empty = b"";
    match field {
        ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm => {
            let inputs = [ResolutionArtifactObjectInputV0::new(
                artifact_key(value, "digest"),
                empty,
            )];
            ResolutionContentClosureV0::construct(&inputs, &[]).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::ArtifactDigest => {
            let inputs = [ResolutionArtifactObjectInputV0::new(
                artifact_key("algorithm", value),
                empty,
            )];
            ResolutionContentClosureV0::construct(&inputs, &[]).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::BundleKind => {
            let inputs = [ResolutionForeignBundleObjectInputV0::new(
                selector(value, "root", "algorithm", "profile", "run"),
                empty,
            )];
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::WitnessRoot => {
            let inputs = [ResolutionForeignBundleObjectInputV0::new(
                selector("bundle", value, "algorithm", "profile", "run"),
                empty,
            )];
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::WitnessAlgorithm => {
            let inputs = [ResolutionForeignBundleObjectInputV0::new(
                selector("bundle", "root", value, "profile", "run"),
                empty,
            )];
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::CanonicalizationProfile => {
            let inputs = [ResolutionForeignBundleObjectInputV0::new(
                selector("bundle", "root", "algorithm", value, "run"),
                empty,
            )];
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err()
        }
        ResolutionContentClosureKeyFieldV0::RunId => {
            let inputs = [ResolutionForeignBundleObjectInputV0::new(
                selector("bundle", "root", "algorithm", "profile", value),
                empty,
            )];
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err()
        }
    }
}

#[test]
fn every_key_field_has_an_ordinary_failure_witness() {
    let hostile = "x".repeat(1_025);
    for field in [
        ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm,
        ResolutionContentClosureKeyFieldV0::ArtifactDigest,
        ResolutionContentClosureKeyFieldV0::BundleKind,
        ResolutionContentClosureKeyFieldV0::WitnessRoot,
        ResolutionContentClosureKeyFieldV0::WitnessAlgorithm,
        ResolutionContentClosureKeyFieldV0::CanonicalizationProfile,
        ResolutionContentClosureKeyFieldV0::RunId,
    ] {
        assert_eq!(
            key_field_error(field, &hostile),
            ResolutionContentClosureConstructionErrorV0::KeyFieldTooLarge {
                field,
                offending_entries: 1,
                maximum_observed_bytes: 1_025,
                maximum_bytes: 1_024,
            }
        );
    }
}

#[test]
fn key_field_boundaries_unicode_aggregation_precedence_and_order_are_exact() {
    let empty = b"";
    let exact_ascii = "x".repeat(1_024);
    let inputs = [ResolutionArtifactObjectInputV0::new(
        artifact_key(&exact_ascii, "digest"),
        empty,
    )];
    let closure = ResolutionContentClosureV0::construct(&inputs, &[]).unwrap();
    assert_success_invariant(&closure);

    let exact_unicode = "é".repeat(512);
    assert_eq!(exact_unicode.chars().count(), 512);
    assert_eq!(exact_unicode.len(), 1_024);
    let inputs = [ResolutionArtifactObjectInputV0::new(
        artifact_key(&exact_unicode, "digest"),
        empty,
    )];
    let closure = ResolutionContentClosureV0::construct(&inputs, &[]).unwrap();
    assert_success_invariant(&closure);

    let hostile_unicode = "é".repeat(513);
    assert_eq!(hostile_unicode.chars().count(), 513);
    assert_eq!(
        key_field_error(
            ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm,
            &hostile_unicode
        ),
        ResolutionContentClosureConstructionErrorV0::KeyFieldTooLarge {
            field: ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm,
            offending_entries: 1,
            maximum_observed_bytes: 1_026,
            maximum_bytes: 1_024,
        }
    );

    let first = "a".repeat(1_025);
    let second = "b".repeat(1_030);
    let hostile_digest = "d".repeat(1_040);
    let forward = [
        ResolutionArtifactObjectInputV0::new(artifact_key(&first, &hostile_digest), empty),
        ResolutionArtifactObjectInputV0::new(artifact_key(&second, "digest"), empty),
    ];
    let reverse = [
        ResolutionArtifactObjectInputV0::new(artifact_key(&second, "digest"), empty),
        ResolutionArtifactObjectInputV0::new(artifact_key(&first, &hostile_digest), empty),
    ];
    let expected = ResolutionContentClosureConstructionErrorV0::KeyFieldTooLarge {
        field: ResolutionContentClosureKeyFieldV0::ArtifactAlgorithm,
        offending_entries: 2,
        maximum_observed_bytes: 1_030,
        maximum_bytes: 1_024,
    };
    assert_eq!(
        ResolutionContentClosureV0::construct(&forward, &[]).unwrap_err(),
        expected
    );
    assert_eq!(
        ResolutionContentClosureV0::construct(&reverse, &[]).unwrap_err(),
        expected
    );
}

#[test]
fn artifact_object_boundary_and_smallest_oversized_key_are_exact() {
    {
        let exact = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0];
        let key = artifact_key("sha256", "exact");
        let inputs = [ResolutionArtifactObjectInputV0::new(key.clone(), &exact)];
        let closure = ResolutionContentClosureV0::construct(&inputs, &[]).unwrap();
        assert_eq!(closure.artifact(&key), Some(exact.as_slice()));
        assert_success_invariant(&closure);
    }

    {
        let over = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0 + 1];
        let key = artifact_key("sha256", "over");
        let inputs = [ResolutionArtifactObjectInputV0::new(key.clone(), &over)];
        assert_eq!(
            ResolutionContentClosureV0::construct(&inputs, &[]).unwrap_err(),
            ResolutionContentClosureConstructionErrorV0::ArtifactObjectTooLarge {
                key,
                maximum_observed_bytes_for_key: 67_108_865,
                maximum_bytes: 67_108_864,
            }
        );
    }

    let oversized = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_ARTIFACT_OBJECT_BYTES_V0 + 2];
    let smallest = artifact_key("a", "a");
    let larger = artifact_key("z", "z");
    let forward = [
        ResolutionArtifactObjectInputV0::new(larger.clone(), &oversized[..67_108_865]),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), &oversized[..67_108_865]),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), &oversized),
    ];
    let reverse = [
        ResolutionArtifactObjectInputV0::new(smallest.clone(), &oversized),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), &oversized[..67_108_865]),
        ResolutionArtifactObjectInputV0::new(larger, &oversized[..67_108_865]),
    ];
    let expected = ResolutionContentClosureConstructionErrorV0::ArtifactObjectTooLarge {
        key: smallest,
        maximum_observed_bytes_for_key: 67_108_866,
        maximum_bytes: 67_108_864,
    };
    assert_eq!(
        ResolutionContentClosureV0::construct(&forward, &[]).unwrap_err(),
        expected
    );
    assert_eq!(
        ResolutionContentClosureV0::construct(&reverse, &[]).unwrap_err(),
        expected
    );
}

#[test]
fn foreign_bundle_object_boundary_and_smallest_oversized_selector_are_exact() {
    {
        let exact = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0];
        let key = simple_selector("exact");
        let inputs = [ResolutionForeignBundleObjectInputV0::new(
            key.clone(),
            &exact,
        )];
        let closure = ResolutionContentClosureV0::construct(&[], &inputs).unwrap();
        assert_eq!(closure.foreign_bundle(&key), Some(exact.as_slice()));
        assert_success_invariant(&closure);
    }

    {
        let over = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0 + 1];
        let key = simple_selector("over");
        let inputs = [ResolutionForeignBundleObjectInputV0::new(
            key.clone(),
            &over,
        )];
        assert_eq!(
            ResolutionContentClosureV0::construct(&[], &inputs).unwrap_err(),
            ResolutionContentClosureConstructionErrorV0::ForeignBundleObjectTooLarge {
                selector: key,
                maximum_observed_bytes_for_key: 1_048_577,
                maximum_bytes: 1_048_576,
            }
        );
    }

    let oversized = vec![0_u8; MAX_RESOLUTION_CONTENT_CLOSURE_FOREIGN_BUNDLE_OBJECT_BYTES_V0 + 2];
    let smallest = simple_selector("a");
    let larger = simple_selector("z");
    let forward = [
        ResolutionForeignBundleObjectInputV0::new(larger.clone(), &oversized[..1_048_577]),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), &oversized[..1_048_577]),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), &oversized),
    ];
    let reverse = [
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), &oversized),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), &oversized[..1_048_577]),
        ResolutionForeignBundleObjectInputV0::new(larger, &oversized[..1_048_577]),
    ];
    let expected = ResolutionContentClosureConstructionErrorV0::ForeignBundleObjectTooLarge {
        selector: smallest,
        maximum_observed_bytes_for_key: 1_048_578,
        maximum_bytes: 1_048_576,
    };
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &forward).unwrap_err(),
        expected
    );
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &reverse).unwrap_err(),
        expected
    );
}

#[test]
fn total_supplied_boundary_uses_repeated_borrowed_references() {
    let one_mib = vec![7_u8; 1_048_576];
    let key = simple_selector("duplicate");
    let exact: Vec<_> = (0..512)
        .map(|_| ResolutionForeignBundleObjectInputV0::new(key.clone(), &one_mib))
        .collect();
    let closure = ResolutionContentClosureV0::construct(&[], &exact).unwrap();
    assert_eq!(closure.supplied_object_bytes(), 536_870_912);
    assert_eq!(closure.retained_object_bytes(), 1_048_576);
    assert_eq!(closure.foreign_bundle_count(), 1);
    assert_success_invariant(&closure);
    drop(closure);
    drop(exact);

    let over: Vec<_> = (0..513)
        .map(|_| ResolutionForeignBundleObjectInputV0::new(key.clone(), &one_mib))
        .collect();
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &over).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::TotalSuppliedBytesExceeded {
            supplied_object_bytes: 537_919_488,
            maximum_bytes: 536_870_912,
        }
    );
}

#[test]
fn total_supplied_failure_precedes_artifact_and_bundle_conflicts() {
    let one_mib_a = vec![1_u8; 1_048_576];
    let one_mib_b = vec![2_u8; 1_048_576];
    let bundle_key = simple_selector("conflict");
    let mut bundles: Vec<_> = (0..511)
        .map(|_| ResolutionForeignBundleObjectInputV0::new(bundle_key.clone(), &one_mib_a))
        .collect();
    bundles.push(ResolutionForeignBundleObjectInputV0::new(
        bundle_key, &one_mib_b,
    ));

    let artifact_key = artifact_key("sha256", "conflict");
    let artifacts = [
        ResolutionArtifactObjectInputV0::new(artifact_key.clone(), b"a"),
        ResolutionArtifactObjectInputV0::new(artifact_key, b"b"),
    ];
    assert_eq!(
        ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::TotalSuppliedBytesExceeded {
            supplied_object_bytes: 536_870_914,
            maximum_bytes: 536_870_912,
        }
    );
}

#[test]
fn artifact_duplicates_conflicts_and_distinct_keys_are_exact() {
    let key = artifact_key("sha256", "same");
    let bytes_a = b"same bytes".to_vec();
    let bytes_b = b"same bytes".to_vec();
    let inputs = [
        ResolutionArtifactObjectInputV0::new(key.clone(), &bytes_a),
        ResolutionArtifactObjectInputV0::new(key.clone(), &bytes_a),
        ResolutionArtifactObjectInputV0::new(key.clone(), &bytes_b),
    ];
    let closure = ResolutionContentClosureV0::construct(&inputs, &[]).unwrap();
    assert_eq!(closure.artifact_count(), 1);
    assert_eq!(closure.artifact(&key), Some(bytes_a.as_slice()));
    assert_eq!(closure.supplied_object_bytes(), 30);
    assert_eq!(closure.retained_object_bytes(), 10);
    assert_success_invariant(&closure);

    let conflict = [
        ResolutionArtifactObjectInputV0::new(key.clone(), b"left"),
        ResolutionArtifactObjectInputV0::new(key.clone(), b"right"),
    ];
    assert_eq!(
        ResolutionContentClosureV0::construct(&conflict, &[]).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::ConflictingArtifactObject { key: key.clone() }
    );

    let smallest = artifact_key("a", "a");
    let larger = artifact_key("z", "z");
    let forward = [
        ResolutionArtifactObjectInputV0::new(larger.clone(), b"1"),
        ResolutionArtifactObjectInputV0::new(larger.clone(), b"2"),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), b"3"),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), b"4"),
    ];
    let reverse = [
        ResolutionArtifactObjectInputV0::new(smallest.clone(), b"4"),
        ResolutionArtifactObjectInputV0::new(smallest.clone(), b"3"),
        ResolutionArtifactObjectInputV0::new(larger.clone(), b"2"),
        ResolutionArtifactObjectInputV0::new(larger, b"1"),
    ];
    let expected =
        ResolutionContentClosureConstructionErrorV0::ConflictingArtifactObject { key: smallest };
    assert_eq!(
        ResolutionContentClosureV0::construct(&forward, &[]).unwrap_err(),
        expected
    );
    assert_eq!(
        ResolutionContentClosureV0::construct(&reverse, &[]).unwrap_err(),
        expected
    );

    let key_a = artifact_key("sha256", "a");
    let key_b = artifact_key("sha256", "b");
    let same_bytes = b"0123456789";
    let distinct = [
        ResolutionArtifactObjectInputV0::new(key_a.clone(), same_bytes),
        ResolutionArtifactObjectInputV0::new(key_b.clone(), same_bytes),
    ];
    let closure = ResolutionContentClosureV0::construct(&distinct, &[]).unwrap();
    assert_eq!(closure.artifact_count(), 2);
    assert_eq!(closure.artifact(&key_a), Some(same_bytes.as_slice()));
    assert_eq!(closure.artifact(&key_b), Some(same_bytes.as_slice()));
    assert_eq!(closure.supplied_object_bytes(), 20);
    assert_eq!(closure.retained_object_bytes(), 20);
    assert_success_invariant(&closure);
}

#[test]
fn foreign_bundle_duplicates_conflicts_and_distinct_selectors_are_exact() {
    let key = simple_selector("same");
    let bytes_a = b"same bytes".to_vec();
    let bytes_b = b"same bytes".to_vec();
    let inputs = [
        ResolutionForeignBundleObjectInputV0::new(key.clone(), &bytes_a),
        ResolutionForeignBundleObjectInputV0::new(key.clone(), &bytes_a),
        ResolutionForeignBundleObjectInputV0::new(key.clone(), &bytes_b),
    ];
    let closure = ResolutionContentClosureV0::construct(&[], &inputs).unwrap();
    assert_eq!(closure.foreign_bundle_count(), 1);
    assert_eq!(closure.foreign_bundle(&key), Some(bytes_a.as_slice()));
    assert_eq!(closure.supplied_object_bytes(), 30);
    assert_eq!(closure.retained_object_bytes(), 10);
    assert_success_invariant(&closure);

    let conflict = [
        ResolutionForeignBundleObjectInputV0::new(key.clone(), b"left"),
        ResolutionForeignBundleObjectInputV0::new(key.clone(), b"right"),
    ];
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &conflict).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::ConflictingForeignBundleObject {
            selector: key,
        }
    );

    let smallest = simple_selector("a");
    let larger = simple_selector("z");
    let forward = [
        ResolutionForeignBundleObjectInputV0::new(larger.clone(), b"1"),
        ResolutionForeignBundleObjectInputV0::new(larger.clone(), b"2"),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), b"3"),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), b"4"),
    ];
    let reverse = [
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), b"4"),
        ResolutionForeignBundleObjectInputV0::new(smallest.clone(), b"3"),
        ResolutionForeignBundleObjectInputV0::new(larger.clone(), b"2"),
        ResolutionForeignBundleObjectInputV0::new(larger, b"1"),
    ];
    let expected = ResolutionContentClosureConstructionErrorV0::ConflictingForeignBundleObject {
        selector: smallest,
    };
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &forward).unwrap_err(),
        expected
    );
    assert_eq!(
        ResolutionContentClosureV0::construct(&[], &reverse).unwrap_err(),
        expected
    );

    let key_a = simple_selector("a");
    let key_b = simple_selector("b");
    let same_bytes = b"0123456789";
    let distinct = [
        ResolutionForeignBundleObjectInputV0::new(key_a.clone(), same_bytes),
        ResolutionForeignBundleObjectInputV0::new(key_b.clone(), same_bytes),
    ];
    let closure = ResolutionContentClosureV0::construct(&[], &distinct).unwrap();
    assert_eq!(closure.foreign_bundle_count(), 2);
    assert_eq!(closure.foreign_bundle(&key_a), Some(same_bytes.as_slice()));
    assert_eq!(closure.foreign_bundle(&key_b), Some(same_bytes.as_slice()));
    assert_eq!(closure.supplied_object_bytes(), 20);
    assert_eq!(closure.retained_object_bytes(), 20);
    assert_success_invariant(&closure);
}

#[test]
fn artifact_conflict_precedes_foreign_bundle_conflict() {
    let artifact_key = artifact_key("a", "a");
    let bundle_key = simple_selector("a");
    let artifacts = [
        ResolutionArtifactObjectInputV0::new(artifact_key.clone(), b"a"),
        ResolutionArtifactObjectInputV0::new(artifact_key.clone(), b"b"),
    ];
    let bundles = [
        ResolutionForeignBundleObjectInputV0::new(bundle_key.clone(), b"a"),
        ResolutionForeignBundleObjectInputV0::new(bundle_key, b"b"),
    ];
    assert_eq!(
        ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap_err(),
        ResolutionContentClosureConstructionErrorV0::ConflictingArtifactObject {
            key: artifact_key,
        }
    );
}

#[test]
fn order_and_duplicate_interleaving_do_not_change_the_closure() {
    let artifact_a = artifact_key("algorithm-a", "digest-a");
    let artifact_b = artifact_key("algorithm-b", "digest-b");
    let bundle_a = simple_selector("a");
    let bundle_b = simple_selector("b");

    let forward_artifacts = [
        ResolutionArtifactObjectInputV0::new(artifact_a.clone(), b"artifact-a"),
        ResolutionArtifactObjectInputV0::new(artifact_b.clone(), b"artifact-b"),
        ResolutionArtifactObjectInputV0::new(artifact_a.clone(), b"artifact-a"),
    ];
    let reverse_artifacts = [
        ResolutionArtifactObjectInputV0::new(artifact_a.clone(), b"artifact-a"),
        ResolutionArtifactObjectInputV0::new(artifact_a.clone(), b"artifact-a"),
        ResolutionArtifactObjectInputV0::new(artifact_b.clone(), b"artifact-b"),
    ];
    let forward_bundles = [
        ResolutionForeignBundleObjectInputV0::new(bundle_a.clone(), b"bundle-a"),
        ResolutionForeignBundleObjectInputV0::new(bundle_b.clone(), b"bundle-b"),
        ResolutionForeignBundleObjectInputV0::new(bundle_a.clone(), b"bundle-a"),
    ];
    let reverse_bundles = [
        ResolutionForeignBundleObjectInputV0::new(bundle_a.clone(), b"bundle-a"),
        ResolutionForeignBundleObjectInputV0::new(bundle_a.clone(), b"bundle-a"),
        ResolutionForeignBundleObjectInputV0::new(bundle_b.clone(), b"bundle-b"),
    ];

    let forward =
        ResolutionContentClosureV0::construct(&forward_artifacts, &forward_bundles).unwrap();
    let reverse =
        ResolutionContentClosureV0::construct(&reverse_artifacts, &reverse_bundles).unwrap();
    assert_equivalent_closures(
        &forward,
        &reverse,
        &[artifact_a, artifact_b],
        &[bundle_a, bundle_b],
    );
}

#[test]
fn caller_buffers_are_copied_only_into_successful_immutable_closure() {
    let artifact_key = artifact_key("sha256", "caller-artifact");
    let bundle_key = simple_selector("caller-bundle");
    let mut artifact_buffer = b"artifact-original".to_vec();
    let mut bundle_buffer = b"bundle-original".to_vec();

    let closure = {
        let artifacts = [ResolutionArtifactObjectInputV0::new(
            artifact_key.clone(),
            &artifact_buffer,
        )];
        let bundles = [ResolutionForeignBundleObjectInputV0::new(
            bundle_key.clone(),
            &bundle_buffer,
        )];
        ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap()
    };
    let manifest_before = closure.canonical_manifest_bytes().to_vec();
    let identity_before = closure.identity().clone();
    let counts_before = (closure.artifact_count(), closure.foreign_bundle_count());

    artifact_buffer.fill(b'x');
    bundle_buffer.fill(b'y');
    assert_eq!(
        closure.artifact(&artifact_key),
        Some(b"artifact-original".as_slice())
    );
    assert_eq!(
        closure.foreign_bundle(&bundle_key),
        Some(b"bundle-original".as_slice())
    );
    assert_eq!(closure.canonical_manifest_bytes(), manifest_before);
    assert_eq!(closure.identity(), &identity_before);
    assert_eq!(
        (closure.artifact_count(), closure.foreign_bundle_count()),
        counts_before
    );
    assert_success_invariant(&closure);
}

#[test]
fn exact_lookup_mismatched_expected_keys_and_unusual_algorithms_are_inert() {
    let bytes = b"payload whose digest differs";
    let actual = sha256_hex(bytes);
    let expected_artifact_key = artifact_key("sha256", "expected-D-not-X");
    let bundle_key = selector(
        "unusual-bundle",
        "expected-R-not-X",
        "unsupported-witness-algorithm",
        "unusual-profile",
        "unusual-run",
    );
    let artifacts = [ResolutionArtifactObjectInputV0::new(
        expected_artifact_key.clone(),
        bytes,
    )];
    let bundles = [ResolutionForeignBundleObjectInputV0::new(
        bundle_key.clone(),
        bytes,
    )];
    let closure = ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap();

    assert_eq!(
        closure.artifact(&expected_artifact_key),
        Some(bytes.as_slice())
    );
    assert_eq!(closure.foreign_bundle(&bundle_key), Some(bytes.as_slice()));
    let changed_artifact = artifact_key("sha256", "changed");
    let changed_bundle = selector(
        "unusual-bundle",
        "expected-R-not-X",
        "unsupported-witness-algorithm",
        "unusual-profile",
        "changed-run",
    );
    assert_eq!(closure.artifact(&changed_artifact), None);
    assert_eq!(closure.foreign_bundle(&changed_bundle), None);
    assert_eq!(closure.artifact(&artifact_key("missing", "missing")), None);
    assert_eq!(closure.foreign_bundle(&simple_selector("missing")), None);

    let manifest = std::str::from_utf8(closure.canonical_manifest_bytes()).unwrap();
    assert!(manifest.contains(r#""digest":"expected-D-not-X""#));
    assert!(manifest.contains(r#""witness_root":"expected-R-not-X""#));
    assert!(manifest.contains(r#""witness_algorithm":"unsupported-witness-algorithm""#));
    assert!(manifest.matches(&actual).count() >= 2);
    assert!(!manifest.contains("verified"));

    let manifest_before = closure.canonical_manifest_bytes().to_vec();
    let identity_before = closure.identity().clone();
    assert_eq!(
        closure.artifact(&expected_artifact_key),
        Some(bytes.as_slice())
    );
    assert_eq!(closure.foreign_bundle(&bundle_key), Some(bytes.as_slice()));
    assert_eq!(closure.canonical_manifest_bytes(), manifest_before);
    assert_eq!(closure.identity(), &identity_before);

    let _artifact_api: for<'a> fn(
        &'a ResolutionContentClosureV0,
        &ResolutionArtifactObjectKeyV0,
    ) -> Option<&'a [u8]> = ResolutionContentClosureV0::artifact;
    let _bundle_api: for<'a> fn(
        &'a ResolutionContentClosureV0,
        &ArtifactProvenanceAnchorSelectorV0,
    ) -> Option<&'a [u8]> = ResolutionContentClosureV0::foreign_bundle;
    assert_success_invariant(&closure);
}

#[test]
fn canonical_string_encoding_is_exact_and_non_normalizing() {
    let algorithm = "\"\\/\u{0008}\t\n\u{000c}\r\u{0001}é";
    let digest = "e\u{0301}";
    let key = artifact_key(algorithm, digest);
    let inputs = [ResolutionArtifactObjectInputV0::new(key, b"")];
    let closure = ResolutionContentClosureV0::construct(&inputs, &[]).unwrap();
    let expected = concat!(
        r#"{"schema":"magpie-resolution-content-closure-v0","artifact_objects":[{"key":{"algorithm":"\"\\/\b\t\n\f\r\u0001é","digest":"e"#,
        "\u{0301}",
        r#""},"byte_length":0,"content_sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}],"foreign_bundle_objects":[]}"#
    );
    assert_eq!(closure.canonical_manifest_bytes(), expected.as_bytes());
    assert!(expected.contains('/'));
    assert!(expected.contains(r#"\"\\/\b"#));
    assert!(expected.contains(r"\u0001"));
    assert!(!expected.contains(r"\u000A"));
    assert!(expected.contains('é'));
    assert!(expected.contains("e\u{0301}"));
    assert!(!expected.contains(' '));
    assert_manifest_envelope(&closure);
    assert_success_invariant(&closure);
}

fn assert_normative_vector(
    closure: &ResolutionContentClosureV0,
    expected_manifest: &str,
    expected_sha256: &str,
) {
    assert_eq!(
        closure.canonical_manifest_bytes(),
        expected_manifest.as_bytes()
    );
    assert_eq!(
        closure.canonical_manifest_bytes().len(),
        expected_manifest.len()
    );
    assert_eq!(
        sha256_hex(closure.canonical_manifest_bytes()),
        expected_sha256
    );
    assert_identity(closure, expected_sha256);
    assert_manifest_envelope(closure);
    assert_success_invariant(closure);
}

#[test]
fn empty_normative_vector_is_exact() {
    let closure = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
    assert!(closure.is_empty());
    assert_eq!(closure.artifact_count(), 0);
    assert_eq!(closure.foreign_bundle_count(), 0);
    assert_eq!(closure.supplied_object_bytes(), 0);
    assert_eq!(closure.retained_object_bytes(), 0);
    assert_eq!(EMPTY_MANIFEST.len(), 99);
    assert_normative_vector(&closure, EMPTY_MANIFEST, EMPTY_MANIFEST_SHA256);

    let serialized_identity = serde_json::to_vec(closure.identity()).unwrap();
    assert_ne!(serialized_identity, closure.canonical_manifest_bytes());
    let second = ResolutionContentClosureV0::construct(&[], &[]).unwrap();
    assert_eq!(second.identity(), closure.identity());
    assert_success_invariant(&second);
}

#[test]
fn acquisition_normative_vector_is_exact() {
    let artifact_key = artifact_key("sha256", ACQUISITION_DIGEST);
    let bundle_key = acquisition_selector();
    let artifacts = [ResolutionArtifactObjectInputV0::new(
        artifact_key.clone(),
        ACQUISITION_ARTIFACT,
    )];
    let bundles = [ResolutionForeignBundleObjectInputV0::new(
        bundle_key.clone(),
        ACQUISITION_BUNDLE,
    )];
    let closure = ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap();
    assert_eq!(closure.artifact_count(), 1);
    assert_eq!(closure.foreign_bundle_count(), 1);
    assert_eq!(closure.supplied_object_bytes(), 306);
    assert_eq!(closure.retained_object_bytes(), 306);
    assert_eq!(closure.artifact(&artifact_key), Some(ACQUISITION_ARTIFACT));
    assert_eq!(
        closure.foreign_bundle(&bundle_key),
        Some(ACQUISITION_BUNDLE)
    );
    assert_eq!(ACQUISITION_MANIFEST.len(), 663);
    assert_normative_vector(&closure, ACQUISITION_MANIFEST, ACQUISITION_MANIFEST_SHA256);
}

#[test]
fn complete_five_object_normative_vector_is_exact() {
    let derived_key = artifact_key("sha256", DERIVED_DIGEST);
    let acquisition_key = artifact_key("sha256", ACQUISITION_DIGEST);
    let parent_key = artifact_key("sha256", PARENT_DIGEST);
    let acquisition_bundle_key = acquisition_selector();
    let derivation_bundle_key = derivation_selector();
    let artifacts = [
        ResolutionArtifactObjectInputV0::new(parent_key.clone(), DERIVATION_PARENT),
        ResolutionArtifactObjectInputV0::new(acquisition_key.clone(), ACQUISITION_ARTIFACT),
        ResolutionArtifactObjectInputV0::new(derived_key.clone(), DERIVATION_DERIVED),
    ];
    let bundles = [
        ResolutionForeignBundleObjectInputV0::new(derivation_bundle_key.clone(), DERIVATION_BUNDLE),
        ResolutionForeignBundleObjectInputV0::new(
            acquisition_bundle_key.clone(),
            ACQUISITION_BUNDLE,
        ),
    ];
    let closure = ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap();
    assert_eq!(closure.artifact_count(), 3);
    assert_eq!(closure.foreign_bundle_count(), 2);
    assert_eq!(closure.supplied_object_bytes(), 692);
    assert_eq!(closure.retained_object_bytes(), 692);
    assert_eq!(closure.artifact(&derived_key), Some(DERIVATION_DERIVED));
    assert_eq!(
        closure.artifact(&acquisition_key),
        Some(ACQUISITION_ARTIFACT)
    );
    assert_eq!(closure.artifact(&parent_key), Some(DERIVATION_PARENT));
    assert_eq!(
        closure.foreign_bundle(&acquisition_bundle_key),
        Some(ACQUISITION_BUNDLE)
    );
    assert_eq!(
        closure.foreign_bundle(&derivation_bundle_key),
        Some(DERIVATION_BUNDLE)
    );
    assert_eq!(COMPLETE_MANIFEST.len(), 1_434);
    assert_normative_vector(&closure, COMPLETE_MANIFEST, COMPLETE_MANIFEST_SHA256);
}

#[test]
fn closure_and_input_debug_never_disclose_raw_object_bytes() {
    let sentinel = b"DISTINCTIVE-RAW-PAYLOAD-SENTINEL";
    let artifact_key = artifact_key("sha256", "sentinel-artifact");
    let bundle_key = simple_selector("sentinel-bundle");
    let artifact_input = ResolutionArtifactObjectInputV0::new(artifact_key.clone(), sentinel);
    let bundle_input = ResolutionForeignBundleObjectInputV0::new(bundle_key.clone(), sentinel);

    let artifact_debug = format!("{artifact_input:?}");
    let bundle_debug = format!("{bundle_input:?}");
    assert!(artifact_debug.contains("byte_len: 32"));
    assert!(bundle_debug.contains("byte_len: 32"));
    assert!(!artifact_debug.contains("DISTINCTIVE-RAW-PAYLOAD-SENTINEL"));
    assert!(!bundle_debug.contains("DISTINCTIVE-RAW-PAYLOAD-SENTINEL"));

    let closure =
        ResolutionContentClosureV0::construct(&[artifact_input], &[bundle_input]).unwrap();
    let debug = format!("{closure:?}");
    assert!(debug.contains("artifact_count: 1"));
    assert!(debug.contains("foreign_bundle_count: 1"));
    assert!(!debug.contains("DISTINCTIVE-RAW-PAYLOAD-SENTINEL"));
    assert_success_invariant(&closure);
}

#[test]
fn construction_is_standing_inert_and_does_not_change_existing_verifier_behavior() {
    const CLAIM_ID: &str = "closure-standing-inert";
    let selector = acquisition_selector();
    let store = MemStore::new();
    let signing_key = SigningKey::from_bytes(&[41; 32]);
    {
        let mut timestamp = 100;
        let mut writer = LogWriter::<MemStore>::open_with_clock(
            store.clone(),
            signing_key.clone(),
            Box::new(move || {
                timestamp += 1;
                timestamp
            }),
        )
        .unwrap();
        writer
            .append(
                Provenance::new("closure-test", "standing-inert"),
                Payload::SegmentAnchored {
                    bundle_kind: selector.bundle_kind().to_owned(),
                    witness_root: selector.witness_root().to_owned(),
                    witness_algorithm: selector.witness_algorithm().to_owned(),
                    canonicalization_profile: selector.canonicalization_profile().to_owned(),
                    run_id: selector.run_id().to_owned(),
                },
            )
            .unwrap();
        writer
            .append(
                Provenance::new("closure-test", "standing-inert"),
                Payload::ClaimAssertedV2 {
                    claim_id: CLAIM_ID.to_owned(),
                    statement: "closure construction has no standing effect".to_owned(),
                    scope_ref: "scope:closure".to_owned(),
                    actor_class: "AgentProposer".to_owned(),
                    content_hash: String::new(),
                    metadata_json: r#"{"claim_domain":"Interpretation"}"#.to_owned(),
                },
            )
            .unwrap();
    }
    let reader = LogReader::open(store, signing_key.verifying_key());
    let snapshot = replay_standing_context(&reader).unwrap();
    let snapshot_before = snapshot.canonical_bytes();
    let v0_before = snapshot.standing().resolved_standing_with_trace(CLAIM_ID);
    let v1_before = snapshot.resolved_standing_with_trace_v1(CLAIM_ID);
    let v2_before = snapshot.resolved_standing_with_trace_v2(CLAIM_ID);
    let verifier_before = snapshot.resolve_artifact_acquisition_context_v0(
        &selector,
        Some(ACQUISITION_BUNDLE),
        Some(ACQUISITION_ARTIFACT),
    );

    let artifact_key = artifact_key("sha256", ACQUISITION_DIGEST);
    let artifacts = [ResolutionArtifactObjectInputV0::new(
        artifact_key,
        ACQUISITION_ARTIFACT,
    )];
    let bundles = [ResolutionForeignBundleObjectInputV0::new(
        selector.clone(),
        ACQUISITION_BUNDLE,
    )];
    let closure = ResolutionContentClosureV0::construct(&artifacts, &bundles).unwrap();
    assert_success_invariant(&closure);

    assert_eq!(snapshot.canonical_bytes(), snapshot_before);
    assert_eq!(
        snapshot.standing().resolved_standing_with_trace(CLAIM_ID),
        v0_before
    );
    assert_eq!(
        snapshot.resolved_standing_with_trace_v1(CLAIM_ID),
        v1_before
    );
    assert_eq!(
        snapshot.resolved_standing_with_trace_v2(CLAIM_ID),
        v2_before
    );
    assert_eq!(
        snapshot.resolve_artifact_acquisition_context_v0(
            &selector,
            Some(ACQUISITION_BUNDLE),
            Some(ACQUISITION_ARTIFACT),
        ),
        verifier_before
    );
    assert_eq!(v0_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v1_before.governed_standing, Some(Status::Conjectured));
    assert_eq!(v2_before.governed_standing, Some(Status::Conjectured));
}
