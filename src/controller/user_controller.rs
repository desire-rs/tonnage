use crate::libs::get_poll;
use crate::schema::{User, UserQuery};
use crate::service;
use crate::types::{ApiPageResult, ApiResult, PageData, Resp};
use crate::utils::{gen_salt, sha_256};
use desire::Request;
use tokio_stream::StreamExt;
pub async fn get_all(req: Request) -> ApiPageResult<User> {
  let poll = get_poll().await?;
  let query = req.get_query::<UserQuery>()?;
  let mut wheres = format!("1 = 1");
  let mut limit = 20;
  let mut page = 1;
  if let Some(query) = query {
    limit = query.limit;
    page = query.page;
    if let Some(id) = query.id {
      wheres = format!("{} AND id = {}", wheres, id);
    }
    if let Some(username) = query.username {
      wheres = format!("{} AND username = {}", wheres, username);
    }
    if let Some(nickname) = query.nickname {
      wheres = format!("{} AND nickname = {}", wheres, nickname);
    }
    if let Some(email) = query.email {
      wheres = format!("{} AND email = {}", wheres, email);
    }
    if let Some(mobile) = query.mobile {
      wheres = format!("{} AND mobile = {}", wheres, mobile);
    }
    if let Some(date_start) = query.date_start {
      wheres = format!("{} AND created_at >= '{}'", wheres, date_start);
    }
    if let Some(date_end) = query.date_end {
      wheres = format!("{} AND created_at < '{}'", wheres, date_end);
    }
  }
  let offset = (page - 1) * limit;
  let sql = format!(
    "SELECT * FROM users where {} LIMIT {} OFFSET {}",
    wheres, limit, offset
  );
  let count_sql = format!("SELECT COUNT(1) FROM users where {}", wheres);
  let mut rows = sqlx::query_as::<_, User>(&sql).fetch(&poll);
  let mut list = Vec::new();
  while let Some(row) = rows.try_next().await? {
    list.push(row);
  }
  let total: (i64,) = sqlx::query_as(&count_sql).fetch_one(&poll).await?;
  let result = PageData::new(list, total.0);
  Ok(Resp::data(result))
}

pub async fn get_by_id(req: Request) -> ApiResult<User> {
  let id = req.get_param::<i64>("id")?;
  let result = service::get_user_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn create(req: Request) -> ApiResult<User> {
  let mut user = req.body::<User>().await?;
  let salt = gen_salt();
  let password = sha_256(&user.password.unwrap(), &salt);
  user.password = Some(password);
  user.salt = Some(salt);
  info!("user: {:?}", user);
  let poll = get_poll().await?;
  let result = sqlx::query("INSERT INTO users (username,nickname,password,salt,birthday,gender,email,mobile,created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
  .bind(&user.username)
  .bind(&user.nickname)
  .bind(&user.password)
  .bind(&user.salt)
  .bind(&user.birthday)
  .bind(&user.gender)
  .bind(&user.email)
  .bind(&user.mobile)
  .bind(&user.created_at)
  .bind(&user.updated_at)
  .execute(&poll)
  .await?;
  let id = result.last_insert_rowid();
  let result = service::get_user_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn update(req: Request) -> ApiResult<User> {
  let poll = get_poll().await?;
  let id = req.get_param::<i64>("id")?;
  let user = req.body::<User>().await?;
  let result = sqlx::query("UPDATE users SET nickname = ?,email=?,mobile =?,avatar =?,gender=?, updated_at = ? WHERE id = ?")
    .bind(&user.nickname)
    .bind(&user.email)
    .bind(&user.mobile)
    .bind(&user.avatar)
    .bind(&user.gender)
    .bind(&user.updated_at)
    .bind(id)
    .execute(&poll)
    .await?;
  info!("result: {:?}", result);
  let result = service::get_user_by_id(id).await?;
  Ok(Resp::data(result))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let poll = get_poll().await?;
  let id = req.get_param::<i64>("id")?;
  let result = sqlx::query("DELETE FROM users where id = ?")
    .bind(id)
    .execute(&poll)
    .await?;
  info!("result: {:?}", result);
  Ok(Resp::data("OK".to_string()))
}
