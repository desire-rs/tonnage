use crate::utils::now_str;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i32>,
  pub username: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(default = "default_password")]
  pub password: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub birthday: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gender: Option<String>,
  pub email: String,
  pub mobile: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription: Option<i8>,
  #[serde(default = "now_str")]
  pub created_at: String,
  #[serde(default = "now_str")]
  pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weight {
  pub id: Option<i32>,
  pub user_id: i32,
  pub weight: f32,
  #[serde(default = "now_str")]
  pub created_at: String,
  #[serde(default = "now_str")]
  pub updated_at: String,
}

fn default_password() -> String {
  "123456".to_owned()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartQuery {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<i32>,
  pub date_start: String,
  pub date_end: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
  pub user_id: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mobile: Option<String>,
  pub weight: f32,
  pub date: String,
}

impl ChartQuery {
  pub fn new(date_start: String, date_end: String) -> Self {
    ChartQuery {
      user_id: None,
      date_start,
      date_end,
    }
  }
}
