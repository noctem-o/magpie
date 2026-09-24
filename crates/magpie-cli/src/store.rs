//! A store directory: its files, its lock, and its rollback checkpoint.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use magpie_log::{
    ContentHash, FileStore, HistoryCheckpointV0, HistoryExpectationOutcomeV0,
    HistoryExpectationRelationV0, LogError, LogReader, LogStore, LogWriter, SigningKey,
    VerifyingKey,
};
use serde::{Deserialize, Serialize};

use crate::CliError;

const LOG_FILE: &str = "log.jsonl";
const KEY_FILE: &str = "signing.key";
const CONFIG_FILE: &str = "config.json";
const LOCK_FILE: &str = "lock";
const STORE_FORMAT: &str = "magpie-cli-store-v0";
const CHECKPOINT_FORMAT: &str = "magpie-cli-checkpoint-v0";
const LOCK_ATTEMPTS: u32 = 30;
const LOCK_RETRY: Duration = Duration::from_millis(100);

#[derive(Serialize, Deserialize)]
struct StoreConfig {
    format: String,
    agent: String,
    verifying_key: String,
}

#[derive(Serialize, Deserialize)]
struct CheckpointRecord {
    format: String,
    event_count: u64,
    tip: ContentHash,
    log: String,
}

/// A checkpoint saved after an earlier append.
pub(crate) struct SavedCheckpoint {
    pub event_count: u64,
    pub tip: ContentHash,
    pub path: PathBuf,
}

pub(crate) enum CheckpointState {
    /// The log verified and still contains the saved checkpoint.
    Contained(SavedCheckpoint),
    /// No checkpoint is saved for this store's key.
    Missing,
}

#[derive(Clone, Copy)]
pub(crate) enum LockMode {
    Shared,
    Exclusive,
}

pub(crate) struct Store {
    dir: PathBuf,
    agent: String,
    verifying_key: VerifyingKey,
}

/// Map a kernel error to the caller's next step: I/O is operational, anything
/// else means the history itself failed verification.
pub(crate) fn log_failure(error: LogError) -> CliError {
    match error {
        LogError::Io(error) => CliError::Io(error.to_string()),
        other => CliError::Refused(format!("the log failed verification: {other}")),
    }
}

pub(crate) fn parse_verifying_key(text: &str) -> Result<VerifyingKey, CliError> {
    let bytes: [u8; 32] = hex::decode(text.trim())
        .ok()
        .and_then(|bytes| bytes.try_into().ok())
        .ok_or_else(|| {
            CliError::Usage(format!("`{text}` is not a 64-character hex verifying key"))
        })?;
    VerifyingKey::from_bytes(&bytes)
        .map_err(|_| CliError::Usage(format!("`{text}` is not a valid Ed25519 verifying key")))
}

impl Store {
    /// Create a new store: a fresh signing key, its config, and the genesis event.
    pub(crate) fn init(dir: &Path, agent: String) -> Result<(Store, ContentHash), CliError> {
        if agent.trim().is_empty() {
            return Err(CliError::Usage("--agent must not be empty".into()));
        }
        refuse_existing(dir)?;
        create_dir_all_durable(dir)?;

        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed).map_err(|error| {
            CliError::Io(format!(
                "could not gather randomness for the signing key: {error}"
            ))
        })?;
        let signing_key = SigningKey::from_bytes(&seed);
        seed.fill(0);
        let verifying_key = signing_key.verifying_key();
        let store = Store {
            dir: dir.to_path_buf(),
            agent,
            verifying_key,
        };
        // Both locks before the first store file, so a missing state directory
        // fails here and leaves nothing that would make a retry refuse.
        let _store_lock = store.lock(LockMode::Exclusive)?;
        let _checkpoint_lock = store.lock_checkpoint(LockMode::Exclusive)?;
        // Checked again under the lock: from here on, any store file in the
        // directory was created by this attempt, so a failure may remove it.
        refuse_existing(dir)?;
        let writer = store
            .create_files(signing_key)
            .map_err(|error| store.discard_partial(error))?;
        // The store exists now, so a failure from here on must not send the
        // caller back to `init`, which refuses an existing store.
        if let Err(error) = store
            .sync_log()
            .and_then(|()| store.save_checkpoint(writer.len(), writer.tip()))
        {
            return Err(CliError::Io(format!(
                "created the store in {}, but could not finish: {error}. Don't run `magpie \
                 init` again; fix the problem, then run `magpie checkpoint --accept-current`.",
                dir.display()
            )));
        }
        Ok((store, writer.tip()))
    }

    /// Write the signing key, the config, and the genesis event.
    fn create_files(&self, signing_key: SigningKey) -> Result<LogWriter<FileStore>, CliError> {
        write_new(
            &self.dir.join(KEY_FILE),
            format!("{}\n", hex::encode(signing_key.to_bytes())).as_bytes(),
            true,
        )?;
        let config = StoreConfig {
            format: STORE_FORMAT.into(),
            agent: self.agent.clone(),
            verifying_key: hex::encode(self.verifying_key.as_bytes()),
        };
        let mut config_text = serde_json::to_string_pretty(&config)?;
        config_text.push('\n');
        write_new(&self.dir.join(CONFIG_FILE), config_text.as_bytes(), false)?;
        // Opening an empty FileStore writes the library's genesis event.
        LogWriter::<FileStore>::open(FileStore::new(self.log_path()), signing_key)
            .map_err(log_failure)
    }

    /// Remove the store files a failed `init` created, so a retry starts clean.
    ///
    /// Nothing here was ever a usable store: the genesis event did not finish,
    /// so the key has signed nothing that survives.
    fn discard_partial(&self, error: CliError) -> CliError {
        let mut left = Vec::new();
        for name in [LOG_FILE, CONFIG_FILE, KEY_FILE] {
            match fs::remove_file(self.dir.join(name)) {
                Ok(()) => {}
                Err(removal) if removal.kind() == ErrorKind::NotFound => {}
                Err(_) => left.push(name),
            }
        }
        if left.is_empty() {
            return error;
        }
        CliError::Io(format!(
            "{error}. The partly created store could not be removed, so delete {} in {} before \
             retrying.",
            left.join(", "),
            self.dir.display()
        ))
    }

    /// Open an existing store without touching its signing key.
    pub(crate) fn open(dir: &Path) -> Result<Store, CliError> {
        let config_path = dir.join(CONFIG_FILE);
        let text = fs::read_to_string(&config_path).map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                CliError::Usage(format!(
                    "{} is not a Magpie store (no {CONFIG_FILE}); create one with `magpie init <dir>`",
                    dir.display()
                ))
            } else {
                error.into()
            }
        })?;
        let config: StoreConfig = serde_json::from_str(&text).map_err(|error| {
            CliError::Refused(format!("{} is unreadable: {error}", config_path.display()))
        })?;
        if config.format != STORE_FORMAT {
            return Err(CliError::Refused(format!(
                "{} has format `{}`, but this tool reads `{STORE_FORMAT}`",
                config_path.display(),
                config.format
            )));
        }
        let verifying_key = parse_verifying_key(&config.verifying_key)?;
        Ok(Store {
            dir: dir.to_path_buf(),
            agent: config.agent,
            verifying_key,
        })
    }

    /// A store opened only to verify its log against a caller-supplied key.
    ///
    /// It reads neither `config.json` nor `signing.key`, so an off-machine key
    /// can still check the log when that local state is damaged. It has no
    /// agent and must never be used to write.
    pub(crate) fn for_verification(
        dir: &Path,
        verifying_key: VerifyingKey,
    ) -> Result<Store, CliError> {
        let store = Store {
            dir: dir.to_path_buf(),
            agent: String::new(),
            verifying_key,
        };
        // Checked before anything takes a lock, so a mistyped path gets a
        // clear answer instead of a stray lock file.
        if !store.log_path().is_file() {
            return Err(CliError::Usage(format!(
                "{} is not a Magpie store (no {LOG_FILE})",
                dir.display()
            )));
        }
        Ok(store)
    }

    pub(crate) fn log_path(&self) -> PathBuf {
        self.dir.join(LOG_FILE)
    }

    pub(crate) fn config_path(&self) -> PathBuf {
        self.dir.join(CONFIG_FILE)
    }

    /// Flush the log and the store directory's entries to disk.
    ///
    /// A checkpoint must never reach disk ahead of the events it names, or a
    /// power cut could leave a log that no longer contains its own checkpoint.
    /// So every checkpoint save runs this first, including the one that
    /// recovers from an earlier failed sync.
    pub(crate) fn sync_log(&self) -> Result<(), CliError> {
        sync_file(&self.log_path())?;
        sync_dir(&self.dir)
    }

    pub(crate) fn agent(&self) -> &str {
        &self.agent
    }

    /// The agent to record as the author of new events.
    ///
    /// `init` refuses an empty name, but `config.json` can be edited later, so
    /// every write checks again rather than sign events with no author.
    pub(crate) fn writing_agent(&self) -> Result<&str, CliError> {
        if self.agent.trim().is_empty() {
            return Err(CliError::Refused(format!(
                "{} records an empty agent name, so new events would have no author; set \
                 \"agent\" there, then retry",
                self.config_path().display()
            )));
        }
        Ok(&self.agent)
    }

    pub(crate) fn verifying_key(&self) -> VerifyingKey {
        self.verifying_key
    }

    /// Load the signing key, refusing one that doesn't match the recorded
    /// verifying key.
    pub(crate) fn signing_key(&self) -> Result<SigningKey, CliError> {
        let path = self.dir.join(KEY_FILE);
        let text = fs::read_to_string(&path)?;
        let bytes: [u8; 32] = hex::decode(text.trim())
            .ok()
            .and_then(|bytes| bytes.try_into().ok())
            .ok_or_else(|| {
                CliError::Refused(format!(
                    "{} does not hold a 64-character hex signing key",
                    path.display()
                ))
            })?;
        let key = SigningKey::from_bytes(&bytes);
        if key.verifying_key() != self.verifying_key {
            return Err(CliError::Refused(format!(
                "{KEY_FILE} does not match the verifying key recorded in {CONFIG_FILE}; nothing was written"
            )));
        }
        Ok(key)
    }

    /// Take the store's advisory lock; it is released when the file is dropped.
    pub(crate) fn lock(&self, mode: LockMode) -> Result<Option<File>, CliError> {
        lock_file(
            &self.dir.join(LOCK_FILE),
            mode,
            "another magpie command is using this store; try again when it finishes",
        )
    }

    /// Take the lock on this key's checkpoint.
    ///
    /// Checkpoints are shared by every copy of a store that holds the same key,
    /// while the store lock covers only one directory. Writes hold this lock
    /// from checking the checkpoint until replacing it, so two copies cannot
    /// both extend the same checkpoint and fork the history. The checkpoint
    /// itself is replaced atomically, so a reader that finds no lock file to
    /// wait on still sees a whole checkpoint or none.
    pub(crate) fn lock_checkpoint(&self, mode: LockMode) -> Result<Option<File>, CliError> {
        let path = self.checkpoint_path()?.with_extension("lock");
        if let (LockMode::Exclusive, Some(parent)) = (mode, path.parent()) {
            create_dir_all_durable(parent)?;
        }
        lock_file(
            &path,
            mode,
            "another magpie command is using this key's checkpoint (perhaps from a copy of \
             this store); try again when it finishes",
        )
    }

    /// The state directory for this store's checkpoint and scratch copies.
    ///
    /// Refused when it resolves inside the store or the directory holding the
    /// real log, or when it holds the real log itself, symlinks included. A
    /// checkpoint kept with the log is backed up and restored along with it,
    /// so a restored older log would still contain its equally old checkpoint
    /// and the rollback would pass unnoticed.
    fn state_dir(&self) -> Result<PathBuf, CliError> {
        let state = configured_state_dir()?;
        let unresolved = |error: std::io::Error| {
            CliError::Io(format!(
                "could not check the state directory {} against the store: {error}",
                state.display()
            ))
        };
        let resolved_state = resolve(&state).map_err(unresolved)?;
        let resolved_store = resolve(&self.dir).map_err(unresolved)?;
        // Where the log's bytes really live, which differs from the store
        // directory when log.jsonl is a link.
        let resolved_log = resolve(&self.log_path()).map_err(unresolved)?;
        let log_dir = resolved_log.parent().unwrap_or(&resolved_log);
        let reason = if resolved_state.starts_with(&resolved_store) {
            Some(format!("is inside the store {}", self.dir.display()))
        } else if resolved_state.starts_with(log_dir) {
            Some(format!(
                "is inside {}, which holds the log that log.jsonl links to",
                log_dir.display()
            ))
        } else if resolved_log.starts_with(&resolved_state) {
            Some("holds the log that log.jsonl links to".to_owned())
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(CliError::Usage(format!(
                "the state directory {} {reason}, so its checkpoints would be backed up and \
                 restored with the log and could not reveal a rollback; set MAGPIE_STATE_DIR \
                 to a directory apart from the log",
                state.display()
            )));
        }
        Ok(state)
    }

    /// Where the portable verifier's private copies of the log may go, in
    /// order: the state directory, which is private to this user, then the
    /// system temporary directory. Never the store directory, which may be
    /// read-only, and never a state directory refused for lying inside it.
    pub(crate) fn scratch_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Ok(state) = self.state_dir() {
            dirs.push(state.join("scratch"));
        }
        dirs.push(std::env::temp_dir());
        dirs
    }

    fn checkpoint_path(&self) -> Result<PathBuf, CliError> {
        Ok(self.state_dir()?.join("checkpoints").join(format!(
            "{}.json",
            hex::encode(self.verifying_key.as_bytes())
        )))
    }

    pub(crate) fn saved_checkpoint(&self) -> Result<Option<SavedCheckpoint>, CliError> {
        let path = self.checkpoint_path()?;
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let record: CheckpointRecord = serde_json::from_str(&text).map_err(|error| {
            CliError::Refused(format!(
                "the saved checkpoint {} is unreadable: {error}",
                path.display()
            ))
        })?;
        if record.format != CHECKPOINT_FORMAT {
            return Err(CliError::Refused(format!(
                "the saved checkpoint {} has format `{}`, but this tool reads `{CHECKPOINT_FORMAT}`",
                path.display(),
                record.format
            )));
        }
        Ok(Some(SavedCheckpoint {
            event_count: record.event_count,
            tip: record.tip,
            path,
        }))
    }

    /// Save `(event_count, tip)` atomically: write a temporary file, sync, rename.
    pub(crate) fn save_checkpoint(
        &self,
        event_count: u64,
        tip: ContentHash,
    ) -> Result<(), CliError> {
        let path = self.checkpoint_path()?;
        let Some(parent) = path.parent() else {
            return Err(CliError::Io(format!(
                "{} has no parent directory",
                path.display()
            )));
        };
        create_dir_all_durable(parent)?;
        let record = CheckpointRecord {
            format: CHECKPOINT_FORMAT.into(),
            event_count,
            tip,
            log: self.log_path().display().to_string(),
        };
        let mut text = serde_json::to_string_pretty(&record)?;
        text.push('\n');
        // Clear any leftover from an interrupted save, then create the file
        // exclusively. `create_new` refuses anything already at the path, a
        // planted symlink included, so the write can't be redirected into
        // another file even if the state directory is shared. Removing a
        // symlink removes only the link. The checkpoint lock keeps other
        // `magpie` commands off this path.
        let temporary = path.with_extension("json.tmp");
        let _ = fs::remove_file(&temporary);
        write_new(&temporary, text.as_bytes(), false)?;
        if let Err(error) = fs::rename(&temporary, &path) {
            let _ = fs::remove_file(&temporary);
            return Err(error.into());
        }
        // The rename survives a power cut only once its directory is synced.
        // Without this, a crash could keep the appended event but lose the
        // new checkpoint, and a later restore to the old tip would pass.
        sync_dir(parent)
    }

    /// Verify the log and check it still contains the saved checkpoint.
    pub(crate) fn check_checkpoint<S: LogStore>(
        &self,
        reader: &LogReader<S>,
    ) -> Result<CheckpointState, CliError> {
        let Some(saved) = self.saved_checkpoint()? else {
            return Ok(CheckpointState::Missing);
        };
        let evaluation = reader
            .evaluate_history_expectation_v0(
                HistoryCheckpointV0::new(saved.event_count, saved.tip),
                HistoryExpectationRelationV0::ContainsCheckpoint,
            )
            .map_err(log_failure)?;
        match evaluation.outcome() {
            HistoryExpectationOutcomeV0::Satisfied => Ok(CheckpointState::Contained(saved)),
            HistoryExpectationOutcomeV0::NotSatisfied => Err(CliError::Refused(format!(
                "this log does not contain the checkpoint saved in {} (event {}, tip {}). It may \
                 have been rolled back to an older copy or replaced. If you restored it on \
                 purpose, run `magpie checkpoint --accept-current`.",
                saved.path.display(),
                saved.event_count,
                saved.tip
            ))),
        }
    }
}

/// The configured state directory, before it is checked against a store.
fn configured_state_dir() -> Result<PathBuf, CliError> {
    let from_env = |name: &str| std::env::var_os(name).filter(|value| !value.is_empty());
    if let Some(dir) = from_env("MAGPIE_STATE_DIR") {
        return Ok(PathBuf::from(dir));
    }
    if let Some(dir) = from_env("XDG_STATE_HOME") {
        return Ok(PathBuf::from(dir).join("magpie"));
    }
    if let Some(home) = from_env("HOME") {
        return Ok(PathBuf::from(home)
            .join(".local")
            .join("state")
            .join("magpie"));
    }
    if let Some(dir) = from_env("LOCALAPPDATA") {
        return Ok(PathBuf::from(dir).join("magpie").join("state"));
    }
    Err(CliError::Usage(
        "cannot find a state directory for checkpoints; set MAGPIE_STATE_DIR".into(),
    ))
}

/// `path` made absolute with its symlinks resolved, even when its last
/// components don't exist yet: those can't be links, so they are appended as
/// written. A missing component followed by `..` can't be resolved and is an
/// error, which callers treat as a refusal.
fn resolve(path: &Path) -> std::io::Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut existing = absolute.as_path();
    let mut missing = Vec::new();
    loop {
        match fs::canonicalize(existing) {
            Ok(mut resolved) => {
                resolved.extend(missing.iter().rev());
                return Ok(resolved);
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                let (Some(name), Some(parent)) = (existing.file_name(), existing.parent()) else {
                    return Err(error);
                };
                missing.push(name);
                existing = parent;
            }
            Err(error) => return Err(error),
        }
    }
}

/// Refuse a directory that already has a store file, even a dangling link.
fn refuse_existing(dir: &Path) -> Result<(), CliError> {
    for name in [LOG_FILE, KEY_FILE, CONFIG_FILE] {
        match fs::symlink_metadata(dir.join(name)) {
            Ok(_) => {
                return Err(CliError::Usage(format!(
                    "{} already holds a Magpie store ({name} exists); refusing to replace it",
                    dir.display()
                )))
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

/// Take an advisory lock, retrying briefly while another command holds it.
///
/// Only an exclusive lock creates the lock file. A shared lock opens it
/// read-only, so reads work on read-only media, and returns `None` when there
/// is no lock file: every writer creates it before writing, so no `magpie`
/// writer was running when we looked. One that starts afterwards can race the
/// read, but a torn read fails the portable check and is refused.
fn lock_file(path: &Path, mode: LockMode, busy: &str) -> Result<Option<File>, CliError> {
    let opened = match mode {
        LockMode::Shared => OpenOptions::new().read(true).open(path),
        LockMode::Exclusive => OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path),
    };
    let file = match opened {
        Ok(file) => file,
        Err(error) if matches!(mode, LockMode::Shared) && error.kind() == ErrorKind::NotFound => {
            return Ok(None)
        }
        Err(error) => return Err(error.into()),
    };
    for _ in 0..LOCK_ATTEMPTS {
        let attempt = match mode {
            LockMode::Shared => file.try_lock_shared(),
            LockMode::Exclusive => file.try_lock(),
        };
        match attempt {
            Ok(()) => return Ok(Some(file)),
            Err(TryLockError::WouldBlock) => thread::sleep(LOCK_RETRY),
            Err(TryLockError::Error(error)) => return Err(error.into()),
        }
    }
    Err(CliError::Refused(busy.to_owned()))
}

/// Flush a file to disk. The handle is opened for writing because Windows'
/// `FlushFileBuffers` refuses a read-only handle; nothing is written through it.
fn sync_file(path: &Path) -> Result<(), CliError> {
    OpenOptions::new().write(true).open(path)?.sync_all()?;
    Ok(())
}

/// Create `dir` and any missing parents, syncing each parent that gains an
/// entry. A plain `create_dir_all` can lose new directories in a power cut
/// even after the files inside them are synced; losing the checkpoint
/// directory would quietly turn rollback detection into a warning.
fn create_dir_all_durable(dir: &Path) -> Result<(), CliError> {
    if dir.as_os_str().is_empty() || dir.is_dir() {
        return Ok(());
    }
    let parent = entry_parent(dir);
    if let Some(parent) = parent {
        create_dir_all_durable(parent)?;
    }
    match fs::create_dir(dir) {
        Ok(()) => {}
        // Another process created it first. Sync the parent anyway rather than
        // rely on that process having done so.
        Err(error) if error.kind() == ErrorKind::AlreadyExists && dir.is_dir() => {}
        Err(error) => return Err(error.into()),
    }
    match parent {
        Some(parent) => sync_dir(parent),
        None => Ok(()),
    }
}

/// The directory holding `path`'s entry. For a bare relative name such as
/// `ledger`, `Path::parent` is empty and the entry lives in the current
/// directory, which must be synced like any other parent.
fn entry_parent(path: &Path) -> Option<&Path> {
    match path.parent() {
        Some(parent) if parent.as_os_str().is_empty() => Some(Path::new(".")),
        parent => parent,
    }
}

fn sync_dir(dir: &Path) -> Result<(), CliError> {
    #[cfg(unix)]
    File::open(dir)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = dir;
    Ok(())
}

fn write_new(path: &Path, bytes: &[u8], private: bool) -> Result<(), CliError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    if private {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    #[cfg(not(unix))]
    let _ = private;
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failed_init_removes_its_store_files_and_names_any_it_cannot() {
        let dir = std::env::temp_dir().join(format!("magpie-cli-discard-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let store = Store {
            dir: dir.clone(),
            agent: "tester".into(),
            verifying_key: SigningKey::from_bytes(&[3u8; 32]).verifying_key(),
        };
        // An attempt that wrote the key and config, then failed at genesis.
        fs::write(dir.join(KEY_FILE), "key\n").unwrap();
        fs::write(dir.join(CONFIG_FILE), "{}\n").unwrap();
        fs::write(dir.join(LOCK_FILE), "").unwrap();
        let error = store.discard_partial(CliError::Io("disk full".into()));
        assert_eq!(error.to_string(), "disk full");
        assert!(!dir.join(KEY_FILE).exists());
        assert!(!dir.join(CONFIG_FILE).exists());
        assert!(dir.join(LOCK_FILE).exists(), "only store files are removed");

        // A file that can't be removed is named, so the retry can be unblocked.
        fs::create_dir_all(dir.join(LOG_FILE).join("inner")).unwrap();
        let error = store.discard_partial(CliError::Io("disk full".into()));
        assert!(
            error.to_string().contains(&format!("delete {LOG_FILE} in")),
            "{error}"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn resolve_follows_links_and_keeps_missing_components() {
        let root = std::env::temp_dir().join(format!("magpie-cli-resolve-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("store")).unwrap();
        let store = fs::canonicalize(root.join("store")).unwrap();
        assert_eq!(resolve(&root.join("store")).unwrap(), store);
        assert_eq!(
            resolve(&root.join("store").join("new").join("state")).unwrap(),
            store.join("new").join("state")
        );
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(root.join("store"), root.join("link")).unwrap();
            assert_eq!(
                resolve(&root.join("link").join("state")).unwrap(),
                store.join("state")
            );
        }
        assert!(resolve(&root.join("gone").join("..").join("x")).is_err());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_bare_relative_name_lives_in_the_current_directory() {
        assert_eq!(entry_parent(Path::new("ledger")), Some(Path::new(".")));
        assert_eq!(
            entry_parent(Path::new("state/checkpoints")),
            Some(Path::new("state"))
        );
        assert_eq!(
            entry_parent(Path::new("/srv/ledger")),
            Some(Path::new("/srv"))
        );
        assert_eq!(entry_parent(Path::new("/")), None);
    }

    #[test]
    fn durable_directory_creation_makes_every_missing_level_once() {
        let root = std::env::temp_dir().join(format!("magpie-cli-durable-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let nested = root.join("state").join("checkpoints");
        create_dir_all_durable(&nested).unwrap();
        assert!(nested.is_dir());
        // Already there: nothing to do, and no error.
        create_dir_all_durable(&nested).unwrap();
        // A file in the way is an error, not a silent success.
        let blocked = root.join("file");
        fs::write(&blocked, b"").unwrap();
        assert!(create_dir_all_durable(&blocked.join("below")).is_err());
        fs::remove_dir_all(&root).unwrap();
    }
}
