CREATE TABLE user_antiques (
    user_id INTEGER NOT NULL,
    antique_id INTEGER NOT NULL,
    get_time INTEGER NOT NULL,  -- Unix timestamp in ms
    PRIMARY KEY (user_id, antique_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_antiques ON user_antiques(user_id);
