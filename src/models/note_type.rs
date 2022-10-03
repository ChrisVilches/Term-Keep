use crate::TaskStatus;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NoteType {
  Normal,
  Task(TaskStatus),
}
