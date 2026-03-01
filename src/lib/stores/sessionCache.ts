/**
 * Session Cache Store
 *
 * Unified session storage system for caching multiple data types
 * Reduces unnecessary refetches and improves performance
 */

import type { User } from '$lib/types/anilist';
import type { AppConfig } from '$lib/types/config';

// ============================================================================
// Types
// ============================================================================

interface CachedData<T> {
	data: T;
	timestamp: number;
}

export interface ListStats {
	watching: number;
	completed: number;
	planning: number;
}

interface AuthCacheData {
	isAuthenticated: boolean;
	user: User | null;
	listStats?: ListStats | null;
}

interface SessionCacheState {
	auth: CachedData<AuthCacheData> | null;
	config: CachedData<AppConfig> | null;
	themes: CachedData<ThemeConfig> | null;
}

// Re-export AppConfig for convenience
export type { AppConfig };

// Theme configuration type (extend as needed)
export interface ThemeConfig {
	mode: 'light' | 'dark' | 'system';
	currentTheme: string;
	availableThemes: Array<{ id: string; name: string }>;
	isDark: boolean;
}

// ============================================================================
// Constants
// ============================================================================

const STORAGE_KEY = 'zafkiel_session_cache';

// Cache durations (in milliseconds)
export const CACHE_DURATIONS = {
	auth: 5 * 60 * 1000, // 5 minutes
	config: 15 * 60 * 1000, // 15 minutes
	themes: 15 * 60 * 1000, // 15 minutes
} as const;

// ============================================================================
// Browser Check
// ============================================================================

const isBrowser = typeof window !== 'undefined' && typeof sessionStorage !== 'undefined';

// ============================================================================
// Cache Management Functions
// ============================================================================

/**
 * Load all cached data from sessionStorage
 */
function loadCache(): SessionCacheState | null {
	if (!isBrowser) return null;

	try {
		const cached = sessionStorage.getItem(STORAGE_KEY);
		if (!cached) return null;

		return JSON.parse(cached);
	} catch (error) {
		console.error('[SessionCache] Error loading cache:', error);
		return null;
	}
}

/**
 * Save cache to sessionStorage
 */
function saveCache(cache: SessionCacheState): void {
	if (!isBrowser) return;

	try {
		sessionStorage.setItem(STORAGE_KEY, JSON.stringify(cache));
	} catch (error) {
		console.error('[SessionCache] Error saving cache:', error);
	}
}

/**
 * Clear all cached data
 */
export function clearAllCache(): void {
	if (!isBrowser) return;

	try {
		sessionStorage.removeItem(STORAGE_KEY);
		console.log('[SessionCache] All cache cleared');
	} catch (error) {
		console.error('[SessionCache] Error clearing cache:', error);
	}
}

/**
 * Check if cached data is still valid
 */
function isValidCache<T>(cached: CachedData<T> | null, duration: number): boolean {
	if (!cached) return false;
	const age = Date.now() - cached.timestamp;
	return age < duration;
}

// ============================================================================
// Auth Cache Functions
// ============================================================================

export function loadAuthCache(): AuthCacheData | null {
	const cache = loadCache();
	if (!cache?.auth) return null;

	if (isValidCache(cache.auth, CACHE_DURATIONS.auth)) {
		console.log('[SessionCache] Auth cache hit');
		return cache.auth.data;
	}

	console.log('[SessionCache] Auth cache expired');
	return null;
}

export function saveAuthCache(isAuthenticated: boolean, user: User | null, listStats?: ListStats | null): void {
	const cache = loadCache() || { auth: null, config: null, themes: null };
	cache.auth = {
		data: { isAuthenticated, user, listStats: listStats ?? null },
		timestamp: Date.now(),
	};
	saveCache(cache);
	console.log('[SessionCache] Auth cached');
}

export function clearAuthCache(): void {
	const cache = loadCache();
	if (!cache) return;

	cache.auth = null;
	saveCache(cache);
	console.log('[SessionCache] Auth cache cleared');
}

// ============================================================================
// Config Cache Functions
// ============================================================================

export function loadConfigCache(): AppConfig | null {
	const cache = loadCache();
	if (!cache?.config) return null;

	if (isValidCache(cache.config, CACHE_DURATIONS.config)) {
		console.log('[SessionCache] Config cache hit');
		return cache.config.data;
	}

	console.log('[SessionCache] Config cache expired');
	return null;
}

export function saveConfigCache(config: AppConfig): void {
	const cache = loadCache() || { auth: null, config: null, themes: null };
	cache.config = {
		data: config,
		timestamp: Date.now(),
	};
	saveCache(cache);
	console.log('[SessionCache] Config cached');
}

export function clearConfigCache(): void {
	const cache = loadCache();
	if (!cache) return;

	cache.config = null;
	saveCache(cache);
	console.log('[SessionCache] Config cache cleared');
}

// ============================================================================
// Theme Cache Functions
// ============================================================================

export function loadThemeCache(): ThemeConfig | null {
	const cache = loadCache();
	if (!cache?.themes) return null;

	if (isValidCache(cache.themes, CACHE_DURATIONS.themes)) {
		console.log('[SessionCache] Theme cache hit');
		return cache.themes.data;
	}

	console.log('[SessionCache] Theme cache expired');
	return null;
}

export function saveThemeCache(themes: ThemeConfig): void {
	const cache = loadCache() || { auth: null, config: null, themes: null };
	cache.themes = {
		data: themes,
		timestamp: Date.now(),
	};
	saveCache(cache);
	console.log('[SessionCache] Theme cached');
}

export function clearThemeCache(): void {
	const cache = loadCache();
	if (!cache) return;

	cache.themes = null;
	saveCache(cache);
	console.log('[SessionCache] Theme cache cleared');
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Get cache statistics
 */
export function getCacheStats() {
	const cache = loadCache();
	if (!cache) {
		return { auth: false, config: false, themes: false };
	}

	return {
		auth: cache.auth !== null && isValidCache(cache.auth, CACHE_DURATIONS.auth),
		config: cache.config !== null && isValidCache(cache.config, CACHE_DURATIONS.config),
		themes: cache.themes !== null && isValidCache(cache.themes, CACHE_DURATIONS.themes),
	};
}

/**
 * Invalidate specific cache type
 */
export function invalidateCache(type: keyof SessionCacheState): void {
	const cache = loadCache();
	if (!cache) return;

	cache[type] = null;
	saveCache(cache);
	console.log(`[SessionCache] ${type} cache invalidated`);
}
