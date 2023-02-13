use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::models::traits::RequireId;
use crate::services;
use crate::util::cli;
use anyhow::{anyhow, Result};

fn change_aux(
  task: &Note,
  current_status: TaskStatus,
  status: TaskStatus,
) -> Result<(), RowNotChangedError> {
  if current_status == status {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    services::notes::change_task_status(task.require_id(), status as i32)?;
  }

  Ok(())
}

pub fn change_status(task_id: u32, status_str: &str) -> Result<()> {
  let task = services::notes::find_one(task_id)?;

  match task.note_type {
    // TODO: Refactor/simplify this.
    NoteType::Task(current_status) => TaskStatus::from_string(status_str)
      .map(|new_status| change_aux(&task, current_status, new_status))
      .map(|_| ())
      .map_err(|s| anyhow!(s.to_owned())), // TODO: Does this need Err(...) ? Does that work?
    NoteType::Normal => Err(anyhow!("Not a task")),
  }
}
