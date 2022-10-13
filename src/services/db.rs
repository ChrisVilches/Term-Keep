use crate::abort_with_message;
use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;
use crate::util::env;
use rusqlite::Connection;

const CREATE_NOTE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS note (
  id          INTEGER PRIMARY KEY,
  content     TEXT NOT NULL,
  task_status INTEGER,
  archived    BOOLEAN NOT NULL DEFAULT false,
  pinned      BOOLEAN NOT NULL DEFAULT false
);";

const CREATE_TEMPLATE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS template (
  id      INTEGER PRIMARY KEY,
  name    VARCHAR(255) NOT NULL,
  content TEXT NOT NULL
);";

// TODO: Should be singleton.
pub fn connection() -> rusqlite::Connection {
  let db_path = env::require_string_env_var("DB_PATH");

  Connection::open(db_path).unwrap_or_else(|e| abort_with_message(e))
}

// TODO: Add created_at, updated_at. Can I just use triggers (using SQLite though)?
// TODO: Add install SQL as .sql file in /data folder. Load using the static string loader.
//       But only do it if it's possible to execute multiple statements at once.
pub fn install_database() -> Result<(), rusqlite::Error> {
  connection().execute(CREATE_NOTE_TABLE_SQL, ())?;
  connection().execute(CREATE_TEMPLATE_TABLE_SQL, ())?;

  Ok(())
}

fn row_to_template<T: FromSqlRow>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
  T::from_row(row)
}

pub fn rows_to_vec<T: FromSqlRow>(query: &str, params: &[&dyn rusqlite::ToSql]) -> Vec<T> {
  let conn = connection();
  let mut stmt = conn.prepare(query).unwrap();
  let mapped = stmt.query_map(params, row_to_template::<T>).unwrap();
  mapped.map(std::result::Result::unwrap).collect()
}

pub fn single_row<T: FromSqlRow + Clone>(
  query: &str,
  params: &[&dyn rusqlite::ToSql],
) -> Option<T> {
  rows_to_vec::<T>(query, params).first().cloned()
}

pub fn change_row<T: ModelName>(
  query: &str,
  params: &[&dyn rusqlite::ToSql],
) -> Result<(), RowNotChangedError> {
  let conn = connection();
  let stmt = conn.prepare(query);
  let rows_changed = stmt.unwrap().execute(params).unwrap();

  match rows_changed {
    1 => Ok(()),
    _ => Err(RowNotChangedError::new::<T>()),
  }
}
