use crate::commands;
use clap::{Args, Parser, Subcommand};

// TODO: File is too large. Split into smaller files.

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

  #[command(about = "Pin a note")]
  Pin(Pin),

  #[command(about = "Archive a note")]
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

/**
 * TODO:
 * Some of these things can be improved by passing the entire Command object as argument,
 * since it's still a module related to comments, the modules are still decoupled (this wouldn't
 * be the case if I was passing the objects to the services module).
 */

 /**
  * TODO: Another way of structuring this is by making all commands return an error, and then
  * the error is handled here. With this structure, I can then in the future easily build an interactive
  * application that requires you to input a command in the CLI interface (like git-change-date), and then
  * if it fails, it's handled by simply printing a message but without exiting.
  * Doing this restructure only requires one little change (in this file, at least, since each command file must
  * be changed in several places, probably), since all I have to do is get the value evaluated by the "match" in the
  * command parser, and then handle that error if any.
  */

pub fn create_cli() {
  let cli = Cli::parse();

  let default_cmd = Commands::ShowAllNotes(ShowAllNotes { archived: false });

  match &cli.command.unwrap_or(default_cmd) {
    // Display
    Commands::ShowAllNotes(args) => commands::note_display::show_all(args.archived),
    Commands::Show { id } => commands::note_display::show_one(*id),
    Commands::Search(args) => commands::search::find_fuzzy(args.text.to_string(), args.archived),

    // Editing
    Commands::EditNote { id } => commands::note_edit::edit_content(*id),
    Commands::NewNote(args) => commands::note_creation::create_note(&args.template_name),
    Commands::NewTask(args) => commands::note_creation::create_task(&args.template_name),
    Commands::ChangeTaskStatus(args) => commands::tasks::change_status(args.id, &args.status),

    // Pin / Archive
    Commands::Pin(pin) => commands::note_edit::pin_note(pin.id, !pin.remove),
    Commands::Archive(archive) => commands::note_edit::archive(archive.id, !archive.remove),

    // Templates
    Commands::Templates => commands::templates::show_all(),
    Commands::UpsertTemplate(args) => commands::templates::upsert(args.template_name.to_string()),
    Commands::RemoveTemplate(args) => commands::templates::remove(args.template_name.to_string()),

    // Misc
    Commands::Info => commands::info::info(),
  }
}
