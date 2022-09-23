#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod middleware;
mod schema;
mod types;

use config::ADDR;
use desire::{IntoResponse, Request, Router};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use types::AnyResult;
#[tokio::main]
async fn main() -> AnyResult<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  info!("APP running at {:?}", ADDR);
  let mut app = Router::new();
  app.get("/", controller::hello);
  app.get("/db_init", controller::db_init);
  app.get("/db_reset", controller::db_reset);
  let sever = desire::new(ADDR);
  sever.run(app).await?;
  Ok(())
}
