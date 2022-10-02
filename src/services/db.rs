use rusqlite::Connection;

// TODO: Should be singleton.
pub fn connection() -> rusqlite::Connection {
  Connection::open("./test.db").unwrap()
}

// Should be idempotent.
// ^ or just execute it once and with "if not exists"
//
// Should be in a different file. This does not belong here.
pub fn install_database() -> Result<usize, rusqlite::Error> {
  connection().execute(
    "CREATE TABLE IF NOT EXISTS note (
            id          INTEGER PRIMARY KEY,
            content     TEXT NOT NULL,
            task_status INTEGER,
            archived    BOOLEAN NOT NULL DEFAULT false, 
            pinned      BOOLEAN NOT NULL DEFAULT false
        )",
    (),
  )
}
