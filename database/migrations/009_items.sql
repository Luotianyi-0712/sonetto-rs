-- Main items table (currency, materials, consumables)
CREATE TABLE items (
    user_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    last_use_time INTEGER,  -- Unix timestamp
    last_update_time INTEGER,  -- Unix timestamp
    total_gain_count INTEGER DEFAULT 0,
    PRIMARY KEY (user_id, item_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Power items (temporary buffs/boosts with expiration)
CREATE TABLE power_items (
    uid INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    expire_time INTEGER NOT NULL,  -- Unix timestamp
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Insight items (temporary resources with expiration)
CREATE TABLE insight_items (
    uid INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    expire_time INTEGER NOT NULL,  -- Unix timestamp
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for common queries
CREATE INDEX idx_items_user ON items(user_id);
CREATE INDEX idx_power_items_user ON power_items(user_id);
CREATE INDEX idx_power_items_user_expire ON power_items(user_id, expire_time);
CREATE INDEX idx_power_items_item_id ON power_items(item_id);
CREATE INDEX idx_insight_items_user ON insight_items(user_id);
CREATE INDEX idx_insight_items_user_expire ON insight_items(user_id, expire_time);
CREATE INDEX idx_insight_items_item_id ON insight_items(item_id);
