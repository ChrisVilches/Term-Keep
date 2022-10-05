use crate::models::template::Template;
use crate::services;
use colored::*;
use std::error::Error;

pub fn show_all() -> Result<(), Box<dyn Error>> {
  let templates = services::templates::find_all_templates()?;

  println!("{} template(s)", templates.len().to_string().bold());
  println!();

  for template in templates {
    println!("{}", template.name);
  }
  Ok(())
}

fn edit(template: Template) -> Result<(), Box<dyn Error>> {
  let content = edit::edit(template.content.to_string())?;

  if content == template.content {
    println!("Not changed");
  } else {
    services::templates::update(template.id.unwrap(), &content)?;
  }
  Ok(())
}

fn create(name: &String) -> Result<(), Box<dyn Error>> {
  let content = edit::edit("")?;
  services::templates::create(name, &content)?;
  println!("Created a new template");
  Ok(())
}

pub fn upsert(name: &String) -> Result<(), Box<dyn Error>> {
  // TODO: Does this work? It should, because it compiled, and this is Rust.
  //       Update: Even if it works, it discards the error, so, should I use this?
  //       Update: I already commented this function with a TODO. It should return Result of Option,
  //               plus contain a SQL error (if it happens).
  //
  // This is a bit tricky to solve. I think the only way is to see which error was returned,
  // and if it was "Not Found By Field", then dispatch a "create".
  //
  // One alternative is to use the "anyhow" library, or simply change the return to:
  // find_one_template -> Result<Option<Template>, rusqlite::Error>
  // Then handle the error (propagate using ?), and then, once you get the optional,
  // use the "match" as below.
  //
  // find_one_note(note_id)?; is used in many places (it unwraps the Result, not expecting an Optional
  // to be inside), so it'd be a bit cumbersome to change all of that.
  //
  // So try using "anyhow" (and migrating every Box<dyn Error> to "anyhow" if necessary).
  let template = services::templates::find_one_template(&name).ok();

  match template {
    Some(t) => edit(t),
    None => create(&name),
  }
}

pub fn remove(name: &String) -> Result<(), Box<dyn Error>> {
  let template = services::templates::find_one_template(&name)?;
  services::templates::remove(template.id.unwrap())
    .map(|_| ())
    .map_err(|e| e.into())
}
