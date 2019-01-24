CREATE TABLE "user" (
  id SERIAL PRIMARY KEY NOT NULL,
  reddit_username TEXT UNIQUE NOT NULL,
  lang VARCHAR(2) NOT NULL DEFAULT 'en',
  refresh_token TEXT NOT NULL,
  is_global_admin BOOLEAN NOT NULL DEFAULT false,
  spacex__is_admin BOOLEAN NOT NULL DEFAULT false,
  spacex__is_mod BOOLEAN NOT NULL DEFAULT false,
  spacex__is_slack_member BOOLEAN NOT NULL DEFAULT false
);
