PRAGMA foreign_keys = OFF;

CREATE TABLE IF NOT EXISTS work_type (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  type TEXT NOT NULL UNIQUE -- remote | onsite | hybrid
);

CREATE TABLE IF NOT EXISTS status_type (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  type TEXT NOT NULL UNIQUE -- pending | in_progress | failed | completed
);

-- Add migration script here
CREATE TABLE IF NOT EXISTS jobs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  idx INTEGER NOT NULL,
  card_title TEXT NOT NULL,
  job_title TEXT NOT NULL,
  company_name TEXT NOT NULL,
  state TEXT NOT NULL,
  country TEXT NOT NULL,
  -- work_type TEXT NOT NULL, -- USE FK table of db.work_type 
  work_type_fk INTEGER NOT NULL,
  is_actively_reviewing BOOLEAN NOT NULL,
  already_viewed BOOLEAN NOT NULL,
  full_date TEXT NOT NULL, -- maybe we serialize as an actual date type later
  relative_date TEXT NOT NULL,
  has_easy_apply BOOLEAN NOT NULL,
  -- | pending | in_progress | failed | completed |
  -- status TEXT NOT NULL DEFAULT 'pending' -- USE FK table of db.job_status
  applied_date DATETIME DEFAULT CURRENT_TIMESTAMP,
  status_type_fk INTEGER NOT NULL,
  FOREIGN KEY (work_type_fk) REFERENCES work_type (id),
  FOREIGN KEY (status_type_fk) REFERENCES status_type (id)
);

INSERT INTO
  work_type (type)
VALUES
  ('remote'),
  ('on-site'),
  ('hybrid');

INSERT INTO
  status_type (type)
VALUES
  ('pending'),
  ('in_progress'),
  ('completed'),
  ('failed');

CREATE TABLE IF NOT EXISTS autofill (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  input_key TEXT NOT NULL UNIQUE, -- reprs. the title of input field. We use this to identify what the field is/was
  data TEXT NOT NULL -- serialized JSON data representing autofill info
);

PRAGMA foreign_keys = ON;
