use magpie_claims::{
    check_standing_receipt_v0, produce_standing_receipt_v0, replay_standing_context,
    ProducedStandingReceiptV0, StandingReceiptCheckpointV0, StandingReceiptContextV0,
    StandingReceiptErrorV0, StandingReceiptExpectationV0, StandingReceiptHistoryIdentityV0,
    StandingReceiptRequestV0, MAGPIE_CLAIMS_POLICY_V2_ID, STANDING_RECEIPT_HISTORY_PROFILE_V0,
};
use magpie_log::{
    ContentHash, LogReader, LogWriter, MemStore, Payload, PortableRejectionClass, Provenance,
    SigningKey, Status,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

const CLAIM: &str = "claim-machine";
const SCOPE: &str = "scope:receipt";
const DIGEST: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

fn key() -> SigningKey {
    SigningKey::from_bytes(&[92; 32])
}

fn typed_claim(id: &str) -> Payload {
    let statement = format!("sha256_bytes_equals_v0:{DIGEST}");
    Payload::ClaimAssertedV2 {
        claim_id: id.into(),
        statement: statement.clone(),
        scope_ref: SCOPE.into(),
        actor_class: "AgentProposer".into(),
        content_hash: hex::encode(Sha256::digest(statement.as_bytes())),
        metadata_json: format!(
            r#"{{"claim_domain":"ExactMachineCheckable","machine_predicate":{{"schema":"magpie-machine-predicate-v0","predicate_id":"sha256_bytes_equals_v0","expected_sha256":"{DIGEST}"}}}}"#
        ),
    }
}

fn support_payloads() -> Vec<Payload> {
    vec![
        Payload::ClaimAsserted {
            claim_id: CLAIM.into(),
            statement: format!("sha256_bytes_equals_v0:{DIGEST}"),
            status: Status::Conjectured,
        },
        typed_claim(CLAIM),
        Payload::EvidenceRegistered {
            evidence_id: "evidence".into(),
            evidence_kind: "DeterministicVerification".into(),
            summary: "exact inline bytes".into(),
            scope_ref: SCOPE.into(),
            actor_class: "SourceImporter".into(),
            content_hash: String::new(),
            metadata_json: format!(
                r#"{{"verification_witness":{{"schema":"magpie-verification-witness-v0","predicate_id":"sha256_bytes_equals_v0","subject_claim_id":"{CLAIM}","scope_ref":"{SCOPE}","witness_hex":"616263"}}}}"#
            ),
        },
        Payload::JustificationEdgeRecorded {
            edge_id: "edge".into(),
            edge_kind: "supports".into(),
            source_id: "evidence".into(),
            target_id: CLAIM.into(),
            scope_ref: SCOPE.into(),
            actor_class: "HumanRoot".into(),
            rationale: "exact support candidate".into(),
            metadata_json: "{}".into(),
        },
    ]
}

fn records(payloads: Vec<Payload>) -> Vec<Vec<u8>> {
    let store = MemStore::new();
    let mut timestamp = 0;
    let mut writer = LogWriter::<MemStore>::open_with_clock(
        store.clone(),
        key(),
        Box::new(move || {
            timestamp += 1;
            timestamp
        }),
    )
    .unwrap();
    for payload in payloads {
        writer
            .append(Provenance::new("test", "receipt"), payload)
            .unwrap();
    }
    store.records()
}

fn image(records: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for record in records {
        bytes.extend_from_slice(record);
        bytes.push(b'\n');
    }
    bytes
}

fn request(bytes: &[u8]) -> StandingReceiptRequestV0 {
    StandingReceiptRequestV0 {
        context: StandingReceiptContextV0 {
            profile: STANDING_RECEIPT_HISTORY_PROFILE_V0.into(),
            history: StandingReceiptHistoryIdentityV0::of(bytes).unwrap(),
            verifying_key: hex::encode(key().verifying_key().as_bytes()),
            expectation: StandingReceiptExpectationV0::None,
            claim_id: CLAIM.into(),
        },
        policy_id: MAGPIE_CLAIMS_POLICY_V2_ID.into(),
    }
}

fn checkpoint(receipt: &ProducedStandingReceiptV0) -> StandingReceiptCheckpointV0 {
    StandingReceiptCheckpointV0 {
        event_count: receipt.wire().verified_prefix.event_count,
        commitment_sha256: receipt.wire().verified_prefix.tip_sha256,
    }
}

fn produce(bytes: &[u8]) -> ProducedStandingReceiptV0 {
    produce_standing_receipt_v0(&request(bytes), bytes).unwrap()
}

fn rejection(bytes: &[u8], req: &StandingReceiptRequestV0) -> PortableRejectionClass {
    match produce_standing_receipt_v0(req, bytes).unwrap_err() {
        StandingReceiptErrorV0::HistoryRejected(reason) => reason.class(),
        other => panic!("expected portable rejection, got {other:?}"),
    }
}

#[test]
fn real_supported_history_round_trip_and_deleted_state_rederivation() {
    let records = records(support_payloads());
    let bytes = image(&records);
    let req = request(&bytes);
    let receipt = produce_standing_receipt_v0(&req, &bytes).unwrap();
    let outcome = &receipt.wire().outcome;
    assert_eq!(outcome.governed_standing, Some(Status::Supported));
    assert_eq!(outcome.legacy_raw_standing, Some(Status::Conjectured));
    assert_eq!(outcome.trace.len(), 1);
    assert_eq!(
        outcome.trace[0]
            .application
            .as_ref()
            .unwrap()
            .achieved_standing,
        Some(Status::Supported)
    );
    // Independent test oracle only: production never uses compatibility parsing.
    let native = replay_standing_context(&LogReader::open(
        MemStore::from_records(records),
        key().verifying_key(),
    ))
    .unwrap()
    .resolved_standing_with_trace_v2(CLAIM);
    assert_eq!(outcome.canonical_bytes(), native.canonical_bytes());
    let detached = receipt.canonical_bytes().to_vec();
    drop(native);
    drop(receipt); // No projection, vector, receipt or cache retained.
    let rederived = check_standing_receipt_v0(&req, &bytes, &detached).unwrap();
    assert_eq!(rederived.canonical_bytes(), detached);
}

#[test]
fn exact_transport_bytes_change_both_identities_without_changing_outcome() {
    let bytes = image(&records(support_payloads()));
    let original = produce(&bytes);
    let crlf = bytes
        .iter()
        .flat_map(|b| {
            if *b == b'\n' {
                vec![b'\r', b'\n']
            } else {
                vec![*b]
            }
        })
        .collect::<Vec<_>>();
    let without_final_lf = bytes[..bytes.len() - 1].to_vec();
    for alternate in [crlf, without_final_lf] {
        let changed = produce(&alternate);
        assert_eq!(changed.wire().outcome, original.wire().outcome);
        assert_eq!(
            changed.wire().verified_prefix,
            original.wire().verified_prefix
        );
        assert_ne!(changed.context_sha256(), original.context_sha256());
        assert_ne!(changed.result_sha256(), original.result_sha256());
        assert!(matches!(
            check_standing_receipt_v0(&request(&alternate), &alternate, original.canonical_bytes()),
            Err(StandingReceiptErrorV0::DetachedBytesMismatch)
        ));
    }
}

#[test]
fn exact_history_identity_is_checked_before_history_verification() {
    let bytes = b"{malformed";
    let mut req = request(bytes);
    req.context.history.byte_count += 1;
    assert!(matches!(
        produce_standing_receipt_v0(&req, bytes),
        Err(StandingReceiptErrorV0::HistoryByteCountMismatch)
    ));
    req.context.history = StandingReceiptHistoryIdentityV0::of(bytes).unwrap();
    req.context.history.sha256 = ContentHash::ZERO;
    assert!(matches!(
        produce_standing_receipt_v0(&req, bytes),
        Err(StandingReceiptErrorV0::HistoryDigestMismatch)
    ));
}

#[test]
fn key_preflight_and_wrong_key_do_not_become_standing() {
    let bytes = image(&records(support_payloads()));
    let mut req = request(&bytes);
    req.context.verifying_key =
        hex::encode(SigningKey::from_bytes(&[93; 32]).verifying_key().as_bytes());
    assert_eq!(rejection(&bytes, &req), PortableRejectionClass::Signature);
    req.context.verifying_key = "not-a-key".into();
    req.context.history.byte_count = 0;
    assert_eq!(
        rejection(b"{bad", &req),
        PortableRejectionClass::ExternalKey
    );
}

#[test]
fn malformed_and_blank_history_preserve_frozen_rejection_classes() {
    for (bytes, class) in [
        (b"{broken".as_slice(), PortableRejectionClass::JsonSyntax),
        (b"\n".as_slice(), PortableRejectionClass::Framing),
        (b" \t\n".as_slice(), PortableRejectionClass::Framing),
        (b"\xff".as_slice(), PortableRejectionClass::Framing),
        (b"{}".as_slice(), PortableRejectionClass::Schema),
    ] {
        assert_eq!(rejection(bytes, &request(bytes)), class);
    }
    let mut suffix = image(&records(support_payloads()));
    suffix.push(b'\n');
    assert_eq!(
        rejection(&suffix, &request(&suffix)),
        PortableRejectionClass::Framing
    );
}

#[test]
fn explicit_expectation_variants_have_distinct_contexts() {
    let bytes = image(&records(support_payloads()));
    let none = produce(&bytes);
    let cp = checkpoint(&none);
    let mut req = request(&bytes);
    req.context.expectation = StandingReceiptExpectationV0::Exact {
        checkpoint: cp.clone(),
    };
    let exact = produce_standing_receipt_v0(&req, &bytes).unwrap();
    req.context.expectation = StandingReceiptExpectationV0::ContainsCheckpoint { checkpoint: cp };
    let contains = produce_standing_receipt_v0(&req, &bytes).unwrap();
    for (a, b) in [(&none, &exact), (&none, &contains), (&exact, &contains)] {
        assert_ne!(a.context_sha256(), b.context_sha256());
        assert_ne!(a.result_sha256(), b.result_sha256());
        assert_eq!(a.wire().outcome, b.wire().outcome);
    }
    req.context.expectation = StandingReceiptExpectationV0::Exact {
        checkpoint: StandingReceiptCheckpointV0 {
            event_count: 0,
            commitment_sha256: ContentHash::ZERO,
        },
    };
    assert!(matches!(
        produce_standing_receipt_v0(&req, &bytes),
        Err(StandingReceiptErrorV0::ExpectationNotSatisfied(_))
    ));
    req.context.expectation = StandingReceiptExpectationV0::ContainsCheckpoint {
        checkpoint: StandingReceiptCheckpointV0 {
            event_count: 0,
            commitment_sha256: ContentHash::ZERO,
        },
    };
    assert!(produce_standing_receipt_v0(&req, &bytes).is_ok());
}

#[test]
fn contains_binds_whole_verified_suffix_and_never_falls_back_from_exact() {
    let mut payloads = support_payloads();
    let prefix_bytes = image(&records(payloads.clone()));
    let prefix = produce(&prefix_bytes);
    let cp = checkpoint(&prefix);
    payloads.push(Payload::Note {
        text: "verified suffix".into(),
    });
    let bytes = image(&records(payloads));
    let mut req = request(&bytes);
    req.context.expectation = StandingReceiptExpectationV0::Exact {
        checkpoint: cp.clone(),
    };
    assert!(matches!(
        produce_standing_receipt_v0(&req, &bytes),
        Err(StandingReceiptErrorV0::ExpectationNotSatisfied(_))
    ));
    req.context.expectation = StandingReceiptExpectationV0::ContainsCheckpoint { checkpoint: cp };
    let extended = produce_standing_receipt_v0(&req, &bytes).unwrap();
    assert_eq!(
        extended.wire().verified_prefix.event_count,
        prefix.wire().verified_prefix.event_count + 1
    );
    assert_ne!(extended.context_sha256(), prefix.context_sha256());
    assert_ne!(extended.result_sha256(), prefix.result_sha256());
    assert_ne!(
        extended.wire().verified_prefix.tip_sha256,
        prefix.wire().verified_prefix.tip_sha256
    );
    let mut malformed_suffix = bytes.clone();
    malformed_suffix.extend_from_slice(b"{bad");
    req.context.history = StandingReceiptHistoryIdentityV0::of(&malformed_suffix).unwrap();
    assert_eq!(
        rejection(&malformed_suffix, &req),
        PortableRejectionClass::JsonSyntax
    );
}

#[test]
fn truncated_valid_history_and_wrong_position_or_commitment_refuse_expectation() {
    let records = records(support_payloads());
    let full_bytes = image(&records);
    let full = produce(&full_bytes);
    let shorter = image(&records[..records.len() - 1]);
    let mut req = request(&shorter);
    for expectation in [
        StandingReceiptExpectationV0::Exact {
            checkpoint: checkpoint(&full),
        },
        StandingReceiptExpectationV0::ContainsCheckpoint {
            checkpoint: checkpoint(&full),
        },
        StandingReceiptExpectationV0::ContainsCheckpoint {
            checkpoint: StandingReceiptCheckpointV0 {
                event_count: full.wire().verified_prefix.event_count - 1,
                commitment_sha256: full.wire().verified_prefix.tip_sha256,
            },
        },
    ] {
        req.context.expectation = expectation;
        assert!(matches!(
            produce_standing_receipt_v0(&req, &shorter),
            Err(StandingReceiptErrorV0::ExpectationNotSatisfied(_))
        ));
    }
    let mut req = request(&full_bytes);
    req.context.expectation = StandingReceiptExpectationV0::Exact {
        checkpoint: StandingReceiptCheckpointV0 {
            event_count: full.wire().verified_prefix.event_count,
            commitment_sha256: ContentHash::ZERO,
        },
    };
    assert!(matches!(
        produce_standing_receipt_v0(&req, &full_bytes),
        Err(StandingReceiptErrorV0::ExpectationNotSatisfied(_))
    ));
}

#[test]
fn selector_is_unique_typed_exact_and_legacy_compatible() {
    let bytes = image(&records(vec![Payload::ClaimAsserted {
        claim_id: CLAIM.into(),
        statement: "legacy only".into(),
        status: Status::Settled,
    }]));
    assert!(matches!(
        produce_standing_receipt_v0(&request(&bytes), &bytes),
        Err(StandingReceiptErrorV0::MissingTypedClaim)
    ));
    let bytes = image(&records(vec![typed_claim(CLAIM), typed_claim(CLAIM)]));
    assert!(matches!(
        produce_standing_receipt_v0(&request(&bytes), &bytes),
        Err(StandingReceiptErrorV0::AmbiguousTypedClaim)
    ));
    let bytes = image(&records(support_payloads()));
    let mut req = request(&bytes);
    for id in ["", "CLAIM-MACHINE", "missing"] {
        req.context.claim_id = id.into();
        match produce_standing_receipt_v0(&req, &bytes).unwrap_err() {
            StandingReceiptErrorV0::EmptyClaimSelector if id.is_empty() => {}
            StandingReceiptErrorV0::MissingTypedClaim if !id.is_empty() => {}
            other => panic!("unexpected selector refusal {other:?}"),
        }
    }
    let bytes = image(&records(vec![typed_claim("é"), typed_claim("e\u{301}")]));
    let mut req = request(&bytes);
    req.context.claim_id = "é".into();
    let a = produce_standing_receipt_v0(&req, &bytes).unwrap();
    req.context.claim_id = "e\u{301}".into();
    let b = produce_standing_receipt_v0(&req, &bytes).unwrap();
    assert_ne!(a.context_sha256(), b.context_sha256());
    assert_ne!(a.result_sha256(), b.result_sha256());
}

fn hash(domain: &str, bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(domain.as_bytes());
    digest.update([0]);
    digest.update(bytes);
    hex::encode(digest.finalize())
}

#[test]
fn internally_consistent_fabricated_stronger_outcome_is_still_untrusted() {
    let bytes = image(&records(support_payloads()));
    let produced = produce(&bytes);
    let mut forged = produced.wire().clone(); // Inert wire only.
    forged.outcome.governed_standing = Some(Status::Settled);
    #[derive(Serialize)]
    struct Binding<'a> {
        context_sha256: &'a str,
        verified_prefix: &'a magpie_claims::StandingReceiptVerifiedPrefixV0,
        outcome: &'a magpie_claims::StandingResolutionV2,
    }
    forged.result_sha256 = hash(
        "magpie-standing-receipt-result-v0",
        &serde_json::to_vec(&Binding {
            context_sha256: &forged.context_sha256,
            verified_prefix: &forged.verified_prefix,
            outcome: &forged.outcome,
        })
        .unwrap(),
    );
    let detached = serde_json::to_vec(&forged).unwrap();
    assert!(matches!(
        check_standing_receipt_v0(&request(&bytes), &bytes, &detached),
        Err(StandingReceiptErrorV0::DetachedBytesMismatch)
    ));
    assert_eq!(
        produced.wire().outcome.governed_standing,
        Some(Status::Supported)
    );
}

#[test]
fn exact_detached_check_rejects_json_normalization_and_field_changes() {
    let bytes = image(&records(support_payloads()));
    let receipt = produce(&bytes);
    let canonical = String::from_utf8(receipt.canonical_bytes().to_vec()).unwrap();
    let reordered = canonical.replacen(r#"{"schema":"magpie-standing-receipt-v0","encoding_profile":"magpie-standing-receipt-json-v0""#,
        r#"{"encoding_profile":"magpie-standing-receipt-json-v0","schema":"magpie-standing-receipt-v0""#,1);
    let variants = [
        format!("{canonical}\n"),
        canonical.replacen("{", "{ ", 1),
        reordered,
        canonical.replacen("{", r#"{"extra":0,"#, 1),
        canonical.replacen(r#""schema":"magpie-standing-receipt-v0","#, "", 1),
        canonical.replacen("{", r#"{"schema":"magpie-standing-receipt-v0","#, 1),
        canonical.replacen("claim-machine", r#"\u0063laim-machine"#, 1),
        canonical.replacen("Supported", "Settled", 1),
        canonical.replacen("unknown", "Unknown", 1),
    ];
    for variant in variants {
        assert_ne!(variant.as_bytes(), receipt.canonical_bytes());
        assert!(matches!(
            check_standing_receipt_v0(&request(&bytes), &bytes, variant.as_bytes()),
            Err(StandingReceiptErrorV0::DetachedBytesMismatch)
        ));
    }
    // No retained source: detached bytes cannot substitute for a packet.
    assert!(matches!(
        check_standing_receipt_v0(&request(&bytes), b"", receipt.canonical_bytes()),
        Err(StandingReceiptErrorV0::HistoryByteCountMismatch)
    ));
}

#[test]
fn unsupported_profile_and_policy_refuse_before_inspecting_input() {
    let bytes = image(&records(support_payloads()));
    let mut req = request(&bytes);
    req.context.profile = "latest".into();
    assert!(matches!(
        produce_standing_receipt_v0(&req, b"bad"),
        Err(StandingReceiptErrorV0::UnsupportedProfile)
    ));
    req.context.profile = STANDING_RECEIPT_HISTORY_PROFILE_V0.into();
    req.policy_id = "magpie-claims-standing-v4".into();
    assert!(matches!(
        produce_standing_receipt_v0(&req, b"bad"),
        Err(StandingReceiptErrorV0::UnsupportedPolicy)
    ));
}

#[test]
fn inherited_v1_settled_retains_exact_native_application() {
    let root = "0123456789abcdef".repeat(4);
    let reference = format!(
        r#"{{"schema":"segment_anchored_v1","bundle_kind":"kernel-decision-witness","witness_root":"{root}","witness_algorithm":"sha256","canonicalization_profile":"kernel-decision-witness-v1","run_id":"receipt-run"}}"#
    );
    // Existing Magpie recorded-anchor semantics; no external repository is used.
    let mut claim = typed_claim(CLAIM);
    if let Payload::ClaimAssertedV2 {
        statement,
        content_hash,
        metadata_json,
        ..
    } = &mut claim
    {
        *statement = "exact anchor occurs".into();
        *content_hash = String::new();
        *metadata_json = format!(
            r#"{{"claim_domain":"Occurrence/Inclusion","deadbolt_occurrence":{reference}}}"#
        );
    }
    let mut evidence = support_payloads().remove(2);
    if let Payload::EvidenceRegistered {
        evidence_kind,
        metadata_json,
        ..
    } = &mut evidence
    {
        *evidence_kind = "DeadboltAnchor".into();
        *metadata_json = format!(r#"{{"deadbolt_anchor_ref":{reference}}}"#);
    }
    let edge = support_payloads().remove(3);
    let records = records(vec![
        claim,
        evidence,
        edge,
        Payload::SegmentAnchored {
            bundle_kind: "kernel-decision-witness".into(),
            witness_root: root,
            witness_algorithm: "sha256".into(),
            canonicalization_profile: "kernel-decision-witness-v1".into(),
            run_id: "receipt-run".into(),
        },
    ]);
    let bytes = image(&records);
    let receipt = produce(&bytes);
    assert_eq!(
        receipt.wire().outcome.governed_standing,
        Some(Status::Settled)
    );
    assert_eq!(
        receipt.wire().outcome.trace[0]
            .application
            .as_ref()
            .unwrap()
            .achieved_standing,
        Some(Status::Settled)
    );
    let native = replay_standing_context(&LogReader::open(
        MemStore::from_records(records),
        key().verifying_key(),
    ))
    .unwrap()
    .resolved_standing_with_trace_v2(CLAIM);
    assert_eq!(receipt.wire().outcome, native);
}
