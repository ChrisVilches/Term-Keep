use super::task_status::TaskStatus;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NoteType {
  Normal,
  Task(TaskStatus),
}
