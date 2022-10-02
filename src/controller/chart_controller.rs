use crate::libs::get_pool;
use crate::schema::{Chart, ChartQuery};
use crate::types::{ApiPageResult, PageData, Resp};
use chrono::Datelike;
use desire::Request;

pub async fn chart(req: Request) -> ApiPageResult<Chart> {
  let pool = get_pool().await?;
  let query = req.get_query::<ChartQuery>()?;
  let dt = chrono::Utc::now();

  let date_end = dt.format("%Y-%m-%d %H:%M:%S").to_string();
  let date_start = dt
    .with_day(1)
    .unwrap()
    .format("%Y-%m-%d 00:00:00")
    .to_string();

  let mut wheres = format!("1 = 1");
  let mut limit = 20;
  let mut page = 1;
  if let Some(mut query) = query {
    limit = query.limit;
    page = query.page;
    if query.date_start.is_none() {
      query.date_start = Some(date_start);
    }
    if query.date_end.is_none() {
      query.date_end = Some(date_end);
    }
    if let Some(user_id) = query.user_id {
      wheres = format!("{} AND user_id = {}", wheres, user_id);
    }
    if let Some(date_start) = query.date_start {
      wheres = format!("{} AND created_at >= '{}'", wheres, date_start);
    }
    if let Some(date_end) = query.date_end {
      wheres = format!("{} AND created_at < '{}'", wheres, date_end);
    }
  }
  let offset = (page - 1) * limit;

  let sql = format!(
    r#"
  SELECT
    user_id,
    weight,
    DATE(weights.created_at) date
  FROM
    weights
  WHERE {}
  GROUP BY user_id,date
  LIMIT {} OFFSET {}
  "#,
    wheres, limit, offset
  );

  let count_sql = format!("SELECT COUNT(1) total FROM weights WHERE {}", wheres);
  let total: (i64,) = sqlx::query_as(&count_sql).fetch_one(&pool).await?;
  let rows: Vec<Chart> = sqlx::query_as(&sql).fetch_all(&pool).await?;
  let result = PageData::new(rows, total.0);
  Ok(Resp::data(result))
}
