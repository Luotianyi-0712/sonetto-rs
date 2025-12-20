CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT,       -- Hashed password
    email TEXT,
    account_type INTEGER NOT NULL DEFAULT 10,  -- 10=email, 13=Bluepoch, 15=Steam
    registration_account_type INTEGER NOT NULL DEFAULT 1,
    token TEXT,
    refresh_token TEXT,
    token_expires_at INTEGER,

    -- Player attributes
    vip_level INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1,
    exp INTEGER NOT NULL DEFAULT 0,

    -- Real name verification
    need_real_name BOOLEAN NOT NULL DEFAULT 0,
    real_name_status BOOLEAN NOT NULL DEFAULT 1,
    age INTEGER,
    is_adult BOOLEAN NOT NULL DEFAULT 1,

    -- Flags
    need_activate BOOLEAN NOT NULL DEFAULT 0,
    cipher_mark BOOLEAN NOT NULL DEFAULT 1,
    first_join BOOLEAN NOT NULL DEFAULT 0,
    account_tags TEXT,

    -- Timestamps
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    last_login_at INTEGER
);
