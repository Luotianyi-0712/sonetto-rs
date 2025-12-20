-- Main tower info
CREATE TABLE user_tower_info (
    user_id INTEGER PRIMARY KEY,
    mop_up_times INTEGER NOT NULL DEFAULT 0,
    trial_hero_season INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Tower open status
CREATE TABLE user_tower_opens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,
    round INTEGER NOT NULL DEFAULT 0,
    next_time INTEGER NOT NULL DEFAULT 0,
    tower_start_time INTEGER NOT NULL DEFAULT 0,
    task_end_time INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, tower_type, tower_id)
);

-- Tower progress
CREATE TABLE user_towers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    pass_layer_id INTEGER NOT NULL DEFAULT 0,
    history_high_score INTEGER NOT NULL DEFAULT 0,
    params TEXT NOT NULL DEFAULT '',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, tower_type, tower_id)
);

-- Open special layer IDs
CREATE TABLE user_tower_open_sp_layers (
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    sp_layer_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, tower_type, tower_id, sp_layer_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Pass teach IDs
CREATE TABLE user_tower_pass_teaches (
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    teach_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, tower_type, tower_id, teach_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Layer progress
CREATE TABLE user_tower_layers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    curr_high_score INTEGER NOT NULL DEFAULT 0,
    history_high_score INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, tower_type, tower_id, layer_id)
);

-- Episode progress
CREATE TABLE user_tower_episodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,
    assist_boss_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, tower_type, tower_id, layer_id, episode_id)
);

-- Episode heroes
CREATE TABLE user_tower_episode_heroes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    trial_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Episode hero equipment
CREATE TABLE user_tower_episode_hero_equips (
    user_id INTEGER NOT NULL,
    tower_type INTEGER NOT NULL,
    tower_id INTEGER NOT NULL,
    layer_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    equip_uid INTEGER NOT NULL,
    PRIMARY KEY (user_id, tower_type, tower_id, layer_id, episode_id, hero_id, equip_uid),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Assist bosses
CREATE TABLE user_assist_bosses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    boss_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    use_talent_plan INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, boss_id)
);

-- Talent plans
CREATE TABLE user_assist_boss_talent_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    boss_id INTEGER NOT NULL,
    plan_id INTEGER NOT NULL,
    talent_point INTEGER NOT NULL DEFAULT 0,
    plan_name TEXT NOT NULL DEFAULT '',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, boss_id, plan_id)
);

-- Talent plan talent IDs
CREATE TABLE user_assist_boss_plan_talents (
    user_id INTEGER NOT NULL,
    boss_id INTEGER NOT NULL,
    plan_id INTEGER NOT NULL,
    talent_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, boss_id, plan_id, talent_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_tower_opens ON user_tower_opens(user_id);
CREATE INDEX idx_user_towers ON user_towers(user_id);
CREATE INDEX idx_user_tower_layers ON user_tower_layers(user_id, tower_type, tower_id);
CREATE INDEX idx_user_tower_episodes ON user_tower_episodes(user_id, tower_type, tower_id, layer_id);
CREATE INDEX idx_user_assist_bosses ON user_assist_bosses(user_id);
