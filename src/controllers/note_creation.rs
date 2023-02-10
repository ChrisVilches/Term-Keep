use crate::services;
use crate::util::cli::get_text_input;
use crate::util::cli::{self, validate_text_input_mode};
use crate::util::note_fmt;
use std::error::Error;

fn template_text(template_name: &Option<String>) -> Result<String, Box<dyn Error>> {
  Ok(match template_name {
    Some(name) => services::templates::find_one(name)?.content,
    None => String::new(),
  })
}

fn create(template_name: &Option<String>, task: bool) -> Result<(), Box<dyn Error>> {
  let (content, mode) = get_text_input(&template_text(template_name)?)?;

  validate_text_input_mode(mode, template_name.is_some())?;

  if content.trim().is_empty() {
    println!("{}", cli::color_secondary("Not saved"));
    return Ok(());
  }

  if task {
    services::notes::create_task(&content)?;
  } else {
    services::notes::create_note(&content)?;
  }

  note_fmt::print_note(&services::notes::find_latest().unwrap(), false);

  println!();

  println!("{}", cli::color_primary("Created"));
  Ok(())
}

pub fn create_note(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, false)
}

pub fn create_task(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, true)
}
