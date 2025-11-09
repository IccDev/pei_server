-- Your SQL goes here
CREATE TABLE sections (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  comment TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now()
)