use crate::config::{CLEAR_SQL, DATABASE_URI, INIT_SQL};
use crate::schema::{Chart, ChartQuery};
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use chrono::Datelike;
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

pub async fn sign_up(_req: Request) -> ApiResult<String> {
  Ok(Resp::data("sign_up".to_string()))
}

pub async fn sign_in(_req: Request) -> ApiResult<String> {
  Ok(Resp::data("sign_in".to_string()))
}

pub async fn sign_out(_req: Request) -> ApiResult<String> {
  Ok(Resp::data("sign_out".to_string()))
}

pub async fn chart(req: Request) -> ApiPageResult<Chart> {
  let conn = Connection::open(DATABASE_URI)?;
  let query = req.get_query::<ChartQuery>()?;
  let dt = chrono::Utc::now();

  let date_end = dt.format("%Y-%m-%d %H:%M:%S").to_string();
  let date_start = dt
    .with_day(1)
    .unwrap()
    .format("%Y-%m-%d 00:00:00")
    .to_string();
  let query = query.unwrap_or(ChartQuery::new(date_start, date_end));

  info!("query {:?}", query);
  let sql = r#"
  SELECT
    userId,
    weight,
    DATE(createdAt) date
  FROM
    weights
  WHERE
    createdAt >= ?
    AND createdAt <= ?
  GROUP BY
    userId,
    date
"#;

  let mut stmt = conn.prepare(sql)?;
  let chart_iter = stmt.query_map([&query.date_start, &query.date_end], |row| {
    Ok(Chart {
      user_id: row.get(0)?,
      weight: row.get(1)?,
      date: row.get(2)?,
    })
  })?;
  let mut list: Vec<Chart> = Vec::new();
  for chart in chart_iter {
    list.push(chart?);
  }
  let len = list.len() as u64;
  let result = PageData::new(list, len);
  Ok(Resp::data(result))
}
