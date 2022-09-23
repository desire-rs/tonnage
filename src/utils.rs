use chrono::Utc;

pub fn now_str() -> String {
  Utc::now().to_string()
}
