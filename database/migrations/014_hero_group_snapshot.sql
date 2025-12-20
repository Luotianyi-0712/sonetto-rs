CREATE TABLE hero_group_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    snapshot_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, snapshot_id)
);

CREATE TABLE hero_group_snapshot_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_id INTEGER NOT NULL,  -- References hero_group_snapshots.id
    group_id INTEGER NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    cloth_id INTEGER NOT NULL DEFAULT 1,
    assist_boss_id INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (snapshot_id) REFERENCES hero_group_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE hero_group_snapshot_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_group_id INTEGER NOT NULL,
    hero_uid INTEGER NOT NULL,
    position INTEGER NOT NULL,
    FOREIGN KEY (snapshot_group_id) REFERENCES hero_group_snapshot_groups(id) ON DELETE CASCADE
);

CREATE TABLE hero_group_snapshot_equips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_group_id INTEGER NOT NULL,
    index_slot INTEGER NOT NULL,
    equip_uid INTEGER NOT NULL,
    FOREIGN KEY (snapshot_group_id) REFERENCES hero_group_snapshot_groups(id) ON DELETE CASCADE
);

CREATE TABLE hero_group_snapshot_activity104_equips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_group_id INTEGER NOT NULL,
    index_slot INTEGER NOT NULL,
    equip_uid INTEGER NOT NULL,
    FOREIGN KEY (snapshot_group_id) REFERENCES hero_group_snapshot_groups(id) ON DELETE CASCADE
);

CREATE TABLE hero_group_snapshot_sort_ids (
    snapshot_id INTEGER NOT NULL,
    sub_id INTEGER NOT NULL,
    sort_order INTEGER NOT NULL,
    FOREIGN KEY (snapshot_id) REFERENCES hero_group_snapshots(id) ON DELETE CASCADE
);
