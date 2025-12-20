-- Finished stories
CREATE TABLE user_finished_stories (
    user_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, story_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Processing stories (in-progress)
CREATE TABLE user_processing_stories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    step_id INTEGER NOT NULL DEFAULT 0,
    favor INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, story_id)
);

CREATE INDEX idx_user_finished_stories ON user_finished_stories(user_id);
CREATE INDEX idx_user_processing_stories ON user_processing_stories(user_id);
