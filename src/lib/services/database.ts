/**
 * Database service for caching AniList data in SQLite
 * Handles offline storage and retrieval of media, users, and other data
 */

import { db } from '$lib/server/db/index';
import {
	users,
	media,
	cached_images,
	recently_viewed,
	search_cache,
} from '$lib/server/db/schema';
import { eq, like, and, desc, inArray } from 'drizzle-orm';
import type { Media, User } from '$lib/types/anilist';

export class DatabaseService {
	/**
	 * Cache user data
	 */
	static async cacheUser(userData: User): Promise<void> {
		const now = Date.now();

		await db.insert(users).values({
			id: userData.id,
			name: userData.name,
			avatar_large: userData.avatar?.large || null,
			banner_image: userData.bannerImage,
			anime_count: userData.statistics?.anime?.count || 0,
			manga_count: userData.statistics?.manga?.count || 0,
			episodes_watched: userData.statistics?.anime?.episodesWatched || 0,
			chapters_read: userData.statistics?.manga?.chaptersRead || 0,
			cached_at: now,
			updated_at: userData.updatedAt,
		}).onConflictDoUpdate({
			target: users.id,
			set: {
				name: userData.name,
				avatar_large: userData.avatar?.large || null,
				banner_image: userData.bannerImage,
				anime_count: userData.statistics?.anime?.count || 0,
				manga_count: userData.statistics?.manga?.count || 0,
				episodes_watched: userData.statistics?.anime?.episodesWatched || 0,
				chapters_read: userData.statistics?.manga?.chaptersRead || 0,
				cached_at: now,
				updated_at: userData.updatedAt,
			},
		});
	}

	/**
	 * Get cached user
	 */
	static async getCachedUser(userId: number): Promise<typeof users.$inferSelect | null> {
		const result = await db.select().from(users).where(eq(users.id, userId)).limit(1);
		return result[0] || null;
	}

	/**
	 * Cache media data
	 */
	static async cacheMedia(mediaData: Media, extensionSource?: string): Promise<void> {
		const now = Date.now();

		// Get user preferred title
		const title = mediaData.title?.userPreferred ||
		             mediaData.title?.english ||
		             mediaData.title?.romaji ||
		             mediaData.title?.native ||
		             'Unknown Title';

		// Calculate total units (episodes for anime, chapters for manga)
		const totalUnits = mediaData.type === 'ANIME' ? mediaData.episodes : mediaData.chapters;

		await db.insert(media).values({
			id: mediaData.id,
			type: mediaData.type || 'ANIME',
			format: mediaData.format,
			status: mediaData.status,
			title,
			description: mediaData.description,
			genres: mediaData.genres ? JSON.stringify(mediaData.genres) : null,
			total_units: totalUnits,
			duration: mediaData.duration,
			average_score: mediaData.averageScore,
			popularity: mediaData.popularity,
			favourites: mediaData.favourites,
			is_adult: mediaData.isAdult || false,
			user_status: mediaData.mediaListEntry?.status,
			user_score: mediaData.mediaListEntry?.score,
			user_progress: mediaData.mediaListEntry?.progress,
			local_progress: mediaData.mediaListEntry?.progress || 0, // Initialize with AniList progress
			extension_source: extensionSource,
			data_source: 'anilist',
			cached_at: now,
			updated_at: mediaData.updatedAt,
		}).onConflictDoUpdate({
			target: media.id,
			set: {
				type: mediaData.type || 'ANIME',
				format: mediaData.format,
				status: mediaData.status,
				title,
				description: mediaData.description,
				genres: mediaData.genres ? JSON.stringify(mediaData.genres) : null,
				total_units: totalUnits,
				duration: mediaData.duration,
				average_score: mediaData.averageScore,
				popularity: mediaData.popularity,
				favourites: mediaData.favourites,
				is_adult: mediaData.isAdult || false,
				user_status: mediaData.mediaListEntry?.status,
				user_score: mediaData.mediaListEntry?.score,
				user_progress: mediaData.mediaListEntry?.progress,
				extension_source: extensionSource,
				cached_at: now,
				updated_at: mediaData.updatedAt,
			},
		});
	}

	/**
	 * Get cached media
	 */
	static async getCachedMedia(mediaId: number): Promise<typeof media.$inferSelect | null> {
		const result = await db.select().from(media).where(eq(media.id, mediaId)).limit(1);
		return result[0] || null;
	}

	/**
	 * Cache media list (search results, trending, etc.)
	 */
	static async cacheMediaList(mediaList: Media[], extensionSource?: string): Promise<void> {
		for (const mediaItem of mediaList) {
			await this.cacheMedia(mediaItem, extensionSource);
		}
	}

	/**
	 * Search cached media
	 */
	static async searchCachedMedia(
		query: string,
		type?: 'ANIME' | 'MANGA',
		limit: number = 20
	): Promise<(typeof media.$inferSelect)[]> {
		const conditions = [like(media.title, `%${query}%`)];

		if (type) {
			conditions.push(eq(media.type, type));
		}

		return await db.select()
			.from(media)
			.where(and(...conditions))
			.orderBy(desc(media.popularity))
			.limit(limit);
	}

	/**
	 * Get recently viewed media
	 */
	static async getRecentlyViewed(limit: number = 10): Promise<(typeof media.$inferSelect)[]> {
		const recentViews = await db.select()
			.from(recently_viewed)
			.orderBy(desc(recently_viewed.viewed_at))
			.limit(limit);

		if (recentViews.length === 0) return [];

		const mediaIds = recentViews.map(view => view.media_id);
		return await db.select()
			.from(media)
			.where(inArray(media.id, mediaIds));
	}

	/**
	 * Add to recently viewed
	 */
	static async addToRecentlyViewed(mediaId: number): Promise<void> {
		const now = Date.now();

		// Remove existing entry
		await db.delete(recently_viewed).where(eq(recently_viewed.media_id, mediaId));

		// Add new entry
		await db.insert(recently_viewed).values({
			media_id: mediaId,
			viewed_at: now,
		});

		// Keep only last 50 entries
		const allViews = await db.select()
			.from(recently_viewed)
			.orderBy(desc(recently_viewed.viewed_at));

		if (allViews.length > 50) {
			const toDelete = allViews.slice(50).map(view => view.id);
			await db.delete(recently_viewed).where(inArray(recently_viewed.id, toDelete));
		}
	}

	/**
	 * Cache search results
	 */
	static async cacheSearchResults(
		query: string,
		mediaType: 'ANIME' | 'MANGA' | null,
		results: Media[],
		totalCount: number
	): Promise<void> {
		const now = Date.now();
		const expiresAt = now + (30 * 60 * 1000); // 30 minutes TTL

		// Cache individual media items
		await this.cacheMediaList(results);

		// Cache search query
		const mediaIds = results.map(item => item.id);
		await db.insert(search_cache).values({
			query: query.toLowerCase().trim(),
			media_type: mediaType,
			results: JSON.stringify(mediaIds),
			total_count: totalCount,
			cached_at: now,
			expires_at: expiresAt,
		}).onConflictDoUpdate({
			target: [search_cache.query, search_cache.media_type],
			set: {
				results: JSON.stringify(mediaIds),
				total_count: totalCount,
				cached_at: now,
				expires_at: expiresAt,
			},
		});
	}

	/**
	 * Get cached search results
	 */
	static async getCachedSearchResults(
		query: string,
		mediaType: 'ANIME' | 'MANGA' | null
	): Promise<{ results: (typeof media.$inferSelect)[], totalCount: number } | null> {
		const now = Date.now();
		const searchQuery = query.toLowerCase().trim();

		let queryConditions;
		if (mediaType === null) {
			queryConditions = eq(search_cache.query, searchQuery);
		} else {
			queryConditions = and(
				eq(search_cache.query, searchQuery),
				eq(search_cache.media_type, mediaType)
			);
		}

		const cached = await db.select()
			.from(search_cache)
			.where(queryConditions)
			.limit(1);

		if (cached.length === 0 || cached[0].expires_at < now) {
			return null;
		}

		const mediaIds = JSON.parse(cached[0].results) as number[];
		const results = await db.select()
			.from(media)
			.where(inArray(media.id, mediaIds));

		return {
			results,
			totalCount: cached[0].total_count || 0,
		};
	}

	/**
	 * Update local progress
	 */
	static async updateLocalProgress(
		mediaId: number,
		progress: number,
		timestamp?: number
	): Promise<void> {
		const updates: Partial<typeof media.$inferInsert> = {
			local_progress: progress,
		};

		if (timestamp !== undefined) {
			updates.last_timestamp_watched = timestamp;
		}

		await db.update(media)
			.set(updates)
			.where(eq(media.id, mediaId));
	}

	/**
	 * Cache image and return cached image ID
	 */
	static async cacheImage(originalUrl: string, localPath: string): Promise<number> {
		const now = Date.now();

		const result = await db.insert(cached_images).values({
			original_url: originalUrl,
			local_path: localPath,
			last_accessed: now,
		}).onConflictDoUpdate({
			target: cached_images.original_url,
			set: {
				last_accessed: now,
			},
		}).returning({ id: cached_images.id });

		return result[0].id;
	}

	/**
	 * Get cached image path
	 */
	static async getCachedImagePath(originalUrl: string): Promise<string | null> {
		const result = await db.select()
			.from(cached_images)
			.where(eq(cached_images.original_url, originalUrl))
			.limit(1);

		if (result.length === 0) return null;

		// Update last accessed
		await db.update(cached_images)
			.set({ last_accessed: Date.now() })
			.where(eq(cached_images.id, result[0].id));

		return result[0].local_path;
	}

	/**
	 * Remove cached image by URL
	 */
	static async removeCachedImage(originalUrl: string): Promise<void> {
		try {
			await db.delete(cached_images).where(eq(cached_images.original_url, originalUrl));
		} catch (error) {
			console.error('Failed to remove cached image:', error);
		}
	}

	/**
	 * Get all cached images (for management)
	 */
	static async getAllCachedImages(): Promise<Array<{
		id: number;
		original_url: string;
		local_path: string;
		last_accessed: number;
	}>> {
		try {
			return await db.select().from(cached_images);
		} catch (error) {
			console.error('Failed to get cached images:', error);
			return [];
		}
	}

	/**
	 * Clean up old cache entries
	 */
	static async cleanupCache(): Promise<void> {
		const oneWeekAgo = Date.now() - (7 * 24 * 60 * 60 * 1000);

		// Clean up expired search cache
		await db.delete(search_cache)
			.where(and(
				eq(search_cache.expires_at, oneWeekAgo)
			));

		// Clean up old images (not accessed in 30 days)
		const thirtyDaysAgo = Date.now() - (30 * 24 * 60 * 60 * 1000);
		await db.delete(cached_images)
			.where(and(
				eq(cached_images.last_accessed, thirtyDaysAgo)
			));
	}
}
