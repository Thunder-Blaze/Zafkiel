use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::recommendation::{
        FetchRecommendationOptions, SaveRecommendationOptions,
    },
    objects::{recommendation::Recommendation, responses::Page},
};
use std::sync::Arc;
use tauri::State;

pub type AniListState = Arc<AniListService>;

/// Fetch recommendations with filters
#[tauri::command]
pub async fn fetch_recommendations(
    options: FetchRecommendationOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Recommendation>>>, String> {
    log::info!("fetch_recommendations");
    let client = service.client().await;
    let result = client.recommendation().fetch(&options).await;
    Ok(result.into())
}

/// Get recommendations for a specific media
#[tauri::command]
pub async fn get_recommendations_by_media(
    media_id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Recommendation>>>, String> {
    log::info!("get_recommendations_by_media: media_id={}", media_id);
    let client = service.client().await;
    let result = client
        .recommendation()
        .get_by_media_id(media_id, page, per_page)
        .await;
    Ok(result.into())
}

/// Save (rate) a recommendation
#[tauri::command]
pub async fn save_recommendation(
    options: SaveRecommendationOptions,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Recommendation>, String> {
    log::info!(
        "save_recommendation: media_id={} rec_id={}",
        options.media_id,
        options.media_recommendation_id
    );
    let client = service.client().await;
    let result = client.recommendation().save(&options).await;
    Ok(result.into())
}
