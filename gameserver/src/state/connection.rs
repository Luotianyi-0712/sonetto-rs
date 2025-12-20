use common::time::ServerTime;
use prost::Message;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::error::AppError;
use crate::utils::common::send_raw_server_message;
use sonettobuf::CmdId;

use super::{AppState, CommandPacket, PlayerState, encode_message};

/// Represents a single TCP connection
#[allow(dead_code)]
pub struct ConnectionContext {
    pub socket: Arc<Mutex<TcpStream>>,
    pub state: Arc<AppState>,
    pub player_id: Option<i64>,
    pub send_queue: VecDeque<CommandPacket>,

    // Persistent player state
    pub player_state: Option<PlayerState>,

    // Connection-only state
    pub logged_in: bool,

    // Internal
    next_sequence: u32,

    pub active_battle: Option<ActiveBattle>,
}

#[allow(dead_code)]
pub struct ActiveBattle {
    pub tower_type: Option<i32>,
    pub tower_id: Option<i32>,
    pub layer_id: Option<i32>,
    pub episode_id: i32,
    pub chapter_id: i32,
    pub difficulty: Option<i32>,
    pub talent_plan_id: Option<i32>,
    pub fight: Option<sonettobuf::Fight>, // Current battle state
    pub current_round: i32,
    pub act_point: i32, //  Remaining action points
    pub power: i32,
    pub current_deck: Vec<sonettobuf::CardInfo>,
    pub fight_group: Option<sonettobuf::FightGroup>,
    pub fight_id: Option<i64>,
    pub is_replay: Option<bool>,
    pub replay_episode_id: Option<i32>,
    pub multiplication: Option<i32>,
}

#[allow(dead_code)]
impl ConnectionContext {
    pub fn new(socket: Arc<Mutex<TcpStream>>, state: Arc<AppState>) -> Self {
        Self {
            socket,
            state,
            player_id: None,
            send_queue: VecDeque::new(),
            player_state: None,
            logged_in: false,
            next_sequence: 0,
            active_battle: None,
        }
    }

    /// Load player state from database after successful login
    pub async fn load_player_state(&mut self, player_id: i64) -> Result<(), AppError> {
        self.player_id = Some(player_id);
        self.logged_in = true;

        let current_time = ServerTime::now_ms() as i64;

        // Try to load existing player state
        let state =
            sqlx::query_as::<_, PlayerState>("SELECT * FROM player_state WHERE player_id = ?1")
                .bind(player_id)
                .fetch_optional(&self.state.db)
                .await?;

        let player_state = match state {
            Some(state) => {
                // Update login timestamp
                let mut state = state;
                state.update_login(current_time as u64);

                // Check if we need to reset for new day
                if state.is_new_day_for_reset(current_time as u64) {
                    tracing::info!(
                        "New day detected for player {}, performing daily reset",
                        player_id
                    );
                    state.mark_daily_reset(current_time as u64);
                }

                state
            }
            None => {
                tracing::info!("Creating new player state for player {}", player_id);
                let mut new_state = PlayerState::new(player_id);
                new_state.mark_login_complete();
                new_state
            }
        };

        // Save the updated state
        self.save_player_state_to_db(&player_state).await?;

        self.player_state = Some(player_state);

        tracing::info!("Loaded player state for player {}", player_id);
        Ok(())
    }

    /// Save player state to database
    pub async fn save_player_state_to_db(&self, state: &PlayerState) -> Result<(), AppError> {
        sqlx::query(
            "INSERT OR REPLACE INTO player_state (
                player_id, initial_login_complete, last_login_timestamp,
                created_at, updated_at,
                last_state_push_sent_timestamp, last_activity_push_sent_timestamp,
                last_currency_push_sent_timestamp,
                last_daily_reward_time, last_daily_reset_time,
                month_card_claimed, last_month_card_claim_timestamp,
                last_sign_in_day, last_sign_in_time,
                vip_level,
                last_energy_refill_time, last_weekly_reset_time, last_monthly_reset_time
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)"
        )
        .bind(state.player_id)
        .bind(state.initial_login_complete as i64)
        .bind(state.last_login_timestamp)
        .bind(state.created_at)
        .bind(state.updated_at)
        .bind(state.last_state_push_sent_timestamp)
        .bind(state.last_activity_push_sent_timestamp)
        .bind(state.last_currency_push_sent_timestamp)
        .bind(state.last_daily_reward_time)
        .bind(state.last_daily_reset_time)
        .bind(state.month_card_claimed as i64)
        .bind(state.last_month_card_claim_timestamp)
        .bind(state.last_sign_in_day)
        .bind(state.last_sign_in_time)
        .bind(state.vip_level)
        .bind(state.last_energy_refill_time)
        .bind(state.last_weekly_reset_time)
        .bind(state.last_monthly_reset_time)
        .execute(&self.state.db)
        .await?;

        Ok(())
    }

    /// Save current player state to database
    pub async fn save_current_player_state(&self) -> Result<(), AppError> {
        if let Some(state) = &self.player_state {
            self.save_player_state_to_db(state).await?;
        }
        Ok(())
    }

    /// Update and save player state
    pub async fn update_and_save_player_state<F>(&mut self, update_fn: F) -> Result<(), AppError>
    where
        F: FnOnce(&mut PlayerState),
    {
        let mut state = match self.player_state.clone() {
            Some(s) => s,
            None => return Ok(()),
        };

        update_fn(&mut state);
        state.updated_at = ServerTime::now_ms() as i64;

        self.save_player_state_to_db(&state).await?;

        self.player_state = Some(state);

        Ok(())
    }

    /// Check if should send state pushes and mark as sent if needed
    pub async fn check_and_mark_state_pushes(&mut self) -> Result<bool, AppError> {
        let now = ServerTime::now_ms();

        let mut state = match self.player_state.clone() {
            Some(s) => s,
            None => return Ok(false),
        };

        let should_send = state.should_send_state_pushes(now);

        if should_send {
            state.mark_state_pushes_sent(now);
            state.updated_at = now as i64;

            self.save_player_state_to_db(&state).await?;
            self.player_state = Some(state);
        }

        Ok(should_send)
    }

    /// Check if should send activity pushes and mark as sent if needed
    pub async fn check_and_mark_activity_pushes(&mut self) -> Result<bool, AppError> {
        let now = ServerTime::now_ms();

        let mut state = match self.player_state.clone() {
            Some(s) => s,
            None => return Ok(false),
        };

        let should_send = state.should_send_activity_pushes(now);

        if should_send {
            state.mark_activity_pushes_sent(now);
            state.updated_at = now as i64;

            self.save_player_state_to_db(&state).await?;
            self.player_state = Some(state);
        }

        Ok(should_send)
    }

    /// Check if should send currency pushes and mark as sent if needed
    pub async fn check_and_mark_currency_pushes(&mut self) -> Result<bool, AppError> {
        let now = ServerTime::now_ms();

        let mut state = match self.player_state.clone() {
            Some(s) => s,
            None => return Ok(false),
        };

        let should_send = state.should_send_currency_pushes(now);

        if should_send {
            state.mark_currency_pushes_sent(now);
            state.updated_at = now as i64;

            self.save_player_state_to_db(&state).await?;
            self.player_state = Some(state);
        }

        Ok(should_send)
    }

    /// Check if can claim daily reward and mark as claimed if needed
    pub async fn check_and_mark_daily_reward(&mut self) -> Result<bool, AppError> {
        let now = ServerTime::now_ms();

        let mut state = match self.player_state.clone() {
            Some(s) => s,
            None => return Ok(false),
        };

        let should_send = state.is_new_day_for_rewards(now);

        if should_send {
            state.mark_daily_reward_claimed(now);
            state.updated_at = now as i64;

            self.save_player_state_to_db(&state).await?;
            self.player_state = Some(state);
        }

        Ok(should_send)
    }

    /// Get mutable reference to player state
    pub fn player_state_mut(&mut self) -> Option<&mut PlayerState> {
        self.player_state.as_mut()
    }

    /// Get immutable reference to player state
    pub fn player_state(&self) -> Option<&PlayerState> {
        self.player_state.as_ref()
    }

    pub fn next_sequence(&mut self) -> u32 {
        let seq = self.next_sequence;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        seq
    }

    pub fn queue_packet(&mut self, packet: CommandPacket) {
        self.send_queue.push_back(packet);
    }

    /// Send a push notification to the client
    pub async fn send_push<T: Message>(&mut self, cmd_id: CmdId, msg: T) -> Result<(), AppError> {
        let body = encode_message(&msg)?;
        let down_tag = self.state.reserve_down_tag().await;

        let packet = CommandPacket::Push {
            cmd_id,
            body,
            down_tag,
        };

        self.queue_packet(packet);
        Ok(())
    }

    /// Send a reply to a client request
    pub async fn send_reply<T: Message>(
        &mut self,
        cmd_id: CmdId,
        msg: T,
        result_code: i16,
        up_tag: u8,
    ) -> Result<(), AppError> {
        let body = encode_message(&msg)?;
        let down_tag = self.state.reserve_down_tag().await;

        let packet = CommandPacket::Reply {
            cmd_id,
            body,
            result_code,
            up_tag,
            down_tag,
        };

        self.queue_packet(packet);
        Ok(())
    }

    /// Send a reply with explicit down_tag
    pub async fn send_reply_with_down_tag<T: Message>(
        &mut self,
        cmd_id: CmdId,
        msg: T,
        result_code: i16,
        up_tag: u8,
        down_tag: u8,
    ) -> Result<(), AppError> {
        let body = encode_message(&msg)?;

        let packet = CommandPacket::Reply {
            cmd_id,
            body,
            result_code,
            up_tag,
            down_tag,
        };

        self.queue_packet(packet);
        Ok(())
    }

    /// Send raw bytes as reply
    pub async fn send_raw_reply(
        &mut self,
        cmd_id: CmdId,
        body: Vec<u8>,
        result_code: i16,
        up_tag: u8,
    ) -> Result<(), AppError> {
        let down_tag = self.state.reserve_down_tag().await;

        let packet = CommandPacket::Reply {
            cmd_id,
            body,
            result_code,
            up_tag,
            down_tag,
        };

        self.queue_packet(packet);
        Ok(())
    }

    /// Send raw bytes as reply with explicit down_tag
    pub async fn send_raw_reply_with_down_tag(
        &mut self,
        cmd_id: CmdId,
        body: Vec<u8>,
        result_code: i16,
        up_tag: u8,
        down_tag: u8,
    ) -> Result<(), AppError> {
        let packet = CommandPacket::Reply {
            cmd_id,
            body,
            result_code,
            up_tag,
            down_tag,
        };

        self.queue_packet(packet);
        Ok(())
    }

    /// Flush all queued packets to the socket
    pub async fn flush_send_queue(&mut self) -> Result<(), AppError> {
        let mut socket = self.socket.lock().await;

        while let Some(packet) = self.send_queue.pop_front() {
            match packet {
                CommandPacket::Push {
                    cmd_id,
                    body,
                    down_tag,
                } => {
                    send_raw_server_message(&mut socket, cmd_id, body, 0, 255, down_tag).await?;
                }
                CommandPacket::Reply {
                    cmd_id,
                    body,
                    result_code,
                    up_tag,
                    down_tag,
                } => {
                    send_raw_server_message(
                        &mut socket,
                        cmd_id,
                        body,
                        result_code,
                        up_tag,
                        down_tag,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    /// Register this connection in the sessions map
    /// Should be called after successful login with the Arc wrapping this context
    pub async fn register(ctx: Arc<Mutex<Self>>) {
        let ctx_lock = ctx.lock().await;
        if let Some(player_id) = ctx_lock.player_id {
            ctx_lock.state.register_session(player_id, Arc::clone(&ctx));
            tracing::info!("Registered session for player {}", player_id);
        } else {
            tracing::warn!("Attempted to register session without player_id");
        }
    }
}

impl Drop for ConnectionContext {
    fn drop(&mut self) {
        if let Some(player_id) = self.player_id {
            self.state.unregister_session(player_id);
        }
    }
}
