-- Main weekwalk v2 info
CREATE TABLE user_weekwalk_v2_info (
    user_id INTEGER PRIMARY KEY,
    time_id INTEGER NOT NULL DEFAULT 0,
    start_time INTEGER NOT NULL DEFAULT 0,
    end_time INTEGER NOT NULL DEFAULT 0,
    pop_rule BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Layer info
CREATE TABLE user_weekwalk_v2_layers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    scene_id INTEGER NOT NULL DEFAULT 0,
    all_pass BOOLEAN NOT NULL DEFAULT 0,
    finished BOOLEAN NOT NULL DEFAULT 0,
    unlock BOOLEAN NOT NULL DEFAULT 0,
    show_finished BOOLEAN NOT NULL DEFAULT 0,
    params TEXT NOT NULL DEFAULT '',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, layer_id)
);

-- Battle info
CREATE TABLE user_weekwalk_v2_battles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,
    hero_group_select INTEGER NOT NULL DEFAULT 0,
    element_id INTEGER NOT NULL DEFAULT 0,
    params TEXT NOT NULL DEFAULT '',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Battle hero IDs
CREATE TABLE user_weekwalk_v2_battle_heroes (
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, layer_id, battle_id, hero_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Battle choose skill IDs
CREATE TABLE user_weekwalk_v2_battle_skills (
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, layer_id, battle_id, skill_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Cup info
CREATE TABLE user_weekwalk_v2_cups (
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    cup_id INTEGER NOT NULL,
    result INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, layer_id, battle_id, cup_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Element info
CREATE TABLE user_weekwalk_v2_elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    element_id INTEGER NOT NULL,
    finish BOOLEAN NOT NULL DEFAULT 0,
    index_num INTEGER NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Previous settle info
CREATE TABLE user_weekwalk_v2_prev_settle (
    user_id INTEGER PRIMARY KEY,
    max_layer_id INTEGER NOT NULL DEFAULT 0,
    max_battle_id INTEGER NOT NULL DEFAULT 0,
    max_battle_index INTEGER NOT NULL DEFAULT 0,
    show BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Previous settle layer info
CREATE TABLE user_weekwalk_v2_prev_settle_layers (
    user_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    platinum_cup_num INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, layer_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Snapshot info
CREATE TABLE user_weekwalk_v2_snapshots (
    user_id INTEGER NOT NULL,
    snapshot_no INTEGER NOT NULL,
    PRIMARY KEY (user_id, snapshot_no),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Snapshot skill IDs
CREATE TABLE user_weekwalk_v2_snapshot_skills (
    user_id INTEGER NOT NULL,
    snapshot_no INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, snapshot_no, skill_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_weekwalk_v2_layers ON user_weekwalk_v2_layers(user_id);
CREATE INDEX idx_user_weekwalk_v2_battles ON user_weekwalk_v2_battles(user_id, layer_id);
CREATE INDEX idx_user_weekwalk_v2_elements ON user_weekwalk_v2_elements(user_id, layer_id);
