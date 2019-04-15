ALTER TABLE thread
ALTER COLUMN youtube_id
TYPE VARCHAR(255);

ALTER TABLE thread
RENAME youtube_id TO video_url;
