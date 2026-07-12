//! Chain integrity, tip recovery on reopen, tamper detection, and the
//! genesis/domain-separation rules of the `magpie-core-v1` format.

use std::cell::Cell;
use std::rc::Rc;

use ed25519_dalek::{Signer, SigningKey};
use magpie_log::{
    ContentHash, EventCore, FileStore, LogError, LogReader, LogStore, LogWriter, MemStore, Payload,
    Projection, Provenance, Sig, SignedEvent, CANONICALIZATION_PROFILE,
};

const SEED: [u8; 32] = [42u8; 32];

fn key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer<S: magpie_log::LogStore>(store: S) -> LogWriter<S> {
    let mut t = 0u64;
    LogWriter::open_with_clock(
        store,
        key(),
        Box::new(move || {
            t += 1;
            t
        }),
    )
    .unwrap()
}

fn note(text: &str) -> Payload {
    Payload::Note { text: text.into() }
}

fn anchor(witness_root: &str) -> Payload {
    Payload::SegmentAnchored {
        bundle_kind: "kernel-decision-witness".into(),
        witness_root: witness_root.into(),
        witness_algorithm: "sha256".into(),
        canonicalization_profile: "phase5-interim-jcs-like-v1".into(),
        run_id: "run-0001".into(),
    }
}

fn prov() -> Provenance {
    Provenance::new("test", "chain.rs")
}

/// Sign an arbitrary hand-built core exactly the way the spec says the writer
/// must: Ed25519 over `magpie-sig-v1 || content-hash`. Used to construct
/// *honestly signed but semantically wrong* records — and doubling as an
/// independent check of the signing rule itself.
fn hand_signed_record(core: EventCore, sk: &SigningKey) -> Vec<u8> {
    let hash = core.hash();
    let mut msg = b"magpie-sig-v1".to_vec();
    msg.extend_from_slice(hash.as_bytes());
    let signature = sk.sign(&msg);
    serde_json::to_vec(&SignedEvent {
        core,
        hash,
        signature: Sig::new(signature.to_bytes()),
    })
    .unwrap()
}

fn genesis_core(profile: &str, vk_hex: &str) -> EventCore {
    EventCore {
        seq: 0,
        timestamp_nanos: 1,
        prev_hash: ContentHash::ZERO,
        provenance: Provenance::new("magpie-log", "genesis"),
        payload: Payload::Genesis {
            canonicalization_profile: profile.into(),
            verifying_key: vk_hex.into(),
        },
    }
}

struct ChangingSnapshotStore {
    first_snapshot: Vec<Vec<u8>>,
    second_snapshot: Vec<Vec<u8>>,
    read_count: Rc<Cell<usize>>,
}

impl LogStore for ChangingSnapshotStore {
    fn append_record(&mut self, _bytes: &[u8]) -> Result<(), LogError> {
        panic!("ChangingSnapshotStore is read-only in replay tests")
    }

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

#[derive(Default)]
struct RecordingProjection {
    payloads: Vec<Payload>,
}

impl Projection for RecordingProjection {
    fn apply(&mut self, event: &SignedEvent) {
        self.payloads.push(event.core.payload.clone());
    }
}

#[test]
fn genesis_is_written_on_empty_open() {
    let store = MemStore::new();
    let w = writer(store.clone());
    assert_eq!(
        w.len(),
        1,
        "opening an empty store must write exactly the genesis"
    );

    let reader = LogReader::open(store, key().verifying_key());
    let events = reader.events().unwrap();
    match &events[0].core.payload {
        Payload::Genesis {
            canonicalization_profile,
            verifying_key,
        } => {
            assert_eq!(canonicalization_profile, CANONICALIZATION_PROFILE);
            assert_eq!(
                verifying_key,
                &hex::encode(key().verifying_key().as_bytes())
            );
        }
        other => panic!("seq 0 must be Genesis, got {other:?}"),
    }
    assert_eq!(reader.verify_chain().unwrap(), 1);
}

#[test]
fn chain_verifies_and_counts() {
    let store = MemStore::new();
    let mut w = writer(store.clone());
    for i in 0..3 {
        w.append(prov(), note(&format!("event {i}"))).unwrap();
    }
    assert_eq!(w.len(), 4); // genesis + 3

    let reader = LogReader::open(store, key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 4);
}

#[test]
fn replay_uses_one_verified_snapshot_when_store_changes_on_read() {
    let source = MemStore::new();
    let (first_snapshot, mut second_snapshot) = {
        let mut w = writer(source.clone());
        w.append(prov(), note("verified snapshot note")).unwrap();
        let first_snapshot = source.records();
        w.append(
            prov(),
            anchor("851d2a8f265e21192c4b1f1ff3bee2a8dc6305a848160412c74b66b74a909141"),
        )
        .unwrap();
        (first_snapshot, source.records())
    };

    let mut injected: SignedEvent =
        serde_json::from_slice(second_snapshot.last().unwrap()).unwrap();
    injected.signature = Sig::new([0u8; 64]);
    *second_snapshot.last_mut().unwrap() = serde_json::to_vec(&injected).unwrap();

    let read_count = Rc::new(Cell::new(0));
    let store = ChangingSnapshotStore {
        first_snapshot,
        second_snapshot,
        read_count: Rc::clone(&read_count),
    };
    let reader = LogReader::open(store, key().verifying_key());
    let mut projection = RecordingProjection::default();

    assert_eq!(reader.replay(&mut projection).unwrap(), 2);
    assert_eq!(read_count.get(), 1, "replay must read exactly one snapshot");
    assert_eq!(projection.payloads.len(), 2);
    assert!(matches!(projection.payloads[0], Payload::Genesis { .. }));
    assert!(matches!(projection.payloads[1], Payload::Note { .. }));
    assert!(
        !projection
            .payloads
            .iter()
            .any(|payload| matches!(payload, Payload::SegmentAnchored { .. })),
        "the forged second-snapshot anchor must never be applied"
    );
}

#[test]
fn replay_does_not_apply_valid_prefix_before_later_verification_failure() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("valid prefix")).unwrap();
        w.append(
            prov(),
            anchor("851d2a8f265e21192c4b1f1ff3bee2a8dc6305a848160412c74b66b74a909141"),
        )
        .unwrap();
    }

    let mut records = store.records();
    let mut invalid: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    invalid.signature = Sig::new([0u8; 64]);
    *records.last_mut().unwrap() = serde_json::to_vec(&invalid).unwrap();

    let reader = LogReader::open(MemStore::from_records(records), key().verifying_key());
    let mut projection = RecordingProjection::default();
    let err = reader.replay(&mut projection).unwrap_err();

    assert!(matches!(err, LogError::BadSignature { seq: 2 }));
    assert!(
        projection.payloads.is_empty(),
        "no valid-prefix event may be applied before the full snapshot verifies"
    );
}

#[test]
fn tip_is_recovered_on_reopen() {
    let store = MemStore::new();

    let tip_after_two = {
        let mut w = writer(store.clone());
        w.append(prov(), note("first")).unwrap();
        let second = w.append(prov(), note("second")).unwrap();
        assert_eq!(w.tip(), second.hash);
        w.tip()
    }; // the write capability is dropped here

    // Reopen over the same bytes: the writer must walk and verify the stored
    // chain, recover the tip, and continue from seq 3.
    let mut w2 = writer(store.clone());
    assert_eq!(w2.len(), 3); // genesis + 2
    assert_eq!(w2.tip(), tip_after_two);

    let third = w2.append(prov(), note("third")).unwrap();
    assert_eq!(third.core.seq, 3);
    assert_eq!(third.core.prev_hash, tip_after_two);

    let reader = LogReader::open(store, key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 4);
}

#[test]
fn file_store_round_trips_and_recovers() {
    let path = std::env::temp_dir().join(format!(
        "magpie-chain-test-{}-{}.jsonl",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    {
        let mut w = writer(FileStore::new(&path));
        w.append(prov(), note("durable one")).unwrap();
        w.append(prov(), note("durable two")).unwrap();
    }

    let w2 = writer(FileStore::new(&path));
    assert_eq!(w2.len(), 3);

    let reader = LogReader::open(FileStore::new(&path), key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 3);

    std::fs::remove_file(&path).ok();
}

#[test]
fn altered_payload_is_detected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("honest")).unwrap();
        w.append(prov(), note("also honest")).unwrap();
    }

    // Tamper with the first note's text (seq 1; seq 0 is the genesis).
    let mut records = store.records();
    let mut v: serde_json::Value = serde_json::from_slice(&records[1]).unwrap();
    v["core"]["payload"]["text"] = serde_json::Value::String("revised history".into());
    records[1] = serde_json::to_vec(&v).unwrap();

    let reader = LogReader::open(MemStore::from_records(records), key().verifying_key());
    let err = reader.verify_chain().unwrap_err();
    assert!(
        matches!(err, LogError::ChainBroken { seq: 1, .. }),
        "expected ChainBroken at seq 1, got: {err:?}"
    );
}

#[test]
fn forged_rehash_without_the_key_is_detected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("one")).unwrap();
        w.append(prov(), note("two")).unwrap();
    }

    // A smarter attacker: rewrite the LAST record's payload AND recompute its
    // stored content hash so seq/prev/content checks all pass. Without the
    // signing key, the old signature no longer covers the new hash.
    let mut records = store.records();
    let mut ev: SignedEvent = serde_json::from_slice(records.last().unwrap()).unwrap();
    ev.core.payload = note("two, but improved");
    ev.hash = ev.core.hash();
    *records.last_mut().unwrap() = serde_json::to_vec(&ev).unwrap();

    let reader = LogReader::open(MemStore::from_records(records), key().verifying_key());
    let err = reader.verify_chain().unwrap_err();
    assert!(
        matches!(err, LogError::BadSignature { seq: 2 }),
        "expected BadSignature at seq 2, got: {err:?}"
    );
}

#[test]
fn truncation_from_the_middle_is_detected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("a")).unwrap();
        w.append(prov(), note("b")).unwrap();
    }

    let mut records = store.records();
    records.remove(1);

    let reader = LogReader::open(MemStore::from_records(records), key().verifying_key());
    assert!(matches!(
        reader.verify_chain().unwrap_err(),
        LogError::ChainBroken { .. }
    ));
}

#[test]
fn appending_a_second_genesis_is_rejected() {
    let store = MemStore::new();
    let mut w = writer(store);
    let err = w
        .append(
            prov(),
            Payload::Genesis {
                canonicalization_profile: CANONICALIZATION_PROFILE.into(),
                verifying_key: hex::encode(key().verifying_key().as_bytes()),
            },
        )
        .unwrap_err();
    assert!(
        matches!(err, LogError::ChainBroken { seq: 1, .. }),
        "genesis after seq 0 must be refused at write time, got: {err:?}"
    );
}

#[test]
fn foreign_key_chain_is_rejected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("mine")).unwrap();
    }

    // A verifier holding a different trust root must reject the whole chain
    // at the first event.
    let other = SigningKey::from_bytes(&[9u8; 32]);
    let reader = LogReader::open(store, other.verifying_key());
    assert!(matches!(
        reader.verify_chain().unwrap_err(),
        LogError::BadSignature { seq: 0 }
    ));
}

#[test]
fn honestly_signed_wrong_key_genesis_is_detected() {
    // A genesis correctly signed by our key but *declaring someone else's* —
    // a writer bug, not an attack; the self-description must stay consistent.
    let liar = SigningKey::from_bytes(&[9u8; 32]);
    let record = hand_signed_record(
        genesis_core(
            CANONICALIZATION_PROFILE,
            &hex::encode(liar.verifying_key().as_bytes()),
        ),
        &key(),
    );

    let reader = LogReader::open(MemStore::from_records(vec![record]), key().verifying_key());
    let err = reader.verify_chain().unwrap_err();
    match err {
        LogError::ChainBroken { seq: 0, detail } => {
            assert!(
                detail.contains("verifying key"),
                "unexpected detail: {detail}"
            )
        }
        other => panic!("expected ChainBroken at seq 0, got: {other:?}"),
    }
}

#[test]
fn wrong_profile_genesis_is_detected() {
    let record = hand_signed_record(
        genesis_core(
            "magpie-core-v0",
            &hex::encode(key().verifying_key().as_bytes()),
        ),
        &key(),
    );

    let reader = LogReader::open(MemStore::from_records(vec![record]), key().verifying_key());
    let err = reader.verify_chain().unwrap_err();
    match err {
        LogError::ChainBroken { seq: 0, detail } => {
            assert!(detail.contains("profile"), "unexpected detail: {detail}")
        }
        other => panic!("expected ChainBroken at seq 0, got: {other:?}"),
    }
}

#[test]
fn chain_missing_genesis_is_rejected() {
    // An otherwise perfectly valid chain whose seq 0 is a plain Note.
    let record = hand_signed_record(
        EventCore {
            seq: 0,
            timestamp_nanos: 1,
            prev_hash: ContentHash::ZERO,
            provenance: prov(),
            payload: note("I am not a genesis"),
        },
        &key(),
    );

    let reader = LogReader::open(MemStore::from_records(vec![record]), key().verifying_key());
    let err = reader.verify_chain().unwrap_err();
    match err {
        LogError::ChainBroken { seq: 0, detail } => {
            assert!(detail.contains("genesis"), "unexpected detail: {detail}")
        }
        other => panic!("expected ChainBroken at seq 0, got: {other:?}"),
    }
}

#[test]
fn writer_rejects_invalid_anchor_witness_root_before_signing() {
    let store = MemStore::new();
    let mut w = writer(store.clone());

    let err = w.append(prov(), anchor("ABC")).unwrap_err();
    match err {
        LogError::ChainBroken { seq, detail } => {
            assert_eq!(seq, 1);
            assert!(
                detail.contains("witness_root"),
                "unexpected detail: {detail}"
            );
        }
        other => panic!("expected ChainBroken for invalid witness_root, got: {other:?}"),
    }

    let reader = LogReader::open(store, key().verifying_key());
    assert_eq!(
        reader.verify_chain().unwrap(),
        1,
        "invalid anchor must not be appended after genesis"
    );
}

#[test]
fn verifier_rejects_signed_anchor_with_invalid_witness_root() {
    let genesis = hand_signed_record(
        genesis_core(
            CANONICALIZATION_PROFILE,
            &hex::encode(key().verifying_key().as_bytes()),
        ),
        &key(),
    );
    let genesis_event: SignedEvent = serde_json::from_slice(&genesis).unwrap();
    let anchor_record = hand_signed_record(
        EventCore {
            seq: 1,
            timestamp_nanos: 2,
            prev_hash: genesis_event.hash,
            provenance: prov(),
            payload: anchor("851D2A8F265E21192C4B1F1FF3BEE2A8DC6305A848160412C74B66B74A909141"),
        },
        &key(),
    );

    let reader = LogReader::open(
        MemStore::from_records(vec![genesis, anchor_record]),
        key().verifying_key(),
    );
    let err = reader.verify_chain().unwrap_err();
    match err {
        LogError::ChainBroken { seq, detail } => {
            assert_eq!(seq, 1);
            assert!(
                detail.contains("witness_root"),
                "unexpected detail: {detail}"
            );
        }
        other => panic!("expected ChainBroken for invalid witness_root, got: {other:?}"),
    }
}
