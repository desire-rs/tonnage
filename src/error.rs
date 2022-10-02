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
  #[error("sqlx::Error {0:?}")]
  SqlxError(#[from] sqlx::Error),
  #[error("redis::Error {0:?}")]
  RedisError(#[from] redis::RedisError),
  #[error("jsonwebtoken::errors::Error {0:?}")]
  JwtError(#[from] jsonwebtoken::errors::Error),
  #[error("unwrap `{0}` is not none")]
  OptionError(String),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    let val = self.to_string();
    Response::with_status(500, val)
  }
}

pub fn option_error(msg: &str) -> Error {
  Error::OptionError(msg.to_string())
}
