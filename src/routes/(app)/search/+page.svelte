<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { useBrowseMedia } from '$lib/hooks/useAnilist.svelte';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import type {
		BrowseParams,
		MediaFormat,
		MediaStatus,
		MediaSeason,
		MediaSort,
		MediaSource,
	} from '$lib/types/anilist';

	// Get URL search params
	const urlParams = $derived(page.url.searchParams);

	// Filter state
	let searchQuery = $state('');
	let selectedType = $state<'ANIME' | 'MANGA' | ''>('');
	let selectedFormat = $state<MediaFormat | ''>('');
	let selectedStatus = $state<MediaStatus | ''>('');
	let selectedSeason = $state<MediaSeason | ''>('');
	let selectedYear = $state('');
	let selectedSource = $state<MediaSource | ''>('');
	let selectedGenres = $state<string[]>([]);
	let excludedGenres = $state<string[]>([]);
	let sortBy = $state<MediaSort>('POPULARITY_DESC');
	let currentPage = $state<number>(1);

	// Sync URL params to state
	$effect(() => {
		searchQuery = urlParams.get('search') || '';
		selectedType = (urlParams.get('type')?.toUpperCase() as 'ANIME' | 'MANGA') || '';
		selectedFormat = (urlParams.get('format') as MediaFormat) || '';
		selectedStatus = (urlParams.get('status') as MediaStatus) || '';
		selectedSeason = (urlParams.get('season') as MediaSeason) || '';
		selectedYear = urlParams.get('year') || '';
		selectedSource = (urlParams.get('source') as MediaSource) || '';
		selectedGenres = urlParams.get('genres')?.split(',').filter(Boolean) || [];
		excludedGenres = urlParams.get('excludedGenres')?.split(',').filter(Boolean) || [];
		sortBy = (urlParams.get('sort') as MediaSort) || 'POPULARITY_DESC';
		currentPage = parseInt(urlParams.get('page') || '1');
	});

	// Build browse params
	const browseParams = $derived<BrowseParams>({
		mediaType: selectedType || undefined,
		search: searchQuery || undefined,
		format: selectedFormat || undefined,
		status: selectedStatus || undefined,
		season: selectedSeason || undefined,
		seasonYear: selectedYear ? parseInt(selectedYear) : undefined,
		source: selectedSource || undefined,
		genres: selectedGenres.length > 0 ? selectedGenres : undefined,
		genresExcluded: excludedGenres.length > 0 ? excludedGenres : undefined,
		sortBy: [sortBy],
		page: currentPage,
		perPage: 20,
	});

	// Update URL when filters change
	function updateURL() {
		const params = new URLSearchParams();
		if (searchQuery) params.set('search', searchQuery);
		if (selectedType) params.set('type', selectedType);
		if (selectedFormat) params.set('format', selectedFormat);
		if (selectedStatus) params.set('status', selectedStatus);
		if (selectedSeason) params.set('season', selectedSeason);
		if (selectedYear) params.set('year', selectedYear);
		if (selectedSource) params.set('source', selectedSource);
		if (selectedGenres.length > 0) params.set('genres', selectedGenres.join(','));
		if (excludedGenres.length > 0) params.set('excludedGenres', excludedGenres.join(','));
		if (sortBy !== 'POPULARITY_DESC') params.set('sort', sortBy);
		if (currentPage > 1) params.set('page', currentPage.toString());

		const newUrl = `/search${params.toString() ? '?' + params.toString() : ''}`;
		goto(newUrl, { replaceState: true, noScroll: true });
	}

	// Fetch data using the hook
	const query = $derived(useBrowseMedia(browseParams));
	const mediaList = $derived(query.data?.data?.data || []);
	const pageInfo = $derived(query.data?.data?.pageInfo);

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

	// Generate years from 1940 to current year + 1
	const currentYear = new Date().getFullYear();
	const years = Array.from({ length: currentYear - 1939 + 2 }, (_, i) => currentYear + 1 - i);

	function toggleGenre(genre: string, excluded: boolean = false) {
		if (excluded) {
			if (excludedGenres.includes(genre)) {
				excludedGenres = excludedGenres.filter((g) => g !== genre);
			} else {
				excludedGenres = [...excludedGenres, genre];
				selectedGenres = selectedGenres.filter((g) => g !== genre);
			}
		} else {
			if (selectedGenres.includes(genre)) {
				selectedGenres = selectedGenres.filter((g) => g !== genre);
			} else {
				selectedGenres = [...selectedGenres, genre];
				excludedGenres = excludedGenres.filter((g) => g !== genre);
			}
		}
		currentPage = 1;
		updateURL();
	}

	function handleSearch(e: Event) {
		e.preventDefault();
		currentPage = 1;
		updateURL();
	}

	function handleFilterChange() {
		currentPage = 1;
		updateURL();
	}

	function clearFilters() {
		searchQuery = '';
		selectedType = '';
		selectedFormat = '';
		selectedStatus = '';
		selectedSeason = '';
		selectedYear = '';
		selectedSource = '';
		selectedGenres = [];
		excludedGenres = [];
		sortBy = 'POPULARITY_DESC';
		currentPage = 1;
		goto('/search', { replaceState: true });
	}

	function nextPage() {
		if (pageInfo?.hasNextPage) {
			currentPage += 1;
			updateURL();
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function prevPage() {
		if (currentPage > 1) {
			currentPage -= 1;
			updateURL();
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}
</script>

<div class="h-full w-full">
	<!-- Search Bar Header (AniList Style) -->
	<div
		class="border-b border-border bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60"
	>
		<div class="flex items-center gap-4 px-6 py-4">
			<!-- Search Input -->
			<form onsubmit={handleSearch} class="flex-1">
				<div class="relative">
					<Icon
						icon="solar:magnifer-bold"
						class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-muted-foreground"
					/>
					<input
						type="text"
						placeholder="Search anime, manga..."
						bind:value={searchQuery}
						class="w-full rounded-lg border border-border bg-background py-2.5 pr-4 pl-10 text-sm transition-colors outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
					/>
				</div>
			</form>

			<!-- Type Filter Buttons -->
			<div class="flex gap-2">
				<button
					type="button"
					onclick={() => {
						selectedType = '';
						handleFilterChange();
					}}
					class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors
						{selectedType === ''
						? 'border-primary bg-primary text-primary-foreground'
						: 'border-border bg-background hover:bg-accent'}"
				>
					Any
				</button>
				<button
					type="button"
					onclick={() => {
						selectedType = 'ANIME';
						handleFilterChange();
					}}
					class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors
						{selectedType === 'ANIME'
						? 'border-primary bg-primary text-primary-foreground'
						: 'border-border bg-background hover:bg-accent'}"
				>
					Anime
				</button>
				<button
					type="button"
					onclick={() => {
						selectedType = 'MANGA';
						handleFilterChange();
					}}
					class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors
						{selectedType === 'MANGA'
						? 'border-primary bg-primary text-primary-foreground'
						: 'border-border bg-background hover:bg-accent'}"
				>
					Manga
				</button>
			</div>

			<!-- Advanced Filters Toggle -->
			<Button
				variant="outline"
				size="sm"
				onclick={() => {
					const el = document.getElementById('advanced-filters');
					el?.classList.toggle('hidden');
				}}
			>
				<Icon icon="solar:filter-bold" class="mr-2 h-4 w-4" />
				Filters
			</Button>
		</div>

		<!-- Advanced Filters (Hidden by default) -->
		<div id="advanced-filters" class="hidden border-t border-border px-6 py-4">
			<div class="mb-3 flex items-center justify-between">
				<h3 class="text-sm font-semibold">Advanced Filters</h3>
				<Button variant="ghost" size="sm" onclick={clearFilters}>
					<Icon icon="solar:restart-bold" class="mr-2 h-3 w-3" />
					Clear All
				</Button>
			</div>

			<div class="space-y-4">
				<!-- First Row: Sort, Format, Status -->
				<div class="grid grid-cols-1 gap-3 md:grid-cols-4">
					<div>
						<label for="sort" class="mb-1.5 block text-xs font-medium">Sort By</label>
						<select
							id="sort"
							bind:value={sortBy}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="POPULARITY_DESC">Popularity ↓</option>
							<option value="POPULARITY">Popularity ↑</option>
							<option value="TRENDING_DESC">Trending ↓</option>
							<option value="SCORE_DESC">Score ↓</option>
							<option value="SCORE">Score ↑</option>
							<option value="FAVOURITES_DESC">Favorites ↓</option>
							<option value="START_DATE_DESC">Newest</option>
							<option value="START_DATE">Oldest</option>
						</select>
					</div>

					<div>
						<label for="format" class="mb-1.5 block text-xs font-medium">Format</label>
						<select
							id="format"
							bind:value={selectedFormat}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">Any</option>
							{#each formats as format}
								<option value={format}>{format.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="status" class="mb-1.5 block text-xs font-medium">Airing Status</label>
						<select
							id="status"
							bind:value={selectedStatus}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">Any</option>
							{#each statuses as status}
								<option value={status}>{status.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="source" class="mb-1.5 block text-xs font-medium">Source</label>
						<select
							id="source"
							bind:value={selectedSource}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">Any</option>
							{#each sources as source}
								<option value={source}>{source.replace(/_/g, ' ')}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Second Row: Season, Year -->
				<div class="grid grid-cols-1 gap-3 md:grid-cols-4">
					<div>
						<label for="season" class="mb-1.5 block text-xs font-medium">Season</label>
						<select
							id="season"
							bind:value={selectedSeason}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">Any</option>
							{#each seasons as season}
								<option value={season}>{season}</option>
							{/each}
						</select>
					</div>

					<div>
						<label for="year" class="mb-1.5 block text-xs font-medium">Year</label>
						<select
							id="year"
							bind:value={selectedYear}
							onchange={handleFilterChange}
							class="w-full rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						>
							<option value="">Any</option>
							{#each years as year}
								<option value={year.toString()}>{year}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Genres -->
				<div>
					<span class="mb-2 block text-xs font-medium">Genres</span>
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
								class="rounded-md border px-3 py-1.5 text-xs font-medium transition-all hover:scale-105
								{isIncluded
									? 'border-primary bg-primary/20 text-primary'
									: isExcluded
										? 'border-destructive bg-destructive/20 text-destructive line-through'
										: 'border-border bg-background hover:border-primary/50'}"
							>
								{genre}
							</button>
						{/each}
					</div>
					<p class="mt-2 text-xs text-muted-foreground">
						Click once to include, twice to exclude, three times to remove
					</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Results Section -->
	<div class="px-6 py-6">
		{#if query.isLoading}
			<div class="flex items-center justify-center py-20">
				<Icon icon="svg-spinners:3-dots-scale" class="h-12 w-12 text-primary" />
			</div>
		{:else if query.isError}
			<div class="flex flex-col items-center justify-center py-20">
				<Icon icon="solar:danger-circle-bold" class="mb-4 h-16 w-16 text-destructive" />
				<h3 class="mb-2 text-xl font-semibold">Error loading results</h3>
				<p class="text-sm text-muted-foreground">{query.error?.message || 'Unknown error'}</p>
			</div>
		{:else if mediaList.length === 0}
			<div class="flex flex-col items-center justify-center py-20">
				<Icon icon="solar:ghost-bold" class="mb-4 h-16 w-16 text-muted-foreground" />
				<h3 class="mb-2 text-xl font-semibold">No results found</h3>
				<p class="text-sm text-muted-foreground">Try adjusting your search or filters</p>
			</div>
		{:else}
			<!-- Media Grid -->
			<div
				class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 2xl:grid-cols-7"
			>
				{#each mediaList as media (media.id)}
					<MediaCard {media} />
				{/each}
			</div>

			<!-- Pagination -->
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
	</div>
</div>
