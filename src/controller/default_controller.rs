use crate::schema::TokenData;
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
