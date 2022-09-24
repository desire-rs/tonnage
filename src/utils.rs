use std::ops::Add;

use chrono::Utc;

pub fn now_str() -> String {
  Utc::now().to_string()
}

pub fn now_fmt() -> String {
  let dt = Utc::now();
  let today = dt.format("%Y-%m-%d %H:%M:%S").to_string();
  today
}
