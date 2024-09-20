-- This file should undo anything in `up.sql`
ALTER TABLE user_stats
RENAME COLUMN user_points TO user_point;