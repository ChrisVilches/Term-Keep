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

  let result: String = value.unwrap_or_default().trim().to_string();

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
