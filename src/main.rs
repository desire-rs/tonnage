#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod middleware;
mod schema;
mod types;

use config::ADDR;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use desire::{IntoResponse, Request, Router};
#[tokio::main]
async fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  info!("APP running at {:?}", ADDR);
  let mut app = Router::new();
  app.get("/", controller::hello);
  let sever = desire::new(ADDR);
  sever.run(app).await.unwrap();
}
