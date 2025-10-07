pub mod encryption;
pub mod loader;
pub mod types;

pub use loader::ConfigLoader;
pub use types::{AppConfig, UiConfig};

use directories::ProjectDirs;
use std::{fs, path::PathBuf};

/// Get the config directory path
pub fn get_config_dir() -> Result<PathBuf, String> {
    ProjectDirs::from("", "", "zafkiel")
        .map(|pd| pd.config_dir().to_path_buf())
        .ok_or_else(|| "Failed to determine config directory".to_string())
}

/// Get the config file path
pub fn get_config_path() -> Result<PathBuf, String> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_filename = if cfg!(test) {
        "config.test.ron"
    } else if cfg!(debug_assertions) {
        "config.debug.ron"
    } else {
        "config.ron"
    };

    Ok(config_dir.join(config_filename))
}

/// Load config from file or return default
pub fn load_or_default() -> Result<AppConfig, String> {
    let path = get_config_path()?;

    if path.exists() {
        let contents =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: AppConfig =
            ron::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))?;
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

/// Save config to file
pub fn save(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path()?;
    let ron_string = ron::ser::to_string_pretty(config, Default::default())
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&path, ron_string).map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}
