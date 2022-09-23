use super::types::{ApiResult, Resp};
use crate::config::DATABASE_URI;
use desire::{IntoResponse, Request};
use rusqlite::Connection;

pub async fn hello(_req: Request) -> impl IntoResponse {
  "Hello World!"
}

pub async fn db_init(_req: Request) -> ApiResult<String> {
  let conn = Connection::open(DATABASE_URI)?;
  conn.execute(
    "CREATE TABLE users (
      id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
      username TEXT,
      password TEXT,
      age INTEGER,
      meta TEXT,
      created_at TEXT,
      updated_at TEXT
    )",
    (),
  )?;
  Ok(Resp::data("OK".to_string()))
}

pub async fn db_reset(_req: Request) -> impl IntoResponse {
  let conn = Connection::open(DATABASE_URI)?;
  conn.execute(
    "DROP TABLE users",
    (),
  )?;
  Ok(Resp::data("OK".to_string()))
}
