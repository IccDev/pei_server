-- Your SQL goes here
CREATE TABLE courses (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  comment TEXT,
  start_date TIMESTAMPTZ NOT NULL,
  end_date TIMESTAMPTZ NOT NULL,
  video_link TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now()
)