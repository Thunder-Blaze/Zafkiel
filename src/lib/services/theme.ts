import { invoke } from '@tauri-apps/api/core';

/**
 * Simple theme structure - just ID and name from directory
 */
export interface Theme {
	id: string;
	name: string;
}

/**
 * Loaded theme tracking
 */
interface LoadedTheme {
	id: string;
	linkElement: HTMLLinkElement;
}

/**
 * Theme manager class for dynamic theme loading and switching
 */
class ThemeManager {
	private loadedThemes = new Map<string, LoadedTheme>();
	private currentThemeId: string = 'default';

	async listThemes(): Promise<Theme[]> {
		console.log('[ThemeManager] Listing available themes...');
		try {
			const themeIds = await invoke<string[]>('list_themes');
			console.log(`[ThemeManager] ✓ Found ${themeIds.length} themes:`, themeIds);

			const themes: Theme[] = themeIds.map((id) => ({
id,
name: this.formatThemeName(id)
}));

			return themes;
		} catch (error) {
			console.error('[ThemeManager] ✗ Failed to list themes:', error);
			return [];
		}
	}

	private formatThemeName(id: string): string {
		return id
			.split('-')
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(' ');
	}

	async loadTheme(themeId: string): Promise<void> {
		if (this.loadedThemes.has(themeId)) {
			console.log(`[ThemeManager] Theme "${themeId}" already loaded`);
			return;
		}

		console.log(`[ThemeManager] Loading theme "${themeId}"...`);

		try {
			const link = document.createElement('link');
			link.rel = 'stylesheet';
			link.href = `/themes/${themeId}/index.css`;
			link.dataset.themeId = themeId;

			await new Promise<void>((resolve, reject) => {
				link.onload = () => {
					console.log(`[ThemeManager] ✓ Theme "${themeId}" CSS loaded`);
					resolve();
				};
				link.onerror = () => {
					console.error(`[ThemeManager] ✗ Failed to load theme CSS: ${themeId}`);
					reject(new Error(`Failed to load theme CSS: ${themeId}`));
				};
				document.head.appendChild(link);
			});

			this.loadedThemes.set(themeId, { id: themeId, linkElement: link });
		} catch (error) {
			console.error(`[ThemeManager] Failed to load theme ${themeId}:`, error);
			throw error;
		}
	}

	async switchTheme(themeId: string, isDark: boolean = false): Promise<void> {
		try {
			console.log(`[ThemeManager] Switching to theme: ${themeId}`);

			if (!this.loadedThemes.has(themeId)) {
				await this.loadTheme(themeId);
			}

			const root = document.documentElement;
			root.setAttribute('data-theme', themeId);

			if (isDark) {
				root.classList.add('dark');
			} else {
				root.classList.remove('dark');
			}

			this.currentThemeId = themeId;
			await this.saveThemePreference(themeId);

			console.log(`[ThemeManager] ✓ Switched to ${themeId}`);
		} catch (error) {
			console.error(`[ThemeManager] ✗ Failed to switch theme:`, error);
			throw error;
		}
	}

	getCurrentTheme(): string {
		return this.currentThemeId;
	}

	getLoadedThemes(): string[] {
		return Array.from(this.loadedThemes.keys());
	}

	isThemeLoaded(themeId: string): boolean {
		return this.loadedThemes.has(themeId);
	}

	private async saveThemePreference(themeId: string): Promise<void> {
		try {
			await invoke('save_theme_preference', { themeId });
			console.log(`[ThemeManager] ✓ Saved theme preference to config: ${themeId}`);
		} catch (error) {
			console.error('[ThemeManager] ✗ Failed to save theme preference:', error);
		}
	}

	async loadThemePreference(): Promise<string> {
		try {
			const themeId = await invoke<string>('get_theme_preference');
			console.log(`[ThemeManager] ✓ Loaded theme preference from config: ${themeId}`);
			return themeId || 'default';
		} catch (error) {
			console.error('[ThemeManager] ✗ Failed to load theme preference:', error);
			return 'default';
		}
	}

	async initialize(isDark: boolean = false): Promise<void> {
		try {
			console.log('[ThemeManager] Initializing...');

			// Load theme from backend config (single source of truth)
			const themeToLoad = await this.loadThemePreference();

			console.log(`[ThemeManager] Loading theme: ${themeToLoad}`);
			await this.loadTheme(themeToLoad);

			this.currentThemeId = themeToLoad;

			const root = document.documentElement;
			if (isDark) {
				root.classList.add('dark');
			} else {
				root.classList.remove('dark');
			}

			root.setAttribute('data-theme', themeToLoad);

			console.log(`[ThemeManager] ✓ Initialized with ${themeToLoad}`);
		} catch (error) {
			console.error('[ThemeManager] ✗ Initialization failed:', error);
			this.currentThemeId = 'default';
			document.documentElement.setAttribute('data-theme', 'default');
		}
	}
}

export const themeManager = new ThemeManager();
