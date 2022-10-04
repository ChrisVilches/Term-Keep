use crate::abort_with_message;
use crate::config;
use crate::models::traits::FromSqlRow;
use rusqlite::Connection;

// TODO: Should be singleton.
pub fn connection() -> rusqlite::Connection {
  let db_path = config::env::require_string_env_var("DB_PATH");

  /*
  // TODO: Prefer to use a vanilla "panic!" here, because this module shouldn't be using
  //       methods from the CLI module.
  //       However, even the config module uses this, and that module may be used in several places,
  //       so it's possible that this method is used from several places (not necessarily CLI), so
  //       maybe there should be a "abort_with_message" for CLI (human friendly) and another one
  //       for system related messages. But actually messages from the config module can come
  //       from both CLI and services/models, etc, so perhaps the best way is to not care too much
  //       and just make it global? (like for any kind of error). This should have a negative impact
  //       on the scalability, project structure, etc.
   */

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

pub fn single_row<T: FromSqlRow + Clone>(
  query: &str,
  params: &[&dyn rusqlite::ToSql],
) -> Option<T> {
  // TODO: I'm not so sure about this...
  rows_to_vec::<T>(query, params)
    .map(|rows| rows.first().map(|r| r.clone()))
    .ok()?
}

/// Query that inserts or changes rows.
pub fn change_rows(query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize, rusqlite::Error> {
  let conn = connection();
  let stmt = conn.prepare(query);
  let rows_changed = stmt?.execute(params)?;
  Ok(rows_changed)
}
