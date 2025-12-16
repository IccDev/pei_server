-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  identifier TEXT NOT NULL,
  last_name VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  age INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now()
)