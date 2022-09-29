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
  "email" ASC
);

CREATE INDEX "users_nickname"
ON "users" (
  "nickname" ASC
);

CREATE INDEX "users_username"
ON "users" (
  "username" ASC
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

COMMIT;
"#;

pub const CLEAR_SQL: &'static str = r#"
BEGIN;
DROP TABLE users;
DROP TABLE weights;
COMMIT;
"#;
