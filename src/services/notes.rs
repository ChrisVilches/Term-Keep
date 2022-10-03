use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::services::db;

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

pub fn create_note(text: String) {
  let conn = db::connection();
  let mut stmt = conn
    .prepare("INSERT INTO note (content) VALUES (?)")
    .unwrap();
  stmt.execute([text]).unwrap();
}

pub fn create_task(text: String) {
  let conn = db::connection();
  let mut stmt = conn
    .prepare("INSERT INTO note (content, task_status) VALUES (?, 0)")
    .unwrap();
  stmt.execute([text]).unwrap();
}

pub fn update_note(id: u32, text: String) {
  let conn = db::connection();
  let mut stmt = conn
    .prepare("UPDATE note SET content = ? WHERE id = ?")
    .unwrap();
  stmt.execute(rusqlite::params![text, id]).unwrap();
}
