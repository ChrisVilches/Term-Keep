use crate::util::db::rows_to_vec;
use crate::models::note::Note;
use crate::services::db;
use crate::services::errors::NotFoundError;

// TODO: This file is a bit messy because each service works a bit differently from each other.

// TODO: Flag arguments are difficult to read.
pub fn find_all_notes(archived: bool) -> Result<Vec<Note>, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn
    .prepare("SELECT id, content, pinned, archived, task_status FROM note WHERE archived = ?")?;

  Ok(rows_to_vec(stmt, rusqlite::params![archived]))
}

pub fn find_one_note(id: u32) -> Option<Note> {
  let conn = db::connection();
  let stmt = conn
    .prepare("SELECT id, content, pinned, archived, task_status FROM note WHERE id = ? LIMIT 1").unwrap();

    rows_to_vec::<Note>(stmt, rusqlite::params![id])
    .first()
    .map(|t| t.clone())
}

// TODO: Maybe it'd be better that all functions return a Result, and that it has
//       to be "unwrapped" from the CLI (or commands/controllers module). I already
//       have some functions that work like that.
pub fn create_note(text: String) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("INSERT INTO note (content) VALUES (?)");
  stmt?.execute([text])?;
  Ok(())
}

pub fn create_task(text: String) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("INSERT INTO note (content, task_status) VALUES (?, 0)");
  stmt?.execute([text])?;
  Ok(())
}

pub fn update_note(id: u32, text: String) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("UPDATE note SET content = ? WHERE id = ?");
  stmt?.execute(rusqlite::params![text, id])?;
  Ok(())
}

// TODO: Throws notfounderror, but unwraps (silents / panics) the rusqlite::Error.
//       How can this be improved?
pub fn pin(id: u32, pinned: bool) -> Result<(), NotFoundError> {
  let conn = db::connection();
  let mut stmt = conn
    .prepare("UPDATE note SET pinned = ? WHERE id = ?")
    .unwrap();
  let rows_changed = stmt.execute(rusqlite::params![pinned, id]).unwrap();

  match rows_changed {
    1 => Ok(()),
    _ => Err(NotFoundError { id }),
  }
}

pub fn archive(id: u32, archived: bool) -> Result<(), NotFoundError> {
  let conn = db::connection();
  let mut stmt = conn
    .prepare("UPDATE note SET archived = ? WHERE id = ?")
    .unwrap();
  let rows_changed = stmt.execute(rusqlite::params![archived, id]).unwrap();

  match rows_changed {
    1 => Ok(()),
    _ => Err(NotFoundError { id }),
  }
}

pub fn change_task_status(id: u32, status: i32) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("UPDATE note SET task_status = ? WHERE id = ?");
  stmt?.execute(rusqlite::params![status, id])?;
  Ok(())
}
