use colored::Colorize;
use std::fmt::Display;

pub fn abort_with_message<S: Display>(msg: S) -> ! {
  eprintln!("{}", format!("Error: {}", msg).red().bold());
  std::process::exit(1);
}
