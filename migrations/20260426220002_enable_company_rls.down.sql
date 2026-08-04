-- Down: remove the company RLS fence for activity module

-- Reverse the company RLS fence for activity.activities
DROP POLICY IF EXISTS activities_company_isolation ON activity.activities;
ALTER TABLE activity.activities NO FORCE ROW LEVEL SECURITY;
ALTER TABLE activity.activities DISABLE ROW LEVEL SECURITY;

