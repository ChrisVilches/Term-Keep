use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::services;
use crate::Note;

fn change_aux(task: &Note, current_status: TaskStatus, status: TaskStatus) {
  if current_status == status {
    println!("Not changed");
    return;
  }
  services::notes::change_task_status(task.id.unwrap(), status as i32).unwrap();
}

pub fn change_status(task_id: u32, status_str: &str) -> Result<(), String> {
  let task = services::notes::find_one_note(task_id).unwrap();

  match task.note_type {
    NoteType::Task(current_status) => {
      change_aux(&task, current_status, TaskStatus::from_string(status_str)?);
      Ok(())
    }
    _ => Err("Not a task".to_string()),
  }
}
