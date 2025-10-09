<script lang="ts">
	import { cubicInOut } from 'svelte/easing';
	import { fade, fly, scale } from 'svelte/transition';
	import Icon from '@iconify/svelte';
	import { tick } from 'svelte';
	import type { MediaData, MediaListStatus } from '$lib/types/media';
	import { ConfigService, type UiConfig } from '$lib/services/config';
	import GenreSubCards from './GenreSubCards.svelte';
	import { Debounced } from "runed";

	let { mediaData }: { mediaData: MediaData } = $props();

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

	// Computed values from mediaData
	const title = $derived(mediaData.title || mediaData.englishTitle || 'Unknown');
	const description = $derived(mediaData.description || 'No description available.');
	const placeholderSvg = 'https://placehold.co/600x400';
	const coverImage = $derived(mediaData.coverImage || placeholderSvg);
	const bannerImage = $derived(mediaData.bannerImage || coverImage);
	const score = $derived(mediaData.score ? Math.round(mediaData.score * 10) : 0);
	const genres = $derived(mediaData.genres || []);
	const status = $derived(mediaData.status || 'UNKNOWN');
	const episodes = $derived(mediaData.totalEpisodes || 0);
	const chapters = $derived(mediaData.totalChapters || 0);
	const season = $derived(mediaData.season || '');
	const seasonYear = $derived(mediaData.year || 0);
	const format = $derived(mediaData.format || 'Unknown');
	const popularity = $derived(mediaData.popularity || 0);
	const type = $derived(mediaData.type || 'ANIME');
	const isAdult = $derived(mediaData.isAdult || false);
	const userStatus = $derived(mediaData.userStatus);
	const userProgress = $derived(mediaData.userProgress || 0);
	const link = $derived('/' + mediaData.type?.toLowerCase() + '/' + mediaData.id);

	// Format season display
	const seasonDisplay = $derived(() => {
		if (!season || !seasonYear) return '';
		const seasonName = season.charAt(0) + season.slice(1).toLowerCase();
		return `${seasonName} ${seasonYear}`;
	});

	// Status action handlers
	const handleStatusChange = (newStatus: MediaListStatus) => {
		// TODO: Implement API call to update status
		console.log('Status changed to:', newStatus);
	};


</script>


<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:this={cardPositioner}
	class="relative flex w-40 flex-col items-center justify-start md:w-48 lg:w-48"
	onmouseenter={onmouseenter}
	onmouseleave={onmouseleave}
>
		<div
			class="ring-border bg-card group/animecard text-card-foreground h-56 md:h-68 lg:h-68 relative flex w-full flex-col overflow-hidden rounded-md shadow-lg ring-4 transition-all"
			in:scale={{ duration: animationsEnabled ? 200 : 0, start: 0.95, easing: cubicInOut }}
		>
			<!-- Cover Image -->
			<a
				href={link}
				rel="noopener noreferrer"
				class="absolute top-0 left-0 flex h-full w-full flex-col transition-all"
				data-sveltekit-preload-data="off"
			>
				<img
					src="{coverImage}"
					onerror={(e) => {
						if ((e.target as HTMLImageElement)?.src) (e.target as HTMLImageElement).src=placeholderSvg;
					}}
					alt={title}
					class="h-full w-full object-cover"
				/>
			</a>

			<!-- Top badges -->
			<div class="relative z-10 flex items-center justify-between p-2">
				<!-- Score Badge -->
				<div
					class="flex items-center justify-center rounded-md px-2 py-0.5 {blurEffectsEnabled
						? 'bg-card/65 backdrop-blur-xl'
						: 'bg-card'} gap-0.5 font-semibold shadow-lg transition-all duration-200"
				>
					<Icon icon="material-symbols:star-rounded" class="text-primary -ml-1 size-5" />
					{score}
					<span class="text-sm font-light">%</span>
				</div>

				<!-- 18+ Badge -->
				{#if isAdult}
					<div
						class="flex items-center justify-center text-destructive-foreground rounded-md px-2 py-0.5 {blurEffectsEnabled
							? 'bg-destructive/80 backdrop-blur-xl'
							: 'bg-destructive'} font-bold shadow-lg transition-all duration-200"

						in:scale={{ duration: animationsEnabled ? 200 : 0, delay: 50 }}
					>
						18+
					</div>
				{/if}
			</div>

			<!-- Progress Bar (if watching/reading) -->
			{#if userProgress && (episodes || chapters)}
				<div class="absolute flex flex-row-reverse bottom-0 left-0 z-10 w-full p-2">
					{#if userStatus !== 'COMPLETED'}
						<div class="h-2 w-full overflow-hidden rounded-md {blurEffectsEnabled
							? 'bg-card/65 backdrop-blur-xl'
							: 'bg-card'} shadow-md"
						in:fly={{ y: -10, duration: animationsEnabled ? 250 : 0 }}>
							<div
								class="h-full rounded-r-md transition-all duration-300"
								style="width: {(userProgress / (episodes || chapters)) * 100}%"
								class:bg-primary={userStatus === 'CURRENT' || userStatus === 'REPEATING'}
								class:bg-secondary={userStatus === 'PAUSED'}
								class:bg-destructive={userStatus === 'DROPPED'}
							></div>
						</div>
					{:else}
						<div class="text-xs flex items-center font-medium w-fit gap-1 whitespace-nowrap rounded-sm tracking-wide px-1.5 py-0.5 text-foreground ml-2 {blurEffectsEnabled
							? 'bg-card/65 backdrop-blur-xl'
							: 'bg-card'} shadow-md"
						in:fly={{ y: -10, duration: animationsEnabled ? 250 : 0 }}>
							<Icon icon="solar:check-read-outline" class="size-4 inline" /> Completed
						</div>
					{/if}
				</div>
			{/if}

		</div>
		<!-- Title at bottom -->
		<div class="w-full mt-3">
			<div
				class="flex flex-col gap-1 rounded-sm"
			>
				<h2 class="line-clamp-2 text-left text-xs font-semibold md:text-sm">
					{title}
				</h2>
				<div class="flex items-center justify-between text-[10px] text-muted-foreground">
					<span>{seasonDisplay()}</span>
					<span>{format}</span>
				</div>
			</div>
		</div>

	{#if hoverCardEnabled && debouncedHovering.current}
		<div
			class="ring-card bg-card group/animecard text-card-foreground absolute z-30 flex h-auto w-[140%] flex-col rounded-md shadow-[0px_0px_20px_20px_rgba(0,_0,_0,_0.4)] ring-[12px] gap-2 transition-all {position ===
			'left'
				? 'right-0'
				: position === 'right'
					? 'left-0'
					: ''}"
			in:scale={{ duration: animationsEnabled ? 100 : 0, start: 0.85, easing: cubicInOut }}
		>
			<!-- Banner Image -->
			<a
				href={link}
				rel="noopener noreferrer"
				class="relative flex h-32 w-full flex-col rounded-lg transition-all"
				data-sveltekit-preload-data="off"
			>
				<img
					src={bannerImage}
					alt={title + ' Banner'}
					class="absolute h-full w-full object-cover rounded-lg"
					onerror={(e) => {
						if ((e.target as HTMLImageElement)?.src && (e.target as HTMLImageElement).src !== coverImage) (e.target as HTMLImageElement).src=coverImage;
						else (e.target as HTMLImageElement).src=placeholderSvg;
					}}
				/>
				<div
					in:fade={{ duration: animationsEnabled ? 300 : 0 }}
					class="absolute inset-0 -z-10 opacity-80"
				>
					{#if glowEffectsEnabled}
						<img
							src={bannerImage}
							alt="Banner Glow"
							class="h-full w-full object-cover blur-lg"
							onerror={(e) => {
								if ((e.target as HTMLImageElement)?.src && (e.target as HTMLImageElement).src !== coverImage) (e.target as HTMLImageElement).src=coverImage;
								else (e.target as HTMLImageElement).src=placeholderSvg;
							}}
						/>
					{/if}
				</div>

				<div class="p-2 w-full h-full flex flex-col justify-between items-start rounded-lg overflow-hidden">
					<!-- Score and 18+ badge on banner -->
					<div class="flex items-center justify-between w-full"
					>
						<div
							class="flex items-center justify-center rounded-md px-2 py-0.5 {blurEffectsEnabled
								? 'bg-card/65 backdrop-blur-xl'
								: 'bg-card'} gap-0.5 font-semibold shadow-lg"
						>
							<Icon icon="material-symbols:star-rounded" class="text-primary -ml-1 size-5" />
							{score}
							<span class="text-sm font-light">%</span>
						</div>

						{#if isAdult}
							<div
								class="flex items-center justify-center text-destructive-foreground rounded-md px-2 py-0.5 {blurEffectsEnabled
									? 'bg-destructive/80 backdrop-blur-xl'
									: 'bg-destructive'} font-bold shadow-lg"
							>
								18+
							</div>
						{/if}
					</div>

					<!-- Progress Bar on banner -->
					{#if userProgress && (episodes || chapters)}
						<div class="flex flex-row-reverse z-10 items-end justify-between w-full">
							{#if userStatus !== 'COMPLETED'}
								<span class="text-xs flex items-center font-light gap-1 whitespace-nowrap rounded-sm tracking-wide px-1.5 py-0.5 text-foreground ml-2 {blurEffectsEnabled
									? 'bg-card/65 backdrop-blur-xl'
									: 'bg-card'} shadow-md"
								>
									{#if userStatus === 'CURRENT' || userStatus === 'REPEATING'}
										<Icon icon="mingcute:play-fill" class="size-2.5 inline" />
									{:else if userStatus === 'PAUSED'}
										<Icon icon="mingcute:pause-fill" class="size-2.5 inline" />
									{:else if userStatus === 'DROPPED'}
										<Icon icon="mingcute:close-fill" class="size-3 inline" />
									{/if}
									{userProgress} / {episodes || chapters}
								</span>
								<div class="h-2 w-full overflow-hidden rounded-md {blurEffectsEnabled
									? 'bg-card/65 backdrop-blur-xl'
									: 'bg-card'} shadow-md"
								>
									<div
										class="h-full rounded-r-md transition-all duration-300"
										style="width: {(userProgress / (episodes || chapters)) * 100}%"
										class:bg-primary={userStatus === 'CURRENT' || userStatus === 'REPEATING'}
										class:bg-secondary={userStatus === 'PAUSED'}
										class:bg-destructive={userStatus === 'DROPPED'}
									></div>
								</div>
							{:else}
								<span class="text-xs flex items-center font-medium gap-1 whitespace-nowrap rounded-sm tracking-wide px-1.5 py-0.5 text-foreground ml-2 {blurEffectsEnabled
									? 'bg-card/65 backdrop-blur-xl'
									: 'bg-card'} shadow-md"
								>
									<Icon icon="solar:check-read-outline" class="size-4 inline" /> Completed
								</span>
							{/if}
						</div>
					{/if}
				</div>
			</a>

			<!-- Content section -->
			<div class="flex flex-col gap-2">
				<!-- Title -->
				<h2
					class="line-clamp-2 text-sm font-semibold md:text-base"
				>
					{title}
				</h2>

				<!-- Genres -->
				<GenreSubCards genres={genres} />

				<!-- Description -->
				<p class="line-clamp-3 text-[10px] text-muted-foreground">
					{description}
				</p>

				<!-- Action Buttons Strip -->
				<div
					class="bg-border flex items-center justify-between gap-1 rounded-md p-1"
				>
					<button
						onclick={() => handleStatusChange('PLANNING')}
						class="hover:bg-primary/20 flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors {userStatus ===
						'PLANNING'
							? 'bg-primary/30'
							: 'bg-background/50'}"
						title="Plan to Watch"
					>
						<Icon icon="material-symbols:bookmark-outline" class="size-3.5" />
						Plan
					</button>
					<button
						onclick={() => handleStatusChange('CURRENT')}
						class="hover:bg-primary/20 flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors {userStatus ===
						'CURRENT'
							? 'bg-primary/30'
							: 'bg-background/50'}"
						title="Watching"
					>
						<Icon icon="material-symbols:play-circle-outline" class="size-3.5" />
						Watch
					</button>
					<button
						onclick={() => handleStatusChange('COMPLETED')}
						class="hover:bg-primary/20 flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors {userStatus ===
						'COMPLETED'
							? 'bg-primary/30'
							: 'bg-background/50'}"
						title="Completed"
					>
						<Icon icon="material-symbols:check-circle-outline" class="size-3.5" />
						Done
					</button>
					<button
						onclick={() => handleStatusChange('PAUSED')}
						class="hover:bg-primary/20 flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors {userStatus ===
						'PAUSED'
							? 'bg-primary/30'
							: 'bg-background/50'}"
						title="Paused"
					>
						<Icon icon="material-symbols:pause-circle-outline" class="size-3.5" />
					</button>
					<button
						onclick={() => handleStatusChange('DROPPED')}
						class="hover:bg-primary/20 flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors {userStatus ===
						'DROPPED'
							? 'bg-primary/30'
							: 'bg-background/50'}"
						title="Dropped"
					>
						<Icon icon="material-symbols:cancel-outline" class="size-3.5" />
					</button>
				</div>

				<!-- Stats Grid -->
				<div
					class="bg-border grid grid-cols-2 gap-1.5 rounded-md p-1.5"
				>
					<!-- Status -->
					<div
						class="col-span-2 rounded-sm p-1.5 text-center text-xs font-semibold {status ===
						'RELEASING'
							? 'bg-green-500/40'
							: status === 'FINISHED'
								? 'bg-blue-500/40'
								: 'bg-muted/40'}"
					>
						{status ? status.charAt(0) + status.slice(1).toLowerCase().replace('_', ' ') : ''}
					</div>

					<!-- Episodes/Chapters -->
					<div class="bg-background/75 flex items-center justify-center gap-1 rounded-sm px-2 py-1.5">
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
					<div class="bg-background/75 flex items-center justify-center gap-1 rounded-sm px-2 py-1.5">
						<Icon icon="mingcute:user-3-fill" class="size-4" />
						<span class="text-xs font-medium">
							{popularity > 1000 ? (popularity / 1000).toFixed(1) + 'k' : popularity}
						</span>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
