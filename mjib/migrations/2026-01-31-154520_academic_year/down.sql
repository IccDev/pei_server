-- This file should undo anything in `up.sql`
-- Drop join table
DROP TABLE IF EXISTS users_sections;

-- Remove FK + column from sections
ALTER TABLE sections
DROP CONSTRAINT IF EXISTS sections_academic_year_id_fkey;

ALTER TABLE sections
DROP COLUMN IF EXISTS academic_year_id;

-- Drop academic_year table
DROP TABLE IF EXISTS academic_year;