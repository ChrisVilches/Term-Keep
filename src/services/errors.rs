use std::fmt;

#[derive(Debug, Clone)]
pub struct NotFoundError {
  pub id: u32,
}

// TODO: Confirm (I think it's implemented properly)
// A custom error should look like this. Just run the command "pin 6666" to test it.
// I think it's working OK now.
// f64::from_str("asd54").unwrap();
// 'called `Result::unwrap()` on an `Err` value: ParseFloatError { kind: Invalid }'

impl fmt::Display for NotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "not found with ID = {}", self.id)
  }
}
