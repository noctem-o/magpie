//! # magpie-cli — the `magpie` command
//!
//! A thin shell over the kernel crates so a person can keep a real claim
//! ledger. Every write goes through [`magpie_log::LogWriter`]. Every read is a
//! complete verified replay into projections that are rebuilt on each
//! invocation and never cached, so nothing this tool shows can drift from the
//! log.
//!
//! ## Storage
//!
//! A store is a directory holding `log.jsonl` (the signed history),
//! `signing.key`, `config.json` (the recorded agent name and verifying key),
//! and a lock file. The log uses [`magpie_log::FileStore`] rather than the
//! supported SQLite L0, because SQLite L0 deliberately offers no replay into
//! projections until projection failure has a typed lifecycle (audit finding
//! A-007), and every read this tool does is a projection. JSONL has one real
//! advantage: the independent Python and Go conformers read it directly.
//!
//! Each command reads the log once and runs every check and projection against
//! that one image. Before any read or write, the image must hold at least the
//! genesis event and its exact bytes must pass
//! [`magpie_log::FileStore::verify_portable_history`], because the record
//! reader skips blank lines that the portable language rejects. That entry
//! point takes a path, so it reads a short-lived private copy in the state
//! directory, or the system temporary directory when that can't take it; reads
//! never write to the store directory, so a read-only store can still be read
//! and verified. Around this the crate adds what `FileStore` lacks for single-user
//! use: advisory locks, a refusal to append after a torn final record, a last
//! byte-for-byte comparison with the image before appending, and an fsync after
//! each append. The locks coordinate only `magpie` processes, so another
//! program that replaces the log between that comparison and the append is not
//! stopped. It makes no multi-host or network-filesystem guarantee.
//!
//! ## Rollback detection
//!
//! After each append the tool saves the log's `(count, tip)` checkpoint outside
//! the store directory (`$MAGPIE_STATE_DIR`, else `$XDG_STATE_HOME/magpie`,
//! else `~/.local/state/magpie`, else `%LOCALAPPDATA%\magpie\state`). Every
//! later command checks, through
//! [`magpie_log::LogReader::evaluate_history_expectation_v0`], that the log
//! still contains that checkpoint, so a store restored from an older copy or
//! swapped for a fork is refused. Restoring the checkpoint directory together
//! with the store defeats this, so keep them apart.
//!
//! Checkpoints are keyed by verifying key, so every copy of a store shares
//! one. A write holds a lock beside it from checking it until replacing it, so
//! two copies cannot both extend the same checkpoint. `magpie verify
//! --verifying-key <hex>` reads neither `config.json` nor `signing.key`, so a
//! key kept off the machine can still check a store whose local state is
//! damaged.
//!
//! ## Authority
//!
//! Writes are recorded with `actor_class` `HumanRoot`: the tool assumes the
//! person typing is the owner. Anyone who can read `signing.key` can sign as the
//! owner, so agents must not be given write access through this tool; agent
//! proposals need the governed admission path the MCP contracts describe. On
//! Unix the key file is created readable only by its owner. On Windows the
//! standard library can't set that, so it inherits the folder's permissions,
//! and `init` says so: keep a Windows store in a folder only you can read.

use std::ffi::OsString;
use std::fmt;
use std::io::Write;

mod args;
mod commands;
mod display;
mod export;
mod index;
mod policy;
mod snapshot;
mod store;
mod trace;

/// A failed invocation, classified by what the caller should do about it.
#[derive(Debug)]
pub(crate) enum CliError {
    /// The command line or its inputs were wrong. Nothing was written. Exit 2.
    Usage(String),
    /// A safety check refused the operation: verification failed, the log was
    /// rolled back or torn, or the store was busy. Nothing was written. Exit 1.
    Refused(String),
    /// The operating system failed underneath us. Exit 3.
    Io(String),
}

impl CliError {
    fn exit_code(&self) -> i32 {
        match self {
            CliError::Refused(_) => 1,
            CliError::Usage(_) => 2,
            CliError::Io(_) => 3,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Usage(message) | CliError::Refused(message) | CliError::Io(message) => {
                f.write_str(message)
            }
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::Io(error.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        CliError::Io(format!("could not write JSON: {error}"))
    }
}

/// Run one `magpie` invocation and return its process exit code.
///
/// `args` excludes the program name. Normal output goes to `out`; warnings and
/// errors go to `err`.
pub fn run(args: &[OsString], out: &mut dyn Write, err: &mut dyn Write) -> i32 {
    let result = args::parse(args).and_then(|invocation| commands::execute(invocation, out, err));
    match result {
        Ok(()) => 0,
        Err(error) => {
            let _ = writeln!(err, "magpie: {error}");
            error.exit_code()
        }
    }
}
