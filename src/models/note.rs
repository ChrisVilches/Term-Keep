use crate::models::note_type::NoteType;

#[derive(Debug, Clone)]
pub struct Note {
  pub id: Option<i32>,
  pub content: String,
  pub pinned: bool,
  pub note_type: NoteType,
  pub archived: bool,
}

// https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite
// I can create an enum table (an actual SQL table that contains the possible data).
// and use table references.
// pub task_status: Option<TaskStatus>,
// (I think) This is more related to the database than to this model though, now.
