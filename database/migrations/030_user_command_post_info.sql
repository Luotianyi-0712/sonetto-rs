-- Main command post info
CREATE TABLE user_command_post_info (
    user_id INTEGER PRIMARY KEY,
    paper INTEGER NOT NULL DEFAULT 0,
    catch_num INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Command post events
CREATE TABLE user_command_post_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    state INTEGER NOT NULL DEFAULT 0,
    start_time INTEGER NOT NULL DEFAULT 0,
    end_time INTEGER NOT NULL DEFAULT 0,
    is_read BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, event_id)
);

-- Event hero IDs
CREATE TABLE user_command_post_event_heroes (
    user_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, event_id, hero_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Command post tasks
CREATE TABLE user_command_post_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    has_finished BOOLEAN NOT NULL DEFAULT 0,
    finish_count INTEGER NOT NULL DEFAULT 0,
    task_type INTEGER NOT NULL DEFAULT 0,
    expiry_time INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, task_id)
);

-- Catch tasks
CREATE TABLE user_command_post_catch_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    has_finished BOOLEAN NOT NULL DEFAULT 0,
    finish_count INTEGER NOT NULL DEFAULT 0,
    task_type INTEGER NOT NULL DEFAULT 0,
    expiry_time INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, task_id)
);

-- Gained bonuses
CREATE TABLE user_command_post_gain_bonus (
    user_id INTEGER NOT NULL,
    bonus_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, bonus_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_command_post_events ON user_command_post_events(user_id);
CREATE INDEX idx_user_command_post_tasks ON user_command_post_tasks(user_id);
CREATE INDEX idx_user_command_post_catch_tasks ON user_command_post_catch_tasks(user_id);
