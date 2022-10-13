use crate::services;
use crate::util::note_fmt;
use crate::Note;
use colored::Colorize;
use std::error::Error;

pub fn edit_content(id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(id)?;
  let template = note.content;

  let content = edit::edit(&template)?;

  let same_content = template.eq(&content);

  if !same_content {
    services::notes::update(id, &content)?;
  }

  note_fmt::print_note(&services::notes::find_one(id)?);

  if same_content {
    println!("{}", "Not changed".black());
  } else {
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
    services::notes::archive(note_id, archived)?;
  }

  Ok(())
}
