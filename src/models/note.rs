use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Note {
  pub id: Option<u32>,
  pub content: String,
  pub pinned: bool,
  pub note_type: NoteType,
  pub archived: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Note {
  pub fn is_edited(&self) -> bool {
    self.created_at != self.updated_at
  }
}

impl ModelName for Note {
  fn model_name() -> String {
    "note".to_owned()
  }
}

impl FromSqlRow for Note {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    let note_type = match row.get("task_status")? {
      None => NoteType::Normal,
      num => NoteType::Task(num.unwrap_or(TaskStatus::Todo)),
    };

    Ok(Self {
      id: row.get("id")?,
      content: row.get("content")?,
      pinned: row.get("pinned")?,
      archived: row.get("archived")?,
      note_type,
      created_at: row.get("created_at")?,
      updated_at: row.get("updated_at")?,
    })
  }
}

// https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite
// I can create an enum table (an actual SQL table that contains the possible data).
// and use table references.
// pub task_status: Option<TaskStatus>,
// (I think) This is more related to the database than to this model though, now.

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_edited() {
    let note = mock_note();
    assert!(!note.is_edited());

    let mut note_edited = mock_note();
    note_edited.updated_at = Utc::now() + chrono::Duration::minutes(2);
    assert!(note_edited.is_edited());
  }

  fn mock_note() -> Note {
    let now = Utc::now();

    Note {
      id: None,
      note_type: NoteType::Normal,
      pinned: false,
      archived: true,
      content: String::new(),
      created_at: now,
      updated_at: now,
    }
  }
}
