use once_cell::sync::Lazy;
use std::env;
pub const JWT_SECRET: &'static str = "8B0271EC-EAA6-48B7-8161-F90B86F454E4";
pub const EXEMPT_ROUTES: &[&str] = &["/", "/signin", "/signup", "/hello", "/assets"];
pub static ENV_NAME: Lazy<String> =
  Lazy::new(|| env::var("ENV_NAME").expect("ENV_NAME must be set"));
pub static DATABASE_URI: Lazy<String> =
  Lazy::new(|| env::var("DATABASE_URI").expect("DATABASE_URI must be set"));
pub const ADDR: &'static str = "0.0.0.0:12306";
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
  "meta" TEXT,
  "subscription" INTEGER,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);

CREATE INDEX "users_email"
ON "users" (
  "email"
);

CREATE INDEX "users_mobile"
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
  "userId" INTEGER NOT NULL,
  "weight" REAL,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);


CREATE INDEX "weights_userId"
ON "weights" (
  "userId"
);

CREATE TABLE "userProps" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "userId" INTEGER NOT NULL,
  "name" TEXT,
  "value" TEXT,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);

CREATE INDEX "user_props_userId_name"
ON "userProps" (
  "userId",
  "name"
);

CREATE TABLE "tags" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "userId" INTEGER NOT NULL,
  "name" TEXT,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);

CREATE INDEX "user_tags_userId_name"
ON "tags" (
  "userId",
  "name"
);
COMMIT;
"#;

pub const CLEAR_SQL: &'static str = r#"
BEGIN;
DROP TABLE tags;
DROP TABLE users;
DROP TABLE weights;
DROP TABLE userProps;
COMMIT;
"#;
