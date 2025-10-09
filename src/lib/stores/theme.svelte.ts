import { themeManager, type Theme } from '$lib/services/theme';
import { ConfigService } from '$lib/services/config';
import { SvelteSet } from 'svelte/reactivity';

interface ThemeState {
	currentTheme: string;
	isDark: boolean;
	themeMode: 'light' | 'dark' | 'system';
	availableThemes: Theme[];
	loadedThemes: SvelteSet<string>;
	isLoading: boolean;
	initialized: boolean;
}

function createThemeStore() {
	const state = $state<ThemeState>({
		currentTheme: 'default',
		isDark: false,
		themeMode: 'dark',
		availableThemes: [],
		loadedThemes: new SvelteSet<string>(),
		isLoading: false,
		initialized: false,
	});

	return {
		get currentTheme() {
			return state.currentTheme;
		},
		get currentThemePath() {
			return `/themes/${state.currentTheme}`;
		},
		get isDark() {
			return state.isDark;
		},
		get themeMode() {
			return state.themeMode;
		},
		get availableThemes() {
			return state.availableThemes;
		},
		get loadedThemes() {
			return state.loadedThemes;
		},
		get isLoading() {
			return state.isLoading;
		},
		get initialized() {
			return state.initialized;
		},

		/**
		 * Get the absolute path to a specific theme's directory
		 * @param themeId - The theme ID (optional, defaults to current theme)
		 * @returns The path to the theme folder (e.g., "/themes/catppuccin")
		 */
		getThemePath(themeId?: string): string {
			return `/themes/${themeId || state.currentTheme}`;
		},

		async initialize() {
			if (state.initialized) return;

			console.log('[ThemeStore] Initializing...');
			state.isLoading = true;

			try {
				// Load theme_mode from config first
				const config = await ConfigService.getUiConfig();
				state.themeMode = config.theme_mode;

				// Determine isDark based on theme_mode
				const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				if (state.themeMode === 'system') {
					state.isDark = prefersDark;
				} else {
					state.isDark = state.themeMode === 'dark';
				}

				// Apply theme mode to document immediately
				this.applyThemeMode(state.themeMode);

				await themeManager.initialize(state.isDark);
				state.currentTheme = themeManager.getCurrentTheme();

				const themes = await themeManager.listThemes();
				state.availableThemes = themes;

				// Mark initially loaded theme
				state.loadedThemes.add(state.currentTheme);

				state.initialized = true;
				console.log('[ThemeStore] ✓ Initialized with theme_mode:', state.themeMode);

				// Listen for system preference changes only if theme_mode is 'system'
				window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
					if (state.themeMode === 'system') {
						state.isDark = e.matches;
						this.applyThemeMode('system');
					}
				});
			} catch (error) {
				console.error('[ThemeStore] ✗ Init failed:', error);
				state.currentTheme = 'default';
				state.isDark = false;
				state.themeMode = 'dark';
				state.availableThemes = [];
				state.initialized = true;
			} finally {
				state.isLoading = false;
			}
		},

		async switchTheme(themeId: string) {
			if (state.currentTheme === themeId) return;

			console.log(`[ThemeStore] Switching to: ${themeId}`);
			state.isLoading = true;

			try {
				await themeManager.switchTheme(themeId, state.isDark);
				state.currentTheme = themeId;
				state.loadedThemes.add(themeId);
				console.log('[ThemeStore] ✓ Switched to:', themeId);
			} catch (error) {
				console.error('[ThemeStore] ✗ Switch failed:', error);
				throw error;
			} finally {
				state.isLoading = false;
			}
		},

		async loadTheme(themeId: string) {
			if (state.loadedThemes.has(themeId)) {
				console.log(`[ThemeStore] Theme ${themeId} already loaded`);
				return;
			}

			console.log(`[ThemeStore] Loading theme: ${themeId}`);
			state.isLoading = true;

			try {
				await themeManager.loadTheme(themeId);
				state.loadedThemes.add(themeId);
				console.log('[ThemeStore] ✓ Loaded:', themeId);
			} catch (error) {
				console.error('[ThemeStore] ✗ Load failed:', error);
				throw error;
			} finally {
				state.isLoading = false;
			}
		},

		/**
		 * Apply theme mode to document
		 * This is the single source of truth for light/dark mode classes
		 */
		applyThemeMode(mode: 'light' | 'dark' | 'system') {
			const root = document.documentElement;

			if (mode === 'system') {
				const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				root.classList.remove('light', 'dark');
				root.classList.add(prefersDark ? 'dark' : 'light');
				state.isDark = prefersDark;
			} else {
				root.classList.remove('light', 'dark');
				root.classList.add(mode);
				state.isDark = mode === 'dark';
			}
		},

		/**
		 * Update theme mode and persist to config
		 */
		async setThemeMode(mode: 'light' | 'dark' | 'system') {
			if (state.themeMode === mode) return;

			try {
				await ConfigService.updateThemeMode(mode);
				state.themeMode = mode;
				this.applyThemeMode(mode);
				console.log('[ThemeStore] ✓ Theme mode updated to:', mode);
			} catch (error) {
				console.error('[ThemeStore] ✗ Failed to update theme mode:', error);
				throw error;
			}
		},

		setDarkMode(isDark: boolean) {
			if (state.isDark === isDark) return;

			state.isDark = isDark;
			const root = document.documentElement;

			if (isDark) {
				root.classList.add('dark');
			} else {
				root.classList.remove('dark');
			}
		},

		toggleDarkMode() {
			this.setDarkMode(!state.isDark);
		},
	};
}

export const themeStore = createThemeStore();
