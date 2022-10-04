use crate::abort_with_message;
use crate::config;
use crate::models::traits::FromSqlRow;
use rusqlite::Connection;

// TODO: Should be singleton.
pub fn connection() -> rusqlite::Connection {
  let db_path = config::env::require_string_env_var("DB_PATH");

  Connection::open(db_path).unwrap_or_else(|e| abort_with_message(e))
}

pub fn install_database() -> Result<(), rusqlite::Error> {
  connection().execute(
    "
  CREATE TABLE IF NOT EXISTS note (
    id          INTEGER PRIMARY KEY,
    content     TEXT NOT NULL,
    task_status INTEGER,
    archived    BOOLEAN NOT NULL DEFAULT false, 
    pinned      BOOLEAN NOT NULL DEFAULT false
  );",
    (),
  )?;

  connection().execute(
    "
  CREATE TABLE IF NOT EXISTS template (
    id      INTEGER PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
  );",
    (),
  )?;

  Ok(())
}

pub fn row_to_template<T: FromSqlRow>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
  T::from_row(row)
}

pub fn rows_to_vec<T: FromSqlRow>(
  query: &str,
  params: &[&dyn rusqlite::ToSql],
) -> Result<Vec<T>, rusqlite::Error> {
  let conn = connection();
  let mut stmt = conn.prepare(query)?;
  let mapped = stmt.query_map(params, row_to_template::<T>)?;

  Ok(mapped.map(|n| n.unwrap()).collect())
}

// TODO: Note that this is silencing the error, if any, by converting it to None,
//       but the error could mean something else (e.g. connection error), not
//       necessarily that the row doesn't exist.
//
//       Workaround: Return a "Result", and then handle it in the component that calls it.
pub fn single_row<T: FromSqlRow + Clone>(
  query: &str,
  params: &[&dyn rusqlite::ToSql],
) -> Option<T> {
  rows_to_vec::<T>(query, params)
    .map(|rows| rows.first().map(|r| r.clone()))
    .ok()
    .flatten()
}

/// Query that inserts or changes rows.
pub fn change_rows(query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize, rusqlite::Error> {
  let conn = connection();
  let stmt = conn.prepare(query);
  let rows_changed = stmt?.execute(params)?;
  Ok(rows_changed)
}
