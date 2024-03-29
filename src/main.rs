#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_unrelated)]

use term_keep::cli;
use term_keep::services;
use term_keep::util::cli::abort_with_message;
use term_keep::util::env;

fn main() {
  let db_path: String = env::require_string_env_var("DB_PATH");
  services::db::set_database(&db_path).unwrap_or_else(|e| abort_with_message(e));
  cli::parser::execute();
}
