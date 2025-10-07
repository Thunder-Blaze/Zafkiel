use crate::anilist::{AniListResponse, AniListService};
use crate::commands::ConfigState;
use anilist_moe::{enums::media::MediaSeason, objects::{media::Media, user::User}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Shared state for AniList service
pub type AniListState = Arc<AniListService>;

/// Helper to get or create AniList service with current token
async fn get_service(
    config: &State<'_, ConfigState>,
    service: &State<'_, AniListState>,
) -> Result<(), String> {
    // Get token from config
    let token = config.get_anilist_token().map_err(|e| e.to_string())?;

    // Update service token
    service.set_token(token);

    Ok(())
}

// ============================================================================
// Anime Commands
// ============================================================================

/// Search for anime
#[tauri::command]
pub async fn search_anime(
    query: String,
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    log::info!("Command: search_anime called with query='{}', page={:?}, per_page={:?}", query, page, per_page);
    get_service(&config, &service).await?;
    let result = service.search_anime(&query, page, per_page).await;
    match &result {
        Ok(media) => log::info!("Command: search_anime succeeded with {} items", media.len()),
        Err(e) => log::error!("Command: search_anime failed: {:?}", e),
    }
    Ok(result.into())
}

/// Get anime by ID
#[tauri::command]
pub async fn get_anime_by_id(
    id: i32,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    get_service(&config, &service).await?;
    let result = service.get_anime_by_id(id).await;
    Ok(result.into())
}

/// Get trending anime
#[tauri::command]
pub async fn get_trending_anime(
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    log::info!("Command: get_trending_anime called with page={:?}, per_page={:?}", page, per_page);
    get_service(&config, &service).await?;
    let result = service.get_trending_anime(page, per_page).await;
    match &result {
        Ok(media) => {
            log::info!("Command: get_trending_anime succeeded with {} items", media.len());
            // Try to serialize and log the first item for debugging
            if let Some(first) = media.first() {
                match serde_json::to_string(first) {
                    Ok(json) => log::info!("First anime serialized successfully: {} bytes", json.len()),
                    Err(e) => log::error!("Failed to serialize first anime: {}", e),
                }
            }
        }
        Err(e) => log::error!("Command: get_trending_anime failed: {:?}", e),
    }
    let response: AniListResponse<Vec<Media>> = result.into();
    
    // Try to serialize the whole response
    match serde_json::to_string(&response) {
        Ok(json) => log::info!("Response serialized successfully: {} bytes", json.len()),
        Err(e) => log::error!("Failed to serialize response: {}", e),
    }
    
    Ok(response)
}

/// Get popular anime
#[tauri::command]
pub async fn get_popular_anime(
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    get_service(&config, &service).await?;
    let result = service.get_popular_anime(page, per_page).await;
    Ok(result.into())
}

/// Params for seasonal anime
#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonalAnimeParams {
    pub season: String, // "WINTER", "SPRING", "SUMMER", "FALL"
    pub year: i32,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Get seasonal anime
#[tauri::command]
pub async fn get_seasonal_anime(
    params: SeasonalAnimeParams,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    get_service(&config, &service).await?;

    // Parse season string
    let season = match params.season.to_uppercase().as_str() {
        "WINTER" => MediaSeason::Winter,
        "SPRING" => MediaSeason::Spring,
        "SUMMER" => MediaSeason::Summer,
        "FALL" | "AUTUMN" => MediaSeason::Fall,
        _ => return Ok(AniListResponse::error("Invalid season".to_string())),
    };

    let result = service
        .get_seasonal_anime(season, params.year, params.page, params.per_page)
        .await;
    Ok(result.into())
}

// ============================================================================
// Manga Commands
// ============================================================================

/// Search for manga
#[tauri::command]
pub async fn search_manga(
    query: String,
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    get_service(&config, &service).await?;
    let result = service.search_manga(&query, page, per_page).await;
    Ok(result.into())
}

/// Get manga by ID
#[tauri::command]
pub async fn get_manga_by_id(
    id: i32,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    get_service(&config, &service).await?;
    let result = service.get_manga_by_id(id).await;
    Ok(result.into())
}

/// Get trending manga
#[tauri::command]
pub async fn get_trending_manga(
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    get_service(&config, &service).await?;
    let result = service.get_trending_manga(page, per_page).await;
    Ok(result.into())
}

/// Get popular manga
#[tauri::command]
pub async fn get_popular_manga(
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<Media>>, String> {
    get_service(&config, &service).await?;
    let result = service.get_popular_manga(page, per_page).await;
    Ok(result.into())
}

// ============================================================================
// User Commands
// ============================================================================

/// Get current authenticated user
#[tauri::command]
pub async fn get_current_user(
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<User>, String> {
    get_service(&config, &service).await?;
    let result = service.get_current_user().await;
    Ok(result.into())
}

/// Get user by ID
#[tauri::command]
pub async fn get_user_by_id(
    id: i32,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<User>, String> {
    get_service(&config, &service).await?;
    let result = service.get_user_by_id(id).await;
    Ok(result.into())
}

/// Get user by name
#[tauri::command]
pub async fn get_user_by_name(
    name: String,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<User>, String> {
    get_service(&config, &service).await?;
    let result = service.get_user_by_name(&name).await;
    Ok(result.into())
}

/// Search users
#[tauri::command]
pub async fn search_users(
    query: String,
    page: Option<i32>,
    per_page: Option<i32>,
    config: State<'_, ConfigState>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Vec<User>>, String> {
    get_service(&config, &service).await?;
    let result = service.search_users(&query, page, per_page).await;
    Ok(result.into())
}
