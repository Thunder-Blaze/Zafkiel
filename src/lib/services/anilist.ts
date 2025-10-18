/**
 * AniList API Service
 * Wraps Tauri commands for AniList operations
 */

import { invoke } from '@tauri-apps/api/core';
import type {
	AniListResponse,
	Media,
	User,
	PaginationParams,
	SeasonalAnimeParams,
	SearchParams,
} from '$lib/types/anilist';

// ============================================================================
// Anime API
// ============================================================================

export const animeApi = {
	/**
	 * Search for anime
	 */
	search: async (params: SearchParams): Promise<AniListResponse<Media[]>> => {
		console.log('[AniList API] Calling search_anime:', params);
		const response = (await invoke('search_anime', {
			query: params.query,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		})) as AniListResponse<Media[]>;
		console.log('[AniList API] search_anime response:', response);
		return response;
	},

	/**
	 * Get anime by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Media>> => {
		return invoke('get_anime_by_id', { id });
	},

	/**
	 * Get trending anime
	 */
	getTrending: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		try {
			console.log('[AniList API] Calling get_trending_anime:', params);
			const response = await invoke('get_trending_anime', {
				page: params?.page ?? null,
				perPage: params?.perPage ?? null,
			}) as { success: boolean; data?: { data: Media[] }; error?: string };
			console.log('[AniList API] get_trending_anime response:', response);

			if (!response.success) {
				return {
					success: false,
					error: response.error || 'Backend returned error',
					data: []
				};
			}

			return {
				success: true,
				data: response.data?.data || []
			};
		} catch (error) {
			console.error('[AniList API] Error in getTrending:', error);
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Unknown error',
				data: []
			};
		}
	},

	/**
	 * Get popular anime
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		return invoke('get_popular_anime', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Get seasonal anime
	 */
	getSeasonal: async (params: SeasonalAnimeParams): Promise<AniListResponse<Media[]>> => {
		return invoke('get_seasonal_anime', { params });
	},
};

// ============================================================================
// Manga API
// ============================================================================

export const mangaApi = {
	/**
	 * Search for manga
	 */
	search: async (params: SearchParams): Promise<AniListResponse<Media[]>> => {
		return invoke('search_manga', {
			query: params.query,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		});
	},

	/**
	 * Get manga by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Media>> => {
		return invoke('get_manga_by_id', { id });
	},

	/**
	 * Get trending manga
	 */
	getTrending: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		return invoke('get_trending_manga', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Get popular manga
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		return invoke('get_popular_manga', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},
};

// ============================================================================
// User API
// ============================================================================

export const userApi = {
	/**
	 * Get current authenticated user
	 */
	getCurrent: async (): Promise<AniListResponse<User>> => {
		return invoke('get_current_user');
	},

	/**
	 * Get user by ID
	 */
	getById: async (id: number): Promise<AniListResponse<User>> => {
		return invoke('get_user_by_id', { id });
	},

	/**
	 * Get user by name
	 */
	getByName: async (name: string): Promise<AniListResponse<User>> => {
		return invoke('get_user_by_name', { name });
	},

	/**
	 * Search users
	 */
	search: async (params: SearchParams): Promise<AniListResponse<User[]>> => {
		return invoke('search_users', {
			query: params.query,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		});
	},
};

// ============================================================================
// Combined API Export
// ============================================================================

export const anilistApi = {
	anime: animeApi,
	manga: mangaApi,
	user: userApi,
};
