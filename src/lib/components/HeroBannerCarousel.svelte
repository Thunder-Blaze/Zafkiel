<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Media } from '$lib/types/anilist';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { useConfigState } from '$lib/stores/config.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';

	let {
		items = [],
		autoplayInterval = 5000,
	}: {
		items: Media[];
		autoplayInterval?: number;
	} = $props();

	const config = useConfigState();
	const animationsEnabled = $derived(config.animations);
	const bgDuration = $derived(animationsEnabled ? 700 : 120);
	const contentDuration = $derived(animationsEnabled ? 500 : 120);
	const contentDelay = $derived(animationsEnabled ? 180 : 0);

	let currentIndex = $state(0);
	let isPaused = $state(false);
	let autoplayTimer: ReturnType<typeof setInterval> | null = null;
	let progressTimer: ReturnType<typeof setInterval> | null = null;
	let direction = $state<'left' | 'right'>('right');
	let progress = $state(0);
	const tickStep = $derived(100 / (autoplayInterval / 80));

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
		if (progressTimer) clearInterval(progressTimer);
		progress = 0;

		progressTimer = setInterval(() => {
			if (!isPaused) progress = Math.min(100, progress + tickStep);
		}, 80);

		autoplayTimer = setInterval(() => {
			if (!isPaused) {
				direction = 'right';
				currentIndex = (currentIndex + 1) % items.length;
				progress = 0;
			}
		}, autoplayInterval);
	}

	function resetAutoplay() {
		progress = 0;
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
			if (progressTimer) clearInterval(progressTimer);
		};
	});

	function formatDescription(desc: string | null | undefined): string {
		if (!desc) return '';
		const text = desc.replace(/<[^>]*>/g, '').replace(/\n/g, ' ');
		return text.length > 220 ? text.substring(0, 220) + '…' : text;
	}

	function handleViewDetails() {
		if (currentItem) {
			goto(`/${currentItem.type?.toLowerCase() || 'anime'}/${currentItem.id}`);
		}
	}
</script>

{#if items.length > 0 && currentItem}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="group relative h-[420px] w-full overflow-hidden"
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
	>
		<!-- Background image cross-fade -->
		{#key currentItem.id}
			<div
				in:fade={{ duration: bgDuration, easing: cubicOut }}
				out:fade={{ duration: Math.floor(bgDuration * 0.6), easing: cubicOut }}
				class="absolute inset-0 bg-cover bg-center"
				style="background-image: url('{currentItem.bannerImage ||
					currentItem.coverImage?.extraLarge ||
					currentItem.coverImage?.large}')"
			>
				<div
					class="absolute inset-0 bg-linear-to-r from-background/90 via-background/55 to-background/10"
				></div>
				<div
					class="absolute inset-0 bg-linear-to-t from-background/80 via-transparent to-transparent"
				></div>
			</div>
		{/key}

		<!-- Content grid: info left | cover art right -->
		<div class="relative grid h-full grid-cols-[1fr_auto] items-end gap-4 px-8 pb-10 lg:px-14">
			<!-- Left: text info -->
			<div class="relative grid min-w-0">
				{#key currentItem.id}
					<div
						style="grid-area: 1/1"
						in:fly={{
							x: direction === 'right' ? 50 : -50,
							y: 0,
							duration: contentDuration,
							delay: contentDelay,
							easing: cubicOut,
						}}
						out:fly={{
							x: direction === 'right' ? -30 : 30,
							y: 0,
							duration: Math.floor(contentDuration * 0.6),
							easing: cubicOut,
						}}
						class="flex max-w-[760px] flex-col gap-2.5"
					>
						<!-- Season badge -->
						{#if currentItem.season && currentItem.seasonYear}
							<p class="text-[11px] font-bold tracking-widest text-primary uppercase">
								{currentItem.season}
								{currentItem.seasonYear}
							</p>
						{/if}

						<!-- Title -->
						<h2 class="line-clamp-1 text-3xl leading-tight font-bold drop-shadow-lg md:text-4xl">
							{currentItem.title?.english || currentItem.title?.romaji || currentItem.title?.native}
						</h2>

						<!-- Score · format · episodes -->
						<div class="flex flex-wrap items-center gap-2 text-sm">
							{#if currentItem.averageScore}
								<span class="flex items-center gap-1 font-semibold">
									<Icon icon="solar:star-bold" class="h-3.5 w-3.5 text-yellow-400" />
									{(currentItem.averageScore / 10).toFixed(1)}
								</span>
							{/if}
							{#if currentItem.format}
								<span class="rounded bg-foreground/10 px-2 py-0.5 text-xs font-medium"
									>{currentItem.format.replace(/_/g, ' ')}</span
								>
							{/if}
							{#if currentItem.episodes}
								<span class="text-xs text-muted-foreground">{currentItem.episodes} eps</span>
							{/if}
						</div>

						<!-- Genres -->
						{#if currentItem.genres && currentItem.genres.length > 0}
							<div class="flex flex-wrap gap-1.5">
								{#each currentItem.genres.slice(0, 4) as genre}
									<span
										class="rounded-md border border-border/40 bg-background/40 px-2.5 py-0.5 text-[11px] font-medium backdrop-blur-sm"
										>{genre}</span
									>
								{/each}
							</div>
						{/if}

						<!-- Description -->
						{#if currentItem.description}
							<p class="line-clamp-2 max-w-lg text-xs leading-relaxed text-muted-foreground">
								{formatDescription(currentItem.description)}
							</p>
						{/if}

						<!-- Actions -->
						<div class="flex items-center gap-2 pt-0.5">
							<Button size="sm" onclick={handleViewDetails}>
								<Icon icon="solar:play-bold" class="mr-1.5 h-4 w-4" />
								View Details
							</Button>
							<Button size="sm" class="gap-2 bg-foreground text-background hover:bg-foreground/90">
								<Icon icon="solar:add-circle-bold" class="h-4 w-4" />
								Add to List
							</Button>
						</div>
					</div>
				{/key}
			</div>

			<!-- Right: cover art -->
			<div class="relative mb-1 hidden shrink-0 md:grid">
				{#key currentItem.id}
					<div
						style="grid-area: 1/1"
						in:fly={{
							x: 20,
							y: 0,
							duration: contentDuration,
							delay: Math.floor(contentDelay * 0.5),
							easing: cubicOut,
						}}
						out:fade={{ duration: Math.floor(contentDuration * 0.4), easing: cubicOut }}
					>
						{#if currentItem.coverImage?.large || currentItem.coverImage?.medium}
							<CachedImage
								src={currentItem.coverImage.large ?? currentItem.coverImage.medium ?? ''}
								alt={currentItem.title?.romaji ?? ''}
								class="h-52 w-36 rounded-xl object-cover shadow-2xl ring-2 ring-border/40 transition-transform duration-300 group-hover:scale-[1.02]"
							/>
						{/if}
					</div>
				{/key}
			</div>
		</div>

		<!-- Navigation arrows (visible on group hover) -->
		{#if items.length > 1}
			<button
				onclick={prevSlide}
				class="absolute top-1/2 left-4 -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 group-hover:opacity-100 hover:bg-background/70"
				aria-label="Previous slide"
			>
				<Icon icon="solar:alt-arrow-left-bold" class="h-5 w-5" />
			</button>
			<button
				onclick={nextSlide}
				class="absolute top-1/2 right-4 -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 group-hover:opacity-100 hover:bg-background/70"
				aria-label="Next slide"
			>
				<Icon icon="solar:alt-arrow-right-bold" class="h-5 w-5" />
			</button>
		{/if}

		<!-- Dot indicators + autoplay progress bar -->
		{#if items.length > 1}
			<div class="absolute bottom-3 left-1/2 flex -translate-x-1/2 gap-2">
				{#each items as _, index}
					<button
						onclick={() => goToSlide(index)}
						class="rounded-full transition-all duration-300 {index === currentIndex
							? 'h-1.5 w-7 bg-primary'
							: 'h-1.5 w-1.5 bg-muted-foreground/40 hover:bg-muted-foreground/70'}"
						aria-label="Go to slide {index + 1}"
					></button>
				{/each}
			</div>

			<!-- Thin progress bar at banner bottom -->
			<div class="absolute bottom-0 left-0 h-0.5 w-full bg-border/20">
				<div class="h-full bg-primary/60 transition-none" style="width: {progress}%"></div>
			</div>
		{/if}
	</div>
{/if}
