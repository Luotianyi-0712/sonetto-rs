CREATE TABLE currencies (
    user_id INTEGER NOT NULL,
    currency_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    last_recover_time INTEGER,  -- Unix timestamp in ms
    expired_time INTEGER,        -- Unix timestamp in ms
    PRIMARY KEY (user_id, currency_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_currencies_user ON currencies(user_id);
