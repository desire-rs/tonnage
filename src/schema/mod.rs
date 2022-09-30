use crate::utils::now_fmt;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  pub username: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthday: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  pub email: String,
  pub mobile: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub salt: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription: Option<i8>,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weight {
  pub id: Option<u32>,
  pub user_id: u32,
  pub weight: f32,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<u32>,
  pub date_start: String,
  pub date_end: String,
  pub limit: u32,
  pub page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
  pub user_id: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mobile: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border_color: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub background_color: Option<String>,
  pub weight: f32,
  pub date: String,
}

impl ChartQuery {
  pub fn new(date_start: String, date_end: String) -> Self {
    ChartQuery {
      user_id: None,
      date_start,
      date_end,
      limit: 20,
      page: 1,
    }
  }
  pub fn offset(&self) -> u32 {
    (self.page - 1) * self.limit
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: u32,
  pub company: String,
  pub exp: usize,
}

impl Claims {
  pub fn new(sub: u32) -> Self {
    Self {
      sub,
      company: String::from("Tonnage"),
      exp: 86400 * 30 * 1000,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TokenData {
  pub uid: u32,
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
  pub id: Option<u32>,
  pub username: String,
  pub password: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub salt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  pub user_id: u32,
  pub name: String,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<u32>,
  pub name: String,
  pub value: serde_json::Value,
  #[serde(default = "now_fmt")]
  pub created_at: String,
  #[serde(default = "now_fmt")]
  pub updated_at: String,
}
