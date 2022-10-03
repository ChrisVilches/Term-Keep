use std::env;

const ENV_VAR_PREFIX: &str = "TERM_KEEP_";

fn prefixed_env_var(name: &str) -> String {
  return format!("{}{}", ENV_VAR_PREFIX, name);
}

pub fn get_string_env_var(name: &str) -> String {
  let var_name: String = prefixed_env_var(name);
  let value = env::var(&var_name);

  let result: String = value.unwrap_or_default().trim().to_string();

  match result.len() {
    0 => panic!(
      "{}",
      format!("{} (environment variable) must be set.", &var_name)
    ),
    _ => result,
  }
}
