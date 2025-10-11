<!--
  Image Cache Management Component
  Provides UI for managing cached images, cleanup, and statistics
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { ImageCacheService } from '$lib/services/imageCache';
	import { ClientDatabaseService } from '$lib/services/client-database';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Trash2, RefreshCw, HardDrive, Image } from 'lucide-svelte';

	interface CacheStats {
		totalImages: number;
		totalSize: number;
		oldestImage: number;
	}

	let cacheStats: CacheStats = {
		totalImages: 0,
		totalSize: 0,
		oldestImage: 0,
	};

	let cachedImages: Array<{
		id: number;
		original_url: string;
		local_path: string;
		last_accessed: number;
	}> = [];

	let isLoading = false;
	let isCleaningUp = false;

	async function loadCacheStats() {
		try {
			isLoading = true;
			cacheStats = await ImageCacheService.getCacheStats();
			cachedImages = await ClientDatabaseService.getAllCachedImages();
		} catch (error) {
			console.error('Failed to load cache stats:', error);
		} finally {
			isLoading = false;
		}
	}

	async function cleanupCache() {
		try {
			isCleaningUp = true;
			await ImageCacheService.cleanupCache();
			await loadCacheStats(); // Reload stats after cleanup
		} catch (error) {
			console.error('Failed to cleanup cache:', error);
		} finally {
			isCleaningUp = false;
		}
	}

	async function removeImage(url: string) {
		try {
			await ImageCacheService.removeCachedImage(url);
			await loadCacheStats(); // Reload stats after removal
		} catch (error) {
			console.error('Failed to remove image:', error);
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
		if (timestamp === 0) return 'Never';
		return new Date(timestamp * 1000).toLocaleDateString();
	}

	function getHostname(url: string): string {
		try {
			return new URL(url).hostname;
		} catch {
			return 'Unknown';
		}
	}

	onMount(() => {
		loadCacheStats();
	});
</script>

<Card>
	<CardHeader>
		<CardTitle class="flex items-center gap-2">
			<Image class="w-5 h-5" />
			Image Cache Management
		</CardTitle>
		<CardDescription>
			Manage cached images for offline viewing. Images are automatically downloaded and stored locally for faster loading.
		</CardDescription>
	</CardHeader>
	<CardContent class="space-y-6">
		<!-- Cache Statistics -->
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

		<Separator />

		<!-- Actions -->
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

		{#if cachedImages.length > 0}
			<Separator />

			<!-- Cached Images List -->
			<div class="space-y-2">
				<h4 class="font-medium">Cached Images ({cachedImages.length})</h4>
				<div class="max-h-64 overflow-y-auto space-y-2">
					{#each cachedImages.slice(0, 20) as image}
						<div class="flex items-center justify-between p-3 border rounded-lg">
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2 mb-1">
									<Badge variant="secondary" class="text-xs">
										{getHostname(image.original_url)}
									</Badge>
									<span class="text-xs text-muted-foreground">
										{formatDate(image.last_accessed)}
									</span>
								</div>
								<p class="text-sm font-mono truncate" title={image.local_path}>
									{image.local_path}
								</p>
							</div>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => removeImage(image.original_url)}
								class="ml-2"
							>
								<Trash2 class="w-4 h-4" />
							</Button>
						</div>
					{/each}

					{#if cachedImages.length > 20}
						<p class="text-sm text-muted-foreground text-center">
							... and {cachedImages.length - 20} more images
						</p>
					{/if}
				</div>
			</div>
		{/if}

		{#if cacheStats.totalImages === 0 && !isLoading}
			<div class="text-center py-8">
				<Image class="w-12 h-12 mx-auto text-muted-foreground mb-2" />
				<p class="text-muted-foreground">No cached images found</p>
				<p class="text-sm text-muted-foreground">Images will be cached automatically as you browse</p>
			</div>
		{/if}
	</CardContent>
</Card>
