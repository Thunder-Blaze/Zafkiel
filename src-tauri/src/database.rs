use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite_migration::{Migrations, M};
use std::path::PathBuf;

/// Database connection pool wrapper with versioned migrations.
///
/// Uses R2D2 connection pool (up to 8 concurrent connections) and WAL journal
/// mode for better concurrent read performance.
#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

/// Build the migration list. Each entry is a forward migration; order is
/// significant and must never change.
fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        // 001 — Image cache (was created ad-hoc before; now managed)
        M::up(
            "CREATE TABLE IF NOT EXISTS cached_images (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                original_url TEXT    NOT NULL UNIQUE,
                local_path   TEXT    NOT NULL UNIQUE,
                file_size    INTEGER,
                cached_at    INTEGER NOT NULL,
                last_accessed INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_cached_images_url
                ON cached_images(original_url);",
        ),
        // 002 — Normalized media cache (replaces JSON-blob table)
        M::up(
            "CREATE TABLE IF NOT EXISTS cached_media (
                id            INTEGER PRIMARY KEY,
                anilist_id    INTEGER NOT NULL UNIQUE,
                title_romaji  TEXT,
                title_english TEXT,
                title_native  TEXT,
                media_type    TEXT    NOT NULL,
                format        TEXT,
                status        TEXT,
                cover_large   TEXT,
                cover_medium  TEXT,
                banner_image  TEXT,
                avg_score     INTEGER,
                mean_score    INTEGER,
                episodes      INTEGER,
                chapters      INTEGER,
                volumes       INTEGER,
                season        TEXT,
                season_year   INTEGER,
                genres        TEXT,
                is_adult      INTEGER DEFAULT 0,
                site_url      TEXT,
                cached_at     INTEGER NOT NULL,
                last_accessed INTEGER NOT NULL,
                full_json     TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_cached_media_type
                ON cached_media(media_type);
            CREATE INDEX IF NOT EXISTS idx_cached_media_score
                ON cached_media(avg_score);",
        ),
        // 003 — Recently viewed (with proper deduplication)
        M::up(
            "CREATE TABLE IF NOT EXISTS recently_viewed (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                media_id   INTEGER NOT NULL UNIQUE,
                media_type TEXT,
                title      TEXT,
                cover_url  TEXT,
                viewed_at  INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_recently_viewed_time
                ON recently_viewed(viewed_at DESC);",
        ),
        // 004 — User profile cache
        M::up(
            "CREATE TABLE IF NOT EXISTS cached_users (
                id           INTEGER PRIMARY KEY,
                anilist_id   INTEGER NOT NULL UNIQUE,
                name         TEXT    NOT NULL,
                avatar_large TEXT,
                banner_image TEXT,
                about        TEXT,
                site_url     TEXT,
                cached_at    INTEGER NOT NULL,
                full_json    TEXT
            );",
        ),
        // 005 — Local progress / media list (offline-first, synced to AniList)
        M::up(
            "CREATE TABLE IF NOT EXISTS local_progress (
                id               INTEGER PRIMARY KEY AUTOINCREMENT,
                media_id         INTEGER NOT NULL UNIQUE,
                entry_id         INTEGER,
                media_type       TEXT    NOT NULL,
                status           TEXT    NOT NULL,
                progress         INTEGER NOT NULL DEFAULT 0,
                progress_volumes INTEGER DEFAULT 0,
                score            REAL    DEFAULT 0,
                notes            TEXT,
                private          INTEGER DEFAULT 0,
                repeat           INTEGER DEFAULT 0,
                started_year     INTEGER,
                started_month    INTEGER,
                started_day      INTEGER,
                finished_year    INTEGER,
                finished_month   INTEGER,
                finished_day     INTEGER,
                updated_at       INTEGER NOT NULL,
                created_at       INTEGER NOT NULL,
                synced_at        INTEGER
            );
            CREATE INDEX IF NOT EXISTS idx_local_progress_status
                ON local_progress(status);
            CREATE INDEX IF NOT EXISTS idx_local_progress_type
                ON local_progress(media_type);",
        ),
        // 006 — Notification cache
        M::up(
            "CREATE TABLE IF NOT EXISTS notifications_cache (
                id              INTEGER PRIMARY KEY,
                notification_id INTEGER NOT NULL UNIQUE,
                type            TEXT    NOT NULL,
                is_read         INTEGER DEFAULT 0,
                created_at      INTEGER NOT NULL,
                cached_at       INTEGER NOT NULL,
                full_json       TEXT    NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_notifications_unread
                ON notifications_cache(is_read);
            CREATE INDEX IF NOT EXISTS idx_notifications_time
                ON notifications_cache(created_at DESC);",
        ),
        // 007 — Activity feed cache
        M::up(
            "CREATE TABLE IF NOT EXISTS activities_cache (
                id          INTEGER PRIMARY KEY,
                activity_id INTEGER NOT NULL UNIQUE,
                type        TEXT    NOT NULL,
                user_id     INTEGER,
                created_at  INTEGER NOT NULL,
                cached_at   INTEGER NOT NULL,
                full_json   TEXT    NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_activities_user
                ON activities_cache(user_id);
            CREATE INDEX IF NOT EXISTS idx_activities_time
                ON activities_cache(created_at DESC);",
        ),
        // 008 — Reviews cache
        M::up(
            "CREATE TABLE IF NOT EXISTS reviews_cache (
                id           INTEGER PRIMARY KEY,
                review_id    INTEGER NOT NULL UNIQUE,
                media_id     INTEGER NOT NULL,
                user_id      INTEGER NOT NULL,
                score        INTEGER,
                rating       INTEGER,
                rating_amount INTEGER,
                created_at   INTEGER NOT NULL,
                cached_at    INTEGER NOT NULL,
                full_json    TEXT    NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_reviews_media
                ON reviews_cache(media_id);
            CREATE INDEX IF NOT EXISTS idx_reviews_user
                ON reviews_cache(user_id);",
        ),
    ])
}

impl Database {
    /// Open the database at `db_path`, configure WAL mode + pragmas, run all
    /// pending migrations, and return a ready-to-use pool.
    pub fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = SqliteConnectionManager::file(&db_path).with_init(|conn| {
            conn.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous   = NORMAL;
                 PRAGMA foreign_keys  = ON;
                 PRAGMA cache_size    = -20000;
                 PRAGMA temp_store    = MEMORY;",
            )?;
            Ok(())
        });

        let pool = Pool::builder()
            .max_size(8)
            .build(manager)
            .map_err(|e| format!("Failed to build DB pool: {e}"))?;

        // Run migrations on a single connection synchronously at startup
        {
            let mut conn = pool
                .get()
                .map_err(|e| format!("Failed to get DB connection: {e}"))?;
            migrations()
                .to_latest(&mut *conn)
                .map_err(|e| format!("Database migration failed: {e}"))?;
        }

        log::info!("[Database] Initialized at {}", db_path.display());

        Ok(Database { pool })
    }

    /// Get a pooled connection. Panics if the pool is exhausted (shouldn't
    /// happen with max_size = 8 for our use-case).
    pub fn get(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.pool
            .get()
            .expect("[Database] Pool exhausted — this should never happen")
    }
}
