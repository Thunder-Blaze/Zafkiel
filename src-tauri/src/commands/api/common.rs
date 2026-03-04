use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::unions::likeable::LikeableUnion;
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

/// Toggle like on a forum thread
#[tauri::command]
pub async fn toggle_like_thread(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<LikeableUnion>, String> {
    log::info!("toggle_like_thread: {}", id);
    let client = service.client().await;
    let result = client.common().like_thread(id).await;
    Ok(result.into())
}

/// Toggle like on a thread comment
#[tauri::command]
pub async fn toggle_like_thread_comment(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<LikeableUnion>, String> {
    log::info!("toggle_like_thread_comment: {}", id);
    let client = service.client().await;
    let result = client.common().like_thread_comment(id).await;
    Ok(result.into())
}
