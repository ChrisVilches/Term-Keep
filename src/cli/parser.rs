use crate::abort_with_message;
use crate::cli::commands::Commands;
use crate::cli::commands::ShowAllNotes;
use crate::controllers;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

fn command_result(cmd: &Commands) -> Result<(), Box<dyn Error>> {
  match cmd {
    // Display
    Commands::ShowAllNotes(args) => {
      controllers::note_display::show_all(args.archived);
      Ok(())
    }
    Commands::Show(args) => controllers::note_display::show_one(args.id, args.less),
    Commands::Search(args) => {
      controllers::search::find_fuzzy(&args.text, args.archived);
      Ok(())
    }

    // Editing
    Commands::EditNote { id } => controllers::note_edit::edit_content(*id),
    Commands::NewNote(args) => controllers::note_creation::create_note(&args.template_name),
    Commands::NewTask(args) => controllers::note_creation::create_task(&args.template_name),
    Commands::ChangeTaskStatus(args) => controllers::tasks::change_status(args.id, &args.status),
    Commands::ArchiveAllDone => {
      controllers::note_edit::archive_all_done();
      Ok(())
    }

    // Pin / Archive
    Commands::Pin(pin) => controllers::note_edit::pin_note(pin.id, !pin.remove),
    Commands::Archive(archive) => controllers::note_edit::archive(archive.id, !archive.remove),

    // Templates
    Commands::Templates => {
      controllers::templates::show_all();
      Ok(())
    }
    Commands::UpsertTemplate(args) => controllers::templates::upsert(&args.template_name),
    Commands::RemoveTemplate(args) => controllers::templates::remove(&args.template_name),

    // Misc
    Commands::Info => controllers::info::info(),
  }
}

pub fn create() {
  let cli = Cli::parse();

  let default_cmd = Commands::ShowAllNotes(ShowAllNotes { archived: false });

  let result: Result<(), Box<dyn Error>> = command_result(&cli.command.unwrap_or(default_cmd));

  result.unwrap_or_else(|e| abort_with_message(e));
}
