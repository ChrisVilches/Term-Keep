use chrono::{DateTime, Utc};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn format_date(date: DateTime<Utc>) -> String {
  date.format(DATE_FORMAT).to_string()
}
