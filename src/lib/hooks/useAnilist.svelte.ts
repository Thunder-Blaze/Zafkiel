/**
 * TanStack Query hooks for AniList API
 * Provides caching and optimized data fetching
 */

import { createQuery } from '@tanstack/svelte-query';
import { anilistApi } from '$lib/services/anilist';
import type { PaginationParams, SearchParams, SeasonalAnimeParams } from '$lib/types/anilist';

// ============================================================================
// Query Key Factories
// ============================================================================

export const anilistKeys = {
	all: ['anilist'] as const,

	// Anime keys
	anime: {
		all: ['anilist', 'anime'] as const,
		search: (params: SearchParams) => ['anilist', 'anime', 'search', params] as const,
		byId: (id: number) => ['anilist', 'anime', 'byId', id] as const,
		trending: (params?: PaginationParams) => ['anilist', 'anime', 'trending', params] as const,
		popular: (params?: PaginationParams) => ['anilist', 'anime', 'popular', params] as const,
		seasonal: (params: SeasonalAnimeParams) =>
			['anilist', 'anime', 'seasonal', params] as const,
	},

	// Manga keys
	manga: {
		all: ['anilist', 'manga'] as const,
		search: (params: SearchParams) => ['anilist', 'manga', 'search', params] as const,
		byId: (id: number) => ['anilist', 'manga', 'byId', id] as const,
		trending: (params?: PaginationParams) => ['anilist', 'manga', 'trending', params] as const,
		popular: (params?: PaginationParams) => ['anilist', 'manga', 'popular', params] as const,
	},

	// User keys
	user: {
		all: ['anilist', 'user'] as const,
		current: () => ['anilist', 'user', 'current'] as const,
		byId: (id: number) => ['anilist', 'user', 'byId', id] as const,
		byName: (name: string) => ['anilist', 'user', 'byName', name] as const,
		search: (params: SearchParams) => ['anilist', 'user', 'search', params] as const,
	},
};

// ============================================================================
// Anime Queries
// ============================================================================

/**
 * Search for anime
 */
export function useSearchAnime(
	params: SearchParams | (() => SearchParams),
	enabled: boolean | (() => boolean) = true
) {
	return createQuery(() => {
		const p = typeof params === 'function' ? params() : params;
		const e = typeof enabled === 'function' ? enabled() : enabled;
		
		return {
			queryKey: anilistKeys.anime.search(p),
			queryFn: async () => {
				console.log('[useAnilist] Searching anime with params:', p);
				const response = await anilistApi.anime.search(p);
				console.log('[useAnilist] Search anime response:', response);
				if (!response.success || !response.data) {
					console.error('[useAnilist] Search anime failed:', response.error);
					throw new Error(response.error ?? 'Failed to search anime');
				}
				console.log('[useAnilist] Search anime success, got', response.data.length, 'items');
				return response.data;
			},
			staleTime: 1000 * 60 * 10, // 10 minutes
			enabled: e,
		};
	});
}

/**
 * Get anime by ID
 */
export function useAnimeById(id: number, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.byId(id),
		queryFn: async () => {
			const response = await anilistApi.anime.getById(id);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch anime');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 30, // 30 minutes - anime details change less frequently
		enabled,
	}));
}

/**
 * Get trending anime
 */
export function useTrendingAnime(params?: PaginationParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.trending(params),
		queryFn: async () => {
			console.log('[useAnilist] Fetching trending anime with params:', params);
			const response = await anilistApi.anime.getTrending(params);
			console.log('[useAnilist] Trending anime response:', response);
			if (!response.success || !response.data) {
				console.error('[useAnilist] Trending anime failed:', response.error);
				throw new Error(response.error ?? 'Failed to fetch trending anime');
			}
			console.log('[useAnilist] Trending anime success, got', response.data.length, 'items');
			return response.data;
		},
		staleTime: 1000 * 60 * 5, // 5 minutes - trending updates frequently
		enabled,
	}));
}

/**
 * Get popular anime
 */
export function usePopularAnime(params?: PaginationParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.popular(params),
		queryFn: async () => {
			const response = await anilistApi.anime.getPopular(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch popular anime');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 15, // 15 minutes
		enabled,
	}));
}

/**
 * Get seasonal anime
 */
export function useSeasonalAnime(params: SeasonalAnimeParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.seasonal(params),
		queryFn: async () => {
			const response = await anilistApi.anime.getSeasonal(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch seasonal anime');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 60, // 60 minutes - seasonal data is stable
		enabled,
	}));
}

// ============================================================================
// Manga Queries
// ============================================================================

/**
 * Search for manga
 */
export function useSearchManga(params: SearchParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.search(params),
		queryFn: async () => {
			const response = await anilistApi.manga.search(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to search manga');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 10, // 10 minutes
		enabled,
	}));
}

/**
 * Get manga by ID
 */
export function useMangaById(id: number, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.byId(id),
		queryFn: async () => {
			const response = await anilistApi.manga.getById(id);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch manga');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 30, // 30 minutes
		enabled,
	}));
}

/**
 * Get trending manga
 */
export function useTrendingManga(params?: PaginationParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.trending(params),
		queryFn: async () => {
			const response = await anilistApi.manga.getTrending(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch trending manga');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 5, // 5 minutes
		enabled,
	}));
}

/**
 * Get popular manga
 */
export function usePopularManga(params?: PaginationParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.popular(params),
		queryFn: async () => {
			const response = await anilistApi.manga.getPopular(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch popular manga');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 15, // 15 minutes
		enabled,
	}));
}

// ============================================================================
// User Queries
// ============================================================================

/**
 * Get current authenticated user
 */
export function useCurrentUser(enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.current(),
		queryFn: async () => {
			const response = await anilistApi.user.getCurrent();
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch current user');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 60, // 60 minutes - user profile changes infrequently
		retry: false, // Don't retry if not authenticated
		enabled,
	}));
}

/**
 * Get user by ID
 */
export function useUserById(id: number, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.byId(id),
		queryFn: async () => {
			const response = await anilistApi.user.getById(id);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch user');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 30, // 30 minutes
		enabled,
	}));
}

/**
 * Get user by name
 */
export function useUserByName(name: string, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.byName(name),
		queryFn: async () => {
			const response = await anilistApi.user.getByName(name);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to fetch user');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 30, // 30 minutes
		enabled,
	}));
}

/**
 * Search users
 */
export function useSearchUsers(params: SearchParams, enabled = true) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.search(params),
		queryFn: async () => {
			const response = await anilistApi.user.search(params);
			if (!response.success || !response.data) {
				throw new Error(response.error ?? 'Failed to search users');
			}
			return response.data;
		},
		staleTime: 1000 * 60 * 10, // 10 minutes
		enabled,
	}));
}
