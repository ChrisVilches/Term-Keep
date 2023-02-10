use crate::models::note::Note;
use crate::services;
use crate::util::cli;
use crate::util::cli::get_text_input;
use crate::util::note_fmt;
use std::error::Error;

pub fn edit_content(id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(id)?;
  let prev_content = note.content;

  let (new_content, _) = get_text_input(&prev_content)?;

  let same_content = prev_content.eq(&new_content);

  if !same_content {
    services::notes::update(id, &new_content)?;
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
    cli::color_primary(&format!("{changed} note(s) were archived"))
  );
}

pub fn remove_note(note_id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(note_id)?;

  if !note.archived {
    Err("The note must be archived before removing permanently")?;
  }

  services::notes::remove(note_id)?;

  note_fmt::print_note(&note, false);

  println!();

  println!("{}", cli::color_danger("Removed"));

  Ok(())
}
