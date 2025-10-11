/**
 * Image caching service for offline storage
 * Downloads and manages local copies of images for offline viewing
 */

import { ClientDatabaseService } from './client-database';
import { invoke } from '@tauri-apps/api/core';

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
	 * Get cached image path or download if not cached
	 */
	static async getCachedImage(
		url: string, 
		options: ImageCacheOptions = {}
	): Promise<string | null> {
		if (!url) return null;

		try {
			// Check if image is already cached
			const cachedPath = await ClientDatabaseService.getCachedImagePath(url);
			
			if (cachedPath && !options.forceRefresh) {
				// Verify file still exists
				const exists = await this.fileExists(cachedPath);
				if (exists) {
					return cachedPath;
				} else {
					// File was deleted, remove from cache
					await this.removeCachedImage(url);
				}
			}

			// Download and cache the image
			return await this.downloadAndCache(url, options);
		} catch (error) {
			console.error('Failed to get cached image:', error);
			return null;
		}
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

			// Download image using Tauri
			const success = await invoke<boolean>('download_image', {
				url,
				localPath,
				quality: options.quality || 'large'
			});

			if (!success) {
				throw new Error('Failed to download image');
			}

			// Store in database
			await ClientDatabaseService.cacheImage(url, localPath);

			return localPath;
		} catch (error) {
			console.error('Failed to download and cache image:', error);
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
			hash = ((hash << 5) - hash) + char;
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
	 * Remove image from cache
	 */
	static async removeCachedImage(url: string): Promise<void> {
		try {
			const cachedPath = await ClientDatabaseService.getCachedImagePath(url);
			if (cachedPath) {
				// Delete file
				await invoke('delete_file', { path: cachedPath });
				
				// Remove from database
				await ClientDatabaseService.removeCachedImage(url);
			}
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
		coverImage?: { large?: string; medium?: string; };
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
		}
	};
}