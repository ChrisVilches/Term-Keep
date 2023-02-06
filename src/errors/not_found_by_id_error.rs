use crate::models::traits::ModelName;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotFoundByIdError {
  id: u32,
  model_name: String,
}

impl NotFoundByIdError {
  #[must_use]
  pub fn new<T: ModelName>(id: u32) -> Self {
    Self {
      id,
      model_name: T::model_name(),
    }
  }
}

impl Error for NotFoundByIdError {}

impl fmt::Display for NotFoundByIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} not found with ID = {}", self.model_name, self.id)
  }
}
