use crate::commands;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

// TODO: Cannot be executed as "$ term_keep -a". It must be "$ term_keep all -a"
//       I'd like to fix this so it can be executed with -a. Not urgent.
#[derive(Args)]
struct ShowAllNotes {
  #[clap(
    long = "archived",
    short = 'a',
    action,
    help = "Only archived notes/tasks"
  )]
  archived: bool,
}

#[derive(Subcommand)]
enum Commands {
  #[command(name = "all", about = "Show all notes")]
  ShowAllNotes(ShowAllNotes),

  #[command(about = "Show one note/task")]
  Show { id: u32 },

  #[command(name = "edit", about = "Edit a note")]
  EditNote { id: u32 },

  #[command(name = "new", about = "Create a new note")]
  NewNote,

  #[command(about = "Create a new task")]
  NewTask,

  #[command(about = "Pin a note")]
  Pin { id: u32 },

  #[command(about = "Unpin a note")]
  Unpin { id: u32 },

  #[command(about = "Archive a note")]
  Archive { id: u32 },

  #[command(about = "Unarchive a note")]
  Unarchive { id: u32 },

  #[command(about = "Show configuration information")]
  Info,
}

pub fn create_cli() {
  let cli = Cli::parse();

  match &cli
    .command
    .unwrap_or(Commands::ShowAllNotes(ShowAllNotes { archived: false }))
  {
    Commands::ShowAllNotes(show_all_notes) => {
      // TODO: This way of calling it should be different.
      //       Maybe scoped by similarity (display, creation, etc).
      //       Also, shouldn't it be "controllers"?
      commands::show_all::show_all(show_all_notes.archived);
    }
    Commands::Show { id } => {
      commands::show_one::show_one(*id);
    }
    Commands::EditNote { id } => {
      commands::edit_note::edit_note(*id);
    }
    Commands::NewNote => {
      commands::create_note::create_note(false);
    }
    Commands::NewTask => {
      commands::create_note::create_note(true);
    }
    Commands::Pin { id } => {
      commands::pin_note::pin_note(*id, true);
    }
    Commands::Unpin { id } => {
      commands::pin_note::pin_note(*id, false);
    }
    Commands::Archive { id } => {
      commands::deletion::archive(*id, true);
    }
    Commands::Unarchive { id } => {
      commands::deletion::archive(*id, false);
    }
    Commands::Info => {
      commands::info::info();
    }
  }
}
