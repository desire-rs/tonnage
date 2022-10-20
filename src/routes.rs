use crate::controller::*;
use crate::middleware;
use desire::Router;
use desire::ServeDir;
use desire::ServeFile;
pub fn default_routes() -> desire::Router {
  let mut app = Router::new();
  app.with(middleware::Auth);
  app.with(middleware::Logger);
  app.with(middleware::DB);
  app.get("/", ServeFile::new("dist/index.html".into()));
  app.get("/assets/:file", ServeDir::new("dist".into()));
  app.get("images/:file", ServeDir::new("storage/images".into()));

  app.get("/hello", default_controller::hello);
  app.get("/hello_page", default_controller::hello_page);
  app.get("/hello_option", default_controller::hello_option);
  app.get("/token_data", default_controller::token_data);
  app.get("/liveness", default_controller::liveness);

  app.get("/db_init", db_controller::db_init);
  app.get("/db_reset", db_controller::db_reset);

  app.post("/signin", auth_controller::signin);
  app.post("/signup", auth_controller::signup);
  app.post("/vcode", auth_controller::vcode);

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
  app.post("/upload", upload_controller::upload);
  app
}
