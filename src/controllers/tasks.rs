use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::services;
use crate::Note;
use std::error::Error;

fn change_aux(
  task: &Note,
  current_status: TaskStatus,
  status: TaskStatus,
) -> Result<(), RowNotChangedError> {
  if current_status == status {
    println!("Not changed");
  } else {
    services::notes::change_task_status(task.id.unwrap(), status as i32)?;
  }

  Ok(())
}

pub fn change_status(task_id: u32, status_str: &str) -> Result<(), Box<dyn Error>> {
  let task = services::notes::find_one(task_id)?;

  match task.note_type {
    NoteType::Task(current_status) => TaskStatus::from_string(status_str)
      .map(|new_status| change_aux(&task, current_status, new_status))
      .map(|_| ())
      .map_err(std::convert::Into::into),
    NoteType::Normal => Err("Not a task")?,
  }
}
