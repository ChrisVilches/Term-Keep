use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::services::db;
use crate::services::errors::NotFoundError;

fn row_to_note(row: &rusqlite::Row) -> Result<Note, rusqlite::Error> {
  let note_type = match row.get(4)? {
    None => NoteType::Normal,
    num => NoteType::Task(num.unwrap()), // TODO: wtf?
  };

  Ok(Note {
    id: row.get(0)?,
    content: row.get(1)?,
    pinned: row.get(2)?,
    archived: row.get(3)?,
    note_type,
  })
}

fn rows_to_vec(mut stmt: rusqlite::Statement, params: &[&dyn rusqlite::ToSql]) -> Vec<Note> {
  stmt
    .query_map(params, row_to_note)
    .unwrap()
    .map(|n| n.unwrap())
    .collect()
}

pub fn find_all_notes() -> Result<Vec<Note>, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare(
    "SELECT id, content, pinned, archived, task_status FROM note WHERE archived = false",
  )?;

  Ok(rows_to_vec(stmt, rusqlite::params![]))
}

pub fn find_one_note(id: u32) -> Result<Note, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn
    .prepare("SELECT id, content, pinned, archived, task_status FROM note WHERE id = ? LIMIT 1")?;

  Ok(
    rows_to_vec(stmt, rusqlite::params![id])
      .first()
      .expect(&*format!("Note (ID = {}) not found", id))
      .clone(),
  )
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

pub fn pin(id: u32, pinned: bool) -> Result<(), NotFoundError> {
  // TODO: Add check to see if the note actually exists or not.
  //       I think this can be done with the query return (number of rows modified)
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
