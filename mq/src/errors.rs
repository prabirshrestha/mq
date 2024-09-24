use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Not supported error: {0}")]
    NotSupported(String),

    #[error("Other error: {0}")]
    OtherError(#[from] Box<dyn std::error::Error + Sync + Send>),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}
