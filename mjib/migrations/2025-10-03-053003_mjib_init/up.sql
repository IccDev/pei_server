-- Your SQL goes here
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  create_at TIMESTAMPTZ NOT NULL
)