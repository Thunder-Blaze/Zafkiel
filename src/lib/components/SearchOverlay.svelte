<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { useConfigState } from '$lib/stores/config.svelte';
	import { searchOverlay } from '$lib/stores/search-overlay.svelte';
	import type { SearchAllResults } from '$lib/types/anilist';

	const config = useConfigState();
	const animationsEnabled = $derived(config.animations);

	let searchInput = $state<HTMLInputElement | null>(null);
	let searchQuery = $state('');
	let searchResults = $state<SearchAllResults | null>(null);
	let isSearching = $state(false);
	let searchError = $state<string | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	let loadingWidth = $state(0);
	let loadingDuration = $state(0);

	// Global Ctrl+K / Cmd+K listener for toggle
	$effect(() => {
		if (!browser) return;
		const handleKey = (e: KeyboardEvent) => {
			if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
				e.preventDefault();
				searchOverlay.toggle();
			}
		};
		document.addEventListener('keydown', handleKey);
		return () => document.removeEventListener('keydown', handleKey);
	});

	// Auto-focus input when overlay opens
	$effect(() => {
		if (searchOverlay.open && searchInput) {
			queueMicrotask(() => searchInput?.focus());
		}
	});

	// Debounced search: fires after 3 s of idle input
	$effect(() => {
		const q = searchQuery.trim();
		if (debounceTimer) clearTimeout(debounceTimer);
		if (!q) {
			searchResults = null;
			isSearching = false;
			searchError = null;
			loadingWidth = 0;
			loadingDuration = 0;
			return;
		}
		isSearching = true;
		loadingDuration = 0;
		loadingWidth = 0;
		requestAnimationFrame(() => {
			requestAnimationFrame(() => {
				loadingDuration = 3000;
				loadingWidth = 100;
			});
		});
		debounceTimer = setTimeout(() => {
			triggerSearch(q);
		}, 3000);
		return () => {
			if (debounceTimer) clearTimeout(debounceTimer);
		};
	});

	async function triggerSearch(q: string): Promise<void> {
		isSearching = true;
		searchError = null;
		loadingDuration = 300;
		loadingWidth = 100;
		try {
			type Resp = { success: boolean; data?: SearchAllResults; error?: string };
			const res = await invoke<Resp>('search_all', { query: q, perPage: 5 });
			if (res.success && res.data) {
				searchResults = res.data;
			} else {
				searchError = res.error ?? 'Search failed';
			}
		} catch (e) {
			searchError = String(e);
		} finally {
			isSearching = false;
			setTimeout(() => {
				loadingWidth = 0;
				loadingDuration = 0;
			}, 300);
		}
	}

	function closeSearch(): void {
		searchOverlay.close();
		searchQuery = '';
		searchResults = null;
		searchError = null;
		isSearching = false;
		loadingWidth = 0;
		loadingDuration = 0;
		if (debounceTimer) clearTimeout(debounceTimer);
	}

	function handleSearchKeydown(e: KeyboardEvent): void {
		if (e.key === 'Escape') {
			closeSearch();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			const q = searchQuery.trim();
			if (q) {
				if (debounceTimer) clearTimeout(debounceTimer);
				loadingDuration = 200;
				loadingWidth = 100;
				triggerSearch(q);
			}
		}
	}

	function navigateAndClose(path: string): void {
		closeSearch();
		goto(path);
	}
</script>

{#if browser && searchOverlay.open}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		transition:fade={{ duration: animationsEnabled ? 150 : 0 }}
		class="fixed inset-x-0 top-12 bottom-0 z-[1000000] flex flex-col items-center bg-black/70 backdrop-blur-md"
		onclick={(e) => {
			if (e.target === e.currentTarget) closeSearch();
		}}
		onkeydown={handleSearchKeydown}
	>
		<!-- Search box -->
		<div
			in:fly={{ y: -10, duration: animationsEnabled ? 220 : 0, easing: cubicOut }}
			class="mt-6 w-full max-w-2xl overflow-hidden rounded-xl border border-border/50 bg-background shadow-2xl"
		>
			<div class="flex items-center gap-3 px-5 py-3.5">
				<Icon icon="solar:magnifer-bold" class="h-5 w-5 shrink-0 text-muted-foreground" />
				<input
					bind:this={searchInput}
					bind:value={searchQuery}
					type="text"
					placeholder="Search anime, manga, characters…"
					class="flex-1 bg-transparent text-base outline-none border-none placeholder:text-muted-foreground/60"
				/>
				{#if searchQuery}
					<button
						type="button"
						onclick={() => {
							searchQuery = '';
							searchResults = null;
							isSearching = false;
						}}
						class="rounded-md p-1 text-muted-foreground transition-colors hover:text-foreground"
						aria-label="Clear"
					>
						<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
					</button>
				{/if}
				<kbd
					class="rounded border border-border/60 bg-muted px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground"
					>Esc</kbd
				>
			</div>

			{#if isSearching}
				<div class="h-0.5 w-full overflow-hidden bg-muted">
					<div
						class="h-full bg-primary"
						style="width: {loadingWidth}%; transition: width {loadingDuration}ms linear"
					></div>
				</div>
			{:else if !searchResults && searchQuery.trim()}
				<div class="border-t border-border/40 px-5 py-3 text-center text-sm text-muted-foreground">
					Press <kbd
						class="rounded border border-border/60 bg-muted px-1 py-0.5 font-mono text-[10px]"
						>↵</kbd
					> or wait 3 s to search
				</div>
			{/if}
		</div>

		<!-- Results -->
		{#if searchResults}
			{@const q = searchQuery.trim()}
			{@const hasAny =
				searchResults.anime.data.length ||
				searchResults.manga.data.length ||
				searchResults.characters.data.length ||
				searchResults.staff.data.length ||
				searchResults.studios.data.length ||
				searchResults.users.data.length}

			{#if !hasAny}
				<div
					in:fly={{ y: -6, duration: animationsEnabled ? 180 : 0, easing: cubicOut }}
					class="mt-3 rounded-xl border border-border/50 bg-background px-10 py-8 text-center text-sm text-muted-foreground shadow-2xl"
				>
					No results for <span class="text-foreground">"{q}"</span>
				</div>
			{:else}
				<div
					in:fly={{ y: -6, duration: animationsEnabled ? 180 : 0, easing: cubicOut }}
					class="mx-auto mt-2 w-full max-w-6xl overflow-hidden rounded-xl border border-border/50 bg-background shadow-2xl"
				>
					<!-- Row 1 — Anime | Manga | Characters -->
					<div class="grid grid-cols-3 divide-x divide-border/40">
						<!-- Anime -->
						<div class="flex flex-col">
							<div class="flex items-center justify-between border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Anime</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.anime.data as item}
									<button
										onclick={() => navigateAndClose(`/anime/${item.id}`)}
										class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40"
									>
										{#if item.coverImage?.medium}
											<img
												src={item.coverImage.medium}
												alt=""
												class="h-9 w-6 shrink-0 rounded object-cover"
											/>
										{:else}
											<div class="h-9 w-6 shrink-0 rounded bg-muted"></div>
										{/if}
										<div class="min-w-0 flex-1">
											<p class="truncate text-sm leading-snug">
												{item.title?.userPreferred ?? item.title?.romaji ?? '—'}
											</p>
											<p class="text-[11px] text-muted-foreground">
												{item.format ? String(item.format).replace(/_/g, ' ') : ''}
												{#if item.meanScore}<span class="ml-1 text-yellow-500"
														>{item.meanScore}%</span
													>{/if}
											</p>
										</div>
									</button>
								{/each}
							</div>
							{#if searchResults.anime.data.length}
								<button
									onclick={() => navigateAndClose(`/browse/anime`)}
									class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30"
								>
									View all anime results
								</button>
							{/if}
						</div>

						<!-- Manga -->
						<div class="flex flex-col">
							<div class="flex items-center justify-between border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Manga</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.manga.data as item}
									<button
										onclick={() => navigateAndClose(`/manga/${item.id}`)}
										class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40"
									>
										{#if item.coverImage?.medium}
											<img
												src={item.coverImage.medium}
												alt=""
												class="h-9 w-6 shrink-0 rounded object-cover"
											/>
										{:else}
											<div class="h-9 w-6 shrink-0 rounded bg-muted"></div>
										{/if}
										<div class="min-w-0 flex-1">
											<p class="truncate text-sm leading-snug">
												{item.title?.userPreferred ?? item.title?.romaji ?? '—'}
											</p>
											<p class="text-[11px] text-muted-foreground">
												{item.format ? String(item.format).replace(/_/g, ' ') : ''}
												{#if item.meanScore}<span class="ml-1 text-yellow-500"
														>{item.meanScore}%</span
													>{/if}
											</p>
										</div>
									</button>
								{/each}
							</div>
							{#if searchResults.manga.data.length}
								<button
									onclick={() => navigateAndClose(`/browse/manga`)}
									class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30"
								>
									View all manga results
								</button>
							{/if}
						</div>

						<!-- Characters -->
						<div class="flex flex-col">
							<div class="flex items-center justify-between border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Characters</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.characters.data as item}
									<button
										onclick={() => navigateAndClose(`/character/${item.id}`)}
										class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40"
									>
										{#if item.image?.medium}
											<img
												src={item.image.medium}
												alt=""
												class="h-8 w-8 shrink-0 rounded-full object-cover"
											/>
										{:else}
											<div class="h-8 w-8 shrink-0 rounded-full bg-muted"></div>
										{/if}
										<span class="flex-1 truncate text-sm"
											>{item.name?.userPreferred ?? item.name?.full ?? '—'}</span
										>
									</button>
								{/each}
							</div>
							{#if searchResults.characters.data.length}
								<button
									onclick={() => navigateAndClose(`/search/characters`)}
									class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30"
								>
									View all character results
								</button>
							{/if}
						</div>
					</div>

					<div class="border-t border-border/60"></div>

					<!-- Row 2 — Staff | Studios | Users -->
					<div class="grid grid-cols-3 divide-x divide-border/40">
						<!-- Staff -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Staff</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.staff.data as item}
									<button
										onclick={() => navigateAndClose(`/staff/${item.id}`)}
										class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40"
									>
										{#if item.image?.medium}
											<img
												src={item.image.medium}
												alt=""
												class="h-8 w-8 shrink-0 rounded-full object-cover"
											/>
										{:else}
											<div class="h-8 w-8 shrink-0 rounded-full bg-muted"></div>
										{/if}
										<span class="flex-1 truncate text-sm"
											>{item.name?.userPreferred ?? item.name?.full ?? '—'}</span
										>
									</button>
								{/each}
							</div>
							{#if searchResults.staff.data.length}
								<button
									onclick={() => navigateAndClose(`/search/staff`)}
									class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30"
								>
									View all staff results
								</button>
							{/if}
						</div>

						<!-- Studios -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Studios</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.studios.data as item}
									<button
										onclick={() => navigateAndClose(`/studio/${item.id}`)}
										class="flex w-full items-center gap-2.5 px-4 py-3 text-left transition-colors hover:bg-muted/40"
									>
										<span class="flex-1 truncate text-sm">{item.name ?? '—'}</span>
										{#if item.isAnimationStudio}<span
												class="shrink-0 rounded bg-primary/10 px-1.5 py-0.5 text-[10px] text-primary"
												>Animation</span
											>{/if}
									</button>
								{/each}
							</div>
							{#if searchResults.studios.data.length}
								<button
									onclick={() => navigateAndClose(`/search/studios`)}
									class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30"
								>
									View all studio results
								</button>
							{/if}
						</div>

						<!-- Users -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span
									class="text-[11px] font-semibold tracking-widest text-muted-foreground uppercase"
									>Users</span
								>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.users.data as item}
									<button
										onclick={() => navigateAndClose(`/user/${item.name}`)}
										class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40"
									>
										{#if item.avatar?.medium}
											<img
												src={item.avatar.medium}
												alt=""
												class="h-8 w-8 shrink-0 rounded-full object-cover"
											/>
										{:else}
											<div class="h-8 w-8 shrink-0 rounded-full bg-muted"></div>
										{/if}
										<span class="flex-1 truncate text-sm">{item.name ?? '—'}</span>
									</button>
								{/each}
							</div>
						</div>
					</div>
				</div>
			{/if}
		{:else if searchError}
			<div
				class="mt-3 rounded-xl border border-border/50 bg-background px-10 py-6 text-center text-sm text-destructive shadow-2xl"
			>
				{searchError}
			</div>
		{/if}

		<!-- Hints -->
		<div class="mt-3 flex items-center gap-4 text-xs text-white/40">
			<span class="flex items-center gap-1"
				><kbd class="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 font-mono text-[10px]"
					>↵</kbd
				> search</span
			>
			<span class="flex items-center gap-1"
				><kbd class="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 font-mono text-[10px]"
					>Esc</kbd
				> close</span
			>
			<span class="text-white/30">Powered by AniList</span>
		</div>
	</div>
{/if}
