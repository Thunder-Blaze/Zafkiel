mod api;
mod auth;
mod commands;
mod config;
mod database;
mod db_commands;
mod image_cache_commands;

use api::anilist::AniListService;
use auth::anilist::AuthState;
use database::Database;
use std::{sync::Arc, vec};
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, prelude::*, Layer};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Setup file logging
    let log_dir = std::path::PathBuf::from("/tmp/zafkiel");
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");
    
    // Torrent logs
    let torrent_appender = tracing_appender::rolling::daily(&log_dir, "torrent.log");
    let (torrent_nb, _torrent_guard) = tracing_appender::non_blocking(torrent_appender);

    // General logs
    let file_appender = tracing_appender::rolling::daily(&log_dir, "zafkiel.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Filters
    let torrent_filter = tracing_subscriber::filter::Targets::new()
        .with_target("app_lib::commands::torrent", tracing::Level::DEBUG)
        .with_target("librqbit", tracing::Level::DEBUG)
        .with_target("dht", tracing::Level::DEBUG);

    let app_filter = tracing_subscriber::EnvFilter::new("info,zafkiel=debug")
        .add_directive("app_lib::commands::torrent=off".parse().unwrap())
        .add_directive("librqbit=off".parse().unwrap())
        .add_directive("dht=off".parse().unwrap());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(torrent_nb)
                .with_ansi(false)
                .with_filter(torrent_filter)
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(app_filter)
        )
        .init();

    tauri::Builder::default()
        .setup(|app| {
            // Clear old logs
            let log_dir = std::path::PathBuf::from("/tmp/zafkiel");
            if log_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&log_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(ext) = path.extension() {
                            if ext == "log" {
                                let _ = std::fs::remove_file(path);
                            }
                        }
                    }
                }
            }

            // Load environment variables from .env file
            if let Err(e) = dotenvy::dotenv() {
                log::warn!("[Setup] Failed to load .env file: {}. Make sure .env exists in the project root with ANILIST_CLIENT_ID and ANILIST_CLIENT_SECRET", e);
            } else {
                log::info!("[Setup] Successfully loaded .env file");
            }

            // Setup file logging - REMOVED (moved to run())
            
            // Initialize config loader
            let config_loader =
                config::ConfigLoader::new().expect("Failed to initialize config loader");

            // Load decrypted token from config if available
            let token = config_loader
                .get_anilist_token()
                .ok()
                .flatten(); // flatten converts Option<Option<String>> to Option<String>

            if token.is_some() {
                log::info!("[Setup] Found existing AniList token in config (decrypted)");
            } else {
                log::info!("[Setup] No AniList token found, user needs to authenticate");
            }

            // Store config loader in app state
            app.manage(Arc::new(config_loader));

            // Initialize AniList service with token from config
            // This maintains a single AniListClient instance in app state
            let anilist_service = AniListService::new(token);
            app.manage(Arc::new(anilist_service));

            // Initialize auth state for OAuth flow
            let auth_state = AuthState::new();
            app.manage(auth_state);

            // Initialize database
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join("zafkiel.db");

            log::info!("[Setup] Database path: {:?}", db_path);

            // Create parent directory if it doesn't exist
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent).expect("Failed to create app data directory");
            }

            let database = Database::new(db_path.clone()).expect("Failed to initialize database");
            app.manage(database);
            log::info!("[Setup] Database initialized at: {:?}", db_path);

            // Initialize Global Torrent Session
            let download_dir = app.path().download_dir().unwrap_or(std::path::PathBuf::from("downloads")).join("zafkiel");
            std::fs::create_dir_all(&download_dir).expect("Failed to create download directory");
            
            let session = tauri::async_runtime::block_on(async {
                librqbit::Session::new(download_dir).await
            }).expect("Failed to create torrent session");
            
            app.manage(session.clone()); // Session::new returns Arc<Session>
            log::info!("[Setup] Global torrent session initialized");

            // Restore torrents from persistence
            let app_handle = app.handle().clone();
            let session_clone = session.clone();
            tauri::async_runtime::spawn(async move {
                commands::torrent::restore_torrents(&app_handle, &session_clone).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::config::get_config,
            commands::config::get_anilist_token,
            commands::config::set_anilist_token,
            commands::config::clear_anilist_token,
            commands::config::get_ui_config,
            commands::config::update_ui_config,
            commands::config::update_theme,
            commands::config::update_glow_effects,
            commands::config::update_blur_effects,
            commands::config::update_animations,
            commands::config::update_smooth_scroll,
            commands::config::update_hover_card,
            commands::config::update_theme_mode,
            commands::config::update_ui_scale,
            commands::config::apply_ui_scale,
            commands::config::open_devtools,
            commands::config::get_config_path,
            commands::config::get_themes_with_paths,

            // Auth commands
            commands::auth::start_oauth_flow,
            commands::auth::open_auth_browser,
            commands::auth::wait_for_oauth_callback,
            commands::auth::check_auth_status,
            commands::auth::logout,

            // API commands
            // Media commands
            commands::api::anilist::search_media,
            commands::api::anilist::get_media_by_id,
            commands::api::anilist::get_anime_by_id,
            commands::api::anilist::get_manga_by_id,
						// Anime commands
            commands::api::anilist::get_trending_anime,
            commands::api::anilist::get_popular_anime,
            commands::api::anilist::get_upcoming_anime,
            commands::api::anilist::get_airing_anime,
            // Manga commands
            commands::api::anilist::get_trending_manga,
            commands::api::anilist::get_popular_manga,
            // User commands
            commands::api::anilist::get_current_user,
            commands::api::anilist::get_user_by_id,
            commands::api::anilist::get_user_by_name,
            commands::api::anilist::search_users,
            // Studio commands
            commands::api::anilist::get_studio_by_id,
            // Character commands
            commands::api::anilist::get_character_by_id,
            // Staff commands
            commands::api::anilist::get_staff_by_id,
            // Image cache commands
            image_cache_commands::download_image,
            image_cache_commands::file_exists,
            image_cache_commands::delete_file,
            image_cache_commands::get_cache_stats,
            image_cache_commands::cleanup_image_cache,
            image_cache_commands::get_cached_file_path,
            // Database commands
            db_commands::update_local_progress,
            db_commands::cache_media,
            db_commands::cache_user,
            db_commands::add_to_recently_viewed,
            db_commands::get_recently_viewed,
            db_commands::search_cached_media,
            db_commands::cleanup_cache,
            db_commands::get_all_cached_images,
            // Image database commands
            db_commands::get_cached_image_path,
            db_commands::cache_image,
            db_commands::remove_cached_image,
            // Utils
            commands::utils::fetch_url,
            commands::torrent::stream_torrent,
            commands::torrent::stream_torrent_by_id,
            commands::torrent::open_in_external_player,
            commands::torrent::get_torrents,
            commands::torrent::pause_torrent,
            commands::torrent::resume_torrent,
            commands::torrent::delete_torrent,
            commands::torrent::get_stream_base_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
