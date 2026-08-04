-- Down: drop activity.activities table
DROP TABLE IF EXISTS activity.activities CASCADE;
DROP FUNCTION IF EXISTS activity.activities_audit_timestamp() CASCADE;
