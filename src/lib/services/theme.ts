import { invoke } from '@tauri-apps/api/core';

/**
 * Theme metadata structure from theme.json
 */
export interface ThemeMetadata {
	id: string;
	name: string;
	description: string;
	author: string;
	version: string;
	colors: {
		light: {
			primary: string;
			background: string;
			accent: string;
		};
		dark: {
			primary: string;
			background: string;
			accent: string;
		};
	};
}

/**
 * Loaded theme tracking
 */
interface LoadedTheme {
	metadata: ThemeMetadata;
	linkElement: HTMLLinkElement;
}

/**
 * Theme manager class for dynamic theme loading and switching
 * Implements on-demand loading for fast theme switching
 */
class ThemeManager {
	private loadedThemes = new Map<string, LoadedTheme>();
	private currentThemeId: string = 'default';

	/**
	 * List all available themes from the static/themes directory
	 */
	async listThemes(): Promise<ThemeMetadata[]> {
		console.log('[ThemeManager] listThemes() called');
		try {
			console.log('[ThemeManager] Invoking Tauri command: list_themes');
			const themes = await invoke<ThemeMetadata[]>('list_themes');
			console.log('[ThemeManager] ✓ Received themes from backend:', themes.length, 'themes');
			console.log('[ThemeManager] Themes:', themes.map((t) => t.id).join(', '));
			return themes;
		} catch (error) {
			console.error('[ThemeManager] ✗ Failed to list themes:', error);
			return [];
		}
	}

	/**
	 * Load a theme's CSS file dynamically
	 * @param themeId - The theme identifier
	 * @returns Promise that resolves when the theme CSS is loaded
	 */
	async loadTheme(themeId: string): Promise<ThemeMetadata> {
		// Return if already loaded
		if (this.loadedThemes.has(themeId)) {
			return this.loadedThemes.get(themeId)!.metadata;
		}

		try {
			// Fetch theme metadata
			const metadata = await invoke<ThemeMetadata>('get_theme_metadata', { themeId });

			// Create link element for CSS
			const link = document.createElement('link');
			link.rel = 'stylesheet';
			link.href = `/themes/${themeId}/index.css`;
			link.dataset.themeId = themeId;

			// Wait for CSS to load
			await new Promise<void>((resolve, reject) => {
				link.onload = () => resolve();
				link.onerror = () => reject(new Error(`Failed to load theme CSS: ${themeId}`));
				document.head.appendChild(link);
			});

			// Track loaded theme
			this.loadedThemes.set(themeId, { metadata, linkElement: link });

			return metadata;
		} catch (error) {
			console.error(`Failed to load theme ${themeId}:`, error);
			throw error;
		}
	}

	/**
	 * Unload a theme's CSS file to free memory
	 * @param themeId - The theme identifier
	 */
	unloadTheme(themeId: string): void {
		const theme = this.loadedThemes.get(themeId);
		if (!theme) return;

		// Remove CSS link element
		theme.linkElement.remove();
		this.loadedThemes.delete(themeId);
	}

	/**
	 * Switch to a different theme
	 * Loads the theme if not already loaded, then switches
	 * @param themeId - The theme identifier
	 * @param isDark - Whether to use dark mode
	 */
	async switchTheme(themeId: string, isDark: boolean = false): Promise<void> {
		try {
			console.log(`[ThemeManager] Switching to theme: ${themeId}, dark: ${isDark}`);

			// Store old theme ID
			const oldThemeId = this.currentThemeId;

			// Load new theme if not already loaded
			if (!this.loadedThemes.has(themeId)) {
				console.log(`[ThemeManager] Loading new theme: ${themeId}`);
				await this.loadTheme(themeId);
			} else {
				console.log(`[ThemeManager] Theme ${themeId} already loaded`);
			}

			// Update data-theme attribute on root and dark class
			const root = document.documentElement;
			root.setAttribute('data-theme', themeId);

			// Update dark mode class
			if (isDark) {
				root.classList.add('dark');
			} else {
				root.classList.remove('dark');
			}

			this.currentThemeId = themeId;

			// Unload old theme to prevent CSS conflicts (but keep new theme)
			if (oldThemeId !== themeId && this.loadedThemes.has(oldThemeId)) {
				console.log(`[ThemeManager] Unloading old theme: ${oldThemeId}`);
				this.unloadTheme(oldThemeId);
			}

			// Save to config
			await this.saveThemePreference(themeId);

			console.log(`[ThemeManager] ✓ Successfully switched to ${themeId}`);
		} catch (error) {
			console.error(`[ThemeManager] ✗ Failed to switch theme to ${themeId}:`, error);
			throw error;
		}
	}

	/**
	 * Preview a theme by loading it without switching
	 * Useful for theme selector previews
	 * @param themeId - The theme identifier
	 * @returns Theme metadata with preview colors
	 */
	async previewTheme(themeId: string): Promise<ThemeMetadata> {
		try {
			// Get metadata without loading CSS
			const metadata = await invoke<ThemeMetadata>('get_theme_metadata', { themeId });
			return metadata;
		} catch (error) {
			console.error(`Failed to preview theme ${themeId}:`, error);
			throw error;
		}
	}

	/**
	 * Get currently active theme ID
	 */
	getCurrentTheme(): string {
		return this.currentThemeId;
	}

	/**
	 * Get list of loaded theme IDs
	 */
	getLoadedThemes(): string[] {
		return Array.from(this.loadedThemes.keys());
	}

	/**
	 * Check if a theme is currently loaded
	 */
	isThemeLoaded(themeId: string): boolean {
		return this.loadedThemes.has(themeId);
	}

	/**
	 * Save theme preference to config
	 */
	private async saveThemePreference(themeId: string): Promise<void> {
		try {
			// Save to localStorage for fast synchronous access on page load
			localStorage.setItem('theme-preference', themeId);

			// Also save to Tauri config for persistence
			await invoke('save_theme_preference', { themeId });
		} catch (error) {
			console.error('Failed to save theme preference:', error);
		}
	}

	/**
	 * Load theme preference from config
	 */
	async loadThemePreference(): Promise<string> {
		try {
			const themeId = await invoke<string>('get_theme_preference');
			return themeId || 'default';
		} catch (error) {
			console.error('Failed to load theme preference:', error);
			return 'default';
		}
	}

	/**
	 * Initialize theme system on app startup
	 * Loads saved theme preference
	 */
	async initialize(isDark: boolean = false): Promise<void> {
		try {
			// Check localStorage first (fast, synchronous)
			let themeToLoad = localStorage.getItem('theme-preference');

			// Fallback to Tauri config if not in localStorage
			if (!themeToLoad) {
				themeToLoad = await this.loadThemePreference();
				// Save to localStorage for next time
				localStorage.setItem('theme-preference', themeToLoad);
			}

			// Set current theme without loading CSS (CSS is already loaded by app.html script)
			this.currentThemeId = themeToLoad;

			// Apply dark mode class if needed
			const root = document.documentElement;
			if (isDark) {
				root.classList.add('dark');
			} else {
				root.classList.remove('dark');
			}

			// Set data-theme attribute (should already be set by app.html, but ensure it's correct)
			root.setAttribute('data-theme', themeToLoad);

			console.log(`Theme system initialized with theme: ${themeToLoad}, dark mode: ${isDark}`);
		} catch (error) {
			console.error('Failed to initialize theme system:', error);
			// Fallback to default theme
			this.currentThemeId = 'default';
			document.documentElement.setAttribute('data-theme', 'default');
		}
	}

	/**
	 * Unload all themes except the current one
	 * Useful for memory management
	 */
	unloadInactiveThemes(): void {
		for (const [themeId, theme] of this.loadedThemes.entries()) {
			if (themeId !== this.currentThemeId) {
				theme.linkElement.remove();
				this.loadedThemes.delete(themeId);
			}
		}
	}
}

// Export singleton instance
export const themeManager = new ThemeManager();
