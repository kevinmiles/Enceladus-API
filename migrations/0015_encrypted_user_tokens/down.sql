ALTER TABLE "user"
ALTER COLUMN refresh_token
TYPE TEXT
USING refresh_token::TEXT;

ALTER TABLE "user"
ALTER COLUMN access_token
TYPE VARCHAR(255)
USING access_token::VARCHAR(255);
