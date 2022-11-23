use crate::models::template::Template;
use crate::models::traits::RequireId;
use crate::services;
use crate::util::cli;
use colored::Colorize;
use std::error::Error;

pub fn show_all() {
  let templates = services::templates::find_all();

  println!("{} template(s)", templates.len().to_string().bold());
  println!();

  for template in templates {
    println!("{}", template.name);
  }
}

fn edit(template: &Template) -> Result<(), Box<dyn Error>> {
  let content = edit::edit(&template.content)?;

  if content == template.content {
    println!("{}", cli::color_secondary("Not changed"));
  } else {
    services::templates::update(template.require_id(), &content)?;
  }
  Ok(())
}

fn create(name: &str) -> Result<(), Box<dyn Error>> {
  let content = edit::edit("")?;
  services::templates::create(name, &content)?;
  println!("{}", cli::color_primary("Created a new template"));
  Ok(())
}

pub fn upsert(name: &str) -> Result<(), Box<dyn Error>> {
  let template = services::templates::find_one(name);

  template.map_or_else(|_| create(name), |t| edit(&t))
}

pub fn remove(name: &str) -> Result<(), Box<dyn Error>> {
  let template = services::templates::find_one(name)?;
  services::templates::remove(template.require_id())?;
  Ok(())
}
