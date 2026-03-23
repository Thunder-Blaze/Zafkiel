use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::notification::NotificationSearchOptions, objects::responses::Page,
    unions::notification::NotificationUnion,
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

/// Fetch notifications with filters
#[tauri::command]
pub async fn fetch_notifications(
    options: NotificationSearchOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<NotificationUnion>>>, String> {
    log::info!("fetch_notifications");
    let client = service.client().await;
    let result = client.notification().fetch(&options).await;
    Ok(result.into())
}

/// Get all notifications (paginated)
#[tauri::command]
pub async fn get_all_notifications(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<NotificationUnion>>>, String> {
    log::info!("get_all_notifications");
    let client = service.client().await;
    let result = client.notification().get_all(page, per_page).await;
    Ok(result.into())
}

/// Get unread notifications and mark them read
#[tauri::command]
pub async fn get_and_mark_notifications_read(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<NotificationUnion>>>, String> {
    log::info!("get_and_mark_notifications_read");
    let client = service.client().await;
    let result = client
        .notification()
        .get_and_mark_read(page, per_page)
        .await;
    Ok(result.into())
}
