use tauri::command;
use serde::{Deserialize, Serialize};

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

// Simple database commands that will be handled by the frontend DatabaseService
// These are just pass-through commands for now

#[command]
pub async fn update_local_progress(params: UpdateProgressParams) -> Result<(), String> {
    // This will be implemented later when we have proper database connection
    log::info!("Update local progress: {:?}", params);
    Ok(())
}

#[command]
pub async fn cache_media(params: CacheMediaParams) -> Result<(), String> {
    log::info!("Cache media called");
    Ok(())
}

#[command]
pub async fn cache_user(params: CacheUserParams) -> Result<(), String> {
    log::info!("Cache user called");
    Ok(())
}

#[command]
pub async fn add_to_recently_viewed(media_id: i32) -> Result<(), String> {
    log::info!("Add to recently viewed: {}", media_id);
    Ok(())
}

#[command]
pub async fn get_recently_viewed(limit: Option<i32>) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Get recently viewed with limit: {:?}", limit);
    Ok(vec![])
}

#[command]
pub async fn search_cached_media(query: String, media_type: Option<String>) -> Result<Vec<serde_json::Value>, String> {
    log::info!("Search cached media: {} type: {:?}", query, media_type);
    Ok(vec![])
}

#[command]
pub async fn cleanup_cache() -> Result<(), String> {
    log::info!("Cleanup cache called");
    Ok(())
}

#[command]
pub async fn get_all_cached_images() -> Result<Vec<serde_json::Value>, String> {
    log::info!("Get all cached images called");
    Ok(vec![])
}