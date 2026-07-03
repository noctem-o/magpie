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
use magpie_log::{LogReader, LogWriter, MemStore, Payload, Provenance, Status};

const SEED: [u8; 32] = [7u8; 32];

const EXPECTED_HASHES: [&str; 6] = [
    "cbb7685efd5d679a5a6f548fe42a36d8dacd391606a0c23c767d89052f01e90f", // seq 0
    "deded0c88bccccec9174e20a7149efe7595a7cf1daac4bb710c704f125b4f3cf", // seq 1
    "4e064d9749d987a49400bf789fe12948ac2069da25cfddf45a4550d6654ea9c2", // seq 2
    "0aacdb3a4e34f266d9b5f86399ac3f854087cdad224a334a8735689b8e4f0359", // seq 3
    "ae39e575ce0e20866db47d9748b3d1937719e33446098255f28694ab7201800f", // seq 4
    "640cf8cc21dbec6e82702063116c55a265def44b68edbdf2aa786d49592cc692", // seq 5 (SegmentAnchored)
];

const EXPECTED_SIGS: [&str; 6] = [
    "52b35da72fca6fffb61f6155871aa7af44023be964a10daa5238d192e5e059db4421f4e6b37ed8e149546546f62fd1fcf29ecbb3b9e601723d64770c30b76d01",
    "132738d446a7b4e9d1c9b27b4588e81d197bd6d0965bc8db11d1d29e0a9e2d7c8ab4f1af4e77648df7d7628e76b3dfc1b43979e502ebad7a5a3672e1a3aeff0e",
    "36512b71013a2778aeef8d945f042234bb0a4748d9fddae33295be0017720624c90bffb790dd7eacf1897ef04d576d3408b814e6e328290868486ed4b5003d05",
    "a76093433f0d688b1e08d6826c07138caa993a06cb3530c6e1ac552bcc1a60cb7109f9ed499b0e5f6dcbe5b996da7f9c130dcd28fa78397c9d113e56ae300f06",
    "e36d22974d389d6cb431e390edec62ca557d76c50d17110f5b98827b55a4d93272cfb71edf1b4ee1b0f46f835f380e3dfd4615510b340a3e50a23adf73cd5d01",
    "c88ad8eafc7b9b31828e42726fba67babd06853abd1d1b1081da64169d1ca791c805b0498209d3937d8d231182a4dde6d3581a710141a4bc10af8720106a8807",
];

/// Full canonical preimage of the genesis (seq 0) — the worked example in
/// `docs/FORMAT.md` decodes this string byte by byte.
const CANONICAL_SEQ0: &str = "6d61677069652d636f72652d7631000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000a6d61677069652d6c6f67000000000000000767656e6573697300000000000000000e6d61677069652d636f72652d7631000000000000004065613461366336336532396335323061626566353530376231333265633566393935343737366165626562653762393234323165656136393134343664323263";

/// Canonical preimage of seq 3 (a `ClaimStatusChanged`).
const CANONICAL_SEQ3: &str = "6d61677069652d636f72652d7631000000000000000300000000000000044e064d9749d987a49400bf789fe12948ac2069da25cfddf45a4550d6654ea9c2000000000000000667656f726765000000000000000a6d616e7573637269707403000000000000000474686d320103000000000000003a5468656f72656d20322070726f76656e20616e6420696e646570656e64656e746c7920726570726f647563656420284e3d36342e2e313932292e";

fn fixture_events() -> [(Provenance, Payload); 5] {
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

#[test]
fn committed_fixture_verifies_and_matches_pinned_values() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 6);

    for ev in reader.events().unwrap() {
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
fn canonical_preimages_match_the_spec_examples() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    let events = reader.events().unwrap();
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
        let mut t = 0u64;
        let mut w = LogWriter::open_with_clock(
            store.clone(),
            SigningKey::from_bytes(&SEED),
            Box::new(move || {
                t += 1;
                t
            }),
        )
        .unwrap();
        for (prov, payload) in fixture_events() {
            w.append(prov, payload).unwrap();
        }
    }

    let reader = LogReader::open(store, SigningKey::from_bytes(&SEED).verifying_key());
    for ev in reader.events().unwrap() {
        let i = ev.core.seq as usize;
        assert_eq!(ev.hash.to_hex(), EXPECTED_HASHES[i]);
        assert_eq!(ev.signature.to_hex(), EXPECTED_SIGS[i]);
    }
}

#[test]
fn a_writer_recovers_over_the_golden_chain() {
    let mut t = 100u64;
    let mut w = LogWriter::open_with_clock(
        load_golden(),
        SigningKey::from_bytes(&SEED),
        Box::new(move || {
            t += 1;
            t
        }),
    )
    .unwrap();
    assert_eq!(w.len(), 6);
    assert_eq!(w.tip().to_hex(), EXPECTED_HASHES[5]);
    let next = w
        .append(
            Provenance::new("test", "golden.rs"),
            Payload::Note {
                text: "beyond".into(),
            },
        )
        .unwrap();
    assert_eq!(next.core.seq, 6);
}

#[test]
fn anchor_event_round_trips_with_foreign_profile_named() {
    let reader = LogReader::open(load_golden(), SigningKey::from_bytes(&SEED).verifying_key());
    let events = reader.events().unwrap();
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
