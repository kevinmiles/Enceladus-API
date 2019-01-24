CREATE TABLE thread (
  id SERIAL PRIMARY KEY NOT NULL,
  thread_name VARCHAR(255) NOT NULL,
  launch_name VARCHAR(255) NOT NULL,
  post_id VARCHAR(16),
  subreddit VARCHAR(255) NOT NULL,
  t0 BIGINT,
  youtube_id VARCHAR(11),
  spacex__api_id VARCHAR(255),
  created_by_user_id INTEGER NOT NULL,
  sections_id INTEGER[] NOT NULL DEFAULT '{}',
  events_id INTEGER[] NOT NULL DEFAULT '{}'
);
