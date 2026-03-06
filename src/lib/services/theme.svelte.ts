import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { SvelteMap } from 'svelte/reactivity';
import { useConfigState } from '../stores/config.svelte';

const config = useConfigState();

export interface Theme {
	id: string;
	name: string;
	path: string;
	cssPath: string;
	themeImagePath: string;
	linkElement?: HTMLLinkElement;
}

interface TauriResponse<T> {
	success: boolean;
	data: T;
	error?: string;
}

/**
 * Simple theme structure - just ID and name from directory
 */
export type Themes = SvelteMap<string, Theme>;

export type ThemeMode = 'light' | 'dark' | 'system';

/**
 * Theme manager class for dynamic theme loading and switching
 */
export const formatThemeName = (id: string): string => {
	return id
		.split('-')
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(' ');
};

const ensureTrailingSlash = (url: string): string => (url.endsWith('/') ? url : `${url}/`);

const resolveThemeAssetUrl = (baseUrl: string, fileName: string): string => {
	return new URL(fileName, ensureTrailingSlash(baseUrl)).toString();
};

export const listThemes = async (): Promise<Themes> => {
	console.log('[ThemeManager] Listing available themes...');
	try {
		const themes = (await invoke<TauriResponse<Record<string, string>>>('get_themes_with_paths'))
			.data;
		const themeEntries = Object.entries(themes);
		console.log(`[ThemeManager] ✓ Found ${themeEntries.length} themes:`, themes);

		const formattedThemes: Themes = new SvelteMap(
			themeEntries.map(([id, fileSystemPath]) => {
				const basePath = convertFileSrc(fileSystemPath);
				const cssPath = resolveThemeAssetUrl(basePath, 'index.css');
				const themeImagePath = resolveThemeAssetUrl(basePath, 'theme.png');

				return [id, { id, name: formatThemeName(id), path: basePath, cssPath, themeImagePath }];
			})
		);

		console.log('[ThemeManager] ✓ Formatted theme list:', formattedThemes);
		return formattedThemes;
	} catch (error) {
		console.error('[ThemeManager] ✗ Failed to list themes:', error);
		return new SvelteMap();
	}
};

export const loadTheme = async (theme: Theme, loadedThemes: Themes): Promise<void> => {
	// Skip if already loaded in this session — no re-validation needed
	const existing = loadedThemes.get(theme.id);
	if (existing?.linkElement) {
		return;
	}

	console.log(`[ThemeManager] Loading theme "${theme.id}"...`);

	return new Promise<void>((resolve, reject) => {
		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = theme.cssPath;
		link.dataset.themeId = theme.id;

		link.onload = () => {
			console.log(`[ThemeManager] ✓ Theme "${theme.id}" CSS loaded`);
			loadedThemes.set(theme.id, { ...theme, linkElement: link });
			resolve();
		};
		link.onerror = () => {
			document.head.removeChild(link);
			const reason =
				`CSS file not found or failed to load at: ${theme.cssPath}` +
				` — the theme directory may be missing, moved, or corrupt.`;
			console.error(`[ThemeManager] ✗ "${theme.id}": ${reason}`);
			reject(new Error(reason));
		};
		document.head.appendChild(link);
	});
};

/** Apply the theme id + dark-mode class to <html> without touching the store layer. */
const applyThemeToDom = (id: string, mode: ThemeMode): void => {
	const root = document.documentElement;
	root.setAttribute('data-theme', id);
	const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
	root.classList.toggle('dark', mode === 'dark' || (mode === 'system' && prefersDark));
};

export const setTheme = async (theme: Theme, mode: ThemeMode, themes: Themes): Promise<void> => {
	// If already loaded, apply immediately — no CSS fetch needed
	const existing = themes.get(theme.id);
	if (existing?.linkElement) {
		applyThemeToDom(theme.id, mode);
		config.setTheme(theme.id);
		config.setThemeMode(mode);
		console.log(`[ThemeManager] ✓ Switched to ${theme.id} (cached)`);
		return;
	}

	try {
		console.log(`[ThemeManager] Loading + switching to theme: ${theme.id}`);
		await loadTheme(theme, themes);
		applyThemeToDom(theme.id, mode);
		config.setTheme(theme.id);
		config.setThemeMode(mode);
		console.log(`[ThemeManager] ✓ Switched to ${theme.id}`);
	} catch (error) {
		const msg = error instanceof Error ? error.message : String(error);
		console.error(`[ThemeManager] ✗ Failed to load "${theme.id}": ${msg}`);

		if (theme.id !== 'default') {
			console.warn(`[ThemeManager] Falling back to default theme`);
			const fallback = themes.get('default');
			if (fallback) {
				await setTheme(fallback, mode, themes);
			}
		}
		// Don't re-throw — the fallback handles the UX
	}
};

export const initTheme = async (themes: Themes): Promise<void> => {
	const themeId = config.theme;
	const mode = config.themeMode;

	const theme = themes.get(themeId);
	if (!theme) {
		console.warn(`[ThemeManager] Configured theme "${themeId}" not found in index`);
		const fallback = themes.get('default');
		if (fallback) {
			await setTheme(fallback, mode, themes);
		}
		return;
	}

	await setTheme(theme, mode, themes);
};

export default {
	formatThemeName,
	listThemes,
	loadTheme,
	setTheme,
	initTheme,
};
