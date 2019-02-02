ALTER TABLE thread
ADD CONSTRAINT thread_created_by_fk
FOREIGN KEY (created_by_user_id)
REFERENCES "user"
ON DELETE RESTRICT;
