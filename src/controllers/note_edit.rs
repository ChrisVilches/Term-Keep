use crate::services;
use crate::Note;
use colored::*;
use std::error::Error;

pub fn edit_content(id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(id)?;
  let template = note.content;

  let content = edit::edit(template.to_string())?;

  if template.eq(&content) {
    println!("{}", "Not changed".black());
  } else {
    services::notes::update(id, &content)?;
    println!("{}", content);
    println!("{}", "Updated".blue());
  }

  Ok(())
}

pub fn pin_note(id: u32, pinned: bool) -> Result<(), Box<dyn Error>> {
  let note = services::notes::find_one(id)?;

  if note.pinned == pinned {
    println!("Not changed");
  } else {
    services::notes::pin(id, pinned)?;
  }

  Ok(())
}

pub fn archive(note_id: u32, archived: bool) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(note_id)?;

  if note.archived == archived {
    println!("Not changed");
  } else {
    services::notes::archive(note_id, archived)?
  }

  Ok(())
}
