use crate::config;
use rusqlite::Connection;

// TODO: Should be singleton.
pub fn connection() -> rusqlite::Connection {
  let db_path = config::env::get_string_env_var("DB_PATH");
  Connection::open(db_path).unwrap()
}

// Should be idempotent.
// ^ or just execute it once and with "if not exists"
//
// Should be in a different file. This does not belong here.
pub fn install_database() -> Result<(), rusqlite::Error> {
  connection().execute(
    "
  CREATE TABLE IF NOT EXISTS note (
    id          INTEGER PRIMARY KEY,
    content     TEXT NOT NULL,
    task_status INTEGER,
    archived    BOOLEAN NOT NULL DEFAULT false, 
    pinned      BOOLEAN NOT NULL DEFAULT false
  );",
    (),
  )?;

  connection().execute(
    "
  CREATE TABLE IF NOT EXISTS template (
    id      INTEGER PRIMARY KEY,
    name    VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
  );",
    (),
  )?;

  Ok(())
}
