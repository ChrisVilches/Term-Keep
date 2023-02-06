use crate::services::tips;
use colored::Colorize;
use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn show_random_tip() {
  if let Some(t) = tips::random_tip() {
    println!();
    println!("💡 Tip: {}", t);
  }
}

pub fn abort_with_message<S: Display>(msg: S) -> ! {
  eprintln!("{}", color_danger(&format!("Error: {}", msg)));
  std::process::exit(1);
}

fn less_aux(text: &str) -> Result<(), Box<dyn Error>> {
  let mut child = Command::new("less")
    .args(["-R"])
    .stdin(Stdio::piped())
    .spawn()?;

  match child.stdin.take() {
    None => return Err("cannot open stdin".into()),
    Some(mut s) => std::thread::spawn({
      let t = text.to_owned();
      move || {
        s.write_all(t.as_bytes()).expect("cannot write to stdin");
      }
    }),
  };

  child.wait()?;
  Ok(())
}

#[must_use]
pub fn color_primary(text: &str) -> String {
  text.blue().to_string()
}

#[must_use]
pub fn color_secondary(text: &str) -> String {
  text.dimmed().to_string()
}

#[must_use]
pub fn color_danger(text: &str) -> String {
  text.red().bold().to_string()
}

pub fn less(text: &str) {
  less_aux(text).unwrap_or_else(|e| abort_with_message(format!("Couldn't use 'less' ({})", e)));
}
