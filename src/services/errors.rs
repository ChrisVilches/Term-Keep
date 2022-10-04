use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotFoundByIdError {
  pub id: u32,
  pub type_name: String,
}

#[derive(Debug, Clone)]
pub struct NotFoundByFieldError {
  pub type_name: String,
  pub field: String,
  pub value: String,
}

impl Error for NotFoundByIdError {}
impl Error for NotFoundByFieldError {}

impl fmt::Display for NotFoundByIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} not found with ID = {}", self.type_name, self.id)
  }
}

impl fmt::Display for NotFoundByFieldError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} not found with {} = {}",
      self.type_name, self.field, self.value
    )
  }
}
