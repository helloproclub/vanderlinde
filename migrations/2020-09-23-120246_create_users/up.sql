-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users" (
  "id" UUID PRIMARY KEY,
  "email" VARCHAR(255) UNIQUE NOT NULL,
  "nim" VARCHAR(10) UNIQUE NOT NULL,
  "name" VARCHAR(255) NOT NULL,
  "password_hash" TEXT NOT NULL,
  "ktm_url" TEXT NOT NULL,
  "cv_url" TEXT NOT NULL,
  "letter_url" TEXT NOT NULL,
  "linkedin_url" TEXT NOT NULL
);