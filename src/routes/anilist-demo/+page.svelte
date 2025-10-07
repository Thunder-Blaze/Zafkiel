<script lang="ts">
	import { browser } from '$app/environment';
	import { useTrendingAnime, useSearchAnime } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import Icon from '@iconify/svelte';
	import type { Media } from '$lib/types/anilist';

	let searchQuery = $state('');
	let debouncedQuery = $state('');
	let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);

	// Debounce search input
	$effect(() => {
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		debounceTimer = setTimeout(() => {
			debouncedQuery = searchQuery.trim();
		}, 800); // Increased from 500ms to 800ms to reduce API spam

		return () => {
			if (debounceTimer) {
				clearTimeout(debounceTimer);
			}
		};
	});

	// Always fetch trending anime
	const trendingQuery = useTrendingAnime({ page: 1, perPage: 12 });

	// Only fetch search results when there's a query
	const searchQueryResult = useSearchAnime(
		() => ({ query: debouncedQuery, page: 1, perPage: 12 }),
		() => debouncedQuery.length > 0
	);

	// Determine which data to show
	const isSearching = $derived(debouncedQuery.length > 0);

	// Force deep reactivity by accessing each property individually
	// TanStack Query returns store-like objects, and Svelte 5 needs explicit property access
	let trendingData = $state<Media[] | undefined>(undefined);
	let trendingLoading = $state(true);
	let trendingFetching = $state(false);
	let trendingStale = $state(false);
	let trendingError = $state<Error | null>(null);
	let trendingUpdatedAt = $state<number | undefined>(undefined);

	let searchData = $state<Media[] | undefined>(undefined);
	let searchLoading = $state(false);
	let searchFetching = $state(false);
	let searchStale = $state(false);
	let searchError = $state<Error | null>(null);
	let searchUpdatedAt = $state<number | undefined>(undefined);

	// Sync query state to local state (forces reactivity)
	$effect(() => {
		trendingData = trendingQuery.data;
		trendingLoading = trendingQuery.isLoading;
		trendingFetching = trendingQuery.isFetching;
		trendingStale = trendingQuery.isStale;
		trendingError = trendingQuery.error;
		trendingUpdatedAt = trendingQuery.dataUpdatedAt;
	});

	$effect(() => {
		searchData = searchQueryResult.data;
		searchLoading = searchQueryResult.isLoading;
		searchFetching = searchQueryResult.isFetching;
		searchStale = searchQueryResult.isStale;
		searchError = searchQueryResult.error;
		searchUpdatedAt = searchQueryResult.dataUpdatedAt;
	});

	// Extract state from the active query
	const isLoading = $derived(isSearching ? searchLoading : trendingLoading);
	const isFetching = $derived(isSearching ? searchFetching : trendingFetching);
	const isStale = $derived(isSearching ? searchStale : trendingStale);
	const error = $derived(isSearching ? searchError : trendingError);
	const animeData = $derived(isSearching ? searchData : trendingData);
	const dataUpdatedAt = $derived(isSearching ? searchUpdatedAt : trendingUpdatedAt);

	// Debug: Log when data changes
	$effect(() => {
		console.log('[Page] Data changed:', {
			isSearching,
			isLoading,
			isFetching,
			dataCount: animeData?.length ?? 0,
			trendingDataCount: trendingData?.length ?? 0,
			searchDataCount: searchData?.length ?? 0,
		});
	});

	function formatScore(score: number | null): string {
		if (!score) return 'N/A';
		return `${score}%`;
	}

	function formatTitle(media: Media): string {
		return media.title?.english ?? media.title?.romaji ?? media.title?.native ?? 'Unknown';
	}
</script>

{#if browser}
	<div class="container mx-auto space-y-8 p-8">
	<!-- Debug Info -->
	<Card class="border-blue-500/50 bg-blue-500/5">
		<CardHeader>
			<CardTitle class="flex items-center gap-2 text-blue-600 dark:text-blue-400">
				<Icon icon="solar:bug-bold" class="size-5" />
				Debug Info
			</CardTitle>
		</CardHeader>
		<CardContent class="space-y-2 font-mono text-sm">
			<div class="flex justify-between">
				<span class="text-muted-foreground">Search Query:</span>
				<span class="font-semibold">{searchQuery || '(empty)'}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Debounced Query:</span>
				<span class="font-semibold">{debouncedQuery || '(empty)'}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Is Searching:</span>
				<span class="font-semibold">{isSearching}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Active Query:</span>
				<span class="font-semibold">{isSearching ? 'Search' : 'Trending'}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Trending Status:</span>
				<span class="font-semibold">
					{trendingLoading ? '⏳ Loading' : trendingData ? `✓ ${trendingData.length} items` : '❌ No data'}
				</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Search Status:</span>
				<span class="font-semibold">
					{!isSearching ? '⏸️ Disabled' : searchLoading ? '⏳ Loading' : searchData ? `✓ ${searchData.length} items` : '❌ No data'}
				</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Active - Is Loading:</span>
				<span class="font-semibold">{isLoading}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Active - Is Fetching:</span>
				<span class="font-semibold">{isFetching}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Active - Has Error:</span>
				<span class="font-semibold">{!!error}</span>
			</div>
			<div class="flex justify-between">
				<span class="text-muted-foreground">Active - Data Count:</span>
				<span class="font-semibold">{animeData?.length ?? 0}</span>
			</div>
			{#if error}
				<div class="rounded border border-red-500/50 bg-red-500/10 p-2 text-red-600 dark:text-red-400">
					Error: {error.message}
				</div>
			{/if}
		</CardContent>
	</Card>

	<div class="space-y-4">
		<div class="space-y-2">
			<h1 class="text-4xl font-bold">AniList Demo</h1>
			<p class="text-muted-foreground">
				Testing AniList API integration with TanStack Query caching
			</p>
		</div>

		<!-- Search -->
		<div class="flex gap-2">
			<div class="relative flex-1">
				<Icon
					icon="solar:magnifer-bold"
					class="absolute left-3 top-1/2 size-5 -translate-y-1/2 text-muted-foreground"
				/>
				<Input
					bind:value={searchQuery}
					placeholder="Search anime..."
					class="pl-10"
				/>
			</div>
			{#if searchQuery}
				<Button
					variant="outline"
					size="icon"
					onclick={() => {
						searchQuery = '';
					}}
				>
					<Icon icon="solar:close-circle-bold" class="size-5" />
				</Button>
			{/if}
		</div>
	</div>

	<!-- Results -->
	<div class="space-y-4">
		<h2 class="text-2xl font-semibold">
			{debouncedQuery ? `Search Results for "${debouncedQuery}"` : 'Trending Anime'}
		</h2>

		{#if isLoading}
			<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
				{#each Array(12) as _}
					<Card>
						<CardContent class="p-0">
							<Skeleton class="h-64 w-full rounded-t-lg" />
							<div class="space-y-2 p-4">
								<Skeleton class="h-4 w-3/4" />
								<Skeleton class="h-4 w-1/2" />
							</div>
						</CardContent>
					</Card>
				{/each}
			</div>
		{:else if error}
			<Card class="border-destructive">
				<CardHeader>
					<CardTitle class="flex items-center gap-2 text-destructive">
						<Icon icon="solar:danger-triangle-bold" class="size-5" />
						Error
					</CardTitle>
					<CardDescription>
						{error.message}
					</CardDescription>
				</CardHeader>
			</Card>
		{:else if animeData && animeData.length > 0}
			<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
				{#each animeData as anime}
					<Card class="group overflow-hidden transition-all hover:shadow-lg">
						<CardContent class="p-0">
							<!-- Cover Image -->
							{#if anime.coverImage?.large}
								<div class="relative aspect-[2/3] overflow-hidden">
									<img
										src={anime.coverImage.large}
										alt={formatTitle(anime)}
										class="size-full object-cover transition-transform group-hover:scale-105"
									/>
									<!-- Score Badge -->
									{#if anime.averageScore}
										<div
											class="absolute right-2 top-2 flex items-center gap-1 rounded-full bg-background/90 px-2 py-1 text-sm font-semibold backdrop-blur-sm"
										>
											<Icon icon="solar:star-bold" class="size-4 text-yellow-500" />
											{formatScore(anime.averageScore)}
										</div>
									{/if}
								</div>
							{:else}
								<div
									class="flex aspect-[2/3] items-center justify-center bg-muted"
								>
									<Icon icon="solar:gallery-bold" class="size-12 text-muted-foreground" />
								</div>
							{/if}

							<!-- Info -->
							<div class="space-y-2 p-4">
								<h3 class="line-clamp-2 font-semibold leading-tight">
									{formatTitle(anime)}
								</h3>
								<div class="flex items-center gap-2 text-sm text-muted-foreground">
									{#if anime.format}
										<span class="capitalize">{anime.format.replace('_', ' ')}</span>
										<span>•</span>
									{/if}
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
											<span
												class="rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary"
											>
												{genre}
											</span>
										{/each}
									</div>
								{/if}
							</div>
						</CardContent>
					</Card>
				{/each}
			</div>
		{:else}
			<Card>
				<CardHeader>
					<CardTitle class="flex items-center gap-2">
						<Icon icon="solar:inbox-line-bold" class="size-5" />
						No Results
					</CardTitle>
					<CardDescription>
						{debouncedQuery
							? `No anime found for "${debouncedQuery}"`
							: 'No trending anime available'}
					</CardDescription>
				</CardHeader>
			</Card>
		{/if}
	</div>

	<!-- Cache Info -->
	<Card>
		<CardHeader>
			<CardTitle>Cache Information</CardTitle>
			<CardDescription>
				TanStack Query automatically caches and deduplicates requests
			</CardDescription>
		</CardHeader>
		<CardContent class="space-y-2">
			<div class="flex items-center justify-between rounded-lg border p-3">
				<span class="font-medium">Status:</span>
				<span class="flex items-center gap-2">
					{#if isLoading}
						<Icon icon="solar:refresh-bold" class="size-4 animate-spin" />
						Loading
					{:else if isFetching}
						<Icon icon="solar:refresh-bold" class="size-4 animate-spin text-blue-500" />
						Refetching
					{:else if isStale}
						<Icon icon="solar:clock-circle-bold" class="size-4 text-yellow-500" />
						Stale
					{:else}
						<Icon icon="solar:check-circle-bold" class="size-4 text-green-500" />
						Fresh
					{/if}
				</span>
			</div>
			<div class="flex items-center justify-between rounded-lg border p-3">
				<span class="font-medium">Data Age:</span>
				<span>
					{dataUpdatedAt
						? `${Math.floor((Date.now() - dataUpdatedAt) / 1000)}s ago`
						: 'Never fetched'}
				</span>
			</div>
			<div class="flex items-center justify-between rounded-lg border p-3">
				<span class="font-medium">Stale Time:</span>
				<span>{debouncedQuery ? '10 minutes (search)' : '5 minutes (trending)'}</span>
			</div>
		</CardContent>
	</Card>
	</div>
{:else}
	<div class="container mx-auto flex min-h-screen items-center justify-center p-8">
		<Card>
			<CardHeader>
				<CardTitle>Loading...</CardTitle>
				<CardDescription>Initializing AniList demo</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="flex items-center gap-2">
					<Icon icon="solar:refresh-bold" class="size-5 animate-spin" />
					<span>Please wait</span>
				</div>
			</CardContent>
		</Card>
	</div>
{/if}
