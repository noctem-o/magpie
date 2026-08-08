use std::cell::Cell;
use std::rc::Rc;

use magpie_claims::{replay_standing_context, DeadboltAnchorIdentity, StandingReplaySnapshot};
use magpie_log::{
    LogError, LogReader, LogStore, LogWriter, MemStore, Payload, Provenance, Sig, SignedEvent,
    SigningKey, Status, VerifyingKey,
};

const SEED: [u8; 32] = [53u8; 32];
const CLAIM_ID: &str = "claim-snapshot";
const EVIDENCE_ID: &str = "evidence-snapshot";
const EDGE_ID: &str = "edge-snapshot";
const SCOPE: &str = "scope:snapshot";
const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deadbolt-anchor-v1/anchor-log.jsonl"
);
const FIXTURE_VERIFYING_KEY_HEX: &str =
    "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";
const FIXTURE_WITNESS_ROOT: &str =
    "79791d454eaaf2f86c1b5a3c944d021c6721905a7a70790c7089bb5a6638af0b";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut timestamp = 0u64;
    LogWriter::<MemStore>::open_with_clock(
        store,
        signing_key(),
        Box::new(move || {
            timestamp += 1;
            timestamp
        }),
    )
    .unwrap()
}

fn provenance() -> Provenance {
    Provenance::new("test", "standing-replay-snapshot")
}

fn claim_payload() -> Payload {
    Payload::ClaimAssertedV2 {
        claim_id: CLAIM_ID.into(),
        statement: "A replayed occurrence claim.".into(),
        scope_ref: SCOPE.into(),
        actor_class: "AgentProposer".into(),
        content_hash: String::new(),
        metadata_json: r#"{"claim_domain":"Occurrence/Inclusion"}"#.into(),
    }
}

fn evidence_payload() -> Payload {
    Payload::EvidenceRegistered {
        evidence_id: EVIDENCE_ID.into(),
        evidence_kind: "DeadboltAnchor".into(),
        summary: "Exact anchor candidate material.".into(),
        scope_ref: SCOPE.into(),
        actor_class: "DeadboltAnchorer".into(),
        content_hash: String::new(),
        metadata_json: "{}".into(),
    }
}

fn edge_payload() -> Payload {
    Payload::JustificationEdgeRecorded {
        edge_id: EDGE_ID.into(),
        edge_kind: "supports".into(),
        source_id: EVIDENCE_ID.into(),
        target_id: CLAIM_ID.into(),
        scope_ref: SCOPE.into(),
        actor_class: "HumanRoot".into(),
        rationale: "Candidate relationship only.".into(),
        metadata_json: "{}".into(),
    }
}

fn anchor_identity(run_id: &str, witness_root: &str) -> DeadboltAnchorIdentity {
    DeadboltAnchorIdentity {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: witness_root.into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "kernel-decision-witness-v1".into(),
        run_id: run_id.into(),
    }
}

fn anchor_payload(identity: &DeadboltAnchorIdentity) -> Payload {
    Payload::SegmentAnchored {
        bundle_kind: identity.bundle_kind.clone(),
        witness_root: identity.witness_root.clone(),
        witness_algorithm: identity.witness_algorithm.clone(),
        canonicalization_profile: identity.canonicalization_profile.clone(),
        run_id: identity.run_id.clone(),
    }
}

fn append_context(writer: &mut LogWriter<MemStore>, identity: &DeadboltAnchorIdentity) {
    for payload in [
        claim_payload(),
        evidence_payload(),
        edge_payload(),
        anchor_payload(identity),
    ] {
        writer.append(provenance(), payload).unwrap();
    }
}

struct ChangingStandingStore {
    first_snapshot: Vec<Vec<u8>>,
    second_snapshot: Vec<Vec<u8>>,
    read_count: Rc<Cell<usize>>,
}

impl LogStore for ChangingStandingStore {
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        let read_count = self.read_count.get();
        self.read_count.set(read_count + 1);
        if read_count == 0 {
            Ok(self.first_snapshot.clone())
        } else {
            Ok(self.second_snapshot.clone())
        }
    }
}

#[test]
fn standing_replay_snapshot_uses_one_changing_store_snapshot() {
    let identity_a = anchor_identity("run-a", &"a".repeat(64));
    let identity_b = anchor_identity("run-b", &"b".repeat(64));
    let source = MemStore::new();
    let (first_snapshot, second_snapshot) = {
        let mut writer = writer(source.clone());
        append_context(&mut writer, &identity_a);
        let first_snapshot = source.records();
        writer
            .append(provenance(), anchor_payload(&identity_b))
            .unwrap();
        (first_snapshot, source.records())
    };

    let read_count = Rc::new(Cell::new(0));
    let store = ChangingStandingStore {
        first_snapshot,
        second_snapshot,
        read_count: Rc::clone(&read_count),
    };
    let reader = LogReader::open(store, signing_key().verifying_key());

    let snapshot = replay_standing_context(&reader).unwrap();

    assert_eq!(read_count.get(), 1);
    assert_eq!(snapshot.event_count(), 5);
    assert_eq!(snapshot.standing().typed_claim_len(), 1);
    assert_eq!(snapshot.standing().typed_evidence_len(), 1);
    assert_eq!(snapshot.standing().justification_edge_len(), 1);
    assert!(snapshot.standing().typed_claim(CLAIM_ID).is_some());
    assert!(snapshot.standing().typed_evidence(EVIDENCE_ID).is_some());
    assert!(snapshot.standing().justification_edge(EDGE_ID).is_some());
    assert_eq!(snapshot.anchors().occurrences(&identity_a).len(), 1);
    assert!(snapshot.anchors().occurrences(&identity_b).is_empty());
}

#[test]
fn standing_replay_snapshot_constructs_both_views_from_normal_replay() {
    let identity = anchor_identity("ordinary-run", &"c".repeat(64));
    let store = MemStore::new();
    {
        let mut writer = writer(store.clone());
        append_context(&mut writer, &identity);
    }
    let reader = LogReader::open(store, signing_key().verifying_key());

    let snapshot = replay_standing_context(&reader).unwrap();

    assert_eq!(snapshot.event_count(), 5);
    let claim = snapshot.standing().get(CLAIM_ID).unwrap();
    assert_eq!(claim.asserted_initial_status, Status::Conjectured);
    assert_eq!(claim.standing, Status::Conjectured);
    assert!(snapshot.standing().typed_claim(CLAIM_ID).is_some());
    assert!(snapshot.standing().typed_evidence(EVIDENCE_ID).is_some());
    assert!(snapshot.standing().justification_edge(EDGE_ID).is_some());
    let occurrence = snapshot.anchors().first_occurrence(&identity).unwrap();
    assert_eq!(occurrence.identity, identity);
    assert_eq!(occurrence.sequence, 4);
}

fn fixture_store() -> MemStore {
    let data = std::fs::read(FIXTURE_PATH).expect("portable anchor fixture must be committed");
    MemStore::from_records(
        data.split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect(),
    )
}

fn fixture_verifying_key() -> VerifyingKey {
    VerifyingKey::from_bytes(&decode_hex_32(FIXTURE_VERIFYING_KEY_HEX))
        .expect("fixture verifying key must be valid")
}

fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64);
    let bytes = value.as_bytes();
    let mut output = [0u8; 32];
    for (index, slot) in output.iter_mut().enumerate() {
        *slot = (hex_nibble(bytes[index * 2]) << 4) | hex_nibble(bytes[index * 2 + 1]);
    }
    output
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("fixture hex must be lowercase"),
    }
}

#[test]
fn standing_replay_snapshot_portable_fixture_is_anchor_only() {
    let reader = LogReader::open(fixture_store(), fixture_verifying_key());
    let snapshot = replay_standing_context(&reader).unwrap();
    let identity = DeadboltAnchorIdentity {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: FIXTURE_WITNESS_ROOT.into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "kernel-decision-witness-v1".into(),
        run_id: "portable-run-0001".into(),
    };

    assert_eq!(snapshot.event_count(), 2);
    assert!(snapshot.standing().is_structurally_empty());
    assert_eq!(snapshot.anchors().len(), 1);
    let occurrence = snapshot.anchors().first_occurrence(&identity).unwrap();
    assert_eq!(
        occurrence.identity.canonicalization_profile,
        "kernel-decision-witness-v1"
    );
}

#[test]
fn standing_replay_snapshot_returns_no_value_on_verification_failure() {
    let store = MemStore::new();
    {
        let mut writer = writer(store.clone());
        writer.append(provenance(), claim_payload()).unwrap();
    }
    let mut records = store.records();
    let mut tampered: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    tampered.signature = Sig::new([0u8; 64]);
    *records.last_mut().unwrap() = serde_json::to_vec(&tampered).unwrap();
    let reader = LogReader::open(
        MemStore::from_records(records),
        signing_key().verifying_key(),
    );

    let result: Result<StandingReplaySnapshot, LogError> = replay_standing_context(&reader);

    assert!(matches!(result, Err(LogError::BadSignature { seq: 1 })));
}

#[test]
fn standing_replay_snapshot_regenerates_deterministically() {
    let identity = anchor_identity("repeatable-run", &"d".repeat(64));
    let store = MemStore::new();
    {
        let mut writer = writer(store.clone());
        append_context(&mut writer, &identity);
    }
    let reader = LogReader::open(store, signing_key().verifying_key());

    let first = replay_standing_context(&reader).unwrap();
    let second = replay_standing_context(&reader).unwrap();

    assert_eq!(first, second);
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
}
