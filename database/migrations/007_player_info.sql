-- Player profile information
CREATE TABLE IF NOT EXISTS player_info (
    player_id INTEGER PRIMARY KEY,
    signature TEXT NOT NULL DEFAULT '',
    birthday TEXT NOT NULL DEFAULT '',
    portrait INTEGER NOT NULL DEFAULT 171603,
    show_achievement TEXT NOT NULL DEFAULT '',
    bg INTEGER NOT NULL DEFAULT 0,
    total_login_days INTEGER NOT NULL DEFAULT 0,
    last_episode_id INTEGER NOT NULL DEFAULT 0,
    last_logout_time INTEGER,

    -- Hero rarity counts (cached from hero table)
    hero_rare_nn_count INTEGER NOT NULL DEFAULT 0,
    hero_rare_n_count INTEGER NOT NULL DEFAULT 0,
    hero_rare_r_count INTEGER NOT NULL DEFAULT 0,
    hero_rare_sr_count INTEGER NOT NULL DEFAULT 0,
    hero_rare_ssr_count INTEGER NOT NULL DEFAULT 0,

    -- Timestamps
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_player_info_player_id ON player_info(player_id);

-- Table for show heroes (profile display - up to 3 heroes)
CREATE TABLE IF NOT EXISTS player_show_heroes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    rank INTEGER NOT NULL DEFAULT 0,
    ex_skill_level INTEGER NOT NULL DEFAULT 0,
    skin INTEGER NOT NULL DEFAULT 0,
    display_order INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(player_id, display_order)
);

CREATE INDEX IF NOT EXISTS idx_show_heroes_player ON player_show_heroes(player_id);
