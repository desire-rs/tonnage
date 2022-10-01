use crate::libs::get_pool;
use crate::schema::{Prop, Tag, User, Weight};
use crate::types::AnyResult;
use sqlx::Row;
use tokio_stream::StreamExt;
pub async fn get_user_tags(user_id: i64) -> AnyResult<Vec<Tag>> {
  let pool = get_pool().await?;
  let mut list: Vec<Tag> = Vec::new();
  let sql = format!("select id, user_id, name,created_at, updated_at from tags where user_id = ?");
  let mut rows = sqlx::query(&sql).bind(user_id).fetch(&pool);
  while let Some(row) = rows.try_next().await? {
    let tag = Tag {
      id: row.try_get("id")?,
      user_id: row.try_get("user_id")?,
      name: row.try_get("name")?,
      created_at: row.try_get("created_at")?,
      updated_at: row.try_get("updated_at")?,
    };
    list.push(tag);
  }
  Ok(list)
}
pub async fn get_user_props(user_id: i64) -> AnyResult<Vec<Prop>> {
  let mut list: Vec<Prop> = Vec::new();
  let pool = get_pool().await?;
  let sql =
    format!("select id, user_id, name, value, created_at, updated_at from props where user_id = ?");
  let mut rows = sqlx::query(&sql).bind(user_id).fetch(&pool);

  while let Some(row) = rows.try_next().await? {
    let tag = Prop {
      id: row.try_get("id")?,
      user_id: row.try_get("user_id")?,
      name: row.try_get("name")?,
      value: row.try_get("name")?,
      created_at: row.try_get("created_at")?,
      updated_at: row.try_get("updated_at")?,
    };
    list.push(tag);
  }
  Ok(list)
}
pub async fn get_user_by_id(id: i64) -> AnyResult<User> {
  let pool = get_pool().await?;
  let mut user = sqlx::query_as::<_, User>("SELECT * FROM users where id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;
  user.password = None;
  user.salt = None;
  Ok(user)
}
pub async fn get_tag_by_id(id: i64) -> AnyResult<Tag> {
  let pool = get_pool().await?;
  let tag: Tag = sqlx::query_as("SELECT * FROM tags where id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;

  Ok(tag)
}
pub async fn get_prop_by_id(id: i64) -> AnyResult<Prop> {
  let pool = get_pool().await?;
  let prop = sqlx::query_as("SELECT * FROM props where id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;
  Ok(prop)
}

pub async fn get_weight_by_id(id: i64) -> AnyResult<Weight> {
  let pool = get_pool().await?;
  let item = sqlx::query_as("SELECT * FROM weights where id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await?;
  Ok(item)
}
