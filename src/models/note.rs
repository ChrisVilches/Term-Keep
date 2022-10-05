use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;

#[derive(Debug, Clone)]
pub struct Note {
  pub id: Option<u32>,
  pub content: String,
  pub pinned: bool,
  pub note_type: NoteType,
  pub archived: bool,
}

impl ModelName for Note {
  fn model_name() -> String {
    "note".to_string()
  }
}

impl FromSqlRow for Note {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    let note_type = match row.get(4)? {
      None => NoteType::Normal,
      num => NoteType::Task(num.unwrap_or(TaskStatus::Todo)),
    };

    Ok(Note {
      id: row.get(0)?,
      content: row.get(1)?,
      pinned: row.get(2)?,
      archived: row.get(3)?,
      note_type,
    })
  }
}

// https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite
// I can create an enum table (an actual SQL table that contains the possible data).
// and use table references.
// pub task_status: Option<TaskStatus>,
// (I think) This is more related to the database than to this model though, now.
