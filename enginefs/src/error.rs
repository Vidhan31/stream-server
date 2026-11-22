use thiserror::Error;

/// Core error type for EngineFS operations.
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Torrent not found: {0}")]
    TorrentNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File index {index} out of range (total files: {max})")]
    FileIndexOutOfRange { index: usize, max: usize },

    #[error("Session error: {0}")]
    Session(String),

    #[error("Alert channel error: {0}")]
    Alert(String),

    #[error("Playback request was superseded: {0}")]
    Superseded(String),

    #[error("Operation timed out: {0}")]
    Timeout(String),

    #[error("Extraction failed: {0}")]
    Extraction(String),

    #[error("Media probe failed: {0}")]
    Probe(String),

    #[error("Invalid embedded track: {0}")]
    InvalidTrack(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Internal engine error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for EngineError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

pub type Result<T, E = EngineError> = std::result::Result<T, E>;
