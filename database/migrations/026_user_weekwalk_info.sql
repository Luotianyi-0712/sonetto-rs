-- Main weekwalk info
CREATE TABLE user_weekwalk_info (
    user_id INTEGER PRIMARY KEY,
    time INTEGER NOT NULL DEFAULT 0,
    end_time INTEGER NOT NULL DEFAULT 0,
    max_layer INTEGER NOT NULL DEFAULT 0,
    issue_id INTEGER NOT NULL DEFAULT 0,
    is_pop_deep_rule BOOLEAN NOT NULL DEFAULT 0,
    is_open_deep BOOLEAN NOT NULL DEFAULT 0,
    is_pop_shallow_settle BOOLEAN NOT NULL DEFAULT 0,
    is_pop_deep_settle BOOLEAN NOT NULL DEFAULT 0,
    deep_progress TEXT NOT NULL DEFAULT '',
    time_this_week INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Map info
CREATE TABLE user_weekwalk_maps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    scene_id INTEGER NOT NULL,
    is_finish INTEGER NOT NULL DEFAULT 0,
    is_finished INTEGER NOT NULL DEFAULT 0,
    buff_id INTEGER NOT NULL DEFAULT 0,
    is_show_buff BOOLEAN NOT NULL DEFAULT 0,
    is_show_finished BOOLEAN NOT NULL DEFAULT 0,
    is_show_select_cd BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, map_id)
);

-- Battle info
CREATE TABLE user_weekwalk_battles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    star INTEGER NOT NULL DEFAULT 0,
    max_star INTEGER NOT NULL DEFAULT 0,
    hero_group_select INTEGER NOT NULL DEFAULT 0,
    element_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Battle hero IDs
CREATE TABLE user_weekwalk_battle_heroes (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id, battle_id, hero_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Element info
CREATE TABLE user_weekwalk_elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    element_id INTEGER NOT NULL,
    is_finish BOOLEAN NOT NULL DEFAULT 0,
    index_num INTEGER NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Element history list
CREATE TABLE user_weekwalk_element_history (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    element_id INTEGER NOT NULL,
    history_entry TEXT NOT NULL,
    sort_order INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id, element_id, sort_order),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Hero info
CREATE TABLE user_weekwalk_heroes (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    cd INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, map_id, hero_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Story IDs
CREATE TABLE user_weekwalk_stories (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id, story_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_weekwalk_maps ON user_weekwalk_maps(user_id);
CREATE INDEX idx_user_weekwalk_battles ON user_weekwalk_battles(user_id, map_id);
CREATE INDEX idx_user_weekwalk_elements ON user_weekwalk_elements(user_id, map_id);
