use crate::models::template::Template;
use crate::services;
use colored::*;

fn template_text(template_name: &Option<String>) -> String {
  match template_name {
    Some(t) => {
      let template: Template = services::templates::find_one_template(t).unwrap();
      template.content
    }
    None => "New note".to_string(),
  }
}

fn create(template_name: &Option<String>, task: bool) {
  // TODO: Should not create if the user closes the editor without saving. Is it possible?
  let content = edit::edit(template_text(&template_name)).unwrap();

  println!("{}", content);

  // TODO: Should handle error (show message). This applies to most services as well.

  if task {
    services::notes::create_task(content).unwrap();
  } else {
    services::notes::create_note(content).unwrap();
  }

  // TODO: I think it'd be better if the create/update methods
  // showed the result after creating using the same logic as the "show_one",
  // because that one shows a lot of data with a nice format.

  println!("{}", "Created".blue());
}

pub fn create_note(template_name: &Option<String>) {
  create(template_name, false);
}

pub fn create_task(template_name: &Option<String>) {
  create(template_name, true);
}
