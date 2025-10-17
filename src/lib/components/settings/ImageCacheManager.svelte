<!--
  Image Cache Management Component
  Provides UI for managing cached images, cleanup, and statistics
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { ImageCacheService } from '$lib/services/imageCache';
	import { ClientDatabaseService, type CachedImageInfo } from '$lib/services/client-database';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Trash2, RefreshCw, HardDrive, Image, Database } from 'lucide-svelte';

	// Props - receive initial data from page load
	let { initialCachedImages = [] }: { initialCachedImages?: CachedImageInfo[] } = $props();

	let cachedImages: CachedImageInfo[] = $state(initialCachedImages);

	let isLoading = $state(false);
	let isCleaningUp = $state(false);

	// Calculate cache stats from cached images
	let cacheStats = $derived({
totalImages: cachedImages.length,
totalSize: cachedImages.reduce((sum, img) => sum + (img.file_size || 0), 0),
		oldestImage: cachedImages.length > 0
			? Math.min(...cachedImages.map(img => img.cached_at * 1000)) // Convert to milliseconds
			: 0,
	});

	async function loadCacheStats() {
		try {
			isLoading = true;
			// Reload cached images from database via Tauri command
			const allImages = await ClientDatabaseService.getAllCachedImages();
			cachedImages = allImages;
			console.log('[ImageCache] ✓ Loaded', cachedImages.length, 'cached images');
		} catch (error) {
			console.error('[ImageCache] Failed to load stats:', error);
		} finally {
			isLoading = false;
		}
	}

	async function cleanupCache() {
		try {
			isCleaningUp = true;
			await ImageCacheService.cleanupCache();
			console.log('[ImageCache] ✓ Cleanup completed');
			// Reload stats after cleanup
			await loadCacheStats();
		} catch (error) {
			console.error('[ImageCache] Failed to cleanup:', error);
		} finally {
			isCleaningUp = false;
		}
	}

	async function clearLocalStorage() {
		try {
			localStorage.clear();
			console.log('[Storage] ✓ Local storage cleared');
			alert('Local storage cleared successfully!');
		} catch (error) {
			console.error('[Storage] Failed to clear local storage:', error);
			alert('Failed to clear local storage');
		}
	}

	async function clearSessionStorage() {
		try {
			sessionStorage.clear();
			console.log('[Storage] ✓ Session storage cleared');
			alert('Session storage cleared successfully! Please reload the page.');
		} catch (error) {
			console.error('[Storage] Failed to clear session storage:', error);
			alert('Failed to clear session storage');
		}
	}

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 Bytes';

		const k = 1024;
		const sizes = ['Bytes', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));

		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function formatDate(timestamp: number): string {
		if (timestamp === 0) return 'N/A';
		// Timestamp is already in milliseconds from JavaScript Date
		return new Date(timestamp).toLocaleDateString();
	}

	onMount(() => {
		loadCacheStats();
	});
</script>

<Card>
	<CardHeader>
		<CardTitle class="flex items-center gap-2">
			<Image class="w-5 h-5" />
			Cache & Storage Management
		</CardTitle>
		<CardDescription>
			Manage cached images and browser storage. Images are automatically downloaded and stored locally for faster loading.
		</CardDescription>
	</CardHeader>
	<CardContent class="space-y-6">
		<!-- Image Cache Statistics -->
		<div>
			<h4 class="font-medium mb-4 flex items-center gap-2">
				<Image class="w-4 h-4" />
				Image Cache
			</h4>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<div class="flex items-center space-x-2">
					<Image class="w-4 h-4 text-muted-foreground" />
					<div class="space-y-1">
						<p class="text-sm font-medium">Total Images</p>
						<p class="text-2xl font-bold">{cacheStats.totalImages}</p>
					</div>
				</div>

				<div class="flex items-center space-x-2">
					<HardDrive class="w-4 h-4 text-muted-foreground" />
					<div class="space-y-1">
						<p class="text-sm font-medium">Cache Size</p>
						<p class="text-2xl font-bold">{formatBytes(cacheStats.totalSize)}</p>
					</div>
				</div>

				<div class="space-y-1">
					<p class="text-sm font-medium">Oldest Image</p>
					<p class="text-2xl font-bold">{formatDate(cacheStats.oldestImage)}</p>
				</div>
			</div>
		</div>

		<Separator />

		<!-- Image Cache Actions -->
		<div>
			<h4 class="font-medium mb-3">Image Cache Actions</h4>
			<div class="flex flex-wrap gap-2">
				<Button
					variant="outline"
					onclick={loadCacheStats}
					disabled={isLoading}
					class="flex items-center gap-2"
				>
					<RefreshCw class="w-4 h-4 {isLoading ? 'animate-spin' : ''}" />
					Refresh Stats
				</Button>

				<Button
					variant="destructive"
					onclick={cleanupCache}
					disabled={isCleaningUp || cacheStats.totalImages === 0}
					class="flex items-center gap-2"
				>
					<Trash2 class="w-4 h-4" />
					{isCleaningUp ? 'Cleaning...' : 'Cleanup Old Images'}
				</Button>
			</div>
		</div>

		<Separator />

		<!-- Browser Storage Actions -->
		<div>
			<h4 class="font-medium mb-3 flex items-center gap-2">
				<Database class="w-4 h-4" />
				Browser Storage
			</h4>
			<p class="text-sm text-muted-foreground mb-3">
				Clear browser storage caches. This will remove all cached authentication, config, and theme data. You may need to reload the page after clearing.
			</p>
			<div class="flex flex-wrap gap-2">
				<Button
					variant="outline"
					onclick={clearSessionStorage}
					class="flex items-center gap-2"
				>
					<Trash2 class="w-4 h-4" />
					Clear Session Storage
				</Button>

				<Button
					variant="outline"
					onclick={clearLocalStorage}
					class="flex items-center gap-2"
				>
					<Trash2 class="w-4 h-4" />
					Clear Local Storage
				</Button>
			</div>
		</div>

		{#if cacheStats.totalImages === 0 && !isLoading}
			<Separator />
			<div class="text-center py-8">
				<Image class="w-12 h-12 mx-auto text-muted-foreground mb-2" />
				<p class="text-muted-foreground">No cached images found</p>
				<p class="text-sm text-muted-foreground">Images will be cached automatically as you browse</p>
			</div>
		{/if}
	</CardContent>
</Card>
