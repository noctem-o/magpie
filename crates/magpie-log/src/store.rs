use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::error::LogError;

/// Compatibility read surface for one serialized [`crate::SignedEvent`] per record.
///
/// This deliberately tiny whole-vector trait remains the FileStore/MemStore and
/// downstream custom-reader boundary. The supported bounded SQLite L0 path has
/// a separate stable-snapshot implementation and does not implement this trait.
pub trait LogStore {
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError>;
}

/// Writer-owned persistence seam. This trait is deliberately crate-private:
/// public storage substitution is read-only, while only `LogWriter` may ask a
/// supported backend to persist a record.
pub(crate) trait WriterStore: LogStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError>;
}

/// Simple JSONL compatibility/development/inspection storage.
///
/// This preserves its existing append behavior but makes no supported SQLite
/// L0 transaction, concurrent-writer, or crash-durability guarantee.
pub struct FileStore {
    path: PathBuf,
}

impl FileStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Read one exact finite file image for the crate-internal portable path.
    ///
    /// Unlike [`LogStore::read_records`], this retains every byte so framing,
    /// blank records, and terminators remain visible to the portable frontend.
    /// A missing compatibility file continues to denote the empty history.
    pub(crate) fn read_portable_input_bytes(&self) -> Result<Vec<u8>, LogError> {
        match std::fs::read(&self.path) {
            Ok(data) => Ok(data),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error.into()),
        }
    }
}

impl LogStore for FileStore {
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

impl WriterStore for FileStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        f.write_all(bytes)?;
        f.write_all(b"\n")?;
        Ok(())
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
    fn read_records(&self) -> Result<Vec<Vec<u8>>, LogError> {
        Ok(self.records.borrow().clone())
    }
}

impl WriterStore for MemStore {
    fn append_record(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        self.records.borrow_mut().push(bytes.to_vec());
        Ok(())
    }
}
