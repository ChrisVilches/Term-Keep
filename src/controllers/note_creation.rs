use crate::models::template::Template;
use crate::services;
use crate::util::note_fmt;
use colored::Colorize;
use std::error::Error;

fn template_text(template_name: &Option<String>) -> String {
  match template_name {
    Some(t) => {
      let template: Template = services::templates::find_one(t).unwrap_or_default();
      template.content
    }
    None => "".to_string(),
  }
}

fn create(template_name: &Option<String>, task: bool) -> Result<(), Box<dyn Error>> {
  let content = edit::edit(template_text(template_name))?;

  if content.trim().is_empty() {
    println!("Not saved");
    return Ok(());
  }

  if task {
    services::notes::create_task(&content)?;
  } else {
    services::notes::create_note(&content)?;
  }

  note_fmt::print_note(&services::notes::find_latest().unwrap());

  println!("{}", "Created".blue());
  Ok(())
}

pub fn create_note(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, false)
}

pub fn create_task(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, true)
}
