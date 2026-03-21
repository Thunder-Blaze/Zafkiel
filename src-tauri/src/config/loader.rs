use super::{
    encryption::{self, EncryptionError},
    types::{AppConfig, PlayerConfig, UiConfig},
};
use directories::ProjectDirs;
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use thiserror::Error;

/// Config loader errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to determine config directory")]
    NoConfigDir,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] ron::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Encryption error: {0}")]
    Encryption(#[from] EncryptionError),
}

/// Configuration loader with encryption support
pub struct ConfigLoader {
    config_path: PathBuf,
    config: Arc<RwLock<AppConfig>>,
}

impl ConfigLoader {
    /// Create a new config loader
    /// The config will be loaded from ~/.config/zafkiel/config.ron
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create(&config_path)?;

        Ok(Self {
            config_path,
            config: Arc::new(RwLock::new(config)),
        })
    }

    /// Get the config file path
    /// Uses different files based on build profile:
    /// - config.test.ron for tests (cfg(test))
    /// - config.debug.ron for debug builds
    /// - config.ron for release builds
    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let project_dirs = ProjectDirs::from("", "", "zafkiel").ok_or(ConfigError::NoConfigDir)?;
        let config_dir = project_dirs.config_dir();

        // Create config directory if it doesn't exist
        fs::create_dir_all(config_dir)?;

        // Determine config filename based on build profile
        let config_filename = if cfg!(test) {
            "config.test.ron"
        } else if cfg!(debug_assertions) {
            "config.debug.ron"
        } else {
            "config.ron"
        };

        Ok(config_dir.join(config_filename))
    }

    /// Load config from file or create default if not exists
    fn load_or_create(path: &PathBuf) -> Result<AppConfig, ConfigError> {
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let config: AppConfig = ron::from_str(&contents)
                .map_err(|e| ConfigError::Deserialization(e.to_string()))?;
            Ok(config)
        } else {
            let config = AppConfig::default();
            Self::save_config(path, &config)?;
            Ok(config)
        }
    }

    /// Save config to file
    fn save_config(path: &PathBuf, config: &AppConfig) -> Result<(), ConfigError> {
        let ron_string = ron::ser::to_string_pretty(config, Default::default())?;
        fs::write(path, ron_string)?;
        Ok(())
    }

    /// Get a read-only copy of the current config
    pub fn get_config(&self) -> Result<AppConfig, ConfigError> {
        let config = self
            .config
            .read()
            .map_err(|_| ConfigError::Deserialization("Failed to acquire read lock".to_string()))?;
        Ok(config.clone())
    }

    /// Save the current config to disk
    pub fn save(&self) -> Result<(), ConfigError> {
        let config = self
            .config
            .read()
            .map_err(|_| ConfigError::Deserialization("Failed to acquire read lock".to_string()))?;
        Self::save_config(&self.config_path, &config)
    }

    /// Set AniList access token (will be encrypted)
    /// If encryption key is empty, a new one will be generated
    pub fn set_anilist_token(&self, token: &str) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        // Generate encryption key if not exists
        if config.security.encryption_key.is_empty() {
            config.security.encryption_key = encryption::generate_key();
        }

        // Encrypt the token
        let encrypted_token = encryption::encrypt(token, &config.security.encryption_key)?;
        config.anilist.access_token = Some(encrypted_token);

        // Save to disk
        drop(config); // Release write lock before saving
        self.save()
    }

    /// Get decrypted AniList access token
    pub fn get_anilist_token(&self) -> Result<Option<String>, ConfigError> {
        let config = self
            .config
            .read()
            .map_err(|_| ConfigError::Deserialization("Failed to acquire read lock".to_string()))?;

        match &config.anilist.access_token {
            Some(encrypted_token) => {
                if config.security.encryption_key.is_empty() {
                    return Err(ConfigError::Encryption(EncryptionError::InvalidKey(
                        "Encryption key is empty".to_string(),
                    )));
                }

                let decrypted =
                    encryption::decrypt(encrypted_token, &config.security.encryption_key)?;
                Ok(Some(decrypted))
            }
            None => Ok(None),
        }
    }

    /// Clear AniList access token
    pub fn clear_anilist_token(&self) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.anilist.access_token = None;
        drop(config);
        self.save()
    }

    /// Update UI configuration
    pub fn update_ui_config(&self, ui_config: UiConfig) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui = ui_config;
        drop(config);
        self.save()
    }

    /// Get UI configuration
    pub fn get_ui_config(&self) -> Result<UiConfig, ConfigError> {
        let config = self
            .config
            .read()
            .map_err(|_| ConfigError::Deserialization("Failed to acquire read lock".to_string()))?;
        Ok(config.ui.clone())
    }

    /// Update a specific UI setting
    pub fn update_ui_theme(&self, theme: String) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.theme = theme;
        drop(config);
        self.save()
    }

    /// Update glow effects setting
    pub fn update_glow_effects(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.glow_effects = enabled;
        drop(config);
        self.save()
    }

    /// Update blur effects setting
    pub fn update_blur_effects(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.blur_effects = enabled;
        drop(config);
        self.save()
    }

    /// Update animations setting
    pub fn update_animations(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.animations = enabled;
        drop(config);
        self.save()
    }

    /// Update smooth scroll setting
    pub fn update_smooth_scroll(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.smooth_scroll = enabled;
        drop(config);
        self.save()
    }

    /// Update hover card previews setting
    pub fn update_hover_card(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.hover_card = enabled;
        drop(config);
        self.save()
    }

    /// Update theme mode (light/dark/system)
    pub fn update_theme_mode(&self, mode: String) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        config.ui.theme_mode = mode;
        drop(config);
        self.save()
    }

    /// Update UI scale factor
    pub fn update_ui_scale(&self, scale: f32) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;

        // Clamp scale to reasonable bounds (50% to 200%)
        let clamped_scale = scale.max(0.5).min(2.0);
        config.ui.ui_scale = clamped_scale;
        drop(config);
        self.save()
    }

    /// Update activity feed tab and filter preferences
    pub fn update_activity_prefs(
        &self,
        tab: String,
        filter: String,
    ) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;
        config.ui.activity_feed_tab = tab;
        config.ui.activity_feed_filter = filter;
        drop(config);
        self.save()
    }

    /// Get player configuration
    pub fn get_player_config(&self) -> Result<PlayerConfig, ConfigError> {
        let config = self
            .config
            .read()
            .map_err(|_| ConfigError::Deserialization("Failed to acquire read lock".to_string()))?;
        Ok(config.player.clone())
    }

    /// Update external player executable path
    pub fn update_external_player_path(&self, path: Option<String>) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;
        config.player.external_player_path = path;
        drop(config);
        self.save()
    }

    /// Update auto-select next stream setting
    pub fn update_auto_select_next_stream(&self, enabled: bool) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;
        config.player.auto_select_next_stream = enabled;
        drop(config);
        self.save()
    }

    /// Update shader configuration
    pub fn update_shader_config(&self, enabled: bool, selected_shaders: Vec<String>) -> Result<(), ConfigError> {
        let mut config = self.config.write().map_err(|_| {
            ConfigError::Deserialization("Failed to acquire write lock".to_string())
        })?;
        config.player.shaders.enabled = enabled;
        config.player.shaders.selected_shaders = selected_shaders;
        drop(config);
        self.save()
    }

    /// Get the config file path for debugging
    pub fn get_config_file_path(&self) -> PathBuf {
        self.config_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loader_creation() {
        // Tests use config.test.ron, so they don't interfere with dev/prod configs
        let loader = ConfigLoader::new().unwrap();
        let config = loader.get_config().unwrap();

        // Config may have been modified by other tests, so we just check structure
        assert!(config.ui.theme == "catppuccin" || !config.ui.theme.is_empty());
    }

    #[test]
    fn test_config_path_separation() {
        // Verify test config uses separate file
        let path = ConfigLoader::get_config_path().unwrap();
        assert!(path.to_string_lossy().contains("config.test.ron"));
    }

    #[test]
    fn test_set_and_get_token() {
        let loader = ConfigLoader::new().unwrap();
        let test_token = "test_anilist_token_12345";

        // Set token
        loader.set_anilist_token(test_token).unwrap();

        // Verify encryption key was generated
        let config = loader.get_config().unwrap();
        assert!(!config.security.encryption_key.is_empty());
        assert!(config.anilist.access_token.is_some());

        // Get and verify token
        let retrieved_token = loader.get_anilist_token().unwrap();
        assert_eq!(retrieved_token, Some(test_token.to_string()));
    }

    #[test]
    fn test_clear_token() {
        let loader = ConfigLoader::new().unwrap();

        loader.set_anilist_token("test_token").unwrap();
        assert!(loader.get_anilist_token().unwrap().is_some());

        loader.clear_anilist_token().unwrap();
        assert!(loader.get_anilist_token().unwrap().is_none());
    }

    #[test]
    fn test_ui_config_updates() {
        let loader = ConfigLoader::new().unwrap();

        loader.update_ui_theme("dark".to_string()).unwrap();
        loader.update_glow_effects(false).unwrap();
        loader.update_blur_effects(false).unwrap();
        loader.update_animations(false).unwrap();
        loader.update_smooth_scroll(false).unwrap();

        let ui_config = loader.get_ui_config().unwrap();
        assert_eq!(ui_config.theme, "dark");
        assert!(!ui_config.glow_effects);
        assert!(!ui_config.blur_effects);
        assert!(!ui_config.animations);
        assert!(!ui_config.smooth_scroll);
    }
}
