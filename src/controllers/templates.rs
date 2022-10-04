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
