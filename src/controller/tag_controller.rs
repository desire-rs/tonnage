use crate::config::DATABASE_URI;
use crate::schema::Tag;
use crate::types::{ApiPageResult, ApiResult, PageData, Resp};
use desire::Request;
use rusqlite::params;
use rusqlite::Connection;

pub async fn get_all(_req: Request) -> ApiPageResult<Tag> {
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt = conn.prepare("SELECT id, userId, name,createdAt, updatedAt FROM tags")?;
  let rows = stmt.query_map([], |row| {
    Ok(Tag {
      id: row.get(0)?,
      user_id: row.get(1)?,
      name: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  })?;
  let mut list: Vec<Tag> = Vec::new();
  for tag in rows {
    list.push(tag?);
  }
  let len = list.len() as u64;
  let result = PageData::new(list, len);
  Ok(Resp::data(result))
}

pub async fn get_by_id(req: Request) -> ApiResult<Tag> {
  let id = req.get_param::<i32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let mut stmt =
    conn.prepare("SELECT id, userId, name,createdAt, updatedAt FROM tags WHERE id = ?")?;

  let tag = stmt.query_row([&id], |row| {
    Ok(Tag {
      id: row.get(0)?,
      user_id: row.get(1)?,
      name: row.get(2)?,
      created_at: row.get(3)?,
      updated_at: row.get(4)?,
    })
  })?;
  Ok(Resp::data(tag))
}

pub async fn create(req: Request) -> ApiResult<Tag> {
  let tag = req.body::<Tag>().await?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let sql = "INSERT into tags(userId, name, createdAt, updatedAt) VALUES (?,?,?,?)";
  let result = conn.execute(
    sql,
    params![&tag.user_id, &tag.name, &tag.created_at, &tag.updated_at],
  )?;
  info!("result: {:?}", result);
  let tag = conn.query_row(
    "SELECT id,userId,name,createdAt,updatedAt from tags where userId = ? AND name = ?",
    params![&tag.user_id, &tag.name],
    |row| {
      Ok(Tag {
        id: row.get(0)?,
        user_id: row.get(1)?,
        name: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
      })
    },
  )?;
  Ok(Resp::data(tag))
}

pub async fn update(req: Request) -> ApiResult<Tag> {
  let id = req.get_param::<u32>("id")?;
  let tag = req.body::<Tag>().await?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let sql = "UPDATE tags SET name = ?, updatedAt = ? WHERE id = ?";
  let result = conn.execute(sql, params![&tag.name, &tag.updated_at, id])?;

  info!("result: {:?}", result);
  let tag = conn.query_row(
    "SELECT id,userId,name,createdAt,updatedAt from tags where id = ?",
    params![id],
    |row| {
      Ok(Tag {
        id: row.get(0)?,
        user_id: row.get(1)?,
        name: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
      })
    },
  )?;
  Ok(Resp::data(tag))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.get_param::<u32>("id")?;
  let conn = Connection::open(DATABASE_URI.as_str())?;
  let result = conn.execute("DELETE FROM tags where id = ?", [&id])?;
  info!("result: {}", result);
  Ok(Resp::data("OK".to_string()))
}
