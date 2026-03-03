import ThemeService, { type Themes, type Theme, type ThemeMode } from '$lib/services/theme.svelte';
import { useConfigState } from './config.svelte';
import { browser } from '$app/environment';
import { THEME_CACHE_KEY } from '$lib/constants';
import { SvelteMap } from 'svelte/reactivity';

const config = useConfigState();

interface ThemeState {
	themes: Themes;
}

const initialThemeState = browser ? JSON.parse(sessionStorage.getItem(THEME_CACHE_KEY) || 'null') : null;
let themeState = $state<ThemeState | null>(initialThemeState);

let cleanup: (() => void) | null = null;

// Only create effects in browser environment
if (browser) {
	cleanup = $effect.root(() => {
		if (themeState) {
				try {
					sessionStorage.setItem(THEME_CACHE_KEY, JSON.stringify(themeState));
				} catch (error) {
					console.error('[ThemeState] ✗ Failed to save theme state to session storage, reverting:', error);
					themeState = JSON.parse(sessionStorage.getItem(THEME_CACHE_KEY) || 'null');
				}
			}

		return () => {
			console.log('[ThemeState] Effect root cleanup');
		};
	});
}

export const useThemeState = () => {
	return {
		init: async () => {
			if (!themeState) {
				themeState = {
					themes: await ThemeService.listThemes(),
				};
				await ThemeService.initTheme(themeState.themes);
			}
		},

		// Getter && Setter Function
		get: () => themeState,

		get themes(): SvelteMap<string, Theme> {
			return themeState ? themeState.themes : new SvelteMap();
		},

		get currentThemeId(): string | null {
			return config ? config.theme : null;
		},

		get currentThemePath(): string | null {
			if (!themeState || !config) return null;
			const theme = themeState.themes.get(config.theme);
			return theme ? theme.path : null;
		},

		get currentThemeImagePath(): string | null {
			if (!themeState || !config) return null;
			const theme = themeState.themes.get(config.theme);
			return theme ? theme.themeImagePath : null;
		},

		get currentThemeMode(): ThemeMode | null {
			return config ? config.themeMode : null;
		},

		get currentTheme(): Theme | null {
			if (!themeState || !config) return null;
			return themeState.themes.get(config.theme) || null;
		},

		setAvailableThemes: (themes: Theme[]) => {
			if (themeState) {
				themeState.themes = new SvelteMap(themes.map(theme => [theme.id, theme]));
			}
		},

		setThemeAndMode: ({ theme, mode }: { theme: Theme; mode: ThemeMode }) => {
			if (themeState) {
				ThemeService.setTheme(theme, mode, themeState.themes);
			}
		},

		setTheme: (theme: Theme) => {
			if (themeState && config.themeMode) {
				ThemeService.setTheme(theme, config.themeMode, themeState.themes);
			}
			ThemeService.setTheme(theme, config.themeMode, themeState!.themes);
		},

		setThemeMode: (mode: ThemeMode) => {
			if (themeState && config.theme) {
				ThemeService.setTheme(themeState.themes.get(config.theme)!, mode, themeState.themes);
			}
		},

		// Cleanup function to destroy the effect root when no longer needed
    destroy: () => {
      if (cleanup) {
        cleanup();
        cleanup = null;
      }
    }
	};
}
