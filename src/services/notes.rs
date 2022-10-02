use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::services::db;

pub fn create_note(content: &str) -> Note {
  Note {
    id: None,
    content: content.to_string(),
    pinned: false,
    note_type: NoteType::Normal,
    archived: false,
  }
}

// TODO: Not sure about the Result<>.
pub fn insert_note(note: Note) -> Result<usize, rusqlite::Error> {
  db::connection().execute(
    "INSERT INTO note (content, task) VALUES (?1, ?2)",
    (&note.content, false),
  )
}

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

pub fn find_one_note(id: i32) -> Result<Note, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn
    .prepare("SELECT id, content, pinned, archived, task_status FROM note WHERE id = ? LIMIT 1")?;

  Ok(
    rows_to_vec(stmt, rusqlite::params![id])
      .first()
      .unwrap()
      .clone(),
  )
}
