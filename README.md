# TONNAGE

## users

```sql
CREATE TABLE "users_demo" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "username" TEXT NOT NULL,
  "nickname" TEXT,
  "password" TEXT NOT NULL,
  "birthday" INTEGER,
  "gender" TEXT,
  "email" TEXT,
  "mobile" TEXT,
  "meta" TEXT,
  "subscription" INTEGER,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);

CREATE INDEX "email"
ON "users_demo" (
  "email" ASC
);

CREATE INDEX "nickname"
ON "users_demo" (
  "nickname" ASC
);

CREATE INDEX "username"
ON "users_demo" (
  "username" ASC
);
```

## weights_demo

```sql
CREATE TABLE "weights_demo" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "userId" INTEGER,
  "weight" REAL,
  "createdAt" DATE NOT NULL,
  "updatedAt" DATE
);

CREATE INDEX "userId"
ON "weights_demo" (
  "userId"
);
```
