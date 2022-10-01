#[macro_use]
extern crate tracing;

mod config;
mod controller;
mod error;
mod libs;
mod middleware;
mod schema;
mod service;
mod types;
mod utils;

use config::{ADDR, ENV_NAME};
use desire::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use types::AnyResult;

// use controller::default_controller;
use controller::*;
// use controller::user_controller;
// use controller::user_Prop_controller;
// use controller::weight_controller;
use desire::ServeDir;
use desire::ServeFile;
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
  info!("APP running at {:?}, ENV:{}", ADDR, ENV_NAME.to_string());

  let mut app = Router::new();
  app.with(middleware::Auth);
  app.with(middleware::Logger);
  app.get("/", ServeFile::new("dist/index.html".into()));
  app.get("/assets/:file", ServeDir::new("dist".into()));

  app.get("/hello", default_controller::hello);
  app.get("/hello_page", default_controller::hello_page);
  app.get("/hello_option", default_controller::hello_option);
  app.get("/token_data", default_controller::token_data);

  app.get("/db_init", db_controller::db_init);
  app.get("/db_reset", db_controller::db_reset);

  app.post("/signin", auth_controller::signin);
  app.post("/signup", auth_controller::signup);

  app.get("/user", user_controller::get_all);
  app.post("/user", user_controller::create);
  app.get("/user/info", user_controller::user_info);
  app.put("/user/:id", user_controller::update);
  app.get("/user/:id", user_controller::get_by_id);
  app.delete("/user/:id", user_controller::remove);

  app.get("/weight", weight_controller::get_all);
  app.post("/weight", weight_controller::create);
  app.put("/weight/:id", weight_controller::update);
  app.get("/weight/:id", weight_controller::get_by_id);
  app.delete("/weight/:id", weight_controller::remove);

  app.get("/tag", tag_controller::get_all);
  app.get("/tag/:id", tag_controller::get_by_id);
  app.post("/tag", tag_controller::create);
  app.delete("/tag/:id", tag_controller::remove);
  app.put("/tag/:id", tag_controller::update);

  app.get("/prop", prop_controller::get_all);
  app.get("/prop/:id", prop_controller::get_by_id);
  app.post("/prop", prop_controller::create);
  app.delete("/prop/:id", prop_controller::remove);
  app.put("/prop/:id", prop_controller::update);

  // chart
  app.get("/chart", chart_controller::chart);

  let sever = desire::new(ADDR);
  sever.run(app).await?;
  Ok(())
}
