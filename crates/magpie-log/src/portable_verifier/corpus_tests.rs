use std::convert::Infallible;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::signature_profile::V_SIG_PROFILE_ID;
use crate::{EventCore, Payload, Provenance, SignedEvent, Status};

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
    pending
        .complete_semantics::<Infallible>(Ok(()))
        .expect("the conspicuous frontend-only test driver cannot fail")
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
