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
 * Security and encryption configuration
 */
export interface SecurityConfig {
	/** Base64-encoded encryption key */
	encryption_key: string;
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
 * Application configuration structure
 */
export interface AppConfig {
	anilist: AniListConfig;
	security: SecurityConfig;
	ui: UiConfig;
}
