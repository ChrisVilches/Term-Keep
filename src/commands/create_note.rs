use crate::services;
use colored::*;

// TODO: This flag parameter should be named when calling... not sure how to do it.
// pseudocode create_note({ task: true })
pub fn create_note(task: bool) {
  let template = "New note";
  let content = edit::edit(template).unwrap();

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
