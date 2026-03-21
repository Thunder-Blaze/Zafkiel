import { invoke } from '@tauri-apps/api/core';

// Re-export shared types
export type { AppConfig, AniListConfig, UiConfig } from '$lib/types/config';
import type { AppConfig, UiConfig } from '$lib/types/config';

/**
 * Response wrapper for config operations
 */
export interface ConfigResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
}

/**
 * Config service for managing application configuration
 */
export class ConfigService {
	/**
	 * Get the full application config
	 */
	static async getConfig(): Promise<AppConfig> {
		const response = await invoke<ConfigResponse<AppConfig>>('get_config');

		console.log('[ConfigService] getConfig response:', response);

		if (!response.success || !response.data) {
			throw new Error(response.error || 'Failed to get config');
		}
		return response.data;
	}

	/**
	 * Clear the AniList access token
	 */
	static async clearAniListToken(): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('clear_anilist_token');
		if (!response.success) {
			throw new Error(response.error || 'Failed to clear AniList token');
		}
	}

	/**
	 * Get UI configuration
	 */
	static async getUiConfig(): Promise<UiConfig> {
		const response = await invoke<ConfigResponse<UiConfig>>('get_ui_config');
		if (!response.success || !response.data) {
			throw new Error(response.error || 'Failed to get UI config');
		}
		return response.data;
	}

	/**
	 * Update the entire UI configuration
	 */
	static async updateUiConfig(uiConfig: UiConfig): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_ui_config', { uiConfig });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update UI config');
		}
	}

	/**
	 * Update theme
	 */
	static async updateTheme(theme: string): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_theme', { theme });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update theme');
		}
	}

	/**
	 * Update glow effects setting
	 */
	static async updateGlowEffects(enabled: boolean): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_glow_effects', { enabled });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update glow effects');
		}
	}

	/**
	 * Update blur effects setting
	 */
	static async updateBlurEffects(enabled: boolean): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_blur_effects', { enabled });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update blur effects');
		}
	}

	/**
	 * Update animations setting
	 */
	static async updateAnimations(enabled: boolean): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_animations', { enabled });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update animations');
		}
	}

	/**
	 * Update smooth scroll setting
	 */
	static async updateSmoothScroll(enabled: boolean): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_smooth_scroll', { enabled });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update smooth scroll');
		}
	}

	/**
	 * Update hover card previews setting
	 */
	static async updateHoverCard(enabled: boolean): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_hover_card', { enabled });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update hover card');
		}
	}

	/**
	 * Update theme mode (light/dark/system)
	 */
	static async updateThemeMode(mode: 'light' | 'dark' | 'system'): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_theme_mode', { mode });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update theme mode');
		}
	}

	/**
	 * Update UI scale factor (0.5 to 2.0) and apply webview zoom
	 */
	static async updateUiScale(scale: number): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_ui_scale', { scale });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update UI scale');
		}
	}

	/**
	 * Apply UI scale from config (call on app startup)
	 */
	static async applyUiScale(): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('apply_ui_scale');
		if (!response.success) {
			throw new Error(response.error || 'Failed to apply UI scale');
		}
	}

	/**
	 * Open developer tools (only works in Tauri desktop app)
	 */
	static async openDevtools(): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('open_devtools');
		if (!response.success) {
			throw new Error(response.error || 'Failed to open devtools');
		}
	}

	/**
	 * Get config file path (for debugging)
	 */
	static async getConfigPath(): Promise<string> {
		const response = await invoke<ConfigResponse<string>>('get_config_path');
		if (!response.success || !response.data) {
			throw new Error(response.error || 'Failed to get config path');
		}
		return response.data;
	}

	/**
	 * Update shader configuration
	 */
	static async updateShaderConfig(enabled: boolean, selectedShaders: string[]): Promise<void> {
		const response = await invoke<ConfigResponse<void>>('update_shader_config', { enabled, selectedShaders });
		if (!response.success) {
			throw new Error(response.error || 'Failed to update shader config');
		}
	}

	/**
	 * Get available shaders from bundled resources
	 */
	static async getAvailableShaders(): Promise<string[]> {
		const response = await invoke<ConfigResponse<string[]>>('get_available_shaders');
		if (!response.success || !response.data) {
			throw new Error(response.error || 'Failed to get available shaders');
		}
		return response.data;
	}
}
