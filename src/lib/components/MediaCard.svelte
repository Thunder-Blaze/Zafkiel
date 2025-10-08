<script lang="ts">
	import type { MediaData } from '$lib/types/media';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Progress } from '$lib/components/ui/progress';
	import { Separator } from '$lib/components/ui/separator';
	import Icon from '@iconify/svelte';
	import { scale, fly } from 'svelte/transition';
	import { isAuthenticated } from '$lib/stores/auth';
	import { ConfigService, type UiConfig } from '$lib/services/config';

	interface Props {
		mediaData: MediaData;
		onProgressUpdate?: (newProgress: number) => void;
		onStatusChange?: (newStatus: string) => void;
	}

	let { mediaData, onProgressUpdate, onStatusChange }: Props = $props();

	let isHovered = $state(false);
	let previewScrollable = $state(false);
	let animationsEnabled = $state(true);
	let glowEffectsEnabled = $state(true);
	let blurEffectsEnabled = $state(true);

	// Load config on mount
	$effect(() => {
		ConfigService.getUiConfig().then((config: UiConfig) => {
			if (config) {
				animationsEnabled = config.animations;
				glowEffectsEnabled = config.glow_effects;
				blurEffectsEnabled = config.blur_effects;
			}
		});
	});

	// Computed values
	const displayTitle = $derived(mediaData.title || mediaData.englishTitle || 'Unknown Title');
	const isAnime = $derived(mediaData.type === 'ANIME');
	const progressPercentage = $derived(
		mediaData.userProgress && mediaData.totalEpisodes
			? (mediaData.userProgress / mediaData.totalEpisodes) * 100
			: 0
	);
	const isWatching = $derived(
		mediaData.userStatus === 'CURRENT' || mediaData.userStatus === 'REPEATING'
	);

	// Status display mapping - Make it a function to access reactive isAnime
	const getStatusLabel = (status: string): string => {
		const labels: Record<string, string> = {
			CURRENT: isAnime ? 'Watching' : 'Reading',
			PLANNING: 'Planning',
			COMPLETED: 'Completed',
			DROPPED: 'Dropped',
			PAUSED: 'Paused',
			REPEATING: isAnime ? 'Rewatching' : 'Rereading'
		};
		return labels[status] || status;
	};

	// Format score for display
	function formatScore(score?: number): string {
		if (!score) return 'N/A';
		return score.toFixed(1);
	}

	// Truncate description
	function truncateDescription(text?: string, maxLength: number = 250): string {
		if (!text) return 'No description available.';
		const stripped = text.replace(/<[^>]*>/g, '');
		if (stripped.length <= maxLength) return stripped;
		return stripped.substring(0, maxLength) + '...';
	}

	// Handle progress increment
	function handleProgressIncrement() {
		if (!mediaData.totalEpisodes) return;
		const newProgress = Math.min((mediaData.userProgress || 0) + 1, mediaData.totalEpisodes);
		onProgressUpdate?.(newProgress);
	}

	// Handle progress decrement
	function handleProgressDecrement() {
		const newProgress = Math.max((mediaData.userProgress || 0) - 1, 0);
		onProgressUpdate?.(newProgress);
	}

	// Handle status change
	function handleStatusChange(value: string) {
		onStatusChange?.(value);
	}

	// Check if description needs scrolling
	function checkScrollable(node: HTMLElement) {
		previewScrollable = node.scrollHeight > node.clientHeight;
	}
</script>

<div
	class="relative group"
	role="article"
	onmouseenter={() => (isHovered = true)}
	onmouseleave={() => (isHovered = false)}
>
	<!-- Normal Card -->
	<Card class="overflow-hidden py-0 transition-all {animationsEnabled ? 'hover:shadow-lg' : ''}">
		<CardContent class="p-0">
			<!-- Cover Image with optional glow effect -->
			<div class="relative aspect-[2/3] overflow-hidden bg-muted">
				{#if mediaData.coverImage}
					<div class="relative h-full w-full">
						<!-- Blurred glow layer behind -->
						{#if glowEffectsEnabled}
							<img
								src={mediaData.coverImage}
								alt=""
								class="absolute inset-0 h-full w-full object-cover blur-2xl opacity-60 scale-110"
								aria-hidden="true"
							/>
						{/if}
						<!-- Main cover image -->
						<img
							src={mediaData.coverImage}
							alt={displayTitle}
							class="relative h-full w-full object-cover {animationsEnabled
								? 'transition-transform duration-300 group-hover:scale-105'
								: ''}"
						/>
					</div>
				{:else}
					<div class="flex h-full w-full items-center justify-center bg-muted">
						<Icon icon="solar:video-library-bold" class="h-16 w-16 text-muted-foreground/20" />
					</div>
				{/if}

				<!-- 18+ Badge -->
				{#if mediaData.isAdult}
					<Badge variant="destructive" class="absolute top-2 right-2 font-bold shadow-lg">
						18+
					</Badge>
				{/if}

				<!-- Score Badge -->
				{#if mediaData.score}
					<div
						class="absolute top-2 left-2 flex items-center gap-1 rounded-md bg-background/90 px-2 py-1 shadow-lg backdrop-blur-sm {animationsEnabled
							? 'transition-all duration-200 hover:scale-105'
							: ''}"
					>
						<Icon icon="solar:star-bold" class="h-3.5 w-3.5 text-yellow-500" />
						<span class="text-xs font-semibold">{formatScore(mediaData.score)}</span>
					</div>
				{/if}

				<!-- User Progress -->
				{#if $isAuthenticated && mediaData.userProgress !== undefined}
					<div class="absolute bottom-0 left-0 right-0 bg-background/90 p-2 backdrop-blur-sm">
						<div class="flex items-center justify-between text-xs">
							<span class="text-muted-foreground">
								{mediaData.userProgress}/{mediaData.totalEpisodes || '?'}
							</span>
							{#if mediaData.userStatus}
								<Badge variant="secondary" class="h-5 text-xs">
									{getStatusLabel(mediaData.userStatus)}
								</Badge>
							{/if}
						</div>
						{#if mediaData.totalEpisodes}
							<Progress value={progressPercentage} class="mt-1.5 h-1" />
						{/if}
					</div>
				{/if}
			</div>

			<!-- Title -->
			<div class="p-3">
				<h3 class="line-clamp-2 text-sm font-semibold leading-tight">
					{displayTitle}
				</h3>
			</div>
		</CardContent>
	</Card>

	<!-- Preview Card (on hover) -->
	{#if isHovered}
		<div
			class="absolute left-0 top-0 z-50 w-[320px]"
			transition:scale={animationsEnabled ? { duration: 200, start: 0.95 } : { duration: 0 }}
		>
			<Card class="py-0 overflow-hidden shadow-2xl {blurEffectsEnabled ? 'backdrop-blur-xl' : ''}">
				<CardContent class="p-0 {blurEffectsEnabled ? 'backdrop-blur-xl bg-background/95' : 'bg-background'}">
					<!-- Banner Image with glow -->
					<div
						class="relative h-32 overflow-hidden bg-gradient-to-br from-primary/20 to-accent/20"
					>
						{#if mediaData.bannerImage}
							<div class="relative h-full w-full">
								<!-- Blurred glow layer -->
								{#if glowEffectsEnabled}
									<img
										src={mediaData.bannerImage}
										alt=""
										class="absolute inset-0 h-full w-full object-cover blur-xl opacity-50 scale-110"
										aria-hidden="true"
									/>
								{/if}
								<!-- Main banner image -->
								<img
									src={mediaData.bannerImage}
									alt=""
									class="relative h-full w-full object-cover"
								/>
								<div
									class="absolute inset-0 bg-gradient-to-t from-background/90 to-transparent"
								></div>
							</div>
						{:else}
							<div class="flex h-full w-full items-center justify-center">
								<Icon icon="solar:gallery-bold" class="h-12 w-12 text-muted-foreground/20" />
							</div>
						{/if}

						<!-- Title overlay on banner -->
						<div class="absolute bottom-2 left-3 right-3">
							<h3 class="line-clamp-2 text-sm font-bold text-white drop-shadow-lg">
								{displayTitle}
							</h3>
						</div>
					</div>

					<!-- Content Section -->
					<div class="space-y-3 p-4">
						<!-- Metadata Row -->
						<div class="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
							<!-- Type Badge -->
							{#if mediaData.type}
								<Badge
									variant="outline"
									class="gap-1 {animationsEnabled
										? 'transition-all hover:scale-105 hover:bg-primary/10'
										: ''}"
								>
									<Icon
										icon={isAnime ? 'solar:tv-bold' : 'solar:book-2-bold'}
										class="h-3 w-3"
									/>
									{mediaData.type}
								</Badge>
							{/if}

							<!-- Format -->
							{#if mediaData.format}
								<Badge
									variant="outline"
									class={animationsEnabled
										? 'transition-all hover:scale-105 hover:bg-primary/10'
										: ''}
								>
									{mediaData.format}
								</Badge>
							{/if}

							<!-- Year & Season -->
							{#if mediaData.year}
								<Badge variant="outline" class="gap-1">
									<Icon icon="solar:calendar-bold" class="h-3 w-3" />
									{mediaData.season ? `${mediaData.season} ` : ''}{mediaData.year}
								</Badge>
							{/if}

							<!-- Episodes/Chapters -->
							{#if mediaData.totalEpisodes}
								<Badge variant="outline" class="gap-1">
									<Icon
										icon={isAnime
											? 'solar:videocamera-record-bold'
											: 'solar:documents-bold'}
										class="h-3 w-3"
									/>
									{mediaData.totalEpisodes}
									{isAnime ? 'eps' : 'ch'}
								</Badge>
							{/if}

							<!-- Duration (Anime only) -->
							{#if isAnime && mediaData.duration}
								<Badge variant="outline" class="gap-1">
									<Icon icon="solar:clock-circle-bold" class="h-3 w-3" />
									{mediaData.duration}m
								</Badge>
							{/if}
						</div>

						<!-- Score & Popularity -->
						<div class="flex items-center gap-3 text-xs">
							{#if mediaData.score}
								<div class="flex items-center gap-1">
									<Icon icon="solar:star-bold" class="h-4 w-4 text-yellow-500" />
									<span class="font-semibold">{formatScore(mediaData.score)}</span>
								</div>
							{/if}
							{#if mediaData.popularity}
								<div class="flex items-center gap-1 text-muted-foreground">
									<Icon icon="solar:users-group-rounded-bold" class="h-4 w-4" />
									<span>{mediaData.popularity.toLocaleString()}</span>
								</div>
							{/if}
							{#if mediaData.favourites}
								<div class="flex items-center gap-1 text-muted-foreground">
									<Icon icon="solar:heart-bold" class="h-4 w-4" />
									<span>{mediaData.favourites.toLocaleString()}</span>
								</div>
							{/if}
						</div>

						<!-- Genres -->
						{#if mediaData.genres && mediaData.genres.length > 0}
							<div class="flex flex-wrap gap-1.5">
								{#each mediaData.genres.slice(0, 5) as genre}
									<Badge variant="secondary" class="text-xs">
										{genre}
									</Badge>
								{/each}
							</div>
						{/if}

						<Separator />

						<!-- Description -->
						<div
							class="max-h-24 overflow-y-auto text-xs leading-relaxed text-muted-foreground"
							use:checkScrollable
						>
							<p>{truncateDescription(mediaData.description, 300)}</p>
						</div>

						{#if $isAuthenticated}
							<Separator />

							<!-- Progress Controls (if watching/reading) -->
							{#if isWatching && mediaData.totalEpisodes}
								<div class="space-y-2">
									<div class="flex items-center justify-between text-xs">
										<span class="text-muted-foreground">Progress</span>
										<span class="font-semibold">
											{mediaData.userProgress || 0}/{mediaData.totalEpisodes}
										</span>
									</div>
									<Progress value={progressPercentage} class="h-2" />
									<div class="flex gap-2">
										<Button
											variant="outline"
											size="sm"
											class="flex-1 gap-1 {animationsEnabled
												? 'transition-all active:scale-95'
												: ''}"
											onclick={handleProgressDecrement}
											disabled={(mediaData.userProgress || 0) === 0}
										>
											<Icon icon="solar:minus-circle-bold" class="h-4 w-4" />
											<span class="text-xs">-1</span>
										</Button>
										<Button
											variant="default"
											size="sm"
											class="flex-1 gap-1 {animationsEnabled
												? 'transition-all active:scale-95'
												: ''}"
											onclick={handleProgressIncrement}
											disabled={mediaData.userProgress === mediaData.totalEpisodes}
										>
											<Icon icon="solar:play-circle-bold" class="h-4 w-4" />
											<span class="text-xs">+1 {isAnime ? 'Episode' : 'Chapter'}</span>
										</Button>
									</div>
								</div>
							{/if}

							<!-- Status Selector (if not watching) -->
							{#if !isWatching}
								<div class="space-y-2">
									<div class="text-xs text-muted-foreground">Change Status</div>
									<div class="grid grid-cols-2 gap-2">
										<Button
											variant="outline"
											size="sm"
											class="gap-1 justify-start {animationsEnabled
												? 'transition-all hover:scale-105 active:scale-95'
												: ''}"
											onclick={() => handleStatusChange('CURRENT')}
										>
											<Icon icon="solar:play-circle-bold" class="h-3.5 w-3.5" />
											<span class="text-xs">{isAnime ? 'Watch' : 'Read'}</span>
										</Button>
										<Button
											variant="outline"
											size="sm"
											class="gap-1 justify-start {animationsEnabled
												? 'transition-all hover:scale-105 active:scale-95'
												: ''}"
											onclick={() => handleStatusChange('COMPLETED')}
										>
											<Icon icon="solar:check-circle-bold" class="h-3.5 w-3.5" />
											<span class="text-xs">Complete</span>
										</Button>
										<Button
											variant="outline"
											size="sm"
											class="gap-1 justify-start {animationsEnabled
												? 'transition-all hover:scale-105 active:scale-95'
												: ''}"
											onclick={() => handleStatusChange('PAUSED')}
										>
											<Icon icon="solar:pause-circle-bold" class="h-3.5 w-3.5" />
											<span class="text-xs">Pause</span>
										</Button>
										<Button
											variant="outline"
											size="sm"
											class="gap-1 justify-start {animationsEnabled
												? 'transition-all hover:scale-105 active:scale-95'
												: ''}"
											onclick={() => handleStatusChange('DROPPED')}
										>
											<Icon icon="solar:close-circle-bold" class="h-3.5 w-3.5" />
											<span class="text-xs">Drop</span>
										</Button>
									</div>
								</div>
							{/if}
						{/if}
					</div>
				</CardContent>
			</Card>
		</div>
	{/if}
</div>

<style>
	/* Custom scrollbar for description */
	.overflow-y-auto::-webkit-scrollbar {
		width: 4px;
	}

	.overflow-y-auto::-webkit-scrollbar-track {
		background: transparent;
	}

	.overflow-y-auto::-webkit-scrollbar-thumb {
		background: hsl(var(--muted-foreground) / 0.3);
		border-radius: 2px;
	}

	.overflow-y-auto::-webkit-scrollbar-thumb:hover {
		background: hsl(var(--muted-foreground) / 0.5);
	}
</style>
