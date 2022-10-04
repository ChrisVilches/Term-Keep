use crate::models::template::Template;
use crate::services;
use crate::util::cli::abort_with_message;
use colored::*;

pub fn show_all() {
  let templates = services::templates::find_all_templates().unwrap();

  println!("{} template(s)", templates.len().to_string().bold());
  println!();

  for template in templates {
    println!("{}", template.name);
  }
}

fn edit(template: Template) {
  let content = edit::edit(template.content.to_string()).unwrap();

  if content == template.content {
    println!("Not changed");
  } else {
    services::templates::update(template.id.unwrap(), &content).unwrap();
  }
}

fn create(name: &String) {
  let content = edit::edit("").unwrap();
  services::templates::create(name, &content).unwrap();
  println!("Created a new template");
}

pub fn upsert(name: String) {
  let template = services::templates::find_one_template(&name);

  match template {
    Some(t) => edit(t),
    None => create(&name),
  };
}

pub fn remove(name: String) {
  let template = services::templates::find_one_template(&name);

  match template {
    Some(t) => {
      services::templates::remove(t.id.unwrap()).unwrap();
    }
    None => {
      abort_with_message(format!("Template with name '{}' not found", name));
    }
  };
}
