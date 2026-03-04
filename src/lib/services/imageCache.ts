/**
 * Image caching service for offline storage
 * Downloads and manages local copies of images for offline viewing
 */

import { ClientDatabaseService } from './client-database';
import { invoke } from '@tauri-apps/api/core';

/**
 * Session-level cache: URL → resolved local path (or null for failures).
 * Eliminates repeated Tauri IPC round-trips for URLs already resolved this session.
 */
const sessionPathCache = new Map<string, string | null>();

/**
 * In-flight request deduplication: URL → pending Promise.
 * If two components mount simultaneously with the same URL, only one IPC call is made.
 */
const pendingRequests = new Map<string, Promise<string | null>>();

export interface ImageCacheOptions {
	quality?: 'original' | 'large' | 'medium' | 'small';
	maxAge?: number; // in days
	forceRefresh?: boolean;
}

export interface CachedImageInfo {
	id: number;
	originalUrl: string;
	localPath: string;
	lastAccessed: number;
	fileSize?: number;
}

export class ImageCacheService {
	private static readonly CACHE_DIR = 'images';
	private static readonly DEFAULT_MAX_AGE = 30; // 30 days

	/**
	 * Get cached image path or download if not cached.
	 * Results are memoised in a session-level Map so the same URL never triggers
	 * more than one Tauri IPC call per page session.
	 */
	static async getCachedImage(
		url: string,
		options: ImageCacheOptions = {}
	): Promise<string | null> {
		if (!url) return null;

		// 1. Instant session-cache hit (no IPC at all)
		if (!options.forceRefresh && sessionPathCache.has(url)) {
			return sessionPathCache.get(url) ?? null;
		}

		// 2. Deduplicate concurrent requests for the same URL
		if (!options.forceRefresh && pendingRequests.has(url)) {
			return pendingRequests.get(url)!;
		}

		// 3. Start a new request and register it as in-flight
		const request = this._resolveImage(url, options).then((result) => {
			sessionPathCache.set(url, result);
			pendingRequests.delete(url);
			return result;
		});

		pendingRequests.set(url, request);
		return request;
	}

	/** Internal resolver – performs the actual Tauri IPC work. */
	private static async _resolveImage(
		url: string,
		options: ImageCacheOptions
	): Promise<string | null> {
		try {
			console.log('[ImageCache] Checking cache for:', url);
			const cachedPath = await ClientDatabaseService.getCachedImagePath(url);

			if (cachedPath && !options.forceRefresh) {
				console.log('[ImageCache] Found in cache:', cachedPath);
				const exists = await this.fileExists(cachedPath);
				if (exists) {
					console.log('[ImageCache] File exists, using cached version');
					return cachedPath;
				} else {
					console.warn('[ImageCache] Cached file missing, will re-download');
					await this.removeCachedImage(url);
				}
			} else {
				console.log('[ImageCache] Not in cache, will download');
			}

			return await this.downloadAndCache(url, options);
		} catch (error) {
			console.error('[ImageCache] Failed to get cached image:', error);
			return null;
		}
	}

	/** Invalidate a single URL from the session cache (e.g. after forced refresh). */
	static invalidateSessionCache(url: string): void {
		sessionPathCache.delete(url);
		pendingRequests.delete(url);
	}

	/** Clear the entire session cache (e.g. on logout or settings change). */
	static clearSessionCache(): void {
		sessionPathCache.clear();
		pendingRequests.clear();
	}

	/**
	 * Download image and store in cache
	 */
	private static async downloadAndCache(
		url: string,
		options: ImageCacheOptions
	): Promise<string | null> {
		try {
			// Generate unique filename
			const filename = this.generateFilename(url, options.quality);
			const localPath = `${this.CACHE_DIR}/${filename}`;

			console.log('[ImageCache] Downloading:', url, '→', localPath);

			// Download image using Tauri
			const success = await invoke<boolean>('download_image', {
				url,
				localPath,
				quality: options.quality || 'large',
			});

			if (!success) {
				throw new Error('Failed to download image');
			}

			console.log('[ImageCache] Download successful, storing in database...');

			// Store in database
			try {
				const id = await ClientDatabaseService.cacheImage(url, localPath);
				console.log('[ImageCache] Stored in database with ID:', id);
			} catch (dbError) {
				console.error('[ImageCache] Failed to store in database:', dbError);
				throw dbError;
			}

			return localPath;
		} catch (error) {
			console.error('[ImageCache] Failed to download and cache image:', error);
			return null;
		}
	}

	/**
	 * Generate unique filename for cached image
	 */
	private static generateFilename(url: string, quality?: string): string {
		// Create hash from URL
		const hash = this.simpleHash(url);
		const extension = this.getFileExtension(url) || 'jpg';
		const qualitySuffix = quality && quality !== 'original' ? `_${quality}` : '';

		return `${hash}${qualitySuffix}.${extension}`;
	}

	/**
	 * Simple hash function for URLs
	 */
	private static simpleHash(str: string): string {
		let hash = 0;
		for (let i = 0; i < str.length; i++) {
			const char = str.charCodeAt(i);
			hash = (hash << 5) - hash + char;
			hash = hash & hash; // Convert to 32-bit integer
		}
		return Math.abs(hash).toString(36);
	}

	/**
	 * Extract file extension from URL
	 */
	private static getFileExtension(url: string): string | null {
		try {
			const pathname = new URL(url).pathname;
			const match = pathname.match(/\.([a-zA-Z0-9]+)$/);
			return match ? match[1].toLowerCase() : null;
		} catch {
			return null;
		}
	}

	/**
	 * Check if file exists locally
	 */
	private static async fileExists(path: string): Promise<boolean> {
		try {
			return await invoke<boolean>('file_exists', { path });
		} catch {
			return false;
		}
	}

	/**
	 * Remove image from cache (also invalidates the session cache entry).
	 */
	static async removeCachedImage(url: string): Promise<void> {
		try {
			const cachedPath = await ClientDatabaseService.getCachedImagePath(url);
			if (cachedPath) {
				await invoke('delete_file', { path: cachedPath });
				await ClientDatabaseService.removeCachedImage(url);
			}
			// Always clear from session cache so the next access re-resolves
			this.invalidateSessionCache(url);
		} catch (error) {
			console.error('Failed to remove cached image:', error);
		}
	}

	/**
	 * Cleanup old cached images
	 */
	static async cleanupCache(maxAge: number = this.DEFAULT_MAX_AGE): Promise<void> {
		try {
			await invoke('cleanup_image_cache', { maxAgeDays: maxAge });
			await ClientDatabaseService.cleanupCache();
		} catch (error) {
			console.error('Failed to cleanup image cache:', error);
		}
	}

	/**
	 * Get cache statistics
	 */
	static async getCacheStats(): Promise<{
		totalImages: number;
		totalSize: number;
		oldestImage: number;
	}> {
		try {
			return await invoke('get_cache_stats');
		} catch (error) {
			console.error('Failed to get cache stats:', error);
			return {
				totalImages: 0,
				totalSize: 0,
				oldestImage: 0,
			};
		}
	}

	/**
	 * Preload images for media
	 */
	static async preloadMediaImages(media: {
		bannerImage?: string;
		coverImage?: { large?: string; medium?: string };
	}): Promise<void> {
		const promises: Promise<string | null>[] = [];

		if (media.bannerImage) {
			promises.push(this.getCachedImage(media.bannerImage, { quality: 'large' }));
		}

		if (media.coverImage?.large) {
			promises.push(this.getCachedImage(media.coverImage.large, { quality: 'large' }));
		}

		if (media.coverImage?.medium) {
			promises.push(this.getCachedImage(media.coverImage.medium, { quality: 'medium' }));
		}

		try {
			await Promise.all(promises);
		} catch (error) {
			console.error('Failed to preload media images:', error);
		}
	}
}

/**
 * Hook for image caching with TanStack Query
 */
export function useCachedImage(url: string | null | undefined, options: ImageCacheOptions = {}) {
	return {
		getCachedUrl: async () => {
			if (!url) return null;
			return await ImageCacheService.getCachedImage(url, options);
		},
		preload: async () => {
			if (!url) return;
			await ImageCacheService.getCachedImage(url, options);
		},
	};
}
