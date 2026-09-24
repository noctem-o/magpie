//! A store directory: its files, its lock, and its rollback checkpoint.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use magpie_log::{
    ContentHash, FileStore, HistoryCheckpointV0, HistoryExpectationOutcomeV0,
    HistoryExpectationRelationV0, LogError, LogReader, LogWriter, SigningKey, VerifyingKey,
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
        for name in [LOG_FILE, KEY_FILE, CONFIG_FILE] {
            if dir.join(name).exists() {
                return Err(CliError::Usage(format!(
                    "{} already holds a Magpie store; refusing to replace it",
                    dir.display()
                )));
            }
        }
        if agent.trim().is_empty() {
            return Err(CliError::Usage("--agent must not be empty".into()));
        }
        fs::create_dir_all(dir)?;

        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed).map_err(|error| {
            CliError::Io(format!(
                "could not gather randomness for the signing key: {error}"
            ))
        })?;
        let signing_key = SigningKey::from_bytes(&seed);
        seed.fill(0);
        let verifying_key = signing_key.verifying_key();

        write_new(
            &dir.join(KEY_FILE),
            format!("{}\n", hex::encode(signing_key.to_bytes())).as_bytes(),
            true,
        )?;
        let config = StoreConfig {
            format: STORE_FORMAT.into(),
            agent: agent.clone(),
            verifying_key: hex::encode(verifying_key.as_bytes()),
        };
        let mut config_text = serde_json::to_string_pretty(&config)?;
        config_text.push('\n');
        write_new(&dir.join(CONFIG_FILE), config_text.as_bytes(), false)?;

        let store = Store {
            dir: dir.to_path_buf(),
            agent,
            verifying_key,
        };
        let _lock = store.lock(LockMode::Exclusive)?;
        // Opening an empty FileStore writes the library's genesis event.
        let writer = LogWriter::<FileStore>::open(FileStore::new(store.log_path()), signing_key)
            .map_err(log_failure)?;
        sync_file(&store.log_path())?;
        sync_dir(dir)?;
        store.save_checkpoint(writer.len(), writer.tip())?;
        Ok((store, writer.tip()))
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

    pub(crate) fn log_path(&self) -> PathBuf {
        self.dir.join(LOG_FILE)
    }

    pub(crate) fn agent(&self) -> &str {
        &self.agent
    }

    pub(crate) fn verifying_key(&self) -> VerifyingKey {
        self.verifying_key
    }

    pub(crate) fn reader(&self) -> LogReader<FileStore> {
        self.reader_with_key(self.verifying_key)
    }

    pub(crate) fn reader_with_key(&self, verifying_key: VerifyingKey) -> LogReader<FileStore> {
        LogReader::open(FileStore::new(self.log_path()), verifying_key)
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
    pub(crate) fn lock(&self, mode: LockMode) -> Result<File, CliError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.dir.join(LOCK_FILE))?;
        for _ in 0..LOCK_ATTEMPTS {
            let attempt = match mode {
                LockMode::Shared => file.try_lock_shared(),
                LockMode::Exclusive => file.try_lock(),
            };
            match attempt {
                Ok(()) => return Ok(file),
                Err(TryLockError::WouldBlock) => thread::sleep(LOCK_RETRY),
                Err(TryLockError::Error(error)) => return Err(error.into()),
            }
        }
        Err(CliError::Refused(
            "another magpie command is using this store; try again when it finishes".into(),
        ))
    }

    /// Refuse to append after a torn final record.
    ///
    /// `FileStore` writes a record and its newline separately. If a crash
    /// landed between the two, the next append would run two records together
    /// and break the chain, so this check runs before every write.
    pub(crate) fn ensure_terminated(&self) -> Result<(), CliError> {
        let path = self.log_path();
        let mut file = File::open(&path)?;
        if file.metadata()?.len() == 0 {
            return Err(CliError::Refused(format!(
                "{} is empty, so it has no genesis event; create a new store with `magpie init`",
                path.display()
            )));
        }
        file.seek(SeekFrom::End(-1))?;
        let mut last = [0u8; 1];
        file.read_exact(&mut last)?;
        if last[0] != b'\n' {
            return Err(CliError::Refused(format!(
                "{} ends part-way through a record, probably from an interrupted append, so \
                 nothing was written. If its last line is a complete record, add the missing \
                 newline; then run `magpie verify`.",
                path.display()
            )));
        }
        Ok(())
    }

    fn checkpoint_path(&self) -> Result<PathBuf, CliError> {
        Ok(state_dir()?.join("checkpoints").join(format!(
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
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let record = CheckpointRecord {
            format: CHECKPOINT_FORMAT.into(),
            event_count,
            tip,
            log: self.log_path().display().to_string(),
        };
        let mut text = serde_json::to_string_pretty(&record)?;
        text.push('\n');
        let temporary = path.with_extension("json.tmp");
        {
            let mut file = File::create(&temporary)?;
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
        }
        fs::rename(&temporary, &path)?;
        Ok(())
    }

    /// Verify the log and check it still contains the saved checkpoint.
    pub(crate) fn check_checkpoint(
        &self,
        reader: &LogReader<FileStore>,
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

/// Where checkpoints live: deliberately outside any store directory.
fn state_dir() -> Result<PathBuf, CliError> {
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

pub(crate) fn sync_file(path: &Path) -> Result<(), CliError> {
    File::open(path)?.sync_all()?;
    Ok(())
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
