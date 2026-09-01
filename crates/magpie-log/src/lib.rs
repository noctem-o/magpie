//! # magpie-log — L0, the hoard
//!
//! The signed, hash-chained, append-only event log. In Magpie this is the **sole
//! source of truth**: every other store (typed memory, the wiki, indexes) is a
//! *derived, regenerable projection* of this log.
//!
//! Two structural invariants are enforced by the type system, not by convention:
//!
//! 1. **Append-only.** There is no `update` or `delete`. A correction is a new
//!    event that supersedes an old one.
//! 2. **The gate is the only writer.** Only [`LogWriter`] has [`LogWriter::append`].
//!    Holding a `LogWriter` *is* the write capability — in the full system the
//!    gate (`deadbolt`) is its only holder. Memory layers are handed a
//!    [`LogReader`], which structurally cannot write.
//!
//! ## Public capability boundary
//!
//! [`LogStore`] is a read-only substitution surface. No supported storage
//! backend exposes Magpie's private record-persistence seam:
//!
//! ```compile_fail,E0599
//! use magpie_log::{FileStore, LogStore};
//!
//! let mut store = FileStore::new(std::env::temp_dir().join("magpie-raw-append.jsonl"));
//! store.append_record(b"caller-supplied record bytes").unwrap();
//! ```
//!
//! ```compile_fail,E0599
//! use magpie_log::{LogStore, MemStore};
//!
//! let mut store = MemStore::new();
//! store.append_record(b"caller-supplied record bytes").unwrap();
//! ```
//!
//! The read trait itself grants no append operation:
//!
//! ```compile_fail,E0599
//! use magpie_log::LogStore;
//!
//! fn append_without_writer<S: LogStore>(store: &mut S) {
//!     store.append_record(b"caller-supplied record bytes").unwrap();
//! }
//! ```
//!
//! A reader exposes neither append nor a mutable backend escape hatch:
//!
//! ```compile_fail,E0599
//! use magpie_log::{LogReader, MemStore, SigningKey};
//!
//! let key = SigningKey::from_bytes(&[7u8; 32]);
//! let mut reader = LogReader::open(MemStore::new(), key.verifying_key());
//! reader.append_record(b"caller-supplied record bytes").unwrap();
//! ```
//!
//! ```compile_fail,E0599
//! use magpie_log::{LogReader, MemStore, SigningKey};
//!
//! let key = SigningKey::from_bytes(&[7u8; 32]);
//! let reader = LogReader::open(MemStore::new(), key.verifying_key());
//! let _store = reader.into_store();
//! ```
//!
//! Downstream storage implementations remain valid for reading, but cannot be
//! opened as writer backends:
//!
//! ```compile_fail,E0599
//! use magpie_log::{LogError, LogStore, LogWriter, SigningKey};
//!
//! struct MyReadStore;
//!
//! impl LogStore for MyReadStore {
//!     fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
//!         Ok(Vec::new())
//!     }
//! }
//!
//! let key = SigningKey::from_bytes(&[7u8; 32]);
//! let _writer = LogWriter::<MyReadStore>::open(MyReadStore, key).unwrap();
//! ```
//!
//! A signing key plus public storage still provides no persistence operation:
//!
//! ```compile_fail,E0599
//! use magpie_log::{MemStore, SigningKey};
//!
//! let key = SigningKey::from_bytes(&[7u8; 32]);
//! let mut store = MemStore::new();
//! key.append_record(&mut store, b"caller-supplied record bytes").unwrap();
//! ```
//!
//! The complete portable-history conformer is intentionally crate-internal:
//!
//! ```compile_fail,E0603
//! use magpie_log::portable_verifier::FrontendSession;
//! ```

mod canonical;
mod error;
mod event;
mod hashing;
mod history_expectation;
mod logimpl;
// The authoritative portable conformer and its governed detail types remain
// crate-internal. A narrow public FileStore method maps that exact-byte path to
// a boolean verdict without exposing a portable result type or second grammar.
// Keep the suppression limited to non-test reachability analysis; test builds
// must account for the complete detailed file path.
#[cfg_attr(not(test), allow(dead_code))]
mod portable_verifier;
mod sqlite_l0;
mod store;

pub mod signature_profile;

pub use canonical::CANONICALIZATION_PROFILE;
pub use error::LogError;
pub use event::{EventCore, Payload, Provenance, Sig, SignedEvent, Status};
pub use hashing::ContentHash;
pub use history_expectation::{
    HistoryCheckpointV0, HistoryExpectationEvaluationV0, HistoryExpectationOutcomeV0,
    HistoryExpectationProfileV0, HistoryExpectationRelationV0, HISTORY_EXPECTATION_PROFILE_V0_ID,
};
pub use logimpl::{
    Clock, LogReader, LogWriter, Projection, VerifiedReplayEvent, VerifiedReplaySummary,
};
pub use sqlite_l0::{
    CheckpointQualifiedWriterOpenV0, L0ResourceLimitsV0, SqliteCheckpointOpenError, SqliteL0Store,
};
pub use store::{FileStore, LogStore, MemStore};

// Re-exported so callers don't need a direct ed25519-dalek dependency for the basics.
pub use ed25519_dalek::{SigningKey, VerifyingKey};
