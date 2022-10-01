use crate::config::DATABASE_URI;
use crate::schema::{User, UserQuery, Weight};
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use crate::utils::{gen_salt, sha_256};
use desire::Request;
use rusqlite::Connection;

pub async fn create(req: Request) -> ApiResult<String> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut user = req.body::<User>().await?;
  let salt = gen_salt();
  let password = sha_256(&user.password.unwrap(), &salt);
  user.password = Some(password);
  user.salt = Some(salt);
  info!("user: {:?}", user);
  let result = conn.execute(
    "INSERT INTO users (username,nickname,password,salt,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    (&user.username, &user.nickname, &user.password, &user.salt, &user.birthday, &user.gender, &user.email, &user.mobile, &user.meta, &user.subscription, &user.created_at, &user.updated_at),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn update(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let mut user = req.body::<User>().await?;
  let password = sha_256(&user.password.unwrap(), &user.salt.unwrap());
  user.password = Some(password);
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute(
    "update users set nickname=?1,password=?2,birthday=?3,gender=?4,email=?5,mobile=?6,meta=?7,subscription=?8,updatedAt=?9 where id = ?10",
    (&user.nickname, &user.password, &user.birthday, &user.gender, &user.email, &user.mobile, &user.meta, &user.subscription, &user.updated_at, &id),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute("DELETE FROM users where id = ?1", [&id])?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}

pub async fn get_all(req: Request) -> ApiPageResult<User> {
  let query = req.get_query::<UserQuery>()?;
  let mut wheres = " 1 = 1".to_string();
  let mut limit = 20;
  let mut page = 1;
  if let Some(query) = query {
    limit = query.limit;
    page = query.page;
    if let Some(uid) = query.uid {
      wheres = format!("{} AND id = {}", wheres, uid);
    }
    if let Some(username) = query.username {
      wheres = format!("{} AND username = {}", wheres, username);
    }
    if let Some(email) = query.email {
      wheres = format!("{} AND email = {}", wheres, email);
    }
    if let Some(mobile) = query.mobile {
      wheres = format!("{} AND mobile = {}", wheres, mobile);
    }
    if let Some(date_start) = query.date_start {
      wheres = format!("{} AND createdAt >= '{}'", wheres, date_start);
    }
    if let Some(date_end) = query.date_end {
      wheres = format!("{} AND createdAt < '{}'", wheres, date_end);
    }
  }
  let offset = (page - 1) * limit;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let sql = format!("SELECT id, username,nickname,salt,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt FROM users WHERE {} LIMIT {} OFFSET {}", wheres, limit, offset);
  let count_sql = format!("SELECT COUNT(1) FROM users WHERE {}", wheres);
  info!("sql: {}", sql);
  info!("count_sql: {}", count_sql);
  let mut stmt = conn.prepare(&sql)?;
  let rows = stmt.query_map([], |row| {
    Ok(User {
      id: row.get(0)?,
      username: row.get(1)?,
      nickname: row.get(2)?,
      salt: row.get(3)?,
      password: None,
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

  let total: u64 = conn.query_row(&count_sql, [], |row| row.get(0))?;
  let mut list: Vec<User> = Vec::new();
  for user in rows {
    list.push(user?);
  }
  let result = PageData::new(list, total);
  Ok(Resp::data(result))
}
pub async fn get_by_id(req: Request) -> ApiOptionResult<User> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt = conn.prepare("SELECT id, username,nickname,salt,birthday,gender,email,mobile,meta,subscription,createdAt, updatedAt FROM users WHERE id = ?")?;

  let user = stmt.query_row([&id], |row| {
    Ok(User {
      id: row.get(0)?,
      username: row.get(1)?,
      nickname: row.get(2)?,
      password: None,
      salt: row.get(3)?,
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

pub async fn get_user_weights(req: Request) -> ApiPageResult<Weight> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let user_id = req.get_param::<i32>("id")?;
  let mut stmt = conn
    .prepare("SELECT id, userId, weight, createdAt, updatedAt FROM weights where userId = ?")?;
  let user_iter = stmt.query_map([&user_id], |row| {
    Ok(Weight {
      id: row.get(0)?,
      user_id: row.get(1)?,
      weight: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  })?;
  let mut list: Vec<Weight> = Vec::new();
  for weight in user_iter {
    list.push(weight?);
  }
  let len = list.len() as u64;
  let result = PageData::new(list, len);
  Ok(Resp::data(result))
}
