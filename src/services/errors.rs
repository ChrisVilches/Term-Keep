use crate::models::traits::ModelName;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotFoundByIdError {
  id: u32,
  model_name: String,
}

#[derive(Debug, Clone)]
pub struct NotFoundByFieldError {
  model_name: String,
  field: String,
  value: String,
}

// TODO: Is this how constructors are supposed to be made?
impl NotFoundByIdError {
  pub fn new<T: ModelName>(id: u32) -> Self {
    NotFoundByIdError {
      id,
      model_name: T::model_name(),
    }
  }
}

// TODO: Is this how constructors are supposed to be made?
impl NotFoundByFieldError {
  pub fn new<T: ModelName>(field: String, value: String) -> Self {
    NotFoundByFieldError {
      model_name: T::model_name(),
      field,
      value,
    }
  }
}

impl Error for NotFoundByIdError {}
impl Error for NotFoundByFieldError {}

impl fmt::Display for NotFoundByIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} not found with ID = {}", self.model_name, self.id)
  }
}

impl fmt::Display for NotFoundByFieldError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} not found with {} = {}",
      self.model_name, self.field, self.value
    )
  }
}
