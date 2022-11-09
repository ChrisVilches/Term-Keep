use crate::services;
use crate::util::cli;
use crate::util::note_fmt;
use crate::Note;
use std::error::Error;

pub fn edit_content(id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(id)?;
  let template = note.content;

  let content = edit::edit(&template)?;

  let same_content = template.eq(&content);

  if !same_content {
    services::notes::update(id, &content)?;
  }

  note_fmt::print_note(&services::notes::find_one(id)?, false);

  println!();

  if same_content {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    println!("{}", cli::color_primary("Updated"));
  }

  Ok(())
}

pub fn pin_note(id: u32, pinned: bool) -> Result<(), Box<dyn Error>> {
  let note = services::notes::find_one(id)?;

  if note.pinned == pinned {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    services::notes::pin(id, pinned)?;
  }

  Ok(())
}

pub fn archive(note_id: u32, archived: bool) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(note_id)?;

  if note.archived == archived {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    services::notes::archive(note_id, archived)?;
  }

  Ok(())
}

pub fn archive_all_done() {
  let changed = services::notes::archive_all_done();

  println!(
    "{}",
    cli::color_primary(&format!("{} note(s) were archived", changed))
  );
}
