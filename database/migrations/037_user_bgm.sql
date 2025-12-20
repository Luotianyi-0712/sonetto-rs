CREATE TABLE IF NOT EXISTS user_bgm (
    player_id      INTEGER NOT NULL,
    bgm_id         INTEGER NOT NULL,

    unlock_time    INTEGER NOT NULL,   -- unix seconds
    is_favorite    BOOLEAN NOT NULL DEFAULT 0,
    is_read        BOOLEAN NOT NULL DEFAULT 0,

    PRIMARY KEY (player_id, bgm_id),
    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_bgm_state (
    player_id     INTEGER PRIMARY KEY,
    use_bgm_id    INTEGER NOT NULL,

    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE
);
