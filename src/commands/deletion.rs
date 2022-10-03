use crate::services;
use crate::Note;

pub fn archive(note_id: u32, archived: bool) {
  let note: Note = services::notes::find_one_note(note_id).unwrap();
  if note.archived == archived {
    println!("Not changed");
    return;
  }

  services::notes::archive(note_id, archived).unwrap();
}

// TODO: Implement permanent deletion?
