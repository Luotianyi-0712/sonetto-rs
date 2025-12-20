-- Main explore info
CREATE TABLE user_explore_info (
    user_id INTEGER PRIMARY KEY,
    last_map_id INTEGER NOT NULL DEFAULT 0,
    is_show_bag BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Chapter simple info
CREATE TABLE user_explore_chapters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    is_finish BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, chapter_id)
);

-- Archive IDs per chapter
CREATE TABLE user_explore_chapter_archives (
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    archive_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, chapter_id, archive_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Bonus scenes per chapter
CREATE TABLE user_explore_bonus_scenes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    bonus_scene_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, chapter_id, bonus_scene_id)
);

-- Bonus scene options
CREATE TABLE user_explore_bonus_scene_options (
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    bonus_scene_id INTEGER NOT NULL,
    option_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, chapter_id, bonus_scene_id, option_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Map simple info
CREATE TABLE user_explore_maps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    bonus_num INTEGER NOT NULL DEFAULT 0,
    gold_coin INTEGER NOT NULL DEFAULT 0,
    purple_coin INTEGER NOT NULL DEFAULT 0,
    bonus_num_total INTEGER NOT NULL DEFAULT 0,
    gold_coin_total INTEGER NOT NULL DEFAULT 0,
    purple_coin_total INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, map_id)
);

-- Map bonus IDs
CREATE TABLE user_explore_map_bonuses (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    bonus_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id, bonus_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Unlocked map IDs
CREATE TABLE user_explore_unlocked_maps (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_explore_chapters ON user_explore_chapters(user_id);
CREATE INDEX idx_user_explore_maps ON user_explore_maps(user_id);
