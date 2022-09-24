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

use config::ADDR;
use desire::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use types::AnyResult;

use controller::default_controller;
use controller::user_controller;
use controller::weight_controller;
use desire::ServeDir;
use desire::ServeFile;
#[tokio::main]
async fn main() -> AnyResult<()> {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  info!("APP running at {:?}", ADDR);

  let mut app = Router::new();
  app.with(middleware::Logger);
  app.get("/", ServeFile::new("dist/index.html".into()));
  app.get("/assets/:file", ServeDir::new("dist".into()));

  app.get("/hello", default_controller::hello);
  app.get("/hello_page", default_controller::hello_page);
  app.get("/hello_option", default_controller::hello_option);
  app.get("/db_init", default_controller::db_init);
  app.get("/db_reset", default_controller::db_reset);

  app.post("/sign_in", default_controller::sign_in);
  app.post("/sign_up", default_controller::sign_up);
  app.post("/sign_out", default_controller::sign_out);

  app.get("/user", user_controller::get_all);
  app.post("/user", user_controller::create);
  app.put("/user/:id", user_controller::update);
  app.get("/user/:id", user_controller::get_by_id);
  app.get("/user/:id/weight", user_controller::get_user_weights);
  app.delete("/user/:id", user_controller::remove);

  app.get("/weight", weight_controller::get_all);
  app.post("/weight", weight_controller::create);
  app.put("/weight/:id", weight_controller::update);
  app.get("/weight/:id", weight_controller::get_by_id);
  app.delete("/weight/:id", weight_controller::remove);

  // chart
  app.get("/chart", default_controller::chart);

  let sever = desire::new(ADDR);
  sever.run(app).await?;
  Ok(())
}
