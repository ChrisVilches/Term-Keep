use crate::services;
use crate::util::note_fmt;
use crate::Note;
use colored::*;
use std::error::Error;

fn print_count() -> Result<(), Box<dyn Error>> {
  let non_archived_notes: Vec<Note> = services::notes::find_all_notes(false)?;
  let archived_notes: Vec<Note> = services::notes::find_all_notes(true)?;

  println!(
    "{} note(s) ({} archived)",
    non_archived_notes.len().to_string().bold(),
    archived_notes.len().to_string().bold()
  );
  println!();
  Ok(())
}

pub fn show_all(archived: bool) -> Result<(), Box<dyn Error>> {
  print_count()?;

  let notes: Vec<Note> = services::notes::find_all_notes(archived)?;

  let pinned: Vec<&Note> = notes.iter().filter(|n| n.pinned).collect();
  let not_pinned: Vec<&Note> = notes.iter().filter(|n| !n.pinned).collect();

  for note in &pinned {
    println!(
      "{} {}",
      note_fmt::note_icons(&note),
      note_fmt::format_note_summary(&note)
    );
  }

  if pinned.len() > 0 {
    println!();
  }

  for note in &not_pinned {
    println!(
      "{} {}",
      note_fmt::note_icons(&note),
      note_fmt::format_note_summary(&note)
    );
  }

  Ok(())
}

pub fn show_one(note_id: u32) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one_note(note_id)?;

  match note.id {
    None => println!("ID: -"),
    Some(id) => println!("ID: {}", id),
  }

  println!("{}", note.content);

  Ok(())
}
