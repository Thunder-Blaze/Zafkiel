<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Media } from '$lib/types/anilist';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	let {
		items = [],
		autoplayInterval = 5000,
	}: {
		items: Media[];
		autoplayInterval?: number;
	} = $props();

	let currentIndex = $state(0);
	let isPaused = $state(false);
	let autoplayTimer: ReturnType<typeof setInterval> | null = null;
	let direction = $state<'left' | 'right'>('right');

	const currentItem = $derived(items[currentIndex]);

	function goToSlide(index: number) {
		direction = index > currentIndex ? 'right' : 'left';
		currentIndex = index;
		resetAutoplay();
	}

	function nextSlide() {
		direction = 'right';
		currentIndex = (currentIndex + 1) % items.length;
		resetAutoplay();
	}

	function prevSlide() {
		direction = 'left';
		currentIndex = (currentIndex - 1 + items.length) % items.length;
		resetAutoplay();
	}

	function startAutoplay() {
		if (autoplayTimer) clearInterval(autoplayTimer);
		autoplayTimer = setInterval(() => {
			if (!isPaused) {
				currentIndex = (currentIndex + 1) % items.length;
			}
		}, autoplayInterval);
	}

	function resetAutoplay() {
		startAutoplay();
	}

	function handleMouseEnter() {
		isPaused = true;
	}

	function handleMouseLeave() {
		isPaused = false;
	}

	$effect(() => {
		if (items.length > 1) {
			startAutoplay();
		}
		return () => {
			if (autoplayTimer) clearInterval(autoplayTimer);
		};
	});

	function formatDescription(desc: string | null | undefined): string {
		if (!desc) return '';
		// Remove HTML tags and limit to ~200 characters
		const text = desc.replace(/<[^>]*>/g, '').replace(/\n/g, ' ');
		return text.length > 200 ? text.substring(0, 200) + '...' : text;
	}

	function handleViewDetails() {
		if (currentItem) {
			goto(`/${currentItem.type?.toLowerCase() || 'anime'}/${currentItem.id}`);
		}
	}
</script>

{#if items.length > 0 && currentItem}
	<div
		class="group relative h-[350px] w-full overflow-hidden"
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
	>
		<!-- Background Image with Gradient Overlay -->
		{#key currentItem.id}
			<div
				in:fade={{ duration: 600, easing: cubicOut }}
				out:fade={{ duration: 400, easing: cubicOut }}
				class="absolute inset-0 bg-cover bg-center"
				style="background-image: url('{currentItem.bannerImage || currentItem.coverImage?.extraLarge || currentItem.coverImage?.large}')"
			>
				<!-- Gradient Overlays -->
				<div class="absolute inset-0 bg-gradient-to-r from-background/70 via-background/40 to-transparent"></div>
				<div class="absolute inset-0 bg-gradient-to-t from-background/80 via-transparent to-transparent"></div>
			</div>
		{/key}

		<!-- Content -->
		<div class="relative flex h-full flex-col justify-end px-6 pb-6 md:px-8 lg:px-12">
			{#key currentItem.id}
				<div 
					in:fly={{ x: direction === 'right' ? 100 : -100, y: 0, duration: 500, delay: 200, easing: cubicOut }}
					out:fly={{ x: direction === 'right' ? -50 : 50, y: 0, duration: 300, easing: cubicOut }}
					class="absolute bottom-6 left-6 right-6 max-w-xl space-y-2 md:left-8 md:right-8 lg:left-12 lg:right-12"
				>
					<!-- Title -->
					<h2 class="line-clamp-2 text-2xl font-bold leading-tight drop-shadow-lg md:text-3xl">
						{currentItem.title?.english || currentItem.title?.romaji || currentItem.title?.native}
					</h2>

					<!-- Metadata -->
					<div class="flex flex-wrap items-center gap-2 text-xs">
						{#if currentItem.format}
							<span class="rounded-full bg-primary px-2 py-0.5 text-xs font-semibold text-primary-foreground">
								{currentItem.format.replace(/_/g, ' ')}
							</span>
						{/if}
						{#if currentItem.seasonYear}
							<span class="font-medium text-muted-foreground">{currentItem.seasonYear}</span>
						{/if}
						{#if currentItem.averageScore}
							<div class="flex items-center gap-1">
								<Icon icon="solar:star-bold" class="h-3 w-3 text-yellow-500" />
								<span class="font-medium">{(currentItem.averageScore / 10).toFixed(1)}</span>
							</div>
						{/if}
						{#if currentItem.episodes}
							<span class="text-muted-foreground">{currentItem.episodes} Episodes</span>
						{/if}
					</div>

					<!-- Genres -->
					{#if currentItem.genres && currentItem.genres.length > 0}
						<div class="flex flex-wrap gap-1.5">
							{#each currentItem.genres.slice(0, 5) as genre}
								<span class="rounded border border-border/50 bg-background/50 px-1.5 py-0.5 text-[10px] font-medium backdrop-blur-sm">{genre}</span>
							{/each}
						</div>
					{/if}

					<!-- Description -->
					{#if currentItem.description}
						<p class="line-clamp-2 text-xs leading-relaxed text-muted-foreground">
							{formatDescription(currentItem.description)}
						</p>
					{/if}

					<!-- Actions -->
					<div class="flex items-center gap-2 pt-1">
						<Button size="sm" onclick={handleViewDetails}>
							<Icon icon="solar:play-bold" class="mr-1.5 h-4 w-4" />
							View Details
						</Button>
						<Button variant="outline" size="sm">
							<Icon icon="solar:add-circle-bold" class="mr-1.5 h-4 w-4" />
							Add to List
						</Button>
					</div>
				</div>
			{/key}
		</div>

		<!-- Navigation Arrows -->
		{#if items.length > 1}
			<button
				onclick={prevSlide}
				class="absolute left-4 top-1/2 -translate-y-1/2 rounded-full bg-background/50 p-3 opacity-0 backdrop-blur-sm transition-opacity hover:bg-background/70 group-hover:opacity-100"
			>
				<Icon icon="solar:alt-arrow-left-bold" class="h-6 w-6" />
			</button>

			<button
				onclick={nextSlide}
				class="absolute right-4 top-1/2 -translate-y-1/2 rounded-full bg-background/50 p-3 opacity-0 backdrop-blur-sm transition-opacity hover:bg-background/70 group-hover:opacity-100"
			>
				<Icon icon="solar:alt-arrow-right-bold" class="h-6 w-6" />
			</button>
		{/if}

		<!-- Dot Indicators -->
		{#if items.length > 1}
			<div class="absolute bottom-4 left-1/2 flex -translate-x-1/2 gap-2">
				{#each items as _, index}
					<button
						onclick={() => goToSlide(index)}
						class="h-2 rounded-full transition-all {index === currentIndex
							? 'w-8 bg-primary'
							: 'w-2 bg-muted-foreground/50 hover:bg-muted-foreground'}"
						aria-label="Go to slide {index + 1}"
					></button>
				{/each}
			</div>
		{/if}
	</div>
{/if}
