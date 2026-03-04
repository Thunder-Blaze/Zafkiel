<!--
  Cached Image Component
  Automatically handles image caching and displays cached/fallback images
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { ImageCacheService, type ImageCacheOptions } from '$lib/services/imageCache';

	export let src: string | null | undefined;
	export let alt: string = '';
	export let placeholder: string = '/placeholder.png';
	export let cacheOptions: ImageCacheOptions = {};
	export let preload: boolean = false;

	// Standard img attributes
	export let width: string | number | undefined = undefined;
	export let height: string | number | undefined = undefined;
	export let loading: 'lazy' | 'eager' = 'lazy';

	let displaySrc = placeholder;
	let isLoading = false;
	let hasError = false;
	let imageElement: HTMLImageElement;

	async function loadImage() {
		if (!src) {
			displaySrc = placeholder;
			return;
		}

		try {
			isLoading = true;
			hasError = false;

			// Try to get cached image first
			const cachedPath = await ImageCacheService.getCachedImage(src, cacheOptions);

			if (cachedPath) {
				// Use cached image
				displaySrc = `asset://localhost/${cachedPath}`;
			} else {
				// Fallback to original URL
				displaySrc = src;
			}
		} catch (error) {
			console.error('Failed to load cached image:', error);
			displaySrc = src || placeholder;
			hasError = true;
		} finally {
			isLoading = false;
		}
	}

	function handleError() {
		hasError = true;
		if (displaySrc !== placeholder && displaySrc !== src) {
			// If cached image failed, try original
			displaySrc = src || placeholder;
		} else if (displaySrc !== placeholder) {
			// If original failed, use placeholder
			displaySrc = placeholder;
		}
	}

	function handleLoad() {
		hasError = false;

		// If we successfully loaded the original image, cache it in background
		if (displaySrc === src && src) {
			ImageCacheService.getCachedImage(src, cacheOptions).catch((err) => {
				console.warn('Background caching failed:', err);
			});
		}
	}

	// Preload image if requested
	async function preloadImage() {
		if (preload && src) {
			try {
				await ImageCacheService.getCachedImage(src, cacheOptions);
			} catch (error) {
				console.warn('Preload failed:', error);
			}
		}
	}

	// React to src changes
	$: if (src) {
		loadImage();
	}

	onMount(() => {
		preloadImage();
	});
</script>

<img
	bind:this={imageElement}
	src={displaySrc}
	{alt}
	{width}
	{height}
	{loading}
	class="transition-opacity duration-200 {isLoading ? 'opacity-50' : 'opacity-100'} {hasError
		? 'grayscale filter'
		: ''} {$$props.class || ''}"
	on:load={handleLoad}
	on:error={handleError}
	{...$$restProps}
/>

<style>
	img {
		/* Ensure smooth transitions */
		transition:
			opacity 0.2s ease-in-out,
			filter 0.2s ease-in-out;
	}
</style>
