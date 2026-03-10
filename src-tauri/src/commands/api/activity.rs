use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::activity::{
        DeleteActivityOptions, DeleteActivityReplyOptions, FetchActivityOptions,
        SaveMessageActivityOptions, SaveTextActivityOptions, SaveActivityReplyOptions,
        SubscribeActivityOptions,
    },
    objects::{activity::ActivityReply, responses::Page},
    unions::activity::ActivityUnion,
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

// ============================================================================
// Read commands
// ============================================================================

/// Fetch activity feed with filters
#[tauri::command]
pub async fn fetch_activities(
    options: FetchActivityOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<ActivityUnion>>>, String> {
    log::info!("fetch_activities");
    let client = service.client().await;
    let result = client.activity().fetch(&options).await;
    Ok(result.into())
}

/// Get a single activity by ID
#[tauri::command]
pub async fn get_activity_by_id(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ActivityUnion>, String> {
    log::info!("get_activity_by_id: {}", id);
    let client = service.client().await;
    let result = client.activity().get_by_id(id).await;
    Ok(result.into())
}

/// Get recent global activity feed
#[tauri::command]
pub async fn get_recent_activity(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<ActivityUnion>>>, String> {
    log::info!("get_recent_activity");
    let client = service.client().await;
    
    // Instead of get_recent, we use fetch to pass specific filters
    let result = client
        .activity()
        .fetch(&FetchActivityOptions {
            page,
            per_page,
            is_following: Some(false), // Global feed doesn't filter by following
            has_replies_or_type_text: Some(true), // Only fetch posts with replies or text posts
            ..Default::default()
        })
        .await;
    Ok(result.into())
}

/// Get activity feed from followed users
#[tauri::command]
pub async fn get_following_activity(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<ActivityUnion>>>, String> {
    log::info!("get_following_activity");
    let client = service.client().await;
    let result = client
        .activity()
        .get_following(page, per_page)
        .await;
    Ok(result.into())
}

/// Fetch replies to an activity
#[tauri::command]
pub async fn fetch_activity_replies(
    activity_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<ActivityReply>>>, String> {
    log::info!("fetch_activity_replies: activity_id={}", activity_id);
    use anilist_moe::endpoints::activity::FetchActivityRepliesOptions;
    let client = service.client().await;
    let result = client
        .activity()
        .fetch_replies(&FetchActivityRepliesOptions {
            activity_id,
            page,
            per_page,
            ..Default::default()
        })
        .await;
    Ok(result.into())
}

// ============================================================================
// Mutation commands
// ============================================================================

/// Create or update a text activity
#[tauri::command]
pub async fn save_text_activity(
    options: SaveTextActivityOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ActivityUnion>, String> {
    log::info!("save_text_activity");
    let client = service.client().await;
    let result = client.activity().save_text_activity(&options).await;
    Ok(result.into())
}

/// Create or update a message activity (DM to another user)
#[tauri::command]
pub async fn save_message_activity(
    options: SaveMessageActivityOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ActivityUnion>, String> {
    log::info!("save_message_activity: recipient={}", options.recipient_id);
    let client = service.client().await;
    let result = client.activity().save_message_activity(&options).await;
    Ok(result.into())
}

/// Reply to an activity
#[tauri::command]
pub async fn save_activity_reply(
    options: SaveActivityReplyOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ActivityReply>, String> {
    log::info!("save_activity_reply: activity_id={}", options.activity_id);
    let client = service.client().await;
    let result = client.activity().save_reply(&options).await;
    Ok(result.into())
}

/// Delete an activity
#[tauri::command]
pub async fn delete_activity(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_activity: {}", id);
    let client = service.client().await;
    let result = client
        .activity()
        .delete(&DeleteActivityOptions { id })
        .await;
    Ok(result.into())
}

/// Delete an activity reply
#[tauri::command]
pub async fn delete_activity_reply(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_activity_reply: {}", id);
    let client = service.client().await;
    let result = client
        .activity()
        .delete_reply(&DeleteActivityReplyOptions { id })
        .await;
    Ok(result.into())
}

/// Toggle like on an activity
#[tauri::command]
pub async fn toggle_activity_subscription(
    id: i32,
    subscribe: bool,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ActivityUnion>, String> {
    log::info!("toggle_activity_subscription: id={} subscribe={}", id, subscribe);
    let client = service.client().await;
    let result = client
        .activity()
        .subscribe(&SubscribeActivityOptions { id, subscribe })
        .await;
    Ok(result.into())
}
