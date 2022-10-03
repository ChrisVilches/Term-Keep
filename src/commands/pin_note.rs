use crate::services;
use crate::Note;

pub fn pin_note(note_id: u32, pinned: bool) {
  let note: Note = services::notes::find_one_note(note_id).unwrap();
  if note.pinned == pinned {
    println!("Not changed");
    return;
  }

  services::notes::pin(note_id, pinned).unwrap();
}
