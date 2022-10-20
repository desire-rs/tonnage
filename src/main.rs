#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod libs;
mod middleware;
mod routes;
mod schema;
mod service;
mod types;
mod utils;

use config::{ADDR, ENV_NAME};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use types::AnyResult;
use types::ApiResult;

use std::env;

#[tokio::main]
async fn main() -> AnyResult<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  let arguments: Vec<String> = env::args().collect();
  let env_name = arguments.get(1).expect("env name must be provided");
  let env_file = format!("env/{}.env", env_name);
  dotenv::from_filename(env_file).ok();
  info!(
    "APP running at {:?}, ENV:{}",
    ADDR.to_string(),
    ENV_NAME.to_string()
  );
  let app = routes::default_routes();
  let sever = desire::new(ADDR.as_str());
  sever.run(app).await?;
  Ok(())
}
