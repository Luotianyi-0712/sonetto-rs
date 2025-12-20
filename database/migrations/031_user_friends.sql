-- Friends
CREATE TABLE user_friends (
    user_id INTEGER NOT NULL,
    friend_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, friend_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Blacklist
CREATE TABLE user_blacklist (
    user_id INTEGER NOT NULL,
    blocked_user_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, blocked_user_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_friends ON user_friends(user_id);
CREATE INDEX idx_user_blacklist ON user_blacklist(user_id);
