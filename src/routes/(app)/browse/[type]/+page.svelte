<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { useBrowseMedia } from '$lib/hooks/useAnilist.svelte';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import HeroBannerCarousel from '$lib/components/HeroBannerCarousel.svelte';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Carousel from '$lib/components/ui/carousel';
	import type {
		BrowseParams,
		MediaFormat,
		MediaStatus,
		MediaSeason,
		MediaSort,
		MediaSource,
	} from '$lib/types/anilist';

	// Get route params
	const routeType = $derived((page.params as { type?: string }).type);
	const type = $derived(routeType?.toUpperCase() as 'ANIME' | 'MANGA' | undefined);

	// Filter state
	let headerSearchQuery = $state(''); // Search in header (redirects to /search)
	let searchQuery = $state(''); // Search in filters panel
	let selectedFormat = $state<MediaFormat | ''>('');
	let selectedStatus = $state<MediaStatus | ''>('');
	let selectedSeason = $state<MediaSeason | ''>('');
	let selectedYear = $state('');
	let selectedSource = $state<MediaSource | ''>('');
	let selectedGenres = $state<string[]>([]);
	let excludedGenres = $state<string[]>([]);
	let sortBy = $state<MediaSort>('POPULARITY_DESC');
	let currentPage = $state<number>(1);
	let showAdult = $state<boolean>(false);
	let showFilters = $state(false);

	// Handle header search submission
	function handleHeaderSearch(e: Event) {
		e.preventDefault();
		if (headerSearchQuery.trim()) {
			goto(`/search?type=${type}&search=${encodeURIComponent(headerSearchQuery.trim())}`);
		}
	}

	// Build browse params
	const browseParams = $derived<BrowseParams>({
		mediaType: type === 'ANIME' || type === 'MANGA' ? type : undefined,
		search: searchQuery || undefined,
		format: selectedFormat || undefined,
		status: selectedStatus || undefined,
		season: selectedSeason || undefined,
		seasonYear: selectedYear ? parseInt(selectedYear) : undefined,
		source: selectedSource || undefined,
		genres: selectedGenres.filter(g => g).length > 0 ? selectedGenres.filter(g => g) : undefined,
		genresExcluded: excludedGenres.filter(g => g).length > 0 ? excludedGenres.filter(g => g) : undefined,
		sortBy: sortBy !== 'POPULARITY_DESC' ? [sortBy] : undefined,
		isAdult: showAdult ? true : undefined,
		page: currentPage,
		perPage: 50,
	});

	// Fetch data using the hook
	const query = $derived(useBrowseMedia(browseParams));
	const mediaList = $derived(query.data?.data?.data || []);
	const pageInfo = $derived(query.data?.data?.pageInfo);

	// Section queries (only when no filters applied)
	const hasFilters = $derived(
		searchQuery || selectedFormat || selectedStatus || selectedSeason ||
		selectedYear || selectedSource || selectedGenres.filter(g => g).length > 0 ||
		excludedGenres.filter(g => g).length > 0 || sortBy !== 'POPULARITY_DESC'
	);

	// Get current season
	const getCurrentSeason = (): MediaSeason => {
		const month = new Date().getMonth() + 1;
		if (month >= 1 && month <= 3) return 'WINTER';
		if (month >= 4 && month <= 6) return 'SPRING';
		if (month >= 7 && month <= 9) return 'SUMMER';
		return 'FALL';
	};

	const getNextSeason = (): { season: MediaSeason; year: number } => {
		const currentMonth = new Date().getMonth() + 1;
		const currentYear = new Date().getFullYear();
		if (currentMonth >= 10) return { season: 'WINTER', year: currentYear + 1 };
		if (currentMonth >= 7) return { season: 'FALL', year: currentYear };
		if (currentMonth >= 4) return { season: 'SUMMER', year: currentYear };
		return { season: 'SPRING', year: currentYear };
	};

	const currentSeason = getCurrentSeason();
	const currentYear = new Date().getFullYear();
	const nextSeason = getNextSeason();

	// Trending query
	const trendingQuery = $derived(
		!hasFilters ? useBrowseMedia({
			mediaType: type,
			sortBy: ['TRENDING_DESC'],
			page: 1,
			perPage: 20,
		}) : null
	);

	// Popular This Season query
	const popularSeasonQuery = $derived(
		!hasFilters && type === 'ANIME' ? useBrowseMedia({
			mediaType: 'ANIME',
			season: currentSeason,
			seasonYear: currentYear,
			sortBy: ['POPULARITY_DESC'],
			page: 1,
			perPage: 20,
		}) : null
	);

	// Upcoming Next Season query
	const upcomingQuery = $derived(
		!hasFilters && type === 'ANIME' ? useBrowseMedia({
			mediaType: 'ANIME',
			season: nextSeason.season,
			seasonYear: nextSeason.year,
			sortBy: ['POPULARITY_DESC'],
			page: 1,
			perPage: 20,
		}) : null
	);

	// All Time Popular query
	const allTimePopularQuery = $derived(
		!hasFilters ? useBrowseMedia({
			mediaType: type,
			sortBy: ['POPULARITY_DESC'],
			page: 1,
			perPage: 20,
		}) : null
	);

	// Top Rated query
	const topRatedQuery = $derived(
		!hasFilters ? useBrowseMedia({
			mediaType: type,
			sortBy: ['SCORE_DESC'],
			page: 1,
			perPage: 20,
		}) : null
	);

	// Available options for filters
	const formats: MediaFormat[] = [
		'TV',
		'TV_SHORT',
		'MOVIE',
		'SPECIAL',
		'OVA',
		'ONA',
		'MUSIC',
		'MANGA',
		'NOVEL',
		'ONE_SHOT',
	];

	const statuses: MediaStatus[] = [
		'FINISHED',
		'RELEASING',
		'NOT_YET_RELEASED',
		'CANCELLED',
		'HIATUS',
	];

	const seasons: MediaSeason[] = ['WINTER', 'SPRING', 'SUMMER', 'FALL'];

	const sources: MediaSource[] = [
		'ORIGINAL',
		'MANGA',
		'LIGHT_NOVEL',
		'VISUAL_NOVEL',
		'VIDEO_GAME',
		'OTHER',
		'NOVEL',
		'DOUJINSHI',
		'ANIME',
		'WEB_NOVEL',
		'LIVE_ACTION',
		'GAME',
		'COMIC',
		'MULTIMEDIA_PROJECT',
		'PICTURE_BOOK',
	];

	const allGenres = [
		'Action',
		'Adventure',
		'Comedy',
		'Drama',
		'Ecchi',
		'Fantasy',
		'Horror',
		'Mahou Shoujo',
		'Mecha',
		'Music',
		'Mystery',
		'Psychological',
		'Romance',
		'Sci-Fi',
		'Slice of Life',
		'Sports',
		'Supernatural',
		'Thriller',
	];

	const sortOptions: { label: string; value: MediaSort }[] = [
		{ label: 'Popularity (High to Low)', value: 'POPULARITY_DESC' },
		{ label: 'Popularity (Low to High)', value: 'POPULARITY' },
		{ label: 'Trending (High to Low)', value: 'TRENDING_DESC' },
		{ label: 'Trending (Low to High)', value: 'TRENDING' },
		{ label: 'Score (High to Low)', value: 'SCORE_DESC' },
		{ label: 'Score (Low to High)', value: 'SCORE' },
		{ label: 'Title (A-Z)', value: 'TITLE_ROMAJI' },
		{ label: 'Title (Z-A)', value: 'TITLE_ROMAJI_DESC' },
		{ label: 'Start Date (Newest)', value: 'START_DATE_DESC' },
		{ label: 'Start Date (Oldest)', value: 'START_DATE' },
		{ label: 'Favorites (High to Low)', value: 'FAVOURITES_DESC' },
		{ label: 'Favorites (Low to High)', value: 'FAVOURITES' },
	];

	// Generate years from 1940 to current year + 1
	const years = Array.from({ length: currentYear - 1939 + 2 }, (_, i) => currentYear + 1 - i);

	function toggleGenre(genre: string, excluded: boolean = false) {
		if (excluded) {
			if (excludedGenres.includes(genre)) {
				excludedGenres = excludedGenres.filter((g) => g !== genre);
			} else {
				excludedGenres = [...excludedGenres, genre];
				// Remove from included if adding to excluded
				selectedGenres = selectedGenres.filter((g) => g !== genre);
			}
		} else {
			if (selectedGenres.includes(genre)) {
				selectedGenres = selectedGenres.filter((g) => g !== genre);
			} else {
				selectedGenres = [...selectedGenres, genre];
				// Remove from excluded if adding to included
				excludedGenres = excludedGenres.filter((g) => g !== genre);
			}
		}
		currentPage = 1; // Reset to page 1 when filters change
	}

	function clearFilters() {
		searchQuery = '';
		selectedFormat = '';
		selectedStatus = '';
		selectedSeason = '';
		selectedYear = '';
		selectedSource = '';
		selectedGenres = [];
		excludedGenres = [];
		sortBy = 'POPULARITY_DESC';
		currentPage = 1;
	}

	function nextPage() {
		if (pageInfo?.hasNextPage) {
			currentPage += 1;
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function prevPage() {
		if (currentPage > 1) {
			currentPage -= 1;
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}
</script>

<div class="h-full w-full">
	<!-- Hero Banner Carousel (only show when no filters applied) -->
	{#if !hasFilters && trendingQuery?.data?.data?.data}
		<HeroBannerCarousel items={trendingQuery.data.data.data.slice(0, 5)} />
	{/if}

	<div class="py-6">
		<!-- Search and Filters Bar (Horizontal Layout) -->
		<div class="mb-6 px-6">
			<div class="flex flex-wrap items-center gap-3">
				<!-- Search -->
				<form onsubmit={handleHeaderSearch} class="relative flex-1 min-w-[200px]">
					<input
						type="text"
						placeholder="Search"
						bind:value={headerSearchQuery}
						class="w-full rounded-lg border border-border bg-background px-4 py-2 pl-10 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
					/>
					<Icon icon="solar:magnifer-bold" class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
				</form>

				<!-- Genres -->
				<select
					bind:value={selectedGenres[0]}
					onchange={() => { if (selectedGenres[0]) selectedGenres = [selectedGenres[0]]; else selectedGenres = []; currentPage = 1; }}
					class="rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				>
					<option value="">Genres</option>
					{#each allGenres as genre}
						<option value={genre}>{genre}</option>
					{/each}
				</select>

				<!-- Year -->
				<select
					bind:value={selectedYear}
					onchange={() => (currentPage = 1)}
					class="rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				>
					<option value="">Year</option>
					{#each years.slice(0, 20) as year}
						<option value={year.toString()}>{year}</option>
					{/each}
				</select>

				<!-- Season -->
				<select
					bind:value={selectedSeason}
					onchange={() => (currentPage = 1)}
					class="rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				>
					<option value="">Season</option>
					{#each seasons as season}
						<option value={season}>{season.charAt(0) + season.slice(1).toLowerCase()}</option>
					{/each}
				</select>

				<!-- Format -->
				<select
					bind:value={selectedFormat}
					onchange={() => (currentPage = 1)}
					class="rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				>
					<option value="">Format</option>
					{#each formats as format}
						<option value={format}>{format.replace(/_/g, ' ')}</option>
					{/each}
				</select>

				<!-- Status -->
				<select
					bind:value={selectedStatus}
					onchange={() => (currentPage = 1)}
					class="rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				>
					<option value="">Airing Status</option>
					{#each statuses as status}
						<option value={status}>{status.replace(/_/g, ' ')}</option>
					{/each}
				</select>

				<!-- Advanced Filters Toggle -->
				<Button variant="outline" size="sm" onclick={() => (showFilters = !showFilters)}>
					<Icon icon="solar:filter-bold" class="h-4 w-4" />
				</Button>
			</div>
		</div>

	<!-- Carousel Sections (only show when no filters applied) -->
	{#if !hasFilters}
		<!-- Trending Now -->
		<section class="mb-8">
			<div class="mb-4 flex items-center justify-between px-6">
				<h2 class="text-2xl font-bold">Trending Now</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto(`/search?type=${type}&sort=TRENDING_DESC`)}
				>
					View All
					<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
				</Button>
			</div>
			{#if trendingQuery?.isLoading}
				<div class="flex items-center justify-center py-10">
					<Icon icon="svg-spinners:3-dots-scale" class="h-8 w-8 text-primary" />
				</div>
			{:else if trendingQuery?.data?.data?.data}
				<Carousel.Root class="w-full px-6 overflow-visible">
					<Carousel.Content class="-ml-4">
						{#each trendingQuery.data.data.data as media (media.id)}
							<Carousel.Item class="basis-1/2 pl-4 sm:basis-1/3 md:basis-1/4 lg:basis-1/5 xl:basis-1/6">
								<MediaCard {media} />
							</Carousel.Item>
						{/each}
					</Carousel.Content>
					<Carousel.Previous class="left-2" />
					<Carousel.Next class="right-2" />
				</Carousel.Root>
			{/if}
		</section>

		<!-- Popular This Season (Anime only) -->
		{#if type === 'ANIME'}
			<section class="mb-8">
				<div class="mb-4 flex items-center justify-between px-6">
					<h2 class="text-2xl font-bold">Popular This Season</h2>
					<Button
						variant="ghost"
						size="sm"
						onclick={() => goto(`/search?type=ANIME&season=${currentSeason}&year=${currentYear}&sort=POPULARITY_DESC`)}
					>
						View All
						<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
					</Button>
				</div>
				{#if popularSeasonQuery?.isLoading}
					<div class="flex items-center justify-center py-10">
						<Icon icon="svg-spinners:3-dots-scale" class="h-8 w-8 text-primary" />
					</div>
				{:else if popularSeasonQuery?.data?.data?.data}
					<Carousel.Root class="w-full px-6">
						<Carousel.Content class="-ml-4">
							{#each popularSeasonQuery.data.data.data as media (media.id)}
								<Carousel.Item class="basis-1/2 pl-4 sm:basis-1/3 md:basis-1/4 lg:basis-1/5 xl:basis-1/6">
									<MediaCard {media} />
								</Carousel.Item>
							{/each}
						</Carousel.Content>
						<Carousel.Previous class="left-2" />
						<Carousel.Next class="right-2" />
					</Carousel.Root>
				{/if}
			</section>

			<!-- Upcoming Next Season -->
			<section class="mb-8">
				<div class="mb-4 flex items-center justify-between px-6">
					<h2 class="text-2xl font-bold">Upcoming {nextSeason.season.charAt(0) + nextSeason.season.slice(1).toLowerCase()} {nextSeason.year}</h2>
					<Button
						variant="ghost"
						size="sm"
						onclick={() => goto(`/search?type=ANIME&season=${nextSeason.season}&year=${nextSeason.year}&sort=POPULARITY_DESC`)}
					>
						View All
						<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
					</Button>
				</div>
				{#if upcomingQuery?.isLoading}
					<div class="flex items-center justify-center py-10">
						<Icon icon="svg-spinners:3-dots-scale" class="h-8 w-8 text-primary" />
					</div>
				{:else if upcomingQuery?.data?.data?.data}
					<Carousel.Root class="w-full px-6">
						<Carousel.Content class="-ml-4">
							{#each upcomingQuery.data.data.data as media (media.id)}
								<Carousel.Item class="basis-1/2 pl-4 sm:basis-1/3 md:basis-1/4 lg:basis-1/5 xl:basis-1/6">
									<MediaCard {media} />
								</Carousel.Item>
							{/each}
						</Carousel.Content>
						<Carousel.Previous class="left-2" />
						<Carousel.Next class="right-2" />
					</Carousel.Root>
				{/if}
			</section>
		{/if}

		<!-- All Time Popular -->
		<section class="mb-8">
			<div class="mb-4 flex items-center justify-between px-6">
				<h2 class="text-2xl font-bold">All Time Popular</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto(`/search?type=${type}&sort=POPULARITY_DESC`)}
				>
					View All
					<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
				</Button>
			</div>
			{#if allTimePopularQuery?.isLoading}
				<div class="flex items-center justify-center py-10">
					<Icon icon="svg-spinners:3-dots-scale" class="h-8 w-8 text-primary" />
				</div>
			{:else if allTimePopularQuery?.data?.data?.data}
				<Carousel.Root class="w-full px-6">
					<Carousel.Content class="-ml-4">
						{#each allTimePopularQuery.data.data.data as media (media.id)}
							<Carousel.Item class="basis-1/2 pl-4 sm:basis-1/3 md:basis-1/4 lg:basis-1/5 xl:basis-1/6">
								<MediaCard {media} />
							</Carousel.Item>
						{/each}
					</Carousel.Content>
					<Carousel.Previous class="left-2" />
					<Carousel.Next class="right-2" />
				</Carousel.Root>
			{/if}
		</section>

		<!-- Top Rated -->
		<section class="mb-8">
			<div class="mb-4 flex items-center justify-between px-6">
				<h2 class="text-2xl font-bold">Top Rated</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto(`/search?type=${type}&sort=SCORE_DESC`)}
				>
					View All
					<Icon icon="solar:alt-arrow-right-bold" class="ml-2 h-4 w-4" />
				</Button>
			</div>
			{#if topRatedQuery?.isLoading}
				<div class="flex items-center justify-center py-10">
					<Icon icon="svg-spinners:3-dots-scale" class="h-8 w-8 text-primary" />
				</div>
			{:else if topRatedQuery?.data?.data?.data}
				<Carousel.Root class="w-full px-6">
					<Carousel.Content class="-ml-4">
						{#each topRatedQuery.data.data.data as media (media.id)}
							<Carousel.Item class="basis-1/2 pl-4 sm:basis-1/3 md:basis-1/4 lg:basis-1/5 xl:basis-1/6">
								<MediaCard {media} />
							</Carousel.Item>
						{/each}
					</Carousel.Content>
					<Carousel.Previous class="left-2" />
					<Carousel.Next class="right-2" />
				</Carousel.Root>
			{/if}
		</section>
	{/if}

	<!-- Filters Panel -->
	{#if showFilters}
		<div class="mb-6 rounded-lg border border-border bg-card p-6 mx-6">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-semibold">Filters</h2>
				<Button variant="ghost" size="sm" onclick={clearFilters}>
					<Icon icon="solar:restart-bold" class="mr-2 h-4 w-4" />
					Clear All
				</Button>
			</div>

			<div class="space-y-6">
				<!-- Search -->
				<div>
					<label for="search" class="mb-2 block text-sm font-medium">Search</label>
					<input
						id="search"
						type="text"
						placeholder="Search by title..."
						bind:value={searchQuery}
						oninput={() => (currentPage = 1)}
						class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
					/>
				</div>

				<!-- Sort, Format, Status -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<div>
						<label for="sort" class="mb-2 block text-sm font-medium">Sort By</label>
						<select
							id="sort"
							bind:value={sortBy}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							{#each sortOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="format" class="mb-2 block text-sm font-medium">Format</label>
						<select
							id="format"
							bind:value={selectedFormat}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">All Formats</option>
							{#each formats as format}
								<option value={format}>{format.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="status" class="mb-2 block text-sm font-medium">Status</label>
						<select
							id="status"
							bind:value={selectedStatus}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">All Status</option>
							{#each statuses as status}
								<option value={status}>{status.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Season, Year, Source -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<div>
						<label for="season" class="mb-2 block text-sm font-medium">Season</label>
						<select
							id="season"
							bind:value={selectedSeason}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">All Seasons</option>
							{#each seasons as season}
								<option value={season}>{season}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="year" class="mb-2 block text-sm font-medium">Year</label>
						<select
							id="year"
							bind:value={selectedYear}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">All Years</option>
							{#each years as year}
								<option value={year.toString()}>{year}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="source" class="mb-2 block text-sm font-medium">Source</label>
						<select
							id="source"
							bind:value={selectedSource}
							onchange={() => (currentPage = 1)}
							class="w-full rounded-lg border border-border bg-background px-4 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">All Sources</option>
							{#each sources as source}
								<option value={source}>{source.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Genres -->
				<div>
					<span class="mb-2 block text-sm font-medium">Genres</span>
					<div class="flex flex-wrap gap-2">
						{#each allGenres as genre}
							{@const isIncluded = selectedGenres.includes(genre)}
							{@const isExcluded = excludedGenres.includes(genre)}
							<button
								type="button"
								onclick={() => {
									if (!isIncluded && !isExcluded) {
										toggleGenre(genre, false);
									} else if (isIncluded) {
										toggleGenre(genre, true);
									} else {
										toggleGenre(genre, true);
									}
								}}
								class="rounded-full border-2 px-4 py-2 text-sm font-medium transition-all duration-200 hover:scale-105
								{isIncluded
									? 'border-primary bg-primary/20 text-primary'
									: isExcluded
										? 'border-destructive bg-destructive/20 text-destructive line-through'
										: 'border-border bg-muted text-muted-foreground hover:border-primary/50'}"
							>
								{genre}
							</button>
						{/each}
					</div>
					<p class="mt-2 text-xs text-muted-foreground">
						Click once to include, twice to exclude, three times to remove
					</p>
				</div>

				<!-- Adult Content -->
				<div class="flex items-center gap-2">
					<input
						type="checkbox"
						id="adult"
						bind:checked={showAdult}
						onchange={() => (currentPage = 1)}
						class="h-4 w-4 rounded border-border"
					/>
					<label for="adult" class="text-sm font-medium">Show adult content</label>
				</div>
			</div>
		</div>
	{/if}

	<!-- Filtered Results (only show when filters applied) -->
	{#if hasFilters}
		<!-- Loading/Error/Empty States -->
		{#if query.isLoading}
		<div class="flex items-center justify-center py-20">
			<Icon icon="svg-spinners:3-dots-scale" class="h-12 w-12 text-primary" />
		</div>
	{:else if query.isError}
		<div class="flex flex-col items-center justify-center py-20">
			<Icon icon="solar:danger-circle-bold" class="mb-4 h-16 w-16 text-destructive" />
			<h3 class="mb-2 text-xl font-semibold">Error loading media</h3>
			<p class="text-sm text-muted-foreground">{query.error?.message || 'Unknown error'}</p>
		</div>
	{:else if mediaList.length === 0}
		<div class="flex flex-col items-center justify-center py-20">
			<Icon icon="solar:ghost-bold" class="mb-4 h-16 w-16 text-muted-foreground" />
			<h3 class="mb-2 text-xl font-semibold">No results found</h3>
			<p class="text-sm text-muted-foreground">Try adjusting your filters</p>
		</div>
	{:else}
		<!-- Media Grid -->
		<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 px-6">
			{#each mediaList as media (media.id)}
				<MediaCard {media} />
			{/each}
		</div>

		<!-- Pagination -->
		<div class="mt-8 flex items-center justify-between px-6">
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
	{/if}
	</div>
</div>
