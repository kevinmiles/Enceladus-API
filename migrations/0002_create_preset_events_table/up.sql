CREATE TABLE preset_event (
  id SERIAL PRIMARY KEY NOT NULL,
  holds_clock BOOLEAN NOT NULL DEFAULT false,
  message TEXT NOT NULL,
  name VARCHAR(255) NOT NULL
);
