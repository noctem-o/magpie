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
pub use logimpl::{Clock, LogReader, LogWriter, Projection};
pub use store::{FileStore, LogStore, MemStore};

// Re-exported so callers don't need a direct ed25519-dalek dependency for the basics.
pub use ed25519_dalek::{SigningKey, VerifyingKey};
