pub const ADDR: &'static str = "0.0.0.0:12306";
pub const DATABASE_URI: &'static str = "db/tonnage.db";
pub const INIT_SQL: &'static str  = r#"
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

pub const CLEAR_SQL: &'static str  = r#"
BEGIN;
DROP TABLE users;
DROP TABLE weights;
COMMIT;
"#;