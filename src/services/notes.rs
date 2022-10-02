use crate::Note;
use rusqlite::Connection;

// TODO: Should be singleton.
pub fn database_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
  Connection::open("./test.db")
}

// Should be idempotent.
pub fn install_database() -> Result<usize, rusqlite::Error> {
  database_connection()?.execute(
    "CREATE TABLE IF NOT EXISTS note (
            id          INTEGER PRIMARY KEY,
            content     TEXT NOT NULL,
            task_status INTEGER,
            archived    BOOLEAN NOT NULL DEFAULT false, 
            pinned      BOOLEAN NOT NULL DEFAULT false,
            task        BOOLEAN NOT NULL
        )",
    (),
  )
}

pub fn create_note(content: &str) -> Note {
  Note {
    id: None,
    content: content.to_string(),
    pinned: false,
    task: false,
    archived: false,
    task_status: None,
  }
}

// TODO: Not sure about the Result<>.
pub fn insert_note(note: Note) -> Result<usize, rusqlite::Error> {
  database_connection()?.execute(
    "INSERT INTO note (content, task) VALUES (?1, ?2)",
    (&note.content, false),
  )
}

pub fn find_all_notes<'stmt>() -> Result<Vec<Note>, rusqlite::Error> {
  let conn = database_connection()?;
  let mut stmt = conn.prepare(
    "SELECT id, content, pinned, task, archived, task_status FROM note WHERE archived = false",
  )?;

  let rows = stmt.query_map([], |row| {
    Ok(Note {
      id: row.get(0)?,
      content: row.get(1)?,
      pinned: row.get(2)?,
      task: row.get(3)?,
      archived: row.get(4)?,
      task_status: row.get(5)?,
    })
  })?;

  let mut notes = Vec::<Note>::new();

  for row in rows {
    notes.push(row?);
  }

  Ok(notes)
}

// TODO: Can I remove this lifetime?
pub fn find_one_note<'stmt>(id: i32) -> Result<Note, rusqlite::Error> {
  let conn = database_connection()?;
  let mut stmt = conn.prepare(
    "SELECT id, content, pinned, task, archived, task_status FROM note WHERE id = ? LIMIT 1",
  )?;

  let rows = stmt.query_map([id], |row| {
    Ok(Note {
      id: row.get(0)?,
      content: row.get(1)?,
      pinned: row.get(2)?,
      task: row.get(3)?,
      archived: row.get(4)?,
      task_status: row.get(5)?,
    })
  })?;

  let notes: Vec<Note> = rows.map(|n| n.unwrap()).collect();

  Ok(notes.first().unwrap().clone())
}
