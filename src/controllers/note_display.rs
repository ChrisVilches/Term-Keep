use crate::services;
use crate::util::cli;
use crate::util::note_fmt;
use crate::Note;
use colored::Colorize;
use std::error::Error;

fn print_count() {
  let non_archived_notes: Vec<Note> = services::notes::find_all(false);
  let archived_notes: Vec<Note> = services::notes::find_all(true);

  println!(
    "{} note(s) ({} archived)",
    non_archived_notes.len().to_string().bold(),
    archived_notes.len().to_string().bold()
  );
  println!();
}

fn format_note_aux(note: &Note) -> String {
  format!(
    "{} {}",
    note_fmt::format_note_icons(note),
    note_fmt::format_note_summary(note)
  )
}

fn find_all_pinned_tuple(archived: bool) -> (Vec<Note>, Vec<Note>) {
  let notes: Vec<Note> = services::notes::find_all(archived);
  let pinned: Vec<Note> = notes.iter().filter(|n| n.pinned).cloned().collect();
  let not_pinned: Vec<Note> = notes.iter().filter(|n| !n.pinned).cloned().collect();
  (pinned, not_pinned)
}

pub fn show_all(archived: bool) {
  print_count();

  let (pinned, not_pinned) = find_all_pinned_tuple(archived);

  for note in &pinned {
    println!("{}", format_note_aux(note));
  }

  if !pinned.is_empty() {
    println!();
  }

  for note in &not_pinned {
    println!("{}", format_note_aux(note));
  }
}

pub fn show_one(note_id: u32, use_less: bool, plain: bool) -> Result<(), Box<dyn Error>> {
  let note: Note = services::notes::find_one(note_id)?;

  if use_less {
    cli::less(&note_fmt::format_note(&note, plain));
  } else {
    println!("{}", note_fmt::format_note(&note, plain));
  }

  Ok(())
}
