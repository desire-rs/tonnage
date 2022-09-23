use crate::config::DATABASE_URI;
use crate::schema::User;
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use desire::Request;
use rusqlite::Connection;

pub async fn create(req: Request) -> ApiResult<String> {
  let conn = Connection::open(DATABASE_URI)?;
  let user = req.body::<User>().await?;
  info!("user: {:?}", user);
  let result = conn.execute(
    "INSERT INTO users (username,nickname,password,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    (&user.username, &user.nickname, &user.password, &user.birthday, &user.gender, &user.email, &user.mobile, &user.meta, &user.subscription, &user.created_at, &user.updated_at),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn update(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let user = req.body::<User>().await?;
  let conn = Connection::open(DATABASE_URI)?;
  let result = conn.execute(
    "update users set nickname=?1,password=?2,birthday=?3,gender=?4,email=?5,mobile=?6,meta=?7,subscription=?8,updatedAt=?9 where id = ?10",
    (&user.nickname, &user.password, &user.birthday, &user.gender, &user.email, &user.mobile, &user.meta, &user.subscription, &user.updated_at, &id),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI)?;
  let result = conn.execute("DELETE FROM users where id = ?1", [&id])?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}

pub async fn get_all(_req: Request) -> ApiPageResult<User> {
  let conn = Connection::open(DATABASE_URI)?;
  let mut stmt = conn.prepare("SELECT id, username,nickname,password,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt FROM users")?;
  let user_iter = stmt.query_map([], |row| {
    Ok(User {
      id: row.get(0)?,
      username: row.get(1)?,
      nickname: row.get(2)?,
      password: row.get(3)?,
      birthday: row.get(4)?,
      gender: row.get(5)?,
      email: row.get(6)?,
      mobile: row.get(7)?,
      meta: row.get(8)?,
      subscription: row.get(9)?,
      created_at: row.get(10)?,
      updated_at: row.get(11)?,
    })
  })?;
  let mut list: Vec<User> = Vec::new();
  for user in user_iter {
    list.push(user?);
  }
  let len = list.len() as u64;
  let result = PageData::new(list, len);
  Ok(Resp::data(result))
}
pub async fn get_by_id(req: Request) -> ApiOptionResult<User> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI)?;
  let mut stmt = conn.prepare("SELECT id, username,nickname,password,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt FROM users WHERE id = ?")?;

  let user = stmt.query_row([&id], |row| {
    Ok(User {
      id: row.get(0)?,
      username: row.get(1)?,
      nickname: row.get(2)?,
      password: row.get(3)?,
      birthday: row.get(4)?,
      gender: row.get(5)?,
      email: row.get(6)?,
      mobile: row.get(7)?,
      meta: row.get(8)?,
      subscription: row.get(9)?,
      created_at: row.get(10)?,
      updated_at: row.get(11)?,
    })
  })?;
  Ok(Resp::data(Some(user)))
}
