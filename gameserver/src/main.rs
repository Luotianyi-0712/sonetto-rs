use crate::state::{AppState, ConnectionContext};
use byteorder::{BE, ByteOrder};
use bytes::BytesMut;
use common::{GAMESERVER_PORT, HOST, init_tracing};
use database::{DatabaseSettings, connect_to, run_migrations};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{error, info};

mod cmd;
mod data_loader;
mod error;
mod handler;
mod packet;
mod state;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let data_path = get_data_path()?;
    info!("Found data path: {}", data_path.display());

    let settings = DatabaseSettings::default();

    let db = connect_to(&settings).await.map_err(|e| {
        eprintln!("Database connection error: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    run_migrations(&db).await.map_err(|e| {
        eprintln!("Migration error: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    //excel db loader
    info!("Loading game data...");
    data::exceldb::init(data_path.to_str().unwrap()).map_err(|e| {
        error!("Failed to load game data: {:#}", e);
        e
    })?;
    info!("Game data loaded");

    // Verify database connection
    let version: (String,) = sqlx::query_as("SELECT sqlite_version()")
        .fetch_one(&db)
        .await?;
    tracing::info!("SQLite version: {}", version.0);

    let state = Arc::new(AppState::new(db));

    let addr = format!("{}:{}", HOST, GAMESERVER_PORT);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Listening on tcp://{}", &addr);

    loop {
        let (raw_socket, client) = listener.accept().await?;
        tracing::info!("New client connected: {:?}", client);

        let state = state.clone();
        let socket = Arc::new(Mutex::new(raw_socket));

        tokio::spawn(async move {
            let ctx = Arc::new(Mutex::new(ConnectionContext::new(
                socket.clone(),
                state.clone(),
            )));

            let result = handle_client(ctx).await;

            if let Err(e) = result {
                tracing::error!("Client handler error: {e}");
            }
        });
    }
}

async fn handle_client(
    ctx: Arc<Mutex<ConnectionContext>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Peek to get initial packet size
    let mut peek_buf = [0; 4];
    {
        let ctx_guard = ctx.lock().await;
        let socket = ctx_guard.socket.lock().await;
        let peek_cnt = socket.peek(&mut peek_buf).await?;

        if peek_cnt != 4 {
            return Err("Incomplete peek".into());
        }
    }

    let packet_size = BE::read_i32(&peek_buf[..]) as usize + 4;
    let _buffer = BytesMut::with_capacity(packet_size);

    loop {
        // Read packet header
        let mut header = [0u8; 4];
        {
            let ctx_guard = ctx.lock().await;
            let mut socket = ctx_guard.socket.lock().await;
            if let Err(e) = socket.read_exact(&mut header).await {
                tracing::debug!("Client disconnected: {e}");
                break;
            }
        }

        // Read packet body
        let packet_len = BE::read_i32(&header) as usize;
        let mut buffer = vec![0u8; packet_len];
        {
            let ctx_guard = ctx.lock().await;
            let mut socket = ctx_guard.socket.lock().await;
            if let Err(e) = socket.read_exact(&mut buffer).await {
                tracing::warn!("Failed to read packet body ({} bytes): {e}", packet_len);
                break;
            }
        }

        // Reconstruct full packet
        let mut packet = Vec::with_capacity(4 + packet_len);
        packet.extend_from_slice(&header);
        packet.extend_from_slice(&buffer);

        // Dispatch command
        if let Err(e) = handler::dispatch_command(ctx.clone(), &packet[..]).await {
            tracing::error!("Dispatch error: {e}");
            break;
        }

        // Flush any queued responses
        {
            let mut ctx_guard = ctx.lock().await;
            if let Err(e) = ctx_guard.flush_send_queue().await {
                tracing::error!("Failed to flush send queue: {e}");
                break;
            }
        }
    }

    // Cleanup on disconnect
    let ctx_guard = ctx.lock().await;
    if let Some(player_id) = ctx_guard.player_id {
        tracing::info!("Player {} disconnected", player_id);
        ctx_guard.state.unregister_session(player_id);
    }

    Ok(())
}

fn get_data_path() -> anyhow::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_data_path = current_dir.join("data").join("excel2json");
    if current_data_path.exists() {
        return Ok(current_data_path);
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_data_path = exe_dir.join("data").join("excel2json");
            if exe_data_path.exists() {
                return Ok(exe_data_path);
            }
        }
    }

    let base_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        let mut current = std::env::current_dir()?;
        loop {
            if current.join("Cargo.toml").exists() {
                break;
            }
            if !current.pop() {
                return Err(anyhow::anyhow!("Could not find project root"));
            }
        }
        current
    };

    let data_path = base_dir.join("data").join("excel2json");
    if !data_path.exists() {
        return Err(anyhow::anyhow!(
            "Data directory not found. Checked:\n  - {}\n  - Executable directory\n  - Project root: {}",
            current_data_path.display(),
            data_path.display()
        ));
    }

    Ok(data_path)
}
