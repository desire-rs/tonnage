use crate::config::DATABASE_URI;
use crate::schema::Weight;
use crate::types::{ApiOptionResult, ApiPageResult, ApiResult, PageData, Resp};
use desire::Request;
use rusqlite::Connection;

pub async fn create(req: Request) -> ApiResult<String> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let weight = req.body::<Weight>().await?;
  info!("weight: {:?}", weight);
  let result = conn.execute(
    "INSERT INTO weights (userId,weight,createdAt, updatedAt) VALUES (?1, ?2, ?3, ?4)",
    (
      &weight.user_id,
      &weight.weight,
      &weight.created_at,
      &weight.updated_at,
    ),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn update(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let weight = req.body::<Weight>().await?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute(
    "update weight set weights=?1,updatedAt=?2 where id = ?3",
    (&weight.weight, &weight.updated_at, &id),
  )?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute("DELETE FROM weights where id = ?1", [&id])?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}

pub async fn get_all(_req: Request) -> ApiPageResult<Weight> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt = conn.prepare("SELECT id, userId,weight,createdAt, updatedAt FROM weights")?;
  let user_iter = stmt.query_map([], |row| {
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
pub async fn get_by_id(req: Request) -> ApiOptionResult<Weight> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt = conn.prepare("SELECT id, userId weight,createdAt, updatedAt FROM weights WHERE id = ?")?;

  let weight = stmt.query_row([&id], |row| {
    Ok(Weight {
      id: row.get(0)?,
      user_id: row.get(1)?,
      weight: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  })?;
  Ok(Resp::data(Some(weight)))
}
