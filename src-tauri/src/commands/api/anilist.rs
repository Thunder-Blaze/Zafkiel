use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::media::{FetchMediaOneOptions, FetchMediaOptions},
    objects::{media::Media, responses::Page, user::User, studio::Studio, character::Character, staff::Staff},
};
use std::sync::Arc;
use tauri::State;

/// Shared state for AniList service
pub type AniListState = Arc<AniListService>;

// ============================================================================
// Helper Macro
// ============================================================================

// In anilist_commands.rs

// This macro creates a complete Tauri command function for us.
macro_rules! create_anilist_command {
    // Pattern for commands with arguments
    (
        $command_name:ident,
				$category:ident,
        $client_method:ident,
        $return_type:ty,
        ( $( $arg_name:ident: $arg_type:ty => $pass_style:tt ),+ )
    ) => {
        #[tauri::command]
        pub async fn $command_name(
            $($arg_name: $arg_type,)+
            service: State<'_, AniListState>
        ) -> Result<AniListResponse<$return_type>, String> {
            log::info!("Executing command: {}", stringify!($command_name));
            // Note the use of `&` for string arguments to avoid unnecessary cloning
						let client = service.client().await;
            let result = client.$category().$client_method(
							$( create_anilist_command!(@pass $arg_name, $pass_style) ),+
						).await;
            Ok(result.into())
        }
    };
    // Pattern for commands with NO arguments
    (
        $command_name:ident,
				$category:ident,
        $client_method:ident,
        $return_type:ty
    ) => {
        #[tauri::command]
        pub async fn $command_name(
            service: State<'_, AniListState>
        ) -> Result<AniListResponse<$return_type>, String> {
            log::info!("Executing command: {}", stringify!($command_name));
            let client = service.client().await;
            let result = client.$category().$client_method().await;
            Ok(result.into())
        }
    };

		// --- HELPER RULES ---
    // @pass $arg, val => passes the argument by value (for Copy types like i32)
    (@pass $arg:ident, val) => { $arg };

    // @pass $arg, ref => passes the argument as-is (for existing references like &str)
    (@pass $arg:ident, ref) => { $arg };

    // @pass $arg, borrow => passes the argument as a new reference (for owned structs)
    (@pass $arg:ident, borrow) => { &$arg };
}

// ============================================================================
// Media Commands
// ============================================================================

create_anilist_command!(search_media, media, fetch, Page<Vec<Media>>, (
    options: FetchMediaOptions => borrow
));

create_anilist_command!(get_media_by_id, media, fetch_one, Media, (
    options: FetchMediaOneOptions => borrow
));

#[tauri::command]
pub async fn get_anime_by_id(
    id: i32,
    service: State<'_, AniListState>
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_anime_by_id");
    let client = service.client().await;
    let result = client.media().fetch_one(&FetchMediaOneOptions {
        id: Some(id),
        fetch_characters: Some(true),
        fetch_staff: Some(true),
        fetch_recommendations: Some(true),
        fetch_reviews: Some(true),
        ..Default::default()
    }).await;

    match result {
        Ok(media) => {
            log::info!("Fetched media: ID={:?}, Type={:?}", media.id, media.media_type);
            Ok(AniListResponse::success(media))
        },
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e)))
    }
}

#[tauri::command]
pub async fn get_manga_by_id(
    id: i32,
    service: State<'_, AniListState>
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_manga_by_id");
    let client = service.client().await;
    let result = client.media().fetch_one(&FetchMediaOneOptions {
        id: Some(id),
        fetch_characters: Some(true),
        fetch_staff: Some(true),
        fetch_recommendations: Some(true),
        fetch_reviews: Some(true),
        ..Default::default()
    }).await;

    match result {
        Ok(media) => {
            match media.media_type {
                Some(anilist_moe::enums::media::MediaType::Manga) => Ok(AniListResponse::success(media)),
                _ => Ok(AniListResponse::error("Manga not found (wrong type)".to_string()))
            }
        },
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e)))
    }
}

// ============================================================================
// Anime Commands
// ============================================================================

create_anilist_command!(get_trending_anime, media, get_trending_anime, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_popular_anime, media, get_popular_anime, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_upcoming_anime, media, get_upcoming_anime, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_airing_anime, media, get_airing_anime, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

// ============================================================================
// Manga Commands
// ============================================================================

create_anilist_command!(get_trending_manga, media, get_trending_manga, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_popular_manga, media, get_popular_manga, Page<Vec<Media>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

// ============================================================================
// User Commands
// ============================================================================

create_anilist_command!(get_current_user, user, get_current_user, User);

#[tauri::command]
pub async fn get_user_by_id(
    id: i32,
    service: State<'_, AniListState>
) -> Result<AniListResponse<User>, String> {
    log::info!("Executing command: get_user_by_id for id: {}", id);
    let client = service.client().await;
    let result = client.user().get_by_id(id).await;
    match result {
        Ok(user) => Ok(AniListResponse::success(user)),
        Err(e) => {
            log::error!("Error fetching user by id {}: {:?}", id, e);
            Ok(AniListResponse::error(format!("{:?}", e)))
        }
    }
}

create_anilist_command!(get_user_by_name, user, get_by_name, User, (
    name: &str => ref
));

create_anilist_command!(search_users, user, search, Page<Vec<User>>, (
    search: &str => ref,
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

// ============================================================================
// Studio Commands
// ============================================================================

create_anilist_command!(get_studio_by_id, studio, get_by_id, Studio, (
    id: i32 => val
));

// ============================================================================
// Character Commands
// ============================================================================

create_anilist_command!(get_character_by_id, character, get_by_id, Character, (
    id: i32 => val
));

// ============================================================================
// Staff Commands
// ============================================================================

create_anilist_command!(get_staff_by_id, staff, get_by_id, Staff, (
    id: i32 => val
));
