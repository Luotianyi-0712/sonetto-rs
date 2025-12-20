-- Charge/purchase info per product
CREATE TABLE user_charge_info (
    user_id INTEGER NOT NULL,
    charge_id INTEGER NOT NULL,  -- Product/package ID
    buy_count INTEGER NOT NULL DEFAULT 0,
    first_charge BOOLEAN NOT NULL DEFAULT 1,  -- True if never bought
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, charge_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Sandbox settings (for testing purchases)
CREATE TABLE user_sandbox_settings (
    user_id INTEGER PRIMARY KEY,
    sandbox_enable BOOLEAN NOT NULL DEFAULT 0,
    sandbox_balance INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_charge_info ON user_charge_info(user_id);
