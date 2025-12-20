CREATE TABLE user_stats (
    user_id INTEGER PRIMARY KEY,
    first_charge BOOLEAN NOT NULL DEFAULT 0,
    total_charge_amount INTEGER NOT NULL DEFAULT 0,
    is_first_login BOOLEAN NOT NULL DEFAULT 1,
    user_tag TEXT NOT NULL DEFAULT '用户类型7',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
