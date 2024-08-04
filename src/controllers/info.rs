use crate::services::notes;
use crate::services::tips::random_tip;
use crate::util::cli::hide_logo;
use crate::util::env;
use crate::util::strings::bool_to_str;
use anyhow::Result;
use colored::Colorize;

fn print_item(label: &str, value: &str) {
  println!("{} {}", label.bold().to_string() + ":", value);
}

fn get_editor_name() -> Result<String> {
  Ok(edit::get_editor()?.display().to_string())
}

pub fn info() -> Result<()> {
  print_item("Database location", &env::require_string_env_var("DB_PATH"));

  let can_read_tips = random_tip().is_some();

  // For debugging.
  print_item("Can read tips", bool_to_str(can_read_tips));
  print_item("Hide logo", bool_to_str(hide_logo()));

  print_item("Editor", &get_editor_name()?);
  print_item("Notes", &notes::find_all(false).len().to_string());
  print_item("Archived", &notes::find_all(true).len().to_string());

  Ok(())
}
