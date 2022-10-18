use crate::errors::not_found_by_id_error::NotFoundByIdError;
use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::note::Note;
use crate::models::task_status::TaskStatus;
use crate::services::db::change_row;
use crate::services::db::change_rows;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;

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

pub fn find_latest() -> Option<Note> {
  single_row::<Note>(
    "SELECT id, content, pinned, archived, task_status FROM note ORDER BY id DESC LIMIT 1",
    rusqlite::params![],
  )
}

pub fn create_note(text: &str) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "INSERT INTO note (content) VALUES (?)",
    rusqlite::params![text],
  )
}

pub fn create_task(text: &str) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "INSERT INTO note (content, task_status) VALUES (?, 0)",
    rusqlite::params![text],
  )
}

pub fn update(id: u32, text: &String) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET content = ? WHERE id = ?",
    rusqlite::params![text, id],
  )
}

pub fn pin(id: u32, pinned: bool) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET pinned = ? WHERE id = ?",
    rusqlite::params![pinned, id],
  )
}

pub fn archive(id: u32, archived: bool) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET archived = ? WHERE id = ?",
    rusqlite::params![archived, id],
  )
}

pub fn change_task_status(id: u32, status: i32) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET task_status = ? WHERE id = ?",
    rusqlite::params![status, id],
  )
}

pub fn archive_all_done() -> usize {
  change_rows::<Note>(
    "UPDATE note SET archived = true WHERE task_status = ? AND archived = false",
    rusqlite::params![TaskStatus::Done],
  )
}
