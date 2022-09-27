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
  let conn = Connection::open(DATABASE_URI.as_str())?;
  conn.execute_batch(INIT_SQL)?;
  Ok(Resp::data("OK".to_string()))
}

pub async fn db_reset(_req: Request) -> ApiResult<String> {
  info!("run clear sql: {}", CLEAR_SQL);
  let conn = Connection::open(DATABASE_URI.as_str())?;
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
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let query = req.get_query::<ChartQuery>()?;
  let dt = chrono::Utc::now();

  let date_end = dt.format("%Y-%m-%d %H:%M:%S").to_string();
  let date_start = dt
    .with_day(1)
    .unwrap()
    .format("%Y-%m-%d 00:00:00")
    .to_string();
  let query = query.unwrap_or(ChartQuery::new(date_start, date_end));

  let mut and_where = "1 = 1".to_string();
  if let Some(user_id) = query.user_id {
    and_where = format!("weights.userId = {}", user_id);
  }
  let sql = format!(
    r#"
  SELECT
    userId,
    users.nickname,
    users.email,
    users.mobile,
    JSON_EXTRACT(users.meta, '$.borderColor') borderColor,
    JSON_EXTRACT(users.meta, '$.backgroundColor') backgroundColor,
    weight,
    DATE(weights.createdAt) date
  FROM
    weights
    INNER JOIN users ON weights.userId = users.id
  WHERE weights.createdAt >= '{}'
    AND weights.createdAt <= '{}'
    AND {}
  GROUP BY userId,date
  LIMIT {} OFFSET {}
  "#,
    &query.date_start,
    &query.date_end,
    and_where.clone(),
    &query.limit,
    &query.offset()
  );

  let count_sql = format!(
    r#"
    SELECT
      COUNT(1) total
    FROM weights
    WHERE weights.createdAt >= '{}'
    AND weights.createdAt <= '{}'
    AND {}
  "#,
    &query.date_start, &query.date_end, and_where
  );

  info!("sql {}", sql);
  let total: u64 = conn
    .query_row(count_sql.as_str(), [], |row| row.get(0))
    .unwrap();
  info!("total {}", total);
  let mut stmt = conn.prepare(sql.as_str())?;
  let mut rows = stmt.raw_query();
  let mut list: Vec<Chart> = Vec::new();
  while let Some(row) = rows.next()? {
    let chart = Chart {
      user_id: row.get(0)?,
      nickname: row.get(1)?,
      email: row.get(2)?,
      mobile: row.get(3)?,
      border_color: row.get(4)?,
      background_color: row.get(5)?,
      weight: row.get(6)?,
      date: row.get(7)?,
    };
    list.push(chart);
  }
  let result = PageData::new(list, total);
  Ok(Resp::data(result))
}
