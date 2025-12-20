-- Main red dot table
CREATE TABLE IF NOT EXISTS red_dots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    define_id INTEGER NOT NULL,  -- Type of red dot (2218, 2220, 2221, etc)
    info_id INTEGER NOT NULL DEFAULT 0,
    value INTEGER NOT NULL DEFAULT 0,
    time INTEGER NOT NULL DEFAULT 0,
    ext TEXT NOT NULL DEFAULT '',
    replace_all BOOLEAN NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(player_id, define_id, info_id)  -- One entry per player/type/id combo
);

CREATE INDEX IF NOT EXISTS idx_red_dots_player ON red_dots(player_id);
CREATE INDEX IF NOT EXISTS idx_red_dots_define ON red_dots(player_id, define_id);
