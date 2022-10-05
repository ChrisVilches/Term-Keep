use crate::services::notes;
use crate::tips::random_tip;
use crate::util::env;
use crate::util::strings::bool_to_str;
use colored::*;
use std::error::Error;

fn print_item(label: &str, value: String) {
  println!("{} {}", label.bold().to_string() + ":", value);
}

fn get_editor_name() -> Result<String, Box<dyn Error>> {
  Ok(edit::get_editor()?.display().to_string())
}

pub fn info() -> Result<(), Box<dyn Error>> {
  print_item("Database location", env::require_string_env_var("DB_PATH"));

  let can_read_tips = match random_tip() {
    Some(_) => true,
    None => false,
  };

  // For debugging.
  print_item("Can read tips", bool_to_str(can_read_tips));

  print_item("Editor", get_editor_name()?);
  print_item("Notes", notes::find_all(false)?.len().to_string());
  print_item("Archived", notes::find_all(true)?.len().to_string());

  Ok(())
}
