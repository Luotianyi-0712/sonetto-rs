CREATE TABLE IF NOT EXISTS battle_replays (
    user_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    battle_id INTEGER NOT NULL,
    round_number INTEGER NOT NULL,
    cloth_skill_opers TEXT NOT NULL,  -- JSON array of UseClothSkillOperRecord
    opers TEXT NOT NULL,               -- JSON array of BeginRoundOper
    created_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, episode_id, battle_id, round_number),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_battle_replays ON battle_replays(user_id, episode_id);
