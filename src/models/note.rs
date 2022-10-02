use crate::models::task_status::TaskStatus;

/**
 * TODO: A more idiomatic way would be to create another enum, for "Note type"
 * and then the task type has a TaskStatus type (normal note doesn't have it)
 */

#[derive(Debug, Clone)]
pub struct Note {
  pub id: Option<i32>,
  pub content: String,
  pub pinned: bool,
  pub task: bool,

  // https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite
  // I can create an enum table (an actual SQL table that contains the possible data).
  // and use table references.
  pub task_status: Option<TaskStatus>,
  pub archived: bool,
}
