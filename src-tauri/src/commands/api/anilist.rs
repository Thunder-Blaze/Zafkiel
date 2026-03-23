use crate::api::anilist::{AniListResponse, AniListService};
use anilist_moe::{
    endpoints::character::FetchCharacterOptions,
    endpoints::media::{FetchMediaOneOptions, FetchMediaOptions},
    endpoints::staff::FetchStaffOptions,
    endpoints::user::FetchUserOptions,
    enums::character::CharacterSort,
    enums::media::{MediaFormat, MediaSeason, MediaSort, MediaSource, MediaStatus, MediaType},
    enums::staff::StaffSort,
    objects::{
        character::Character, favourites::Favourites, media::Media, responses::Page, staff::Staff,
        studio::Studio, user::User,
    },
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
    country_of_origin: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
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
        sorts
            .iter()
            .filter_map(|s| match s.as_str() {
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
            })
            .collect::<Vec<_>>()
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
        country_of_origin,
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
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_anime_by_id");
    let client = service.client().await;
    let result = client
        .media()
        .fetch_one(&FetchMediaOneOptions {
            id: Some(id),
            fetch_characters: None,
            fetch_staff: None,
            fetch_recommendations: None,
            fetch_reviews: None,
            ..Default::default()
        })
        .await;

    match result {
        Ok(media) => {
            log::info!(
                "Fetched media: ID={:?}, Type={:?}",
                media.id,
                media.media_type
            );
            Ok(AniListResponse::success(media))
        }
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
    }
}

#[tauri::command]
pub async fn get_anime_characters_by_id(
    id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    language: Option<String>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_anime_characters_by_id");
    let client = service.client().await;

    let voice_actor_language_str = language.unwrap_or_else(|| "JAPANESE".to_string());
    let voice_actor_language = serde_json::from_value(serde_json::Value::String(
        voice_actor_language_str.to_uppercase(),
    ))
    .ok();

    let options = FetchMediaOptions {
        id: Some(id),
        include_characters: Some(true),
        characters_page: page,
        characters_per_page: per_page,
        voice_actor_language,
        ..Default::default()
    };

    match client.media().fetch(&options).await {
        Ok(mut res) => {
            if let Some(media) = res.data.pop() {
                Ok(AniListResponse::success(media))
            } else {
                Ok(AniListResponse::error("Anime not found".to_string()))
            }
        }
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
    }
}

#[tauri::command]
pub async fn get_anime_staff_by_id(
    id: i32,
    page: Option<i32>,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_anime_staff_by_id");
    let client = service.client().await;

    let options = FetchMediaOptions {
        id: Some(id),
        include_staff: Some(true),
        staff_page: page,
        staff_per_page: per_page,
        ..Default::default()
    };

    match client.media().fetch(&options).await {
        Ok(mut res) => {
            if let Some(media) = res.data.pop() {
                Ok(AniListResponse::success(media))
            } else {
                Ok(AniListResponse::error("Anime not found".to_string()))
            }
        }
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
    }
}

#[tauri::command]
pub async fn get_manga_by_id(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Media>, String> {
    log::info!("Executing command: get_manga_by_id");
    let client = service.client().await;
    let result = client
        .media()
        .fetch_one(&FetchMediaOneOptions {
            id: Some(id),
            fetch_characters: Some(true),
            fetch_staff: Some(true),
            fetch_recommendations: Some(true),
            fetch_reviews: Some(true),
            ..Default::default()
        })
        .await;

    match result {
        Ok(media) => match media.media_type {
            Some(anilist_moe::enums::media::MediaType::Manga) => {
                Ok(AniListResponse::success(media))
            }
            _ => Ok(AniListResponse::error(
                "Manga not found (wrong type)".to_string(),
            )),
        },
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
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
create_anilist_command!(fetch_basic, user, fetch_basic, User);

#[tauri::command]
pub async fn get_user_by_id(
    id: i32,
    service: State<'_, AniListState>,
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

/// Get user by username.
/// The crate's `get_by_name()` calls `fetch_one` which uses a `GetUserById($id: Int!)` query
/// and completely ignores the `name` field, producing a 400 error. We work around this by
/// using `fetch()` (the `SearchUsers` query that has an optional `$name: String`) to resolve
/// the numeric user ID first, then fetching the full profile via `get_by_id()`.
#[tauri::command]
pub async fn get_user_by_name(
    name: &str,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<User>, String> {
    log::info!("Executing command: get_user_by_name for name: {}", name);
    let client = service.client().await;

    // Step 1: resolve the user ID with the SearchUsers query (supports $name: String)
    let search_result = client
        .user()
        .fetch(FetchUserOptions {
            name: Some(name.to_string()),
            per_page: Some(1),
            include_statistics: Some(true),
            ..Default::default()
        })
        .await;

    match search_result {
        Ok(page) => {
            if let Some(user) = page.data.into_iter().next() {
                let user_id = user.id;
                // Step 2: fetch full profile by ID (GetUserById query)
                let full_result = client.user().get_by_id(user_id).await;
                match full_result {
                    Ok(full_user) => Ok(AniListResponse::success(full_user)),
                    Err(e) => {
                        log::error!(
                            "Error fetching full user by id {} (name '{}'): {:?}",
                            user_id,
                            name,
                            e
                        );
                        Ok(AniListResponse::error(format!("{:?}", e)))
                    }
                }
            } else {
                Ok(AniListResponse::error(format!("User '{}' not found", name)))
            }
        }
        Err(e) => {
            log::error!("Error searching for user '{}': {:?}", name, e);
            Ok(AniListResponse::error(format!("{:?}", e)))
        }
    }
}

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
    log::info!(
        "Executing command: search_characters is_birthday={:?}",
        is_birthday
    );
    let client = service.client().await;
    let result = client
        .character()
        .fetch(&FetchCharacterOptions {
            search: Some(query.to_string()),
            page,
            per_page,
            is_birthday,
            sort: Some(vec![CharacterSort::SearchMatch]),
            ..Default::default()
        })
        .await;
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
    log::info!(
        "Executing command: search_staff is_birthday={:?}",
        is_birthday
    );
    let client = service.client().await;
    let result = client
        .staff()
        .fetch(&FetchStaffOptions {
            search: Some(query.to_string()),
            sort: Some(vec![StaffSort::SearchMatch]),
            page,
            per_page,
            is_birthday,
            ..Default::default()
        })
        .await;
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
    log::info!(
        "Executing command: get_seasonal_anime season={} year={}",
        season,
        year
    );

    let season_enum = match season.as_str() {
        "WINTER" => MediaSeason::Winter,
        "SPRING" => MediaSeason::Spring,
        "SUMMER" => MediaSeason::Summer,
        "FALL" => MediaSeason::Fall,
        _ => MediaSeason::Spring,
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

// ============================================================================
// Combined Search Command — one GraphQL round-trip for all categories
// ============================================================================

static SEARCH_ALL_GQL: &str = r#"
query SearchAll($search: String!, $perPage: Int) {
  anime: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    media(search: $search, type: ANIME, sort: SEARCH_MATCH) {
      id type format status meanScore
      title { romaji english userPreferred }
      coverImage { medium }
    }
  }
  manga: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    media(search: $search, type: MANGA, sort: SEARCH_MATCH) {
      id type format status meanScore
      title { romaji english userPreferred }
      coverImage { medium }
    }
  }
  characters: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    characters(search: $search, sort: SEARCH_MATCH) {
      id
      name { full userPreferred }
      image { medium }
    }
  }
  staff: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    staff(search: $search, sort: SEARCH_MATCH) {
      id
      name { full userPreferred }
      image { medium }
    }
  }
  studios: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    studios(search: $search) {
      id name isAnimationStudio
    }
  }
  users: Page(perPage: $perPage) {
    pageInfo { currentPage hasNextPage lastPage perPage total }
    users(search: $search) {
      id name
      avatar { medium }
    }
  }
}
"#;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCoverImage {
    pub medium: Option<String>,
}

/// Lean anime/manga result — only the fields requested in `SEARCH_ALL_GQL`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMediaResult {
    pub id: Option<i32>,
    /// "ANIME" or "MANGA"
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    /// e.g. "TV", "OVA", "MOVIE" …
    pub format: Option<String>,
    /// e.g. "FINISHED", "RELEASING" …
    pub status: Option<String>,
    pub mean_score: Option<i32>,
    pub title: Option<SearchTitle>,
    pub cover_image: Option<SearchCoverImage>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPersonName {
    pub full: Option<String>,
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPersonImage {
    pub medium: Option<String>,
}

/// Lean character result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCharacterResult {
    pub id: Option<i32>,
    pub name: Option<SearchPersonName>,
    pub image: Option<SearchPersonImage>,
}

/// Lean staff result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStaffResult {
    pub id: Option<i32>,
    pub name: Option<SearchPersonName>,
    pub image: Option<SearchPersonImage>,
}

/// Lean studio result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStudioResult {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub is_animation_studio: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchUserAvatar {
    pub medium: Option<String>,
}

/// Lean user result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchUserResult {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub avatar: Option<SearchUserAvatar>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchAllResults {
    pub anime: Page<Vec<SearchMediaResult>>,
    pub manga: Page<Vec<SearchMediaResult>>,
    pub characters: Page<Vec<SearchCharacterResult>>,
    pub staff: Page<Vec<SearchStaffResult>>,
    pub studios: Page<Vec<SearchStudioResult>>,
    pub users: Page<Vec<SearchUserResult>>,
}

#[tauri::command]
pub async fn favourite_character(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Favourites>, String> {
    log::info!("Executing command: favourite_character id={}", id);
    let client = service.client().await;
    match client.common().favourite_character(id).await {
        Ok(favs) => Ok(AniListResponse::success(favs)),
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
    }
}

#[tauri::command]
pub async fn favourite_staff(
    id: i32,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<Favourites>, String> {
    log::info!("Executing command: favourite_staff id={}", id);
    let client = service.client().await;
    match client.common().favourite_staff(id).await {
        Ok(favs) => Ok(AniListResponse::success(favs)),
        Err(e) => Ok(AniListResponse::error(format!("{:?}", e))),
    }
}

#[tauri::command]
pub async fn search_all(
    query: String,
    per_page: Option<i32>,
    service: State<'_, AniListState>,
) -> Result<AniListResponse<SearchAllResults>, String> {
    log::info!("Executing command: search_all query={}", query);
    search_all_inner(&query, per_page.unwrap_or(5), &service).await
}

/// Core logic for `search_all`, decoupled from Tauri `State` so it can be
/// called directly in integration tests with a real `AniListService`.
pub(crate) async fn search_all_inner(
    query: &str,
    per_page: i32,
    service: &AniListState,
) -> Result<AniListResponse<SearchAllResults>, String> {
    let variables = serde_json::json!({
        "search": query,
        "perPage": per_page,
    });

    let client = service.client().await;
    let data = match client.query(SEARCH_ALL_GQL, Some(&variables)).await {
        Ok(v) => v,
        Err(e) => return Ok(AniListResponse::error(format!("{:?}", e))),
    };

    match build_search_results(&data["data"]) {
        Ok(results) => Ok(AniListResponse::success(results)),
        Err(e) => Ok(AniListResponse::error(e)),
    }
}

/// Deserialise the `data` portion of the combined search GraphQL response.
/// Extracted to a plain function so it can be unit-tested without Tauri State.
pub(crate) fn build_search_results(d: &serde_json::Value) -> Result<SearchAllResults, String> {
    fn deser<T>(d: &serde_json::Value, key: &str) -> Result<T, String>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_value(d[key].clone())
            .map_err(|e| format!("search_all: failed to decode '{}': {}", key, e))
    }

    Ok(SearchAllResults {
        anime: deser(d, "anime")?,
        manga: deser(d, "manga")?,
        characters: deser(d, "characters")?,
        staff: deser(d, "staff")?,
        studios: deser(d, "studios")?,
        users: deser(d, "users")?,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::anilist::AniListService;
    use std::sync::Arc;

    /// Integration test: calls the real AniList API via `search_all_inner`.
    ///
    /// Requires network access. Searches for "Steins Gate" and asserts that:
    /// - the call succeeds
    /// - at least one anime result is returned with a valid id and title
    /// - all six category lists are present in the response
    #[tokio::test]
    async fn test_search_all_live() {
        let service: AniListState = Arc::new(AniListService::new(None));

        let resp = search_all_inner("Steins Gate", 5, &service)
            .await
            .expect("search_all_inner returned Err");

        assert!(resp.success, "Response not successful: {:?}", resp.error);

        println!("Raw response data: {:?}", resp.data);

        let results = resp.data.expect("data is None on a successful response");

        // Anime results — Steins;Gate should always appear
        assert!(
            !results.anime.data.is_empty(),
            "Expected at least one anime result for 'Steins Gate', got zero"
        );

        let first = &results.anime.data[0];
        assert!(first.id.is_some(), "First anime result has no id");
        let title = first
            .title
            .as_ref()
            .and_then(|t| t.user_preferred.as_deref().or(t.romaji.as_deref()))
            .unwrap_or("");
        assert!(!title.is_empty(), "First anime result title is empty");

        // Pagination metadata should be present (we request it in the GQL query)
        let page_info = results
            .anime
            .page_info
            .as_ref()
            .expect("pageInfo should be returned — it is requested in SEARCH_ALL_GQL");
        assert!(
            page_info.total.unwrap_or(0) > 0,
            "Expected total > 0 for 'Steins Gate' anime search"
        );

        // The other five category lists must at least be initialized (may be empty for
        // this specific query, but the keys must deserialise without error)
        let _ = results.manga;
        let _ = results.characters;
        let _ = results.staff;
        let _ = results.studios;
        let _ = results.users;
    }

    /// Sanity check: an empty query string should still return a successful (but
    /// possibly empty or error) response — not panic or Err out entirely.
    #[tokio::test]
    async fn test_search_all_empty_query() {
        let service: AniListState = Arc::new(AniListService::new(None));
        let resp = search_all_inner("", 5, &service)
            .await
            .expect("search_all_inner must not return Err");
        // AniList may return an error for blank search — that's fine, just
        // make sure we handle it gracefully as AniListResponse::error.
        // The important thing is no panic and no Rust Err.
        let _ = resp;
    }
}
