use crate::models::traits::ModelName;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RowNotChangedError {
  model_name: String,
}

impl RowNotChangedError {
  pub fn new<T: ModelName>() -> Self {
    Self {
      model_name: T::model_name(),
    }
  }
}

impl Error for RowNotChangedError {}

impl fmt::Display for RowNotChangedError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "expected to change a {}, but nothing happened",
      self.model_name
    )
  }
}
