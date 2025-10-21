use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::media::{FetchMediaOneOptions, FetchMediaOptions},
    objects::{media::Media, responses::Page, user::User},
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

create_anilist_command!(get_user_by_id, user, get_by_id, User, (
    id: i32 => ref
));

create_anilist_command!(get_user_by_name, user, get_by_name, User, (
    name: &str => ref
));

create_anilist_command!(search_users, user, search, Page<Vec<User>>, (
    search: &str => ref,
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));
