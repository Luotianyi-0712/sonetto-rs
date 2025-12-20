CREATE TABLE user_achievements (
    user_id INTEGER NOT NULL,
    achievement_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    has_finish BOOLEAN NOT NULL DEFAULT 0,
    is_new BOOLEAN NOT NULL DEFAULT 0,
    finish_time INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, achievement_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_achievements ON user_achievements(user_id);
