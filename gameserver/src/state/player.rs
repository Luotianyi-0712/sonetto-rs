use common::time::ServerTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerState {
    pub player_id: i64,

    // Login / lifecycle
    pub initial_login_complete: bool,
    pub last_login_timestamp: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,

    // Push tracking (UTC ms)
    pub last_state_push_sent_timestamp: Option<i64>,
    pub last_activity_push_sent_timestamp: Option<i64>,
    pub last_currency_push_sent_timestamp: Option<i64>,

    // Daily logic (UTC ms)
    pub last_daily_reward_time: Option<i64>,
    pub last_daily_reset_time: Option<i64>,

    // Monthly card
    pub month_card_claimed: bool,
    pub last_month_card_claim_timestamp: Option<i64>,

    // Sign-in tracking (SERVER DAY, NOT YYYYMMDD)
    pub last_sign_in_day: i32, // server_day
    pub last_sign_in_time: Option<i64>,

    // VIP
    pub vip_level: i32,

    // Optional future
    pub last_energy_refill_time: Option<i64>,
    pub last_weekly_reset_time: Option<i64>,
    pub last_monthly_reset_time: Option<i64>,
}

#[allow(dead_code)]
impl PlayerState {
    pub fn new(player_id: i64) -> Self {
        let now = ServerTime::now_ms() as i64;
        let server_day = ServerTime::server_day(now as u64);

        Self {
            player_id,

            initial_login_complete: false,
            last_login_timestamp: Some(now),
            created_at: now,
            updated_at: now,

            last_state_push_sent_timestamp: None,
            last_activity_push_sent_timestamp: None,
            last_currency_push_sent_timestamp: None,

            // New user = never claimed / never reset
            last_daily_reward_time: None,
            last_daily_reset_time: None,

            month_card_claimed: false,
            last_month_card_claim_timestamp: None,

            last_sign_in_day: server_day,
            last_sign_in_time: Some(now),

            vip_level: 0,

            last_energy_refill_time: None,
            last_weekly_reset_time: None,
            last_monthly_reset_time: None,
        }
    }

    // Check if should send state pushes (daily reset logic)
    pub fn should_send_state_pushes(&self, current_time: u64) -> bool {
        // Send on new day or if never sent before
        self.is_new_day_for_reset(current_time) || self.last_state_push_sent_timestamp.is_none()
    }

    // Check if should send activity pushes
    pub fn should_send_activity_pushes(&self, current_time: u64) -> bool {
        self.is_new_day_for_reset(current_time) || self.last_activity_push_sent_timestamp.is_none()
    }

    // Check if should send currency pushes
    pub fn should_send_currency_pushes(&self, current_time: u64) -> bool {
        self.is_new_day_for_reset(current_time) || self.last_currency_push_sent_timestamp.is_none()
    }

    // Mark state pushes as sent
    pub fn mark_state_pushes_sent(&mut self, current_time: u64) {
        self.last_state_push_sent_timestamp = Some(current_time as i64);
        self.updated_at = current_time as i64;
    }

    // Mark activity pushes as sent
    pub fn mark_activity_pushes_sent(&mut self, current_time: u64) {
        self.last_activity_push_sent_timestamp = Some(current_time as i64);
        self.updated_at = current_time as i64;
    }

    // Mark currency pushes as sent
    pub fn mark_currency_pushes_sent(&mut self, current_time: u64) {
        self.last_currency_push_sent_timestamp = Some(current_time as i64);
        self.updated_at = current_time as i64;
    }

    // Mark daily reward as claimed
    pub fn mark_daily_reward_claimed(&mut self, current_time: u64) {
        self.last_daily_reward_time = Some(current_time as i64);
        self.updated_at = current_time as i64;

        // Also update daily reset time if this is the first action of the day
        if self.is_new_day_for_reset(current_time) {
            self.mark_daily_reset(current_time);
        }
    }

    // Mark daily reset
    pub fn mark_daily_reset(&mut self, current_time: u64) {
        self.last_daily_reset_time = Some(current_time as i64);
        self.updated_at = current_time as i64;
    }

    // Check if can claim month card
    pub fn can_claim_month_card(&self, now: u64) -> bool {
        match self.last_month_card_claim_timestamp {
            Some(ts) => ServerTime::server_day(ts as u64) != ServerTime::server_day(now),
            None => true,
        }
    }

    // Claim month card
    pub fn claim_month_card(&mut self, current_time: u64) {
        self.month_card_claimed = true;
        self.last_month_card_claim_timestamp = Some(current_time as i64);
        self.updated_at = current_time as i64;
    }

    // Check if it's a new sign-in day (using server time)
    pub fn is_new_sign_in_day(&self, now: u64) -> bool {
        let today = ServerTime::server_day(now);
        self.last_sign_in_day != today
    }

    // Record sign-in
    pub fn record_sign_in(&mut self, now: u64) {
        self.last_sign_in_day = ServerTime::server_day(now);
        self.last_sign_in_time = Some(now as i64);
        self.updated_at = now as i64;
    }

    pub fn is_same_server_day(time1: u64, time2: u64) -> bool {
        common::time::ServerTime::is_same_day(time1, time2)
    }

    // Update login
    pub fn update_login(&mut self, current_time: u64) {
        self.last_login_timestamp = Some(current_time as i64);

        // Check if we need daily reset using server time
        if self.is_new_day_for_reset(current_time) {
            self.mark_daily_reset(current_time);
        }

        self.updated_at = current_time as i64;
    }

    // Mark initial login complete
    pub fn mark_login_complete(&mut self) {
        self.initial_login_complete = true;
        self.updated_at = ServerTime::now_ms() as i64;
    }
}

#[allow(dead_code)]
impl PlayerState {
    // Daily rewards (already fixed in is_new_day_for_rewards)
    pub fn is_new_day_for_rewards(&self, now: u64) -> bool {
        match self.last_daily_reward_time {
            Some(ts) => ServerTime::server_day(ts as u64) != ServerTime::server_day(now),
            None => true,
        }
    }

    // Daily reset check
    pub fn is_new_day_for_reset(&self, now: u64) -> bool {
        match self.last_daily_reset_time {
            Some(ts) => ServerTime::server_day(ts as u64) != ServerTime::server_day(now),
            None => true,
        }
    }

    // Weekly reset (if you have weekly features)
    pub fn is_new_week_for_reset(&self, current_time: u64) -> bool {
        match self.last_weekly_reset_time {
            Some(last_reset) => {
                !common::time::ServerTime::is_same_week(last_reset as u64, current_time)
            }
            None => true,
        }
    }

    // Monthly reset
    pub fn is_new_month_for_reset(&self, current_time: u64) -> bool {
        match self.last_monthly_reset_time {
            Some(last_reset) => {
                !common::time::ServerTime::is_same_month(last_reset as u64, current_time)
            }
            None => true,
        }
    }
}
