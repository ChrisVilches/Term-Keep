use crate::models::template::Template;
use crate::services;
use colored::*;
use std::error::Error;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;

const DEFAULT_NOTE_CONTENT: &str = "New note";
const DEFAULT_TASK_CONTENT: &str = "New task";

fn template_text(template_name: &Option<String>, task: bool) -> String {
  match template_name {
    Some(t) => {
      let template: Template = services::templates::find_one(t).unwrap_or_default();
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

// TODO: Should work without the user having to press ENTER key.
// TODO: When the user presses CTRL+D the output gets a bit weird. Maybe I can easily
//       fix this by adding a new line after the prompt message (instead of doing it without
//       new line and flushing manually).
fn prompt_confirm(msg: &str) -> bool {
  print!("{}", msg);
  stdout().flush().unwrap();

  let mut buff: [u8; 1] = [0];
  stdin().read(&mut buff).unwrap();
  let c = buff[0] as char;

  !(c == 'n' || c == 'N')
}

fn create(template_name: &Option<String>, task: bool) -> Result<(), Box<dyn Error>> {
  let content = edit::edit(template_text(&template_name, task))?;

  println!("{}", content);
  println!();

  if !prompt_confirm("Press any key to continue...") {
    println!("Not saved");
    return Ok(());
  }

  if task {
    services::notes::create_task(content);
  } else {
    services::notes::create_note(content);
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
