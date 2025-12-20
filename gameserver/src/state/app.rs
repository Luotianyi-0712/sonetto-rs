use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::ConnectionContext;

/// App-level shared state
pub struct AppState {
    next_down_tag: Mutex<u8>,
    pub db: SqlitePool,
    sessions: dashmap::DashMap<i64, Arc<Mutex<ConnectionContext>>>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            next_down_tag: Mutex::new(0),
            db,
            sessions: dashmap::DashMap::new(),
        }
    }

    pub async fn reserve_down_tag(&self) -> u8 {
        let mut tag = self.next_down_tag.lock().await;
        let current = *tag & 0x7F;
        *tag = (*tag + 1) & 0x7F;
        current
    }

    pub fn get_connection_context(&self, player_id: i64) -> Option<Arc<Mutex<ConnectionContext>>> {
        self.sessions.get(&player_id).map(|v| Arc::clone(v.value()))
    }

    pub fn register_session(&self, player_id: i64, ctx: Arc<Mutex<ConnectionContext>>) {
        self.sessions.insert(player_id, ctx);
    }

    pub fn unregister_session(&self, player_id: i64) {
        self.sessions.remove(&player_id);
    }
}
