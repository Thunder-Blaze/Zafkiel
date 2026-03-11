<script lang="ts">
	import Icon from '@iconify/svelte';
	import { untrack } from 'svelte';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import AdvancedFiltersDialog from './AdvancedFiltersDialog.svelte';
	import {
		GENRES,
		ANIME_SEASONS,
		ANIME_FORMATS,
		ANIME_STATUS,
		MANGA_FORMATS,
		MANGA_STATUS,
		COUNTRY_OF_ORIGIN,
		getYearsOffset
	} from '$lib/constants/search';

	let { 
		type = 'ANIME',
		searchQuery = $bindable(''),
		selectedGenre = $bindable('Any'),
		selectedYear = $bindable('Any'),
		selectedSeason = $bindable('Any'),
		selectedFormat = $bindable('Any'),
		selectedStatus = $bindable('Any'),
		selectedCountry = $bindable('Any'),
		advancedState = $bindable({})
	} = $props<{
		type: 'ANIME' | 'MANGA';
		searchQuery?: string;
		selectedGenre?: string;
		selectedYear?: string;
		selectedSeason?: string;
		selectedFormat?: string;
		selectedStatus?: string;
		selectedCountry?: string;
		advancedState?: Record<string, string[]>;
	}>();

	const years = getYearsOffset();
	
	let advancedFiltersOpen = $state(false);

	// Local state for debouncing
	let localSearchQuery = $state(searchQuery);

	// Sync exterior searchQuery -> localSearchQuery (e.g., initial load, nav pop)
	$effect(() => {
		const ex = searchQuery;
		untrack(() => {
			if (localSearchQuery !== ex) {
				localSearchQuery = ex;
			}
		});
	});

	// Sync localSearchQuery -> exterior searchQuery (with 500ms debounce)
	$effect(() => {
		const local = localSearchQuery;
		const t = setTimeout(() => {
			untrack(() => {
				if (searchQuery !== local) {
					searchQuery = local;
				}
			});
		}, 500);
		return () => clearTimeout(t);
	});
</script>

<div class="flex w-full items-center gap-4 bg-background px-6 py-4" style="view-transition-name: search-filters;">
	<!-- Search -->
	<div class="flex min-w-[140px] flex-1">
		<div class="relative w-full">
			<Icon
				icon="solar:magnifer-bold"
				class="absolute left-3 top-1/2 z-10 size-4 -translate-y-1/2 text-primary"
			/>
			<Input
				type="text"
				placeholder="Search..."
				bind:value={localSearchQuery}
				class="h-9 w-full rounded-lg border-border/40 bg-card/60 pl-9 text-sm font-medium shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:border-primary"
			/>
		</div>
	</div>

	<!-- Genres -->
	<div class="flex min-w-[140px] flex-1">
		<Select.Root type="single" bind:value={selectedGenre}>
			<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
				<div class="flex items-center gap-2">
					<Icon icon="solar:tag-bold" class="size-4 text-primary" />
					<span class="truncate text-sm font-medium text-foreground">{selectedGenre === 'Any' ? 'Genres' : selectedGenre}</span>
				</div>
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="Any" label="Any">Any</Select.Item>
				{#each GENRES as genre}
					<Select.Item value={genre} label={genre}>{genre}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>

	{#if type === 'ANIME'}
		<!-- Anime: Year -->
		<div class="flex min-w-[110px]">
			<Select.Root type="single" bind:value={selectedYear}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:calendar-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedYear === 'Any' ? 'Year' : selectedYear}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each years as year}
						<Select.Item value={year} label={year}>{year}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Anime: Season -->
		<div class="flex min-w-[125px] flex-1">
			<Select.Root type="single" bind:value={selectedSeason}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:cloud-sun-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedSeason === 'Any' ? 'Season' : selectedSeason}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each ANIME_SEASONS as season}
						<Select.Item value={season} label={season}>{season}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Anime: Format -->
		<div class="flex min-w-[125px] flex-1">
			<Select.Root type="single" bind:value={selectedFormat}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:video-frame-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedFormat === 'Any' ? 'Format' : selectedFormat}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each ANIME_FORMATS as format}
						<Select.Item value={format} label={format}>{format}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Anime: Airing Status -->
		<div class="flex min-w-[145px] flex-1">
			<Select.Root type="single" bind:value={selectedStatus}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:play-circle-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedStatus === 'Any' ? 'Status' : selectedStatus}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each ANIME_STATUS as status}
						<Select.Item value={status} label={status}>{status}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	{:else}
		<!-- Manga: Format -->
		<div class="flex min-w-[125px] flex-1">
			<Select.Root type="single" bind:value={selectedFormat}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:notebook-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedFormat === 'Any' ? 'Format' : selectedFormat}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each MANGA_FORMATS as format}
						<Select.Item value={format} label={format}>{format}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Manga: Publishing Status -->
		<div class="flex min-w-[145px] flex-1">
			<Select.Root type="single" bind:value={selectedStatus}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:pen-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedStatus === 'Any' ? 'Status' : selectedStatus}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each MANGA_STATUS as status}
						<Select.Item value={status} label={status}>{status}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Manga: Country Of Origin -->
		<div class="flex min-w-[150px] flex-1">
			<Select.Root type="single" bind:value={selectedCountry}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:global-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedCountry === 'Any' ? 'Country' : selectedCountry}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each COUNTRY_OF_ORIGIN as country}
						<Select.Item value={country} label={country}>{country}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>

		<!-- Manga: Year -->
		<div class="flex min-w-[110px]">
			<Select.Root type="single" bind:value={selectedYear}>
				<Select.Trigger class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary">
					<div class="flex items-center gap-2">
						<Icon icon="solar:calendar-bold" class="size-4 text-primary" />
						<span class="truncate text-sm font-medium text-foreground">{selectedYear === 'Any' ? 'Year' : selectedYear}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					{#each years as year}
						<Select.Item value={year} label={year}>{year}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	{/if}

	<!-- Settings Trigger -->
	<div class="flex shrink-0">
		<Button 
			variant="secondary" 
			size="icon" 
			class="h-9 w-9 rounded-lg border border-border/40 bg-card/60 shadow-sm backdrop-blur-md hover:bg-card/80 transition-colors"
			onclick={() => advancedFiltersOpen = true}
		>
			<Icon icon="solar:filter-bold" class="size-5 text-primary" />
		</Button>
	</div>
</div>

<AdvancedFiltersDialog bind:open={advancedFiltersOpen} {type} bind:filtersState={advancedState} />
