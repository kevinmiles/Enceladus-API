ALTER TABLE "user"
ALTER COLUMN refresh_token
TYPE BYTEA
USING refresh_token::BYTEA;

ALTER TABLE "user"
ALTER COLUMN access_token
TYPE BYTEA
USING access_token::BYTEA;
