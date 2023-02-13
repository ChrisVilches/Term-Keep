use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::models::traits::RequireId;
use crate::services;
use crate::util::cli;
use anyhow::{anyhow, Result};

fn change_aux(task: &Note, current_status: TaskStatus, status: TaskStatus) {
  if current_status == status {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    services::notes::change_task_status(task.require_id(), status as i32)
      .expect("Task status should have changed");
  }
}

pub fn change_status(task_id: u32, status_str: &str) -> Result<()> {
  let task = services::notes::find_one(task_id)?;

  match task.note_type {
    NoteType::Task(current_status) => TaskStatus::from_string(status_str)
      .map(|new_status| change_aux(&task, current_status, new_status)),
    NoteType::Normal => Err(anyhow!("Not a task")),
  }
}
