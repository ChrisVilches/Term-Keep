use crate::models::template::Template;
use crate::services;
use colored::*;
use std::error::Error;

const DEFAULT_NOTE_CONTENT: &str = "New note";
const DEFAULT_TASK_CONTENT: &str = "New task";

fn template_text(template_name: &Option<String>, task: bool) -> String {
  match template_name {
    Some(t) => {
      let template: Template = services::templates::find_one_template(t).unwrap_or_default();
      template.content
    }
    None => if task {
      DEFAULT_TASK_CONTENT
    } else {
      DEFAULT_NOTE_CONTENT
    }
    .to_string(),
  }
}

fn create(template_name: &Option<String>, task: bool) -> Result<(), Box<dyn Error>> {
  // TODO: Should not create if the user closes the editor without saving. Is it possible?
  let content = edit::edit(template_text(&template_name, task))?;

  println!("{}", content);

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
