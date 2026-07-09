use magpie_log::{LogReader, MemStore, Payload, Projection, SignedEvent, VerifyingKey};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/anchor-log.jsonl"
);
const FIXTURE_README_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/README.md"
);
const VERIFYING_KEY_HEX: &str = "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";
const BUNDLE_KIND: &str = "kernel-decision-witness";
const WITNESS_ROOT: &str = "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b";
const WITNESS_ALGORITHM: &str = "sha256";
const FOREIGN_PROFILE: &str = "kernel-decision-witness-v1";
const RUN_ID: &str = "portable-run-0001";
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

#[derive(Default)]
struct AnchorReplay {
    anchors: Vec<AnchorFields>,
}

#[derive(Debug, PartialEq, Eq)]
struct AnchorFields {
    bundle_kind: String,
    witness_root: String,
    witness_algorithm: String,
    canonicalization_profile: String,
    run_id: String,
}

impl Projection for AnchorReplay {
    fn apply(&mut self, event: &SignedEvent) {
        if let Payload::SegmentAnchored {
            bundle_kind,
            witness_root,
            witness_algorithm,
            canonicalization_profile,
            run_id,
        } = &event.core.payload
        {
            self.anchors.push(AnchorFields {
                bundle_kind: bundle_kind.clone(),
                witness_root: witness_root.clone(),
                witness_algorithm: witness_algorithm.clone(),
                canonicalization_profile: canonicalization_profile.clone(),
                run_id: run_id.clone(),
            });
        }
    }
}

fn fixture_store() -> MemStore {
    let data = std::fs::read(FIXTURE_PATH).expect("portable anchor fixture must be committed");
    MemStore::from_records(
        data.split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect(),
    )
}

fn verifying_key() -> VerifyingKey {
    VerifyingKey::from_bytes(&decode_hex_32(VERIFYING_KEY_HEX))
        .expect("fixture verifying key must be valid")
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let mut out = [0u8; 32];
    for (i, slot) in out.iter_mut().enumerate() {
        let hi = hex_nibble(value.as_bytes()[i * 2]);
        let lo = hex_nibble(value.as_bytes()[i * 2 + 1]);
        *slot = (hi << 4) | lo;
    }
    out
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("fixture hex must be lowercase"),
    }
}

#[test]
fn portable_deadbolt_anchor_fixture_verifies_and_replays() {
    let reader = LogReader::open(fixture_store(), verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 2);

    let mut replay = AnchorReplay::default();
    assert_eq!(reader.replay(&mut replay).unwrap(), 2);
    assert_eq!(
        replay.anchors,
        vec![AnchorFields {
            bundle_kind: BUNDLE_KIND.into(),
            witness_root: WITNESS_ROOT.into(),
            witness_algorithm: WITNESS_ALGORITHM.into(),
            canonicalization_profile: FOREIGN_PROFILE.into(),
            run_id: RUN_ID.into(),
        }]
    );
}

#[test]
fn portable_deadbolt_anchor_fixture_preserves_foreign_profile_verbatim() {
    let reader = LogReader::open(fixture_store(), verifying_key());
    let events = reader.events().unwrap();
    assert_eq!(events.len(), 2);

    match &events[1].core.payload {
        Payload::SegmentAnchored {
            bundle_kind,
            witness_root,
            witness_algorithm,
            canonicalization_profile,
            run_id,
        } => {
            assert_eq!(bundle_kind, BUNDLE_KIND);
            assert_eq!(witness_root, WITNESS_ROOT);
            assert_eq!(witness_algorithm, WITNESS_ALGORITHM);
            assert_eq!(canonicalization_profile, FOREIGN_PROFILE);
            assert_ne!(
                canonicalization_profile,
                magpie_log::CANONICALIZATION_PROFILE
            );
            assert_eq!(run_id, RUN_ID);
        }
        other => panic!("seq 1 must be SegmentAnchored, got {other:?}"),
    }
}

#[test]
fn portable_deadbolt_anchor_fixture_contains_only_anchor_payloads_after_genesis() {
    let reader = LogReader::open(fixture_store(), verifying_key());
    let events = reader.events().unwrap();
    assert!(matches!(events[0].core.payload, Payload::Genesis { .. }));
    assert!(matches!(
        events[1].core.payload,
        Payload::SegmentAnchored { .. }
    ));
}

#[test]
fn portable_deadbolt_anchor_fixture_has_no_machine_specific_assumptions() {
    for path in [FIXTURE_PATH, FIXTURE_README_PATH] {
        let text = std::fs::read_to_string(path).expect("fixture text file must be readable");
        for marker in FORBIDDEN_MACHINE_MARKERS {
            assert!(
                !text.contains(marker),
                "fixture file {path} must not contain machine-specific marker {marker:?}"
            );
        }
    }
}
