ALTER TABLE thread
RENAME video_url TO youtube_id;

ALTER TABLE thread
ALTER COLUMN youtube_id
TYPE VARCHAR(11);
