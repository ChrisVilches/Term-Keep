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

fn main() {
  services::db::install_database().unwrap();
  create_cli();
}
