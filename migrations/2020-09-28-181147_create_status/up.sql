-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users_status" (
  "id" UUID PRIMARY KEY,
  "user_id" UUID,
  "status" TEXT NOT NULL,
  "message" TEXT,
  "discord_invite" TEXT
);