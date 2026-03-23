<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import SearchHeroCarousel from '$lib/components/SearchHeroCarousel.svelte';
	import SearchFilters from '$lib/components/search/SearchFilters.svelte';
	import {
		useTrendingAnime,
		useBrowseMedia,
		useAddAnimeToList,
	} from '$lib/hooks/useAnilist.svelte';
	import type { Media, MediaListStatus, BrowseParams } from '$lib/types/anilist';
	import { hscroll } from '$lib/utils/actions';
	import { toast } from 'svelte-sonner';
	import { gsapReveal, gsapStagger } from '$lib/utils/gsap-animations';

	// ── Search & Filter State ────────────────────────────────────────────────
	const urlParams = $derived(page.url.searchParams);

	let searchQuery = $state('');
	let selectedGenre = $state('Any');
	let selectedYear = $state('Any');
	let selectedSeason = $state('Any');
	let selectedFormat = $state('Any');
	let selectedStatus = $state('Any');
	let advancedState = $state<Record<string, string[]>>({});
	let currentPage = $state(1);

	// Sync state from URL
	$effect(() => {
		searchQuery = urlParams.get('search') || '';
		selectedGenre = urlParams.get('genre') || 'Any';
		selectedYear = urlParams.get('year') || 'Any';
		selectedSeason = urlParams.get('season') || 'Any';
		selectedFormat = urlParams.get('format') || 'Any';
		selectedStatus = urlParams.get('status') || 'Any';
		const pageParam = urlParams.get('page');
		currentPage = pageParam ? parseInt(pageParam) : 1;
	});

	// Check if active filters exist
	const hasFilters = $derived(
		searchQuery !== '' ||
			selectedGenre !== 'Any' ||
			selectedYear !== 'Any' ||
			selectedSeason !== 'Any' ||
			selectedFormat !== 'Any' ||
			selectedStatus !== 'Any' ||
			Object.values(advancedState).flat().length > 0
	);

	// Update URL when filters change
	$effect(() => {
		const params = new URLSearchParams();
		if (searchQuery) params.set('search', searchQuery);
		if (selectedGenre !== 'Any') params.set('genre', selectedGenre);
		if (selectedYear !== 'Any') params.set('year', selectedYear);
		if (selectedSeason !== 'Any') params.set('season', selectedSeason);
		if (selectedFormat !== 'Any') params.set('format', selectedFormat);
		if (selectedStatus !== 'Any') params.set('status', selectedStatus);
		if (currentPage > 1) params.set('page', currentPage.toString());
		// Advanced filters could be serialized here if needed, keeping URL clean for now

		const url = `/search/anime${params.toString() ? '?' + params.toString() : ''}`;
		goto(url, { replaceState: true, noScroll: true, keepFocus: true });
	});

	// Build query params
	const browseParams = $derived<BrowseParams>({
		mediaType: 'ANIME',
		search: searchQuery || undefined,
		format: selectedFormat !== 'Any' ? (selectedFormat.replace(/\s+/g, '_') as any) : undefined,
		status: selectedStatus !== 'Any' ? (selectedStatus.replace(/\s+/g, '_') as any) : undefined,
		season: selectedSeason !== 'Any' ? (selectedSeason.toUpperCase() as any) : undefined,
		seasonYear: selectedYear !== 'Any' ? parseInt(selectedYear) : undefined,
		genres: selectedGenre !== 'Any' ? [selectedGenre] : undefined,
		page: currentPage,
		perPage: 30,
		sortBy: ['POPULARITY_DESC'],
	});

	// ── Data Queries ───────────────────────────────────────────────────────────
	const trendingQ = useTrendingAnime({ page: 1, perPage: 20 });
	const trending = $derived<Media[]>(
		Array.isArray(trendingQ.data?.data) ? (trendingQ.data.data as Media[]) : []
	);
	const heroes = $derived(trending.filter((m) => m.bannerImage).slice(0, 8));

	// The filtered results (only enabled when filters are active or when navigating sections)
	const filteredQuery = useBrowseMedia(() => browseParams);
	const filteredMedia = $derived<Media[]>(
		Array.isArray(filteredQuery.data?.data?.data)
			? (filteredQuery.data.data.data as Media[])
			: Array.isArray(filteredQuery.data?.data)
				? (filteredQuery.data.data as Media[])
				: []
	);
	const pageInfo = $derived(filteredQuery.data?.data?.pageInfo);

	// Fallback section queries
	const popularQ = useBrowseMedia(
		() => ({ mediaType: 'ANIME', sortBy: ['POPULARITY_DESC'], page: 1, perPage: 20 }),
		() => ({ enabled: !hasFilters })
	);
	const popular = $derived<Media[]>(
		Array.isArray(popularQ.data?.data?.data)
			? (popularQ.data.data.data as Media[])
			: Array.isArray(popularQ.data?.data)
				? (popularQ.data.data as Media[])
				: []
	);

	const moviesQ = useBrowseMedia(
		() => ({ mediaType: 'ANIME', format: 'MOVIE', sortBy: ['SCORE_DESC'], page: 1, perPage: 20 }),
		() => ({ enabled: !hasFilters })
	);
	const movies = $derived<Media[]>(
		Array.isArray(moviesQ.data?.data?.data)
			? (moviesQ.data.data.data as Media[])
			: Array.isArray(moviesQ.data?.data)
				? (moviesQ.data.data as Media[])
				: []
	);

	// ── Add to List ────────────────────────────────────────────────────────────
	const addToListMutation = useAddAnimeToList();
	const addToListPending = $derived(addToListMutation.isPending);

	const statusOptions: { value: MediaListStatus; label: string; icon: string }[] = [
		{ value: 'PLANNING', label: 'Plan to Watch', icon: 'solar:bookmark-linear' },
		{ value: 'CURRENT', label: 'Currently Watching', icon: 'solar:play-circle-linear' },
		{ value: 'COMPLETED', label: 'Completed', icon: 'solar:check-circle-linear' },
		{ value: 'PAUSED', label: 'On Hold', icon: 'solar:pause-circle-linear' },
		{ value: 'DROPPED', label: 'Dropped', icon: 'solar:close-circle-linear' },
	];

	async function handleAddToList(status: MediaListStatus, item: Media) {
		try {
			await addToListMutation.mutateAsync({ mediaId: item.id, status });
			toast.success(`Added successfully.`);
		} catch {
			toast.error('Failed to add to list');
		}
	}

	function nextPage() {
		if (pageInfo?.hasNextPage) {
			currentPage += 1;
			window.scrollTo({ top: 500, behavior: 'smooth' });
		}
	}

	function prevPage() {
		if (currentPage > 1) {
			currentPage -= 1;
			window.scrollTo({ top: 500, behavior: 'smooth' });
		}
	}
</script>

<div class="flex flex-col gap-0">
	<!-- ── Hero Carousel ────────────────────────────────────────────────────── -->
	{#if !hasFilters && heroes.length}
		<SearchHeroCarousel
			items={heroes}
			mediaType="anime"
			{statusOptions}
			{addToListPending}
			onAddToList={handleAddToList}
		/>
	{/if}

	<!-- ── Search Filters ───────────────────────────────────────────────────── -->
	<div
		class="sticky top-0 z-30 border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
	>
		<SearchFilters
			type="ANIME"
			bind:searchQuery
			bind:selectedGenre
			bind:selectedYear
			bind:selectedSeason
			bind:selectedFormat
			bind:selectedStatus
			bind:advancedState
		/>
	</div>

	<div class="flex flex-col px-6 py-8">
		{#if hasFilters}
			<!-- ── Filtered Results Grid ────────────────────────────────────────── -->
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Search Results</h2>
				<span class="text-sm text-muted-foreground">{pageInfo?.total || 0} items</span>
			</div>

			{#if filteredQuery.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if filteredMedia.length === 0}
				<div class="flex flex-col items-center justify-center py-20">
					<Icon icon="solar:ghost-bold" class="mb-4 h-16 w-16 text-muted-foreground" />
					<h3 class="mb-2 text-xl font-semibold">No results found</h3>
					<p class="text-sm text-muted-foreground">Try adjusting your filters.</p>
				</div>
			{:else}
				<div
					class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7"
					use:gsapReveal
				>
					{#each filteredMedia as media (media.id)}
						<MediaCard {media} />
					{/each}
				</div>

				<div class="mt-8 flex items-center justify-between">
					<Button variant="outline" disabled={currentPage === 1} onclick={prevPage}>
						<Icon icon="solar:alt-arrow-left-bold" class="mr-2 h-4 w-4" />
						Previous
					</Button>
					<div class="text-sm text-muted-foreground">
						Page {pageInfo?.currentPage || 1} of {pageInfo?.lastPage || 1}
					</div>
					<Button variant="outline" disabled={!pageInfo?.hasNextPage} onclick={nextPage}>
						Next
						<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
					</Button>
				</div>
			{/if}
		{:else}
			<!-- ── Default Content Sections ─────────────────────────────────────── -->
			<div class="flex flex-col gap-10">
				<!-- Trending Now -->
				<section id="trending" use:gsapReveal>
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-xl font-bold">Trending Now</h2>
					</div>
					{#if trendingQ.isLoading}
						<div class="flex h-48 items-center justify-center">
							<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
						</div>
					{:else}
						<div use:hscroll>
							<div use:gsapStagger class="flex gap-8 py-4">
								{#each trending as media}
									<div class="w-fit shrink-0">
										<MediaCard {media} />
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</section>

				<!-- All Time Popular -->
				<section id="popular" use:gsapReveal>
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-xl font-bold">All Time Popular</h2>
					</div>
					{#if popularQ.isLoading}
						<div class="flex h-48 items-center justify-center">
							<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
						</div>
					{:else}
						<div use:hscroll>
							<div use:gsapStagger class="flex gap-8 py-4">
								{#each popular as media}
									<div class="w-fit shrink-0">
										<MediaCard {media} />
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</section>

				<!-- Top Movies -->
				<section id="movies" use:gsapReveal>
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-xl font-bold">Top Movies</h2>
					</div>
					{#if moviesQ.isLoading}
						<div class="flex h-48 items-center justify-center">
							<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
						</div>
					{:else if movies.length > 0}
						<div use:hscroll>
							<div use:gsapStagger class="flex gap-8 py-4">
								{#each movies as media}
									<div class="w-fit shrink-0">
										<MediaCard {media} />
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</section>
			</div>
		{/if}
	</div>
</div>
