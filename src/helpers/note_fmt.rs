use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::util::strings;
use colored::*;

const NOTE_SUMMARY_MAX_LENGTH: usize = 25;

fn format_content(content: String) -> String {
  strings::truncate_string_ellipsis(strings::first_line(content), NOTE_SUMMARY_MAX_LENGTH)
}

fn format_normal_note_summary(note: &Note) -> String {
  format!(
    "{} | {}",
    note.id.unwrap().to_string().white(),
    format_content(note.content.to_string()).cyan()
  )
}

fn format_task_status(task_status: TaskStatus) -> &'static str {
  match task_status {
    TaskStatus::Todo => "❐",
    TaskStatus::Progress => "△",
    TaskStatus::Done => "✔",
  }
}

fn format_task_summary(note: &Note, status: TaskStatus) -> String {
  let summary_text = format_content(note.content.to_string());

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

pub fn format_note_summary(note: &Note) -> String {
  match note.note_type {
    NoteType::Normal => format_normal_note_summary(note),
    NoteType::Task(status) => format_task_summary(note, status),
  }
}
