-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN date_of_birth;

ALTER TABLE users
DROP COLUMN gsm;

ALTER TABLE users
DROP COLUMN pays;

ALTER TABLE users
DROP COLUMN ville;

ALTER TABLE users
DROP COLUMN eglise;

ALTER TABLE users
DROP COLUMN situation_professionnelle;

ALTER TABLE users
DROP COLUMN commenaire;

ALTER TABLE users
DROP COLUMN is_admin;

ALTER TABLE users
DROP COLUMN is_deleted;

ALTER TABLE users
ADD age INTEGER;
