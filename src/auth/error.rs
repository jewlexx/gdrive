use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("http request error: {0}")]
    RequsetError(#[from] reqwest::Error),
    #[error("error handling json: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
