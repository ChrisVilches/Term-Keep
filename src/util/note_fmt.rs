use crate::models::note::Note;
use crate::models::note_type::NoteType;
use crate::models::task_status::TaskStatus;
use crate::util::checklists;
use crate::util::env::get_env_var;
use crate::util::strings;
use crate::util::strings::count_lines;
use colored::Colorize;
use termimad::MadSkin;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn note_summary_max_length() -> usize {
  get_env_var::<usize>("SUMMARY_MAX_LENGTH").unwrap_or(50)
}

fn format_content(content: &str) -> String {
  strings::truncate_string_ellipsis(
    strings::first_line(content).trim().to_string(),
    note_summary_max_length(),
  )
}

fn format_task_status_icon(task_status: TaskStatus) -> String {
  match task_status {
    TaskStatus::Todo => "[   ]".red().bold(),
    TaskStatus::Progress => "[ - ]".yellow().bold(),
    TaskStatus::Done => "[ ✔ ]".green().bold(),
  }
  .to_string()
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
      desc.push(format_task_status_text(task_status).into());
    }
  };

  desc
    .into_iter()
    .filter(|d| !d.is_empty())
    .collect::<Vec<String>>()
    .join("  |  ")
}

fn lines_amount_info(note: &Note) -> String {
  let lines = count_lines(&note.content);

  if lines == 1 {
    String::new()
  } else {
    format!(" ({} lines)", lines).dimmed().to_string()
  }
}

fn format_normal_note_summary(note: &Note) -> String {
  format!(
    "{: >3}\t{}{}",
    note.id.unwrap().to_string().bold(),
    format_content(&note.content),
    lines_amount_info(note)
  )
}

fn format_task_checklist_completion(full_text: &str) -> String {
  let (complete, total) = checklists::checklist_completion(full_text);

  if total == 0 {
    return " ".into();
  }

  format!(" {} ", format!("({} / {})", complete, total).dimmed())
}

fn format_task_summary(note: &Note, status: TaskStatus) -> String {
  let task_summary = match status {
    TaskStatus::Done => format_content(&note.content).dimmed().to_string(),
    TaskStatus::Todo | TaskStatus::Progress => format_content(&note.content),
  };

  format!(
    "{: >3}\t{}{}{}{}",
    note.id.unwrap().to_string().bold(),
    format_task_status_icon(status),
    format_task_checklist_completion(&note.content),
    task_summary,
    lines_amount_info(note)
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

fn format_note_content(s: &str) -> String {
  // TODO: There are some better (smarter) ways to use termimad:
  //       - Use a different skin, or use different colors.
  //       - Don't convert to string, but instead, use the builtin "print" function,
  //         which makes the terminal scrollable and probably sets the width automatically.
  //         I think this might be overkill, but worth trying.
  MadSkin::default()
    .text(&checklists::format_checklist(s), None)
    .to_string()
    .trim()
    .into()
}

pub fn print_note(note: &Note, plain: bool) {
  println!("{}", format_note_description(note).blue());

  // TODO: Date display is beta. Might need to change the DateTime<Utc> to something else?

  let mut date_display = String::new();
  date_display += &format!("{}", note.created_at.format(DATE_FORMAT));

  if note.created_at != note.updated_at {
    date_display += &format!(" (Updated: {})", note.updated_at.format(DATE_FORMAT));
  }

  println!("{}", date_display.dimmed());

  println!();

  if plain {
    println!("{}", note.content);
  } else {
    println!("{}", format_note_content(&note.content));
  }
}
