import { themeManager, type ThemeMetadata } from '$lib/services/theme';

/**
 * Theme store state
 */
interface ThemeState {
	currentTheme: string;
	isDark: boolean;
	availableThemes: ThemeMetadata[];
	isLoading: boolean;
	initialized: boolean;
}

/**
 * Create theme store with reactive state using Svelte 5 runes
 */
function createThemeStore() {
	const state = $state<ThemeState>({
		currentTheme: 'default',
		isDark: false,
		availableThemes: [],
		isLoading: false,
		initialized: false,
	});

	return {
		// Reactive getters
		get currentTheme() {
			return state.currentTheme;
		},
		get isDark() {
			return state.isDark;
		},
		get availableThemes() {
			return state.availableThemes;
		},
		get isLoading() {
			return state.isLoading;
		},
		get initialized() {
			return state.initialized;
		},

		/**
		 * Initialize theme system
		 * Should be called once on app startup
		 */
		async initialize() {
			if (state.initialized) return;

			console.log('[ThemeStore] initialize() called');
			state.isLoading = true;

			try {
				// Detect system dark mode preference
				const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				console.log('[ThemeStore] System prefers dark mode:', prefersDark);
				state.isDark = prefersDark;

				// Initialize theme manager (this now just sets attributes, no async loading)
				console.log('[ThemeStore] Initializing theme manager...');
				await themeManager.initialize(prefersDark);
				state.currentTheme = themeManager.getCurrentTheme();
				console.log('[ThemeStore] Current theme:', state.currentTheme);

				// Load available themes (MUST await to populate dropdown)
				console.log('[ThemeStore] Loading available themes...');
				try {
					const themes = await themeManager.listThemes();
					console.log('[ThemeStore] ✓ Loaded themes:', themes.length, 'themes');
					state.availableThemes = themes;
				} catch (error) {
					console.error('[ThemeStore] ✗ Failed to load theme list:', error);
					state.availableThemes = [];
				}

				state.initialized = true;
				console.log('[ThemeStore] ✓ Initialization complete');

				// Listen for system theme changes
				window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
					this.setDarkMode(e.matches);
				});
			} catch (error) {
				console.error('Failed to initialize theme store:', error);
				// Set defaults even if initialization fails
				state.currentTheme = 'default';
				state.isDark = false;
				state.availableThemes = [];
				state.initialized = true;
			} finally {
				state.isLoading = false;
			}
		},

		/**
		 * Switch to a different theme
		 */
		async switchTheme(themeId: string) {
			if (state.currentTheme === themeId) return;

			state.isLoading = true;

			try {
				await themeManager.switchTheme(themeId, state.isDark);
				state.currentTheme = themeId;
			} catch (error) {
				console.error(`Failed to switch to theme ${themeId}:`, error);
				throw error;
			} finally {
				state.isLoading = false;
			}
		},

		/**
		 * Toggle dark mode
		 */
		async toggleDarkMode() {
			await this.setDarkMode(!state.isDark);
		},

		/**
		 * Set dark mode state
		 */
		async setDarkMode(isDark: boolean) {
			if (state.isDark === isDark) return;

			state.isDark = isDark;

			// Re-apply current theme with new dark mode state
			try {
				await themeManager.switchTheme(state.currentTheme, isDark);
			} catch (error) {
				console.error('Failed to update dark mode:', error);
			}
		},

		/**
		 * Preview a theme without switching
		 * Returns theme metadata for preview UI
		 */
		async previewTheme(themeId: string): Promise<ThemeMetadata> {
			return themeManager.previewTheme(themeId);
		},

		/**
		 * Get theme by ID from available themes
		 */
		getTheme(themeId: string): ThemeMetadata | undefined {
			return state.availableThemes.find((t) => t.id === themeId);
		},

		/**
		 * Check if a theme is currently loaded
		 */
		isThemeLoaded(themeId: string): boolean {
			return themeManager.isThemeLoaded(themeId);
		},
	};
}

// Export singleton instance
export const themeStore = createThemeStore();
