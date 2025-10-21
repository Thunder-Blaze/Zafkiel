use crate::config::{AppConfig, ConfigLoader, UiConfig};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tauri::{Manager, State};

/// Shared state for config loader
pub type ConfigState = Arc<ConfigLoader>;

/// Response type for config operations
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ConfigResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

/// Get the full application config
#[tauri::command]
pub fn get_config(config: State<ConfigState>) -> ConfigResponse<AppConfig> {
    match config.get_config() {
        Ok(cfg) => ConfigResponse::success(cfg),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Get the decrypted AniList access token
#[tauri::command]
pub fn get_anilist_token(config: State<ConfigState>) -> ConfigResponse<Option<String>> {
    match config.get_anilist_token() {
        Ok(token) => ConfigResponse::success(token),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Set the AniList access token (will be encrypted)
#[tauri::command]
pub fn set_anilist_token(token: String, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.set_anilist_token(&token) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Clear the AniList access token
#[tauri::command]
pub fn clear_anilist_token(config: State<ConfigState>) -> ConfigResponse<()> {
    match config.clear_anilist_token() {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Get UI configuration
#[tauri::command]
pub fn get_ui_config(config: State<ConfigState>) -> ConfigResponse<UiConfig> {
    match config.get_ui_config() {
        Ok(ui_config) => ConfigResponse::success(ui_config),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update UI configuration
#[tauri::command]
pub fn update_ui_config(ui_config: UiConfig, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_ui_config(ui_config) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update theme
#[tauri::command]
pub fn update_theme(theme: String, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_ui_theme(theme) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update glow effects setting
#[tauri::command]
pub fn update_glow_effects(enabled: bool, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_glow_effects(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update blur effects setting
#[tauri::command]
pub fn update_blur_effects(enabled: bool, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_blur_effects(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update animations setting
#[tauri::command]
pub fn update_animations(enabled: bool, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_animations(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update smooth scroll setting
#[tauri::command]
pub fn update_smooth_scroll(enabled: bool, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_smooth_scroll(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update hover card previews setting
#[tauri::command]
pub fn update_hover_card(enabled: bool, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_hover_card(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update theme mode (light/dark/system)
#[tauri::command]
pub fn update_theme_mode(mode: String, config: State<ConfigState>) -> ConfigResponse<()> {
    // Validate mode
    if mode != "light" && mode != "dark" && mode != "system" {
        return ConfigResponse::error(
            "Invalid theme mode. Must be 'light', 'dark', or 'system'".to_string(),
        );
    }

    match config.update_theme_mode(mode) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update UI scale factor and apply webview zoom
#[tauri::command]
pub fn update_ui_scale(
    scale: f32,
    config: State<ConfigState>,
    app: tauri::AppHandle,
) -> ConfigResponse<()> {
    // Clamp scale between 0.5 and 2.0
    let clamped_scale = scale.max(0.5).min(2.0);

    // Save to config
    match config.update_ui_scale(clamped_scale) {
        Ok(_) => {
            // Apply zoom to webview
            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = window.set_zoom(clamped_scale as f64) {
                    return ConfigResponse::error(format!("Failed to set zoom: {}", e));
                }
            }
            ConfigResponse::success(())
        }
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Set webview zoom level on startup
#[tauri::command]
pub fn apply_ui_scale(app: tauri::AppHandle, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.get_ui_config() {
        Ok(ui_config) => {
            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = window.set_zoom(ui_config.ui_scale as f64) {
                    return ConfigResponse::error(format!("Failed to apply zoom: {}", e));
                }
            }
            ConfigResponse::success(())
        }
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Open developer tools (only works in debug mode or with devtools enabled)
#[tauri::command]
pub fn open_devtools(app: tauri::AppHandle) -> ConfigResponse<()> {
    if let Some(window) = app.get_webview_window("main") {
        #[cfg(debug_assertions)]
        {
            if window.is_devtools_open() {
                let _ = window.close_devtools();
            } else {
                let _ = window.open_devtools();
            }
            return ConfigResponse::success(());
        }

        #[cfg(not(debug_assertions))]
        {
            // In production, try to toggle devtools if available
            if window.is_devtools_open() {
                let _ = window.close_devtools();
            } else {
                // Will only work if devtools are enabled in tauri.conf.json
                let _ = window.open_devtools();
            }
            return ConfigResponse::success(());
        }
    }
    ConfigResponse::error("Main window not found".to_string())
}

/// Get config file path (for debugging)
#[tauri::command]
pub fn get_config_path(config: State<ConfigState>) -> ConfigResponse<String> {
    let path = config.get_config_file_path();
    ConfigResponse::success(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_themes_with_paths(app: tauri::AppHandle) -> ConfigResponse<HashMap<String, String>> {
    let mut themes = HashMap::new();
    let project_dirs = ProjectDirs::from("", "", "zafkiel").expect("Failed to get project dirs");
    let resources_dir = app.path().resource_dir();
    let themes_dir_config = project_dirs.config_dir().join("themes");
    // Resources directory as themes location for default themes
    let themes_dir_default = if let Some(res_dir) = resources_dir.unwrap().to_str() {
        Some(PathBuf::from(res_dir).join("themes"))
    } else {
        None
    };
    log::info!(
        "[Themes] Default themes directory: {:?}",
        themes_dir_default
    );
    log::info!("[Themes] Config themes directory: {:?}", themes_dir_config);

    if let Some(themes_dir) = themes_dir_default {
        // Read theme files
        if let Ok(entries) = fs::read_dir(&themes_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // 1. Check if the entry is a directory
                    if path.is_dir() {
                        // 2. Get the theme name (the directory's name)
                        if let Some(theme_name_osstr) = path.file_name() {
                            if let Some(theme_name) = theme_name_osstr.to_str() {
                                // 3. Construct the path to the 'index.css' file
                                let css_path = path.join("index.css");

                                // 4. Check if 'index.css' actually exists
                                if css_path.is_file() {
                                    // 5. Insert the theme name and the path to the css file
                                    themes.insert(
                                        theme_name.to_string(),
                                        path.to_string_lossy().into(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(themes_dir) = Some(themes_dir_config) {
        // Read theme files
        if let Ok(entries) = fs::read_dir(&themes_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    // 1. Check if the entry is a directory
                    if path.is_dir() {
                        // 2. Get the theme name (the directory's name)
                        if let Some(theme_name_osstr) = path.file_name() {
                            if let Some(theme_name) = theme_name_osstr.to_str() {
                                // 3. Construct the path to the 'index.css' file
                                let css_path = path.join("index.css");

                                // 4. Check if 'index.css' actually exists
                                if css_path.is_file() {
                                    // 5. Insert the theme name and the path to the css file
                                    themes.insert(
                                        theme_name.to_string(),
                                        path.to_string_lossy().into(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    ConfigResponse::success(themes)
}
