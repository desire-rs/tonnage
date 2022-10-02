use crate::error::option_error;
use crate::schema::{Liveness, TokenData};
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use crate::utils::now;
use desire::{IntoResponse, Request};
use redis::AsyncCommands;
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

pub async fn liveness(req: Request) -> ApiResult<Liveness> {
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  let mut result = Liveness::default();
  let key = "liveness:tonnage";
  let value = now();
  redis.set_nx(key, value).await?;
  redis.expire(key, 600).await?;
  let value = redis.get(key).await?;
  result.redis = Some(value);
  Ok(Resp::data(result))
}
