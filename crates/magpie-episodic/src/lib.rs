//! # magpie-episodic - the event memory, as a projection
//!
//! [`EpisodicView`] folds the log into a time-ordered SQLite store plus an FTS5
//! index over event text. It is *derived*: drop the database file and rebuild it
//! from the log at any time. It reads through `LogReader::replay`, implements
//! the same [`magpie_log::Projection`] trait as `magpie-claims`, and has no API
//! that can write back to the log.
//!
//! SQLite is deliberately only in this projection crate. The log hash path stays
//! dependency-minimal; this crate is a rebuildable search surface over signed
//! facts, not an authority.

use std::path::Path;

use magpie_log::{Payload, Projection, SignedEvent};
use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::Serialize;

const SCHEMA_VERSION: i32 = 2;

const CREATE_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS events (
    seq             INTEGER PRIMARY KEY,
    timestamp_nanos INTEGER NOT NULL,
    agent           TEXT NOT NULL,
    source          TEXT NOT NULL,
    kind            TEXT NOT NULL,
    claim_id        TEXT,
    claim_status    TEXT,
    from_status     TEXT,
    to_status       TEXT,
    body            TEXT NOT NULL
);
CREATE VIRTUAL TABLE IF NOT EXISTS events_fts USING fts5(
    body, content='events', content_rowid='seq'
);
";

/// One row in the episodic projection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EpisodicEvent {
    pub seq: u64,
    pub timestamp_nanos: u64,
    pub agent: String,
    pub source: String,
    pub kind: String,
    pub claim_id: Option<String>,
    pub claim_status: Option<String>,
    pub from_status: Option<String>,
    pub to_status: Option<String>,
    pub body: String,
}

/// A derived, full-text-searchable view of the event log.
///
/// The base `events` table is the deterministic comparison surface, not the
/// SQLite file bytes. SQLite page layout, freelists, and vacuum history are not
/// a stable serialization format, so [`Self::canonical_bytes`] serializes all
/// rows ordered by `seq` as JSON instead.
///
/// Search returns matching sequence numbers ordered by `seq` ascending, not by
/// bm25 rank. Until Magpie has an explicit reranker, deterministic replay order
/// is more important than relevance scoring.
///
/// Claim statuses are stored as structured columns because the episodic
/// projection is the record of what each signed event said at entry time. The
/// claims projection owns current claim state; this projection owns the
/// replayed timeline.
///
/// Log timestamps are `u64`, while SQLite `INTEGER` is signed 64-bit. Applying
/// an event whose timestamp cannot fit in SQLite panics with a clear message;
/// silently truncating signed log data would make the projection dishonest.
///
/// `Projection::apply` cannot return `Result`, so SQLite errors while folding
/// panic with clear messages. A derived view that failed mid-fold has no valid
/// state to limp along with; drop it and rebuild from the log. Changing the
/// projection trait to return `Result` is a core-trait decision outside this
/// crate.
///
/// File-backed views are stamped with a schema version. A mismatch means the
/// derived tables are dropped and recreated empty for a from-zero replay rather
/// than patched in place. The pre-status-column schema did not store the signed
/// status fields, so `ALTER TABLE` would leave NULLs where a from-scratch
/// rebuild would have values, breaking the invariant that regenerated derived
/// state is byte-identical to incrementally built state.
pub struct EpisodicView {
    conn: Connection,
}

impl EpisodicView {
    /// Create an in-memory episodic view.
    pub fn in_memory() -> Result<Self, rusqlite::Error> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    /// Create a file-backed episodic view.
    ///
    /// The file is only a derived index. Deleting it must never lose
    /// information because the append-only log remains the source of truth.
    pub fn at_path(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        Self::from_connection(Connection::open(path)?)
    }

    fn from_connection(conn: Connection) -> Result<Self, rusqlite::Error> {
        let user_version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if user_version == SCHEMA_VERSION {
            conn.execute_batch(CREATE_SCHEMA)?;
        } else {
            conn.execute_batch(&format!(
                "
                BEGIN IMMEDIATE;
                DROP TABLE IF EXISTS events_fts;
                DROP TABLE IF EXISTS events;
                {CREATE_SCHEMA}
                PRAGMA user_version = {SCHEMA_VERSION};
                COMMIT;
                "
            ))?;
        }
        Ok(Self { conn })
    }

    pub fn len(&self) -> usize {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .expect("EpisodicView failed to count events");
        usize::try_from(count).expect("EpisodicView event count must be non-negative")
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, seq: u64) -> Option<EpisodicEvent> {
        let seq_i64 = match i64::try_from(seq) {
            Ok(seq) => seq,
            Err(_) => return None,
        };
        self.conn
            .query_row(
                "SELECT seq, timestamp_nanos, agent, source, kind, claim_id,
                        claim_status, from_status, to_status, body
                 FROM events
                 WHERE seq = ?1",
                params![seq_i64],
                row_to_event,
            )
            .optional()
            .expect("EpisodicView failed to read event")
    }

    /// Return matching event sequence numbers in ascending log order.
    pub fn search(&self, query: &str) -> Result<Vec<u64>, rusqlite::Error> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(Vec::new());
        }
        let query = fts_phrase_query(query);
        let mut stmt = self.conn.prepare(
            "SELECT rowid
             FROM events_fts
             WHERE events_fts MATCH ?1
             ORDER BY rowid ASC",
        )?;
        let rows = stmt.query_map(params![query], |row| {
            let seq: i64 = row.get(0)?;
            Ok(sql_i64_to_u64(seq, "events_fts.rowid"))
        })?;

        let mut seqs = Vec::new();
        for row in rows {
            seqs.push(row?);
        }
        Ok(seqs)
    }

    /// Deterministic bytes for regeneration equality checks.
    ///
    /// This is JSON over `events ORDER BY seq`, not SQLite file bytes. The file
    /// is an implementation detail of a derived store; the ordered rows are the
    /// reproducible projection state.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let events = self
            .events_ordered()
            .expect("EpisodicView failed to read canonical rows");
        serde_json::to_vec(&events).expect("EpisodicEvent rows are always serializable")
    }

    fn events_ordered(&self) -> Result<Vec<EpisodicEvent>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT seq, timestamp_nanos, agent, source, kind, claim_id,
                    claim_status, from_status, to_status, body
             FROM events
             ORDER BY seq ASC",
        )?;
        let rows = stmt.query_map([], row_to_event)?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        Ok(events)
    }
}

impl Default for EpisodicView {
    fn default() -> Self {
        Self::in_memory().expect("EpisodicView failed to open in-memory SQLite")
    }
}

impl Projection for EpisodicView {
    fn apply(&mut self, event: &SignedEvent) {
        let seq = u64_to_sql_i64(event.core.seq, "seq");
        let timestamp_nanos = u64_to_sql_i64(event.core.timestamp_nanos, "timestamp_nanos");
        let payload = payload_parts(&event.core.payload);

        let tx = self
            .conn
            .transaction()
            .expect("EpisodicView failed to begin SQLite transaction");
        tx.execute(
            "INSERT INTO events (
                 seq, timestamp_nanos, agent, source, kind, claim_id,
                 claim_status, from_status, to_status, body
             ) VALUES (
                 ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
             )",
            params![
                seq,
                timestamp_nanos,
                &event.core.provenance.agent,
                &event.core.provenance.source,
                payload.kind,
                payload.claim_id,
                payload.claim_status,
                payload.from_status,
                payload.to_status,
                payload.body,
            ],
        )
        .expect("EpisodicView failed to insert event row while applying log event");
        tx.execute(
            "INSERT INTO events_fts(rowid, body) VALUES (?1, ?2)",
            params![seq, payload.body],
        )
        .expect("EpisodicView failed to insert FTS row while applying log event");
        tx.commit()
            .expect("EpisodicView failed to commit applied log event");
    }
}

struct PayloadParts<'a> {
    kind: &'static str,
    claim_id: Option<&'a str>,
    claim_status: Option<&'static str>,
    from_status: Option<&'static str>,
    to_status: Option<&'static str>,
    body: &'a str,
}

fn payload_parts(payload: &Payload) -> PayloadParts<'_> {
    match payload {
        Payload::Genesis {
            canonicalization_profile,
            verifying_key: _,
        } => PayloadParts {
            kind: "genesis",
            claim_id: None,
            claim_status: None,
            from_status: None,
            to_status: None,
            body: canonicalization_profile,
        },
        Payload::ClaimAsserted {
            claim_id,
            statement,
            status,
        } => PayloadParts {
            kind: "claim_asserted",
            claim_id: Some(claim_id),
            claim_status: Some(status_name(*status)),
            from_status: None,
            to_status: None,
            body: statement,
        },
        Payload::EvidenceRecorded { claim_id, summary } => PayloadParts {
            kind: "evidence_recorded",
            claim_id: Some(claim_id),
            claim_status: None,
            from_status: None,
            to_status: None,
            body: summary,
        },
        Payload::ClaimStatusChanged {
            claim_id,
            from,
            to,
            reason,
        } => PayloadParts {
            kind: "claim_status_changed",
            claim_id: Some(claim_id),
            claim_status: None,
            from_status: Some(status_name(*from)),
            to_status: Some(status_name(*to)),
            body: reason,
        },
        Payload::Note { text } => PayloadParts {
            kind: "note",
            claim_id: None,
            claim_status: None,
            from_status: None,
            to_status: None,
            body: text,
        },
        // Anchors are timeline rows (ADR-0001): the searchable body is the
        // root itself, so pasting a root into FTS finds the anchor that
        // committed it. No claim_id — anchors commit executions, not claims.
        Payload::SegmentAnchored { witness_root, .. } => PayloadParts {
            kind: "segment_anchored",
            claim_id: None,
            claim_status: None,
            from_status: None,
            to_status: None,
            body: witness_root,
        },
    }
}

fn row_to_event(row: &Row<'_>) -> rusqlite::Result<EpisodicEvent> {
    let seq: i64 = row.get(0)?;
    let timestamp_nanos: i64 = row.get(1)?;
    Ok(EpisodicEvent {
        seq: sql_i64_to_u64(seq, "events.seq"),
        timestamp_nanos: sql_i64_to_u64(timestamp_nanos, "events.timestamp_nanos"),
        agent: row.get(2)?,
        source: row.get(3)?,
        kind: row.get(4)?,
        claim_id: row.get(5)?,
        claim_status: row.get(6)?,
        from_status: row.get(7)?,
        to_status: row.get(8)?,
        body: row.get(9)?,
    })
}

fn status_name(status: magpie_log::Status) -> &'static str {
    match status {
        magpie_log::Status::Open => "Open",
        magpie_log::Status::Conjectured => "Conjectured",
        magpie_log::Status::Supported => "Supported",
        magpie_log::Status::Settled => "Settled",
        magpie_log::Status::Refuted => "Refuted",
    }
}

fn fts_phrase_query(query: &str) -> String {
    format!("\"{}\"", query.replace('"', "\"\""))
}

fn u64_to_sql_i64(value: u64, field: &str) -> i64 {
    i64::try_from(value).unwrap_or_else(|_| {
        panic!("EpisodicView cannot store {field}={value}: SQLite INTEGER is signed 64-bit")
    })
}

fn sql_i64_to_u64(value: i64, field: &str) -> u64 {
    u64::try_from(value)
        .unwrap_or_else(|_| panic!("EpisodicView read invalid negative {field}={value}"))
}
