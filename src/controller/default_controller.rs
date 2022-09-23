use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use crate::config::{CLEAR_SQL, DATABASE_URI, INIT_SQL};
use desire::{IntoResponse, Request};
use rusqlite::Connection;

pub async fn hello(_req: Request) -> impl IntoResponse {
  "Hello World!"
}

pub async fn hello_option(_req: Request) -> ApiOptionResult<String> {
  let result = Some("hello_option".to_string());
  Ok(Resp::data(result))
}
pub async fn hello_page(_req: Request) -> ApiPageResult<String> {
  let page_data = PageData::new(Vec::<String>::new(), 0);
  Ok(Resp::data(page_data))
}
pub async fn db_init(_req: Request) -> ApiResult<String> {
  info!("run init sql: {}", INIT_SQL);
  let conn = Connection::open(DATABASE_URI)?;
  conn.execute_batch(INIT_SQL)?;
  Ok(Resp::data("OK".to_string()))
}

pub async fn db_reset(_req: Request) -> ApiResult<String> {
  info!("run clear sql: {}", CLEAR_SQL);
  let conn = Connection::open(DATABASE_URI)?;
  conn.execute_batch(CLEAR_SQL)?;
  Ok(Resp::data("OK".to_string()))
}