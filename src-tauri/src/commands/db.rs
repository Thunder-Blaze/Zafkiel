use crate::database::Database;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, command};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgressParams {
    pub media_id: i32,
    pub entry_id: Option<i32>,
    pub media_type: Option<String>,
    pub status: Option<String>,
    pub progress: i32,
    pub progress_volumes: Option<i32>,
    pub score: Option<f64>,
    pub notes: Option<String>,
    pub private: Option<bool>,
    pub repeat: Option<i32>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub timestamp: Option<i64>,
    pub update_mode: Option<String>,
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
pub struct RecentlyViewedParams {
    pub media_id: i32,
    pub title: Option<String>,
    pub cover_url: Option<String>,
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

// ============================================================================
// Helpers
// ============================================================================

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn json_str(v: &serde_json::Value, key: &str) -> Option<String> {
    v.get(key).and_then(|s| s.as_str()).map(|s| s.to_owned())
}

fn json_i64(v: &serde_json::Value, key: &str) -> Option<i64> {
    v.get(key).and_then(|n| n.as_i64())
}

fn json_f64(v: &serde_json::Value, key: &str) -> Option<f64> {
    v.get(key).and_then(|n| n.as_f64())
}

fn json_bool(v: &serde_json::Value, key: &str) -> Option<bool> {
    v.get(key).and_then(|b| b.as_bool())
}

// ============================================================================
// Progress management
// ============================================================================

#[command]
pub async fn update_local_progress(
    db: State<'_, Database>,
    params: UpdateProgressParams,
) -> Result<(), String> {
    log::info!("Update local progress: media_id={}", params.media_id);
    let conn = db.get();
    let now = params.timestamp.unwrap_or_else(now_secs);
    conn.execute(
        "INSERT INTO local_progress (
            media_id, entry_id, media_type, status, progress, progress_volumes,
            score, notes, private, repeat, started_at, finished_at,
            updated_at, created_at, synced_at, update_mode
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?13, NULL, ?14)
         ON CONFLICT(media_id) DO UPDATE SET
            entry_id         = excluded.entry_id,
            media_type       = excluded.media_type,
            status           = excluded.status,
            progress         = excluded.progress,
            progress_volumes = excluded.progress_volumes,
            score            = excluded.score,
            notes            = excluded.notes,
            private          = excluded.private,
            repeat           = excluded.repeat,
            started_at       = excluded.started_at,
            finished_at      = excluded.finished_at,
            updated_at       = excluded.updated_at,
            update_mode      = COALESCE(excluded.update_mode, local_progress.update_mode),
            synced_at        = NULL",
        params![
            params.media_id,
            params.entry_id,
            params.media_type,
            params.status,
            params.progress,
            params.progress_volumes,
            params.score,
            params.notes,
            params.private,
            params.repeat,
            params.started_at,
            params.finished_at,
            now,
            params.update_mode,
        ],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn get_local_update_mode(
    db: State<'_, Database>,
    media_id: i32,
) -> Result<Option<String>, String> {
    let conn = db.get();
    let result = conn.query_row(
        "SELECT update_mode FROM local_progress WHERE media_id = ?1",
        params![media_id],
        |row| row.get::<_, Option<String>>(0),
    );

    match result {
        Ok(mode) => Ok(mode),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// ============================================================================
// Media caching
// ============================================================================

#[command]
pub async fn cache_media(db: State<'_, Database>, params: CacheMediaParams) -> Result<(), String> {
    let m = &params.media_data;
    let anilist_id = json_i64(m, "id").ok_or("media_data missing id")? as i32;
    log::info!("Cache media: anilist_id={}", anilist_id);

    let title_obj = m.get("title");
    let title_romaji = title_obj
        .and_then(|t| t.get("romaji"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());
    let title_english = title_obj
        .and_then(|t| t.get("english"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());
    let title_native = title_obj
        .and_then(|t| t.get("native"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    let cover_large = m
        .get("coverImage")
        .and_then(|ci| ci.get("large"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());
    let cover_medium = m
        .get("coverImage")
        .and_then(|ci| ci.get("medium"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    let genres: Option<String> = m.get("genres").and_then(|g| {
        if g.is_array() {
            serde_json::to_string(g).ok()
        } else {
            None
        }
    });

    let full_json = serde_json::to_string(m).map_err(|e| format!("Serialization error: {}", e))?;
    let now = now_secs();

    let conn = db.get();
    conn.execute(
        "INSERT INTO cached_media (
            anilist_id, title_romaji, title_english, title_native,
            media_type, format, status,
            cover_large, cover_medium, banner_image,
            avg_score, mean_score, episodes, chapters, volumes,
            season, season_year, genres, is_adult, site_url,
            cached_at, last_accessed, full_json
         ) VALUES (
            ?1,  ?2,  ?3,  ?4,
            ?5,  ?6,  ?7,
            ?8,  ?9,  ?10,
            ?11, ?12, ?13, ?14, ?15,
            ?16, ?17, ?18, ?19, ?20,
            ?21, ?21, ?22
         )
         ON CONFLICT(anilist_id) DO UPDATE SET
            title_romaji   = excluded.title_romaji,
            title_english  = excluded.title_english,
            title_native   = excluded.title_native,
            media_type     = excluded.media_type,
            format         = excluded.format,
            status         = excluded.status,
            cover_large    = excluded.cover_large,
            cover_medium   = excluded.cover_medium,
            banner_image   = excluded.banner_image,
            avg_score      = excluded.avg_score,
            mean_score     = excluded.mean_score,
            episodes       = excluded.episodes,
            chapters       = excluded.chapters,
            volumes        = excluded.volumes,
            season         = excluded.season,
            season_year    = excluded.season_year,
            genres         = excluded.genres,
            is_adult       = excluded.is_adult,
            site_url       = excluded.site_url,
            last_accessed  = excluded.last_accessed,
            full_json      = excluded.full_json",
        params![
            anilist_id,
            title_romaji,
            title_english,
            title_native,
            json_str(m, "type"),
            json_str(m, "format"),
            json_str(m, "status"),
            cover_large,
            cover_medium,
            json_str(m, "bannerImage"),
            json_f64(m, "averageScore"),
            json_f64(m, "meanScore"),
            json_i64(m, "episodes"),
            json_i64(m, "chapters"),
            json_i64(m, "volumes"),
            json_str(m, "season"),
            json_i64(m, "seasonYear"),
            genres,
            json_bool(m, "isAdult"),
            json_str(m, "siteUrl"),
            now,
            full_json,
        ],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

// ============================================================================
// User caching
// ============================================================================

#[command]
pub async fn cache_user(db: State<'_, Database>, params: CacheUserParams) -> Result<(), String> {
    let u = &params.user_data;
    let anilist_id = json_i64(u, "id").ok_or("user_data missing id")? as i32;
    log::info!("Cache user: anilist_id={}", anilist_id);

    let name = json_str(u, "name");
    let avatar = u
        .get("avatar")
        .and_then(|a| a.get("large").or_else(|| a.get("medium")))
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());
    let banner = json_str(u, "bannerImage");
    let about = json_str(u, "about");
    let site_url = json_str(u, "siteUrl");
    let full_json = serde_json::to_string(u).map_err(|e| format!("Serialization error: {}", e))?;
    let now = now_secs();

    let conn = db.get();
    conn.execute(
        "INSERT INTO cached_users (
            anilist_id, name, avatar, banner, about, site_url,
            cached_at, last_accessed, full_json
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7, ?8)
         ON CONFLICT(anilist_id) DO UPDATE SET
            name          = excluded.name,
            avatar        = excluded.avatar,
            banner        = excluded.banner,
            about         = excluded.about,
            site_url      = excluded.site_url,
            last_accessed = excluded.last_accessed,
            full_json     = excluded.full_json",
        params![
            anilist_id, name, avatar, banner, about, site_url, now, full_json
        ],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

// ============================================================================
// Recently viewed
// ============================================================================

#[command]
pub async fn add_to_recently_viewed(
    db: State<'_, Database>,
    params: RecentlyViewedParams,
) -> Result<(), String> {
    log::info!("Add to recently viewed: {}", params.media_id);
    let conn = db.get();
    let now = now_secs();
    conn.execute(
        "INSERT INTO recently_viewed (media_id, title, cover_url, viewed_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(media_id) DO UPDATE SET
            title     = excluded.title,
            cover_url = excluded.cover_url,
            viewed_at = excluded.viewed_at",
        params![params.media_id, params.title, params.cover_url, now],
    )
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[command]
pub async fn get_recently_viewed(
    db: State<'_, Database>,
    limit: Option<i32>,
) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Get recently viewed, limit={:?}", limit);
    let conn = db.get();
    let lim = limit.unwrap_or(20);
    let mut stmt = conn
        .prepare(
            "SELECT media_id, title, cover_url, viewed_at
             FROM recently_viewed
             ORDER BY viewed_at DESC
             LIMIT ?1",
        )
        .map_err(|e| format!("Database error: {}", e))?;
    let rows = stmt
        .query_map(params![lim], |row| {
            let media_id: i32 = row.get(0)?;
            let title: Option<String> = row.get(1)?;
            let cover_url: Option<String> = row.get(2)?;
            let viewed_at: i64 = row.get(3)?;
            Ok(serde_json::json!({
                "media_id": media_id,
                "title": title,
                "cover_url": cover_url,
                "viewed_at": viewed_at
            }))
        })
        .map_err(|e| format!("Database error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(rows)
}

// ============================================================================
// Media search
// ============================================================================

#[command]
pub async fn search_cached_media(
    db: State<'_, Database>,
    query: String,
    media_type: Option<String>,
) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Search cached media: query={} type={:?}", query, media_type);
    let conn = db.get();
    let like = format!("%{}%", query);
    let sql_with_type = "SELECT full_json FROM cached_media \
         WHERE (title_romaji LIKE ?1 OR title_english LIKE ?1 OR title_native LIKE ?1) \
           AND media_type = ?2";
    let sql_no_type = "SELECT full_json FROM cached_media \
         WHERE title_romaji LIKE ?1 OR title_english LIKE ?1 OR title_native LIKE ?1";

    let mut stmt = conn
        .prepare(if media_type.is_some() {
            sql_with_type
        } else {
            sql_no_type
        })
        .map_err(|e| format!("Database error: {}", e))?;

    let closure = |row: &rusqlite::Row| {
        let json: String = row.get(0)?;
        Ok(serde_json::from_str(&json).unwrap_or(serde_json::Value::Null))
    };

    let result = if let Some(ref mtype) = media_type {
        stmt.query_map(params![like, mtype], closure)
    } else {
        stmt.query_map(params![like], closure)
    }
    .map_err(|e| format!("Database error: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(result)
}

// ============================================================================
// Cleanup
// ============================================================================

#[command]
pub async fn cleanup_cache(db: State<'_, Database>) -> Result<(), String> {
    log::info!("Cleanup cache called");
    let conn = db.get();
    conn.execute("DELETE FROM cached_images", [])
        .map_err(|e| format!("Database error: {}", e))?;
    conn.execute("DELETE FROM cached_media", [])
        .map_err(|e| format!("Database error: {}", e))?;
    conn.execute("DELETE FROM recently_viewed", [])
        .map_err(|e| format!("Database error: {}", e))?;
    conn.execute("DELETE FROM cached_users", [])
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

// ============================================================================
// Image caching (DB side)
// ============================================================================

#[command]
pub async fn get_cached_image_path(
    app: AppHandle,
    db: State<'_, Database>,
    url: String,
) -> Result<Option<String>, String> {
    log::debug!("[DB] get_cached_image_path: {}", url);
    let conn = db.get();

    let result = conn.query_row(
        "SELECT local_path FROM cached_images WHERE original_url = ?1",
        params![url],
        |row| row.get::<_, String>(0),
    );

    match result {
        Ok(local_path) => {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data directory: {}", e))?;
            let full = app_data_dir.join("cache").join(&local_path);
            let full_str = full.to_string_lossy().to_string();

            let now = now_secs();
            let _ = conn.execute(
                "UPDATE cached_images SET last_accessed = ?1 WHERE original_url = ?2",
                params![now, url],
            );
            Ok(Some(full_str))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[command]
pub async fn cache_image(
    app: AppHandle,
    db: State<'_, Database>,
    url: String,
    local_path: String,
) -> Result<i64, String> {
    log::info!("[DB] cache_image: {} -> {}", url, local_path);
    let conn = db.get();

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let full = app_data_dir.join("cache").join(&local_path);
    let file_size = if full.exists() {
        std::fs::metadata(&full).map(|m| m.len() as i64).ok()
    } else {
        None
    };

    let now = now_secs();
    conn.execute(
        "INSERT INTO cached_images (original_url, local_path, file_size, cached_at, last_accessed)
         VALUES (?1, ?2, ?3, ?4, ?4)
         ON CONFLICT(original_url) DO UPDATE SET
             local_path    = excluded.local_path,
             file_size     = excluded.file_size,
             last_accessed = excluded.last_accessed",
        params![url, local_path, file_size, now],
    )
    .map_err(|e| format!("Database error: {}", e))?;

    let id = conn.last_insert_rowid();
    log::info!(
        "[DB] Cached image (ID={}): {} ({} bytes)",
        id,
        url,
        file_size.unwrap_or(0)
    );
    Ok(id)
}

#[command]
pub async fn remove_cached_image(db: State<'_, Database>, url: String) -> Result<(), String> {
    let conn = db.get();
    let rows = conn
        .execute(
            "DELETE FROM cached_images WHERE original_url = ?1",
            params![url],
        )
        .map_err(|e| format!("Database error: {}", e))?;
    if rows > 0 {
        log::info!("[DB] Removed cached image: {}", url);
    }
    Ok(())
}

#[command]
pub async fn get_all_cached_images(
    db: State<'_, Database>,
) -> Result<Vec<CachedImageInfo>, String> {
    let conn = db.get();
    let mut stmt = conn
        .prepare(
            "SELECT id, original_url, local_path, file_size, cached_at, last_accessed
             FROM cached_images
             ORDER BY last_accessed DESC",
        )
        .map_err(|e| format!("Database error: {}", e))?;

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
        .map_err(|e| format!("Database error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Database error: {}", e))?;

    log::info!("[DB] Retrieved {} cached images", images.len());
    Ok(images)
}
