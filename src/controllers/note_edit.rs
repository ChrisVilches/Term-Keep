use crate::services;
use crate::Note;
use colored::*;
use std::error::Error;

pub fn edit_content(id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one_note(id)?;
  let template = note.content;

  let content = edit::edit(template.to_string())?;

  if template.eq(&content) {
    println!("{}", "Not changed".black());
    return Ok(());
  }

  println!("{}", content);

  services::notes::update_note(id, content)?;

  println!("{}", "Updated".blue());

  Ok(())
}

// TODO: A bit verbose. Maybe a better way is to simply require the note with a function that
//       either returns a Note instance (not Option, not Result) or makes the CLI crash.
//       Must belong to the CLI model, not services.
pub fn pin_note(id: u32, pinned: bool) -> Result<(), Box<dyn Error>> {
  let note = services::notes::find_one_note(id)?;

  if note.pinned == pinned {
    println!("Not changed");
    Ok(())
  } else {
    services::notes::pin(id, pinned)
  }
}

pub fn archive(note_id: u32, archived: bool) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one_note(note_id)?;

  if note.archived == archived {
    println!("Not changed");
    Ok(())
  } else {
    services::notes::archive(note_id, archived)
  }
}
