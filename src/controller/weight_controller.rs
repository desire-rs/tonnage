use crate::libs::get_pool;
use crate::schema::{Weight, WeightQuery};
use crate::service;
use crate::types::{ApiPageResult, ApiResult, PageData, Resp};
use desire::Request;
use tokio_stream::StreamExt;
pub async fn get_all(req: Request) -> ApiPageResult<Weight> {
  let pool = get_pool().await?;
  let query = req.query::<WeightQuery>()?;
  let mut wheres = format!("1 = 1");
  let mut limit = 20;
  let mut page = 1;
  if let Some(query) = query {
    limit = query.limit;
    page = query.page;
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
    "SELECT * FROM weights where {} LIMIT {} OFFSET {}",
    wheres, limit, offset
  );
  let count_sql = format!("SELECT COUNT(1) FROM weights where {}", wheres);
  let mut rows = sqlx::query_as::<_, Weight>(&sql).fetch(&pool);
  let mut list = Vec::new();
  while let Some(row) = rows.try_next().await? {
    list.push(row);
  }
  let total: (i64,) = sqlx::query_as(&count_sql).fetch_one(&pool).await?;
  let result = PageData::new(list, total.0);
  Ok(Resp::data(result))
}

pub async fn get_by_id(req: Request) -> ApiResult<Weight> {
  let id = req.param::<i64>("id")?;
  let result = service::get_weight_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn create(mut req: Request) -> ApiResult<Weight> {
  let item = req.body::<Weight>().await?;
  let sql = "INSERT into weights(user_id, weight, created_at, updated_at) VALUES (?,?,?,?)";
  info!("sql {}", sql);
  let pool = get_pool().await?;
  let result = sqlx::query(sql)
    .bind(&item.user_id)
    .bind(&item.weight)
    .bind(&item.created_at)
    .bind(&item.updated_at)
    .execute(&pool)
    .await?;
  let id = result.last_insert_rowid();
  let result = service::get_weight_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn update(mut req: Request) -> ApiResult<Weight> {
  let pool = get_pool().await?;
  let id = req.param::<i64>("id")?;
  let weight = req.body::<Weight>().await?;
  let result = sqlx::query("UPDATE weights SET weight = ?, updated_at = ? WHERE id = ?")
    .bind(&weight.weight)
    .bind(&weight.updated_at)
    .bind(id)
    .execute(&pool)
    .await?;
  info!("result: {:?}", result);
  let result = service::get_weight_by_id(result.last_insert_rowid()).await?;
  Ok(Resp::data(result))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let pool = get_pool().await?;
  let id = req.param::<i64>("id")?;
  let result = sqlx::query("DELETE FROM weights where id = ?")
    .bind(id)
    .execute(&pool)
    .await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}