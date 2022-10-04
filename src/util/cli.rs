use crate::models::note::Note;
use crate::services;
use crate::services::errors::NotFoundError;
use colored::*;
use std::fmt::Display;

pub fn abort_with_message<S: Display>(msg: S) -> ! {
  eprintln!("{}", format!("Error: {}", msg).red().bold());
  std::process::exit(1);
}

// TODO: I think this function doesn't belong to this module, specially if I restructure
//       everything so that the commands are not necessarily coupled to the CLI.

/// Fetches a note and prints a CLI friendly error message if it's not found.
pub fn require_note(id: u32) -> Note {
  match services::notes::find_one_note(id) {
    Some(note) => note,
    None => abort_with_message(NotFoundError {
      id,
      type_name: "note".to_string(),
    }),
  }
}
