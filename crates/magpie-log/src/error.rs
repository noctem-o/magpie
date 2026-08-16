#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("chain broken at seq {seq}: {detail}")]
    ChainBroken { seq: u64, detail: String },
    #[error("invalid signature at seq {seq}")]
    BadSignature { seq: u64 },
    #[error("invalid L0 resource limits: {detail}")]
    InvalidResourceLimits { detail: String },
    #[error("L0 resource limit exceeded for {resource}: actual {actual}, limit {limit}")]
    ResourceLimit {
        resource: &'static str,
        actual: u64,
        limit: u64,
    },
    #[error("foreign or unowned SQLite L0 database: {detail}")]
    ForeignDatabase { detail: String },
    #[error("unsupported SQLite L0 database: {detail}")]
    UnsupportedDatabase { detail: String },
    #[error("log sequence is exhausted")]
    SequenceExhausted,
    #[error("SQLite L0 writer is stale")]
    WriterStale,
    #[error("SQLite L0 writer is poisoned; reopen is required")]
    WriterPoisoned,
    #[error("SQLite L0 commit state is unknown; reopen and verify before retrying")]
    CommitStateUnknown,
    #[error("SQLite L0 operation {operation} was busy; the append definitely did not commit")]
    StorageBusy { operation: &'static str },
    #[error("SQLite L0 operation {operation} failed: {detail}")]
    SqliteOperational {
        operation: &'static str,
        detail: String,
    },
}
