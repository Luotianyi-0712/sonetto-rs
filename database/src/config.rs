use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub db_name: String,
}

impl fmt::Display for DatabaseSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sqlite://{}", &self.db_name)
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        let db_path = get_database_path();

        DatabaseSettings {
            db_name: db_path.to_string_lossy().to_string(),
        }
    }
}

/// Get database path relative to the executable location
pub fn get_database_path() -> PathBuf {
    // Get the directory where the executable is running from
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // Database is in db/ subdirectory relative to exe
    exe_dir.join("db").join("sonetto.db")
}
