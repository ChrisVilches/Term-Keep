#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

mod cli;
mod controllers;
mod errors;
mod models;
mod services;
mod util;
use crate::models::note::Note;
use crate::models::task_status::TaskStatus;
use crate::services::tips;
use crate::util::cli::abort_with_message;

fn main() {
  services::db::install_database().unwrap_or_else(|e| abort_with_message(e));
  cli::parser::execute();
}
