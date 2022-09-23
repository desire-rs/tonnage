use desire::{IntoResponse, Response, Result};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
  #[error("desire error {0:?}")]
  DesireError(#[from] desire::Error),
  #[error("json error {0:?}")]
  JsonError(#[from] serde_json::Error),
  #[error("io error")]
  IoError(#[from] std::io::Error),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    error!("err {:?}", self);
    let val = self.to_string();
    Response::with_status(500, val)
  }
}
