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
//! [`LogStore`] is a read-only substitution surface. Neither built-in storage
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

mod canonical;
mod error;
mod event;
mod hashing;
mod logimpl;
mod store;

pub use canonical::CANONICALIZATION_PROFILE;
pub use error::LogError;
pub use event::{EventCore, Payload, Provenance, Sig, SignedEvent, Status};
pub use hashing::ContentHash;
pub use logimpl::{Clock, LogReader, LogWriter, Projection, VerifiedReplaySummary};
pub use store::{FileStore, LogStore, MemStore};

// Re-exported so callers don't need a direct ed25519-dalek dependency for the basics.
pub use ed25519_dalek::{SigningKey, VerifyingKey};
