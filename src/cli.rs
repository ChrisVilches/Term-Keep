use crate::commands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  #[command(about = "Show all notes")]
  All,

  #[command(about = "Show one note/task")]
  Show { id: u32 },

  #[command(about = "Edit a note")]
  EditNote { id: u32 },

  #[command(about = "Create a new note")]
  NewNote,

  #[command(about = "Create a new task")]
  NewTask,

  #[command(about = "Show configuration information")]
  Info,
}

pub fn create_cli() {
  let cli = Cli::parse();

  match &cli.command.unwrap_or(Commands::All) {
    Commands::All => {
      // TODO: This way of calling it should be different.
      //       Maybe scoped by similarity (display, creation, etc).
      //       Also, shouldn't it be "controllers"?
      commands::show_all::show_all();
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
    Commands::Info => {
      commands::info::info();
    }
  }
}
