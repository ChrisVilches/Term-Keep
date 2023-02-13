use anyhow::{anyhow, Result};
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use rusqlite::ToSql;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
  Todo = 0,
  Progress = 1,
  Done = 2,
}

const INVALID_STATUS_ERROR: &str = "Invalid status (allowed values: todo, progress, done)";

impl TaskStatus {
  pub fn from_string(status: &str) -> Result<Self> {
    let lower = status.to_lowercase();

    match lower.as_str() {
      "todo" => Ok(Self::Todo),
      "progress" => Ok(Self::Progress),
      "done" => Ok(Self::Done),
      _ => Err(anyhow!(INVALID_STATUS_ERROR)),
    }
  }
}

impl FromSql for TaskStatus {
  #[inline]
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    match value {
      ValueRef::Integer(0) => Ok(Self::Todo),
      ValueRef::Integer(1) => Ok(Self::Progress),
      ValueRef::Integer(2) => Ok(Self::Done),
      _ => Err(FromSqlError::Other(
        format!("Cannot convert value {value:?} to status").into(),
      )),
    }
  }
}

impl ToSql for TaskStatus {
  fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
    Ok((*self as i32).into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_string() {
    let parse = |s: &str| TaskStatus::from_string(s).ok().unwrap();
    assert_eq!(parse("todo"), TaskStatus::Todo);
    assert_eq!(parse("progress"), TaskStatus::Progress);
    assert_eq!(parse("done"), TaskStatus::Done);
    assert_eq!(parse("ToDo"), TaskStatus::Todo);
    assert_eq!(parse("PrOgReSs"), TaskStatus::Progress);
    assert_eq!(parse("dOnE"), TaskStatus::Done);
  }

  #[test]
  fn test_from_string_invalid() {
    let parse_get_err_msg = |s: &str| TaskStatus::from_string(s).err().unwrap().to_string();
    assert_eq!(parse_get_err_msg("tODOs"), INVALID_STATUS_ERROR);
    assert_eq!(parse_get_err_msg("Doned"), INVALID_STATUS_ERROR);
    assert_eq!(parse_get_err_msg("xxxyyzzz"), INVALID_STATUS_ERROR);
    assert_eq!(parse_get_err_msg(" todo "), INVALID_STATUS_ERROR);
  }

  #[test]
  fn test_column_result() {
    assert_eq!(
      TaskStatus::column_result(ValueRef::Integer(0)),
      Ok(TaskStatus::Todo)
    );
    assert_eq!(
      TaskStatus::column_result(ValueRef::Integer(1)),
      Ok(TaskStatus::Progress)
    );
    assert_eq!(
      TaskStatus::column_result(ValueRef::Integer(2)),
      Ok(TaskStatus::Done)
    );
    assert_eq!(
      TaskStatus::column_result(ValueRef::Integer(3))
        .err()
        .unwrap()
        .to_string(),
      FromSqlError::Other("Cannot convert value Integer(3) to status".into()).to_string()
    );
  }

  #[test]
  fn test_to_sql() {
    assert_eq!(TaskStatus::Todo.to_sql(), Ok(ToSqlOutput::from(0)));
    assert_eq!(TaskStatus::Progress.to_sql(), Ok(ToSqlOutput::from(1)));
    assert_eq!(TaskStatus::Done.to_sql(), Ok(ToSqlOutput::from(2)));
  }
}
