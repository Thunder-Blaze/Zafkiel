<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { staffApi } from '$lib/services/anilist';
	import type { Staff } from '$lib/types/anilist';

	let searchInput = $state('');
	let debouncedQuery = $state('');
	let debounceTimer: ReturnType<typeof setTimeout>;

	function onSearchInput(e: Event) {
		searchInput = (e.currentTarget as HTMLInputElement).value;
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => { debouncedQuery = searchInput.trim(); }, 400);
	}
	function clearSearch() {
		searchInput = '';
		debouncedQuery = '';
		clearTimeout(debounceTimer);
	}

	const isSearching = $derived(debouncedQuery.length > 0);

	const popularQ = createQuery(() => ({
queryKey: ['staff', 'popular', 1, 20],
queryFn: () => staffApi.getPopular({ page: 1, perPage: 20 }),
		staleTime: 15 * 60 * 1000,
	}));

	const birthdayQ = createQuery(() => ({
queryKey: ['staff', 'birthday', 1, 20],
queryFn: () => staffApi.getBirthdayToday({ page: 1, perPage: 20 }),
		staleTime: 5 * 60 * 1000,
	}));

	const searchQ = createQuery(() => ({
		queryKey: ['staff', 'search', debouncedQuery, 1, 40],
		queryFn: () => staffApi.search(debouncedQuery, { page: 1, perPage: 40 }),
		enabled: debouncedQuery.trim().length > 0,
		staleTime: 5 * 60 * 1000,
	}));

	const popular = $derived((popularQ.data?.data?.data ?? popularQ.data?.data ?? []) as Staff[]);
	const birthday = $derived((birthdayQ.data?.data?.data ?? birthdayQ.data?.data ?? []) as Staff[]);
	const searchResults = $derived((searchQ.data?.data?.data ?? searchQ.data?.data ?? []) as Staff[]);

	function staffName(s: Staff): string {
		return s.name?.userPreferred || s.name?.full || `${s.name?.first ?? ''} ${s.name?.last ?? ''}`.trim() || 'Unknown';
	}
	function fmtFav(n?: number | null): string {
		if (!n) return '';
		return n > 999 ? (n / 1000).toFixed(1) + 'k' : n.toLocaleString();
	}
</script>

{#snippet staffCard(staff: Staff, rank?: number, showBirthday?: boolean)}
	<button
		onclick={() => goto(`/staff/${staff.id}`)}
		class="group relative flex w-40 shrink-0 cursor-pointer flex-col text-left transition-transform duration-200 hover:scale-[1.03]"
	>
		<div class="relative h-56 w-full overflow-hidden rounded-md bg-card shadow-lg ring-4 ring-border transition-shadow duration-200 group-hover:ring-primary/50 group-hover:shadow-xl">
			{#if staff.image?.large || staff.image?.medium}
				<CachedImage src={staff.image.large ?? staff.image.medium ?? ''} alt={staffName(staff)} class="h-full w-full object-cover object-top" />
			{:else}
				<div class="flex h-full items-center justify-center bg-muted">
					<Icon icon="solar:user-bold-duotone" class="h-10 w-10 opacity-20" />
				</div>
			{/if}
			<div class="pointer-events-none absolute inset-0 bg-gradient-to-t from-black/60 via-transparent to-transparent"></div>
			{#if staff.favourites}
				<div class="absolute bottom-2 left-2 flex items-center gap-1 rounded-md bg-black/60 px-1.5 py-0.5 text-[10px] font-medium text-white backdrop-blur-sm">
					<Icon icon="solar:heart-bold" class="h-2.5 w-2.5 text-red-400" />
					{fmtFav(staff.favourites)}
				</div>
			{/if}
			{#if rank !== undefined && rank < 3}
				<div class="absolute left-2 top-2 flex h-6 w-6 items-center justify-center rounded-full bg-primary text-[10px] font-bold text-primary-foreground shadow">
					{rank + 1}
				</div>
			{/if}
			{#if showBirthday}
				<div class="absolute right-2 top-2 rounded-full bg-pink-500/90 p-1.5 shadow">
					<Icon icon="solar:gift-bold" class="h-3 w-3 text-white" />
				</div>
			{/if}
		</div>
		<div class="mt-2 w-full">
			<p class="line-clamp-2 text-xs font-semibold leading-snug">{staffName(staff)}</p>
		</div>
	</button>
{/snippet}

<div class="flex flex-col gap-10 px-6 py-8">
	<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
				<Icon icon="solar:users-group-two-rounded-bold-duotone" class="h-5 w-5 text-primary" />
			</div>
			<div>
				<h1 class="text-2xl font-bold">Staff</h1>
				<p class="text-sm text-muted-foreground">Creators, directors, and voice artists</p>
			</div>
		</div>
		<div class="relative w-full max-w-sm">
			<Icon icon="solar:magnifer-linear" class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
			<input type="text" value={searchInput} oninput={onSearchInput} placeholder="Search staff…" class="h-10 w-full rounded-xl border border-border bg-muted/40 pl-9 pr-9 text-sm outline-none transition-colors placeholder:text-muted-foreground focus:border-primary/60 focus:bg-background" />
			{#if searchInput}
				<button onclick={clearSearch} class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground">
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
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if searchResults.length === 0}
				<div class="flex h-40 items-center justify-center rounded-xl border border-dashed text-muted-foreground">
					<div class="text-center">
						<Icon icon="solar:users-group-two-rounded-linear" class="mx-auto mb-2 h-10 w-10 opacity-40" />
						<p class="text-sm">No staff found for "{debouncedQuery}"</p>
					</div>
				</div>
			{:else}
				<div class="grid gap-5 p-0.5 grid-cols-[repeat(auto-fill,minmax(10rem,1fr))]">
					{#each searchResults as staff}{@render staffCard(staff)}{/each}
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
				<button onclick={() => goto('/search/staff/all?section=birthday')} class="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</button>
			</div>
			{#if birthdayQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if birthday.length === 0}
				<div class="flex h-32 items-center justify-center rounded-xl border border-dashed text-muted-foreground">
					<div class="text-center">
						<Icon icon="solar:gift-linear" class="mx-auto mb-2 h-8 w-8 opacity-40" />
						<p class="text-sm">No staff birthdays today</p>
					</div>
				</div>
			{:else}
				<div class="overflow-hidden">
					<div class="-mb-5 grid gap-5 p-0.5 grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] grid-rows-[repeat(2,auto)] auto-rows-[0px]">
						{#each birthday as staff}{@render staffCard(staff, undefined, true)}{/each}
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
				<button onclick={() => goto('/search/staff/all')} class="flex items-center gap-1 text-xs text-muted-foreground transition-colors hover:text-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</button>
			</div>
			{#if popularQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if popular.length === 0}
				<div class="flex h-32 items-center justify-center rounded-xl border border-dashed text-muted-foreground">
					<p class="text-sm">No data available</p>
				</div>
			{:else}
				<div class="overflow-hidden">
					<div class="-mb-5 grid gap-5 p-0.5 grid-cols-[repeat(auto-fill,minmax(10rem,1fr))] grid-rows-[repeat(2,auto)] auto-rows-[0px]">
						{#each popular as staff, i}{@render staffCard(staff, i)}{/each}
					</div>
				</div>
			{/if}
		</section>
	{/if}
</div>
