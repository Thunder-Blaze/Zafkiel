use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub background: String,
    pub accent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColorModes {
    pub light: ThemeColors,
    pub dark: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub colors: ThemeColorModes,
}

/// Get the themes directory path
fn get_themes_dir() -> Result<PathBuf, String> {
    // In Tauri, we need to use the resource path or current executable directory
    // For development, themes are in static/themes relative to project root
    
    println!("[Theme Commands] Searching for themes directory...");
    
    // Try multiple possible locations
    let mut possible_paths = Vec::new();
    
    // Development: relative to workspace root
    possible_paths.push(PathBuf::from("static/themes"));
    
    // Development: relative to current dir (might be src-tauri)
    if let Ok(current_dir) = std::env::current_dir() {
        // If we're in src-tauri, go up one level
        if current_dir.ends_with("src-tauri") {
            possible_paths.push(current_dir.parent().unwrap().join("static").join("themes"));
        } else {
            possible_paths.push(current_dir.join("static").join("themes"));
        }
    }
    
    // Development: go up from target/debug to workspace root
    if let Ok(current_exe) = std::env::current_exe() {
        // From /path/to/zafkiel/src-tauri/target/debug/Zafkiel
        // Go up 4 levels to workspace root: debug -> target -> src-tauri -> zafkiel
        if let Some(exe_parent) = current_exe.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent()) 
        {
            possible_paths.push(exe_parent.join("static").join("themes"));
        }
    }
    
    println!("[Theme Commands] Trying {} possible paths:", possible_paths.len());
    for (i, path) in possible_paths.iter().enumerate() {
        let exists = path.exists();
        let is_dir = path.is_dir();
        println!("[Theme Commands]   {}. {:?} (exists: {}, is_dir: {})", i + 1, path, exists, is_dir);
        
        if exists && is_dir {
            println!("[Theme Commands] ✓ Found themes directory: {:?}", path);
            return Ok(path.clone());
        }
    }
    
    let error_msg = format!(
        "Themes directory not found. Tried paths: {:?}. Current dir: {:?}, Current exe: {:?}",
        possible_paths,
        std::env::current_dir(),
        std::env::current_exe()
    );
    println!("[Theme Commands] ✗ {}", error_msg);
    Err(error_msg)
}

/// List all available themes
#[tauri::command]
pub async fn list_themes() -> Result<Vec<ThemeMetadata>, String> {
    println!("[Theme Commands] list_themes called");
    
    let themes_dir = get_themes_dir()?;
    println!("[Theme Commands] Using themes directory: {:?}", themes_dir);
    
    let mut themes = Vec::new();

    // Read all subdirectories in themes/
    let entries = fs::read_dir(&themes_dir).map_err(|e| {
        let error = format!("Failed to read themes directory: {}", e);
        println!("[Theme Commands] ✗ {}", error);
        error
    })?;
    
    println!("[Theme Commands] Reading theme directories...");

    for entry in entries.flatten() {
        let path = entry.path();
        
        // Skip if not a directory
        if !path.is_dir() {
            continue;
        }

        let theme_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        println!("[Theme Commands] Checking theme: {}", theme_name);

        // Try to read theme.json
        let theme_json_path = path.join("theme.json");
        if theme_json_path.exists() {
            if let Ok(content) = fs::read_to_string(&theme_json_path) {
                match serde_json::from_str::<ThemeMetadata>(&content) {
                    Ok(metadata) => {
                        println!("[Theme Commands]   ✓ Loaded theme: {} (id: {})", metadata.name, metadata.id);
                        themes.push(metadata);
                    }
                    Err(e) => {
                        println!("[Theme Commands]   ✗ Failed to parse theme.json for {}: {}", theme_name, e);
                    }
                }
            } else {
                println!("[Theme Commands]   ✗ Failed to read theme.json for {}", theme_name);
            }
        } else {
            println!("[Theme Commands]   ✗ No theme.json found for {}", theme_name);
        }
    }

    println!("[Theme Commands] ✓ Loaded {} themes total", themes.len());
    Ok(themes)
}

/// Get metadata for a specific theme
#[tauri::command]
pub async fn get_theme_metadata(theme_id: String) -> Result<ThemeMetadata, String> {
    let themes_dir = get_themes_dir()?;
    let theme_path = themes_dir.join(&theme_id).join("theme.json");

    if !theme_path.exists() {
        return Err(format!("Theme '{}' not found", theme_id));
    }

    let content = fs::read_to_string(theme_path).map_err(|e| e.to_string())?;
    let metadata: ThemeMetadata = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    Ok(metadata)
}

/// Save theme preference to config
#[tauri::command]
pub async fn save_theme_preference(
    theme_id: String,
    config_loader: tauri::State<'_, std::sync::Arc<crate::config::ConfigLoader>>,
) -> Result<(), String> {
    config_loader
        .update_ui_theme(theme_id)
        .map_err(|e| e.to_string())
}

/// Get theme preference from config
#[tauri::command]
pub async fn get_theme_preference(
    config_loader: tauri::State<'_, std::sync::Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    let config = config_loader.get_config().map_err(|e| e.to_string())?;
    Ok(config.ui.theme)
}
