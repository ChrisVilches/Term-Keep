use crate::services;
use crate::Note;
use colored::*;

pub fn edit_note(id: u32) {
  let note: Note = services::notes::find_one_note(id).unwrap();
  let template = note.content;

  let content = edit::edit(template.to_string()).unwrap();

  if template.eq(&content) {
    println!("{}", "Not changed".black());
    return;
  }

  println!("{}", content);

  // TODO: Should handle error (show message). This applies to most services as well.
  // Also right now this doesn't throw a "Result", so the error cannot be handled here.
  services::notes::update_note(id, content).unwrap();

  println!("{}", "Updated".blue());
}
