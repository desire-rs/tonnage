use rusqlite::{Connection, Result};
#[derive(Debug)]
struct User {
  id: i32,
  username: String,
  password: String,
  age: u8,
  meta: Option<String>,
  created_at: Option<String>,
  updated_at: Option<String>,
}
fn main() -> Result<()> {
  let conn = Connection::open("db/tonnage.db")?;

  // conn.execute(
  //   "CREATE TABLE users (
  //     id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  //     username TEXT,
  //     password TEXT,
  //     age INTEGER,
  //     meta TEXT,
  //     created_at TEXT,
  //     updated_at TEXT
  //   )",
  //   (), // empty list of parameters.
  // )?;
  let me = User {
    id: 0,
    username: "Steven".to_string(),
    password: "123456".to_string(),
    age: 30,
    meta: Some("meta".to_string()),
    created_at: None,
    updated_at: None,
  };
  conn.execute(
    "INSERT INTO users (username, password, age, meta) VALUES (?1, ?2, ?3,?4)",
    (&me.username, &me.password, &me.age, &me.meta),
  )?;

  let mut stmt = conn.prepare("SELECT id, username, password, age, meta FROM users")?;
  let user_iter = stmt.query_map([], |row| {
    Ok(User {
      id: row.get(0)?,
      username: row.get(1)?,
      password: row.get(2)?,
      age: row.get(3)?,
      meta: row.get(4)?,
      created_at: None,
      updated_at: None,
    })
  })?;

  for user in user_iter {
    println!("Found user {:?}", user.unwrap());
  }
  Ok(())
}
