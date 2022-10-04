mod cli;
mod commands;
mod config;
mod helpers;
mod models;
mod services;
mod util;
use crate::cli::create_cli;
use crate::models::note::Note;
use crate::models::task_status::TaskStatus;
use crate::services::tips;
use crate::util::cli::abort_with_message;
use crate::util::files::lines_from_file;
use colored::*;

const LOGO: &str = "
████████╗██╗░░██╗
╚══██╔══╝██║░██╔╝
░░░██║░░░█████═╝░
░░░██║░░░██╔═██╗░
░░░██║░░░██║░╚██╗
░░░╚═╝░░░╚═╝░░╚═╝";

fn show_random_tip() {
  if let Some(t) = tips::random_tip() {
    println!();
    println!("💡 Tip: {}", t);
  }
}

// TODO: Clean all unwraps (since they can panic).

fn main() {
  match services::db::install_database() {
    Ok(_) => {}
    Err(e) => abort_with_message(format!("Couldn't install database\n{}", e)),
  };

  println!("{}", LOGO.green());
  println!();

  create_cli();
  show_random_tip();
}
