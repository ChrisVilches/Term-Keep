use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
  Todo = 0,
  Progress = 1,
  Done = 2,
}

impl TaskStatus {
  pub fn from_string(status: &str) -> Result<TaskStatus, String> {
    match status {
      "todo" => Ok(TaskStatus::Todo),
      "progress" => Ok(TaskStatus::Progress),
      "done" => Ok(TaskStatus::Done),
      _ => Err("Invalid status (allowed values: todo, progress, done)".to_string()),
    }
  }
}

impl FromSql for TaskStatus {
  #[inline]
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    match value {
      ValueRef::Integer(0) => Ok(TaskStatus::Todo),
      ValueRef::Integer(1) => Ok(TaskStatus::Progress),
      ValueRef::Integer(2) => Ok(TaskStatus::Done),
      _ => todo!(), // TODO: What is this? Can I improve this?
    }
  }
}
