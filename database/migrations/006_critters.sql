-- Main critters table
CREATE TABLE IF NOT EXISTS critters (
    uid INTEGER PRIMARY KEY,
    player_id INTEGER NOT NULL,
    define_id INTEGER NOT NULL,
    create_time INTEGER NOT NULL,

    -- Stats
    efficiency INTEGER NOT NULL DEFAULT 400,
    patience INTEGER NOT NULL DEFAULT 400,
    lucky INTEGER NOT NULL DEFAULT 400,
    efficiency_incr_rate INTEGER NOT NULL DEFAULT 0,
    patience_incr_rate INTEGER NOT NULL DEFAULT 0,
    lucky_incr_rate INTEGER NOT NULL DEFAULT 0,

    -- State
    special_skin BOOLEAN NOT NULL DEFAULT 0,
    current_mood INTEGER NOT NULL DEFAULT 20000,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    finish_train BOOLEAN NOT NULL DEFAULT 0,
    is_high_quality BOOLEAN NOT NULL DEFAULT 0,

    -- Associations
    train_hero_id INTEGER NOT NULL DEFAULT 0,
    total_finish_count INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL DEFAULT '',

    -- Timestamps
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_critters_player ON critters(player_id);
CREATE INDEX idx_critters_uid ON critters(uid);

-- Critter skill tags (many-to-many since tags are a list)
CREATE TABLE IF NOT EXISTS critter_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    critter_uid INTEGER NOT NULL,
    tag TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (critter_uid) REFERENCES critters(uid) ON DELETE CASCADE
);

CREATE INDEX idx_critter_skills_uid ON critter_skills(critter_uid);

-- Critter tag attribute rates (embedded list)
CREATE TABLE IF NOT EXISTS critter_tag_attributes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    critter_uid INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    rate INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (critter_uid) REFERENCES critters(uid) ON DELETE CASCADE
);

CREATE INDEX idx_critter_attrs_uid ON critter_tag_attributes(critter_uid);

-- Critter rest info (optional, embedded object)
CREATE TABLE IF NOT EXISTS critter_rest_info (
    critter_uid INTEGER PRIMARY KEY,
    building_uid INTEGER NOT NULL,
    rest_slot_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (critter_uid) REFERENCES critters(uid) ON DELETE CASCADE
);

-- Critter train info (optional, embedded object)
CREATE TABLE IF NOT EXISTS critter_train_info (
    critter_uid INTEGER PRIMARY KEY,
    -- Add train info fields here when you know the structure
    train_data TEXT,  -- JSON blob for now
    FOREIGN KEY (critter_uid) REFERENCES critters(uid) ON DELETE CASCADE
);

-- Critter work info (optional, embedded object)
CREATE TABLE IF NOT EXISTS critter_work_info (
    critter_uid INTEGER PRIMARY KEY,
    -- Add work info fields here when you know the structure
    work_data TEXT,  -- JSON blob for now
    FOREIGN KEY (critter_uid) REFERENCES critters(uid) ON DELETE CASCADE
);
