use std::error::Error;
use std::fmt;

// TODO: Only works for when the query uses the ID. Not for "name", for example when searching templates.
//       Maybe could change the name to "not found by ID error", or pass the query object (where conditions)
//       and then print that as an error message, but that's difficult.
#[derive(Debug, Clone)]
pub struct NotFoundError {
  pub id: u32,
  pub type_name: String,
}

impl Error for NotFoundError {}

impl fmt::Display for NotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} not found with ID = {}", self.type_name, self.id)
  }
}
