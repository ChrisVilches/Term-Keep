use crate::services;
use crate::Note;

pub fn show_one(note_id: u32) {
  let note: Note = services::notes::find_one_note(note_id).unwrap();

  match note.id {
    None => println!("ID: -"),
    Some(id) => println!("ID: {}", id),
  }

  println!("{}", note.content);
}
