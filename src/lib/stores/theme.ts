import { themeManager, type ThemeMetadata } from '$lib/services/theme';

/**
 * Theme store using Svelte 5 runes
 * Manages theme state and provides reactive access to theme data
 */
class ThemeStore {
	private _currentTheme = $state<string>('default');
	private _isDark = $state<boolean>(false);
	private _availableThemes = $state<ThemeMetadata[]>([]);
	private _isLoading = $state<boolean>(false);
	private _initialized = $state<boolean>(false);

	/**
	 * Current active theme ID
	 */
	get currentTheme() {
		return this._currentTheme;
	}

	/**
	 * Current dark mode state
	 */
	get isDark() {
		return this._isDark;
	}

	/**
	 * List of available themes
	 */
	get availableThemes() {
		return this._availableThemes;
	}

	/**
	 * Loading state for theme operations
	 */
	get isLoading() {
		return this._isLoading;
	}

	/**
	 * Whether theme system has been initialized
	 */
	get initialized() {
		return this._initialized;
	}

	/**
	 * Initialize theme system
	 * Should be called once on app startup
	 */
	async initialize() {
		if (this._initialized) return;

		this._isLoading = true;

		try {
			// Detect system dark mode preference
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			this._isDark = prefersDark;

			// Initialize theme manager (this now just sets attributes, no async loading)
			await themeManager.initialize(prefersDark);
			this._currentTheme = themeManager.getCurrentTheme();

			// Load available themes (non-blocking)
			themeManager.listThemes().then((themes) => {
				this._availableThemes = themes;
			}).catch((error) => {
				console.warn('Failed to load theme list:', error);
				this._availableThemes = [];
			});

			this._initialized = true;

			// Listen for system theme changes
			window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
				this.setDarkMode(e.matches);
			});
		} catch (error) {
			console.error('Failed to initialize theme store:', error);
			// Set defaults even if initialization fails
			this._currentTheme = 'default';
			this._isDark = false;
			this._availableThemes = [];
			this._initialized = true;
		} finally {
			this._isLoading = false;
		}
	}

	/**
	 * Switch to a different theme
	 */
	async switchTheme(themeId: string) {
		if (this._currentTheme === themeId) return;

		this._isLoading = true;

		try {
			await themeManager.switchTheme(themeId, this._isDark);
			this._currentTheme = themeId;
		} catch (error) {
			console.error(`Failed to switch to theme ${themeId}:`, error);
			throw error;
		} finally {
			this._isLoading = false;
		}
	}

	/**
	 * Toggle dark mode
	 */
	async toggleDarkMode() {
		await this.setDarkMode(!this._isDark);
	}

	/**
	 * Set dark mode state
	 */
	async setDarkMode(isDark: boolean) {
		if (this._isDark === isDark) return;

		this._isDark = isDark;

		// Re-apply current theme with new dark mode state
		try {
			await themeManager.switchTheme(this._currentTheme, isDark);
		} catch (error) {
			console.error('Failed to update dark mode:', error);
		}
	}

	/**
	 * Preview a theme without switching
	 * Returns theme metadata for preview UI
	 */
	async previewTheme(themeId: string): Promise<ThemeMetadata> {
		return themeManager.previewTheme(themeId);
	}

	/**
	 * Get theme by ID from available themes
	 */
	getTheme(themeId: string): ThemeMetadata | undefined {
		return this._availableThemes.find((t) => t.id === themeId);
	}

	/**
	 * Check if a theme is currently loaded
	 */
	isThemeLoaded(themeId: string): boolean {
		return themeManager.isThemeLoaded(themeId);
	}
}

// Export singleton instance
export const themeStore = new ThemeStore();
