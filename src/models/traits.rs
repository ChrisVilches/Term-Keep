pub trait FromSqlRow: Sized {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
}

pub trait ModelName {
  fn model_name() -> String;
}
