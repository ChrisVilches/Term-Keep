use crate::cli::command::Command;
use crate::cli::command::ShowAllNotes;
use crate::controllers;
use crate::util;
use crate::util::cli::abort_with_message;
use crate::util::env::get_bool;
use clap::Parser;
use colored::Colorize;
use std::error::Error;

static LOGO: &str = include_str!("../../data/logo.txt");
const DEFAULT_CMD: Command = Command::ShowAllNotes(ShowAllNotes { archived: false });

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>,
}

#[allow(clippy::too_many_lines)]
fn command_result(cmd: &Command) -> Result<(), Box<dyn Error>> {
  match cmd {
    // Display
    Command::ShowAllNotes(args) => {
      controllers::note_display::show_all(args.archived);
      Ok(())
    }
    Command::Show(args) => controllers::note_display::show_one(args.id, args.less, args.plain),
    Command::Search(args) => {
      if args.tag_name {
        controllers::tags::find_notes_by_tag(&args.text);
      } else {
        controllers::search::find_fuzzy(&args.text);
      }
      Ok(())
    }

    // Editing
    Command::EditNote { id } => controllers::note_edit::edit_content(*id),
    Command::NewNote(args) => controllers::note_creation::create_note(&args.template_name),
    Command::NewTask(args) => controllers::note_creation::create_task(&args.template_name),
    Command::ChangeTaskStatus(args) => controllers::tasks::change_status(args.id, &args.status),
    Command::ArchiveAllDone => {
      controllers::note_edit::archive_all_done();
      Ok(())
    }
    Command::RemoveNote(args) => controllers::note_edit::remove_note(args.id),

    // Pin / Archive
    Command::Pin(pin) => controllers::note_edit::pin_note(pin.id, !pin.remove),
    Command::Archive(archive) => controllers::note_edit::archive(archive.id, !archive.remove),

    // Templates
    Command::Templates => {
      controllers::templates::show_all();
      Ok(())
    }
    Command::UpsertTemplate(args) => controllers::templates::upsert(&args.template_name),
    Command::RemoveTemplate(args) => controllers::templates::remove(&args.template_name),

    // Tags
    Command::ShowTags(args) => {
      controllers::tags::show_all(args.case_sensitive);
      Ok(())
    }

    // Misc
    Command::Info => controllers::info::info(),
  }
}

fn should_show_logo(cmd: &Command) -> bool {
  matches!(cmd, Command::ShowAllNotes(_) | Command::Info) && !get_bool("HIDE_LOGO", false)
}

const fn should_show_tips(cmd: &Command) -> bool {
  matches!(cmd, Command::ShowAllNotes(_) | Command::Info)
}

pub fn execute() {
  let cli = Cli::parse();
  let cmd: Command = cli.command.unwrap_or(DEFAULT_CMD);

  if should_show_logo(&cmd) {
    println!("{}", LOGO.trim().green());
    println!();
  }

  let result: Result<(), Box<dyn Error>> = command_result(&cmd);

  result.unwrap_or_else(|e| abort_with_message(e));

  if should_show_tips(&cmd) {
    util::cli::show_random_tip();
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use super::*;

  #[test]
  fn test_should_show_logo() {
    env::set_var("TERM_KEEP_HIDE_LOGO", " 0 ");

    assert!(should_show_logo(&Command::Info));
    assert!(should_show_logo(&Command::ShowAllNotes(ShowAllNotes {
      archived: true
    })));
    assert!(!should_show_logo(&Command::Templates));
    assert!(!should_show_logo(&Command::ArchiveAllDone));

    env::set_var("TERM_KEEP_HIDE_LOGO", " 1 ");

    assert!(!should_show_logo(&Command::Info));
    assert!(!should_show_logo(&Command::ShowAllNotes(ShowAllNotes {
      archived: true
    })));
  }

  #[test]
  fn test_should_show_tips() {
    assert!(should_show_tips(&Command::Info));
    assert!(should_show_tips(&Command::ShowAllNotes(ShowAllNotes {
      archived: true
    })));
    assert!(!should_show_tips(&Command::Templates));
    assert!(!should_show_tips(&Command::ArchiveAllDone));
  }
}
