use crate::config::{INIT_SQL, RESET_SQL};
use crate::error::option_error;
use crate::types::{ApiResult, Pool, Resp};
use desire::Request;
pub async fn db_init(req: Request) -> ApiResult<String> {
  info!("run init sql: {}", INIT_SQL);
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let result = sqlx::query(INIT_SQL).execute(pool).await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}

pub async fn db_reset(req: Request) -> ApiResult<String> {
  info!("run clear sql: {}", RESET_SQL);
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let result = sqlx::query(RESET_SQL).execute(pool).await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}
