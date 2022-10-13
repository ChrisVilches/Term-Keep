#![deny(clippy::all, clippy::pedantic)]

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
use colored::Colorize;

static LOGO: &str = include_str!("../data/logo.txt");

fn show_random_tip() {
  if let Some(t) = tips::random_tip() {
    println!();
    println!("💡 Tip: {}", t);
  }
}

fn main() {
  services::db::install_database().unwrap_or_else(|e| abort_with_message(e));

  println!("{}", LOGO.trim().green());
  println!();

  cli::parser::create();
  show_random_tip();
}
