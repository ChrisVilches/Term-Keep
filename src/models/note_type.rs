use crate::TaskStatus;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NoteType {
  Normal,
  Task(TaskStatus),
}
