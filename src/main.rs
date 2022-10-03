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
use crate::util::files::lines_from_file;

fn show_random_tip() {
  match tips::random_tip() {
    Some(t) => {
      println!();
      println!("💡 Tip: {}", t);
    }
    None => {}
  }
}

fn main() {
  services::db::install_database().unwrap();
  create_cli();
  show_random_tip();
}
