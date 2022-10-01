use crate::error::Error;
use desire::IntoResponse;
use desire::Response;
pub type ApiResult<T> = Result<Resp<T>, Error>;
pub type AnyResult<T> = Result<T, anyhow::Error>;
pub type ApiPageResult<T> = Result<Resp<PageData<T>>, Error>;
pub type ApiOptionResult<T> = Result<Resp<Option<T>>, Error>;

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Resp<T = String> {
  success: bool,
  msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  data: Option<T>,
}
impl<T> Resp<T>
where
  T: Serialize + Send,
{
  pub fn data(data: T) -> Self {
    Resp {
      success: true,
      msg: "OK".to_string(),
      data: Some(data),
    }
  }
}

impl<T> IntoResponse for Resp<T>
where
  T: Serialize + Send + Sync + 'static,
{
  fn into_response(self) -> desire::Result {
    Response::json::<Resp<T>>(self)
  }
}

// 分页数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PageData<T>
where
  T: Serialize + Send,
{
  pub list: Vec<T>,
  pub total: i64,
}

impl<T> PageData<T>
where
  T: Serialize + Send,
{
  #[allow(dead_code)]
  pub fn new(list: Vec<T>, total: i64) -> Self {
    PageData { list, total }
  }
}
