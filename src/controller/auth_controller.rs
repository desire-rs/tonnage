use crate::config::JWT_SECRET;
use crate::error::option_error;
use crate::error::Error;
use crate::schema::{Claims, SignIn, TokenData, TokenInfo, User, VCode};
use crate::service;
use crate::types::{ApiResult, Pool, Resp};
use crate::utils::gen_code;
use crate::utils::{gen_salt, sha_256};
use desire::Request;
use jsonwebtoken::{encode, EncodingKey, Header};
use redis::AsyncCommands;
pub async fn signup(mut req: Request) -> ApiResult<TokenInfo> {
  let mut user = req.body::<User>().await?;
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let salt = gen_salt();
  let password = sha_256(&user.password.unwrap(), &salt);
  user.password = Some(password);
  user.salt = Some(salt);
  info!("user: {:?}", user);
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
  .execute(pool)
  .await?;
  let id = result.last_insert_rowid();
  let user = service::get_user_by_id(id).await?;

  let token = encode(
    &Header::default(),
    &Claims::new(user.id.unwrap()),
    &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
  )?;
  let info = TokenInfo::new(token);
  Ok(Resp::data(info))
}

pub async fn signin(mut req: Request) -> ApiResult<TokenInfo> {
  let login_user = req.body::<SignIn>().await?;
  let pool = req.extensions().get::<Pool>().ok_or(option_error("pool"))?;
  let user: User = sqlx::query_as("SELECT * from users WHERE username = ? LIMIT 1")
    .bind(&login_user.username)
    .fetch_one(pool)
    .await?;

  let password = sha_256(&login_user.password, &user.salt.unwrap());
  if password != user.password.unwrap() {
    return Err(Error::AnyhowError(anyhow::anyhow!(
      "password is not correct"
    )));
  }
  let token = encode(
    &Header::default(),
    &Claims::new(user.id.unwrap()),
    &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
  )?;
  let info = TokenInfo::new(token);
  Ok(Resp::data(info))
}

pub async fn vcode(req: Request) -> ApiResult<VCode> {
  let token_data = req
    .extensions()
    .get::<TokenData>()
    .ok_or(option_error("TokenData"))?;
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  let user_id = token_data.uid;
  let key = format!("tonnage:vcode:{}", user_id);
  let exists: bool = redis.exists(&key).await?;
  if !exists {
    let code = gen_code(6);
    redis.set_nx(&key, &code).await?;
    redis.expire(&key, 60).await?;
  }
  let code: String = redis.get(&key).await?;
  let result = VCode::new(user_id, &code);
  Ok(Resp::data(result))
}
