CREATE TABLE IF NOT EXISTS dungeon_records (
    user_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    record_round INTEGER NOT NULL,  -- Rounds to complete
    hero_list TEXT NOT NULL,         -- JSON array of hero UIDs
    sub_hero_list TEXT NOT NULL,     -- JSON array of sub hero UIDs
    cloth_id INTEGER NOT NULL,
    equips TEXT NOT NULL,            -- JSON equipment data
    version INTEGER NOT NULL DEFAULT 5,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, episode_id)
);
