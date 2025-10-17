import { writable, derived } from 'svelte/store';
import { ConfigService } from '$lib/services/config';
import type { AppConfig, UiConfig } from '$lib/types/config';
import { loadConfigCache, saveConfigCache } from './sessionCache';

/**
 * Reactive store for application configuration
 */
function createConfigStore() {
	const { subscribe, set, update } = writable<AppConfig | null>(null);

	// Guard to prevent multiple initializations
	let isInitialized = false;

	return {
		subscribe,

		/**
		 * Initialize the config store by loading from cache or backend
		 */
		async init() {
			// Prevent multiple initializations
			if (isInitialized) {
				console.log('[Config] Already initialized, skipping');
				return;
			}

			try {
				isInitialized = true;

				// Check cache first
				const cached = loadConfigCache();
				if (cached) {
					console.log('[Config] ✓ Loaded from cache');
					set(cached);
					return;
				}

				// Load from backend if no cache
				console.log('[Config] Loading from backend...');
				const config = await ConfigService.getConfig();
				set(config);
				saveConfigCache(config);
				console.log('[Config] ✓ Loaded and cached');
			} catch (error) {
				console.error('[Config] Failed to initialize:', error);
			}
		},

		/**
		 * Reload config from backend (bypasses cache)
		 */
		async reload() {
			try {
				console.log('[Config] Reloading from backend...');
				const config = await ConfigService.getConfig();
				set(config);
				saveConfigCache(config);
				console.log('[Config] ✓ Reloaded and cached');
			} catch (error) {
				console.error('[Config] Failed to reload:', error);
			}
		},

		/**
		 * Set AniList token and update store
		 */
		async setAniListToken(token: string) {
			await ConfigService.setAniListToken(token);
			await this.reload();
		},

		/**
		 * Clear AniList token and update store
		 */
		async clearAniListToken() {
			await ConfigService.clearAniListToken();
			await this.reload();
		},

		/**
		 * Update UI config and update store
		 */
		async updateUiConfig(uiConfig: UiConfig) {
			await ConfigService.updateUiConfig(uiConfig);
			await this.reload();
		},

		/**
		 * Update theme
		 */
		async updateTheme(theme: string) {
			await ConfigService.updateTheme(theme);
			update((config) => {
				if (config) {
					config.ui.theme = theme;
					saveConfigCache(config);
				}
				return config;
			});
		},

		/**
		 * Update glow effects
		 */
		async updateGlowEffects(enabled: boolean) {
			await ConfigService.updateGlowEffects(enabled);
			update((config) => {
				if (config) {
					config.ui.glow_effects = enabled;
					saveConfigCache(config);
				}
				return config;
			});
		},

		/**
		 * Update animations
		 */
		async updateAnimations(enabled: boolean) {
			await ConfigService.updateAnimations(enabled);
			update((config) => {
				if (config) {
					config.ui.animations = enabled;
					saveConfigCache(config);
				}
				return config;
			});
		},

		/**
		 * Update smooth scroll
		 */
		async updateSmoothScroll(enabled: boolean) {
			await ConfigService.updateSmoothScroll(enabled);
			update((config) => {
				if (config) {
					config.ui.smooth_scroll = enabled;
					saveConfigCache(config);
				}
				return config;
			});
		},
	};
}

/**
 * Global config store
 */
export const configStore = createConfigStore();

/**
 * Derived store for UI config only
 */
export const uiConfig = derived(
	configStore,
	($config) =>
		$config?.ui ?? {
			theme: 'catppuccin',
			glow_effects: true,
			animations: true,
			smooth_scroll: true,
		}
);

/**
 * Derived store for checking if user is authenticated
 */
export const isAuthenticated = derived(
	configStore,
	($config) => $config?.anilist.access_token !== null
);

/**
 * Derived store for theme
 */
export const theme = derived(uiConfig, ($uiConfig) => $uiConfig.theme);

/**
 * Derived store for glow effects setting
 */
export const glowEffects = derived(uiConfig, ($uiConfig) => $uiConfig.glow_effects);

/**
 * Derived store for animations setting
 */
export const animations = derived(uiConfig, ($uiConfig) => $uiConfig.animations);

/**
 * Derived store for smooth scroll setting
 */
export const smoothScroll = derived(uiConfig, ($uiConfig) => $uiConfig.smooth_scroll);
