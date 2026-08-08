//! Golden vectors for the `magpie-core-v1` format.
//!
//! These pin the format itself: the committed fixture must verify, every hash
//! and signature must match the values below, and regenerating the chain from
//! code must reproduce them exactly. If this test breaks, the format broke —
//! that is either a bug or a new profile, never a shrug.
//!
//! A from-scratch reimplementation of `docs/FORMAT.md` must reproduce these
//! numbers. That's the point.

use ed25519_dalek::SigningKey;
use magpie_log::{LogError, LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

const EXPECTED_HASHES: [&str; 9] = [
    "cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f", // seq 0
    "deded0c88bccccec9174e20a7149efe7595a7cf1daac4bb710c704f125b4f3cf", // seq 1
    "4e064d9749d987a49400bf789fe12948ac2069da25cfddf45a4550d6654ea9c2", // seq 2
    "0aacdb3a4e34f266d9b5f86399ac3f854087cdad224a334a8735689b8e4f0359", // seq 3
    "ae39e575ce0e20866db47d9748b3d1937719e33446098255f28694ab7201800f", // seq 4
    "640cf8cc21dbec6e82702063116c55a265def44b68edbdf2aa786d49592cc692", // seq 5 (SegmentAnchored)
    "9a73f5deab900231d6b00aedd3ea2698d678b25df63f64f5b5893202e3485c10", // seq 6 (ClaimAssertedV2)
    "9ace5c1ee51f050e5fb5babec7ab41a548afc704a5537a9fc0b49382d31d0b94", // seq 7 (EvidenceRegistered)
    "d9988cc0bec09d2bb8dca03d2b71bce57571443265b0ce01ab92bf4e5febe8a9", // seq 8 (JustificationEdgeRecorded)
];

const EXPECTED_SIGS: [&str; 9] = [
    "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db4421f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
    "132738d446a7b4e9d1c9b27b4588e81d197bd6d0965bc8db11d1d29e0a9e2d7c8ab4f1af4e77648df7d7628e76b3dfc1b43979e502ebad7a5a3672e1a3aeff0e",
    "36512b71013a2778aeef8d945f042234bb0a4748d9fddae33295be0017720624c90bffb790dd7eacf1897ef04d576d3408b814e6e328290868486ed4b5003d05",
    "a76093433f0d688b1e08d6826c07138caa993a06cb3530c6e1ac552bcc1a60cb7109f9ed499b0e5f6dcbe5b996da7f9c130dcd28fa78397c9d113e56ae300f06",
    "e36d22974d389d6cb431e390edec62ca557d76c50d17110f5b98827b55a4d93272cfb71edf1b4ee1b0f46f835f380e3dfd4615510b340a3e50a23adf73cd5d01",
    "c88ad8eafc7b9b31828e42726fba67babd06853abd1d1b1081da64169d1ca791c805b0498209d3937d8d231182a4dde6d3581a710141a4bc10af8720106a8807",
    "711eb1fd1c1dade5fb9f508f888e218c231b9f5d61e054dd40d70a9a78f0181d1adf23e18f3637db2df5c48666d145bab49957a3a91782ca7d666eaf8dc46700",
    "632d10d29d7c939179fcc9f6e56aeed3232dc777cdcc50d62f2ec6f51f1d0bd602b6f35bbd9ebde80090ef04ff42abc7d38fccc196154e8a583796a7d3aed105",
    "950f83a3310dbbdebe95bd376d1a3e91e2ed8ca70c417743b3b1cae6a5126acfdf11c8ca0f49e838db6b0c46ef916ab68c42134f9628f6eec9da63f536d4670d",
];

/// Full canonical preimage of the genesis (seq 0) — the worked example in
/// `docs/FORMAT.md` decodes this string byte by byte.
const CANONICAL_SEQ0: &str = "6d61677069652d636f72652d7631000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000a6d61677069652d6c6f67000000000000000767656e6573697300000000000000000e6d61677069652d636f72652d7631000000000000004065613461366336336532396335323061626566353530376231333265633566393935343737366165626562653762393234323165656136393134343664323263";

/// Canonical preimage of seq 3 (a `ClaimStatusChanged`).
const CANONICAL_SEQ3: &str = "6d61677069652d636f72652d7631000000000000000300000000000000044e064d9749d987a49400bf789fe12948ac2069da25cfddf45a4550d6654ea9c2000000000000000667656f726765000000000000000a6d616e7573637269707403000000000000000474686d320103000000000000003a5468656f72656d20322070726f76656e20616e6420696e646570656e64656e746c7920726570726f647563656420284e3d36342e2e313932292e";

fn fixture_events() -> [(Provenance, Payload); 8] {
    [
        (
            Provenance::new("george", "manuscript"),
            Payload::ClaimAsserted {
                claim_id: "thm2".into(),
                statement:
                    "Four exponentially small spectral scales share one semiclassical exponent."
                        .into(),
                status: Status::Conjectured,
            },
        ),
        (
            Provenance::new("claude", "verification"),
            Payload::EvidenceRecorded {
                claim_id: "thm2".into(),
                summary: "Independent numeric check to ~1e-15 relative error.".into(),
            },
        ),
        (
            Provenance::new("george", "manuscript"),
            Payload::ClaimStatusChanged {
                claim_id: "thm2".into(),
                from: Status::Conjectured,
                to: Status::Settled,
                reason: "Theorem 2 proven and independently reproduced (N=64..192).".into(),
            },
        ),
        (
            Provenance::new("george", "manuscript"),
            Payload::ClaimAsserted {
                claim_id: "f5".into(),
                statement: "Uniform Riccati sandwich (open problem F5).".into(),
                status: Status::Open,
            },
        ),
        (
            Provenance::new("deadbolt", "kernel-witness"),
            Payload::SegmentAnchored {
                bundle_kind: "kernel-decision-witness".into(),
                witness_root: "851d2a8f265e21192c4b1f1ff3bee2a8dc6305a848160412c74b66b74a909141"
                    .into(),
                witness_algorithm: "sha256".into(),
                canonicalization_profile: "phase5-interim-jcs-like-v1".into(),
                run_id: "run-0001".into(),
            },
        ),
        (
            Provenance::new("george", "adr-0002"),
            Payload::ClaimAssertedV2 {
                claim_id: "claim-v2-thm2".into(),
                statement:
                    "Theorem 2 standing is governed by typed evidence and deterministic replay."
                        .into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "HumanRoot".into(),
                content_hash: "".into(),
                metadata_json: "{}".into(),
            },
        ),
        (
            Provenance::new("deadbolt", "adr-0002"),
            Payload::EvidenceRegistered {
                evidence_id: "ev-deadbolt-anchor-0001".into(),
                evidence_kind: "DeadboltAnchor".into(),
                summary: "Deadbolt anchor can prove occurrence and inclusion, not interpretation truth."
                    .into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "DeadboltAnchorer".into(),
                content_hash: "851d2a8f265e21192c4b1f1ff3bee2a8dc6305a848160412c74b66b74a909141".into(),
                metadata_json: "{\"cites_segment\":\"run-0001\"}".into(),
            },
        ),
        (
            Provenance::new("george", "adr-0002"),
            Payload::JustificationEdgeRecorded {
                edge_id: "edge-0001".into(),
                edge_kind: "supports".into(),
                source_id: "ev-deadbolt-anchor-0001".into(),
                target_id: "claim-v2-thm2".into(),
                scope_ref: "magpie:adr-0002".into(),
                actor_class: "HumanRoot".into(),
                rationale:
                    "The evidence supports occurrence/inclusion only; interpretation remains governed by StandingView ceilings."
                        .into(),
                metadata_json: "{}".into(),
            },
        ),
    ]
}

fn load_golden() -> MemStore {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/testdata/golden-v1.jsonl");
    let data = std::fs::read(path).expect("golden fixture must be committed");
    MemStore::from_records(
        data.split(|b| *b == b'\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_vec())
            .collect(),
    )
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut t = 0u64;
    LogWriter::<MemStore>::open_with_clock(
        store,
        SigningKey::from_bytes(&SEED),
        Box::new(move || {
            t += 1;
            t
        }),
    )
    .unwrap()
}

fn append_error(payload: Payload) -> LogError {
    let store = MemStore::new();
    let mut w = writer(store);
    w.append(Provenance::new("test", "validation"), payload)
        .unwrap_err()
}

fn assert_validation_error(payload: Payload, expected_detail: &str) {
    match append_error(payload) {
        LogError::ChainBroken { seq, detail } => {
            assert_eq!(seq, 1);
            assert!(
                detail.contains(expected_detail),
                "expected {expected_detail:?}, got {detail:?}"
            );
        }
        other => panic!("expected ChainBroken validation error, got {other:?}"),
    }
}

fn valid_claim_asserted_v2() -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: "claim-v2".into(),
        statement: "A typed claim starts conjectured.".into(),
        scope_ref: "magpie:test".into(),
        actor_class: "HumanRoot".into(),
        content_hash: "".into(),
        metadata_json: "{}".into(),
    }
}

fn valid_evidence_registered() -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: "evidence-v2".into(),
        evidence_kind: "DeadboltAnchor".into(),
        summary: "Occurrence evidence is typed but inert by itself.".into(),
        scope_ref: "magpie:test".into(),
        actor_class: "DeadboltAnchorer".into(),
        content_hash: "".into(),
        metadata_json: "{}".into(),
    }
}

fn valid_justification_edge_recorded() -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: "edge-v2".into(),
        edge_kind: "supports".into(),
        source_id: "evidence-v2".into(),
        target_id: "claim-v2".into(),
        scope_ref: "magpie:test".into(),
        actor_class: "HumanRoot".into(),
        rationale: "Typed edge coverage.".into(),
        metadata_json: "{}".into(),
    }
}

#[test]
fn committed_fixture_verifies_and_matches_pinned_values() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 9);

    for ev in reader.unverified_events().unwrap() {
        let i = ev.core.seq as usize;
        assert_eq!(
            ev.hash.to_hex(),
            EXPECTED_HASHES[i],
            "hash drift at seq {i}"
        );
        assert_eq!(
            ev.signature.to_hex(),
            EXPECTED_SIGS[i],
            "signature drift at seq {i}"
        );
    }
}

#[test]
fn existing_golden_prefix_for_tags_0_5_remains_unchanged() {
    assert_eq!(
        &EXPECTED_HASHES[..6],
        &[
            "cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f",
            "deded0c88bccccec9174e20a7149efe7595a7cf1daac4bb710c704f125b4f3cf",
            "4e064d9749d987a49400bf789fe12948ac2069da25cfddf45a4550d6654ea9c2",
            "0aacdb3a4e34f266d9b5f86399ac3f854087cdad224a334a8735689b8e4f0359",
            "ae39e575ce0e20866db47d9748b3d1937719e33446098255f28694ab7201800f",
            "640cf8cc21dbec6e82702063116c55a265def44b68edbdf2aa786d49592cc692",
        ]
    );
    assert_eq!(
        &EXPECTED_SIGS[..6],
        &[
            "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db4421f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
            "132738d446a7b4e9d1c9b27b4588e81d197bd6d0965bc8db11d1d29e0a9e2d7c8ab4f1af4e77648df7d7628e76b3dfc1b43979e502ebad7a5a3672e1a3aeff0e",
            "36512b71013a2778aeef8d945f042234bb0a4748d9fddae33295be0017720624c90bffb790dd7eacf1897ef04d576d3408b814e6e328290868486ed4b5003d05",
            "a76093433f0d688b1e08d6826c07138caa993a06cb3530c6e1ac552bcc1a60cb7109f9ed499b0e5f6dcbe5b996da7f9c130dcd28fa78397c9d113e56ae300f06",
            "e36d22974d389d6cb431e390edec62ca557d76c50d17110f5b98827b55a4d93272cfb71edf1b4ee1b0f46f835f380e3dfd4615510b340a3e50a23adf73cd5d01",
            "c88ad8eafc7b9b31828e42726fba67babd06853abd1d1b1081da64169d1ca791c805b0498209d3937d8d231182a4dde6d3581a710141a4bc10af8720106a8807",
        ]
    );
}

#[test]
fn canonical_preimages_match_the_spec_examples() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    let events = reader.unverified_events().unwrap();
    assert_eq!(
        hex::encode(events[0].core.canonical_bytes()),
        CANONICAL_SEQ0
    );
    assert_eq!(
        hex::encode(events[3].core.canonical_bytes()),
        CANONICAL_SEQ3
    );
}

#[test]
fn regeneration_from_code_reproduces_the_fixture_exactly() {
    // Same seed, same deterministic clock, same events — the format must be a
    // pure function of the content.
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        for (prov, payload) in fixture_events() {
            w.append(prov, payload).unwrap();
        }
    }

    let reader = LogReader::open(store.clone(), SigningKey::from_bytes(&SEED).verifying_key());
    for ev in reader.unverified_events().unwrap() {
        let i = ev.core.seq as usize;
        assert_eq!(ev.hash.to_hex(), EXPECTED_HASHES[i]);
        assert_eq!(ev.signature.to_hex(), EXPECTED_SIGS[i]);
    }
    assert_eq!(store.records(), load_golden().records());
}

#[test]
fn a_writer_recovers_over_the_golden_chain() {
    let mut t = 100u64;
    let mut w = LogWriter::<MemStore>::open_with_clock(
        load_golden(),
        SigningKey::from_bytes(&SEED),
        Box::new(move || {
            t += 1;
            t
        }),
    )
    .unwrap();
    assert_eq!(w.len(), 9);
    assert_eq!(w.tip().to_hex(), EXPECTED_HASHES[8]);
    let next = w
        .append(
            Provenance::new("test", "golden.rs"),
            Payload::Note {
                text: "beyond".into(),
            },
        )
        .unwrap();
    assert_eq!(next.core.seq, 9);
}

#[test]
fn anchor_event_round_trips_with_foreign_profile_named() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    let events = reader.unverified_events().unwrap();
    match &events[5].core.payload {
        Payload::SegmentAnchored {
            bundle_kind,
            witness_root,
            witness_algorithm,
            canonicalization_profile,
            run_id,
        } => {
            assert_eq!(bundle_kind, "kernel-decision-witness");
            assert_eq!(witness_root.len(), 64, "roots are 64 lowercase hex chars");
            assert!(witness_root
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
            assert_eq!(witness_algorithm, "sha256");
            // The anchor names the FOREIGN profile — it is not, and must not
            // claim to be, magpie-core-v1.
            assert_eq!(canonicalization_profile, "phase5-interim-jcs-like-v1");
            assert_eq!(run_id, "run-0001");
        }
        other => panic!("seq 5 must be the anchor, got {other:?}"),
    }
}

#[test]
fn adr_0002_events_round_trip_with_closed_vocabularies() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    let events = reader.unverified_events().unwrap();

    match &events[6].core.payload {
        Payload::ClaimAssertedV2 {
            claim_id,
            statement,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        } => {
            assert_eq!(claim_id, "claim-v2-thm2");
            assert!(statement.contains("deterministic replay"));
            assert_eq!(scope_ref, "magpie:adr-0002");
            assert_eq!(actor_class, "HumanRoot");
            assert_eq!(content_hash, "");
            assert_eq!(metadata_json, "{}");
        }
        other => panic!("seq 6 must be ClaimAssertedV2, got {other:?}"),
    }

    match &events[7].core.payload {
        Payload::EvidenceRegistered {
            evidence_id,
            evidence_kind,
            summary,
            scope_ref,
            actor_class,
            content_hash,
            metadata_json,
        } => {
            assert_eq!(evidence_id, "ev-deadbolt-anchor-0001");
            assert_eq!(evidence_kind, "DeadboltAnchor");
            assert!(summary.contains("not interpretation truth"));
            assert_eq!(scope_ref, "magpie:adr-0002");
            assert_eq!(actor_class, "DeadboltAnchorer");
            assert_eq!(content_hash.len(), 64);
            assert!(content_hash
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
            assert_eq!(metadata_json, "{\"cites_segment\":\"run-0001\"}");
        }
        other => panic!("seq 7 must be EvidenceRegistered, got {other:?}"),
    }

    match &events[8].core.payload {
        Payload::JustificationEdgeRecorded {
            edge_id,
            edge_kind,
            source_id,
            target_id,
            scope_ref,
            actor_class,
            rationale,
            metadata_json,
        } => {
            assert_eq!(edge_id, "edge-0001");
            assert_eq!(edge_kind, "supports");
            assert_eq!(source_id, "ev-deadbolt-anchor-0001");
            assert_eq!(target_id, "claim-v2-thm2");
            assert_eq!(scope_ref, "magpie:adr-0002");
            assert_eq!(actor_class, "HumanRoot");
            assert!(rationale.contains("occurrence/inclusion only"));
            assert_eq!(metadata_json, "{}");
        }
        other => panic!("seq 8 must be JustificationEdgeRecorded, got {other:?}"),
    }
}

#[test]
fn claim_asserted_v2_rejects_unknown_actor_class() {
    let mut payload = valid_claim_asserted_v2();
    if let Payload::ClaimAssertedV2 { actor_class, .. } = &mut payload {
        *actor_class = "UnknownActor".into();
    }

    assert_validation_error(
        payload,
        "payload.actor_class must be a known ADR-0002 actor class",
    );
}

#[test]
fn evidence_registered_rejects_unknown_evidence_kind() {
    let mut payload = valid_evidence_registered();
    if let Payload::EvidenceRegistered { evidence_kind, .. } = &mut payload {
        *evidence_kind = "Hearsayish".into();
    }

    assert_validation_error(
        payload,
        "payload.evidence_kind must be a known ADR-0002 evidence kind",
    );
}

#[test]
fn justification_edge_recorded_rejects_unknown_edge_kind() {
    let mut payload = valid_justification_edge_recorded();
    if let Payload::JustificationEdgeRecorded { edge_kind, .. } = &mut payload {
        *edge_kind = "related_to".into();
    }

    assert_validation_error(
        payload,
        "payload.edge_kind must be a known ADR-0002 edge kind",
    );
}

#[test]
fn adr_0002_content_hash_is_empty_or_lowercase_hex64() {
    let mut invalid = valid_evidence_registered();
    if let Payload::EvidenceRegistered { content_hash, .. } = &mut invalid {
        *content_hash = "ABC".into();
    }
    assert_validation_error(
        invalid,
        "payload.content_hash must be empty or 64 lowercase hex chars",
    );

    let mut valid = valid_claim_asserted_v2();
    if let Payload::ClaimAssertedV2 { content_hash, .. } = &mut valid {
        *content_hash = "".into();
    }
    let store = MemStore::new();
    let mut w = writer(store);
    w.append(Provenance::new("test", "validation"), valid)
        .unwrap();
    assert_eq!(w.len(), 2);
}

#[test]
fn adr_0002_scope_ref_is_required() {
    let mut payload = valid_claim_asserted_v2();
    if let Payload::ClaimAssertedV2 { scope_ref, .. } = &mut payload {
        *scope_ref = "".into();
    }

    assert_validation_error(payload, "payload.scope_ref must be non-empty");
}

#[test]
fn adr_0002_required_ids_are_required() {
    let mut claim = valid_claim_asserted_v2();
    if let Payload::ClaimAssertedV2 { claim_id, .. } = &mut claim {
        *claim_id = "".into();
    }
    assert_validation_error(claim, "payload.claim_id must be non-empty");

    let mut evidence = valid_evidence_registered();
    if let Payload::EvidenceRegistered { evidence_id, .. } = &mut evidence {
        *evidence_id = "".into();
    }
    assert_validation_error(evidence, "payload.evidence_id must be non-empty");

    let mut edge = valid_justification_edge_recorded();
    if let Payload::JustificationEdgeRecorded { edge_id, .. } = &mut edge {
        *edge_id = "".into();
    }
    assert_validation_error(edge, "payload.edge_id must be non-empty");
}
