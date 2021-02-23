-- This file should undo anything in `up.sql`
ALTER TABLE to_do DROP COLUMN user_id;

-- drop the users table
DROP TABLE users
