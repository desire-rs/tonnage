use crate::config::{DATABASE_URI, REDIS_URI};
use crate::types::AnyResult;
use sqlx::pool::Pool;
use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqlitePool;
pub async fn get_pool() -> AnyResult<Pool<Sqlite>> {
  let pool = SqlitePool::connect(DATABASE_URI.as_str()).await?;
  Ok(pool)
}

pub async fn get_redis_client() -> AnyResult<redis::Client> {
  let client = redis::Client::open(REDIS_URI.as_str())?;
  Ok(client)
}
