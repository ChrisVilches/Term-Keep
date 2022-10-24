CREATE TABLE IF NOT EXISTS note (
  id INTEGER PRIMARY KEY,
  content TEXT NOT NULL,
  task_status INTEGER,
  archived BOOLEAN NOT NULL DEFAULT false,
  pinned BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime')),
  updated_at TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime'))
);

CREATE TABLE IF NOT EXISTS template (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime')),
  updated_at TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime'))
);

CREATE TRIGGER IF NOT EXISTS trigger_note_updated_at
AFTER
UPDATE
  ON note BEGIN
UPDATE
  note
SET
  updated_at = DATETIME('now', 'localtime')
WHERE
  rowid == NEW.rowid;

END;

CREATE TRIGGER IF NOT EXISTS trigger_template_updated_at
AFTER
UPDATE
  ON template BEGIN
UPDATE
  template
SET
  updated_at = DATETIME('now', 'localtime')
WHERE
  rowid == NEW.rowid;

END;
