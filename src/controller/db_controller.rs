use crate::config::{INIT_SQL, RESET_SQL};
use crate::libs::get_pool;
use crate::types::{ApiResult, Resp};
use desire::Request;

pub async fn db_init(_req: Request) -> ApiResult<String> {
  info!("run init sql: {}", INIT_SQL);
  let pool = get_pool().await?;
  let result = sqlx::query(INIT_SQL).execute(&pool).await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}

pub async fn db_reset(_req: Request) -> ApiResult<String> {
  info!("run clear sql: {}", RESET_SQL);
  let pool = get_pool().await?;
  let result = sqlx::query(RESET_SQL).execute(&pool).await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}
