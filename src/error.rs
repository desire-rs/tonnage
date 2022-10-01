use desire::{IntoResponse, Response, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
  #[error("anyhow error {0:?}")]
  AnyhowError(#[from] anyhow::Error),
  #[error("desire error {0:?}")]
  DesireError(#[from] desire::Error),
  #[error("json error {0:?}")]
  JsonError(#[from] serde_json::Error),
  #[error("io error")]
  IoError(#[from] std::io::Error),
  #[error("rusqlite::Error {0:?}")]
  SqliteError(#[from] rusqlite::Error),
  #[error("jsonwebtoken::errors::Error {0:?}")]
  JwtError(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    let val = self.to_string();
    Response::with_status(500, val)
  }
}
