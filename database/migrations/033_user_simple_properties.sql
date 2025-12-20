-- Simple properties (key-value store per user)
CREATE TABLE user_simple_properties (
    user_id INTEGER NOT NULL,
    property_id INTEGER NOT NULL,
    property_value TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (user_id, property_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_simple_properties ON user_simple_properties(user_id);
