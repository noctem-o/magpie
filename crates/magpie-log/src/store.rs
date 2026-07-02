use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::error::LogError;

/// Where the log's bytes live. One serialized [`crate::SignedEvent`] per record.
/// Deliberately tiny: the log is an abstraction, storage is swappable (file now;
/// a NAS path, an mmap, or a SQLite WAL later).
pub trait LogStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError>;
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError>;
}

/// Real, durable storage: an append-only file, one JSON record per line.
pub struct FileStore {
    path: PathBuf,
}

impl FileStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl LogStore for FileStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        f.write_all(bytes)?;
        f.write_all(b"\n")?;
        Ok(())
    }

    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let data = std::fs::read(&self.path)?;
        Ok(data
            .split(|b| *b == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.to_vec())
            .collect())
    }
}

/// In-memory storage, cheap to clone (clones share the same buffer), so a writer
/// and a reader can address the same log. For tests and embedding. Single-thread
/// only (swap `Rc`/`RefCell` for `Arc`/`Mutex` if you need threads).
#[derive(Clone, Default)]
pub struct MemStore {
    records: Rc<RefCell<Vec<Vec<u8>>>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_records(records: Vec<Vec<u8>>) -> Self {
        Self {
            records: Rc::new(RefCell::new(records)),
        }
    }

    /// A snapshot of the stored records (for inspection or tamper tests).
    pub fn records(&self) -> Vec<Vec<u8>> {
        self.records.borrow().clone()
    }
}

impl LogStore for MemStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        self.records.borrow_mut().push(bytes.to_vec());
        Ok(())
    }

    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        Ok(self.records.borrow().clone())
    }
}
