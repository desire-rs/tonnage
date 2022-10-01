use crate::schema::{TokenData};
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};

use desire::{IntoResponse, Request};
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

pub async fn token_data(req: Request) -> ApiResult<TokenData> {
  let token_data = req
    .inner
    .extensions()
    .get::<TokenData>()
    .ok_or_else(|| anyhow::anyhow!("token is none"))
    .map(|x| x.clone())?;
  info!("{:?}", token_data);
  Ok(Resp::data(token_data))
}

// pub async fn chart(req: Request) -> ApiPageResult<Chart> {
//   let conn = Connection::open(DATABASE_URI.as_str())?;
//   let query = req.get_query::<ChartQuery>()?;
//   let dt = chrono::Utc::now();

//   let date_end = dt.format("%Y-%m-%d %H:%M:%S").to_string();
//   let date_start = dt
//     .with_day(1)
//     .unwrap()
//     .format("%Y-%m-%d 00:00:00")
//     .to_string();
//   let query = query.unwrap_or(ChartQuery::new(date_start, date_end));

//   let mut and_where = "1 = 1".to_string();
//   if let Some(user_id) = query.user_id {
//     and_where = format!("weights.user_id = {}", user_id);
//   }
//   let sql = format!(
//     r#"
//   SELECT
//     user_id,
//     users.nickname,
//     users.email,
//     users.mobile,
//     JSON_EXTRACT(users.meta, '$.borderColor') borderColor,
//     JSON_EXTRACT(users.meta, '$.backgroundColor') backgroundColor,
//     weight,
//     DATE(weights.created_at) date
//   FROM
//     weights
//     INNER JOIN users ON weights.user_id = users.id
//   WHERE weights.created_at >= '{}'
//     AND weights.created_at <= '{}'
//     AND {}
//   GROUP BY user_id,date
//   LIMIT {} OFFSET {}
//   "#,
//     &query.date_start,
//     &query.date_end,
//     and_where.clone(),
//     &query.limit,
//     &query.offset()
//   );

//   let count_sql = format!(
//     r#"
//     SELECT
//       COUNT(1) total
//     FROM weights
//     WHERE weights.created_at >= '{}'
//     AND weights.created_at <= '{}'
//     AND {}
//   "#,
//     &query.date_start, &query.date_end, and_where
//   );

//   info!("sql {}", sql);
//   let total: u64 = conn
//     .query_row(count_sql.as_str(), [], |row| row.get(0))
//     .unwrap();
//   info!("total {}", total);
//   let mut stmt = conn.prepare(sql.as_str())?;
//   let mut rows = stmt.raw_query();
//   let mut list: Vec<Chart> = Vec::new();
//   while let Some(row) = rows.next()? {
//     let chart = Chart {
//       user_id: row.get(0)?,
//       nickname: row.get(1)?,
//       email: row.get(2)?,
//       mobile: row.get(3)?,
//       border_color: row.get(4)?,
//       background_color: row.get(5)?,
//       weight: row.get(6)?,
//       date: row.get(7)?,
//     };
//     list.push(chart);
//   }
//   let result = PageData::new(list, total);
//   Ok(Resp::data(result))
// }
