-- Block packages (purchased/unlocked packages)
CREATE TABLE user_block_packages (
    user_id INTEGER NOT NULL,
    block_package_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, block_package_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Special blocks (individual special blocks)
CREATE TABLE user_special_blocks (
    user_id INTEGER NOT NULL,
    block_id INTEGER NOT NULL,
    create_time INTEGER NOT NULL,
    PRIMARY KEY (user_id, block_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_block_packages ON user_block_packages(user_id);
CREATE INDEX idx_user_special_blocks ON user_special_blocks(user_id);
