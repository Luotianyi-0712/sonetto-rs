-- Activity 101 claims tracking
CREATE TABLE user_activity101_claims (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    day_id INTEGER NOT NULL,
    claimed_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, activity_id, day_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_activity101_claims ON user_activity101_claims(user_id, activity_id);

-- Activity 101 once bonus tracking
CREATE TABLE user_activity101_once_bonus (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    claimed_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, activity_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
