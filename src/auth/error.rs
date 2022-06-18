use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("http request error: {0}")]
    RequsetError(#[from] reqwest::Error),
    #[error("error handling json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("error parsing hex: {0}")]
    HexError(#[from] hex::FromHexError),
    #[error("invalid utf8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
