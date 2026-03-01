use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::media::{FetchMediaOneOptions, FetchMediaOptions},
    endpoints::character::FetchCharacterOptions,
    endpoints::staff::FetchStaffOptions,
    objects::{media::Media, responses::Page, user::User, studio::Studio, character::Character, staff::Staff},
    enums::media::{MediaType, MediaFormat, MediaStatus, MediaSeason, MediaSort, MediaSource},
    enums::character::CharacterSort,
    enums::staff::StaffSort,
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

/// Browse anime/manga with comprehensive filters
#[tauri::command]
pub async fn browse_media(
    media_type: Option<String>,
    search: Option<String>,
    season: Option<String>,
    season_year: Option<i32>,
    format: Option<String>,
    status: Option<String>,
    source: Option<String>,
    genres: Option<Vec<String>>,
    genres_excluded: Option<Vec<String>>,
    sort_by: Option<Vec<String>>,
    is_adult: Option<bool>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>
) -> Result<AniListResponse<Page<Vec<Media>>>, String> {
    log::info!("Executing command: browse_media with filters");

    // Convert string enums to their proper types
    let media_type_enum = media_type.as_ref().and_then(|t| match t.as_str() {
        "ANIME" => Some(MediaType::Anime),
        "MANGA" => Some(MediaType::Manga),
        _ => None,
    });

    let season_enum = season.as_ref().and_then(|s| match s.as_str() {
        "WINTER" => Some(MediaSeason::Winter),
        "SPRING" => Some(MediaSeason::Spring),
        "SUMMER" => Some(MediaSeason::Summer),
        "FALL" => Some(MediaSeason::Fall),
        _ => None,
    });

    let format_enum = format.as_ref().and_then(|f| match f.as_str() {
        "TV" => Some(MediaFormat::Tv),
        "TV_SHORT" => Some(MediaFormat::TvShort),
        "MOVIE" => Some(MediaFormat::Movie),
        "SPECIAL" => Some(MediaFormat::Special),
        "OVA" => Some(MediaFormat::Ova),
        "ONA" => Some(MediaFormat::Ona),
        "MUSIC" => Some(MediaFormat::Music),
        "MANGA" => Some(MediaFormat::Manga),
        "NOVEL" => Some(MediaFormat::Novel),
        "ONE_SHOT" => Some(MediaFormat::OneShot),
        _ => None,
    });

    let status_enum = status.as_ref().and_then(|s| match s.as_str() {
        "FINISHED" => Some(MediaStatus::Finished),
        "RELEASING" => Some(MediaStatus::Releasing),
        "NOT_YET_RELEASED" => Some(MediaStatus::NotYetReleased),
        "CANCELLED" => Some(MediaStatus::Cancelled),
        "HIATUS" => Some(MediaStatus::Hiatus),
        _ => None,
    });

    let source_enum = source.as_ref().and_then(|s| match s.as_str() {
        "ORIGINAL" => Some(MediaSource::Original),
        "MANGA" => Some(MediaSource::Manga),
        "LIGHT_NOVEL" => Some(MediaSource::LightNovel),
        "VISUAL_NOVEL" => Some(MediaSource::VisualNovel),
        "VIDEO_GAME" => Some(MediaSource::VideoGame),
        "OTHER" => Some(MediaSource::Other),
        "NOVEL" => Some(MediaSource::Novel),
        "DOUJINSHI" => Some(MediaSource::Doujinshi),
        "ANIME" => Some(MediaSource::Anime),
        "WEB_NOVEL" => Some(MediaSource::WebNovel),
        "LIVE_ACTION" => Some(MediaSource::LiveAction),
        "GAME" => Some(MediaSource::Game),
        "COMIC" => Some(MediaSource::Comic),
        "MULTIMEDIA_PROJECT" => Some(MediaSource::MultimediaProject),
        "PICTURE_BOOK" => Some(MediaSource::PictureBook),
        _ => None,
    });

    let sort_enums = sort_by.as_ref().map(|sorts| {
        sorts.iter().filter_map(|s| match s.as_str() {
            "POPULARITY_DESC" => Some(MediaSort::PopularityDesc),
            "POPULARITY" => Some(MediaSort::Popularity),
            "TRENDING_DESC" => Some(MediaSort::TrendingDesc),
            "TRENDING" => Some(MediaSort::Trending),
            "SCORE_DESC" => Some(MediaSort::ScoreDesc),
            "SCORE" => Some(MediaSort::Score),
            "TITLE_ROMAJI" => Some(MediaSort::TitleRomaji),
            "TITLE_ROMAJI_DESC" => Some(MediaSort::TitleRomajiDesc),
            "TITLE_ENGLISH" => Some(MediaSort::TitleEnglish),
            "TITLE_ENGLISH_DESC" => Some(MediaSort::TitleEnglishDesc),
            "TITLE_NATIVE" => Some(MediaSort::TitleNative),
            "TITLE_NATIVE_DESC" => Some(MediaSort::TitleNativeDesc),
            "START_DATE" => Some(MediaSort::StartDate),
            "START_DATE_DESC" => Some(MediaSort::StartDateDesc),
            "END_DATE" => Some(MediaSort::EndDate),
            "END_DATE_DESC" => Some(MediaSort::EndDateDesc),
            "FAVOURITES_DESC" => Some(MediaSort::FavouritesDesc),
            "FAVOURITES" => Some(MediaSort::Favourites),
            "ID" => Some(MediaSort::Id),
            "ID_DESC" => Some(MediaSort::IdDesc),
            _ => None,
        }).collect::<Vec<_>>()
    });

    let options = FetchMediaOptions {
        media_type: media_type_enum,
        search,
        season: season_enum,
        season_year,
        format: format_enum,
        status: status_enum,
        source: source_enum.map(|s| {
            // Serialize using serde to get the SCREAMING_SNAKE_CASE format
            serde_json::to_value(s)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        }),
        genre_in: genres,
        genre_not_in: genres_excluded,
        sort: sort_enums,
        is_adult,
        page,
        per_page,
        ..Default::default()
    };

    let client = service.client().await;
    let result = client.media().fetch(&options).await;
    Ok(result.into())
}

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

create_anilist_command!(get_popular_characters, character, get_most_favorited, Page<Vec<Character>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_birthday_characters, character, get_today_birthday, Page<Vec<Character>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

#[tauri::command]
pub async fn search_characters(
    query: &str,
    page: Option<i32>,
    per_page: Option<i32>,
    is_birthday: Option<bool>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Character>>>, String> {
    log::info!("Executing command: search_characters is_birthday={:?}", is_birthday);
    let client = service.client().await;
    let result = client.character().fetch(&FetchCharacterOptions {
        search: Some(query.to_string()),
        page,
        per_page,
        is_birthday,
        sort: Some(vec![CharacterSort::SearchMatch]),
        ..Default::default()
    }).await;
    Ok(result.into())
}

// ============================================================================
// Staff Commands
// ============================================================================

create_anilist_command!(get_staff_by_id, staff, get_by_id, Staff, (
    id: i32 => val
));

create_anilist_command!(get_popular_staff, staff, get_most_favorited, Page<Vec<Staff>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

create_anilist_command!(get_birthday_staff, staff, get_today_birthday, Page<Vec<Staff>>, (
    page: Option<i32> => ref,
    per_page: Option<i32> => ref
));

#[tauri::command]
pub async fn search_staff(
    query: &str,
    page: Option<i32>,
    per_page: Option<i32>,
    is_birthday: Option<bool>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Staff>>>, String> {
    log::info!("Executing command: search_staff is_birthday={:?}", is_birthday);
    let client = service.client().await;
    let result = client.staff().fetch(&FetchStaffOptions {
        search: Some(query.to_string()),
        sort: Some(vec![StaffSort::SearchMatch]),
        page,
        per_page,
        is_birthday,
        ..Default::default()
    }).await;
    Ok(result.into())
}

// ============================================================================
// Seasonal Anime Commands
// ============================================================================

#[tauri::command]
pub async fn get_seasonal_anime(
    season: String,
    year: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Page<Vec<Media>>>, String> {
    use anilist_moe::enums::media::MediaType;
    log::info!("Executing command: get_seasonal_anime season={} year={}", season, year);

    let season_enum = match season.as_str() {
        "WINTER" => MediaSeason::Winter,
        "SPRING" => MediaSeason::Spring,
        "SUMMER" => MediaSeason::Summer,
        "FALL"   => MediaSeason::Fall,
        _        => MediaSeason::Spring,
    };

    use anilist_moe::endpoints::media::FetchMediaOptions;
    use anilist_moe::enums::media::MediaSort;

    let options = FetchMediaOptions {
        media_type: Some(MediaType::Anime),
        season: Some(season_enum),
        season_year: Some(year),
        sort: Some(vec![MediaSort::PopularityDesc]),
        page,
        per_page,
        ..Default::default()
    };

    let client = service.client().await;
    let result = client.media().fetch(&options).await;
    Ok(result.into())
}
