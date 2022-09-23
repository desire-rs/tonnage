use desire::{IntoResponse, Request};
pub async fn hello(_req: Request) -> impl IntoResponse {
  "Hello World!"
}
