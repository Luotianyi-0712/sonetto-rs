CREATE TABLE guide_progress (
    user_id INTEGER NOT NULL,
    guide_id INTEGER NOT NULL,
    step_id INTEGER NOT NULL DEFAULT -1,  -- -1 means completed
    PRIMARY KEY (user_id, guide_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_guide_progress_user ON guide_progress(user_id);
