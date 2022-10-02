use crate::error::option_error;
use crate::schema::{Tag, TagQuery};
use crate::service;
use crate::types::{ApiPageResult, ApiResult, PageData, Pool, Resp};
use desire::Request;
use tokio_stream::StreamExt;
pub async fn get_all(req: Request) -> ApiPageResult<Tag> {
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let query = req.get_query::<TagQuery>()?;
  let mut wheres = format!("1 = 1");
  let mut limit = 20;
  let mut page = 1;
  if let Some(query) = query {
    limit = query.limit;
    page = query.page;
    if let Some(user_id) = query.user_id {
      wheres = format!("{} AND user_id = {}", wheres, user_id);
    }
    if let Some(name) = query.name {
      wheres = format!("{} AND name = {}", wheres, name);
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
    "SELECT id, user_id, name,created_at, updated_at FROM tags where {} LIMIT {} OFFSET {}",
    wheres, limit, offset
  );
  let count_sql = format!("SELECT COUNT(1) FROM tags where {}", wheres);
  let mut rows = sqlx::query_as::<_, Tag>(&sql).fetch(pool);
  let mut list = Vec::new();
  while let Some(row) = rows.try_next().await? {
    list.push(row);
  }
  let total: (i64,) = sqlx::query_as(&count_sql).fetch_one(pool).await?;
  let result = PageData::new(list, total.0);
  Ok(Resp::data(result))
}

pub async fn get_by_id(req: Request) -> ApiResult<Tag> {
  let id = req.get_param::<i64>("id")?;
  let result = service::get_tag_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn create(mut req: Request) -> ApiResult<Tag> {
  let tag = req.get_body::<Tag>().await?;
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let sql = "INSERT into tags(user_id, name, created_at, updated_at) VALUES (?,?,?,?)";
  info!("sql {}", sql);
  let result = sqlx::query(sql)
    .bind(&tag.user_id)
    .bind(&tag.name)
    .bind(&tag.created_at)
    .bind(&tag.updated_at)
    .execute(pool)
    .await?;
  let result = service::get_tag_by_id(result.last_insert_rowid()).await?;
  Ok(Resp::data(result))
}

pub async fn update(mut req: Request) -> ApiResult<Tag> {
  let id = req.get_param::<i64>("id")?;
  let tag = req.get_body::<Tag>().await?;
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let result = sqlx::query("UPDATE tags SET name = ?, updated_at = ? WHERE id = ?")
    .bind(&tag.name)
    .bind(&tag.updated_at)
    .bind(id)
    .execute(pool)
    .await?;
  info!("result: {:?}", result);
  let result = service::get_tag_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let id = req.get_param::<i64>("id")?;
  let result = sqlx::query("DELETE FROM tags where id = ?")
    .bind(id)
    .execute(pool)
    .await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}
