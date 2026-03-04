use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::forum::{
        DeleteThreadCommentOptions, DeleteThreadOptions, FetchThreadOptions,
        SaveThreadCommentOptions, SaveThreadOptions,
    },
    objects::{
        responses::Page,
        thread::{Thread, ThreadComment},
    },
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

// ============================================================================
// Read commands
// ============================================================================

/// Search forum threads
#[tauri::command]
pub async fn search_forum_threads(
    options: FetchThreadOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("search_forum_threads");
    let client = service.client().await;
    let result = client.forum().fetch(&options).await;
    Ok(result.into())
}

/// Get a forum thread by ID
#[tauri::command]
pub async fn get_forum_thread(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Thread>, String> {
    log::info!("get_forum_thread: {}", id);
    let client = service.client().await;
    let result = client.forum().get_by_id(id).await;
    Ok(result.into())
}

/// Get recent forum threads
#[tauri::command]
pub async fn get_recent_forum_threads(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("get_recent_forum_threads");
    let client = service.client().await;
    let result = client
        .forum()
        .get_recent(page, per_page)
        .await;
    Ok(result.into())
}

/// Get popular forum threads
#[tauri::command]
pub async fn get_popular_forum_threads(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("get_popular_forum_threads");
    let client = service.client().await;
    let result = client
        .forum()
        .get_popular(page, per_page)
        .await;
    Ok(result.into())
}

/// Get threads by category ID
#[tauri::command]
pub async fn get_forum_threads_by_category(
    category_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("get_forum_threads_by_category: {}", category_id);
    let client = service.client().await;
    let result = client
        .forum()
        .get_by_category(category_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Get threads created by a user
#[tauri::command]
pub async fn get_forum_threads_by_user(
    user_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("get_forum_threads_by_user: {}", user_id);
    let client = service.client().await;
    let result = client
        .forum()
        .get_by_user(user_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Get threads the authenticated user is subscribed to
#[tauri::command]
pub async fn get_subscribed_forum_threads(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Thread>>>, String> {
    log::info!("get_subscribed_forum_threads");
    let client = service.client().await;
    let result = client
        .forum()
        .get_subscribed(page, per_page)
        .await;
    Ok(result.into())
}

/// Get comments for a forum thread
#[tauri::command]
pub async fn get_thread_comments(
    thread_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<ThreadComment>>>, String> {
    log::info!("get_thread_comments: thread_id={}", thread_id);
    let client = service.client().await;
    let result = client
        .forum()
        .get_thread_comments(thread_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Get a thread comment by ID
#[tauri::command]
pub async fn get_thread_comment_by_id(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ThreadComment>, String> {
    log::info!("get_thread_comment_by_id: {}", id);
    let client = service.client().await;
    let result = client.forum().get_comment_by_id(id).await;
    Ok(result.into())
}

// ============================================================================
// Mutation commands
// ============================================================================

/// Create or update a forum thread
#[tauri::command]
pub async fn save_forum_thread(
    options: SaveThreadOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Thread>, String> {
    log::info!("save_forum_thread: title={:?}", options.title);
    let client = service.client().await;
    let result = client.forum().save(&options).await;
    Ok(result.into())
}

/// Delete a forum thread
#[tauri::command]
pub async fn delete_forum_thread(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_forum_thread: {}", id);
    let client = service.client().await;
    let result = client.forum().delete(&DeleteThreadOptions { id }).await;
    Ok(result.into())
}

/// Create or update a thread comment
#[tauri::command]
pub async fn save_thread_comment(
    options: SaveThreadCommentOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ThreadComment>, String> {
    log::info!("save_thread_comment: thread_id={:?}", options.thread_id);
    let client = service.client().await;
    let result = client.forum().save_comment(&options).await;
    Ok(result.into())
}

/// Delete a thread comment
#[tauri::command]
pub async fn delete_thread_comment(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_thread_comment: {}", id);
    let client = service.client().await;
    let result = client
        .forum()
        .delete_comment(&DeleteThreadCommentOptions { id })
        .await;
    Ok(result.into())
}

/// Toggle subscription to a forum thread
#[tauri::command]
pub async fn toggle_forum_thread_subscription(
    thread_id: i32,
    subscribe: bool,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Thread>, String> {
    log::info!("toggle_forum_thread_subscription: thread_id={}", thread_id);
    let client = service.client().await;
    let result = if subscribe {
        client.forum().subscribe_to_thread(thread_id).await
    } else {
        client.forum().unsubscribe_from_thread(thread_id).await
    };
    Ok(result.into())
}

/// Reply to a forum thread (shortcut)
#[tauri::command]
pub async fn reply_to_forum_thread(
    thread_id: i32,
    comment: String,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ThreadComment>, String> {
    log::info!("reply_to_forum_thread: thread_id={}", thread_id);
    let client = service.client().await;
    let result = client.forum().reply_to_thread(thread_id, &comment).await;
    Ok(result.into())
}

/// Reply to a specific thread comment (nested reply)
#[tauri::command]
pub async fn reply_to_thread_comment(
    thread_id: i32,
    parent_comment_id: i32,
    comment: String,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<ThreadComment>, String> {
    log::info!(
        "reply_to_thread_comment: thread_id={}, parent={}",
        thread_id, parent_comment_id
    );
    let client = service.client().await;
    let result = client
        .forum()
        .save_comment(&SaveThreadCommentOptions {
            thread_id: Some(thread_id),
            parent_comment_id: Some(parent_comment_id),
            comment: Some(comment),
            ..Default::default()
        })
        .await;
    Ok(result.into())
}
