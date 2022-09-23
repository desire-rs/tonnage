use desire::Middleware;
use desire::Request;
use desire::Result;
use std::time::Instant;
pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
  async fn handle(&self, req: Request, next: desire::Next<'_>) -> Result {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = next.run(req).await?;
    println!(
      "{} {} {} {}ms",
      method,
      path,
      res.status().to_string(),
      start.elapsed().as_millis()
    );
    Ok(res)
  }
}
