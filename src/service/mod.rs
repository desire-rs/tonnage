use crate::libs::get_pool;
use crate::schema::{Prop, Tag, User, Weight};
use crate::types::AnyResult;

pub async fn get_user_tags(user_id: i64) -> AnyResult<Vec<Tag>> {
  let pool = get_pool().await?;
  let sql = format!("select id, user_id, name,created_at, updated_at from tags where user_id = ?");
  let rows = sqlx::query_as(&sql).bind(user_id).fetch_all(&pool).await?;
  Ok(rows)
}
pub async fn get_user_props(user_id: i64) -> AnyResult<Vec<Prop>> {
  let pool = get_pool().await?;
  let sql =
    format!("select id, user_id, name, value, created_at, updated_at from props where user_id = ?");
  let rows = sqlx::query_as(&sql).bind(user_id).fetch_all(&pool).await?;
  Ok(rows)
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
