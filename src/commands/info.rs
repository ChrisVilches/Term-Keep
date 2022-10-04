use crate::config;
use crate::services::notes::find_all_notes;
use crate::tips::random_tip;
use crate::util::strings::bool_to_str;
use colored::*;

fn print_item(label: &str, value: String) {
  println!("{} {}", label.bold().to_string() + ":", value);
}

fn get_editor_name() -> String {
  edit::get_editor().unwrap().display().to_string()
}

pub fn info() {
  print_item(
    "Database location",
    config::env::require_string_env_var("DB_PATH"),
  );

  let can_read_tips = match random_tip() {
    Some(_) => true,
    None => false,
  };

  // For debugging.
  print_item("Can read tips", bool_to_str(can_read_tips));

  print_item("Editor", get_editor_name());
  print_item("Notes", find_all_notes(false).unwrap().len().to_string());
  print_item("Archived", find_all_notes(true).unwrap().len().to_string());
}
