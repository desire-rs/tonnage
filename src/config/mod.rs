use once_cell::sync::Lazy;
use std::env;
pub const JWT_SECRET: &'static str = "8B0271EC-EAA6-48B7-8161-F90B86F454E4";
pub const EXEMPT_ROUTES: &[&str] = &["/", "/signin", "/signup", "/hello", "/assets"];
pub static ADDR: Lazy<String> = Lazy::new(|| env::var("ADDR").expect("ADDR must be set"));
pub static ENV_NAME: Lazy<String> =
  Lazy::new(|| env::var("ENV_NAME").expect("ENV_NAME must be set"));
pub static DATABASE_URI: Lazy<String> =
  Lazy::new(|| env::var("DATABASE_URI").expect("DATABASE_URI must be set"));
pub static REDIS_URI: Lazy<String> =
  Lazy::new(|| env::var("REDIS_URI").expect("REDIS_URI must be set"));
// pub const DATABASE_URI.as_str(): &'static str = "db/tonnage.db";
pub const INIT_SQL: &'static str = r#"
BEGIN;
CREATE TABLE "users" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "username" TEXT NOT NULL,
  "nickname" TEXT,
  "salt" TEXT,
  "avatar" TEXT,
  "password" TEXT NOT NULL,
  "birthday" TEXT,
  "gender" TEXT,
  "email" TEXT,
  "mobile" TEXT,
  "created_at" DATE NOT NULL,
  "updated_at" DATE
);

CREATE UNIQUE INDEX "users_email"
ON "users" (
  "email"
);

CREATE UNIQUE INDEX "users_mobile"
ON "users" (
  "mobile"
);

CREATE INDEX "users_nickname"
ON "users" (
  "nickname"
);

CREATE UNIQUE INDEX "users_username"
ON "users" (
  "username"
);

CREATE TABLE "weights" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "weight" REAL,
  "created_at" DATE NOT NULL,
  "updated_at" DATE
);


CREATE INDEX "weights_user_id"
ON "weights" (
  "user_id"
);

CREATE TABLE "props" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "name" TEXT,
  "value" TEXT,
  "created_at" DATE NOT NULL,
  "updated_at" DATE
);

CREATE UNIQUE INDEX "prop_user_id_name"
ON "props" (
  "user_id",
  "name"
);

CREATE TABLE "tags" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "name" TEXT,
  "created_at" DATE NOT NULL,
  "updated_at" DATE
);

CREATE UNIQUE INDEX "tags_user_id_name"
ON "tags" (
  "user_id",
  "name"
);
COMMIT;
"#;

pub const RESET_SQL: &'static str = r#"
BEGIN;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS weights;
DROP TABLE IF EXISTS prop;
COMMIT;
"#;
