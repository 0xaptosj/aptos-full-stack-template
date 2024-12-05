-- Your SQL goes here
ALTER TABLE module_upgrade_history
ADD COLUMN package_name VARCHAR(300) NOT NULL;