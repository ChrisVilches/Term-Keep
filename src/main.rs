mod cli;
mod commands;
mod models;
mod services;
mod util;
use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use colored::*;
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
fn format_task_status(task_status: TaskStatus) -> &'static str {
  match task_status {
    TaskStatus::Todo => "❐",
    TaskStatus::Progress => "△",
    TaskStatus::Done => "✔",
  }
}

fn format_task_summary(note: &Note, status: TaskStatus) -> String {
  let summary_text =
    strings::truncate_string_ellipsis(note.content.to_string(), NOTE_SUMMARY_MAX_LENGTH);

  let color_summary = match status {
    TaskStatus::Todo => summary_text.red(),
    TaskStatus::Progress => summary_text.yellow(),
    TaskStatus::Done => summary_text.black(),
  };

  format!(
    "{} | {} | {}",
    note.id.unwrap().to_string().white(),
    format_task_status(status),
    color_summary
  )
}

fn format_note_summary(note: &Note) -> String {
  match note.note_type {
    NoteType::Normal => format_normal_note_summary(note),
    NoteType::Task(status) => format_task_summary(note, status),
  }
}

//////////////////////////////////////////////////////////////////

fn main() {
  services::db::install_database().unwrap();

  // TODO: This way of calling it should be different.
  commands::show_all::show_all();

  /*
  insert_note(create_note("Some note hahahahaha")).unwrap();
  insert_note(create_note("Is this note amazing?? hahaah yes it is")).unwrap();
  */

  println!();

  // commands::show_one::show_one(3);

  // commands::create_note::create_note(true);

  // commands::edit_note::edit_note(3);
}
