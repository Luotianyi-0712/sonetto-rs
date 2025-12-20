-- Main sign-in info
CREATE TABLE user_sign_in_info (
    user_id INTEGER PRIMARY KEY,
    addup_sign_in_day INTEGER NOT NULL DEFAULT 0,
    open_function_time INTEGER NOT NULL DEFAULT 0,
    reward_mark INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Days signed in
CREATE TABLE user_sign_in_days (
    user_id INTEGER NOT NULL,
    server_day INTEGER NOT NULL,
    day_of_month INTEGER NOT NULL,
    PRIMARY KEY (user_id, server_day),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Collected addup bonuses
CREATE TABLE user_sign_in_addup_bonus (
    user_id INTEGER NOT NULL,
    bonus_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, bonus_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Month card days
CREATE TABLE user_month_card_days (
    user_id INTEGER NOT NULL,
    server_day INTEGER NOT NULL,
    day_of_month INTEGER NOT NULL,
    PRIMARY KEY (user_id, server_day),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
-- Month card history
CREATE TABLE user_month_card_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);


CREATE INDEX idx_user_sign_in_days ON user_sign_in_days(user_id);
CREATE INDEX idx_user_month_card_history ON user_month_card_history(user_id);
CREATE INDEX idx_user_month_card_days_user ON user_month_card_days(user_id);
