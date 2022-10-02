use crate::TaskStatus;

#[derive(Debug, Copy, Clone)]
pub enum NoteType {
  Normal,
  Task(TaskStatus),
}
