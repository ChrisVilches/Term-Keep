pub trait FromSqlRow: Sized {
  // TODO: The only problem of this trait is that the column order should
  //       always be the same for all queries.
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>;
}
