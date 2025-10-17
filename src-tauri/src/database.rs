use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Database connection wrapper with thread-safe access
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Initialize database connection and create tables if needed
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create cached_images table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cached_images (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_url TEXT NOT NULL UNIQUE,
                local_path TEXT NOT NULL UNIQUE,
                file_size INTEGER,
                cached_at INTEGER NOT NULL,
                last_accessed INTEGER NOT NULL
            )",
            [],
        )?;

        // Create index for faster lookups
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_cached_images_url
             ON cached_images(original_url)",
            [],
        )?;

        log::info!("Database initialized successfully");

        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get database connection (locked)
    pub fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}
