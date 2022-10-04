use crate::commands;
use crate::util::cli::abort_with_message;
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
  #[clap(long = "archived", short = 'a', action, help = "Only archived notes")]
  archived: bool,
}

#[derive(Args)]
struct Archive {
  #[clap(name = "Note ID")]
  id: u32,

  #[clap(
    long = "remove",
    short = 'r',
    action,
    help = "Remove from archive list"
  )]
  remove: bool,
}

#[derive(Args)]
struct Pin {
  #[clap(name = "Note ID")]
  id: u32,

  // TODO: If I remove the "action", does the value never become true? I think that's what it does (confirm)
  #[clap(long = "remove", short = 'r', action, help = "Remove pin")]
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

  #[clap(long = "archived", short = 'a', action, help = "Only archived notes")]
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
 *
 *
 * TODO: Prettify (by using abort_with_message, or something similar) all the crashes of other commands.
 */

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
    Commands::Search(search) => {
      commands::search::find_fuzzy(search.text.to_string(), search.archived);
    }
    Commands::EditNote { id } => {
      commands::edit_note::edit_note(*id);
    }
    Commands::NewNote(new_note) => {
      commands::create_note::create_note(false, &new_note.template_name);
    }
    Commands::NewTask(new_note) => {
      commands::create_note::create_note(true, &new_note.template_name);
    }
    Commands::ChangeTaskStatus(change_task_status) => {
      let result =
        commands::tasks::change_status(change_task_status.id, &change_task_status.status);

      match result {
        Ok(_) => {}
        Err(e) => abort_with_message(e),
      }
    }
    Commands::Pin(pin) => {
      commands::pin_note::pin_note(pin.id, !pin.remove);
    }
    Commands::Archive(archive) => {
      commands::deletion::archive(archive.id, !archive.remove);
    }
    Commands::Templates => {
      commands::templates::show_all();
    }
    Commands::UpsertTemplate(upsert) => {
      commands::templates::upsert(upsert.template_name.to_string());
    }
    Commands::RemoveTemplate(remove) => {
      commands::templates::remove(remove.template_name.to_string());
    }
    Commands::Info => {
      commands::info::info();
    }
  }
}
