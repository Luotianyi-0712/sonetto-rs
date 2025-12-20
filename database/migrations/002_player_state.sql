-- Player state with timestamp-based tracking
CREATE TABLE IF NOT EXISTS player_state (
    player_id INTEGER PRIMARY KEY,

    -- Login/initialization
    initial_login_complete BOOLEAN NOT NULL DEFAULT 0,  -- Still boolean (one-time event)
    last_login_timestamp INTEGER,  -- Last successful login
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    -- Reward/daily reset timestamps (replaces boolean flags)
    last_state_push_sent_timestamp INTEGER,      -- Last time state pushes were sent
    last_activity_push_sent_timestamp INTEGER,   -- Last time activity pushes were sent
    last_currency_push_sent_timestamp INTEGER,   -- Last time currency pushes were sent

    -- Daily rewards/resets
    last_daily_reward_time INTEGER,              -- Last daily reward claim
    last_daily_reset_time INTEGER,               -- Last daily reset (for tracking)

    -- Monthly/regular features
    month_card_claimed BOOLEAN NOT NULL DEFAULT 0,
    last_month_card_claim_timestamp INTEGER,     -- When month card was last claimed

    -- Sign-in tracking
    last_sign_in_day INTEGER NOT NULL DEFAULT 0, -- Last calendar day signed in (YYYYMMDD format)
    last_sign_in_time INTEGER,                   -- Timestamp of last sign-in

    -- VIP/level
    vip_level INTEGER NOT NULL DEFAULT 0,

    -- Optional: Add other timestamp-based features you might need
    last_energy_refill_time INTEGER,            -- Last energy/currency refill
    last_weekly_reset_time INTEGER,             -- Last weekly reset
    last_monthly_reset_time INTEGER,            -- Last monthly reset

    FOREIGN KEY (player_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_player_state_player_id ON player_state(player_id);
CREATE INDEX IF NOT EXISTS idx_player_state_last_daily ON player_state(last_daily_reward_time);
CREATE INDEX IF NOT EXISTS idx_player_state_last_reset ON player_state(last_daily_reset_time);
