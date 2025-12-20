-- Main hero groups (for common/shared groups)
CREATE TABLE hero_groups_common (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    cloth_id INTEGER NOT NULL DEFAULT 1,
    assist_boss_id INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, group_id)
);

-- Hero group types (for different categories/modes)
CREATE TABLE hero_group_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,  -- Maps to "id" in the proto (1, 2, 3, etc.)
    current_select INTEGER NOT NULL DEFAULT 0,  -- Which group_id is selected
    group_id INTEGER,  -- Reference to hero_groups_common, can be NULL
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, type_id)
);

-- Hero group members (shared by both common and type groups)
CREATE TABLE hero_group_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hero_group_id INTEGER NOT NULL,
    hero_uid INTEGER NOT NULL,
    position INTEGER NOT NULL,
    FOREIGN KEY (hero_group_id) REFERENCES hero_groups_common(id) ON DELETE CASCADE
);

-- Equipment slots (shared)
CREATE TABLE hero_group_equips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hero_group_id INTEGER NOT NULL,
    index_slot INTEGER NOT NULL,
    equip_uid INTEGER NOT NULL,
    FOREIGN KEY (hero_group_id) REFERENCES hero_groups_common(id) ON DELETE CASCADE
);

-- Activity104 equipment slots (shared)
CREATE TABLE hero_group_activity104_equips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hero_group_id INTEGER NOT NULL,
    index_slot INTEGER NOT NULL,
    equip_uid INTEGER NOT NULL,
    FOREIGN KEY (hero_group_id) REFERENCES hero_groups_common(id) ON DELETE CASCADE
);

CREATE INDEX idx_hero_groups_common_user ON hero_groups_common(user_id);
CREATE INDEX idx_hero_group_types_user ON hero_group_types(user_id);
CREATE INDEX idx_hero_group_members_group ON hero_group_members(hero_group_id);
CREATE INDEX idx_hero_group_equips_group ON hero_group_equips(hero_group_id);
CREATE INDEX idx_hero_group_activity104_equips_group ON hero_group_activity104_equips(hero_group_id);
