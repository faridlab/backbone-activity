-- Down: drop the declared rule constraints for activity module
ALTER TABLE IF EXISTS activity.activities DROP CONSTRAINT IF EXISTS ck_activities_notes_max_length__max_length;
ALTER TABLE IF EXISTS activity.activities DROP CONSTRAINT IF EXISTS ck_activities_outcome_max_length__max_length;
ALTER TABLE IF EXISTS activity.activities DROP CONSTRAINT IF EXISTS ck_activities_subject_max_length__max_length;
ALTER TABLE IF EXISTS activity.activities DROP CONSTRAINT IF EXISTS ck_activities_subject_required__required;
