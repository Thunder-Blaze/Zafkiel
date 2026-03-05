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
    /// Theme name (e.g., "catppuccin", "default", "cyberpunk")
    pub theme: String,

    /// Theme mode: "light", "dark", or "system"
    pub theme_mode: String,

    /// Enable glow effects on UI elements
    pub glow_effects: bool,

    /// Enable blur effects on UI elements
    pub blur_effects: bool,

    /// Enable animations
    pub animations: bool,

    /// Enable smooth scrolling
    pub smooth_scroll: bool,

    /// Enable hover card previews on media cards
    pub hover_card: bool,

    /// UI scale factor (0.5 to 2.0, default 1.0)
    /// Controls the overall size of UI elements via CSS zoom
    pub ui_scale: f32,

    /// Last used activity feed tab: "global" or "following"
    #[serde(default = "default_activity_feed_tab")]
    pub activity_feed_tab: String,

    /// Last used activity feed filter: "all", "list", or "text"
    #[serde(default = "default_activity_feed_filter")]
    pub activity_feed_filter: String,
}

fn default_activity_feed_tab() -> String {
    "global".to_string()
}

fn default_activity_feed_filter() -> String {
    "all".to_string()
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
        Self { access_token: None }
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
            theme: "default".to_string(),
            theme_mode: "dark".to_string(),
            glow_effects: true,
            blur_effects: true,
            animations: true,
            smooth_scroll: true,
            hover_card: true,
            ui_scale: 1.0,
            activity_feed_tab: "global".to_string(),
            activity_feed_filter: "all".to_string(),
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
        assert_eq!(config.ui.theme, "default");
        assert_eq!(config.ui.theme_mode, "dark");
        assert!(config.ui.glow_effects);
        assert!(config.ui.blur_effects);
        assert!(config.ui.animations);
        assert!(config.ui.smooth_scroll);
        assert!(config.ui.hover_card);
    }

    #[test]
    fn test_ron_serialization() {
        let config = AppConfig::default();
        let serialized = ron::to_string(&config).unwrap();
        let deserialized: AppConfig = ron::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }
}
