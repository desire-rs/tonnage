use chrono::Utc;
pub fn now_fmt() -> String {
  let dt = Utc::now();
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
