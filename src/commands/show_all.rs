use crate::helpers::note_fmt;
use crate::services;
use crate::Note;
use colored::*;

fn print_count() {
  let non_archived_notes: Vec<Note> = services::notes::find_all_notes(false).unwrap();
  let archived_notes: Vec<Note> = services::notes::find_all_notes(true).unwrap();

  println!(
    "{} note(s) ({} archived)",
    non_archived_notes.len().to_string().bold(),
    archived_notes.len().to_string().bold()
  );
  println!();
}

pub fn show_all(archived: bool) {
  print_count();

  let notes: Vec<Note> = services::notes::find_all_notes(archived).unwrap();

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
}
