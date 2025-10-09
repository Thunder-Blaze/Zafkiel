use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Get themes directory path
fn get_themes_dir() -> PathBuf {
    let mut current_dir = std::env::current_dir().unwrap_or_default();

    // If running from src-tauri, go up one level to workspace root
    if current_dir.ends_with("src-tauri") {
        current_dir.pop();
    }

    current_dir.push("static");
    current_dir.push("themes");

    println!("[Themes] Using themes directory: {:?}", current_dir);
    current_dir
}

#[tauri::command]
pub async fn list_themes() -> Result<Vec<String>, String> {
    println!("[Themes] list_themes() called");

    let themes_dir = get_themes_dir();

    if !themes_dir.exists() {
        println!(
            "[Themes] ✗ Themes directory does not exist: {:?}",
            themes_dir
        );
        return Err(format!("Themes directory not found: {:?}", themes_dir));
    }

    println!("[Themes] ✓ Themes directory exists: {:?}", themes_dir);

    let entries = match fs::read_dir(&themes_dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("[Themes] ✗ Failed to read themes directory: {}", e);
            return Err(format!("Failed to read themes directory: {}", e));
        }
    };

    let mut theme_ids = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        // Only include directories
        if path.is_dir() {
            if let Some(theme_id) = path.file_name().and_then(|n| n.to_str()) {
                let css_path = path.join("index.css");

                // Only include themes that have index.css
                if css_path.exists() {
                    println!("[Themes] ✓ Found theme: {}", theme_id);
                    theme_ids.push(theme_id.to_string());
                } else {
                    println!("[Themes] ⚠ Skipping {} (no index.css)", theme_id);
                }
            }
        }
    }

    theme_ids.sort(); // Sort alphabetically

    println!("[Themes] ✓ Total themes found: {}", theme_ids.len());
    println!("[Themes] Themes: {:?}", theme_ids);

    Ok(theme_ids)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePreference {
    pub theme_id: String,
}

#[tauri::command]
pub async fn save_theme_preference(
    theme_id: String,
    config: tauri::State<'_, std::sync::Arc<crate::config::ConfigLoader>>,
) -> Result<(), String> {
    println!("[Themes] Saving theme preference: {}", theme_id);

    config
        .update_ui_theme(theme_id)
        .map_err(|e| format!("Failed to save theme preference: {}", e))?;

    println!("[Themes] ✓ Theme preference saved successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_theme_preference(
    config: tauri::State<'_, std::sync::Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    println!("[Themes] Getting theme preference");

    let ui_config = config
        .get_ui_config()
        .map_err(|e| format!("Failed to get theme preference: {}", e))?;

    println!("[Themes] ✓ Got theme preference: {}", ui_config.theme);
    Ok(ui_config.theme)
}
