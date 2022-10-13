use crate::models::template::Template;
use crate::services;
use colored::*;
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

  println!("{}", content);
  println!();

  if task {
    services::notes::create_task(content)?;
  } else {
    services::notes::create_note(content)?;
  }

  // TODO: I think it'd be better if the create/update methods
  // showed the result after creating using the same logic as the "show_one",
  // because that one shows a lot of data with a nice format.

  println!("{}", "Created".blue());
  Ok(())
}

pub fn create_note(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, false)
}

pub fn create_task(template_name: &Option<String>) -> Result<(), Box<dyn Error>> {
  create(template_name, true)
}
