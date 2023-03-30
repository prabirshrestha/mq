use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("underlying I/O")]
    IO(#[from] std::io::Error),

    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),

    #[error("Other error")]
    OtherError(#[from] Box<dyn std::error::Error + Sync + Send>),

    #[error("Unknown Error")]
    UnknownError(String),
}
