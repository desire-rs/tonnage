use crate::utils::now_fmt;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub username: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthday: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avatar: Option<String>,
  pub email: String,
  pub mobile: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub salt: Option<String>,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Weight {
  pub id: Option<i64>,
  pub user_id: i64,
  pub weight: f32,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub user_id: Option<i64>,
  pub date_start: Option<String>,
  pub date_end: Option<String>,
  pub limit: i64,
  pub page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub username: Option<String>,
  pub nickname: Option<String>,
  pub email: Option<String>,
  pub mobile: Option<String>,
  pub date_start: Option<String>,
  pub date_end: Option<String>,
  pub limit: i64,
  pub page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub user_id: Option<i64>,
  pub name: Option<String>,
  pub date_start: Option<String>,
  pub date_end: Option<String>,
  pub limit: i64,
  pub page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub user_id: Option<i64>,
  pub name: Option<String>,
  pub date_start: Option<String>,
  pub date_end: Option<String>,
  pub limit: i64,
  pub page: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChartQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<i64>,
  pub date_start: Option<String>,
  pub date_end: Option<String>,
  pub limit: i64,
  pub page: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Chart {
  pub user_id: i64,
  pub weight: f32,
  pub date: String,
}

impl ChartQuery {
  #[allow(dead_code)]
  pub fn new(date_start: String, date_end: String) -> Self {
    ChartQuery {
      user_id: None,
      date_start: Some(date_start),
      date_end: Some(date_end),
      limit: 20,
      page: 1,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: i64,
  pub company: String,
  pub exp: usize,
}

impl Claims {
  pub fn new(sub: i64) -> Self {
    Self {
      sub,
      company: String::from("Tonnage"),
      exp: 86400 * 30 * 1000,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TokenData {
  pub uid: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
  token: String,
  exp: usize,
}

impl TokenInfo {
  pub fn new(token: String) -> Self {
    Self {
      token,
      exp: 86400 * 30 * 1000,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignIn {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub user_id: i64,
  pub name: String,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Prop {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<i64>,
  pub name: String,
  pub value: String,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
  pub user: User,
  pub tags: Vec<Tag>,
  pub props: Vec<Prop>,
}
impl UserInfo {
  pub fn new(user: User, tags: Vec<Tag>, props: Vec<Prop>) -> Self {
    UserInfo { user, tags, props }
  }
}
