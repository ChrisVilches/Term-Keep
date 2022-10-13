use crate::abort_with_message;
use crate::controllers;
use clap::{Args, Parser, Subcommand};
use std::error::Error;

// TODO: Split into smaller files.

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Args)]
struct ShowAllNotes {
  #[clap(long = "archived", short = 'a', help = "Only archived notes")]
  archived: bool,
}

#[derive(Args)]
struct Archive {
  #[clap(name = "Note ID")]
  id: u32,

  #[clap(long = "remove", short = 'r', help = "Remove from archive list")]
  remove: bool,
}

#[derive(Args)]
struct Pin {
  #[clap(name = "Note ID")]
  id: u32,

  #[clap(long = "remove", short = 'r', help = "Remove pin")]
  remove: bool,
}

#[derive(Args)]
struct NewNote {
  #[clap(long = "template", short = 't', help = "Choose a template")]
  template_name: Option<String>,
}

#[derive(Args)]
struct ChangeTaskStatus {
  #[clap(name = "Note (task) ID")]
  id: u32,

  #[clap(name = "New status")]
  status: String,
}

#[derive(Args)]
struct Search {
  #[clap(name = "Text to search")]
  text: String,

  #[clap(long = "archived", short = 'a', help = "Only archived notes")]
  archived: bool,
}

#[derive(Args)]
struct UpsertTemplate {
  #[clap(name = "Template name")]
  template_name: String,
}

#[derive(Args)]
struct RemoveTemplate {
  #[clap(name = "Template name")]
  template_name: String,
}

#[derive(Subcommand)]
enum Commands {
  #[command(name = "all", about = "Show all notes")]
  ShowAllNotes(ShowAllNotes),

  #[command(about = "Show one note")]
  Show { id: u32 },

  #[command(about = "Find notes (text search)", alias = "find")]
  Search(Search),

  #[command(name = "edit", about = "Edit a note")]
  EditNote { id: u32 },

  #[command(name = "new", about = "Create a new note")]
  NewNote(NewNote),

  #[command(about = "Create a new task")]
  NewTask(NewNote),

  #[command(about = "Change a task status")]
  ChangeTaskStatus(ChangeTaskStatus),

  #[command(about = "Pin a note 📌")]
  Pin(Pin),

  #[command(about = "Archive a note 📁")]
  Archive(Archive),

  #[command(about = "Show all templates")]
  Templates,

  #[command(about = "Upsert a template by name")]
  UpsertTemplate(UpsertTemplate),

  #[command(about = "Remove template by name")]
  RemoveTemplate(RemoveTemplate),

  #[command(about = "Show miscellaneous information")]
  Info,
}

fn command_result(cmd: &Commands) -> Result<(), Box<dyn Error>> {
  match cmd {
    // Display
    Commands::ShowAllNotes(args) => {
      controllers::note_display::show_all(args.archived);
      Ok(())
    }
    Commands::Show { id } => controllers::note_display::show_one(*id),
    Commands::Search(args) => {
      controllers::search::find_fuzzy(&args.text, args.archived);
      Ok(())
    }

    // Editing
    Commands::EditNote { id } => controllers::note_edit::edit_content(*id),
    Commands::NewNote(args) => controllers::note_creation::create_note(&args.template_name),
    Commands::NewTask(args) => controllers::note_creation::create_task(&args.template_name),
    Commands::ChangeTaskStatus(args) => controllers::tasks::change_status(args.id, &args.status),

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
