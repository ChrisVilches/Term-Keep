use crate::config;

pub fn info() {
  println!(
    "Database location: {}",
    config::env::get_string_env_var("DB_PATH")
  );

  // TODO: Add which editor is going to be used.
}
