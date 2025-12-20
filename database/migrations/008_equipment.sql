-- Equipment table
CREATE TABLE IF NOT EXISTS equipment (
    uid INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    equip_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    exp INTEGER NOT NULL DEFAULT 0,
    break_lv INTEGER NOT NULL DEFAULT 0,
    count INTEGER NOT NULL DEFAULT 1,
    is_lock BOOLEAN NOT NULL DEFAULT 0,
    refine_lv INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_equipment_user ON equipment(user_id);
CREATE INDEX IF NOT EXISTS idx_equipment_equip_id ON equipment(equip_id);
