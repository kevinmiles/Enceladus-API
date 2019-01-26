CREATE TABLE section (
  id SERIAL PRIMARY KEY NOT NULL,
  is_events_section BOOLEAN NOT NULL DEFAULT false,
  name VARCHAR(255) NOT NULL DEFAULT '',
  content TEXT NOT NULL DEFAULT '',
  lock_held_by_user_id INTEGER,
  in_thread_id INTEGER NOT NULL
);
