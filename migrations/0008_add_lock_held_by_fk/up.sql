ALTER TABLE section
ADD CONSTRAINT section_lock_held_by_id
FOREIGN KEY (lock_held_by_user_id)
REFERENCES "user"
ON DELETE RESTRICT;
