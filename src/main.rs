mod cli;
mod models;
mod services;
mod util;
use crate::models::note::Note;
use crate::models::task_status::TaskStatus;
use crate::notes::find_all_notes;
use crate::notes::find_one_note;
use colored::*;
use services::notes;
use util::strings;

///// Move to a file like "formatters" or "helpers", etc //////////

const NOTE_SUMMARY_MAX_LENGTH: usize = 25;

fn format_normal_note_summary(note: &Note) -> String {
  format!(
    "{} | {}",
    note.id.unwrap().to_string().white(),
    strings::truncate_string_ellipsis(note.content.to_string(), NOTE_SUMMARY_MAX_LENGTH).cyan()
  )
}

// TODO: Should be an enum as well.
fn format_task_status(task_status: Option<TaskStatus>) -> &'static str {
  match task_status.unwrap_or(TaskStatus::Todo) {
    TaskStatus::Todo => "❐",
    TaskStatus::Progress => "△",
    TaskStatus::Done => "✔",
  }
}

fn format_task_summary(note: &Note) -> String {
  let summary_text =
    strings::truncate_string_ellipsis(note.content.to_string(), NOTE_SUMMARY_MAX_LENGTH);

  let color_summary = match note.task_status.unwrap_or(TaskStatus::Todo) {
    TaskStatus::Todo => summary_text.red(),
    TaskStatus::Progress => summary_text.yellow(),
    TaskStatus::Done => summary_text.black(),
  };

  format!(
    "{} | {} | {}",
    note.id.unwrap().to_string().white(),
    format_task_status(note.task_status),
    color_summary
  )
}

fn format_note_summary(note: &Note) -> String {
  if note.task {
    format_task_summary(note)
  } else {
    format_normal_note_summary(note)
  }
}

//////////////////////////////////////////////////////////////////

fn show_all() {
  let notes: Vec<Note> = find_all_notes().unwrap();
  println!("{} note(s)", notes.len());
  println!();

  let pinned: Vec<&Note> = notes.iter().filter(|n| n.pinned).collect();
  let not_pinned: Vec<&Note> = notes.iter().filter(|n| !n.pinned).collect();

  for note in &pinned {
    println!("📌 {}", format_note_summary(&note));
  }

  if pinned.len() > 0 {
    println!();
  }

  for note in &not_pinned {
    println!("{}", format_note_summary(&note));
  }
}

fn show_one(note_id: i32) {
  let note: Note = find_one_note(note_id).unwrap();

  match note.id {
    None => println!("ID: -"),
    Some(id) => println!("ID: {}", id),
  }

  println!("{}", note.content);
}

fn main() {
  // TODO: This should be added as well.
  // connection::install_database().unwrap();

  show_all();

  /*
  insert_note(create_note("Some note hahahahaha")).unwrap();
  insert_note(create_note("Is this note amazing?? hahaah yes it is")).unwrap();
  */

  println!();

  show_one(3);
}
