-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_passengers_email;
DROP TABLE IF EXISTS passengers;