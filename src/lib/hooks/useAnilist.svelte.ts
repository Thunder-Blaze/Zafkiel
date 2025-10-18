/**
 * TanStack Query hooks for AniList data with comprehensive caching
 * Updated to use only fetch commands from anilist_moe crate
 */

import { createQuery, createMutation, useQueryClient } from '@tanstack/svelte-query';
import type { CreateQueryOptions } from '@tanstack/svelte-query';
import { animeApi, mangaApi, userApi } from '$lib/services/anilist';
import { ClientDatabaseService } from '$lib/services/client-database';
import type {
	Media,
	User,
	PaginationParams,
	SearchParams,
	SeasonalAnimeParams,
	AniListResponse
} from '$lib/types/anilist';

// ============================================================================
// Query Key Factories
// ============================================================================

export const anilistKeys = {
	// Anime
	anime: {
		all: ['anime'] as const,
		lists: () => [...anilistKeys.anime.all, 'list'] as const,
		list: (params: PaginationParams) => [...anilistKeys.anime.lists(), params] as const,
		searches: () => [...anilistKeys.anime.all, 'search'] as const,
		search: (params: SearchParams) => [...anilistKeys.anime.searches(), params] as const,
		detail: (id: number) => [...anilistKeys.anime.all, 'detail', id] as const,
		trending: (params?: PaginationParams) => [...anilistKeys.anime.all, 'trending', params || {}] as const,
		popular: (params?: PaginationParams) => [...anilistKeys.anime.all, 'popular', params || {}] as const,
		seasonal: (params: SeasonalAnimeParams) => [...anilistKeys.anime.all, 'seasonal', params] as const,
	},
	// Manga
	manga: {
		all: ['manga'] as const,
		lists: () => [...anilistKeys.manga.all, 'list'] as const,
		list: (params: PaginationParams) => [...anilistKeys.manga.lists(), params] as const,
		searches: () => [...anilistKeys.manga.all, 'search'] as const,
		search: (params: SearchParams) => [...anilistKeys.manga.searches(), params] as const,
		detail: (id: number) => [...anilistKeys.manga.all, 'detail', id] as const,
		trending: (params?: PaginationParams) => [...anilistKeys.manga.all, 'trending', params || {}] as const,
		popular: (params?: PaginationParams) => [...anilistKeys.manga.all, 'popular', params || {}] as const,
	},
	// Users
	user: {
		all: ['user'] as const,
		current: () => [...anilistKeys.user.all, 'current'] as const,
		detail: (id: number) => [...anilistKeys.user.all, 'detail', id] as const,
		byName: (name: string) => [...anilistKeys.user.all, 'byName', name] as const,
		searches: () => [...anilistKeys.user.all, 'search'] as const,
		search: (params: SearchParams) => [...anilistKeys.user.searches(), params] as const,
	},
	// Media (generic for both anime and manga)
	media: {
		all: ['media'] as const,
		detail: (id: number) => [...anilistKeys.media.all, 'detail', id] as const,
		search: (params: SearchParams & { type?: 'ANIME' | 'MANGA' }) =>
			[...anilistKeys.media.all, 'search', params] as const,
	},
} as const;

// ============================================================================
// Common Query Options
// ============================================================================

const defaultStaleTime = {
	detail: 30 * 60 * 1000, // 30 minutes - detailed data changes less frequently
	list: 5 * 60 * 1000,   // 5 minutes - lists update more frequently
	trending: 5 * 60 * 1000, // 5 minutes - trending changes frequently
	popular: 15 * 60 * 1000, // 15 minutes - popular changes less frequently
	seasonal: 60 * 60 * 1000, // 1 hour - seasonal data very stable
	search: 10 * 60 * 1000,  // 10 minutes - search results moderate stability
	user: 30 * 60 * 1000,    // 30 minutes - user data changes less frequently
} as const;

// ============================================================================
// Anime Queries
// ============================================================================

/**
 * Search anime with caching
 */
export function useAnimeSearch(
	params: SearchParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.search(params),
		queryFn: () => animeApi.search(params),
		staleTime: defaultStaleTime.search,
		enabled: params.query.length > 0,
		...options,
	}));
}

/**
 * Get anime by ID with caching
 */
export function useAnimeById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Media>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.detail(id),
		queryFn: () => animeApi.getById(id),
		staleTime: defaultStaleTime.detail,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Get trending anime with caching
 */
export function useTrendingAnime(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.trending(params),
		queryFn: () => animeApi.getTrending(params),
		staleTime: defaultStaleTime.trending,
		...options,
	}));
}

/**
 * Get popular anime with caching
 */
export function usePopularAnime(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.popular(params),
		queryFn: () => animeApi.getPopular(params),
		staleTime: defaultStaleTime.popular,
		...options,
	}));
}

/**
 * Get seasonal anime with caching
 */
export function useSeasonalAnime(
	params: SeasonalAnimeParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.anime.seasonal(params),
		queryFn: () => animeApi.getSeasonal(params),
		staleTime: defaultStaleTime.seasonal,
		...options,
	}));
}

// ============================================================================
// Manga Queries
// ============================================================================

/**
 * Search manga with caching
 */
export function useMangaSearch(
	params: SearchParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.search(params),
		queryFn: () => mangaApi.search(params),
		staleTime: defaultStaleTime.search,
		enabled: params.query.length > 0,
		...options,
	}));
}

/**
 * Get manga by ID with caching
 */
export function useMangaById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Media>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.detail(id),
		queryFn: () => mangaApi.getById(id),
		staleTime: defaultStaleTime.detail,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Get trending manga with caching
 */
export function useTrendingManga(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.trending(params),
		queryFn: () => mangaApi.getTrending(params),
		staleTime: defaultStaleTime.trending,
		...options,
	}));
}

/**
 * Get popular manga with caching
 */
export function usePopularManga(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.manga.popular(params),
		queryFn: () => mangaApi.getPopular(params),
		staleTime: defaultStaleTime.popular,
		...options,
	}));
}

// ============================================================================
// User Queries
// ============================================================================

/**
 * Get current authenticated user with caching
 */
export function useCurrentUser(
	options?: Partial<CreateQueryOptions<AniListResponse<User>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.current(),
		queryFn: () => userApi.getCurrent(),
		staleTime: defaultStaleTime.user,
		retry: false, // Don't retry if not authenticated
		...options,
	}));
}

/**
 * Get user by ID with caching
 */
export function useUserById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<User>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.detail(id),
		queryFn: () => userApi.getById(id),
		staleTime: defaultStaleTime.user,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Get user by name with caching
 */
export function useUserByName(
	name: string,
	options?: Partial<CreateQueryOptions<AniListResponse<User>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.byName(name),
		queryFn: () => userApi.getByName(name),
		staleTime: defaultStaleTime.user,
		enabled: name.length > 0,
		...options,
	}));
}

/**
 * Search users with caching
 */
export function useUserSearch(
	params: SearchParams,
	options?: Partial<CreateQueryOptions<AniListResponse<User[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.user.search(params),
		queryFn: () => userApi.search(params),
		staleTime: defaultStaleTime.search,
		enabled: params.query.length > 0,
		...options,
	}));
}

// ============================================================================
// Generic Media Queries
// ============================================================================

/**
 * Get media by ID (works for both anime and manga) with caching
 */
export function useMediaById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Media>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.media.detail(id),
		queryFn: async () => {
			// Try anime first, then manga
			try {
				const animeResult = await animeApi.getById(id);
				if (animeResult.success && animeResult.data) {
					return animeResult;
				}
			} catch {
				// If anime fails, try manga
			}

			return await mangaApi.getById(id);
		},
		staleTime: defaultStaleTime.detail,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Search media (both anime and manga) with caching
 */
export function useMediaSearch(
	params: SearchParams & { type?: 'ANIME' | 'MANGA' },
	options?: Partial<CreateQueryOptions<AniListResponse<Media[]>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.media.search(params),
		queryFn: async () => {
			if (params.type === 'ANIME') {
				return await animeApi.search(params);
			} else if (params.type === 'MANGA') {
				return await mangaApi.search(params);
			} else {
				// Search both anime and manga, combine results
				const [animeResults, mangaResults] = await Promise.allSettled([
					animeApi.search(params),
					mangaApi.search(params),
				]);

				const combinedData: Media[] = [];

				if (animeResults.status === 'fulfilled' && animeResults.value.success) {
					combinedData.push(...(animeResults.value.data || []));
				}

				if (mangaResults.status === 'fulfilled' && mangaResults.value.success) {
					combinedData.push(...(mangaResults.value.data || []));
				}

				return {
					success: true,
					data: combinedData,
				} as AniListResponse<Media[]>;
			}
		},
		staleTime: defaultStaleTime.search,
		enabled: params.query.length > 0,
		...options,
	}));
}

// ============================================================================
// Backward Compatibility Aliases
// ============================================================================

/**
 * Alias for useAnimeSearch for backward compatibility
 */
export const useSearchAnime = useAnimeSearch;

// ============================================================================
// Mutation Types
// ============================================================================

export interface UpdateProgressParams {
	mediaId: number;
	progress: number;
	status?: 'CURRENT' | 'COMPLETED' | 'PAUSED' | 'DROPPED' | 'PLANNING' | 'REPEATING';
	score?: number;
	startedAt?: string;
	completedAt?: string;
}

export interface AddToListParams {
	mediaId: number;
	status: 'CURRENT' | 'COMPLETED' | 'PAUSED' | 'DROPPED' | 'PLANNING' | 'REPEATING';
	progress?: number;
	score?: number;
}

export interface RemoveFromListParams {
	mediaId: number;
}

// ============================================================================
// Mutation Hooks
// ============================================================================

/**
 * Update progress for anime/manga with optimistic updates
 */
export function useUpdateProgress() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (params: UpdateProgressParams) => {
			// Update local database first
			await ClientDatabaseService.updateLocalProgress(params.mediaId, params.progress);

			// Then sync with AniList API (implement when API functions are available)
			// For now, just return success
			return { success: true, data: params };
		},
		onMutate: async (params) => {
			// Cancel outgoing refetches
			await queryClient.cancelQueries({
				queryKey: anilistKeys.media.detail(params.mediaId)
			});

			// Snapshot previous value
			const previousData = queryClient.getQueryData(
				anilistKeys.media.detail(params.mediaId)
			);

			// Optimistically update
			queryClient.setQueryData(
				anilistKeys.media.detail(params.mediaId),
				(old: AniListResponse<Media> | undefined) => {
					if (!old?.data) return old;
					return {
						...old,
						data: {
							...old.data,
							mediaListEntry: {
								...old.data.mediaListEntry,
								progress: params.progress,
								status: params.status || old.data.mediaListEntry?.status,
								score: params.score || old.data.mediaListEntry?.score,
							}
						}
					};
				}
			);

			return { previousData };
		},
		onError: (err, params, context) => {
			// Rollback on error
			if (context?.previousData) {
				queryClient.setQueryData(
					anilistKeys.media.detail(params.mediaId),
					context.previousData
				);
			}
		},
		onSettled: (data, error, params) => {
			// Refetch to ensure consistency
			queryClient.invalidateQueries({
				queryKey: anilistKeys.media.detail(params.mediaId)
			});
			queryClient.invalidateQueries({
				queryKey: anilistKeys.user.current()
			});
		},
	}));
}

/**
 * Add media to user's list
 */
export function useAddToList() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (params: AddToListParams) => {
			// Cache media data first
			const media = queryClient.getQueryData(
				anilistKeys.media.detail(params.mediaId)
			) as AniListResponse<Media> | undefined;

			if (media?.data) {
				await ClientDatabaseService.cacheMedia(media.data);
			}

			// Add to recently viewed
			await ClientDatabaseService.addToRecentlyViewed(params.mediaId);

			// TODO: Implement AniList API call when available
			return { success: true, data: params };
		},
		onSuccess: (data, params) => {
			// Invalidate relevant queries
			queryClient.invalidateQueries({
				queryKey: anilistKeys.media.detail(params.mediaId)
			});
			queryClient.invalidateQueries({
				queryKey: anilistKeys.user.current()
			});
		},
	}));
}

/**
 * Remove media from user's list
 */
export function useRemoveFromList() {
	const queryClient = useQueryClient();

	return createMutation(() => ({
		mutationFn: async (params: RemoveFromListParams) => {
			// TODO: Implement AniList API call when available
			return { success: true, data: params };
		},
		onSuccess: (data, params) => {
			// Invalidate relevant queries
			queryClient.invalidateQueries({
				queryKey: anilistKeys.media.detail(params.mediaId)
			});
			queryClient.invalidateQueries({
				queryKey: anilistKeys.user.current()
			});
		},
	}));
}

/**
 * Cache media data to local database
 */
export function useCacheMedia() {
	return createMutation(() => ({
		mutationFn: async (media: Media) => {
			await ClientDatabaseService.cacheMedia(media);
			return { success: true, data: media };
		},
	}));
}

/**
 * Cache user data to local database
 */
export function useCacheUser() {
	return createMutation(() => ({
		mutationFn: async (user: User) => {
			await ClientDatabaseService.cacheUser(user);
			return { success: true, data: user };
		},
	}));
}
