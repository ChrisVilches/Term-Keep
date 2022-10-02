use colored::*;
use crate::services::notes;

pub fn create_note(task: bool) {
  let template = "New note";
  let content = edit::edit(template).unwrap();
  println!("{}", content);

  // TODO: Should handle error (show message). This applies to most services as well.
  // Also right now this doesn't throw a "Result", so the error cannot be handled here.

  if task {
    notes::create_task(content);
  } else {
    notes::create_note(content);
  }

  // TODO: I think it'd be better if the create/update methods
  // showed the result after creating using the same logic as the "show_one",
  // because that one shows a lot of data with a nice format.

  println!("{}", "Created".blue());

}
