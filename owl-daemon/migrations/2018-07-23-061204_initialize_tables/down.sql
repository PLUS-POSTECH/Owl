DROP TRIGGER exploit_tasks_update_timestamp ON exploit_tasks;
DROP TRIGGER exploits_update_timestamp ON exploits;
DROP FUNCTION update_last_updated_time_at_column();
DROP FUNCTION update_last_modified_time_at_column();

DROP TABLE exploit_tasks;
DROP TYPE exploit_status;
DROP TABLE exploit_targets;
DROP TABLE exploit_attachments;
DROP TABLE exploits;
DROP TABLE service_providers;
DROP TABLE service_variant_attachments;
DROP TABLE service_variants;
DROP TABLE services;
DROP TABLE teams;
