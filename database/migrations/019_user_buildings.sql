CREATE TABLE user_buildings (
    uid INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    define_id INTEGER NOT NULL,
    in_use BOOLEAN NOT NULL DEFAULT 0,
    x INTEGER NOT NULL DEFAULT 0,
    y INTEGER NOT NULL DEFAULT 0,
    rotate INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_buildings_user ON user_buildings(user_id);
