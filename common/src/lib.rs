use once_cell::sync::Lazy;
use std::path::PathBuf;

pub mod time;

pub const HOST: &str = "127.0.0.1";
pub const DNS: &str = "localhost";
pub const HTTPSERVER_PORT: u16 = 21000;
pub const GAMESERVER_PORT: u16 = 23301;

pub const CERT_DIR: &str = "./cert";
pub const KEY_FILE_PATH: &str = "./cert/localhost.key";
pub const CERT_FILE_PATH: &str = "./cert/localhost.crt";

// todo: user regist n allat shit
pub const USER_ID: u64 = 1337;
pub const USERNAME: &str = "Yon";

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt().init();
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_time_ms_u128() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

pub static DATA_DIRECTORY: Lazy<PathBuf> = Lazy::new(|| {
    // 1. Explicit override
    if let Ok(dir) = std::env::var("DATA_DIR") {
        let p = PathBuf::from(dir);
        if p.exists() {
            return p;
        }
    }

    // Relative path from repo / binary
    let rel_data = data_relative_path();

    // 2. From current working directory
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join(&rel_data);
        if p.exists() {
            return p;
        }
    }

    // 3. From executable directory
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join(&rel_data);
            if p.exists() {
                return p;
            }
        }
    }

    // 4. Dev fallback (cargo run)
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let p = PathBuf::from(manifest).join(&rel_data);
        if p.exists() {
            return p;
        }
    }

    // 5. Last resort (non-existent but deterministic)
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(&rel_data)
});

fn data_relative_path() -> PathBuf {
    ["..", "..", "data", "static"].iter().collect()
}

pub fn time_ms_u64() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
