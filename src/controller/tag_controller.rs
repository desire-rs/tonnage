use crate::config::DATABASE_URI;
use crate::schema::Tag;
use crate::types::{ApiResult, Resp};
use desire::Request;
use rusqlite::params;
use rusqlite::Connection;
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
