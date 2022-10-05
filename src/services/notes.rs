use crate::models::note::Note;
use crate::services::db::change_rows;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;
use crate::services::errors::NotFoundByIdError;

pub fn find_all(archived: bool) -> Vec<Note> {
  rows_to_vec(
    "SELECT id, content, pinned, archived, task_status FROM note WHERE archived = ?",
    rusqlite::params![archived],
  )
}

pub fn find_one(id: u32) -> Result<Note, NotFoundByIdError> {
  single_row::<Note>(
    "SELECT id, content, pinned, archived, task_status FROM note WHERE id = ? LIMIT 1",
    rusqlite::params![id],
  )
  .ok_or_else(|| NotFoundByIdError::new::<Note>(id))
}

pub fn create_note(text: String) -> usize {
  change_rows(
    "INSERT INTO note (content) VALUES (?)",
    rusqlite::params![text],
  )
}

pub fn create_task(text: String) -> usize {
  change_rows(
    "INSERT INTO note (content, task_status) VALUES (?, 0)",
    rusqlite::params![text],
  )
}

pub fn update(id: u32, text: &String) -> usize {
  change_rows(
    "UPDATE note SET content = ? WHERE id = ?",
    rusqlite::params![text, id],
  )
}

pub fn pin(id: u32, pinned: bool) -> Result<(), NotFoundByIdError> {
  let rows_changed: usize = change_rows(
    "UPDATE note SET pinned = ? WHERE id = ?",
    rusqlite::params![pinned, id],
  );

  // TODO: A little bit verbose, try to refactor and use magic stuff.
  match rows_changed {
    1 => Ok(()),
    _ => Err(NotFoundByIdError::new::<Note>(id)),
  }
}

pub fn archive(id: u32, archived: bool) -> Result<(), NotFoundByIdError> {
  let rows_changed: usize = change_rows(
    "UPDATE note SET archived = ? WHERE id = ?",
    rusqlite::params![archived, id],
  );

  match rows_changed {
    1 => Ok(()),
    _ => Err(NotFoundByIdError::new::<Note>(id)),
  }
}

pub fn change_task_status(id: u32, status: i32) -> usize {
  change_rows(
    "UPDATE note SET task_status = ? WHERE id = ?",
    rusqlite::params![status, id],
  )
}
