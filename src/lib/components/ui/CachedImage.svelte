<!--
  Cached Image Component
  Automatically handles image caching and displays cached/fallback images.
  Results are memoised in a session-level Map – the same URL never triggers
  more than one Tauri IPC call per page session.
-->
<script lang="ts">
	import { ImageCacheService, type ImageCacheOptions } from '$lib/services/imageCache';
	import { convertFileSrc, invoke } from '@tauri-apps/api/core';

	interface Props {
		src?: string | null;
		alt?: string;
		/** Shown while src loads or on error (after fallbackSrc). */
		placeholder?: string;
		/** Secondary image attempted before falling back to placeholder. */
		fallbackSrc?: string | null;
		cacheOptions?: ImageCacheOptions;
		preload?: boolean;
		width?: string | number;
		height?: string | number;
		loading?: 'lazy' | 'eager';
		class?: string;
		[key: string]: unknown;
	}

	let {
		src = null,
		alt = '',
		placeholder = '/placeholder.png',
		fallbackSrc = null,
		cacheOptions = {},
		preload = false,
		width = undefined,
		height = undefined,
		loading = 'lazy',
		class: className = '',
		...rest
	}: Props = $props();

	let displaySrc = $state('');
	let isLoading = $state(false);
	let hasError = $state(false);
	let imageElement: HTMLImageElement | null = $state(null);
	/** The resolved local asset path, kept separately so handleError can distinguish stages. */
	let cachedAssetSrc = $state<string | null>(null);

	$effect(() => {
		// Reset per-src state
		cachedAssetSrc = null;
		hasError = false;

		if (!src) {
			displaySrc = placeholder;
			isLoading = false;
			return;
		}

		// ① Show the CDN URL immediately — no flash of blank / opacity-50
		displaySrc = src;
		isLoading = true;

		ImageCacheService.getCachedImage(src, cacheOptions)
			.then(async (path) => {
				if (path) {
					// Resolve relative cache path to absolute via Tauri backend
					const absolutePath = await invoke<string>('get_cached_file_path', { relativePath: path });
					cachedAssetSrc = convertFileSrc(absolutePath);
					displaySrc = cachedAssetSrc;
				}
				// If no cached path, displaySrc stays as the CDN URL (already set above)
			})
			.catch((err) => {
				console.error('[CachedImage] Failed to resolve cached path:', err);
			})
			.finally(() => {
				isLoading = false;
			});

		// Pre-warm the fallback so it's ready if the primary fails
		if (preload && fallbackSrc) {
			ImageCacheService.getCachedImage(fallbackSrc, cacheOptions).catch(() => {});
		}
	});

	function handleError() {
		hasError = true;
		if (cachedAssetSrc && displaySrc === cachedAssetSrc) {
			// Cached local file is missing/corrupt → fall back to original CDN URL
			displaySrc = src ?? placeholder;
		} else if (fallbackSrc && displaySrc !== fallbackSrc) {
			// Primary URL failed → try fallbackSrc
			displaySrc = fallbackSrc;
		} else if (displaySrc !== placeholder) {
			// Everything failed → placeholder
			displaySrc = placeholder;
		}
	}

	function handleLoad() {
		hasError = false;
		isLoading = false;
	}
</script>

<img
	bind:this={imageElement}
	src={displaySrc}
	{alt}
	{width}
	{height}
	{loading}
	class="transition-opacity duration-200 {hasError ? 'opacity-60 grayscale filter' : 'opacity-100'} {className}"
	onload={handleLoad}
	onerror={handleError}
	{...rest}
/>

<style>
	img {
		transition:
			opacity 0.2s ease-in-out,
			filter 0.2s ease-in-out;
	}
</style>
