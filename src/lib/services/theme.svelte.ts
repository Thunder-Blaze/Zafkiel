import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import { SvelteMap } from 'svelte/reactivity';
import { useConfigState } from '../stores/config.svelte';

const config = useConfigState();

export interface Theme {
	id: string;
	name: string;
	path: string;
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
}

export const listThemes = async (): Promise<Themes> => {
	console.log('[ThemeManager] Listing available themes...');
	try {
		const themes = (await invoke<TauriResponse<SvelteMap<string, string>>>('get_themes_with_paths')).data;
		console.log(`[ThemeManager] ✓ Found ${themes.size} themes:`, themes);

		const formattedThemes: Themes = new SvelteMap(Array.from(Object.entries(themes).map(([id, path]) => {
			return [id, { id, name: formatThemeName(id), path: convertFileSrc(path) }];
		})));

		console.log('[ThemeManager] ✓ Formatted theme list:', formattedThemes);
		return formattedThemes;
	} catch (error) {
		console.error('[ThemeManager] ✗ Failed to list themes:', error);
		return new SvelteMap();
	}
}

export const loadTheme = async (theme: Theme, loadedThemes: &Themes): Promise<void> => {
	if (loadedThemes.get(theme.id)?.linkElement) {
		console.log(`[ThemeManager] Theme "${theme.id}" already loaded`);
		return;
	}

	console.log(`[ThemeManager] Loading theme "${theme.id}"...`);

	try {
		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = `${theme.path}/index.css`;
		link.dataset.themeId = theme.id;

		await new Promise<void>((resolve, reject) => {
			link.onload = () => {
				console.log(`[ThemeManager] ✓ Theme "${theme.id}" CSS loaded`);
				resolve();
			};
			link.onerror = () => {
				console.error(`[ThemeManager] ✗ Failed to load theme CSS: ${theme.id}`);
				reject(new Error(`Failed to load theme CSS: ${theme.id}`));
			};
			document.head.appendChild(link);
		});

		loadedThemes.set(theme.id, { ...theme, linkElement: link });
	} catch (error) {
		console.error(`[ThemeManager] Failed to load theme ${theme.id}:`, error);
		throw error;
	}
}

export const setTheme = async (theme: Theme, mode: ThemeMode, themes: &Themes): Promise<void> => {
	try {
		console.log(`[ThemeManager] Switching to theme: ${theme.id}`);

		if (!theme.linkElement) {
			console.log(`[ThemeManager] Theme "${theme.id}" not loaded yet, loading now...`);
			await loadTheme(theme, themes);
		}

		const root = document.documentElement;
		root.setAttribute('data-theme', theme.id);
		config.setTheme(theme.id);

		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		root.classList.toggle('dark', mode === 'dark' || (mode === 'system' && prefersDark));
		config.setThemeMode(mode);

		console.log(`[ThemeManager] ✓ Switched to ${theme.id}`);
	} catch (error) {
		console.error(`[ThemeManager] ✗ Failed to switch theme:`, error);
		throw error;
	}
}

export const initTheme = async (themes: &Themes): Promise<void> => {
	try {
		const themeId = config.theme;
		const mode = config.themeMode;

		const theme = themes.get(themeId);
		if (!theme) {
			console.warn(`[ThemeManager] Theme "${themeId}" not found, skipping init`);
			return;
		}

		console.log(`[ThemeManager] Initializing theme: ${theme.id}`);

		if (!theme.linkElement) {
			await loadTheme(theme, themes);
		}

		const root = document.documentElement;
		root.setAttribute('data-theme', theme.id);
		config.setTheme(theme.id);

		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		root.classList.toggle('dark', mode === 'dark' || (mode === 'system' && prefersDark));
		config.setThemeMode(mode);

		console.log(`[ThemeManager] ✓ Init finished with ${theme.id}`);
	} catch (error) {
		console.error(`[ThemeManager] ✗ Failed to init theme:`, error);
		throw error;
	}
}

export default {
	formatThemeName,
	listThemes,
	loadTheme,
	setTheme,
	initTheme,
};
