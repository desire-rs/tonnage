use crate::config::UPLOAD_PATH;
use crate::error::message_error;
use crate::schema::UploadFile;
use crate::types::Resp;
use crate::utils::gen_salt;
use crate::ApiResult;
use desire::http::header::CONTENT_TYPE;
use desire::Request;
use multer::Multipart;
use tokio::fs;
use tokio::io::AsyncWriteExt;
pub async fn upload(req: Request) -> ApiResult<Vec<UploadFile>> {
  let boundary = req
    .inner
    .headers()
    .get(CONTENT_TYPE)
    .and_then(|ct| ct.to_str().ok())
    .and_then(|ct| multer::parse_boundary(ct).ok());
  if boundary.is_none() {
    return Err(message_error("file not found"));
  }

  let mut multipart = Multipart::new(req.inner.into_body(), boundary.unwrap());
  let mut list: Vec<UploadFile> = Vec::new();
  while let Some(mut field) = multipart.next_field().await? {
    let random = gen_salt();
    let name = field.name().unwrap_or(&random);
    let file_name = field.file_name().unwrap_or(&random);
    let content_type = field.content_type();
    info!(
      "Name: {:?}, FileName: {:?}, Content-Type: {:?}",
      name, file_name, content_type
    );
    let hash = gen_salt();
    let file_name = format!(
      "{}.{}{}",
      name,
      hash.to_lowercase(),
      file_name.replace(name, "")
    );
    let file_path = format!("{}/{}", UPLOAD_PATH.as_str(), file_name);
    let mut dest = fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(&file_path)
      .await?;
    let mut field_bytes_len = 0;
    while let Some(field_chunk) = field.chunk().await? {
      field_bytes_len += field_chunk.len();
      dest.write_all(&field_chunk).await?;
    }

    list.push(UploadFile {
      name: None,
      file_name: Some(file_name.to_string()),
      file_path: Some(file_path),
      file_size: Some(field_bytes_len.to_string()),
      front_path: Some(format!("images/{}", file_name.to_string())),
    });
    info!("Field Bytes Length: {:?}", field_bytes_len);
  }
  Ok(Resp::data(list))
}
