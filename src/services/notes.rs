use crate::models::note::Note;
use crate::services::db::change_rows;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;
use crate::services::errors::NotFoundByIdError;
use std::error::Error;

pub fn find_all_notes(archived: bool) -> Result<Vec<Note>, rusqlite::Error> {
  rows_to_vec(
    "SELECT id, content, pinned, archived, task_status FROM note WHERE archived = ?",
    rusqlite::params![archived],
  )
}

// TODO: I think in this case it also should return a
//       -> Result<Option<Note>, rusqlite::Error>
//       Another option would be to return a Result<Note, Box<dyn Error>>, but is it necessary?
//       I think I can just return Result<Option<Note>, rusqlite::Error> and then handle the None
//       as an error if I want, in the component that handles this.
//
//       Note that single row also silents the rusqlite::Error, so refactoring all of this would require
//       several changes.
pub fn find_one_note(id: u32) -> Result<Note, NotFoundByIdError> {
  single_row::<Note>(
    "SELECT id, content, pinned, archived, task_status FROM note WHERE id = ? LIMIT 1",
    rusqlite::params![id],
  )
  .ok_or_else(|| NotFoundByIdError {
    id,
    type_name: "note".to_string(),
  })
}

pub fn create_note(text: String) -> Result<usize, rusqlite::Error> {
  change_rows(
    "INSERT INTO note (content) VALUES (?)",
    rusqlite::params![text],
  )
}

pub fn create_task(text: String) -> Result<usize, rusqlite::Error> {
  change_rows(
    "INSERT INTO note (content, task_status) VALUES (?, 0)",
    rusqlite::params![text],
  )
}

pub fn update_note(id: u32, text: &String) -> Result<usize, rusqlite::Error> {
  change_rows(
    "UPDATE note SET content = ? WHERE id = ?",
    rusqlite::params![text, id],
  )
}

pub fn pin(id: u32, pinned: bool) -> Result<(), Box<dyn Error>> {
  let result: Result<usize, rusqlite::Error> = change_rows(
    "UPDATE note SET pinned = ? WHERE id = ?",
    rusqlite::params![pinned, id],
  );

  // TODO: A little bit verbose, try to refactor and use magic stuff.
  match result {
    Ok(rows_changed) => match rows_changed {
      1 => Ok(()),
      _ => Err(Box::new(NotFoundByIdError {
        id,
        type_name: "note".to_string(),
      })),
    },
    Err(db_error) => Err(db_error)?,
  }
}

pub fn archive(id: u32, archived: bool) -> Result<(), Box<dyn Error>> {
  let result: Result<usize, rusqlite::Error> = change_rows(
    "UPDATE note SET archived = ? WHERE id = ?",
    rusqlite::params![archived, id],
  );

  match result {
    Ok(rows_changed) => match rows_changed {
      1 => Ok(()),
      _ => Err(Box::new(NotFoundByIdError {
        id,
        type_name: "note".to_string(),
      })),
    },
    Err(db_error) => Err(Box::new(db_error)),
  }
}

pub fn change_task_status(id: u32, status: i32) -> Result<usize, rusqlite::Error> {
  change_rows(
    "UPDATE note SET task_status = ? WHERE id = ?",
    rusqlite::params![status, id],
  )
}
