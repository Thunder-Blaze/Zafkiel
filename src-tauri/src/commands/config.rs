use crate::config::{AppConfig, ConfigLoader, PlayerConfig, UiConfig};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tauri::{Manager, State};

/// Shared state for config loader
pub type ConfigState = Arc<ConfigLoader>;

/// In-memory theme cache. Populated on first call to `get_themes_with_paths`;
/// cleared by `invalidate_theme_cache` when themes are installed/removed.
pub struct ThemeCache(pub Mutex<Option<HashMap<String, String>>>);
impl ThemeCache {
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }
}

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
pub fn get_config(app: tauri::AppHandle, config: State<ConfigState>) -> ConfigResponse<AppConfig> {
    match config.get_config() {
        Ok(mut cfg) => {
            resolve_shader_paths(&app, &mut cfg.player.shaders.selected_shaders);
            ConfigResponse::success(cfg)
        }
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

/// Update activity feed tab and filter preferences
#[tauri::command]
pub fn update_activity_prefs(
    tab: String,
    filter: String,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_activity_prefs(tab, filter) {
        Ok(_) => ConfigResponse::success(()),
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

/// Get player configuration
#[tauri::command]
pub fn get_player_config(
    app: tauri::AppHandle,
    config: State<ConfigState>,
) -> ConfigResponse<PlayerConfig> {
    match config.get_player_config() {
        Ok(mut player_config) => {
            resolve_shader_paths(&app, &mut player_config.shaders.selected_shaders);
            ConfigResponse::success(player_config)
        }
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update external player executable path (None = use system default "mpv")
#[tauri::command]
pub fn update_external_player_path(
    path: Option<String>,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_external_player_path(path) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update auto-select next stream setting
#[tauri::command]
pub fn update_auto_select_next_stream(
    enabled: bool,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_auto_select_next_stream(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update playback speed setting
#[tauri::command]
pub fn update_playback_speed(speed: f32, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.update_playback_speed(speed) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update shader configuration
#[tauri::command]
pub fn update_shader_config(
    enabled: bool,
    selected_shaders: Vec<String>,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_shader_config(enabled, selected_shaders) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update auto-update progress setting
#[tauri::command]
pub fn update_auto_update_progress(
    enabled: bool,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_auto_update_progress(enabled) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update auto-update threshold setting
#[tauri::command]
pub fn update_auto_update_threshold(
    threshold: f32,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_auto_update_threshold(threshold) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Update default update mode setting
#[tauri::command]
pub fn update_default_update_mode(
    mode: String,
    config: State<ConfigState>,
) -> ConfigResponse<()> {
    match config.update_default_update_mode(mode) {
        Ok(_) => ConfigResponse::success(()),
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Helper to resolve ~~/shaders prefix to absolute paths
fn resolve_shader_paths(app: &tauri::AppHandle, shaders: &mut Vec<String>) {
    if let Ok(res_dir) = app.path().resource_dir() {
        let shaders_dir = res_dir.join("shaders");
        for shader in shaders.iter_mut() {
            if shader.starts_with("~~/shaders/") {
                if let Some(name) = shader.strip_prefix("~~/shaders/") {
                    let abs_path = shaders_dir.join(name);
                    let normalized = normalize_path_for_asset(&abs_path);
                    let path_str = normalized.replace("\\", "/");
                    println!("[Config] Resolved {} -> {}", shader, path_str);
                    *shader = path_str;
                }
            }
        }
    }
}

/// Get available shaders from the bundled resources
#[tauri::command]
pub fn get_available_shaders(app: tauri::AppHandle) -> ConfigResponse<Vec<String>> {
    let mut shaders = Vec::new();
    if let Ok(res_dir) = app.path().resource_dir() {
        let shaders_dir = res_dir.join("shaders");
        if let Ok(entries) = std::fs::read_dir(&shaders_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "glsl" {
                                // Join the absolute dir path with the file name to get a full absolute path
                                let abs_path = entry.path();
                                // Clean up Windows UNC prefixes and normalize slashes for mpv
                                let normalized = normalize_path_for_asset(&abs_path);
                                let path_str = normalized.replace("\\", "/");
                                shaders.push(path_str);
                            }
                        }
                    }
                }
            }
        }
    }
    ConfigResponse::success(shaders)
}

/// Converts a filesystem `Path` to a plain UTF-8 string that Tauri's
/// `convertFileSrc` can consume on every OS.
///
/// On Windows, `fs::read_dir` can return paths with the verbatim
/// extended-length prefix `\\?\` (e.g. `\\?\D:\...`). That prefix gets
/// URL-encoded verbatim and produces a broken asset URL.  Stripping it
/// before handing the string to the frontend fixes the issue without
/// needing an extra crate.
fn normalize_path_for_asset(path: &std::path::Path) -> String {
    let s: String = path.to_string_lossy().into_owned();
    #[cfg(windows)]
    {
        if let Some(stripped) = s.strip_prefix("\\\\?\\") {
            return stripped.to_string();
        }
    }
    s
}

/// Path to the user-installed themes index (`~/.config/zafkiel/themes.index.json`).
/// This file is only written by the theme-install system; default themes are
/// listed in `static/themes/themes.index.json` which ships with the app.
fn user_themes_index_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "zafkiel").map(|d| d.config_dir().join("themes.index.json"))
}

/// Read the user-installed themes index from `~/.config/zafkiel/themes.index.json`.
/// Returns an empty map if the file is absent or unreadable – never an error.
fn read_user_themes_index() -> HashMap<String, String> {
    let Some(path) = user_themes_index_path() else {
        return HashMap::new();
    };
    if !path.exists() {
        return HashMap::new();
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("[Themes] Could not read user themes index: {e}");
            return HashMap::new();
        }
    };
    serde_json::from_str(&content).unwrap_or_else(|e| {
        log::warn!("[Themes] Could not parse user themes index: {e}");
        HashMap::new()
    })
}

/// Persist the user themes index to `~/.config/zafkiel/themes.index.json`.
/// Called by the theme-install/uninstall commands – not by the reader path.
#[allow(dead_code)]
pub fn write_user_themes_index(themes: &HashMap<String, String>) {
    let Some(path) = user_themes_index_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(themes) {
        Ok(content) => {
            if let Err(e) = fs::write(&path, content) {
                log::warn!("[Themes] Failed to write user themes index: {e}");
            } else {
                log::info!(
                    "[Themes] User themes index updated ({} entries)",
                    themes.len()
                );
            }
        }
        Err(e) => log::warn!("[Themes] Failed to serialize user themes index: {e}"),
    }
}

/// Build the merged theme map:
///   1. Start with built-in themes (IDs from `static/themes/themes.index.json`,
///      paths resolved against the app resource directory).
///   2. Overlay user-installed themes from `~/.config/zafkiel/themes.index.json`
///      (user entries win on ID collision).
///
/// No directory scanning is performed.  Path validity is intentionally NOT
/// checked here – errors are surfaced lazily when the frontend tries to load
/// the CSS file for a specific theme.
fn build_theme_map(app: &tauri::AppHandle) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    // ── 1. Built-in themes ───────────────────────────────────────────────────
    let resource_dir = app.path().resource_dir().ok();

    if let Some(ref res_dir) = resource_dir {
        let index_path = res_dir.join("themes").join("themes.index.json");
        match fs::read_to_string(&index_path) {
            Ok(content) => {
                let ids: Vec<String> = serde_json::from_str(&content).unwrap_or_else(|e| {
                    log::warn!("[Themes] Could not parse built-in themes index: {e}");
                    vec![]
                });
                for id in ids {
                    let theme_dir = res_dir.join("themes").join(&id);
                    map.insert(id, normalize_path_for_asset(&theme_dir));
                }
                log::info!("[Themes] Loaded {} built-in theme(s) from index", map.len());
            }
            Err(e) => {
                log::warn!("[Themes] Could not read built-in themes index at {index_path:?}: {e}");
            }
        }
    } else {
        log::warn!("[Themes] Could not resolve resource directory – built-in themes unavailable");
    }

    // ── 2. User themes (override built-ins on ID collision) ──────────────────
    let user_themes = read_user_themes_index();
    let user_count = user_themes.len();
    map.extend(user_themes);

    if user_count > 0 {
        log::info!(
            "[Themes] Merged {user_count} user theme(s) (total: {})",
            map.len()
        );
    }

    map
}

/// Returns a `{id → absolute_dir_path}` map for every known theme.
///
/// Built-in themes come from `static/themes/themes.index.json` (shipped with
/// the app); user-installed themes come from
/// `~/.config/zafkiel/themes.index.json`.  User entries take precedence.
///
/// **Path validity is not checked here.**  If a path is broken the error will
/// surface when the frontend actually tries to load the theme's CSS.
#[tauri::command]
pub fn get_themes_with_paths(
    app: tauri::AppHandle,
    cache: tauri::State<'_, ThemeCache>,
) -> ConfigResponse<HashMap<String, String>> {
    // L1: in-memory cache – avoids re-reading files on every settings open
    {
        let guard = cache.0.lock().expect("theme cache poisoned");
        if let Some(cached) = guard.as_ref() {
            log::debug!("[Themes] cache hit – {} theme(s)", cached.len());
            return ConfigResponse::success(cached.clone());
        }
    }

    let themes = build_theme_map(&app);
    *cache.0.lock().expect("theme cache poisoned") = Some(themes.clone());
    ConfigResponse::success(themes)
}

/// Clears the in-memory theme cache so the next call to `get_themes_with_paths`
/// re-reads the index files.  Call this after installing or removing a user theme.
#[tauri::command]
pub fn invalidate_theme_cache(cache: tauri::State<'_, ThemeCache>) {
    *cache.0.lock().expect("theme cache poisoned") = None;
    log::info!("[Themes] In-memory cache cleared");
}
