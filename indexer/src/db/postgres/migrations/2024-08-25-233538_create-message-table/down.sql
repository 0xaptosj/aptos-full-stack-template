-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS messages;

DROP INDEX IF EXISTS idx_message_obj_addr;