-- Your SQL goes here
CREATE TABLE disciplines (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  comment TEXT,
  section_id INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  FOREIGN KEY ("section_id") REFERENCES "sections"("id")
);