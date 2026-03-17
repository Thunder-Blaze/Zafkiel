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
 * Player/playback configuration
 */
export interface PlayerConfig {
	/** Path to external media player executable */
	external_player_path: string | null;
	/** Whether to automatically select the next best stream when switching episodes */
	auto_select_next_stream: boolean;
}

/**
 * Application configuration structure
 */
export interface AppConfig {
	ui: UiConfig;
	player: PlayerConfig;
}
