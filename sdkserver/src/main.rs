use common::{HOST, HTTPSERVER_PORT, init_tracing};
use database::{DatabaseSettings, connect_to, run_migrations};
use gameserver::state::AppState as GameState;
use reqwest::Client;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info};

mod handlers;
mod middleware;
mod models;
use middleware::crypto::sdk_encryption;
use middleware::logging::full_logger;

#[derive(Clone)]
pub struct SdkState {
    pub http_client: Client,
}

#[derive(Clone)]
pub struct AppState {
    pub sdk: SdkState,
    pub game: Arc<GameState>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let data_path = get_data_path()?;

    // Initialize database
    let settings = DatabaseSettings::default();
    let db = connect_to(&settings)
        .await
        .expect("Failed to connect to database");
    run_migrations(&db).await.expect("Failed to run migrations");

    info!("Loading game data...");
    data::exceldb::init(data_path.to_str().unwrap()).map_err(|e| {
        error!("Failed to load game data: {:#}", e);
        e
    })?;
    info!("Game data loaded");

    let addr: SocketAddr = format!("{}:{}", HOST, HTTPSERVER_PORT).parse().unwrap();

    let state = AppState {
        sdk: SdkState {
            http_client: Client::new(),
        },
        game: Arc::new(GameState::new(db)),
    };

    let with_encryption = {
        handlers::router::account_router()
            .layer(axum::middleware::from_fn(full_logger))
            .layer(axum::middleware::from_fn(sdk_encryption))
    };

    let without_encryption = {
        handlers::router::game_router()
            .merge(handlers::router::jsp_router())
            .merge(handlers::router::index_router())
            .layer(axum::middleware::from_fn(full_logger))
    };

    let app = with_encryption.merge(without_encryption).with_state(state); // Pass AppState directly, not Arc<AppState>

    tracing::info!("HTTP Server listening on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

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
