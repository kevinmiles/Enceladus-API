ALTER TABLE thread
ADD COLUMN event_column_headers TEXT[] NOT NULL DEFAULT '{}';

ALTER TABLE event
DROP COLUMN message;

ALTER TABLE event
DROP COLUMN terminal_count;

ALTER TABLE event
DROP COLUMN utc;

ALTER TABLE event
ADD COLUMN cols JSONB NOT NULL DEFAULT '[]'::json;

-- With this metadata,
-- we can format an event as the proper timestamp
-- (both on Reddit and in the browser).
ALTER TABLE thread
ADD COLUMN space__utc_col_index SMALLINT;
