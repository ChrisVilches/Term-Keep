use crate::models::template::Template;
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
    services::templates::update(template.id.unwrap(), &content)?;
  }
  Ok(())
}

fn create(name: &String) -> Result<(), Box<dyn Error>> {
  let content = edit::edit("")?;
  services::templates::create(name, &content)?;
  println!("{}", cli::color_primary("Created a new template"));
  Ok(())
}

pub fn upsert(name: &String) -> Result<(), Box<dyn Error>> {
  let template = services::templates::find_one(name);

  match template {
    Ok(t) => edit(&t),
    Err(_) => create(name),
  }
}

pub fn remove(name: &String) -> Result<(), Box<dyn Error>> {
  let template = services::templates::find_one(name)?;
  services::templates::remove(template.id.unwrap())?;
  Ok(())
}
