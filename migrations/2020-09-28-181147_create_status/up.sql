-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users_status" (
  "id" UUID PRIMARY KEY,
  "user_id" UUID NOT NULL,
  "status" INT NOT NULL,
  "message" TEXT,
  "discord_invite" TEXT
);