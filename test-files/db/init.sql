-- Initialize the todo database
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert some sample data
INSERT INTO todos (title, description, completed) VALUES
    ('Setup Container Compose', 'Build a Docker Compose-like tool for Apple containers', true),
    ('Test with complex configuration', 'Try multi-service setup with dependencies', false),
    ('Add database integration', 'Connect the API to PostgreSQL database', false),
    ('Implement Redis caching', 'Add Redis for session and data caching', false);

-- Create an index on completed status
CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos(completed);

-- Grant permissions to the user
GRANT ALL PRIVILEGES ON TABLE todos TO "user";
GRANT USAGE, SELECT ON SEQUENCE todos_id_seq TO "user";