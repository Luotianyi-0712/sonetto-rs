-- User dungeon progress
CREATE TABLE user_dungeons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    star INTEGER NOT NULL DEFAULT 0,
    challenge_count INTEGER NOT NULL DEFAULT 0,
    has_record BOOLEAN NOT NULL DEFAULT 0,
    left_return_all_num INTEGER NOT NULL DEFAULT 0,
    today_pass_num INTEGER NOT NULL DEFAULT 0,
    today_total_num INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, chapter_id, episode_id)
);

-- Last hero group used for each dungeon chapter
CREATE TABLE dungeon_last_hero_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    hero_group_id INTEGER NOT NULL,  -- References hero_groups_common
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, chapter_id)
);

-- Unlocked maps
CREATE TABLE user_dungeon_maps (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, map_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Available elements
CREATE TABLE user_dungeon_elements (
    user_id INTEGER NOT NULL,
    element_id INTEGER NOT NULL,
    is_finished BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, element_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Reward points per chapter
CREATE TABLE user_dungeon_reward_points (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    reward_point INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, chapter_id)
);

-- Claimed point rewards
CREATE TABLE user_dungeon_claimed_rewards (
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    point_reward_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, chapter_id, point_reward_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Special equipment chapters
CREATE TABLE user_dungeon_equip_sp_chapters (
    user_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, chapter_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Chapter type daily attempts
CREATE TABLE user_chapter_type_nums (
    user_id INTEGER NOT NULL,
    chapter_type INTEGER NOT NULL,
    today_pass_num INTEGER NOT NULL DEFAULT 0,
    today_total_num INTEGER NOT NULL DEFAULT 0,
    last_reset_date INTEGER NOT NULL,  -- Track daily reset
    PRIMARY KEY (user_id, chapter_type),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Finished puzzles
CREATE TABLE user_dungeon_finished_puzzles (
    user_id INTEGER NOT NULL,
    puzzle_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, puzzle_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_dungeons_user ON user_dungeons(user_id);
CREATE INDEX idx_user_dungeons_chapter ON user_dungeons(user_id, chapter_id);
CREATE INDEX idx_dungeon_last_hero_groups_user ON dungeon_last_hero_groups(user_id);
CREATE INDEX idx_user_dungeon_reward_points_user ON user_dungeon_reward_points(user_id);
CREATE INDEX idx_user_chapter_type_nums_user ON user_chapter_type_nums(user_id);
