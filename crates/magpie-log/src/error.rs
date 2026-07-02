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
}
