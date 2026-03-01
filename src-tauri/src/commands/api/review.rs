use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::review::{
        DeleteReviewOptions, FetchReviewOptions, RateReviewOptions, SaveReviewOptions,
    },
    objects::{responses::Page, review::Review},
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

/// Fetch reviews with filters
#[tauri::command]
pub async fn fetch_reviews(
    options: FetchReviewOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Review>>>, String> {
    log::info!("fetch_reviews");
    let client = service.client().await;
    let result = client.review().fetch(&options).await;
    Ok(result.into())
}

/// Get reviews for a specific media by ID
#[tauri::command]
pub async fn get_reviews_by_media(
    media_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Review>>>, String> {
    log::info!("get_reviews_by_media: media_id={}", media_id);
    let client = service.client().await;
    let result = client
        .review()
        .get_by_media_id(media_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Get reviews by a specific user
#[tauri::command]
pub async fn get_reviews_by_user(
    user_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Review>>>, String> {
    log::info!("get_reviews_by_user: user_id={}", user_id);
    let client = service.client().await;
    let result = client
        .review()
        .get_by_user_id(user_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Get a review by ID
#[tauri::command]
pub async fn get_review_by_id(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Review>, String> {
    log::info!("get_review_by_id: {}", id);
    let client = service.client().await;
    let result = client.review().get_by_id(id).await;
    Ok(result.into())
}

/// Get recent reviews
#[tauri::command]
pub async fn get_recent_reviews(
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Review>>>, String> {
    log::info!("get_recent_reviews");
    let client = service.client().await;
    let result = client.review().get_recent(page, per_page).await;
    Ok(result.into())
}

/// Save (create or update) a review
#[tauri::command]
pub async fn save_review(
    options: SaveReviewOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Review>, String> {
    log::info!("save_review: media_id={}", options.media_id);
    let client = service.client().await;
    let result = client.review().save(&options).await;
    Ok(result.into())
}

/// Delete a review
#[tauri::command]
pub async fn delete_review(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<bool>, String> {
    log::info!("delete_review: {}", id);
    let client = service.client().await;
    let result = client.review().delete(&DeleteReviewOptions { id }).await;
    Ok(result.into())
}

/// Rate a review (upvote / downvote / no-vote)
#[tauri::command]
pub async fn rate_review(
    options: RateReviewOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Review>, String> {
    log::info!("rate_review: review_id={}", options.review_id);
    let client = service.client().await;
    let result = client.review().rate(&options).await;
    Ok(result.into())
}
