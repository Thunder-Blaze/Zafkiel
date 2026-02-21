use crate::database::Database;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, command};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProgressParams {
    pub media_id: i32,
    pub progress: i32,
    pub timestamp: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheMediaParams {
    pub media_data: serde_json::Value,
    pub extension_source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheUserParams {
    pub user_data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedImageInfo {
    pub id: i64,
    pub original_url: String,
    pub local_path: String,
    pub file_size: Option<i64>,
    pub cached_at: i64,
    pub last_accessed: i64,
}

// Simple database commands that will be handled by the frontend DatabaseService
// These are just pass-through commands for now

#[command]
pub async fn update_local_progress(
    db: State<'_, Database>,
    params: UpdateProgressParams,
) -> Result<(), String> {
    log::info!("Update local progress: {:?}", params);
    let conn = db.lock();
    let now = params.timestamp.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32
    });
    conn.execute(
        "INSERT INTO local_progress (media_id, progress, timestamp) VALUES (?1, ?2, ?3)
         ON CONFLICT(media_id) DO UPDATE SET progress = ?2, timestamp = ?3",
        params![params.media_id, params.progress, now],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn cache_media(db: State<'_, Database>, params: CacheMediaParams) -> Result<(), String> {
    log::info!("Cache media called");
    let conn = db.lock();
    let media_json = serde_json::to_string(&params.media_data)
        .map_err(|e| format!("Serialization error: {}", e))?;
    conn.execute(
        "INSERT INTO cached_media (media_data, extension_source) VALUES (?1, ?2)",
        params![media_json, params.extension_source],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn cache_user(db: State<'_, Database>, params: CacheUserParams) -> Result<(), String> {
    log::info!("Cache user called");
    let conn = db.lock();
    let user_json = serde_json::to_string(&params.user_data)
        .map_err(|e| format!("Serialization error: {}", e))?;
    conn.execute(
        "INSERT INTO cached_users (user_data) VALUES (?1)",
        params![user_json],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn add_to_recently_viewed(db: State<'_, Database>, media_id: i32) -> Result<(), String> {
    log::info!("Add to recently viewed: {}", media_id);
    let conn = db.lock();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    conn.execute(
        "INSERT INTO recently_viewed (media_id, viewed_at) VALUES (?1, ?2)",
        params![media_id, now],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn get_recently_viewed(
    db: State<'_, Database>,
    limit: Option<i32>,
) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Get recently viewed with limit: {:?}", limit);
    let conn = db.lock();
    let query = if let Some(lim) = limit {
        format!(
            "SELECT media_id, viewed_at FROM recently_viewed ORDER BY viewed_at DESC LIMIT {}",
            lim
        )
    } else {
        "SELECT media_id, viewed_at FROM recently_viewed ORDER BY viewed_at DESC".to_string()
    };
    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("Database error: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            let media_id: i32 = row.get(0)?;
            let viewed_at: i64 = row.get(1)?;
            Ok(serde_json::json!({"media_id": media_id, "viewed_at": viewed_at}))
        })
        .map_err(|e| format!("Database error: {}", e))?;
    let result = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(result)
}

#[command]
pub async fn search_cached_media(
    db: State<'_, Database>,
    query: String,
    media_type: Option<String>,
) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Search cached media: {} type: {:?}", query, media_type);
    let conn = db.lock();
    let sql = if media_type.is_some() {
        "SELECT media_data FROM cached_media WHERE media_data LIKE ?1 AND extension_source = ?2"
    } else {
        "SELECT media_data FROM cached_media WHERE media_data LIKE ?1"
    };
    let param_query = format!("%{}%", query);
    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| format!("Database error: {}", e))?;
    let closure = |row: &rusqlite::Row| {
        let media_json: String = row.get(0)?;
        Ok(serde_json::from_str(&media_json).unwrap_or(serde_json::Value::Null))
    };
    let rows = if let Some(ref mtype) = media_type {
        stmt.query_map(params![param_query, mtype], closure)
    } else {
        stmt.query_map(params![param_query], closure)
    };
    let result = rows
        .map_err(|e| format!("Database error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(result)
}

#[command]
pub async fn cleanup_cache(db: State<'_, Database>) -> Result<(), String> {
    log::info!("Cleanup cache called");
    let conn = db.lock();
    conn.execute("DELETE FROM cached_images", [])
        .map_err(|e| format!("Database error: {}", e))?;
    conn.execute("DELETE FROM cached_media", [])
        .map_err(|e| format!("Database error: {}", e))?;
    conn.execute("DELETE FROM recently_viewed", [])
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

// ============================================================================
// IMAGE CACHING DATABASE COMMANDS - FULLY IMPLEMENTED
// ============================================================================

/// Get cached image path by URL
#[command]
pub async fn get_cached_image_path(
    app: AppHandle,
    db: State<'_, Database>,
    url: String,
) -> Result<Option<String>, String> {
    log::debug!("[DB] get_cached_image_path called for: {}", url);

    let conn = db.lock();

    let result = conn.query_row(
        "SELECT local_path FROM cached_images WHERE original_url = ?1",
        params![url],
        |row| row.get::<_, String>(0),
    );

    match result {
        Ok(local_path) => {
            // Resolve absolute path
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data directory: {}", e))?;

            let cache_dir = app_data_dir.join("cache");
            let full_path = cache_dir.join(&local_path);
            let full_path_str = full_path.to_string_lossy().to_string();

            log::info!("[DB] Found cached image: {} -> {}", url, full_path_str);

            // Update last_accessed timestamp
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let _ = conn.execute(
                "UPDATE cached_images SET last_accessed = ?1 WHERE original_url = ?2",
                params![now, url],
            );

            Ok(Some(full_path_str))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log::debug!("[DB] Image not cached: {}", url);
            Ok(None)
        }
        Err(e) => {
            log::error!("[DB] Database error querying cached image: {}", e);
            Err(format!("Database error: {}", e))
        }
    }
}

/// Cache image with URL and local path
#[command]
pub async fn cache_image(
    app: AppHandle,
    db: State<'_, Database>,
    url: String,
    local_path: String,
) -> Result<i64, String> {
    log::info!("[DB] cache_image called: {} -> {}", url, local_path);

    let conn = db.lock();

    // Get file size if the file exists
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let cache_dir = app_data_dir.join("cache");
    let full_path = cache_dir.join(&local_path);

    log::debug!("[DB] Checking file at: {:?}", full_path);

    let file_size = if full_path.exists() {
        let size = std::fs::metadata(&full_path).map(|m| m.len() as i64).ok();
        log::debug!("[DB] File exists, size: {:?}", size);
        size
    } else {
        log::warn!("[DB] File does not exist at: {:?}", full_path);
        None
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Insert or update the cached image record
    log::debug!("[DB] Executing INSERT/UPDATE query");
    conn.execute(
        "INSERT INTO cached_images (original_url, local_path, file_size, cached_at, last_accessed)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(original_url) DO UPDATE SET
         local_path = ?2,
         file_size = ?3,
         last_accessed = ?5",
        params![url, local_path, file_size, now, now],
    )
    .map_err(|e| {
        log::error!("[DB] Failed to cache image in database: {}", e);
        format!("Database error: {}", e)
    })?;

    let id = conn.last_insert_rowid();
    log::info!(
        "[DB] Cached image successfully: {} -> {} (ID: {}, Size: {} bytes)",
        url,
        local_path,
        id,
        file_size.unwrap_or(0)
    );

    Ok(id)
}

/// Remove cached image from database
#[command]
pub async fn remove_cached_image(db: State<'_, Database>, url: String) -> Result<(), String> {
    let conn = db.lock();

    let rows_affected = conn
        .execute(
            "DELETE FROM cached_images WHERE original_url = ?1",
            params![url],
        )
        .map_err(|e| {
            log::error!("Failed to remove cached image from database: {}", e);
            format!("Database error: {}", e)
        })?;

    if rows_affected > 0 {
        log::info!("Removed cached image from database: {}", url);
    } else {
        log::warn!("Attempted to remove non-existent cached image: {}", url);
    }

    Ok(())
}

/// Get all cached images from database
#[command]
pub async fn get_all_cached_images(
    db: State<'_, Database>,
) -> Result<Vec<CachedImageInfo>, String> {
    let conn = db.lock();

    let mut stmt = conn
        .prepare(
            "SELECT id, original_url, local_path, file_size, cached_at, last_accessed
         FROM cached_images
         ORDER BY last_accessed DESC",
        )
        .map_err(|e| {
            log::error!("Failed to prepare query for cached images: {}", e);
            format!("Database error: {}", e)
        })?;

    let images = stmt
        .query_map([], |row| {
            Ok(CachedImageInfo {
                id: row.get(0)?,
                original_url: row.get(1)?,
                local_path: row.get(2)?,
                file_size: row.get(3)?,
                cached_at: row.get(4)?,
                last_accessed: row.get(5)?,
            })
        })
        .map_err(|e| {
            log::error!("Failed to query cached images: {}", e);
            format!("Database error: {}", e)
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            log::error!("Failed to collect cached images: {}", e);
            format!("Database error: {}", e)
        })?;

    log::info!("Retrieved {} cached images from database", images.len());
    Ok(images)
}
