/**
 * Shared configuration type definitions
 *
 * This file contains type definitions used across stores and services
 * to prevent circular dependencies.
 */

/**
 * AniList API configuration
 */
export interface AniListConfig {
	/** Encrypted access token (null if not authenticated) */
	access_token: string | null;
}

/**
 * UI/UX preferences
 */
export interface UiConfig {
	/** Theme name (e.g., "catppuccin", "dark", "light") */
	theme: string;
	/** Theme mode: "light", "dark", or "system" */
	theme_mode: 'light' | 'dark' | 'system';
	/** Enable glow effects on UI elements */
	glow_effects: boolean;
	/** Enable blur effects on UI elements */
	blur_effects: boolean;
	/** Enable animations */
	animations: boolean;
	/** Enable smooth scrolling */
	smooth_scroll: boolean;
	/** Enable hover card previews on media cards */
	hover_card: boolean;
	/** UI scale factor (0.5 to 2.0, default 1.0) */
	ui_scale: number;
}

/**
 * Dynamic Shader configuration
 */
export interface ShaderConfig {
	enabled: boolean;
	selected_shaders: string[];
}

/**
 * Player/playback configuration
 */
export interface PlayerConfig {
	/** Path to external media player executable */
	external_player_path: string | null;
	/** Whether to automatically select the next best stream when switching episodes */
	auto_select_next_stream: boolean;
	/** Playback speed */
	playback_speed: number;
	/** Whether to automatically update AniList progress */
	auto_update_progress: boolean;
	/** Progress threshold for auto update (0.0 to 1.0) */
	auto_update_threshold: number;
	/** Default update mode for new anime (yes, no, ask) */
	default_update_mode: string;
	/** Dynamic Shader configuration */
	shaders: ShaderConfig;
}

/**
 * Network configuration (proxy, etc.)
 */
export interface NetworkConfig {
	/** Optional proxy URL (e.g., "http://127.0.0.1:8080") */
	proxy_url: string | null;
}

/**
 * Application configuration structure
 */
export interface AppConfig {
	ui: UiConfig;
	player: PlayerConfig;
	network: NetworkConfig;
}
