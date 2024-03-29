use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;
use parking_lot::Mutex;
use rusqlite::Connection;

const INSTALL_DATABASE_SQL: &str = include_str!("../../data/install.sql");
const STATEMENT_PREPARE_ERROR: &str = "Query statement couldn't be prepared";
const STATEMENT_EXECUTE_ERROR: &str = "Prepared statement execution error";
const CONNECTION_NOT_INITIALIZED: &str = "Connection object should be initialized";

static CONNECTION: Mutex<Option<rusqlite::Connection>> = Mutex::new(None);

fn with_connection<T>(callback: impl Fn(&rusqlite::Connection) -> T) -> T {
  let guard = CONNECTION.lock();
  let conn = guard.as_ref().expect(CONNECTION_NOT_INITIALIZED);
  let result = callback(conn);
  drop(guard);
  result
}

fn create_db(db_file_path: &str) -> Result<(), rusqlite::Error> {
  *CONNECTION.lock() = Some(Connection::open(db_file_path)?);
  Ok(())
}

pub fn set_database(db_file_path: &str) -> Result<(), rusqlite::Error> {
  create_db(db_file_path)?;
  with_connection(|conn| conn.execute_batch(INSTALL_DATABASE_SQL))
}

fn row_to_template<T: FromSqlRow>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
  T::from_row(row)
}

pub fn rows_to_vec<T: FromSqlRow>(query: &str, params: &[&dyn rusqlite::ToSql]) -> Vec<T> {
  with_connection(|conn| {
    let mut stmt = conn.prepare(query).expect(STATEMENT_PREPARE_ERROR);
    let mapped = stmt
      .query_map(params, row_to_template::<T>)
      .expect(STATEMENT_EXECUTE_ERROR);
    mapped.map(Result::unwrap).collect()
  })
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
  with_connection(|conn| {
    let mut stmt = conn.prepare(query).expect(STATEMENT_PREPARE_ERROR);
    let rows_changed = stmt.execute(params).expect(STATEMENT_EXECUTE_ERROR);

    match rows_changed {
      1 => Ok(()),
      _ => Err(RowNotChangedError::new::<T>()),
    }
  })
}

pub fn change_rows<T: ModelName>(query: &str, params: &[&dyn rusqlite::ToSql]) -> usize {
  with_connection(|conn| {
    let mut stmt = conn.prepare(query).expect(STATEMENT_PREPARE_ERROR);
    stmt.execute(params).expect(STATEMENT_EXECUTE_ERROR)
  })
}
