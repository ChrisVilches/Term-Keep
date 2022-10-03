use crate::helpers::note_fmt;
use crate::services;
use crate::Note;

pub fn show_all() {
  let notes: Vec<Note> = services::notes::find_all_notes().unwrap();
  println!("{} note(s)", notes.len());
  println!();

  let pinned: Vec<&Note> = notes.iter().filter(|n| n.pinned).collect();
  let not_pinned: Vec<&Note> = notes.iter().filter(|n| !n.pinned).collect();

  for note in &pinned {
    println!("📌 {}", note_fmt::format_note_summary(&note));
  }

  if pinned.len() > 0 {
    println!();
  }

  for note in &not_pinned {
    println!("{}", note_fmt::format_note_summary(&note));
  }
}
