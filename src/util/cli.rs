use colored::Colorize;
use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn abort_with_message<S: Display>(msg: S) -> ! {
  eprintln!("{}", format!("Error: {}", msg).red().bold());
  std::process::exit(1);
}

fn less_aux(text: &String) -> Result<(), Box<dyn Error>> {
  let mut child = Command::new("less").stdin(Stdio::piped()).spawn()?;

  match child.stdin.take() {
    None => Err("cannot open stdin")?,
    Some(mut s) => std::thread::spawn({
      let t = text.to_string();
      move || {
        s.write_all(t.as_bytes()).expect("cannot write to stdin");
      }
    }),
  };

  child.wait()?;
  Ok(())
}

pub fn color_primary(text: &str) -> String {
  text.blue().to_string()
}

pub fn color_secondary(text: &str) -> String {
  text.dimmed().to_string()
}

pub fn less(text: &String) {
  less_aux(text).unwrap_or_else(|e| abort_with_message(format!("Couldn't use 'less' ({})", e)));
}
