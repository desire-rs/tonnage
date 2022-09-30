use crate::config::DATABASE_URI;
use crate::schema::UserProps;
use crate::types::{ApiPageResult, ApiResult, PageData, Resp};
use desire::Request;
use rusqlite::params;
use rusqlite::Connection;

pub async fn get_all(_req: Request) -> ApiPageResult<UserProps> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt =
    conn.prepare("SELECT id, userId, name, value,createdAt, updatedAt FROM userProps")?;
  let rows = stmt.query_map([], |row| {
    Ok(UserProps {
      id: row.get(0)?,
      user_id: row.get(1)?,
      name: row.get(2)?,
      value: row.get(3)?,
      created_at: row.get(4)?,
      updated_at: row.get(5)?,
    })
  })?;
  let mut list: Vec<UserProps> = Vec::new();
  for user_props in rows {
    list.push(user_props?);
  }
  let len = list.len() as u64;
  let result = PageData::new(list, len);
  Ok(Resp::data(result))
}

pub async fn create(req: Request) -> ApiResult<UserProps> {
  let user_props = req.body::<UserProps>().await?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let sql = "INSERT into userProps(userId, name, value, createdAt, updatedAt) VALUES (?,?,?,?,?)";
  let result = conn.execute(
    sql,
    params![
      &user_props.user_id,
      &user_props.name,
      &user_props.name,
      &user_props.created_at,
      &user_props.updated_at
    ],
  )?;
  info!("result: {:?}", result);
  let user_props = conn.query_row(
    "SELECT id,userId,name,createdAt,updatedAt from userProps where userId = ? AND name = ?",
    params![&user_props.user_id, &user_props.name],
    |row| {
      Ok(UserProps {
        id: row.get(0)?,
        user_id: row.get(1)?,
        name: row.get(2)?,
        value: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
      })
    },
  )?;
  Ok(Resp::data(user_props))
}

pub async fn update(req: Request) -> ApiResult<UserProps> {
  let id = req.get_param::<u32>("id")?;
  let user_props = req.body::<UserProps>().await?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let sql = "UPDATE userProps SET name = ?, value = ? updatedAt = ? WHERE id = ?";
  let result = conn.execute(
    sql,
    params![
      &user_props.name,
      &user_props.value,
      &user_props.updated_at,
      id
    ],
  )?;

  info!("result: {:?}", result);
  let user_props = conn.query_row(
    "SELECT id,userId,name,createdAt,updatedAt from userProps where id = ?",
    params![id],
    |row| {
      Ok(UserProps {
        id: row.get(0)?,
        user_id: row.get(1)?,
        name: row.get(2)?,
        value: row.get(3)?,
        created_at: row.get(4)?,
        updated_at: row.get(5)?,
      })
    },
  )?;
  Ok(Resp::data(user_props))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.get_param::<u32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute("DELETE FROM userProps where id = ?", [&id])?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
