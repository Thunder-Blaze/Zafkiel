import { themeManager, type Theme } from '$lib/services/theme';
import { SvelteSet } from 'svelte/reactivity';

interface ThemeState {
	currentTheme: string;
	isDark: boolean;
	availableThemes: Theme[];
	loadedThemes: SvelteSet<string>;
	isLoading: boolean;
	initialized: boolean;
}

function createThemeStore() {
	const state = $state<ThemeState>({
		currentTheme: 'default',
		isDark: false,
		availableThemes: [],
		loadedThemes: new SvelteSet<string>(),
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
		get loadedThemes() {
			return state.loadedThemes;
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
				
				// Mark initially loaded theme
				state.loadedThemes.add(state.currentTheme);

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
