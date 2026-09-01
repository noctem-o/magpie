use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::signature_profile::V_SIG_PROFILE_ID;
use crate::{ContentHash, EventCore, FileStore, Payload, Provenance, SignedEvent, Status};

use super::conformer::{
    verify_complete_history, CompleteHistoryOutcome, PortableRejection, PortableRejectionClass,
};
use super::file::{
    take_public_file_trace, verify_complete_file, FileVerificationError, FileVerificationTrace,
};
use super::frontend::{FrontendSession, FrontendStep, PendingRecord};
use super::preflight::FrontendStartError;
use super::{FrontendRejection, FrontendRejectionClass};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn required_str<'value>(value: &'value Value, member: &str) -> &'value str {
    value[member]
        .as_str()
        .unwrap_or_else(|| panic!("manifest member {member:?} must be a string"))
}

fn optional_usize(value: &Value) -> Option<usize> {
    value.as_u64().map(|coordinate| {
        usize::try_from(coordinate).expect("frozen corpus coordinate must fit usize")
    })
}

fn expected_complete_class(name: &str) -> PortableRejectionClass {
    match name {
        "ExternalKey" => PortableRejectionClass::ExternalKey,
        "Framing" => PortableRejectionClass::Framing,
        "JsonSyntax" => PortableRejectionClass::JsonSyntax,
        "Schema" => PortableRejectionClass::Schema,
        "Sequence" => PortableRejectionClass::Sequence,
        "PreviousLink" => PortableRejectionClass::PreviousLink,
        "ContentHash" => PortableRejectionClass::ContentHash,
        "Signature" => PortableRejectionClass::Signature,
        "PayloadValidation" => PortableRejectionClass::PayloadValidation,
        "Genesis" => PortableRejectionClass::Genesis,
        _ => panic!("unknown portable rejection class {name:?}"),
    }
}

fn complete_class_name(class: PortableRejectionClass) -> &'static str {
    match class {
        PortableRejectionClass::ExternalKey => "ExternalKey",
        PortableRejectionClass::Framing => "Framing",
        PortableRejectionClass::JsonSyntax => "JsonSyntax",
        PortableRejectionClass::Schema => "Schema",
        PortableRejectionClass::Sequence => "Sequence",
        PortableRejectionClass::PreviousLink => "PreviousLink",
        PortableRejectionClass::ContentHash => "ContentHash",
        PortableRejectionClass::Signature => "Signature",
        PortableRejectionClass::PayloadValidation => "PayloadValidation",
        PortableRejectionClass::Genesis => "Genesis",
    }
}

fn assert_complete_rejection(case_id: &str, rejection: PortableRejection, expected: &Value) {
    assert_eq!(
        rejection.class(),
        expected_complete_class(required_str(expected, "class")),
        "{case_id}: class"
    );
    assert_eq!(
        rejection.line(),
        optional_usize(&expected["line"]),
        "{case_id}: line"
    );
    assert_eq!(
        rejection.record_index(),
        optional_usize(&expected["record_index"]),
        "{case_id}: record_index"
    );
}

fn read_case_input(root: &Path, case: &Value) -> Vec<u8> {
    let case_id = required_str(case, "id");
    let input = std::fs::read(root.join(required_str(case, "input_path")))
        .unwrap_or_else(|error| panic!("{case_id}: input read failed: {error}"));
    assert_eq!(
        hex::encode(Sha256::digest(&input)),
        required_str(case, "input_sha256"),
        "{case_id}: exact input bytes drifted"
    );
    input
}

fn assert_same_file_image(
    case_id: &str,
    manifest_image: &[u8],
    outcome: &CompleteHistoryOutcome,
    acquired_image: Option<&[u8]>,
) {
    match (outcome, acquired_image) {
        (CompleteHistoryOutcome::Reject(rejection), None)
            if rejection.class() == PortableRejectionClass::ExternalKey => {}
        (CompleteHistoryOutcome::Reject(rejection), Some(_))
            if rejection.class() == PortableRejectionClass::ExternalKey =>
        {
            panic!("{case_id}: external-key rejection acquired the history")
        }
        (_, Some(acquired_image)) => assert_eq!(
            acquired_image, manifest_image,
            "{case_id}: verified file image differed from manifest-committed bytes"
        ),
        (_, None) => panic!("{case_id}: non-key outcome did not acquire the history"),
    }
}

fn verify_public_path_with_trace(
    case_id: &str,
    store: &FileStore,
    external_key_text: &str,
) -> FileVerificationTrace {
    let accepted = store
        .verify_portable_history(external_key_text)
        .unwrap_or_else(|error| panic!("{case_id}: public path operational failure: {error}"));
    let trace = take_public_file_trace()
        .unwrap_or_else(|| panic!("{case_id}: public path produced no test observation"));
    assert_eq!(
        accepted,
        matches!(trace.outcome, CompleteHistoryOutcome::Accept(_)),
        "{case_id}: public file path verdict differed from governed outcome"
    );
    trace
}

fn assert_complete_case(root: &Path, case: &Value) -> Option<PortableRejectionClass> {
    let case_id = required_str(case, "id");
    let expected = &case["expected"];
    // This is harness-only oracle preflight. The public verifier call does not
    // receive these bytes and completes its key gate before acquiring its own
    // FileStore image.
    let input_hash_preflight = read_case_input(root, case);
    let store = FileStore::new(root.join(required_str(case, "input_path")));
    let FileVerificationTrace {
        outcome,
        hashes: observed_hashes,
        acquired_image,
    } = verify_public_path_with_trace(
        case_id,
        &store,
        required_str(case, "external_verifying_key_hex"),
    );
    assert_same_file_image(
        case_id,
        &input_hash_preflight,
        &outcome,
        acquired_image.as_deref(),
    );
    assert_eq!(
        read_case_input(root, case),
        input_hash_preflight,
        "{case_id}: case bytes changed while exercising the public file path"
    );

    match (required_str(expected, "verdict"), outcome) {
        ("ACCEPT", CompleteHistoryOutcome::Accept(accepted)) => {
            assert_eq!(
                accepted.event_count(),
                expected["event_count"].as_u64().unwrap(),
                "{case_id}: event_count"
            );
            assert_eq!(
                accepted.tip().to_hex(),
                required_str(expected, "tip"),
                "{case_id}: tip"
            );
            let observed_hashes: Vec<String> =
                observed_hashes.iter().map(ContentHash::to_hex).collect();
            let expected_hashes: Vec<&str> = expected["ordered_recomputed_hashes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|hash| hash.as_str().unwrap())
                .collect();
            assert_eq!(
                observed_hashes, expected_hashes,
                "{case_id}: ordered recomputed hashes"
            );
            None
        }
        ("REJECT", CompleteHistoryOutcome::Reject(rejection)) => {
            assert_complete_rejection(case_id, rejection, expected);
            Some(rejection.class())
        }
        (expected_verdict, actual) => {
            panic!("{case_id}: expected {expected_verdict}, got {actual:?}")
        }
    }
}

fn manifest_case<'manifest>(manifest: &'manifest Value, case_id: &str) -> &'manifest Value {
    manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| required_str(case, "id") == case_id)
        .unwrap_or_else(|| panic!("missing frozen case {case_id:?}"))
}

fn assert_named_cases(case_ids: &[&str]) {
    let root = repository_root();
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap(),
    )
    .unwrap();
    for case_id in case_ids {
        let _ = assert_complete_case(&root, manifest_case(&manifest, case_id));
    }
}

fn expected_frontend_class(name: &str) -> FrontendRejectionClass {
    match name {
        "ExternalKey" => FrontendRejectionClass::ExternalKey,
        "Framing" => FrontendRejectionClass::Framing,
        "JsonSyntax" => FrontendRejectionClass::JsonSyntax,
        "Schema" => FrontendRejectionClass::Schema,
        _ => panic!("{name:?} is not a frontend-owned class"),
    }
}

fn assert_rejection(case_id: &str, rejection: FrontendRejection, expected: &Value) {
    assert_eq!(
        rejection.class(),
        expected_frontend_class(required_str(expected, "class")),
        "{case_id}: class"
    );
    assert_eq!(
        rejection.line(),
        optional_usize(&expected["line"]),
        "{case_id}: line"
    );
    assert_eq!(
        rejection.record_index(),
        optional_usize(&expected["record_index"]),
        "{case_id}: record_index"
    );
}

fn assume_current_semantics_succeed_for_frontend_test(
    pending: Box<PendingRecord<'_>>,
) -> FrontendSession<'_> {
    pending.assume_semantics_succeeded_for_frontend_test()
}

fn drive_frontend_final_case(case_id: &str, case: &Value, input: &[u8]) {
    let expected = &case["expected"];
    let start = FrontendSession::new(
        V_SIG_PROFILE_ID,
        required_str(case, "external_verifying_key_hex"),
        input,
    );

    if expected["class"] == "ExternalKey" {
        match start {
            Err(FrontendStartError::Rejected(rejection)) => {
                assert_rejection(case_id, rejection, expected)
            }
            Err(FrontendStartError::UnsupportedProfile(_)) => {
                panic!("{case_id}: exact profile unexpectedly unsupported")
            }
            Ok(_) => panic!("{case_id}: expected pre-framing ExternalKey rejection"),
        }
        return;
    }

    let mut session = start.unwrap_or_else(|error| panic!("{case_id}: start failed: {error:?}"));
    loop {
        match session
            .next()
            .unwrap_or_else(|error| panic!("{case_id}: operational failure: {error:?}"))
        {
            FrontendStep::Rejected(rejection) => {
                assert_rejection(case_id, rejection, expected);
                return;
            }
            FrontendStep::Pending(pending) => {
                session = assume_current_semantics_succeed_for_frontend_test(pending);
            }
            FrontendStep::End => {
                panic!("{case_id}: reached end before expected frontend rejection")
            }
        }
    }
}

fn drive_later_case_without_preemption(case_id: &str, case: &Value, input: &[u8]) {
    let expected = &case["expected"];
    let mut session = FrontendSession::new(
        V_SIG_PROFILE_ID,
        required_str(case, "external_verifying_key_hex"),
        input,
    )
    .unwrap_or_else(|error| panic!("{case_id}: frontend stole result at start: {error:?}"));

    if expected["verdict"] == "ACCEPT" {
        let mut observed_records = 0_u64;
        loop {
            match session
                .next()
                .unwrap_or_else(|error| panic!("{case_id}: operational failure: {error:?}"))
            {
                FrontendStep::End => break,
                FrontendStep::Rejected(rejection) => {
                    panic!("{case_id}: frontend stole ACCEPT with {rejection:?}")
                }
                FrontendStep::Pending(pending) => {
                    assert_eq!(
                        pending.record().record_index(),
                        usize::try_from(observed_records).unwrap(),
                        "{case_id}: record index"
                    );
                    observed_records += 1;
                    session = assume_current_semantics_succeed_for_frontend_test(pending);
                }
            }
        }
        assert_eq!(
            observed_records,
            expected["event_count"].as_u64().unwrap(),
            "{case_id}: frontend record count"
        );
        return;
    }

    let target_index = optional_usize(&expected["record_index"])
        .unwrap_or_else(|| panic!("{case_id}: later rejection lacks record_index"));
    loop {
        match session
            .next()
            .unwrap_or_else(|error| panic!("{case_id}: operational failure: {error:?}"))
        {
            FrontendStep::End => panic!("{case_id}: ended before later semantic record"),
            FrontendStep::Rejected(rejection) => {
                panic!("{case_id}: frontend stole later result with {rejection:?}")
            }
            FrontendStep::Pending(pending) => {
                let index = pending.record().record_index();
                if index == target_index {
                    assert_eq!(
                        pending.record().line(),
                        optional_usize(&expected["line"]).unwrap(),
                        "{case_id}: later-stage line"
                    );
                    // Deliberately stop here. Inspecting the next record before
                    // this record's governed semantic result would violate the
                    // first-failure law being tested.
                    return;
                }
                assert!(index < target_index, "{case_id}: skipped semantic target");
                session = assume_current_semantics_succeed_for_frontend_test(pending);
            }
        }
    }
}

#[test]
fn frozen_frontend_corpus_accounting_is_exact() {
    let root = repository_root();
    let manifest_bytes =
        std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap();
    let manifest: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    assert_eq!(
        manifest["producing_context"]["signature_profile_identity"],
        V_SIG_PROFILE_ID
    );

    let cases = manifest["cases"].as_array().unwrap();
    let mut external_key = 0_usize;
    let mut framing = 0_usize;
    let mut json_syntax = 0_usize;
    let mut schema = 0_usize;
    let mut eventual_accept = 0_usize;
    let mut later_reject = 0_usize;

    for case in cases {
        let case_id = required_str(case, "id");
        let input = std::fs::read(root.join(required_str(case, "input_path")))
            .unwrap_or_else(|error| panic!("{case_id}: input read failed: {error}"));
        assert_eq!(
            hex::encode(Sha256::digest(&input)),
            required_str(case, "input_sha256"),
            "{case_id}: exact input bytes drifted"
        );

        match case["expected"]["class"].as_str() {
            Some("ExternalKey") => external_key += 1,
            Some("Framing") => framing += 1,
            Some("JsonSyntax") => json_syntax += 1,
            Some("Schema") => schema += 1,
            Some(_) => later_reject += 1,
            None => eventual_accept += 1,
        }

        if matches!(
            case["expected"]["class"].as_str(),
            Some("ExternalKey" | "Framing" | "JsonSyntax" | "Schema")
        ) {
            drive_frontend_final_case(case_id, case, &input);
        } else {
            drive_later_case_without_preemption(case_id, case, &input);
        }
    }

    assert_eq!(cases.len(), 432);
    assert_eq!(
        (external_key, framing, json_syntax, schema),
        (21, 13, 18, 272)
    );
    assert_eq!(external_key + framing + json_syntax + schema, 324);
    assert_eq!((eventual_accept, later_reject), (19, 89));
    assert_eq!(eventual_accept + later_reject, 108);
}

#[test]
fn frozen_complete_conformer_corpus_matches_all_432_cases() {
    const MANIFEST_SHA256: &str =
        "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81";

    let root = repository_root();
    let manifest_bytes =
        std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap();
    assert_eq!(
        hex::encode(Sha256::digest(&manifest_bytes)),
        MANIFEST_SHA256
    );
    let manifest: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    assert_eq!(
        manifest["producing_context"]["signature_profile_identity"],
        V_SIG_PROFILE_ID
    );

    let cases = manifest["cases"].as_array().unwrap();
    let mut inventory = [0_usize; 11];
    for case in cases {
        // Every case takes the same production path. The manifest expectation
        // is consulted only after the conformer returns its complete outcome.
        let actual_class = assert_complete_case(&root, case);
        let inventory_index = match actual_class {
            None => 0,
            Some(PortableRejectionClass::ExternalKey) => 1,
            Some(PortableRejectionClass::Framing) => 2,
            Some(PortableRejectionClass::JsonSyntax) => 3,
            Some(PortableRejectionClass::Schema) => 4,
            Some(PortableRejectionClass::Sequence) => 5,
            Some(PortableRejectionClass::PreviousLink) => 6,
            Some(PortableRejectionClass::ContentHash) => 7,
            Some(PortableRejectionClass::Signature) => 8,
            Some(PortableRejectionClass::PayloadValidation) => 9,
            Some(PortableRejectionClass::Genesis) => 10,
        };
        inventory[inventory_index] += 1;
    }

    assert_eq!(cases.len(), 432);
    assert_eq!(inventory, [19, 21, 13, 18, 272, 4, 5, 3, 16, 46, 15]);
    eprintln!(
        "complete conformer inventory: ACCEPT={} ExternalKey={} Framing={} JsonSyntax={} Schema={} Sequence={} PreviousLink={} ContentHash={} Signature={} PayloadValidation={} Genesis={}",
        inventory[0],
        inventory[1],
        inventory[2],
        inventory[3],
        inventory[4],
        inventory[5],
        inventory[6],
        inventory[7],
        inventory[8],
        inventory[9],
        inventory[10],
    );
}

/// Export actual production-conformer results for the external differential
/// harness without adding a public portable-result type to `magpie-log`.
///
/// Ordinary test runs do no I/O. The harness supplies an explicit output path
/// through `MAGPIE_PORTABLE_DIFFERENTIAL_OUTPUT` and invokes only this exact
/// test. The export reads paths and external keys from the frozen manifest but
/// never uses its expected result fields to construct an observed result.
#[test]
fn export_complete_conformer_results_when_requested() {
    const MANIFEST_SHA256: &str =
        "7d758d3f2dac1161fe15dd064b130ccbfcdaf0437ea8ab8d7772492194801a81";

    let Some(output_path) = std::env::var_os("MAGPIE_PORTABLE_DIFFERENTIAL_OUTPUT") else {
        return;
    };
    let root = repository_root();
    let manifest_bytes =
        std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap();
    assert_eq!(
        hex::encode(Sha256::digest(&manifest_bytes)),
        MANIFEST_SHA256
    );
    let manifest: Value = serde_json::from_slice(&manifest_bytes).unwrap();
    assert_eq!(
        manifest["producing_context"]["signature_profile_identity"],
        V_SIG_PROFILE_ID
    );

    let mut results = Vec::new();
    for case in manifest["cases"].as_array().unwrap() {
        let case_id = required_str(case, "id");
        // Harness-only oracle preflight; the public file path retains its own
        // key-before-acquisition ordering.
        let input_hash_preflight = read_case_input(&root, case);
        let store = FileStore::new(root.join(required_str(case, "input_path")));
        let FileVerificationTrace {
            outcome,
            hashes,
            acquired_image,
        } = verify_public_path_with_trace(
            case_id,
            &store,
            required_str(case, "external_verifying_key_hex"),
        );
        assert_same_file_image(
            case_id,
            &input_hash_preflight,
            &outcome,
            acquired_image.as_deref(),
        );
        assert_eq!(
            read_case_input(&root, case),
            input_hash_preflight,
            "{case_id}: case bytes changed while exercising the public file path"
        );

        let observed = match outcome {
            CompleteHistoryOutcome::Accept(accepted) => serde_json::json!({
                "verdict": "ACCEPT",
                "class": Value::Null,
                "line": Value::Null,
                "record_index": Value::Null,
                "event_count": accepted.event_count(),
                "tip": accepted.tip().to_hex(),
                "ordered_recomputed_hashes": hashes
                    .iter()
                    .map(ContentHash::to_hex)
                    .collect::<Vec<_>>(),
            }),
            CompleteHistoryOutcome::Reject(rejection) => serde_json::json!({
                "verdict": "REJECT",
                "class": complete_class_name(rejection.class()),
                "line": rejection.line(),
                "record_index": rejection.record_index(),
                "event_count": Value::Null,
                "tip": Value::Null,
                "ordered_recomputed_hashes": Vec::<String>::new(),
            }),
        };
        results.push(serde_json::json!({"id": case_id, "result": observed}));
    }

    let export = serde_json::json!({
        "manifest_identity": manifest["manifest_identity"],
        "manifest_sha256": MANIFEST_SHA256,
        "results": results,
    });
    let mut encoded = serde_json::to_vec_pretty(&export).unwrap();
    encoded.push(b'\n');
    std::fs::write(output_path, encoded).unwrap();
}

#[test]
fn file_adapter_preserves_framing_schema_and_raw_token_distinctions() {
    assert_named_cases(&[
        "n7-lf-only-empty-record",
        "n30-extra-final-terminator",
        "n30-terminal-lone-cr",
        "n8-duplicate-signedevent-core",
        "n10-unknown-signedevent",
        "n16-seq-negative-zero",
    ]);
}

#[test]
fn file_adapter_finishes_external_key_gate_before_file_acquisition() {
    let unreadable_as_history = FileStore::new(repository_root());
    let trace = verify_complete_file(
        &unreadable_as_history,
        "0100000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();
    match trace.outcome {
        CompleteHistoryOutcome::Reject(rejection) => {
            assert_eq!(rejection.class(), PortableRejectionClass::ExternalKey);
            assert_eq!(rejection.line(), None);
            assert_eq!(rejection.record_index(), None);
        }
        actual => panic!("inadmissible key must reject before file acquisition: {actual:?}"),
    }

    assert!(matches!(
        verify_complete_file(
            &unreadable_as_history,
            "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
        ),
        Err(FileVerificationError::Read(_))
    ));
}

#[test]
fn sequence_witnesses_cover_first_skip_duplicate_regression_and_high_u64() {
    let root = repository_root();
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap(),
    )
    .unwrap();
    let positive = manifest_case(&manifest, "p2-deadbolt-anchor-v1");
    let input = String::from_utf8(read_case_input(&root, positive)).unwrap();
    let external_key = required_str(positive, "external_verifying_key_hex");

    assert!(matches!(
        verify_complete_history(V_SIG_PROFILE_ID, external_key, input.as_bytes()).unwrap(),
        CompleteHistoryOutcome::Accept(_)
    ));

    for (name, mutated) in [
        ("skip", input.replacen("\"seq\":1", "\"seq\":2", 1)),
        (
            "duplicate/regression",
            input.replacen("\"seq\":1", "\"seq\":0", 1),
        ),
    ] {
        match verify_complete_history(V_SIG_PROFILE_ID, external_key, mutated.as_bytes()).unwrap() {
            CompleteHistoryOutcome::Reject(rejection) => {
                assert_eq!(
                    rejection.class(),
                    PortableRejectionClass::Sequence,
                    "{name}"
                );
                assert_eq!(rejection.line(), Some(2), "{name}");
                assert_eq!(rejection.record_index(), Some(1), "{name}");
            }
            actual => panic!("{name}: expected Sequence, got {actual:?}"),
        }
    }

    assert_named_cases(&[
        "n21-wrong-sequence",
        "u64-p4-sequence-9223372036854775808",
        "u64-p5-sequence-18446744073709551615",
    ]);
}

#[test]
fn previous_link_uses_only_the_last_completely_accepted_recomputed_tip() {
    assert_named_cases(&[
        "n22-wrong-genesis-previous-link",
        "n22-wrong-later-previous-link",
        "n28-later-link-before-signature",
        "n29-valid-prefix-bad-link-later",
    ]);
}

#[test]
fn content_hash_and_signature_use_canonical_core_and_the_bound_profile() {
    assert_named_cases(&[
        "p7-member-order-permutation",
        "n23-wrong-stored-hash",
        "n28-content-hash-before-payload",
        "a21-altered-message",
        "a21-altered-signature",
        "a21-d2-identity-r-equation-true",
        "n28-signature-before-payload",
        "n28-signature-before-genesis",
    ]);
}

#[test]
fn payload_and_genesis_rules_remain_at_their_frozen_late_stages() {
    assert_named_cases(&[
        "payload-empty-claimassertedv2-claim-id",
        "n20-actor-unknown",
        "n20-evidence-unknown",
        "n20-edge-unknown",
        "n3-witness-root-nonhex",
        "n3-claim-content-hash-empty",
        "n28-payload-before-genesis",
        "n25-non-genesis-at-zero",
        "n25-genesis-after-zero",
        "n26-wrong-genesis-profile",
        "n3-genesis-key-nonhex",
        "n1-genesis-key-uppercase",
        "n27-genesis-key-mismatch",
        "positive-complete-vocabulary",
    ]);
}

#[test]
fn current_semantic_failure_beats_malformed_next_record() {
    let root = repository_root();
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(root.join("fixtures/verifier-language-v1/manifest.json")).unwrap(),
    )
    .unwrap();
    let case = manifest_case(&manifest, "n21-wrong-sequence");
    let mut input = read_case_input(&root, case);
    while input.last() == Some(&b'\n') {
        input.pop();
    }
    input.extend_from_slice(b"\n{\n");

    match verify_complete_history(
        V_SIG_PROFILE_ID,
        required_str(case, "external_verifying_key_hex"),
        &input,
    )
    .unwrap()
    {
        CompleteHistoryOutcome::Reject(rejection) => {
            assert_eq!(rejection.class(), PortableRejectionClass::Sequence);
            assert_eq!(rejection.line(), Some(1));
            assert_eq!(rejection.record_index(), Some(0));
        }
        actual => panic!("current Sequence failure was preempted: {actual:?}"),
    }
}

#[test]
fn exact_empty_history_accepts_zero_count_and_zero_tip_after_preflight() {
    let outcome = verify_complete_history(
        V_SIG_PROFILE_ID,
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
        b"",
    )
    .unwrap();
    match outcome {
        CompleteHistoryOutcome::Accept(accepted) => {
            assert_eq!(accepted.event_count(), 0);
            assert_eq!(accepted.tip(), ContentHash::ZERO);
        }
        actual => panic!("empty history must ACCEPT after valid preflight: {actual:?}"),
    }
}

// This compile-only coupling is intentionally exhaustive and uses no `..` or
// wildcard. A governed field or payload variant addition must update the
// frontend schema and frozen corpus inventory before this test can compile.
#[allow(dead_code)]
fn current_rust_schema_is_exhaustively_coupled(event: SignedEvent) {
    let SignedEvent {
        core,
        hash,
        signature,
    } = event;
    let EventCore {
        seq,
        timestamp_nanos,
        prev_hash,
        provenance,
        payload,
    } = core;
    let Provenance { agent, source } = provenance;
    let _ = (
        seq,
        timestamp_nanos,
        prev_hash,
        agent,
        source,
        hash,
        signature,
    );

    match payload {
        Payload::Genesis {
            canonicalization_profile,
            verifying_key,
        } => drop((canonicalization_profile, verifying_key)),
        Payload::ClaimAsserted {
            claim_id,
            statement,
            status,
        } => drop((claim_id, statement, status)),
        Payload::EvidenceRecorded { claim_id, summary } => drop((claim_id, summary)),
        Payload::ClaimStatusChanged {
            claim_id,
            from,
            to,
            reason,
        } => drop((claim_id, from, to, reason)),
        Payload::Note { text } => drop(text),
        Payload::SegmentAnchored {
            bundle_kind,
            witness_root,
            witness_algorithm,
            canonicalization_profile,
            run_id,
        } => drop((
            bundle_kind,
            witness_root,
            witness_algorithm,
            canonicalization_profile,
            run_id,
        )),
        Payload::ClaimAssertedV2 {
            claim_id,
            statement,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        } => drop((
            claim_id,
            statement,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        )),
        Payload::EvidenceRegistered {
            evidence_id,
            evidence_kind,
            summary,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        } => drop((
            evidence_id,
            evidence_kind,
            summary,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        )),
        Payload::JustificationEdgeRecorded {
            edge_id,
            edge_kind,
            source_id,
            target_id,
            scope_ref,
            actor_class,
            rationale,
            metadata_json,
        } => drop((
            edge_id,
            edge_kind,
            source_id,
            target_id,
            scope_ref,
            actor_class,
            rationale,
            metadata_json,
        )),
    }
}

#[allow(dead_code)]
fn current_status_vocabulary_is_exhaustively_coupled(status: Status) {
    match status {
        Status::Open
        | Status::Conjectured
        | Status::Supported
        | Status::Settled
        | Status::Refuted => {}
    }
}
