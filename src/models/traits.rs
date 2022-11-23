pub trait FromSqlRow: Sized {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
}

pub trait ModelName {
  fn model_name() -> String;
}

pub trait RequireId: ModelName {
  fn option_id(&self) -> Option<u32>;

  #[allow(clippy::panic)]
  fn require_id(&self) -> u32 {
    RequireId::option_id(self)
      .unwrap_or_else(|| panic!("ID is not present in this {} instance", Self::model_name()))
  }
}
