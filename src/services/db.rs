use crate::abort_with_message;
use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;
use crate::util::env;
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Mutex;

const INSTALL_DATABASE_SQL: &str = include_str!("../../data/install.sql");

lazy_static! {
  static ref DB_PATH: String = env::require_string_env_var("DB_PATH");
  static ref CONNECTION: Mutex<rusqlite::Connection> =
    Mutex::new(Connection::open(DB_PATH.clone()).unwrap_or_else(|e| abort_with_message(e)));
}

pub fn install_database() -> Result<(), rusqlite::Error> {
  let conn = CONNECTION.lock().unwrap();
  conn.execute_batch(INSTALL_DATABASE_SQL)
}

fn row_to_template<T: FromSqlRow>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
  T::from_row(row)
}

pub fn rows_to_vec<T: FromSqlRow>(query: &str, params: &[&dyn rusqlite::ToSql]) -> Vec<T> {
  let conn = CONNECTION.lock().unwrap();
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
  let conn = CONNECTION.lock().unwrap();
  let stmt = conn.prepare(query);
  let rows_changed = stmt.unwrap().execute(params).unwrap();

  match rows_changed {
    1 => Ok(()),
    _ => Err(RowNotChangedError::new::<T>()),
  }
}

pub fn change_rows<T: ModelName>(query: &str, params: &[&dyn rusqlite::ToSql]) -> usize {
  let conn = CONNECTION.lock().unwrap();
  let mut stmt = conn.prepare(query).unwrap();
  stmt.execute(params).unwrap()
}
