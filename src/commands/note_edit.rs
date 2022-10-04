use crate::abort_with_message;
use crate::services;
use crate::util::cli::require_note;
use crate::Note;
use colored::*;

pub fn edit_content(id: u32) {
  let note: Note = require_note(id);
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

// TODO: A bit verbose. Maybe a better way is to simply require the note with a function that
//       either returns a Note instance (not Option, not Result) or makes the CLI crash.
//       Must belong to the CLI model, not services.
pub fn pin_note(id: u32, pinned: bool) {
  let note = require_note(id);

  if note.pinned == pinned {
    println!("Not changed");
  } else {
    match services::notes::pin(id, pinned) {
      Ok(_) => {}
      Err(e) => abort_with_message(e),
    }
  }
}

pub fn archive(note_id: u32, archived: bool) {
  let note: Note = require_note(note_id);
  if note.archived == archived {
    println!("Not changed");
  } else {
    match services::notes::archive(note_id, archived) {
      Ok(_) => {}
      Err(e) => abort_with_message(e),
    }
  }
}
