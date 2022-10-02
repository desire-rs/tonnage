use desire::Middleware;
use desire::Request;
use desire::Result;
use std::sync::Arc;
use std::sync::Mutex;
use crate::types::State;
pub struct States<T>
where
  T: Send + Sync + 'static,
{
  pub inner: State<T>,
}

impl<T> States<T>
where
  T: Send + Sync + 'static,
{
  pub fn new(value: T) -> Self {
    Self {
      inner: Arc::new(Mutex::new(value)),
    }
  }
}
#[async_trait::async_trait]
impl<T> Middleware for States<T>
where
  T: Send + Sync + 'static,
{
  async fn handle(&self, mut req: Request, next: desire::Next<'_>) -> Result {
    req.inner.extensions_mut().insert(self.inner.clone());
    next.run(req).await
  }
}
