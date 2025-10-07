use serde::{Deserialize, Serialize};

/// Main application configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub anilist: AniListConfig,
    pub security: SecurityConfig,
    pub ui: UiConfig,
}

/// AniList API configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AniListConfig {
    /// Encrypted access token for AniList API
    /// None if not authenticated yet
    pub access_token: Option<String>,
}

/// Security and encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityConfig {
    /// Base64-encoded encryption key for sensitive data
    /// Empty string by default, auto-generated when first needed
    pub encryption_key: String,
}

/// UI/UX preferences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UiConfig {
    /// Theme name (e.g., "catppuccin", "dark", "light")
    pub theme: String,

    /// Enable glow effects on UI elements
    pub glow_effects: bool,

    /// Enable animations
    pub animations: bool,

    /// Enable smooth scrolling
    pub smooth_scroll: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            anilist: AniListConfig::default(),
            security: SecurityConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for AniListConfig {
    fn default() -> Self {
        Self {
            access_token: None,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption_key: String::new(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "catppuccin".to_string(),
            glow_effects: true,
            animations: true,
            smooth_scroll: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.anilist.access_token, None);
        assert_eq!(config.security.encryption_key, "");
        assert_eq!(config.ui.theme, "catppuccin");
        assert!(config.ui.glow_effects);
        assert!(config.ui.animations);
        assert!(config.ui.smooth_scroll);
    }

    #[test]
    fn test_ron_serialization() {
        let config = AppConfig::default();
        let serialized = ron::to_string(&config).unwrap();
        let deserialized: AppConfig = ron::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }
}
