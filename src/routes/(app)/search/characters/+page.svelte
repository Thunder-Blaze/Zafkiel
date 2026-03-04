<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { characterApi } from '$lib/services/anilist';
	import type { Character } from '$lib/types/anilist';

	let searchInput = $state('');
	let debouncedQuery = $state('');
	let debounceTimer: ReturnType<typeof setTimeout>;

	function onSearchInput(e: Event) {
		searchInput = (e.currentTarget as HTMLInputElement).value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			debouncedQuery = searchInput.trim();
		}, 400);
	}
	function clearSearch() {
		searchInput = '';
		debouncedQuery = '';
		clearTimeout(debounceTimer);
	}

	const isSearching = $derived(debouncedQuery.length > 0);

	const popularQ = createQuery(() => ({
		queryKey: ['character', 'popular', 1, 20],
		queryFn: () => characterApi.getPopular({ page: 1, perPage: 20 }),
		staleTime: 15 * 60 * 1000,
	}));

	const birthdayQ = createQuery(() => ({
		queryKey: ['character', 'birthday', 1, 20],
		queryFn: () => characterApi.getBirthdayToday({ page: 1, perPage: 20 }),
		staleTime: 5 * 60 * 1000,
	}));

	const searchQ = createQuery(() => ({
		queryKey: ['character', 'search', debouncedQuery, 1, 40],
		queryFn: () => characterApi.search(debouncedQuery, { page: 1, perPage: 40 }),
		enabled: debouncedQuery.trim().length > 0,
		staleTime: 5 * 60 * 1000,
	}));

	const popular = $derived((popularQ.data?.data?.data ?? popularQ.data?.data ?? []) as Character[]);
	const birthday = $derived(
		(birthdayQ.data?.data?.data ?? birthdayQ.data?.data ?? []) as Character[]
	);
	const searchResults = $derived(
		(searchQ.data?.data?.data ?? searchQ.data?.data ?? []) as Character[]
	);

	function charName(c: Character): string {
		return (
			c.name?.userPreferred ||
			c.name?.full ||
			`${c.name?.first ?? ''} ${c.name?.last ?? ''}`.trim() ||
			'Unknown'
		);
	}
	function mediaTitle(c: Character): string {
		const node = c.media?.nodes?.[0];
		if (!node?.title) return '';
		return node.title.userPreferred || node.title.romaji || node.title.english || '';
	}
	function fmtFav(n?: number | null): string {
		if (!n) return '';
		return n > 999 ? (n / 1000).toFixed(1) + 'k' : n.toLocaleString();
	}
</script>

{#snippet charCard(char: Character, rank?: number, showBirthday?: boolean)}
	<button
		onclick={() => goto(`/character/${char.id}`)}
		class="group relative flex w-40 max-w-48 shrink-0 cursor-pointer flex-col text-left transition-transform duration-200 hover:scale-[1.03]"
	>
		<div
			class="relative h-56 w-full overflow-hidden rounded-md bg-card shadow-lg ring-4 ring-border transition-shadow duration-200 group-hover:shadow-xl group-hover:ring-primary/50"
		>
			{#if char.image?.large || char.image?.medium}
				<CachedImage
					src={char.image.large ?? char.image.medium ?? ''}
					alt={charName(char)}
					class="h-full w-full object-cover object-top"
				/>
			{:else}
				<div class="flex h-full items-center justify-center bg-muted">
					<Icon icon="solar:user-bold-duotone" class="h-10 w-10 opacity-20" />
				</div>
			{/if}
			<div
				class="pointer-events-none absolute inset-0 bg-linear-to-t from-black/60 via-transparent to-transparent"
			></div>
			{#if char.favourites}
				<div
					class="absolute bottom-2 left-2 flex items-center gap-1 rounded-md bg-black/60 px-1.5 py-0.5 text-[10px] font-medium text-white backdrop-blur-sm"
				>
					<Icon icon="solar:heart-bold" class="h-2.5 w-2.5 text-red-400" />
					{fmtFav(char.favourites)}
				</div>
			{/if}
			{#if rank !== undefined && rank < 3}
				<div
					class="absolute top-2 left-2 flex h-6 w-6 items-center justify-center rounded-full bg-primary text-[10px] font-bold text-primary-foreground shadow"
				>
					{rank + 1}
				</div>
			{/if}
			{#if showBirthday}
				<div class="absolute top-2 right-2 rounded-full bg-pink-500/90 p-1.5 shadow">
					<Icon icon="solar:gift-bold" class="h-3 w-3 text-white" />
				</div>
			{/if}
		</div>
		<div class="mt-2 w-full">
			<p class="line-clamp-2 text-xs leading-snug font-semibold">{charName(char)}</p>
			{#if !showBirthday}
				{@const mt = mediaTitle(char)}
				{#if mt}<p class="mt-0.5 line-clamp-1 text-[10px] text-muted-foreground">{mt}</p>{/if}
			{/if}
		</div>
	</button>
{/snippet}

<div class="flex flex-col gap-10 px-6 py-8">
	<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
				<Icon icon="solar:user-bold-duotone" class="h-5 w-5 text-primary" />
			</div>
			<div>
				<h1 class="text-2xl font-bold">Characters</h1>
				<p class="text-sm text-muted-foreground">Beloved characters from anime & manga</p>
			</div>
		</div>
		<div class="relative w-full max-w-sm">
			<Icon
				icon="solar:magnifer-linear"
				class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted-foreground"
			/>
			<input
				type="text"
				value={searchInput}
				oninput={onSearchInput}
				placeholder="Search characters…"
				class="h-10 w-full rounded-xl border border-border bg-muted/40 pr-9 pl-9 text-sm transition-colors outline-none placeholder:text-muted-foreground focus:border-primary/60 focus:bg-background"
			/>
			{#if searchInput}
				<button
					onclick={clearSearch}
					class="absolute top-1/2 right-3 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground"
				>
					<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
				</button>
			{/if}
		</div>
	</div>

	{#if isSearching}
		<section>
			<div class="mb-4 flex items-center gap-2">
				<Icon icon="solar:magnifer-bold-duotone" class="h-5 w-5 text-primary" />
				<h2 class="text-xl font-bold">Results for "{debouncedQuery}"</h2>
			</div>
			{#if searchQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if searchResults.length === 0}
				<div
					class="flex h-40 items-center justify-center rounded-xl border border-dashed text-muted-foreground"
				>
					<div class="text-center">
						<Icon icon="solar:user-linear" class="mx-auto mb-2 h-10 w-10 opacity-40" />
						<p class="text-sm">No characters found for "{debouncedQuery}"</p>
					</div>
				</div>
			{:else}
				<div class="grid grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] gap-5 p-0.5">
					{#each searchResults as char}{@render charCard(char)}{/each}
				</div>
			{/if}
		</section>
	{:else}
		<!-- Birthday Today -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<div class="flex items-center gap-2">
					<Icon icon="solar:gift-bold-duotone" class="h-5 w-5 text-pink-400" />
					<h2 class="text-xl font-bold">Birthday Today</h2>
				</div>
				<button
					onclick={() => goto('/search/characters/all?section=birthday')}
					class="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</button>
			</div>
			{#if birthdayQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if birthday.length === 0}
				<div
					class="flex h-32 items-center justify-center rounded-xl border border-dashed text-muted-foreground"
				>
					<div class="text-center">
						<Icon icon="solar:gift-linear" class="mx-auto mb-2 h-8 w-8 opacity-40" />
						<p class="text-sm">No character birthdays today</p>
					</div>
				</div>
			{:else}
				<div class="overflow-hidden">
					<div
						class="-mb-5 grid auto-rows-[0px] grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] grid-rows-[repeat(2,auto)] gap-5 p-0.5"
					>
						{#each birthday as char}{@render charCard(char, undefined, true)}{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- Most Favourited -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<div class="flex items-center gap-2">
					<Icon icon="solar:heart-bold-duotone" class="h-5 w-5 text-red-400" />
					<h2 class="text-xl font-bold">Most Favourited</h2>
				</div>
				<button
					onclick={() => goto('/search/characters/all')}
					class="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</button>
			</div>
			{#if popularQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if popular.length === 0}
				<div
					class="flex h-32 items-center justify-center rounded-xl border border-dashed text-muted-foreground"
				>
					<p class="text-sm">No data available</p>
				</div>
			{:else}
				<div class="overflow-hidden">
					<div
						class="-mb-5 grid auto-rows-[0px] grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] grid-rows-[repeat(2,auto)] gap-5 p-0.5"
					>
						{#each popular as char, i}{@render charCard(char, i)}{/each}
					</div>
				</div>
			{/if}
		</section>
	{/if}
</div>
