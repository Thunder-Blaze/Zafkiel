mod api;
mod auth;
mod commands;
mod config;
mod constants;
mod database;

use api::anilist::AniListService;
use auth::anilist::AuthState;
use constants::DATABASE_URL;
use database::Database;
use std::sync::Arc;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

fn setup_libs_early() {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let lib_dir = dir.join("lib");

            // DLL aliasing was removed as actual binaries should be present via Git LFS.


            #[cfg(target_os = "windows")]
            {
                let mut path = std::env::var("PATH").unwrap_or_default();
                path = format!("{};{}", lib_dir.display(), path);
                unsafe { std::env::set_var("PATH", path); }
            }

            #[cfg(target_os = "linux")]
            {
                let mut path = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
                path = format!("{}:{}", lib_dir.display(), path);
                unsafe { std::env::set_var("LD_LIBRARY_PATH", path); }
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Log directory: use XDG-compatible location instead of /tmp so logs
    // survive reboots. Falls back to /tmp only if the directory can't be
    // created by the OS path resolver (pre-app-handle stage).
    let log_dir = directories::ProjectDirs::from("com", "zafkiel", "Zafkiel")
        .map(|dirs| dirs.data_local_dir().join("logs"))
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp/zafkiel/logs"));
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    // Torrent logs
    let torrent_appender = tracing_appender::rolling::daily(&log_dir, "torrent.log");
    let (torrent_nb, _torrent_guard) = tracing_appender::non_blocking(torrent_appender);

    // General logs
    let file_appender = tracing_appender::rolling::daily(&log_dir, "zafkiel.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

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
                .with_filter(torrent_filter),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(app_filter),
        )
        .init();
    
    setup_libs_early();

    tauri::Builder::default()
        .plugin(tauri_plugin_libmpv::init())
        .setup(|app| {
            // Initialize config loader
            let config_loader =
                config::ConfigLoader::new().expect("Failed to initialize config loader");

            // Load decrypted token from config if available
            let token = config_loader.get_anilist_token().ok().flatten();

            if token.is_some() {
                log::info!("[Setup] Found existing AniList token in config (decrypted)");
            } else {
                log::info!("[Setup] No AniList token found, user needs to authenticate");
            }

            app.manage(Arc::new(config_loader));

            // Theme scan cache (populated lazily on first call to get_themes_with_paths)
            app.manage(commands::config::ThemeCache::new());

            // AniList service — simplified, no RwLock (AniListClient is already Clone+Arc internally)
            let anilist_service = AniListService::new(token);
            app.manage(Arc::new(anilist_service));

            // OAuth state
            let auth_state = AuthState::new();
            app.manage(auth_state);

            // Database — now uses R2D2 pool + versioned migrations
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join(DATABASE_URL);

            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)
                    .expect("Failed to create app data directory");
            }

            let database =
                Database::new(db_path.clone()).expect("Failed to initialize database");
            app.manage(database);
            log::info!("[Setup] Database initialized at: {:?}", db_path);

            // Torrent session — init on blocking thread to avoid block_on issues
            let download_dir = app
                .path()
                .download_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("downloads"))
                .join("zafkiel");
            std::fs::create_dir_all(&download_dir)
                .expect("Failed to create download directory");

            let session = tauri::async_runtime::block_on(async {
                match librqbit::Session::new(download_dir.clone()).await {
                    Ok(s) => Ok(s),
                    Err(e) => {
                        log::warn!(
                            "[Setup] Torrent session init failed ({}), retrying without persistent DHT...",
                            e
                        );
                        librqbit::Session::new_with_opts(
                            download_dir,
                            librqbit::SessionOptions {
                                disable_dht_persistence: true,
                                ..Default::default()
                            },
                        )
                        .await
                    }
                }
            })
            .expect("Failed to create torrent session");

            app.manage(session.clone());
            log::info!("[Setup] Global torrent session initialized");

            // Initialize Discord RPC State
            app.manage(commands::discord::DiscordState::new());

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
            commands::config::update_activity_prefs,
            commands::config::open_devtools,
            commands::config::get_config_path,
            commands::config::get_themes_with_paths,
            commands::config::invalidate_theme_cache,
            commands::config::get_player_config,
            commands::config::update_external_player_path,
            commands::config::update_auto_select_next_stream,
            commands::config::update_playback_speed,
            commands::config::update_shader_config,
            commands::config::get_available_shaders,

            // ─── Extensions ──────────────────────────────────────────────
            commands::extensions::get_installed_extensions,
            commands::extensions::install_extension,
            commands::extensions::install_extension_from_local,
            commands::extensions::reinstall_extension,
            commands::extensions::uninstall_extension,
            commands::extensions::get_extension_entry_path,
            commands::extensions::ext_storage_get,
            commands::extensions::ext_storage_set,
            commands::extensions::ext_storage_delete,
            commands::extensions::open_extension_auth_webview,
            commands::extensions::collect_cdn_cookies_for_kwik,

            // Auth commands
            commands::auth::start_oauth_flow,
            commands::auth::open_auth_browser,
            commands::auth::wait_for_oauth_callback,
            commands::auth::check_auth_status,
            commands::auth::logout,

            // ─── Media / Browse ──────────────────────────────────────────
            commands::api::anilist::search_media,
            commands::api::anilist::browse_media,
            commands::api::anilist::get_media_by_id,
            commands::api::anilist::get_anime_by_id,
            commands::api::anilist::get_anime_characters_by_id,
            commands::api::anilist::get_anime_staff_by_id,
            commands::api::anilist::get_manga_by_id,
            commands::api::anilist::get_trending_anime,
            commands::api::anilist::get_popular_anime,
            commands::api::anilist::get_upcoming_anime,
            commands::api::anilist::get_airing_anime,
            commands::api::anilist::get_trending_manga,
            commands::api::anilist::get_popular_manga,
            // User commands
            commands::api::anilist::get_current_user,
            commands::api::anilist::fetch_basic,
            commands::api::anilist::get_user_by_id,
            commands::api::anilist::get_user_by_name,
            commands::api::anilist::search_users,
            // Studio / Character / Staff
            commands::api::anilist::get_studio_by_id,
            commands::api::anilist::get_character_by_id,
            commands::api::anilist::get_popular_characters,
            commands::api::anilist::get_birthday_characters,
            commands::api::anilist::search_characters,
            commands::api::anilist::get_staff_by_id,
            commands::api::anilist::get_popular_staff,
            commands::api::anilist::get_birthday_staff,
            commands::api::anilist::search_staff,
            commands::api::anilist::favourite_character,
            commands::api::anilist::favourite_staff,
            commands::api::anilist::get_seasonal_anime,
            commands::api::anilist::search_all,

            // ─── MediaList ───────────────────────────────────────────────
            commands::api::medialist::fetch_media_list,
            commands::api::medialist::get_my_anime_list,
            commands::api::medialist::get_my_manga_list,
            commands::api::medialist::get_user_anime_list,
            commands::api::medialist::get_user_manga_list,
            commands::api::medialist::get_watching,
            commands::api::medialist::get_reading,
            commands::api::medialist::get_plan_to_watch,
            commands::api::medialist::get_plan_to_read,
            commands::api::medialist::get_completed_anime,
            commands::api::medialist::get_completed_manga,
            commands::api::medialist::save_media_list_entry,
            commands::api::medialist::add_anime_to_list,
            commands::api::medialist::add_manga_to_list,
            commands::api::medialist::update_media_progress,
            commands::api::medialist::update_media_score,
            commands::api::medialist::update_media_status,
            commands::api::medialist::delete_media_list_entry,

            // ─── Activity ────────────────────────────────────────────────
            commands::api::activity::fetch_activities,
            commands::api::activity::get_activity_by_id,
            commands::api::activity::get_recent_activity,
            commands::api::activity::get_following_activity,
            commands::api::activity::fetch_activity_replies,
            commands::api::activity::save_text_activity,
            commands::api::activity::save_message_activity,
            commands::api::activity::save_activity_reply,
            commands::api::activity::delete_activity,
            commands::api::activity::delete_activity_reply,
            commands::api::activity::toggle_activity_subscription,

            // ─── Utilities ──────────────────────────────────────────────
            commands::utils::fetch_url,
            commands::utils::post_url,
            commands::utils::upload_to_catbox,
            commands::utils::fetch_image_base64,
            commands::utils::fetch_bytes_base64,

            // ─── Notifications ───────────────────────────────────────────
            commands::api::notification::fetch_notifications,
            commands::api::notification::get_all_notifications,
            commands::api::notification::get_and_mark_notifications_read,

            // ─── Reviews ─────────────────────────────────────────────────
            commands::api::review::fetch_reviews,
            commands::api::review::get_reviews_by_media,
            commands::api::review::get_reviews_by_user,
            commands::api::review::get_review_by_id,
            commands::api::review::get_recent_reviews,
            commands::api::review::save_review,
            commands::api::review::delete_review,
            commands::api::review::rate_review,

            // ─── Recommendations ─────────────────────────────────────────
            commands::api::recommendation::fetch_recommendations,
            commands::api::recommendation::get_recommendations_by_media,
            commands::api::recommendation::save_recommendation,

            // ─── Forum ───────────────────────────────────────────────────
            commands::api::forum::search_forum_threads,
            commands::api::forum::get_forum_thread,
            commands::api::forum::get_recent_forum_threads,
            commands::api::forum::get_popular_forum_threads,
            commands::api::forum::get_forum_threads_by_category,
            commands::api::forum::get_forum_threads_by_user,
            commands::api::forum::get_subscribed_forum_threads,
            commands::api::forum::get_thread_comments,
            commands::api::forum::get_thread_comment_by_id,
            commands::api::forum::save_forum_thread,
            commands::api::forum::delete_forum_thread,
            commands::api::forum::save_thread_comment,
            commands::api::forum::delete_thread_comment,
            commands::api::forum::toggle_forum_thread_subscription,
            commands::api::forum::reply_to_forum_thread,
            commands::api::forum::reply_to_thread_comment,
            commands::api::common::toggle_like_thread,
            commands::api::common::toggle_like_thread_comment,

            // ─── Image cache ─────────────────────────────────────────────
            commands::image_cache::download_image,
            commands::image_cache::file_exists,
            commands::image_cache::delete_file,
            commands::image_cache::get_cache_stats,
            commands::image_cache::cleanup_image_cache,
            commands::image_cache::get_cached_file_path,

            // ─── Database ────────────────────────────────────────────────
            commands::db::update_local_progress,
            commands::db::cache_media,
            commands::db::cache_user,
            commands::db::add_to_recently_viewed,
            commands::db::get_recently_viewed,
            commands::db::search_cached_media,
            commands::db::cleanup_cache,
            commands::db::get_all_cached_images,
            commands::db::get_cached_image_path,
            commands::db::cache_image,
            commands::db::remove_cached_image,

            // ─── Torrent ─────────────────────────────────────────────────
            commands::torrent::stream_torrent,
            commands::torrent::stream_torrent_by_id,
            commands::torrent::open_in_external_player,
            commands::torrent::get_torrents,
            commands::torrent::get_torrent_files,
            commands::torrent::get_torrent_files_by_id,
            commands::torrent::pause_torrent,
            commands::torrent::resume_torrent,
            commands::torrent::delete_torrent,
            commands::torrent::get_stream_base_url,

            // ─── Extension downloads ──────────────────────────────────────
            commands::downloads::start_extension_download,
            commands::downloads::get_extension_downloads,
            commands::downloads::cancel_extension_download,
            commands::downloads::remove_extension_download,

            // ─── MPV window helpers ───────────────────────────────────────
            commands::mpv_window::lower_mpv_subwindow,

            // ─── Discord RPC ──────────────────────────────────────────────
            commands::discord::set_discord_activity,
            commands::discord::clear_discord_activity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
