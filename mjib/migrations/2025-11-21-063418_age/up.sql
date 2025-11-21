-- Your SQL goes here
CREATE TABLE age (
  id INTEGER UNIQUE default(1) PRIMARY KEY,
  value INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  Constraint CHK_age_singlerow CHECK (id = 1)
)