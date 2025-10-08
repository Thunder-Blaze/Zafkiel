mod anilist;
mod anilist_commands;
mod auth;
mod auth_commands;
mod commands;
mod config;
mod theme_commands;

use anilist::AniListService;
use auth::AuthState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Load environment variables from .env file
            if let Err(e) = dotenvy::dotenv() {
                log::warn!("[Setup] Failed to load .env file: {}. Make sure .env exists in the project root with ANILIST_CLIENT_ID and ANILIST_CLIENT_SECRET", e);
            } else {
                log::info!("[Setup] Successfully loaded .env file");
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize config loader
            let config_loader =
                config::ConfigLoader::new().expect("Failed to initialize config loader");

            // Load token from config if available
            let token = config_loader
                .get_config()
                .ok()
                .and_then(|config| config.anilist.access_token);

            if token.is_some() {
                log::info!("[Setup] Found existing AniList token in config");
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::get_config,
            commands::get_anilist_token,
            commands::set_anilist_token,
            commands::clear_anilist_token,
            commands::get_ui_config,
            commands::update_ui_config,
            commands::update_theme,
            commands::update_glow_effects,
            commands::update_blur_effects,
            commands::update_animations,
            commands::update_smooth_scroll,
            commands::update_ui_scale,
            commands::apply_ui_scale,
            commands::open_devtools,
            commands::get_config_path,
            // Auth commands
            auth_commands::start_oauth_flow,
            auth_commands::open_auth_browser,
            auth_commands::wait_for_oauth_callback,
            auth_commands::check_auth_status,
            auth_commands::logout,
            // Anime commands
            anilist_commands::search_anime,
            anilist_commands::get_anime_by_id,
            anilist_commands::get_trending_anime,
            anilist_commands::get_popular_anime,
            anilist_commands::get_seasonal_anime,
            // Manga commands
            anilist_commands::search_manga,
            anilist_commands::get_manga_by_id,
            anilist_commands::get_trending_manga,
            anilist_commands::get_popular_manga,
            // User commands
            anilist_commands::get_current_user,
            anilist_commands::get_user_by_id,
            anilist_commands::get_user_by_name,
            anilist_commands::search_users,
            // Theme commands
            theme_commands::list_themes,
            theme_commands::save_theme_preference,
            theme_commands::get_theme_preference,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
