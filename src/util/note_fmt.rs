use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::util::env::get_env_var;
use crate::util::strings;
use colored::*;

fn note_summary_max_length() -> usize {
  get_env_var::<usize>("SUMMARY_MAX_LENGTH").unwrap_or(50)
}

fn format_content(content: String) -> String {
  strings::truncate_string_ellipsis(strings::first_line(content), note_summary_max_length())
}

fn format_normal_note_summary(note: &Note) -> String {
  format!(
    "{} | {}",
    note.id.unwrap().to_string().white(),
    format_content(note.content.to_string()).cyan()
  )
}

fn format_task_status_icon(task_status: TaskStatus) -> &'static str {
  match task_status {
    TaskStatus::Todo => "❐",
    TaskStatus::Progress => "△",
    TaskStatus::Done => "✔",
  }
}

fn format_task_status_text(task_status: TaskStatus) -> &'static str {
  match task_status {
    TaskStatus::Todo => "To-Do",
    TaskStatus::Progress => "In Progress",
    TaskStatus::Done => "Done",
  }
}

pub fn format_note_description(note: &Note) -> String {
  let mut desc = vec![];

  desc.push(format_note_icons(note));

  desc.push(format!("ID {}", note.id.unwrap()));

  match note.note_type {
    NoteType::Normal => desc.push("Note".into()),
    NoteType::Task(task_status) => {
      desc.push("Task".into());
      desc.push(format_task_status_text(task_status).into())
    }
  };

  desc
    .into_iter()
    .filter(|d| !d.is_empty())
    .collect::<Vec<String>>()
    .join("  |  ")
}

fn format_task_summary(note: &Note, status: TaskStatus) -> String {
  let summary_text = format_content(note.content.to_string());

  let color_summary = match status {
    TaskStatus::Todo => summary_text.red(),
    TaskStatus::Progress => summary_text.on_truecolor(207, 199, 132),
    TaskStatus::Done => summary_text
      .truecolor(160, 160, 160)
      .on_truecolor(91, 168, 72),
  };

  format!(
    "{} | {} | {}",
    note.id.unwrap().to_string().white(),
    format_task_status_icon(status),
    color_summary
  )
}

pub fn format_note_summary(note: &Note) -> String {
  match note.note_type {
    NoteType::Normal => format_normal_note_summary(note),
    NoteType::Task(status) => format_task_summary(note, status),
  }
}

fn note_icons(note: &Note) -> Vec<&str> {
  let pin_icon = "📌";
  let archive_icon = "📁";

  let mut icons = Vec::<&str>::new();

  if note.archived {
    icons.push(archive_icon);
  }

  if note.pinned {
    icons.push(pin_icon);
  }

  icons
}

pub fn format_note_icons(note: &Note) -> String {
  note_icons(note).join(" ")
}
