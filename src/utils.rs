use chrono::Utc;
use sha2::{Digest, Sha256};

pub fn now_fmt() -> String {
  let dt = Utc::now();
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn sha_256(value: &str, salt: &str) -> String {
  let mut hasher = Sha256::new();
  hasher.update(value.as_bytes());
  hasher.update(salt.as_bytes());
  format!("{:X}", hasher.finalize())
}

pub fn gen_salt() -> String {
  let now = now_fmt();
  let sha = sha_256(&now, "salt_salt");
  let salt = &sha[20..30];
  salt.to_string()
}
