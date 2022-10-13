CREATE TABLE IF NOT EXISTS note (
  id          INTEGER PRIMARY KEY,
  content     TEXT NOT NULL,
  task_status INTEGER,
  archived    BOOLEAN NOT NULL DEFAULT false,
  pinned      BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS template (
  id      INTEGER PRIMARY KEY,
  name    VARCHAR(255) NOT NULL,
  content TEXT NOT NULL
);
