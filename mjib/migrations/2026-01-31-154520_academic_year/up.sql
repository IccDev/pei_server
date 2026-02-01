-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE academic_year (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  comment TEXT,
  start_date TIMESTAMPTZ NOT NULL DEFAULT Now(),
  end_date TIMESTAMPTZ NOT NULL DEFAULT Now(),
  created_at TIMESTAMPTZ NOT NULL DEFAULT Now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT Now()
);

INSERT INTO academic_year (name, start_date, end_date)
    VALUES ('Default Academic Year', Now(), Now());

ALTER TABLE sections
ADD academic_year_id INTEGER;

UPDATE sections
    SET academic_year_id = (SELECT id FROM academic_year LIMIT 1);

ALTER TABLE sections
ALTER COLUMN academic_year_id SET NOT NULL;

ALTER TABLE sections
ADD CONSTRAINT sections_academic_year_id_fkey
FOREIGN KEY (academic_year_id) REFERENCES academic_year(id);

CREATE TABLE users_sections (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  section_id INTEGER NOT NULL,
  FOREIGN KEY ("user_id") REFERENCES "users"("id"),
  FOREIGN KEY ("section_id") REFERENCES "sections"("id")
);