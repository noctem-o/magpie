//! One exact image of the log, read once per command.
//!
//! The advisory lock only coordinates `magpie` processes, so a restore or a
//! file swap by anything else could land between two reads of the log path.
//! Each command therefore reads the bytes once and runs every check and every
//! projection against that image: the exact-byte portable verifier sees a
//! private copy of these bytes, and the structural verifier, the checkpoint
//! check, and all projections read the same bytes through a `MemStore`.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use magpie_log::{FileStore, LogReader, MemStore, VerifyingKey};

use crate::store::scratch_dir;
use crate::CliError;

pub(crate) struct Snapshot {
    bytes: Vec<u8>,
}

impl Snapshot {
    /// Read the log for a command that only reads it.
    pub(crate) fn read_checked(path: &Path, verifying_key: VerifyingKey) -> Result<Self, CliError> {
        let snapshot = Self::read(path)?;
        snapshot.require_events(path)?;
        snapshot.require_portable(verifying_key)?;
        Ok(snapshot)
    }

    /// Read the log for a command that appends to it or moves its checkpoint,
    /// which must also refuse a torn final record.
    pub(crate) fn read_for_write(
        path: &Path,
        verifying_key: VerifyingKey,
    ) -> Result<Self, CliError> {
        let snapshot = Self::read(path)?;
        snapshot.require_events(path)?;
        snapshot.ensure_terminated(path)?;
        snapshot.require_portable(verifying_key)?;
        Ok(snapshot)
    }

    /// Read the log with no checks, for `verify`, which reports each one.
    pub(crate) fn read(path: &Path) -> Result<Self, CliError> {
        let bytes = fs::read(path).map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                CliError::Refused(format!("{} is missing", path.display()))
            } else {
                error.into()
            }
        })?;
        Ok(Self { bytes })
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// A reader over these bytes, split into records exactly as `FileStore`
    /// splits them.
    pub(crate) fn reader(&self, verifying_key: VerifyingKey) -> LogReader<MemStore> {
        let records = self
            .bytes
            .split(|byte| *byte == b'\n')
            .filter(|record| !record.is_empty())
            .map(<[u8]>::to_vec)
            .collect();
        LogReader::open(MemStore::from_records(records), verifying_key)
    }

    /// True for a log with no events at all.
    ///
    /// Every store's log begins with the genesis event that binds it to its
    /// key, but the portable verifier accepts an empty history and the
    /// structural reader replays it as zero events, so an empty (for example
    /// truncated) log would otherwise pass every check.
    pub(crate) fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    fn require_events(&self, path: &Path) -> Result<(), CliError> {
        if self.is_empty() {
            return Err(CliError::Refused(format!(
                "{} is empty, so no genesis event binds it to a key; restore it from a backup, \
                 or create a new store with `magpie init`",
                path.display()
            )));
        }
        Ok(())
    }

    /// Refuse a log that ends part-way through a record.
    ///
    /// `FileStore` writes a record and its newline separately. If a crash
    /// landed between the two, the next append would run two records together
    /// and break the chain.
    fn ensure_terminated(&self, path: &Path) -> Result<(), CliError> {
        match self.bytes.last() {
            Some(byte) if *byte != b'\n' => Err(CliError::Refused(format!(
                "{} ends part-way through a record, probably from an interrupted append, so \
                 nothing was written. If its last line is a complete record, add the missing \
                 newline; then run `magpie verify`.",
                path.display()
            ))),
            _ => Ok(()),
        }
    }

    /// Run the exact-byte portable verifier over a private copy of these bytes.
    pub(crate) fn portable_accepts(&self, verifying_key: VerifyingKey) -> Result<bool, CliError> {
        self.portable_accepts_in(&scratch_dir(), verifying_key)
    }

    fn portable_accepts_in(
        &self,
        dir: &Path,
        verifying_key: VerifyingKey,
    ) -> Result<bool, CliError> {
        let copy = ScratchCopy::write(dir, &self.bytes)?;
        FileStore::new(&copy.path)
            .verify_portable_history(&hex::encode(verifying_key.as_bytes()))
            .map_err(|error| CliError::Io(format!("the portable verifier could not run: {error}")))
    }

    /// Refuse a log the portable verifier rejects.
    ///
    /// The structural reader skips blank lines that the portable language
    /// rejects, so without this a write could extend a history that `magpie
    /// verify` would then fail.
    fn require_portable(&self, verifying_key: VerifyingKey) -> Result<(), CliError> {
        if self.portable_accepts(verifying_key)? {
            Ok(())
        } else {
            Err(CliError::Refused(
                "the portable verifier rejects this log (for example a blank line, broken \
                 framing, or a bad signature), so nothing was read or written; run `magpie \
                 verify` for details"
                    .into(),
            ))
        }
    }
}

/// A private temporary copy of the snapshot, removed when dropped.
struct ScratchCopy {
    path: PathBuf,
}

impl ScratchCopy {
    fn write(dir: &Path, bytes: &[u8]) -> Result<Self, CliError> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        for attempt in 0..16u32 {
            let path = dir.join(format!(
                ".snapshot-{}-{nanos}-{attempt}.jsonl",
                std::process::id()
            ));
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            match options.open(&path) {
                Ok(mut file) => {
                    let copy = Self { path };
                    let written = file.write_all(bytes);
                    // Close the handle before a failed copy is removed.
                    drop(file);
                    written?;
                    return Ok(copy);
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.into()),
            }
        }
        Err(CliError::Io(
            "could not create a scratch copy of the log for the portable verifier".into(),
        ))
    }
}

impl Drop for ScratchCopy {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magpie_log::{LogWriter, Payload, Provenance, SigningKey};

    fn scratch_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("magpie-cli-snapshot-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn snapshot_reader_matches_file_store_and_blank_lines_fail_portable() {
        let dir = scratch_dir("parity");
        let log = dir.join("log.jsonl");
        let key = SigningKey::from_bytes(&[9u8; 32]);
        let verifying_key = key.verifying_key();
        let mut writer = LogWriter::<FileStore>::open(FileStore::new(&log), key).unwrap();
        writer
            .append(
                Provenance::new("test", "snapshot"),
                Payload::Note { text: "one".into() },
            )
            .unwrap();

        let snapshot = Snapshot::read(&log).unwrap();
        let from_snapshot = snapshot.reader(verifying_key).verify_chain().unwrap();
        let from_file = LogReader::open(FileStore::new(&log), verifying_key)
            .verify_chain()
            .unwrap();
        assert_eq!(from_snapshot, from_file);
        assert!(snapshot.portable_accepts_in(&dir, verifying_key).unwrap());

        let mut with_blank_line = snapshot.bytes().to_vec();
        with_blank_line.push(b'\n');
        let blank = Snapshot {
            bytes: with_blank_line,
        };
        assert_eq!(
            blank.reader(verifying_key).verify_chain().unwrap(),
            from_file
        );
        assert!(!blank.portable_accepts_in(&dir, verifying_key).unwrap());

        let leftovers = fs::read_dir(&dir)
            .unwrap()
            .filter(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".snapshot-")
            })
            .count();
        assert_eq!(leftovers, 0, "scratch copies must be removed");
        fs::remove_dir_all(&dir).unwrap();
    }
}
