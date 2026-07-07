use std::path::{Path, PathBuf};

use ed25519_dalek::{SigningKey, VerifyingKey};
use magpie_episodic::EpisodicView;
use magpie_log::{
    LogReader, LogWriter, MemStore, Payload, Projection, Provenance, SignedEvent, Status,
};
use rusqlite::{params, Connection};

const SEED: [u8; 32] = [7u8; 32];

struct TempDbPath {
    path: PathBuf,
}

impl TempDbPath {
    fn new(test_name: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "magpie-episodic-{test_name}-{}.sqlite",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDbPath {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn verifying_key() -> VerifyingKey {
    SigningKey::from_bytes(&SEED).verifying_key()
}

fn writer(store: MemStore) -> LogWriter<MemStore> {
    let mut t = 0u64;
    LogWriter::open_with_clock(
        store,
        SigningKey::from_bytes(&SEED),
        Box::new(move || {
            t += 1;
            t
        }),
    )
    .unwrap()
}

fn fixture_events() -> [(Provenance, Payload); 4] {
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
    ]
}

fn append_fixture_events(writer: &mut LogWriter<MemStore>) -> Vec<SignedEvent> {
    fixture_events()
        .into_iter()
        .map(|(provenance, payload)| writer.append(provenance, payload).unwrap())
        .collect()
}

fn fixture_store() -> MemStore {
    let store = MemStore::new();
    {
        let mut w = writer(store.clone());
        append_fixture_events(&mut w);
    }
    store
}

fn create_old_schema_file(path: &Path) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "
        CREATE TABLE events (
            seq             INTEGER PRIMARY KEY,
            timestamp_nanos INTEGER NOT NULL,
            agent           TEXT NOT NULL,
            source          TEXT NOT NULL,
            kind            TEXT NOT NULL,
            claim_id        TEXT,
            body            TEXT NOT NULL
        );
        CREATE VIRTUAL TABLE events_fts USING fts5(
            body, content='events', content_rowid='seq'
        );
        ",
    )
    .unwrap();
    conn.execute(
        "INSERT INTO events (
             seq, timestamp_nanos, agent, source, kind, claim_id, body
         ) VALUES (
             ?1, ?2, ?3, ?4, ?5, ?6, ?7
         )",
        params![
            999_i64,
            1_i64,
            "old-agent",
            "old-source",
            "claim_asserted",
            "old-claim",
            "stale old-format row"
        ],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO events_fts(rowid, body) VALUES (?1, ?2)",
        params![999_i64, "stale old-format row"],
    )
    .unwrap();

    let user_version: i32 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(user_version, 0);
}

#[test]
fn projection_is_byte_identical_after_live_apply_and_replay() {
    let store = MemStore::new();
    let mut live = EpisodicView::in_memory().unwrap();

    {
        let mut w = writer(store.clone());
        let genesis = LogReader::open(store.clone(), verifying_key())
            .events()
            .unwrap()
            .pop()
            .unwrap();
        live.apply(&genesis);

        for event in append_fixture_events(&mut w) {
            live.apply(&event);
        }
    }

    let live_bytes = live.canonical_bytes();
    let reader = LogReader::open(store, verifying_key());
    let mut rebuilt = EpisodicView::in_memory().unwrap();
    let n = reader.replay(&mut rebuilt).unwrap();

    assert_eq!(n, 5);
    assert_eq!(
        live_bytes,
        rebuilt.canonical_bytes(),
        "episodic projection must be regenerable from the log alone"
    );
}

#[test]
fn stale_schema_file_is_dropped_and_rebuilt_on_open() {
    let db = TempDbPath::new("stale_schema_file_is_dropped_and_rebuilt_on_open");
    create_old_schema_file(db.path());

    let store = fixture_store();
    let reader = LogReader::open(store.clone(), verifying_key());
    let mut rebuilt = EpisodicView::at_path(db.path()).unwrap();
    let n = reader.replay(&mut rebuilt).unwrap();

    let mut fresh = EpisodicView::in_memory().unwrap();
    let fresh_n = LogReader::open(store, verifying_key())
        .replay(&mut fresh)
        .unwrap();

    assert_eq!(n, 5);
    assert_eq!(fresh_n, n);
    assert_eq!(rebuilt.len(), 5);
    assert!(rebuilt.get(999).is_none());
    assert_eq!(
        rebuilt.canonical_bytes(),
        fresh.canonical_bytes(),
        "stale derived state must be rebuilt byte-identically from the log"
    );
}

#[test]
fn current_schema_file_survives_reopen_without_drop() {
    let db = TempDbPath::new("current_schema_file_survives_reopen_without_drop");
    let store = fixture_store();

    let (expected_len, expected_bytes) = {
        let reader = LogReader::open(store, verifying_key());
        let mut view = EpisodicView::at_path(db.path()).unwrap();
        assert_eq!(reader.replay(&mut view).unwrap(), 5);
        (view.len(), view.canonical_bytes())
    };

    let reopened = EpisodicView::at_path(db.path()).unwrap();
    assert_eq!(reopened.len(), expected_len);
    assert_eq!(
        reopened.canonical_bytes(),
        expected_bytes,
        "current-version derived state must survive reopen without replay"
    );
}

#[test]
fn replay_rejects_tampered_store_without_applying_partial_state() {
    let store = fixture_store();
    let mut records = store.records();
    let mut event: SignedEvent = serde_json::from_slice(&records[1]).unwrap();

    match &mut event.core.payload {
        Payload::Genesis {
            canonicalization_profile: _,
            verifying_key: _,
        } => panic!("fixture record 1 should not be genesis"),
        Payload::ClaimAsserted {
            claim_id: _,
            statement,
            status: _,
        } => statement.push_str(" tampered"),
        Payload::EvidenceRecorded {
            claim_id: _,
            summary: _,
        } => panic!("fixture record 1 should be claim asserted"),
        Payload::ClaimStatusChanged {
            claim_id: _,
            from: _,
            to: _,
            reason: _,
        } => panic!("fixture record 1 should be claim asserted"),
        Payload::Note { text: _ } => panic!("fixture record 1 should be claim asserted"),
        Payload::SegmentAnchored { .. } => panic!("fixture record 1 should be claim asserted"),
        Payload::ClaimAssertedV2 { .. } => panic!("fixture record 1 should be claim asserted"),
        Payload::EvidenceRegistered { .. } => panic!("fixture record 1 should be claim asserted"),
        Payload::JustificationEdgeRecorded { .. } => {
            panic!("fixture record 1 should be claim asserted")
        }
    }

    records[1] = serde_json::to_vec(&event).unwrap();
    let tampered = MemStore::from_records(records);
    let reader = LogReader::open(tampered, verifying_key());
    let mut view = EpisodicView::in_memory().unwrap();

    assert!(reader.replay(&mut view).is_err());
    assert!(
        view.is_empty(),
        "replay verifies the chain before folding any event"
    );
}

#[test]
fn search_returns_matching_sequences_in_ascending_order() {
    let store = fixture_store();
    let reader = LogReader::open(store, verifying_key());
    let mut view = EpisodicView::in_memory().unwrap();
    reader.replay(&mut view).unwrap();

    assert_eq!(view.search("semiclassical").unwrap(), vec![1]);
    assert_eq!(view.search("1e-15").unwrap(), vec![2]);
    assert_eq!(view.search("N=64..192").unwrap(), vec![3]);
    assert_eq!(view.search("   ").unwrap(), Vec::<u64>::new());
    assert_eq!(view.search("definitelyabsent").unwrap(), Vec::<u64>::new());

    let event = view.get(1).unwrap();
    assert_eq!(event.seq, 1);
    assert_eq!(event.timestamp_nanos, 2);
    assert_eq!(event.agent, "george");
    assert_eq!(event.source, "manuscript");
    assert_eq!(event.kind, "claim_asserted");
    assert_eq!(event.claim_id, Some("thm2".into()));
    assert_eq!(event.claim_status, Some("Conjectured".into()));
    assert_eq!(event.from_status, None);
    assert_eq!(event.to_status, None);
    assert_eq!(
        event.body,
        "Four exponentially small spectral scales share one semiclassical exponent."
    );

    let status_change = view.get(3).unwrap();
    assert_eq!(status_change.kind, "claim_status_changed");
    assert_eq!(status_change.claim_status, None);
    assert_eq!(status_change.from_status, Some("Conjectured".into()));
    assert_eq!(status_change.to_status, Some("Settled".into()));
}

#[test]
fn drop_and_rebuild_from_same_store_is_equal() {
    let store = fixture_store();
    let reader = LogReader::open(store, verifying_key());

    let mut view_a = EpisodicView::in_memory().unwrap();
    assert_eq!(reader.replay(&mut view_a).unwrap(), 5);
    let bytes_a = view_a.canonical_bytes();

    drop(view_a);

    let mut view_b = EpisodicView::in_memory().unwrap();
    assert_eq!(reader.replay(&mut view_b).unwrap(), 5);

    assert_eq!(
        bytes_a,
        view_b.canonical_bytes(),
        "dropping the derived SQLite state must not lose information"
    );
}

#[test]
fn canonical_bytes_preserve_claim_asserted_status() {
    fn bytes_for(status: Status) -> Vec<u8> {
        let store = MemStore::new();
        {
            let mut w = writer(store.clone());
            w.append(
                Provenance::new("george", "manuscript"),
                Payload::ClaimAsserted {
                    claim_id: "same-claim".into(),
                    statement: "Same statement.".into(),
                    status,
                },
            )
            .unwrap();
        }
        let reader = LogReader::open(store, verifying_key());
        let mut view = EpisodicView::in_memory().unwrap();
        reader.replay(&mut view).unwrap();
        view.canonical_bytes()
    }

    assert_ne!(
        bytes_for(Status::Open),
        bytes_for(Status::Conjectured),
        "asserted status is signed event content and must survive projection"
    );
}
