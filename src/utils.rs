use chrono::Utc;

pub fn now_str() -> String {
  Utc::now().to_rfc3339()
}

pub fn now_fmt() -> String {
  let dt = Utc::now();
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
