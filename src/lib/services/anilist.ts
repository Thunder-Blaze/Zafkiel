/**
 * AniList API Service
 * Wraps Tauri commands for AniList operations
 */

import { invoke } from '@tauri-apps/api/core';
import type {
	AniListResponse,
	Media,
	User,
	Studio,
	Character,
	Staff,
	PaginationParams,
	SeasonalAnimeParams,
	SearchParams,
	BrowseParams,
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
import { toast } from 'svelte-sonner';

/**
 * Helper to invoke a Tauri command and show a success toast on completion
 */
async function invokeWithToast<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	try {
		const response = await invoke<T>(cmd, args);
		toast.success(`AniList: Fetched ${cmd.replace(/_/g, ' ')}`);
		return response;
	} catch (error) {
		console.error(`[AniList API] Error in ${cmd}:`, error);
		toast.error(`AniList: Failed ${cmd.replace(/_/g, ' ')}`);
		throw error;
	}
}

// ============================================================================
// Media Browse API (Generic for both Anime and Manga)
// ============================================================================

export const mediaApi = {
	/**
	 * Browse media with comprehensive filters
	 */
	browse: async (params: BrowseParams): Promise<AniListResponse<Page<Media[]>>> => {
		console.log('[AniList API] Calling browse_media:', params);
		const response = (await invokeWithToast('browse_media', {
			mediaType: params.mediaType ?? null,
			search: params.search ?? null,
			season: params.season ?? null,
			seasonYear: params.seasonYear ?? null,
			format: params.format ?? null,
			status: params.status ?? null,
			source: params.source ?? null,
			genres: params.genres ?? null,
			genresExcluded: params.genresExcluded ?? null,
			sortBy: params.sortBy ?? null,
			isAdult: params.isAdult ?? null,
			countryOfOrigin: params.countryOfOrigin ?? null,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		})) as AniListResponse<Page<Media[]>>;
		console.log('[AniList API] browse_media response:', response);
		return response;
	},
};

// ============================================================================
// Anime API
// ============================================================================

export const animeApi = {
	/**
	 * Search for anime
	 */
	search: async (params: SearchParams): Promise<AniListResponse<Media[]>> => {
		console.log('[AniList API] Calling search_anime:', params);
		const response = (await invokeWithToast('search_anime', {
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
		return invokeWithToast('get_anime_by_id', { id });
	},

	/**
	 * Get anime characters by ID with language and pagination
	 */
	getCharactersById: async (
		id: number,
		page?: number,
		perPage?: number,
		language?: string
	): Promise<AniListResponse<Media>> => {
		return invokeWithToast('get_anime_characters_by_id', {
			id,
			page,
			perPage,
			language,
		});
	},

	/**
	 * Get anime staff by ID with pagination
	 */
	getStaffById: async (id: number, page?: number, perPage?: number): Promise<AniListResponse<Media>> => {
		return invokeWithToast('get_anime_staff_by_id', {
			id,
			page,
			perPage,
		});
	},

	/**
	 * Get trending anime
	 */
	getTrending: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		try {
			console.log('[AniList API] Calling get_trending_anime:', params);
			const response = (await invokeWithToast('get_trending_anime', {
				page: params?.page ?? null,
				perPage: params?.perPage ?? null,
			})) as { success: boolean; data?: { data: Media[] }; error?: string };
			console.log('[AniList API] get_trending_anime response:', response);

			if (!response.success) {
				return {
					success: false,
					error: response.error || 'Backend returned error',
					data: [],
				};
			}

			return {
				success: true,
				data: response.data?.data || [],
			};
		} catch (error) {
			console.error('[AniList API] Error in getTrending:', error);
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Unknown error',
				data: [],
			};
		}
	},

	/**
	 * Get popular anime
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		try {
			const response = (await invokeWithToast('get_popular_anime', {
				page: params?.page ?? null,
				perPage: params?.perPage ?? null,
			})) as { success: boolean; data?: { data: Media[] }; error?: string };

			if (!response.success) {
				return { success: false, error: response.error || 'Backend returned error', data: [] };
			}

			return { success: true, data: response.data?.data ?? [] };
		} catch (error) {
			console.error('[AniList API] Error in anime getPopular:', error);
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Unknown error',
				data: [],
			};
		}
	},

	/**
	 * Get seasonal anime
	 */
	getSeasonal: async (params: SeasonalAnimeParams): Promise<AniListResponse<Page<Media[]>>> => {
		return invokeWithToast('get_seasonal_anime', {
			season: params.season,
			year: params.year,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		});
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
		return invokeWithToast('search_manga', {
			query: params.query,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		});
	},

	/**
	 * Get manga by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Media>> => {
		return invokeWithToast('get_manga_by_id', { id });
	},

	/**
	 * Get trending manga
	 */
	getTrending: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		try {
			const response = (await invokeWithToast('get_trending_manga', {
				page: params?.page ?? null,
				perPage: params?.perPage ?? null,
			})) as { success: boolean; data?: { data: Media[] }; error?: string };

			if (!response.success) {
				return { success: false, error: response.error || 'Backend returned error', data: [] };
			}

			return { success: true, data: response.data?.data ?? [] };
		} catch (error) {
			console.error('[AniList API] Error in manga getTrending:', error);
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Unknown error',
				data: [],
			};
		}
	},

	/**
	 * Get popular manga
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Media[]>> => {
		try {
			const response = (await invokeWithToast('get_popular_manga', {
				page: params?.page ?? null,
				perPage: params?.perPage ?? null,
			})) as { success: boolean; data?: { data: Media[] }; error?: string };

			if (!response.success) {
				return { success: false, error: response.error || 'Backend returned error', data: [] };
			}

			return { success: true, data: response.data?.data ?? [] };
		} catch (error) {
			console.error('[AniList API] Error in manga getPopular:', error);
			return {
				success: false,
				error: error instanceof Error ? error.message : 'Unknown error',
				data: [],
			};
		}
	},
};

// ============================================================================
// User API
// ============================================================================

export const userApi = {
	getCurrent: async (): Promise<AniListResponse<User>> => {
		return invokeWithToast('get_current_user');
	},

	/**
	 * Get basic user info (for dashboard)
	 */
	fetchBasic: async (): Promise<AniListResponse<User>> => {
		return invokeWithToast('fetch_basic');
	},

	/**
	 * Get user by ID
	 */
	getById: async (id: number): Promise<AniListResponse<User>> => {
		return invokeWithToast('get_user_by_id', { id });
	},

	/**
	 * Get user by name
	 */
	getByName: async (name: string): Promise<AniListResponse<User>> => {
		return invokeWithToast('get_user_by_name', { name });
	},

	/**
	 * Search users
	 */
	search: async (params: SearchParams): Promise<AniListResponse<User[]>> => {
		return invokeWithToast('search_users', {
			query: params.query,
			page: params.page ?? null,
			perPage: params.perPage ?? null,
		});
	},
};

// ============================================================================
// Studio API
// ============================================================================

export const studioApi = {
	/**
	 * Get studio by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Studio>> => {
		return invokeWithToast('get_studio_by_id', { id });
	},
};

// ============================================================================
// Character API
// ============================================================================

export const characterApi = {
	/**
	 * Get character by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Character>> => {
		return invokeWithToast('get_character_by_id', { id });
	},

	/**
	 * Get most favourited characters
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Page<Character[]>>> => {
		return invokeWithToast('get_popular_characters', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Get characters with birthday today
	 */
	getBirthdayToday: async (
		params?: PaginationParams
	): Promise<AniListResponse<Page<Character[]>>> => {
		return invokeWithToast('get_birthday_characters', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Search characters by name, optionally filtered to today's birthdays
	 */
	search: async (
		query: string,
		params?: PaginationParams & { isBirthday?: boolean }
	): Promise<AniListResponse<Page<Character[]>>> => {
		return invokeWithToast('search_characters', {
			query,
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
			isBirthday: params?.isBirthday ?? null,
		});
	},
};

// ============================================================================
// Staff API
// ============================================================================

export const staffApi = {
	/**
	 * Get staff by ID
	 */
	getById: async (id: number): Promise<AniListResponse<Staff>> => {
		return invokeWithToast('get_staff_by_id', { id });
	},

	/**
	 * Get most favourited staff
	 */
	getPopular: async (params?: PaginationParams): Promise<AniListResponse<Page<Staff[]>>> => {
		return invokeWithToast('get_popular_staff', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Get staff with birthday today
	 */
	getBirthdayToday: async (params?: PaginationParams): Promise<AniListResponse<Page<Staff[]>>> => {
		return invokeWithToast('get_birthday_staff', {
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
		});
	},

	/**
	 * Search staff by name, optionally filtered to today's birthdays
	 */
	search: async (
		query: string,
		params?: PaginationParams & { isBirthday?: boolean }
	): Promise<AniListResponse<Page<Staff[]>>> => {
		return invokeWithToast('search_staff', {
			query,
			page: params?.page ?? null,
			perPage: params?.perPage ?? null,
			isBirthday: params?.isBirthday ?? null,
		});
	},
};

// ============================================================================
// Combined API Export
// ============================================================================

// ============================================================================
// Media List API
// ============================================================================

export const mediaListApi = {
	fetch: async (options: Record<string, unknown>): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('fetch_media_list', { options });
	},
	getMyAnimeList: async (
		status?: MediaListStatus,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_my_anime_list', {
			status: status ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getMyMangaList: async (
		status?: MediaListStatus,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_my_manga_list', {
			status: status ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getUserAnimeList: async (
		username: string,
		status?: MediaListStatus,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_user_anime_list', {
			username,
			status: status ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getUserMangaList: async (
		username: string,
		status?: MediaListStatus,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_user_manga_list', {
			username,
			status: status ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getWatching: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_watching', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getReading: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_reading', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getPlanToWatch: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_plan_to_watch', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getPlanToRead: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_plan_to_read', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getCompletedAnime: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_completed_anime', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getCompletedManga: async (
		username?: string,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<MediaList[]>>> => {
		return invokeWithToast('get_completed_manga', {
			username: username ?? null,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	save: async (options: Record<string, unknown>): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('save_media_list_entry', { options });
	},
	addAnime: async (
		mediaId: number,
		status?: MediaListStatus
	): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('add_anime_to_list', { mediaId, status: status ?? null });
	},
	addManga: async (
		mediaId: number,
		status?: MediaListStatus
	): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('add_manga_to_list', { mediaId, status: status ?? null });
	},
	updateProgress: async (
		entryId: number,
		progress: number
	): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('update_media_progress', { entryId, progress });
	},
	updateScore: async (entryId: number, score: number): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('update_media_score', { entryId, score });
	},
	updateStatus: async (
		entryId: number,
		status: MediaListStatus
	): Promise<AniListResponse<MediaList>> => {
		return invokeWithToast('update_media_status', { entryId, status });
	},
	deleteEntry: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_media_list_entry', { id });
	},
};

// ============================================================================
// Activity API
// ============================================================================

export const activityApi = {
	fetch: async (
		options: Record<string, unknown>
	): Promise<AniListResponse<Page<ActivityUnion[]>>> => {
		return invokeWithToast('fetch_activities', { options });
	},
	getById: async (id: number): Promise<AniListResponse<ActivityUnion>> => {
		return invokeWithToast('get_activity_by_id', { id });
	},
	getRecent: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<ActivityUnion[]>>> => {
		return invokeWithToast('get_recent_activity', { page: page ?? null, perPage: perPage ?? null });
	},
	getFollowing: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<ActivityUnion[]>>> => {
		return invokeWithToast('get_following_activity', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	fetchReplies: async (
		activityId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<ActivityReply[]>>> => {
		return invokeWithToast('fetch_activity_replies', {
			activityId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	saveText: async (options: Record<string, unknown>): Promise<AniListResponse<ActivityUnion>> => {
		return invokeWithToast('save_text_activity', { options });
	},
	saveMessage: async (
		options: Record<string, unknown>
	): Promise<AniListResponse<ActivityUnion>> => {
		return invokeWithToast('save_message_activity', { options });
	},
	saveReply: async (options: Record<string, unknown>): Promise<AniListResponse<ActivityReply>> => {
		return invokeWithToast('save_activity_reply', { options });
	},
	delete: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_activity', { id });
	},
	deleteReply: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_activity_reply', { id });
	},
	toggleSubscription: async (
		id: number,
		subscribe: boolean
	): Promise<AniListResponse<ActivityUnion>> => {
		return invokeWithToast('toggle_activity_subscription', { id, subscribe });
	},
};

// ============================================================================
// Notification API
// ============================================================================

export const notificationApi = {
	fetch: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<NotificationUnion[]>>> => {
		return invokeWithToast('fetch_notifications', { page: page ?? null, perPage: perPage ?? null });
	},
	getAll: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<NotificationUnion[]>>> => {
		return invokeWithToast('get_all_notifications', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getAndMarkRead: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<NotificationUnion[]>>> => {
		return invokeWithToast('get_and_mark_notifications_read', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
};

// ============================================================================
// Forum API
// ============================================================================

export const forumApi = {
	search: async (options: Record<string, unknown>): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('search_forum_threads', { options });
	},
	getThread: async (id: number): Promise<AniListResponse<Thread>> => {
		return invokeWithToast('get_forum_thread', { id });
	},
	getRecent: async (page?: number, perPage?: number): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('get_recent_forum_threads', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getPopular: async (page?: number, perPage?: number): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('get_popular_forum_threads', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getByCategory: async (
		categoryId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('get_forum_threads_by_category', {
			categoryId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getByUser: async (
		userId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('get_forum_threads_by_user', {
			userId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getSubscribed: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Thread[]>>> => {
		return invokeWithToast('get_subscribed_forum_threads', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getComments: async (
		threadId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<ThreadComment[]>>> => {
		return invokeWithToast('get_thread_comments', {
			threadId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getComment: async (id: number): Promise<AniListResponse<ThreadComment>> => {
		return invokeWithToast('get_thread_comment_by_id', { id });
	},
	saveThread: async (options: Record<string, unknown>): Promise<AniListResponse<Thread>> => {
		return invokeWithToast('save_forum_thread', { options });
	},
	deleteThread: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_forum_thread', { id });
	},
	saveComment: async (
		options: Record<string, unknown>
	): Promise<AniListResponse<ThreadComment>> => {
		return invokeWithToast('save_thread_comment', { options });
	},
	deleteComment: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_thread_comment', { id });
	},
	toggleSubscription: async (
		threadId: number,
		subscribe: boolean
	): Promise<AniListResponse<Thread>> => {
		return invokeWithToast('toggle_forum_thread_subscription', { threadId, subscribe });
	},
	replyToThread: async (
		threadId: number,
		comment: string
	): Promise<AniListResponse<ThreadComment>> => {
		return invokeWithToast('reply_to_forum_thread', { threadId, comment });
	},
	replyToComment: async (
		threadId: number,
		parentCommentId: number,
		comment: string
	): Promise<AniListResponse<ThreadComment>> => {
		return invokeWithToast('reply_to_thread_comment', { threadId, parentCommentId, comment });
	},
	toggleLikeThread: async (id: number): Promise<AniListResponse<unknown>> => {
		return invokeWithToast('toggle_like_thread', { id });
	},
	toggleLikeComment: async (id: number): Promise<AniListResponse<unknown>> => {
		return invokeWithToast('toggle_like_thread_comment', { id });
	},
};

// ============================================================================
// Review API
// ============================================================================

export const reviewApi = {
	fetch: async (options: Record<string, unknown>): Promise<AniListResponse<Page<Review[]>>> => {
		return invokeWithToast('fetch_reviews', { options });
	},
	getByMedia: async (
		mediaId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Review[]>>> => {
		return invokeWithToast('get_reviews_by_media', {
			mediaId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getByUser: async (
		userId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Review[]>>> => {
		return invokeWithToast('get_reviews_by_user', {
			userId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	getById: async (id: number): Promise<AniListResponse<Review>> => {
		return invokeWithToast('get_review_by_id', { id });
	},
	getRecent: async (page?: number, perPage?: number): Promise<AniListResponse<Page<Review[]>>> => {
		return invokeWithToast('get_recent_reviews', { page: page ?? null, perPage: perPage ?? null });
	},
	save: async (options: Record<string, unknown>): Promise<AniListResponse<Review>> => {
		return invokeWithToast('save_review', { options });
	},
	delete: async (id: number): Promise<AniListResponse<boolean>> => {
		return invokeWithToast('delete_review', { id });
	},
	rate: async (options: { reviewId: number; rating: string }): Promise<AniListResponse<Review>> => {
		return invokeWithToast('rate_review', { options });
	},
};

// ============================================================================
// Recommendation API
// ============================================================================

export const recommendationApi = {
	fetch: async (
		options: Record<string, unknown>
	): Promise<AniListResponse<Page<Recommendation[]>>> => {
		return invokeWithToast('fetch_recommendations', { options });
	},
	getByMedia: async (
		mediaId: number,
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Recommendation[]>>> => {
		return invokeWithToast('get_recommendations_by_media', {
			mediaId,
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	/**
	 * Get global recommendations sorted by rating (no mediaId filter)
	 */
	getGlobal: async (
		page?: number,
		perPage?: number
	): Promise<AniListResponse<Page<Recommendation[]>>> => {
		return invoke('fetch_recommendations', {
			options: { per_page: perPage ?? 25, page: page ?? 1 },
		});
	},
	save: async (options: Record<string, unknown>): Promise<AniListResponse<Recommendation>> => {
		return invoke('save_recommendation', { options });
	},
};

// ============================================================================
// Airing API
// ============================================================================

export const airingApi = {
	/**
	 * Get currently airing anime
	 */
	getAiring: async (page?: number, perPage?: number): Promise<AniListResponse<Page<Media[]>>> => {
		return invoke('get_airing_anime', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
	/**
	 * Get upcoming anime
	 */
	getUpcoming: async (page?: number, perPage?: number): Promise<AniListResponse<Page<Media[]>>> => {
		return invoke('get_upcoming_anime', {
			page: page ?? null,
			perPage: perPage ?? null,
		});
	},
};

// ============================================================================
// Combined API Export
// ============================================================================

// ============================================================================
// Combined Search API
// ============================================================================

export const searchApi = {
	/**
	 * Search all categories (anime, manga, characters, staff, studios, users)
	 * in a single GraphQL request to minimise rate-limit usage.
	 */
	searchAll: async (
		query: string,
		perPage = 5
	): Promise<
		import('$lib/types/anilist').AniListResponse<import('$lib/types/anilist').SearchAllResults>
	> => {
		return invoke('search_all', { query, perPage });
	},
};

export const anilistApi = {
	media: mediaApi,
	anime: animeApi,
	manga: mangaApi,
	user: userApi,
	studio: studioApi,
	character: characterApi,
	staff: staffApi,
	mediaList: mediaListApi,
	activity: activityApi,
	notification: notificationApi,
	forum: forumApi,
	review: reviewApi,
	recommendation: recommendationApi,
	airing: airingApi,
	search: searchApi,
};
