import { ConfigService } from '$lib/services/config';
import type { AppConfig, UiConfig } from '$lib/types/config';
import { browser } from '$app/environment';
import { CONFIG_CACHE_KEY } from '$lib/constants';

const initialConfigState = browser
	? JSON.parse(sessionStorage.getItem(CONFIG_CACHE_KEY) || 'null')
	: null;
let configState = $state<AppConfig | null>(initialConfigState);

let cleanup: (() => void) | null = null;

// Only create effects in browser environment
if (browser) {
	cleanup = $effect.root(() => {
		// The actual effect goes INSIDE $effect.root
		$effect(() => {
			// Guard against null state
			if (!configState) return;

			try {
				console.log('[Config] Saving UI config to backend...');
				ConfigService.updateUiConfig(configState.ui);
				sessionStorage.setItem(CONFIG_CACHE_KEY, JSON.stringify(configState));
			} catch (error) {
				console.error('[Config] ✗ Failed to save UI config to backend:', error);
				// Revert to cached state on error
				const cachedState = sessionStorage.getItem(CONFIG_CACHE_KEY);
				if (cachedState) {
					configState = JSON.parse(cachedState);
				}
			}
		});

		// Return cleanup function for the root
		return () => {
			console.log('[Config] Effect root cleanup');
		};
	});
}

const init = async () => {
	if (!browser) return;

	console.log('[Config] Initializing...');

	if (configState != null) {
		console.log('[Config] Skipping init, already loaded');
		return;
	}

	try {
		configState = await ConfigService.getConfig();
	} catch (error) {
		console.error('[Config] ✗ Failed to load config from backend:', error);
	}
};

const reload = async () => {
	if (!browser) return;

	try {
		configState = await ConfigService.getConfig();
	} catch (error) {
		console.error('[Config] ✗ Failed to reload config from backend:', error);
	}
};

export const useConfigState = () => {
	return {
		// Init Function
		init: async () => await init(),

		// Reload Function
		reload: async () => await reload(),

		// Getter && Setter Function
		get: () => configState,
		get config(): AppConfig | null {
			return configState;
		},
		set: (newConfig: AppConfig) => (configState = newConfig),

		get uiConfig() {
			return (
				configState?.ui ||
				({
					theme: 'default',
					theme_mode: 'dark',
					glow_effects: true,
					animations: true,
					smooth_scroll: true,
					blur_effects: true,
					hover_card: true,
					ui_scale: 1.0,
				} as UiConfig)
			);
		},
		setUiConfig: (uiConfig: UiConfig) => {
			if (configState) configState.ui = uiConfig;
		},

		get theme() {
			return configState?.ui.theme || 'default';
		},
		setTheme: (themeId: string) => {
			if (configState) configState.ui.theme = themeId;
		},

		get themeMode() {
			return configState?.ui.theme_mode || 'dark';
		},
		setThemeMode: (mode: 'light' | 'dark' | 'system') => {
			if (configState) configState.ui.theme_mode = mode;
		},

		get glowEffects() {
			return configState?.ui.glow_effects || false;
		},
		setGlowEffects: (enabled: boolean) => {
			if (configState) configState.ui.glow_effects = enabled;
		},

		get blurEffects() {
			return configState?.ui.blur_effects || false;
		},
		setBlurEffects: (enabled: boolean) => {
			if (configState) configState.ui.blur_effects = enabled;
		},

		get animations() {
			return configState?.ui.animations || false;
		},
		setAnimations: (enabled: boolean) => {
			if (configState) configState.ui.animations = enabled;
		},

		get smoothScroll() {
			return configState?.ui.smooth_scroll || false;
		},
		setSmoothScroll: (enabled: boolean) => {
			if (configState) configState.ui.smooth_scroll = enabled;
		},

		get hoverCard() {
			return configState?.ui.hover_card || false;
		},
		setHoverCard: (enabled: boolean) => {
			if (configState) configState.ui.hover_card = enabled;
		},

		get uiScale() {
			return configState?.ui.ui_scale || 1.0;
		},
		setUiScale: (scale: number) => {
			if (configState) configState.ui.ui_scale = scale;
		},

		clearAnilistToken: async () => {
			if (!browser) return;

			try {
				await ConfigService.clearAniListToken();
				console.log('[Config] ✓ AniList token cleared');
			} catch (error) {
				console.error('[Config] ✗ Failed to clear AniList token:', error);
				throw error;
			}
		},

		// Cleanup function to destroy the effect root when no longer needed
		destroy: () => {
			if (cleanup) {
				cleanup();
				cleanup = null;
			}
		},
	};
};
