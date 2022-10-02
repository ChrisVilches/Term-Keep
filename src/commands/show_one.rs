use crate::commands::show_one::notes::find_one_note;
use crate::services::notes;
use crate::Note;

pub fn show_one(note_id: i32) {
  let note: Note = find_one_note(note_id).unwrap();

  match note.id {
    None => println!("ID: -"),
    Some(id) => println!("ID: {}", id),
  }

  println!("{}", note.content);
}
