use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ShowAllNotes {
  #[clap(long = "archived", short = 'a', help = "Only archived notes")]
  pub archived: bool,
}

#[derive(Args)]
pub struct ShowOne {
  #[clap(name = "Note ID")]
  pub id: u32,

  #[clap(
    long = "less",
    short = 'l',
    help = "Use the 'less' command to display the content"
  )]
  pub less: bool,

  #[clap(long = "plain", short = 'p', help = "Disable formatting")]
  pub plain: bool,
}

#[derive(Args)]
pub struct Archive {
  #[clap(name = "Note ID")]
  pub id: u32,

  #[clap(long = "remove", short = 'r', help = "Remove from archive list")]
  pub remove: bool,
}

#[derive(Args)]
pub struct Pin {
  #[clap(name = "Note ID")]
  pub id: u32,

  #[clap(long = "remove", short = 'r', help = "Remove pin")]
  pub remove: bool,
}

#[derive(Args)]
pub struct NewNote {
  #[clap(long = "template", short = 't', help = "Choose a template")]
  pub template_name: Option<String>,
}

#[derive(Args)]
pub struct ChangeTaskStatus {
  #[clap(name = "Note (task) ID")]
  pub id: u32,

  #[clap(name = "New status")]
  pub status: String,
}

#[derive(Args)]
pub struct Search {
  #[clap(name = "Text to search")]
  pub text: String,

  // TODO: Note that this flag makes the command description (term_keep search -h) a bit strange.
  //       A more generic way would be "Find notes. By default it does a fuzzy text search".
  //       Then this flag should change to "Search by tag name instead of note content" or something like that,
  //       although the tags are also part of the content, so be careful with the wording.
  #[clap(long = "tag", short = 't', help = "Search by tag")]
  pub tag_name: bool,
}

#[derive(Args)]
pub struct UpsertTemplate {
  #[clap(name = "Template name")]
  pub template_name: String,
}

#[derive(Args)]
pub struct RemoveTemplate {
  #[clap(name = "Template name")]
  pub template_name: String,
}

#[derive(Args)]
pub struct RemoveNote {
  #[clap(name = "Note ID")]
  pub id: u32,
}

#[derive(Args)]
pub struct ShowTags {
  #[clap(
    help = "Normalize tag names to lowercase",
    long = "lowercase",
    short = 'l'
  )]
  pub lowercase: bool,
}

#[derive(Subcommand)]
pub enum Command {
  #[command(name = "all", about = "Show all notes")]
  ShowAllNotes(ShowAllNotes),

  #[command(about = "Show one note")]
  Show(ShowOne),

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

  #[command(about = "Archive all completed tasks")]
  ArchiveAllDone,

  #[command(name = "rm", about = "Remove note permanently")]
  RemoveNote(RemoveNote),

  #[command(name = "tags", about = "Show all tags")]
  ShowTags(ShowTags),

  #[command(about = "Show miscellaneous information")]
  Info,
}
