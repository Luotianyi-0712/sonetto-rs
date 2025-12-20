CREATE TABLE user_character_interactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    interaction_id INTEGER NOT NULL,
    is_finished BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, interaction_id)
);

CREATE TABLE user_character_interaction_selections (
    user_id INTEGER NOT NULL,
    interaction_id INTEGER NOT NULL,
    select_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, interaction_id, select_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Track total interaction count per user
CREATE TABLE user_interaction_stats (
    user_id INTEGER PRIMARY KEY,
    interaction_count INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_character_interactions ON user_character_interactions(user_id);
CREATE INDEX idx_user_character_interaction_selections ON user_character_interaction_selections(user_id, interaction_id);
