<script lang="ts">
	import { useTrendingAnime, useSearchAnime } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import Icon from '@iconify/svelte';
	import type { Media } from '$lib/types/anilist';

	// Search state
	let searchQuery = $state('');
	let debouncedQuery = $state('');

	// Debounce search with 800ms delay
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	$effect(() => {
		if (debounceTimer) clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			debouncedQuery = searchQuery.trim();
		}, 800);
		return () => {
			if (debounceTimer) clearTimeout(debounceTimer);
		};
	});

	// Fetch queries
	const isSearchActive = $derived(debouncedQuery.length > 0);
	
	const trendingQuery = useTrendingAnime({ page: 1, perPage: 20 });
	const searchQueryResult = useSearchAnime(
		() => ({ query: debouncedQuery, page: 1, perPage: 20 }),
		() => isSearchActive
	);

	// Extract reactive state - the key is to create separate $derived for each property
	// This ensures Svelte tracks changes to individual properties
	const activeData = $derived<Media[] | undefined>(
		isSearchActive ? searchQueryResult.data : trendingQuery.data
	);
	const activeIsLoading = $derived(
		isSearchActive ? searchQueryResult.isLoading : trendingQuery.isLoading
	);
	const activeIsFetching = $derived(
		isSearchActive ? searchQueryResult.isFetching : trendingQuery.isFetching
	);
	const activeError = $derived(
		isSearchActive ? searchQueryResult.error : trendingQuery.error
	);

	// Debug logging
	$effect(() => {
		console.log('[Anime Page] State:', {
			isSearchActive,
			activeIsLoading,
			activeIsFetching,
			dataCount: activeData?.length ?? 0
		});
	});

	// Helper functions
	function getTitle(anime: any): string {
		return anime.title?.english || anime.title?.romaji || anime.title?.native || 'Unknown';
	}

	function formatScore(score: number | null): string {
		return score ? `${score}%` : 'N/A';
	}
</script>

<div class="container mx-auto space-y-6 p-8">
	<!-- Header -->
	<div class="space-y-2">
		<h1 class="text-3xl font-bold">Anime Browser</h1>
		<p class="text-muted-foreground">
			Explore trending anime or search for your favorites
		</p>
	</div>

	<!-- Search Bar -->
	<div class="flex gap-2">
		<div class="relative flex-1">
			<Icon
				icon="solar:magnifer-bold"
				class="absolute left-3 top-1/2 size-5 -translate-y-1/2 text-muted-foreground"
			/>
			<Input
				type="text"
				placeholder="Search anime..."
				class="pl-10 pr-10"
				bind:value={searchQuery}
			/>
			{#if searchQuery}
				<Button
					variant="ghost"
					size="icon"
					class="absolute right-1 top-1/2 size-7 -translate-y-1/2"
					onclick={() => (searchQuery = '')}
				>
					<Icon icon="solar:close-circle-bold" class="size-4" />
				</Button>
			{/if}
		</div>
	</div>

	<!-- Results Header -->
	<div class="flex items-center justify-between">
		<h2 class="text-xl font-semibold">
			{isSearchActive ? `Search: "${debouncedQuery}"` : 'Trending Anime'}
		</h2>
		<div class="flex items-center gap-2 text-sm text-muted-foreground">
			{#if activeIsFetching}
				<Icon icon="solar:refresh-bold" class="size-4 animate-spin" />
				<span>Loading...</span>
			{:else if activeData}
				<span>{activeData.length} results</span>
			{/if}
		</div>
	</div>

	<!-- Loading State -->
	{#if activeIsLoading}
		<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
			{#each Array(20) as _}
				<Card class="overflow-hidden">
					<Skeleton class="aspect-[2/3] w-full" />
					<CardContent class="space-y-2 p-4">
						<Skeleton class="h-4 w-3/4" />
						<Skeleton class="h-3 w-1/2" />
					</CardContent>
				</Card>
			{/each}
		</div>
	<!-- Error State -->
	{:else if activeError}
		<Card class="border-destructive">
			<CardHeader>
				<CardTitle class="flex items-center gap-2 text-destructive">
					<Icon icon="solar:danger-triangle-bold" class="size-5" />
					Error Loading Anime
				</CardTitle>
				<CardDescription>{activeError.message}</CardDescription>
			</CardHeader>
		</Card>
	<!-- Empty State -->
	{:else if !activeData || activeData.length === 0}
		<Card>
			<CardContent class="flex flex-col items-center justify-center py-12">
				<Icon icon="solar:box-minimalistic-bold" class="mb-4 size-16 text-muted-foreground" />
				<p class="text-lg font-medium">No anime found</p>
				<p class="text-sm text-muted-foreground">
					{isSearchActive ? 'Try a different search term' : 'No trending anime available'}
				</p>
			</CardContent>
		</Card>
	<!-- Anime Grid -->
	{:else}
		<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
			{#each activeData as anime (anime.id)}
				<Card class="group overflow-hidden transition-all hover:shadow-lg">
					<!-- Cover Image -->
					<div class="relative aspect-[2/3] overflow-hidden bg-muted">
						{#if anime.coverImage?.large}
							<img
								src={anime.coverImage.large}
								alt={getTitle(anime)}
								class="size-full object-cover transition-transform group-hover:scale-105"
							/>
						{/if}
						
						<!-- Score Badge -->
						{#if anime.averageScore}
							<div class="absolute right-2 top-2 rounded-full bg-black/70 px-2 py-1 text-xs font-semibold text-white backdrop-blur-sm">
								⭐ {formatScore(anime.averageScore)}
							</div>
						{/if}

						<!-- Format Badge -->
						{#if anime.format}
							<div class="absolute bottom-2 left-2 rounded-full bg-primary/90 px-2 py-1 text-xs font-medium text-primary-foreground backdrop-blur-sm">
								{anime.format}
							</div>
						{/if}
					</div>

					<!-- Info -->
					<CardContent class="space-y-1 p-4">
						<h3 class="line-clamp-2 font-semibold leading-tight" title={getTitle(anime)}>
							{getTitle(anime)}
						</h3>
						<div class="flex items-center gap-2 text-xs text-muted-foreground">
							{#if anime.seasonYear}
								<span>{anime.seasonYear}</span>
							{/if}
							{#if anime.episodes}
								<span>•</span>
								<span>{anime.episodes} eps</span>
							{/if}
						</div>
						{#if anime.genres && anime.genres.length > 0}
							<div class="flex flex-wrap gap-1">
								{#each anime.genres.slice(0, 3) as genre}
									<span class="rounded-full bg-muted px-2 py-0.5 text-xs">
										{genre}
									</span>
								{/each}
							</div>
						{/if}
					</CardContent>
				</Card>
			{/each}
		</div>
	{/if}
</div>
