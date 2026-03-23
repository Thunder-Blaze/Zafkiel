use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::medialist::{FetchMediaListOptions, SaveMediaListOptions},
    enums::media_list::MediaListStatus,
    objects::{media_list::MediaList, responses::Page},
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

// ============================================================================
// Read commands
// ============================================================================

/// Fetch paginated media list entries with filters
#[tauri::command]
pub async fn fetch_media_list(
    options: FetchMediaListOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("fetch_media_list");
    let client = service.client().await;
    let result = client.medialist().fetch(&options).await;
    Ok(result.into())
}

/// Get the authenticated user's anime list (all statuses, paginated)
#[tauri::command]
pub async fn get_my_anime_list(
    status: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_my_anime_list");
    let status_enum = parse_status(status)?;
    let client = service.client().await;
    let result = client
        .medialist()
        .get_my_anime_list(status_enum, page, per_page)
        .await;
    Ok(result.into())
}

/// Get the authenticated user's manga list (all statuses, paginated)
#[tauri::command]
pub async fn get_my_manga_list(
    status: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_my_manga_list");
    let status_enum = parse_status(status)?;
    let client = service.client().await;
    let result = client
        .medialist()
        .get_my_manga_list(status_enum, page, per_page)
        .await;
    Ok(result.into())
}

/// Get another user's anime list by username
#[tauri::command]
pub async fn get_user_anime_list(
    username: String,
    status: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_user_anime_list: username={}", username);
    let status_enum = parse_status(status)?;
    let client = service.client().await;
    let result = client
        .medialist()
        .get_user_anime_list(&username, status_enum, page, per_page)
        .await;
    Ok(result.into())
}

/// Get another user's manga list by username
#[tauri::command]
pub async fn get_user_manga_list(
    username: String,
    status: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_user_manga_list: username={}", username);
    let status_enum = parse_status(status)?;
    let client = service.client().await;
    let result = client
        .medialist()
        .get_user_manga_list(&username, status_enum, page, per_page)
        .await;
    Ok(result.into())
}

/// Get currently watching anime (optionally for a specific username)
#[tauri::command]
pub async fn get_watching(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_watching");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_watching(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

/// Get currently reading manga (optionally for a specific username)
#[tauri::command]
pub async fn get_reading(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_reading");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_reading(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

/// Get plan-to-watch anime (optionally for a specific username)
#[tauri::command]
pub async fn get_plan_to_watch(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_plan_to_watch");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_plan_to_watch(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

/// Get plan-to-read manga (optionally for a specific username)
#[tauri::command]
pub async fn get_plan_to_read(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_plan_to_read");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_plan_to_read(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

/// Get completed anime (optionally for a specific username)
#[tauri::command]
pub async fn get_completed_anime(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_completed_anime");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_completed_anime(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

/// Get completed manga (optionally for a specific username)
#[tauri::command]
pub async fn get_completed_manga(
    username: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<MediaList>>>, String> {
    log::info!("get_completed_manga");
    let client = service.client().await;
    let result = client
        .medialist()
        .get_completed_manga(username.as_deref(), page, per_page)
        .await;
    Ok(result.into())
}

// ============================================================================
// Mutation commands
// ============================================================================

/// Save (create or update) a media list entry
#[tauri::command]
pub async fn save_media_list_entry(
    options: SaveMediaListOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!("save_media_list_entry: media_id={:?}", options.media_id);
    let client = service.client().await;
    let result = client.medialist().save(&options).await;
    Ok(result.into())
}

/// Add anime to the authenticated user's list with the given status (defaults to PLANNING)
#[tauri::command]
pub async fn add_anime_to_list(
    media_id: i32,
    status: Option<String>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!("add_anime_to_list: media_id={}", media_id);
    let status_enum = parse_status(status)?.unwrap_or(MediaListStatus::Planning);
    let client = service.client().await;
    let result = client.medialist().add_anime(media_id, status_enum).await;
    Ok(result.into())
}

/// Add manga to the authenticated user's list with the given status (defaults to PLANNING)
#[tauri::command]
pub async fn add_manga_to_list(
    media_id: i32,
    status: Option<String>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!("add_manga_to_list: media_id={}", media_id);
    let status_enum = parse_status(status)?.unwrap_or(MediaListStatus::Planning);
    let client = service.client().await;
    let result = client.medialist().add_manga(media_id, status_enum).await;
    Ok(result.into())
}

/// Update progress for a media list entry
#[tauri::command]
pub async fn update_media_progress(
    entry_id: i32,
    progress: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!(
        "update_media_progress: entry_id={} progress={}",
        entry_id,
        progress
    );
    let client = service.client().await;
    let result = client.medialist().update_progress(entry_id, progress).await;
    Ok(result.into())
}

/// Update score for a media list entry
#[tauri::command]
pub async fn update_media_score(
    entry_id: i32,
    score: f64,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!("update_media_score: entry_id={} score={}", entry_id, score);
    let client = service.client().await;
    let result = client.medialist().update_score(entry_id, score).await;
    Ok(result.into())
}

/// Update status for a media list entry
#[tauri::command]
pub async fn update_media_status(
    entry_id: i32,
    status: String,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<MediaList>, String> {
    log::info!(
        "update_media_status: entry_id={} status={}",
        entry_id,
        status
    );

    let status_enum =
        parse_status(Some(status))?.ok_or_else(|| "Status is required".to_string())?;

    let client = service.client().await;
    let result = client
        .medialist()
        .update_status(entry_id, status_enum)
        .await;
    Ok(result.into())
}

/// Delete a media list entry
#[tauri::command]
pub async fn delete_media_list_entry(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_media_list_entry: id={}", id);
    let client = service.client().await;
    let result = client.medialist().delete_entry(id).await;
    Ok(result.into())
}

// ============================================================================
// Helpers
// ============================================================================

fn parse_status(status: Option<String>) -> Result<Option<MediaListStatus>, String> {
    match status.as_deref() {
        None => Ok(None),
        Some("CURRENT") => Ok(Some(MediaListStatus::Current)),
        Some("PLANNING") => Ok(Some(MediaListStatus::Planning)),
        Some("COMPLETED") => Ok(Some(MediaListStatus::Completed)),
        Some("DROPPED") => Ok(Some(MediaListStatus::Dropped)),
        Some("PAUSED") => Ok(Some(MediaListStatus::Paused)),
        Some("REPEATING") => Ok(Some(MediaListStatus::Repeating)),
        Some(s) => Err(format!("Unknown media list status: {}", s)),
    }
}
