<script lang="ts">
	import { cubicInOut } from 'svelte/easing';
	import { fade, fly, scale } from 'svelte/transition';
	import Icon from '@iconify/svelte';
	import { tick } from 'svelte';
	import type { MediaData } from '$lib/types/media';
	import { ConfigService, type UiConfig } from '$lib/services/config';
	import GenreSubCards from './GenreSubCards.svelte';
	import { Debounced } from 'runed';
	import ActionButtonStrip from './ActionButtonStrip.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import AddToListDialog from './AddToListDialog.svelte';
	import type { Media } from '$lib/types/anilist';

	let { media }: { media: Media } = $props();

	// Config states
	let blurEffectsEnabled = $state(false);
	let glowEffectsEnabled = $state(false);
	let animationsEnabled = $state(false);
	let hoverCardEnabled = $state(false);

	// Load config on mount
	$effect(() => {
		ConfigService.getUiConfig().then((config: UiConfig) => {
			if (config) {
				animationsEnabled = config.animations;
				glowEffectsEnabled = config.glow_effects;
				blurEffectsEnabled = config.blur_effects;
				hoverCardEnabled = config.hover_card;
			}
		});
	});

	let isLoading = $state(false);
	let isHovering = $state(false);
	const debouncedHovering = new Debounced(() => isHovering, 100);

	const onmouseenter = () => {
		isHovering = true;
	};

	const onmouseleave = () => {
		isHovering = false;
		debouncedHovering.updateImmediately();
	};

	let cardPositioner: HTMLDivElement | null = $state(null);
	let position: 'left' | 'right' | 'center' = $state('center');

	const adjustPosition = () => {
		if (!cardPositioner) return;

		const rect = cardPositioner.getBoundingClientRect();
		const padding = 100; // buffer from edge
		const viewportWidth = window.innerWidth;
		if (rect.left < padding) {
			position = 'right';
		} else if (rect.right > viewportWidth - padding) {
			position = 'left';
		} else {
			position = 'center';
		}
	};

	$effect(() => {
		if (isHovering) {
			tick().then(adjustPosition);
		}
	});

	// Computed values from media
	const title = $derived(media.title?.userPreferred || 'Unknown Title');
	const description = $derived(media.description || 'No description available.');
	const placeholderSvg = 'https://placehold.co/600x400';
	const coverImage = $derived(media.coverImage?.large || placeholderSvg);
	const bannerImage = $derived(media.bannerImage || coverImage);
	const score = $derived(media.averageScore);
	const genres = $derived(media.genres);
	const status = $derived(media.status);
	const episodes = $derived(media.episodes);
	const chapters = $derived(media.chapters);
	const season = $derived(media.season);
	const seasonYear = $derived(media.seasonYear);
	const format = $derived(media.format);
	const popularity = $derived(media.popularity);
	const type = $derived(media.type || 'ANIME');
	const studio = $derived(media.studios?.edges?.[0]?.node?.name);
	const isAdult = $derived(media.isAdult || false);
	const userStatus = $derived(media?.mediaListEntry?.status);
	const userProgress = $derived(media?.mediaListEntry?.progress || 0);
	const link = $derived('/' + (media.type || 'ANIME').toLowerCase() + '/' + media.id);

	// Format season display
	const seasonDisplay = $derived(() => {
		if (!season || !seasonYear) return '';
		const seasonName = season.charAt(0) + season.slice(1).toLowerCase();
		return `${seasonName} ${seasonYear}`;
	});

	let addToListOpen = $state(false);
</script>

<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:this={cardPositioner}
	class="relative flex w-40 flex-col items-center justify-start md:w-48 lg:w-48"
	{onmouseenter}
	{onmouseleave}
>
	<div
		class="group/animecard relative flex h-56 w-full flex-col overflow-hidden rounded-md bg-card text-card-foreground shadow-lg ring-4 ring-border transition-all md:h-68 lg:h-68"
		in:scale={{ duration: animationsEnabled ? 200 : 0, start: 0.95, easing: cubicInOut }}
	>
		<!-- Cover Image -->
		<a
			href={link}
			rel="noopener noreferrer"
			class="absolute top-0 left-0 flex h-full w-full flex-col transition-all"
			data-sveltekit-preload-data="off"
		>
			<CachedImage src={coverImage} alt={title} class="h-full w-full object-cover" />
		</a>

		<!-- Top badges -->
		<div class="relative z-10 flex items-center justify-between p-2">
			<!-- Score Badge -->
			<div
				class="flex items-center justify-center rounded-md px-2 py-0.5 text-card-foreground {blurEffectsEnabled
					? 'bg-card/65 backdrop-blur-xl'
					: 'bg-card'} gap-0.5 font-semibold shadow-lg transition-all duration-200"
			>
				<Icon icon="solar:star-bold" class="-ml-1 size-5 text-primary" />
				{score}
				<span class="text-sm font-light">%</span>
			</div>

			<!-- 18+ Badge -->
			{#if isAdult}
				<div
					class="flex items-center justify-center rounded-md px-2 py-0.5 text-destructive-foreground {blurEffectsEnabled
						? 'bg-destructive/80 backdrop-blur-xl'
						: 'bg-destructive'} font-bold shadow-lg transition-all duration-200"
					in:scale={{ duration: animationsEnabled ? 200 : 0, delay: 50 }}
				>
					18+
				</div>
			{/if}
		</div>

		<!-- Progress Bar (if watching/reading) -->
		{#if userProgress && (episodes != null || chapters != null)}
			<div class="absolute bottom-0 left-0 z-10 flex w-full flex-row-reverse p-2">
				{#if userStatus !== 'COMPLETED'}
					<div
						class="h-2 w-full overflow-hidden rounded-md {blurEffectsEnabled
							? 'bg-card/65 backdrop-blur-xl'
							: 'bg-card'} shadow-md"
						in:fly={{ y: -10, duration: animationsEnabled ? 250 : 0 }}
					>
						<div
							class="h-full rounded-r-md transition-all duration-300"
							style="width: {(userProgress /
								(episodes != null ? episodes : chapters != null ? chapters : 0)) *
								100}%"
							class:bg-primary={userStatus === 'CURRENT' || userStatus === 'REPEATING'}
							class:bg-secondary={userStatus === 'PAUSED'}
							class:bg-destructive={userStatus === 'DROPPED'}
						></div>
					</div>
				{:else}
					<div
						class="ml-2 flex w-fit items-center gap-1 rounded-sm px-1.5 py-0.5 text-xs font-medium tracking-wide whitespace-nowrap text-foreground {blurEffectsEnabled
							? 'bg-card/65 backdrop-blur-xl'
							: 'bg-card'} shadow-md"
						in:fly={{ y: -10, duration: animationsEnabled ? 250 : 0 }}
					>
						<Icon icon="solar:check-read-outline" class="inline size-4" /> Completed
					</div>
				{/if}
			</div>
		{/if}
	</div>
	<!-- Title at bottom -->
	<div class="mt-3 w-full">
		<div class="flex flex-col gap-1 rounded-sm">
			<h2 class="line-clamp-2 text-left text-xs font-semibold md:text-sm">
				{title}
			</h2>
			<div class="flex items-center justify-between text-[10px] text-muted-foreground">
				<span>{seasonDisplay()}</span>
				<div class="flex items-center gap-1.5">
					<span>{format}</span>
					{#if !hoverCardEnabled}
						<button
							type="button"
							title="Add to list"
							onclick={(e) => {
								e.preventDefault();
								e.stopPropagation();
								addToListOpen = true;
							}}
							class="flex h-4 w-4 items-center justify-center rounded-full transition-colors
								{userStatus
								? 'bg-primary text-primary-foreground'
								: 'bg-muted text-muted-foreground hover:bg-primary hover:text-primary-foreground'}"
						>
							<Icon
								icon={userStatus ? 'solar:check-circle-bold' : 'solar:add-circle-bold'}
								class="size-3"
							/>
						</button>
					{/if}
				</div>
			</div>
		</div>
	</div>

	{#if hoverCardEnabled && debouncedHovering.current}
		<div
			class="group/animecard absolute z-30 flex h-auto w-[140%] flex-col gap-2 rounded-md text-card-foreground shadow-[0px_0px_20px_20px_rgba(0,0,0,0.4)] ring-12 ring-card transition-all {position ===
			'left'
				? 'right-0'
				: position === 'right'
					? 'left-0'
					: ''} {blurEffectsEnabled ? 'bg-card/95 backdrop-blur-xl' : 'bg-card'}"
			in:scale={{ duration: animationsEnabled ? 100 : 0, start: 0.85, easing: cubicInOut }}
		>
			<!-- Banner Image -->
			<a
				href={link}
				rel="noopener noreferrer"
				class="relative flex h-32 w-full flex-col rounded-lg transition-all"
				data-sveltekit-preload-data="off"
			>
				<CachedImage
					src={bannerImage}
					fallbackSrc={coverImage}
					alt={title + ' Banner'}
					class="absolute h-full w-full rounded-lg object-cover"
				/>
				<div
					in:fade={{ duration: animationsEnabled ? 300 : 0 }}
					class="absolute inset-0 -z-10 opacity-80"
				>
					{#if glowEffectsEnabled}
						<CachedImage
							src={bannerImage}
							fallbackSrc={coverImage}
							alt="Banner Glow"
							class="h-full w-full object-cover blur-lg"
						/>
					{/if}
				</div>

				<div
					class="flex h-full w-full flex-col items-start justify-between overflow-hidden rounded-lg p-2"
				>
					<!-- Score and 18+ badge on banner -->
					<div class="flex w-full flex-row-reverse items-center justify-between">
						<!-- <div
							class="flex items-center justify-center rounded-md px-2 py-0.5 {blurEffectsEnabled
								? 'bg-card/65 backdrop-blur-xl'
								: 'bg-card'} gap-0.5 font-semibold shadow-lg"
						>
							<Icon icon="material-symbols:star-rounded" class="-ml-1 size-5 text-primary" />
							{score}
							<span class="text-sm font-light">%</span>
						</div>

						{#if isAdult}
							<div
								class="flex items-center justify-center rounded-md px-2 py-0.5 text-destructive-foreground {blurEffectsEnabled
									? 'bg-destructive/80 backdrop-blur-xl'
									: 'bg-destructive'} font-bold shadow-lg"
							>
								18+
							</div>
						{/if} -->
						{#if studio}
							<div
								class="flex items-center justify-center rounded-md px-2 py-0.5 text-xs text-card-foreground {blurEffectsEnabled
									? 'bg-card/65 backdrop-blur-xl'
									: 'bg-card'} gap-0.5 shadow-lg"
							>
								<Icon icon="solar:clapperboard-open-play-bold" class="-ml-1 size-3" />
								{studio}
							</div>
						{/if}
					</div>

					<!-- Progress Bar on banner -->
					{#if userProgress && (episodes || chapters)}
						<div class="z-10 flex w-full flex-row-reverse items-end justify-between">
							{#if userStatus !== 'COMPLETED'}
								<span
									class="ml-2 flex items-center gap-1 rounded-sm px-1.5 py-0.5 text-xs font-light tracking-wide whitespace-nowrap text-foreground {blurEffectsEnabled
										? 'bg-card/65 backdrop-blur-xl'
										: 'bg-card'} shadow-md"
								>
									{#if userStatus === 'CURRENT' || userStatus === 'REPEATING'}
										<Icon icon="mingcute:play-fill" class="inline size-2.5" />
									{:else if userStatus === 'PAUSED'}
										<Icon icon="mingcute:pause-fill" class="inline size-2.5" />
									{:else if userStatus === 'DROPPED'}
										<Icon icon="mingcute:close-fill" class="inline size-3" />
									{/if}
									{userProgress} / {episodes || chapters}
								</span>
								<div
									class="h-2 w-full overflow-hidden rounded-md {blurEffectsEnabled
										? 'bg-card/65 backdrop-blur-xl'
										: 'bg-card'} shadow-md"
								>
									<div
										class="h-full rounded-r-md transition-all duration-300"
										style="width: {(userProgress /
											(episodes != null ? episodes : chapters != null ? chapters : 0)) *
											100}%"
										class:bg-primary={userStatus === 'CURRENT' || userStatus === 'REPEATING'}
										class:bg-secondary={userStatus === 'PAUSED'}
										class:bg-destructive={userStatus === 'DROPPED'}
									></div>
								</div>
							{:else}
								<span
									class="ml-2 flex items-center gap-1 rounded-sm px-1.5 py-0.5 text-xs font-medium tracking-wide whitespace-nowrap text-foreground {blurEffectsEnabled
										? 'bg-card/65 backdrop-blur-xl'
										: 'bg-card'} shadow-md"
								>
									<Icon icon="solar:check-read-outline" class="inline size-4" /> Completed
								</span>
							{/if}
						</div>
					{/if}
				</div>
			</a>

			<!-- Content section -->
			<div class="flex flex-col gap-2">
				<!-- Title -->
				<h2 class="line-clamp-2 text-sm font-semibold md:text-base">
					{title}
				</h2>

				<!-- Genres -->
				{#if genres && genres.length > 0}
					<GenreSubCards {genres} />
				{/if}

				<!-- Description -->
				<p class="line-clamp-3 text-xs text-muted-foreground">
					{@html description
						.replaceAll('<br />', '')
						.replaceAll('<br><br>', '<br>')
						.replaceAll('<br><br>', '<br>')}
				</p>

				<!-- Action Buttons Strip -->
				<ActionButtonStrip
					{userStatus}
					mediaId={media.id}
					mediaType={media.type === 'MANGA' ? 'MANGA' : 'ANIME'}
				/>

				<!-- Stats Grid -->
				<div class="grid grid-cols-2 gap-1.5 rounded-md bg-border p-1.5">
					<!-- Status + Add to list (same row) -->
					<div
						class="rounded-sm p-1.5 text-center text-xs font-semibold {status === 'RELEASING'
							? 'bg-green-500/40'
							: status === 'FINISHED'
								? 'bg-blue-500/40'
								: 'bg-muted/40'}"
					>
						{status ? status.charAt(0) + status.slice(1).toLowerCase().replace('_', ' ') : ''}
					</div>
					<button
						type="button"
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							addToListOpen = true;
						}}
						class="flex items-center justify-center gap-1 rounded-sm p-1.5 text-xs font-medium transition-colors
							{userStatus
							? 'bg-primary/20 text-primary hover:bg-primary hover:text-primary-foreground'
							: 'bg-muted/40 text-muted-foreground hover:bg-primary/20 hover:text-primary'}"
					>
						<Icon
							icon={userStatus ? 'solar:pen-2-linear' : 'solar:add-circle-linear'}
							class="size-3.5"
						/>
						{userStatus ? 'Edit' : 'Add'}
					</button>

					<!-- Episodes/Chapters -->
					<div
						class="flex items-center justify-center gap-1 rounded-sm bg-background/75 px-2 py-1.5"
					>
						<Icon
							icon={type === 'ANIME' ? 'fluent:tv-16-filled' : 'mynaui:book-solid'}
							class="size-4"
						/>
						<span class="text-xs font-medium">
							{episodes || '??'}
							{type === 'ANIME' ? 'Eps' : 'Ch'}
						</span>
					</div>

					<!-- Popularity -->
					{#if popularity != null}
						<div
							class="flex items-center justify-center gap-1 rounded-sm bg-background/75 px-2 py-1.5"
						>
							<Icon icon="mingcute:user-3-fill" class="size-4" />
							<span class="text-xs font-medium">
								{popularity > 1000 ? (popularity / 1000).toFixed(1) + 'k' : popularity}
							</span>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>

<AddToListDialog bind:open={addToListOpen} {media} />
