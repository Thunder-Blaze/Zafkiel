/**
 * TanStack Query hooks for AniList data with comprehensive caching
 * Updated to use only fetch commands from anilist_moe crate
 */

import { createQuery, createMutation, useQueryClient } from '@tanstack/svelte-query';
import type { CreateQueryOptions } from '@tanstack/svelte-query';
import { animeApi, mangaApi, userApi, studioApi, characterApi, staffApi, mediaApi, mediaListApi, activityApi, notificationApi, forumApi, reviewApi, recommendationApi, airingApi } from '$lib/services/anilist';
import { ClientDatabaseService } from '$lib/services/client-database';
import type {
	Media,
	User,
	Studio,
	Character,
	Staff,
	PaginationParams,
	SearchParams,
	SeasonalAnimeParams,
	BrowseParams,
	AniListResponse,
	Page,
	MediaList,
	MediaListStatus,
	ActivityUnion,
	ActivityReply,
	NotificationUnion,
	Thread,
	ThreadComment,
	Review,
	Recommendation,
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
	// Studio
	studio: {
		all: ['studio'] as const,
		detail: (id: number) => [...anilistKeys.studio.all, 'detail', id] as const,
	},
	// Character
	character: {
		all: ['character'] as const,
		detail: (id: number) => [...anilistKeys.character.all, 'detail', id] as const,
		popular: (params?: PaginationParams) => [...anilistKeys.character.all, 'popular', params || {}] as const,
		birthday: (params?: PaginationParams) => [...anilistKeys.character.all, 'birthday', params || {}] as const,
	},
	// Staff
	staff: {
		all: ['staff'] as const,
		detail: (id: number) => [...anilistKeys.staff.all, 'detail', id] as const,
		popular: (params?: PaginationParams) => [...anilistKeys.staff.all, 'popular', params || {}] as const,
		birthday: (params?: PaginationParams) => [...anilistKeys.staff.all, 'birthday', params || {}] as const,
	},
	// Media (generic for both anime and manga)
	media: {
		all: ['media'] as const,
		detail: (id: number) => [...anilistKeys.media.all, 'detail', id] as const,
		search: (params: SearchParams & { type?: 'ANIME' | 'MANGA' }) =>
			[...anilistKeys.media.all, 'search', params] as const,
		browse: (params: BrowseParams) => [...anilistKeys.media.all, 'browse', params] as const,
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
	browse: 10 * 60 * 1000,  // 10 minutes - browse results moderate stability
	user: 30 * 60 * 1000,    // 30 minutes - user data changes less frequently
	studio: 60 * 60 * 1000,  // 1 hour - studio data very stable
	character: 60 * 60 * 1000, // 1 hour - character data very stable
	staff: 60 * 60 * 1000,   // 1 hour - staff data very stable
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
	options?: Partial<CreateQueryOptions<AniListResponse<Page<Media[]>>>>
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
// Studio Queries
// ============================================================================

/**
 * Get studio by ID with caching
 */
export function useStudioById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Studio>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.studio.detail(id),
		queryFn: () => studioApi.getById(id),
		staleTime: defaultStaleTime.studio,
		enabled: id > 0,
		...options,
	}));
}

// ============================================================================
// Character Queries
// ============================================================================

/**
 * Get character by ID with caching
 */
export function useCharacterById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Character>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.character.detail(id),
		queryFn: () => characterApi.getById(id),
		staleTime: defaultStaleTime.character,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Get most favourited characters with caching
 */
export function usePopularCharacters(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Page<Character[]>>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.character.popular(params),
		queryFn: () => characterApi.getPopular(params),
		staleTime: defaultStaleTime.popular,
		...options,
	}));
}

/**
 * Get characters with birthday today with caching
 */
export function useBirthdayCharacters(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Page<Character[]>>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.character.birthday(params),
		queryFn: () => characterApi.getBirthdayToday(params),
		staleTime: defaultStaleTime.trending,
		...options,
	}));
}

/**
 * Search characters by name
 */
export function useSearchCharacters(query: string, params?: PaginationParams) {
	return createQuery(() => ({
		queryKey: ['character', 'search', query, params?.page ?? 1, params?.perPage ?? 20],
		queryFn: () => characterApi.search(query, params),
		staleTime: 5 * 60 * 1000,
		enabled: query.trim().length > 0,
	}));
}

// ============================================================================
// Staff Queries
// ============================================================================

/**
 * Get staff by ID with caching
 */
export function useStaffById(
	id: number,
	options?: Partial<CreateQueryOptions<AniListResponse<Staff>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.staff.detail(id),
		queryFn: () => staffApi.getById(id),
		staleTime: defaultStaleTime.staff,
		enabled: id > 0,
		...options,
	}));
}

/**
 * Get most favourited staff with caching
 */
export function usePopularStaff(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Page<Staff[]>>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.staff.popular(params),
		queryFn: () => staffApi.getPopular(params),
		staleTime: defaultStaleTime.popular,
		...options,
	}));
}

/**
 * Get staff with birthday today with caching
 */
export function useBirthdayStaff(
	params?: PaginationParams,
	options?: Partial<CreateQueryOptions<AniListResponse<Page<Staff[]>>>>
) {
	return createQuery(() => ({
		queryKey: anilistKeys.staff.birthday(params),
		queryFn: () => staffApi.getBirthdayToday(params),
		staleTime: defaultStaleTime.trending,
		...options,
	}));
}

/**
 * Search staff by name
 */
export function useSearchStaff(query: string, params?: PaginationParams) {
	return createQuery(() => ({
		queryKey: ['staff', 'search', query, params?.page ?? 1, params?.perPage ?? 20],
		queryFn: () => staffApi.search(query, params),
		staleTime: 5 * 60 * 1000,
		enabled: query.trim().length > 0,
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

/**
 * Browse media with comprehensive filters.
 * Accepts static params or a reactive getter `() => BrowseParams` so the
 * query automatically re-fetches when params change without violating the
 * rules of hooks (no calling inside $derived).
 * Options can also be a reactive getter for e.g. `() => ({ enabled: ... })`.
 */
export function useBrowseMedia(
	getParams: BrowseParams | (() => BrowseParams),
	getOptions?:
		| Partial<CreateQueryOptions<AniListResponse<Page<Media[]>>>>
		| (() => Partial<CreateQueryOptions<AniListResponse<Page<Media[]>>>>)
) {
	const resolveParams =
		typeof getParams === 'function' ? getParams : () => getParams;
	const resolveOptions =
		getOptions === undefined
			? () => ({})
			: typeof getOptions === 'function'
				? getOptions
				: () => getOptions;
	return createQuery(() => {
		const p = resolveParams();
		return {
			queryKey: anilistKeys.media.browse(p),
			queryFn: () => mediaApi.browse(p),
			staleTime: defaultStaleTime.browse,
			...resolveOptions(),
		};
	});
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

// ============================================================================
// Query Key Extensions (for new endpoints)
// ============================================================================

export const mediaListKeys = {
	all: ['mediaList'] as const,
	myAnime: (status?: MediaListStatus, page?: number) =>
		['mediaList', 'myAnime', status ?? 'all', page ?? 1] as const,
	myManga: (status?: MediaListStatus, page?: number) =>
		['mediaList', 'myManga', status ?? 'all', page ?? 1] as const,
	userAnime: (username: string, status?: MediaListStatus, page?: number) =>
		['mediaList', 'userAnime', username, status ?? 'all', page ?? 1] as const,
	userManga: (username: string, status?: MediaListStatus, page?: number) =>
		['mediaList', 'userManga', username, status ?? 'all', page ?? 1] as const,
} as const;

export const activityKeys = {
	all: ['activity'] as const,
	recent: (page?: number) => ['activity', 'recent', page ?? 1] as const,
	following: (page?: number) => ['activity', 'following', page ?? 1] as const,
	detail: (id: number) => ['activity', 'detail', id] as const,
	replies: (activityId: number, page?: number) => ['activity', 'replies', activityId, page ?? 1] as const,
} as const;

export const notificationKeys = {
	all: ['notification'] as const,
	list: (page?: number) => ['notification', 'list', page ?? 1] as const,
} as const;

export const forumKeys = {
	all: ['forum'] as const,
	recent: (page?: number) => ['forum', 'recent', page ?? 1] as const,
	popular: (page?: number) => ['forum', 'popular', page ?? 1] as const,
	byCategory: (categoryId: number, page?: number) => ['forum', 'category', categoryId, page ?? 1] as const,
	byUser: (userId: number, page?: number) => ['forum', 'user', userId, page ?? 1] as const,
	thread: (id: number) => ['forum', 'thread', id] as const,
	comments: (threadId: number, page?: number) => ['forum', 'comments', threadId, page ?? 1] as const,
} as const;

export const reviewKeys = {
	all: ['review'] as const,
	recent: (page?: number) => ['review', 'recent', page ?? 1] as const,
	byMedia: (mediaId: number, page?: number) => ['review', 'media', mediaId, page ?? 1] as const,
	byUser: (userId: number, page?: number) => ['review', 'user', userId, page ?? 1] as const,
	detail: (id: number) => ['review', 'detail', id] as const,
} as const;

export const recommendationKeys = {
	all: ['recommendation'] as const,
	byMedia: (mediaId: number, page?: number) => ['recommendation', 'media', mediaId, page ?? 1] as const,
} as const;

// ============================================================================
// Media List Hooks
// ============================================================================

/** Authenticated user's anime list */
export function useMyAnimeList(status?: MediaListStatus, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: mediaListKeys.myAnime(status, page),
		queryFn: () => mediaListApi.getMyAnimeList(status, page, perPage),
		staleTime: 5 * 60 * 1000,
	}));
}

/** Authenticated user's manga list */
export function useMyMangaList(status?: MediaListStatus, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: mediaListKeys.myManga(status, page),
		queryFn: () => mediaListApi.getMyMangaList(status, page, perPage),
		staleTime: 5 * 60 * 1000,
	}));
}

/** Specific user's anime list by username */
export function useUserAnimeList(username: string, status?: MediaListStatus, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: mediaListKeys.userAnime(username, status, page),
		queryFn: () => mediaListApi.getUserAnimeList(username, status, page, perPage),
		staleTime: 5 * 60 * 1000,
		enabled: username.length > 0,
	}));
}

/** Specific user's manga list by username */
export function useUserMangaList(username: string, status?: MediaListStatus, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: mediaListKeys.userManga(username, status, page),
		queryFn: () => mediaListApi.getUserMangaList(username, status, page, perPage),
		staleTime: 5 * 60 * 1000,
		enabled: username.length > 0,
	}));
}

/** Save (create/update) a media list entry */
export function useSaveListEntry() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (options: Record<string, unknown>) => mediaListApi.save(options),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
			queryClient.invalidateQueries({ queryKey: anilistKeys.user.current() });
		},
	}));
}

/** Add anime to authenticated user's list */
export function useAddAnimeToList() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ mediaId, status }: { mediaId: number; status?: MediaListStatus }) =>
			mediaListApi.addAnime(mediaId, status),
		onSuccess: (_, { mediaId }) => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
			queryClient.invalidateQueries({ queryKey: anilistKeys.media.detail(mediaId) });
		},
	}));
}

/** Add manga to authenticated user's list */
export function useAddMangaToList() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ mediaId, status }: { mediaId: number; status?: MediaListStatus }) =>
			mediaListApi.addManga(mediaId, status),
		onSuccess: (_, { mediaId }) => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
			queryClient.invalidateQueries({ queryKey: anilistKeys.media.detail(mediaId) });
		},
	}));
}

/** Update progress for a list entry */
export function useUpdateListProgress() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ entryId, progress }: { entryId: number; progress: number }) =>
			mediaListApi.updateProgress(entryId, progress),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
		},
	}));
}

/** Update score for a list entry */
export function useUpdateListScore() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ entryId, score }: { entryId: number; score: number }) =>
			mediaListApi.updateScore(entryId, score),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
		},
	}));
}

/** Update status for a list entry */
export function useUpdateListStatus() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ entryId, status }: { entryId: number; status: MediaListStatus }) =>
			mediaListApi.updateStatus(entryId, status),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
		},
	}));
}

/** Delete a media list entry */
export function useDeleteListEntry() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (id: number) => mediaListApi.deleteEntry(id),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: mediaListKeys.all });
		},
	}));
}

// ============================================================================
// Activity Hooks
// ============================================================================

/** Recent global activity feed */
export function useRecentActivity(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: activityKeys.recent(page),
		queryFn: () => activityApi.getRecent(page, perPage),
		staleTime: 2 * 60 * 1000,
	}));
}

/** Activity feed from followed users */
export function useFollowingActivity(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: activityKeys.following(page),
		queryFn: () => activityApi.getFollowing(page, perPage),
		staleTime: 2 * 60 * 1000,
	}));
}

/** Single activity by ID */
export function useActivityById(id: number) {
	return createQuery(() => ({
		queryKey: activityKeys.detail(id),
		queryFn: () => activityApi.getById(id),
		staleTime: 5 * 60 * 1000,
		enabled: id > 0,
	}));
}

/** Replies to a specific activity */
export function useActivityReplies(activityId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: activityKeys.replies(activityId, page),
		queryFn: () => activityApi.fetchReplies(activityId, page, perPage),
		staleTime: 2 * 60 * 1000,
		enabled: activityId > 0,
	}));
}

/** Save a text activity post */
export function useSaveTextActivity() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (options: Record<string, unknown>) => activityApi.saveText(options),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: activityKeys.all });
		},
	}));
}

/** Reply to an activity */
export function useSaveActivityReply() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (options: Record<string, unknown>) => activityApi.saveReply(options),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: activityKeys.all });
		},
	}));
}

/** Delete an activity */
export function useDeleteActivity() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (id: number) => activityApi.delete(id),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: activityKeys.all });
		},
	}));
}

// ============================================================================
// Notification Hooks
// ============================================================================

/** All notifications (unread first) */
export function useNotifications(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: notificationKeys.list(page),
		queryFn: () => notificationApi.getAll(page, perPage),
		staleTime: 60 * 1000, // 1 min — notifications change frequently
	}));
}

/** Mark all notifications as read and return them */
export function useMarkNotificationsRead() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ page, perPage }: { page?: number; perPage?: number } = {}) =>
			notificationApi.getAndMarkRead(page, perPage),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: notificationKeys.all });
			queryClient.invalidateQueries({ queryKey: anilistKeys.user.current() });
		},
	}));
}

// ============================================================================
// Forum Hooks
// ============================================================================

/** Recent forum threads */
export function useRecentForumThreads(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: forumKeys.recent(page),
		queryFn: () => forumApi.getRecent(page, perPage),
		staleTime: 5 * 60 * 1000,
	}));
}

/** Popular forum threads */
export function usePopularForumThreads(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: forumKeys.popular(page),
		queryFn: () => forumApi.getPopular(page, perPage),
		staleTime: 5 * 60 * 1000,
	}));
}

/** Threads by category */
export function useForumThreadsByCategory(categoryId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: forumKeys.byCategory(categoryId, page),
		queryFn: () => forumApi.getByCategory(categoryId, page, perPage),
		staleTime: 5 * 60 * 1000,
		enabled: categoryId > 0,
	}));
}

/** Single forum thread */
export function useForumThread(id: number) {
	return createQuery(() => ({
		queryKey: forumKeys.thread(id),
		queryFn: () => forumApi.getThread(id),
		staleTime: 5 * 60 * 1000,
		enabled: id > 0,
	}));
}

/** Comments on a forum thread */
export function useThreadComments(threadId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: forumKeys.comments(threadId, page),
		queryFn: () => forumApi.getComments(threadId, page, perPage),
		staleTime: 2 * 60 * 1000,
		enabled: threadId > 0,
	}));
}

/** Reply to a thread */
export function useReplyToThread() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ threadId, comment }: { threadId: number; comment: string }) =>
			forumApi.replyToThread(threadId, comment),
		onSuccess: (_, { threadId }) => {
			queryClient.invalidateQueries({ queryKey: forumKeys.comments(threadId) });
		},
	}));
}

/** Toggle subscription to a thread */
export function useToggleThreadSubscription() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ threadId, subscribe }: { threadId: number; subscribe: boolean }) =>
			forumApi.toggleSubscription(threadId, subscribe),
		onSuccess: (_, { threadId }) => {
			queryClient.invalidateQueries({ queryKey: forumKeys.thread(threadId) });
		},
	}));
}

/** Toggle like on a forum thread */
export function useToggleLikeThread() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ id }: { id: number }) => forumApi.toggleLikeThread(id),
		onSuccess: (_, { id }) => {
			queryClient.invalidateQueries({ queryKey: forumKeys.thread(id) });
		},
	}));
}

/** Toggle like on a thread comment */
export function useToggleLikeComment() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ id, threadId }: { id: number; threadId: number }) =>
			forumApi.toggleLikeComment(id),
		onSuccess: (_, { threadId }) => {
			queryClient.invalidateQueries({ queryKey: forumKeys.comments(threadId) });
		},
	}));
}

/** Reply to a specific comment (nested reply) */
export function useReplyToComment() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({
			threadId,
			parentCommentId,
			comment,
		}: {
			threadId: number;
			parentCommentId: number;
			comment: string;
		}) => forumApi.replyToComment(threadId, parentCommentId, comment),
		onSuccess: (_, { threadId }) => {
			queryClient.invalidateQueries({ queryKey: forumKeys.comments(threadId) });
		},
	}));
}

// ============================================================================
// Review Hooks
// ============================================================================

/** Reviews for a specific media */
export function useMediaReviews(mediaId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: reviewKeys.byMedia(mediaId, page),
		queryFn: () => reviewApi.getByMedia(mediaId, page, perPage),
		staleTime: 10 * 60 * 1000,
		enabled: mediaId > 0,
	}));
}

/** Reviews by a user */
export function useUserReviews(userId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: reviewKeys.byUser(userId, page),
		queryFn: () => reviewApi.getByUser(userId, page, perPage),
		staleTime: 10 * 60 * 1000,
		enabled: userId > 0,
	}));
}

/** Recent reviews */
export function useRecentReviews(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: reviewKeys.recent(page),
		queryFn: () => reviewApi.getRecent(page, perPage),
		staleTime: 10 * 60 * 1000,
	}));
}

/**
 * Global recommendations sorted by rating
 */
export function useGlobalRecommendations(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: ['recommendation', 'global', page ?? 1, perPage ?? 25],
		queryFn: () => recommendationApi.getGlobal(page, perPage),
		staleTime: 10 * 60 * 1000,
	}));
}

/** Single review by ID */
export function useReviewById(id: number) {
	return createQuery(() => ({
		queryKey: reviewKeys.detail(id),
		queryFn: () => reviewApi.getById(id),
		staleTime: 15 * 60 * 1000,
		enabled: id > 0,
	}));
}

/** Save a review */
export function useSaveReview() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: (options: Record<string, unknown>) => reviewApi.save(options),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: reviewKeys.all });
		},
	}));
}

/** Rate a review */
export function useRateReview() {
	const queryClient = useQueryClient();
	return createMutation(() => ({
		mutationFn: ({ reviewId, rating }: { reviewId: number; rating: string }) =>
			reviewApi.rate({ reviewId, rating }),
		onSuccess: (_, { reviewId }) => {
			queryClient.invalidateQueries({ queryKey: reviewKeys.detail(reviewId) });
		},
	}));
}

// ============================================================================
// Recommendation Hooks
// ============================================================================

/** Recommendations for a media */
export function useMediaRecommendations(mediaId: number, page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: recommendationKeys.byMedia(mediaId, page),
		queryFn: () => recommendationApi.getByMedia(mediaId, page, perPage),
		staleTime: 30 * 60 * 1000,
		enabled: mediaId > 0,
	}));
}

// ============================================================================
// Re-export supplemental types for convenience
// ============================================================================

export type {
	MediaList,
	MediaListStatus,
	ActivityUnion,
	ActivityReply,
	NotificationUnion,
	Thread,
	ThreadComment,
	Review,
	Recommendation,
};

// ============================================================================
// Airing Queries
// ============================================================================

export function useAiringAnime(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: ['airing', 'schedule', page, perPage],
		queryFn: () => airingApi.getAiring(page, perPage),
		staleTime: 5 * 60 * 1000, // 5 minutes - airing schedule changes frequently
	}));
}

export function useUpcomingAnime(page?: number, perPage?: number) {
	return createQuery(() => ({
		queryKey: ['airing', 'upcoming', page, perPage],
		queryFn: () => airingApi.getUpcoming(page, perPage),
		staleTime: 15 * 60 * 1000,
	}));
}
