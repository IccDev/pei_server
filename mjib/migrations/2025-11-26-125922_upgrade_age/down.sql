-- This file should undo anything in `up.sql`
DROP TABLE age
CREATE TABLE age (
  id INTEGER UNIQUE default(1) PRIMARY KEY,
  value INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  Constraint CHK_age_singlerow CHECK (id = 1)
)