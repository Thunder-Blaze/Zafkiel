/**
 * Client-side database service that communicates with Tauri backend
 * This replaces direct database access from browser code
 */

import { invoke } from '@tauri-apps/api/core';
import type { Media, User } from '$lib/types/anilist';

export interface UpdateProgressParams {
	media_id: number;
	progress: number;
	timestamp?: number;
}

export interface CacheMediaParams {
	media_data: Media;
	extension_source?: string;
}

export interface CacheUserParams {
	user_data: User;
}

export interface RecentlyViewedParams {
	media_id: number;
	title?: string;
	cover_url?: string;
}

export interface CachedImageInfo {
	id: number;
	original_url: string;
	local_path: string;
	file_size?: number;
	cached_at: number;
	last_accessed: number;
}

export class ClientDatabaseService {
	/**
	 * Update local progress for media via Tauri command
	 */
	static async updateLocalProgress(
		mediaId: number,
		progress: number,
		timestamp?: number
	): Promise<void> {
		await invoke('update_local_progress', {
			params: {
				media_id: mediaId,
				progress,
				timestamp: timestamp ?? undefined,
			} satisfies UpdateProgressParams,
		});
	}

	/**
	 * Cache media data via Tauri command
	 */
	static async cacheMedia(mediaData: Media, extensionSource?: string): Promise<void> {
		await invoke('cache_media', {
			params: {
				media_data: mediaData,
				extension_source: extensionSource ?? undefined,
			} satisfies CacheMediaParams,
		});
	}

	/**
	 * Cache user data via Tauri command
	 */
	static async cacheUser(userData: User): Promise<void> {
		await invoke('cache_user', {
			params: {
				user_data: userData,
			} satisfies CacheUserParams,
		});
	}

	/**
	 * Add media to recently viewed via Tauri command
	 */
	static async addToRecentlyViewed(media_id: number): Promise<void> {
		await invoke('add_to_recently_viewed', { 
			params: { media_id } satisfies RecentlyViewedParams 
		});
	}

	/**
	 * Get recently viewed media via Tauri command
	 */
	static async getRecentlyViewed(limit = 10): Promise<Media[]> {
		const result = await invoke('get_recently_viewed', { limit });
		return result as Media[];
	}

	/**
	 * Search cached media via Tauri command
	 */
	static async searchCachedMedia(query: string, mediaType?: 'ANIME' | 'MANGA'): Promise<Media[]> {
		const result = await invoke('search_cached_media', {
			query,
			mediaType: mediaType || null,
		});
		return result as Media[];
	}

	/**
	 * Cleanup expired cache entries via Tauri command
	 */
	static async cleanupCache(): Promise<void> {
		await invoke('cleanup_cache');
	}

	/**
	 * Get all cached images via Tauri command
	 */
	static async getAllCachedImages(): Promise<CachedImageInfo[]> {
		const result = await invoke('get_all_cached_images');
		return result as CachedImageInfo[];
	}

	/**
	 * Get cached image path via Tauri command
	 */
	static async getCachedImagePath(url: string): Promise<string | null> {
		const result = await invoke('get_cached_image_path', { url });
		return result as string | null;
	}

	/**
	 * Cache image via Tauri command
	 */
	static async cacheImage(url: string, localPath: string): Promise<number> {
		const result = await invoke('cache_image', { url, localPath });
		return result as number;
	}

	/**
	 * Remove cached image via Tauri command
	 */
	static async removeCachedImage(url: string): Promise<void> {
		await invoke('remove_cached_image', { url });
	}
}
