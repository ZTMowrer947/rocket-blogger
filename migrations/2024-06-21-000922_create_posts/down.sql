-- Removal of updated_at trigger for posts table
DROP TRIGGER IF EXISTS set_updated_at ON posts;

-- Removal of posts table
DROP TABLE IF EXISTS posts;
