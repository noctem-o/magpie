//! Chain integrity, tip recovery on reopen, and tamper detection.
//!
//! (Reconstructed 2026-07-02 to the documented spec after the originals were
//! lost in transit; behaviour-equivalent, not byte-identical, to the first
//! scaffold's tests.)

use ed25519_dalek::SigningKey;
use magpie_log::{
    FileStore, LogError, LogReader, LogWriter, MemStore, Payload, Provenance, SignedEvent,
};

const SEED: [u8; 32] = [42u8; 32];

fn key() -> SigningKey {
    SigningKey::from_bytes(&SEED)
}

fn writer<S: magpie_log::LogStore>(store: S) -> LogWriter<S> {
    let mut t = 0u64;
    LogWriter::open_with_clock(store, key(), Box::new(move || {
        t += 1;
        t
    }))
    .unwrap()
}

fn note(text: &str) -> Payload {
    Payload::Note { text: text.into() }
}

fn prov() -> Provenance {
    Provenance::new("test", "chain.rs")
}

#[test]
fn chain_verifies_and_counts() {
    let store = MemStore::new();
    let mut w = writer(store.clone());
    for i in 0..3 {
        w.append(prov(), note(&format!("event {i}"))).unwrap();
    }
    assert_eq!(w.len(), 3);

    let reader = LogReader::open(store, key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 3);
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
    // chain, recover the tip, and continue from seq 2.
    let mut w2 = writer(store.clone());
    assert_eq!(w2.len(), 2);
    assert_eq!(w2.tip(), tip_after_two);

    let third = w2.append(prov(), note("third")).unwrap();
    assert_eq!(third.core.seq, 2);
    assert_eq!(third.core.prev_hash, tip_after_two);

    let reader = LogReader::open(store, key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 3);
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

    // A fresh writer over the same file recovers the chain; a reader verifies it.
    let w2 = writer(FileStore::new(&path));
    assert_eq!(w2.len(), 2);

    let reader = LogReader::open(FileStore::new(&path), key().verifying_key());
    assert_eq!(reader.verify_chain().unwrap(), 2);

    std::fs::remove_file(&path).ok();
}

#[test]
fn altered_payload_is_detected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("honest")).unwrap();
        w.append(prov(), note("also honest")).unwrap();
        w.append(prov(), note("still honest")).unwrap();
    }

    // Tamper with the middle record's payload text, leaving everything else alone.
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
        matches!(err, LogError::BadSignature { seq: 1 }),
        "expected BadSignature at seq 1, got: {err:?}"
    );
}

#[test]
fn truncation_from_the_middle_is_detected() {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        w.append(prov(), note("a")).unwrap();
        w.append(prov(), note("b")).unwrap();
        w.append(prov(), note("c")).unwrap();
    }

    // Delete the middle record: the survivor's seq/prev no longer line up.
    let mut records = store.records();
    records.remove(1);

    let reader = LogReader::open(MemStore::from_records(records), key().verifying_key());
    assert!(matches!(
        reader.verify_chain().unwrap_err(),
        LogError::ChainBroken { .. }
    ));
}
