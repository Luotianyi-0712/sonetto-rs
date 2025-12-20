CREATE TABLE user_dialogs (
    user_id INTEGER NOT NULL,
    dialog_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, dialog_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_dialogs ON user_dialogs(user_id);
