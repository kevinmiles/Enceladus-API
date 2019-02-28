ALTER TABLE "user"
ADD COLUMN access_token VARCHAR(255) NOT NULL;

ALTER TABLE "user"
ADD COLUMN access_token_expires_at_utc BIGINT NOT NULL;
