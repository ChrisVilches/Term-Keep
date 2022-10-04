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

// TODO: This flag parameter should be named when calling... not sure how to do it.
// pseudocode create_note({ task: true })
pub fn create_note(task: bool, template_name: &Option<String>) {
  // TODO: Should not create if the user closes the editor without saving. Is it possible?
  let content = edit::edit(template_text(&template_name)).unwrap();

  println!("{}", content);

  // TODO: Should handle error (show message). This applies to most services as well.
  // Also right now this doesn't throw a "Result", so the error cannot be handled here.
  // EDIT: The error is now thrown (for most services), that's why I'm using unwrap.

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
