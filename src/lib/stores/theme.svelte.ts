import { themeManager, type Theme } from '$lib/services/theme';

interface ThemeState {
	currentTheme: string;
	isDark: boolean;
	availableThemes: Theme[];
	isLoading: boolean;
	initialized: boolean;
}

function createThemeStore() {
	const state = $state<ThemeState>({
		currentTheme: 'default',
		isDark: false,
		availableThemes: [],
		isLoading: false,
		initialized: false,
	});

	return {
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

		async initialize() {
			if (state.initialized) return;

			console.log('[ThemeStore] Initializing...');
			state.isLoading = true;

			try {
				const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				state.isDark = prefersDark;

				await themeManager.initialize(prefersDark);
				state.currentTheme = themeManager.getCurrentTheme();

				const themes = await themeManager.listThemes();
				state.availableThemes = themes;

				state.initialized = true;
				console.log('[ThemeStore] ✓ Initialized');

				window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
					this.setDarkMode(e.matches);
				});
			} catch (error) {
				console.error('[ThemeStore] ✗ Init failed:', error);
				state.currentTheme = 'default';
				state.isDark = false;
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
				console.log('[ThemeStore] ✓ Switched to:', themeId);
			} catch (error) {
				console.error('[ThemeStore] ✗ Switch failed:', error);
				throw error;
			} finally {
				state.isLoading = false;
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
