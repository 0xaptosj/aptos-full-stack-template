-- This file should undo anything in `up.sql`
ALTER TABLE module_upgrade_history
DROP COLUMN package_name;