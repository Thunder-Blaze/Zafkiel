<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import Icon from '@iconify/svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { staffApi } from '$lib/services/anilist';
	import type { Staff } from '$lib/types/anilist';

	type Section = 'popular' | 'birthday';
	const initialSection = (page.url.searchParams.get('section') ?? 'popular') as Section;

	let activeSection = $state<Section>(initialSection);
	let popularPage = $state(1);
	let birthdayPage = $state(1);
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
queryKey: ['staff', 'popular', popularPage, 40],
queryFn: () => staffApi.getPopular({ page: popularPage, perPage: 40 }),
		staleTime: 15 * 60 * 1000,
		enabled: !isSearching && activeSection === 'popular',
	}));

	const birthdayQ = createQuery(() => ({
queryKey: ['staff', 'birthday', birthdayPage, 40],
queryFn: () => staffApi.getBirthdayToday({ page: birthdayPage, perPage: 40 }),
		staleTime: 5 * 60 * 1000,
		enabled: !isSearching && activeSection === 'birthday',
	}));

	const searchQ = createQuery(() => ({
queryKey: ['staff', 'search', debouncedQuery, activeSection, 1, 40],
queryFn: () => staffApi.search(debouncedQuery, { page: 1, perPage: 40, isBirthday: activeSection === 'birthday' ? true : undefined }),
		enabled: debouncedQuery.trim().length > 0,
		staleTime: 5 * 60 * 1000,
	}));

	const popular = $derived((popularQ.data?.data?.data ?? popularQ.data?.data ?? []) as Staff[]);
	const birthday = $derived((birthdayQ.data?.data?.data ?? birthdayQ.data?.data ?? []) as Staff[]);
	const searchResults = $derived((searchQ.data?.data?.data ?? searchQ.data?.data ?? []) as Staff[]);

	const activeItems = $derived(isSearching ? searchResults : activeSection === 'birthday' ? birthday : popular);
	const activeLoading = $derived(isSearching ? searchQ.isLoading : activeSection === 'birthday' ? birthdayQ.isLoading : popularQ.isLoading);
	const activePage = $derived(activeSection === 'birthday' ? birthdayPage : popularPage);

	function setPage(p: number) {
		if (activeSection === 'birthday') birthdayPage = p;
		else popularPage = p;
	}

	function staffName(s: Staff): string {
		return s.name?.userPreferred || s.name?.full || `${s.name?.first ?? ''} ${s.name?.last ?? ''}`.trim() || 'Unknown';
	}
	function fmtFav(n?: number | null): string {
		if (!n) return '';
		return n > 999 ? (n / 1000).toFixed(1) + 'k' : n.toLocaleString();
	}
	function formatBirthday(s: Staff): string {
		if (!s.dateOfBirth) return '';
		const { month, day } = s.dateOfBirth;
		if (!month || !day) return '';
		return new Date(2000, month - 1, day).toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
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
			{#if showBirthday}
				{@const bd = formatBirthday(staff)}
				{#if bd}<p class="mt-0.5 text-[10px] text-pink-400">{bd}</p>{/if}
			{/if}
		</div>
	</button>
{/snippet}

<div class="flex flex-col gap-8 px-6 py-8">
	<!-- Header -->
	<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div class="flex items-center gap-3">
			<button onclick={() => goto('/search/staff')} class="flex h-8 w-8 items-center justify-center rounded-lg border border-border text-muted-foreground transition-colors hover:border-primary/40 hover:text-foreground">
				<Icon icon="solar:arrow-left-linear" class="h-4 w-4" />
			</button>
			<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
				<Icon icon="solar:users-group-two-rounded-bold-duotone" class="h-5 w-5 text-primary" />
			</div>
			<div>
				<h1 class="text-2xl font-bold">All Staff</h1>
				<p class="text-sm text-muted-foreground">Browse & search all staff</p>
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

	<!-- Section Tabs -->
	<div class="flex items-center gap-1.5">
		<button
			onclick={() => (activeSection = 'popular')}
			class="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {activeSection === 'popular' ? 'bg-primary/10 text-primary' : 'text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:heart-bold-duotone" class="h-4 w-4 text-red-400" />
			Most Favourited
		</button>
		<button
			onclick={() => (activeSection = 'birthday')}
			class="flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {activeSection === 'birthday' ? 'bg-pink-500/15 text-pink-400' : 'text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:gift-bold-duotone" class="h-4 w-4 text-pink-400" />
			Birthday Today
		</button>
	</div>

	<!-- Content -->
	{#if activeLoading}
		<div class="flex h-64 items-center justify-center">
			<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
		</div>
	{:else if activeItems.length === 0}
		<div class="flex h-48 items-center justify-center rounded-xl border border-dashed text-muted-foreground">
			<div class="text-center">
				<Icon icon="solar:users-group-two-rounded-linear" class="mx-auto mb-2 h-10 w-10 opacity-40" />
				<p class="text-sm">{isSearching ? `No staff found for "${debouncedQuery}"` : 'No results'}</p>
			</div>
		</div>
	{:else}
		<div class="grid gap-5 p-0.5 [grid-template-columns:repeat(auto-fill,minmax(10rem,1fr))]">

			{#each activeItems as staff, i}
				{@render staffCard(
					staff,
					!isSearching && activeSection === 'popular' && popularPage === 1 ? i : undefined,
					activeSection === 'birthday'
				)}
			{/each}
		</div>
	{/if}

	<!-- Pagination (hidden while searching) -->
	{#if !isSearching && activeItems.length > 0}
		<div class="flex items-center justify-center gap-3 pt-2">
			<button
				onclick={() => setPage(Math.max(1, activePage - 1))}
				disabled={activePage === 1}
				class="flex h-9 items-center gap-1.5 rounded-lg border border-border px-3 text-sm text-muted-foreground transition-colors hover:border-primary/40 hover:text-foreground disabled:pointer-events-none disabled:opacity-40"
			>
				<Icon icon="solar:arrow-left-linear" class="h-4 w-4" /> Previous
			</button>
			<span class="text-sm text-muted-foreground">Page {activePage}</span>
			<button
				onclick={() => setPage(activePage + 1)}
				class="flex h-9 items-center gap-1.5 rounded-lg border border-border px-3 text-sm text-muted-foreground transition-colors hover:border-primary/40 hover:text-foreground"
			>
				Next <Icon icon="solar:arrow-right-linear" class="h-4 w-4" />
			</button>
		</div>
	{/if}
</div>
