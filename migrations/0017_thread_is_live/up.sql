ALTER TABLE thread
ADD COLUMN is_live BOOLEAN
NOT NULL
DEFAULT false;
