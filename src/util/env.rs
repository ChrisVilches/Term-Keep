use crate::abort_with_message;
use std::env;
use std::str::FromStr;

const ENV_VAR_PREFIX: &str = "TERM_KEEP_";

fn prefixed_env_var(name: &str) -> String {
  format!("{}{}", ENV_VAR_PREFIX, name)
}

pub fn require_string_env_var(name: &str) -> String {
  let var_name: String = prefixed_env_var(name);
  let value = env::var(&var_name);

  let result: String = value.unwrap_or_default().trim().to_owned();

  if result.is_empty() {
    abort_with_message(format!("{} (environment variable) must be set.", &var_name));
  }

  result
}

pub fn get_env_var<T: FromStr>(name: &str) -> Result<T, <T as std::str::FromStr>::Err> {
  let var_name: String = prefixed_env_var(name);
  let value = env::var(&var_name);

  value.unwrap_or_default().trim().parse::<T>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_require_string_env_var() {
    env::set_var("TERM_KEEP_NEW_VARIABLE", "some value");
    assert_eq!(require_string_env_var("NEW_VARIABLE"), "some value");
  }

  #[test]
  fn test_prefixed_env_var() {
    assert_eq!(prefixed_env_var("MY_VAR"), "TERM_KEEP_MY_VAR");
  }

  #[test]
  fn test_get_env_var() {
    env::set_var("TERM_KEEP_NUM", "123");
    assert_eq!(get_env_var::<String>("NUM"), Ok("123".to_owned()));
    assert_eq!(get_env_var::<i32>("NUM"), Ok(123));

    assert_eq!(get_env_var::<String>("INVALID_VARIABLE"), Ok(String::new()));
    assert!(get_env_var::<i32>("INVALID_VARIABLE").is_err());
  }
}
