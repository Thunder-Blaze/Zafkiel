<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
	import ProfileDropdown from '$lib/components/ProfileDropdown.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import BrowseDropdown from '$lib/components/BrowseDropdown.svelte';
	import type { Window as TauriWindow } from '@tauri-apps/api/window';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import {
		initializeKeybindings,
		cleanupKeybindings,
		updateKeyBindingAction,
		formatKeyBinding,
	} from '$lib/utils/keybindings';

	const appWindow: TauriWindow | null = browser ? getCurrentWindow() : null;

	let isMaximized = $state(false);
	let isFullscreen = $state(false);

	// Navigation history management
	let navigationHistory = $state<string[]>([]);
	let currentHistoryIndex = $state(-1);
	let canGoBack = $derived(currentHistoryIndex > 0);
	let canGoForward = $derived(currentHistoryIndex < navigationHistory.length - 1);

	// Search bar
	let searchInput = $state<HTMLInputElement | null>(null);
	let searchQuery = $state('');

	// Browse dropdown & search popup
	let searchOpen = $state(false);

	// Platform-specific keyboard shortcut display
	const isMac =
		browser &&
		(navigator.userAgent.includes('Mac') ||
			navigator.userAgent.includes('iPhone') ||
			navigator.userAgent.includes('iPad'));

	let unlisten: UnlistenFn | null = null;

	onMount(async () => {
		if (!appWindow) return;

		try {
			// Check initial states
			isMaximized = await appWindow.isMaximized();
			isFullscreen = await appWindow.isFullscreen();

			// Listen for window resize events to sync state
			unlisten = await appWindow.onResized(async () => {
				isMaximized = await appWindow.isMaximized();
				isFullscreen = await appWindow.isFullscreen();
			});

			// Initialize navigation history with current page
			if (browser && page.url.pathname) {
				navigationHistory = [page.url.pathname];
				currentHistoryIndex = 0;
			}

			// Initialize keybindings
			if (browser) {
				initializeKeybindings();

				// Set up keybinding actions with cross-platform modifiers
				updateKeyBindingAction('ArrowLeft', { secondary: true }, handleBack);
				updateKeyBindingAction('ArrowRight', { secondary: true }, handleForward);
				updateKeyBindingAction('r', { primary: true }, handleReload);
				updateKeyBindingAction('k', { primary: true }, focusSearch);

				console.log('[TitleBar] Keybindings initialized');
			}
		} catch (error) {
			console.error('[TitleBar] Failed to initialize:', error);
		}
	});

	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
		cleanupKeybindings();
	});

	// Track navigation for history
	$effect(() => {
		if (browser && page.url.pathname) {
			const newPath = page.url.pathname;

			// Skip if this is the same as current path
			if (newPath === navigationHistory[currentHistoryIndex]) {
				return;
			}

			// Check if this is a back/forward navigation
			const existingIndex = navigationHistory.indexOf(newPath);
			if (existingIndex !== -1 && existingIndex < currentHistoryIndex) {
				// User went back
				currentHistoryIndex = existingIndex;
				console.log('[TitleBar] Went back to:', newPath, 'index:', currentHistoryIndex);
			} else if (existingIndex !== -1 && existingIndex > currentHistoryIndex) {
				// User went forward
				currentHistoryIndex = existingIndex;
				console.log('[TitleBar] Went forward to:', newPath, 'index:', currentHistoryIndex);
			} else {
				// New navigation - remove any forward history and add new path
				navigationHistory = [...navigationHistory.slice(0, currentHistoryIndex + 1), newPath];
				currentHistoryIndex = navigationHistory.length - 1;
				$inspect('[TitleBar] New navigation to:', newPath, 'history:', navigationHistory);
			}
		}
	});

	async function minimize(): Promise<void> {
		try {
			await appWindow?.minimize();
		} catch (err) {
			console.error('[TitleBar] Failed to minimize:', err);
		}
	}

	async function toggleMaximize(): Promise<void> {
		try {
			await appWindow?.toggleMaximize();
		} catch (err) {
			console.error('[TitleBar] Failed to toggle maximize:', err);
		}
	}

	async function close(): Promise<void> {
		try {
			await appWindow?.close();
		} catch (err) {
			console.error('[TitleBar] Failed to close window:', err);
		}
	}

	// Navigation functions
	function handleBack(): void {
		if (canGoBack) {
			currentHistoryIndex -= 1;
			goto(navigationHistory[currentHistoryIndex]);
		}
	}

	function handleForward(): void {
		if (canGoForward) {
			currentHistoryIndex += 1;
			goto(navigationHistory[currentHistoryIndex]);
		}
	}

	function handleReload(): void {
		if (browser) {
			window.location.reload();
		}
	}

	import { invoke } from '@tauri-apps/api/core';
	import type { SearchAllResults } from '$lib/types/anilist';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { useConfigState } from '$lib/stores/config.svelte';

	const config = useConfigState();
	const animationsEnabled = $derived(config.animations);

	// Search portal state
	let searchResults = $state<SearchAllResults | null>(null);
	let isSearching = $state(false);
	let searchError = $state<string | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;
	// Progress bar state: width 0–100, duration in ms
	let loadingWidth = $state(0);
	let loadingDuration = $state(0);

	function focusSearch(): void {
		searchOpen = true;
	}

	// Auto-focus input when portal opens
	$effect(() => {
		if (searchOpen && searchInput) {
			queueMicrotask(() => searchInput?.focus());
		}
	});

	// 3-second debounce: fires search after user stops typing
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
		// Show loading bar immediately so the user knows input was registered
		isSearching = true;
		// Reset bar instantly, then animate it filling over 3 s
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
		// Rush the bar to 100% while the API call is in flight
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
			// Briefly show the full bar, then reset it
			setTimeout(() => {
				loadingWidth = 0;
				loadingDuration = 0;
			}, 300);
		}
	}

	function closeSearch(): void {
		searchOpen = false;
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
				// Rush bar to 100% quickly when Enter fires immediately
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

	const navItems = [
		{ path: '/social', icon: 'solar:users-group-rounded-bold', label: 'Social' },
		{ path: '/forum', icon: 'solar:chat-square-bold', label: 'Forum' },
		{ path: '/downloads', icon: 'solar:download-bold', label: 'Downloads' },
		{ path: '/settings', icon: 'solar:settings-bold', label: 'Settings' },
	];
</script>

{#if browser && !isFullscreen}
	<div
		data-tauri-drag-region
		class="fixed top-0 right-0 left-0 z-999999 flex h-12 items-center justify-between border-b border-border/50 bg-background/95 backdrop-blur-xl select-none"
	>
		<!-- Left Section: Logo + Navigation Buttons + Nav Items -->
		<div class="flex h-full items-center gap-2 pl-3" data-tauri-drag-region>
			<!-- Logo -->
			<button
				type="button"
				onclick={() => goto('/')}
				class="flex items-center gap-2 px-2 rounded-lg transition-colors hover:bg-primary/5 active:bg-primary/10 cursor-pointer"
				title="Go to Home"
			>
				<div
					class="flex h-7 w-7 items-center justify-center rounded-lg bg-linear-to-br from-primary to-primary/70 shadow-lg shadow-primary/20"
				>
					<Icon icon="solar:clock-circle-bold" class="h-4 w-4 text-primary-foreground" />
				</div>
				<div class="flex items-baseline gap-1">
					<span class="text-lg font-bold tracking-tight">Zafkiel</span>
					<sup
						class="-top-2 rounded border border-accent/60 bg-accent px-1 py-1.5 text-[8px] font-semibold text-accent-foreground"
					>
						ALPHA
					</sup>
				</div>
			</button>

			<!-- Navigation Controls -->
			<div class="flex h-full items-center gap-1 border-l border-border/30 pl-2">
				<Button
					variant="ghost"
					size="icon"
					class="h-7 w-7"
					disabled={!canGoBack}
					onclick={handleBack}
					title="Go back (Alt+←)"
				>
					<Icon
						icon="solar:alt-arrow-left-bold"
						class="h-4 w-4 {canGoBack ? 'text-foreground' : 'text-muted-foreground'}"
					/>
				</Button>

				<Button
					variant="ghost"
					size="icon"
					class="h-7 w-7"
					disabled={!canGoForward}
					onclick={handleForward}
					title="Go forward (Alt+→)"
				>
					<Icon
						icon="solar:alt-arrow-right-bold"
						class="h-4 w-4 {canGoForward ? 'text-foreground' : 'text-muted-foreground'}"
					/>
				</Button>

				<Button
					variant="ghost"
					size="icon"
					class="h-7 w-7"
					onclick={handleReload}
					title="Reload (⌘+R)"
				>
					<Icon icon="solar:refresh-bold" class="h-4 w-4 text-foreground" />
				</Button>
			</div>

			<!-- Browse dropdown + remaining Nav Items -->
			<div class="ml-2 flex h-full items-center gap-1">
				<!-- Browse dropdown -->
				<BrowseDropdown />

				<!-- Remaining nav items -->
				{#each navItems as item}
					{@const isActive = page.url.pathname === item.path}
					<Button
						variant="ghost"
						size="sm"
						class="h-8 gap-2 {isActive
							? 'bg-primary/10 text-primary'
							: 'text-foreground/70 hover:text-foreground'}"
						onclick={() => goto(item.path)}
					>
						<Icon icon={item.icon} class="h-4 w-4" />
						<span class="text-xs font-medium">{item.label}</span>
					</Button>
				{/each}
			</div>
		</div>

		<!-- Right Section: Search icon + Notifications + Profile + Theme + Window Controls -->
		<div class="flex h-full items-center">
			<!-- Fake search bar -->
			<button
				type="button"
				onclick={() => (searchOpen = true)}
				title="Search ({isMac ? '⌘K' : 'Ctrl+K'})"
				class="mr-2 flex h-8 w-48 items-center gap-2 rounded-md bg-muted/40 px-3 text-sm text-muted-foreground transition-colors hover:bg-muted/70 hover:text-foreground"
			>
				<Icon icon="solar:magnifer-bold" class="h-3.5 w-3.5 shrink-0" />
				<span class="flex-1 text-left text-xs">Search…</span>
				<span class="flex items-center gap-0.5">
					<kbd class="rounded border border-border/50 bg-background/50 px-1.5 py-0.5 font-mono text-[10px] leading-none">{isMac ? '⌘' : 'Ctrl'}</kbd>
					<kbd class="rounded border border-border/50 bg-background/50 px-1.5 py-0.5 font-mono text-[10px] leading-none">K</kbd>
				</span>
			</button>
			<Button
				variant="ghost"
				size="icon"
				class="relative h-8 w-8 {page.url.pathname === '/notifications' ? 'text-primary' : 'text-foreground/70 hover:text-foreground'}"
				onclick={() => goto('/notifications')}
				title="Notifications"
			>
				<Icon icon="solar:bell-bold" class="h-4 w-4" />
			</Button>
			<div class="px-2">
				<ProfileDropdown />
			</div>

			<div class="px-2">
				<ThemeSwitcher />
			</div>

			<div class="flex h-full items-center" data-tauri-drag-region="false">
				<button
					type="button"
					onclick={minimize}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
					aria-label="Minimize"
				>
					<Icon icon="solar:minus-circle-bold" class="h-5 w-5 text-foreground" />
				</button>

				<button
					type="button"
					onclick={toggleMaximize}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
					aria-label={isMaximized ? 'Restore' : 'Maximize'}
				>
					<Icon
						icon={isMaximized
							? 'solar:quit-full-screen-square-bold'
							: 'solar:full-screen-square-bold'}
						class="h-5 w-5 text-foreground"
					/>
				</button>

				<button
					type="button"
					onclick={close}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-red-500 hover:text-white active:bg-red-600"
					aria-label="Close"
				>
					<Icon icon="solar:close-circle-bold" class="h-5 w-5" />
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Search portal — rendered outside titlebar stacking context -->
{#if browser && searchOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		transition:fade={{ duration: animationsEnabled ? 150 : 0 }}
		class="fixed inset-0 z-1000000 flex flex-col items-center bg-black/70 pt-12 backdrop-blur-md"
		onclick={(e) => { if (e.target === e.currentTarget) closeSearch(); }}
		onkeydown={handleSearchKeydown}
	>
		<!-- Search box — narrow, sits at the top -->
		<div
			in:fly={{ y: -10, duration: animationsEnabled ? 220 : 0, easing: cubicOut }}
			class="w-full max-w-2xl overflow-hidden rounded-xl border border-border/50 bg-background shadow-2xl"
		>
			<div class="flex items-center gap-3 px-5 py-3.5">
				<Icon icon="solar:magnifer-bold" class="h-5 w-5 shrink-0 text-muted-foreground" />
				<input
					bind:this={searchInput}
					bind:value={searchQuery}
					type="text"
					placeholder="Search anime, manga, characters…"
					class="flex-1 bg-transparent text-base outline-none placeholder:text-muted-foreground/60"
				/>
				{#if searchQuery}
					<button
						type="button"
						onclick={() => { searchQuery = ''; searchResults = null; isSearching = false; }}
						class="rounded-md p-1 text-muted-foreground transition-colors hover:text-foreground"
						aria-label="Clear"
					>
						<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
					</button>
				{/if}
				<kbd class="rounded border border-border/60 bg-muted px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">Esc</kbd>
			</div>

			<!-- Loading progress bar: fills smoothly over 3 s, rushes when Enter is pressed -->
			{#if isSearching}
				<div class="h-0.5 w-full overflow-hidden bg-muted">
					<div
						class="h-full bg-primary"
						style="width: {loadingWidth}%; transition: width {loadingDuration}ms linear"
					></div>
				</div>
			{:else if !searchResults && searchQuery.trim()}
				<div class="border-t border-border/40 px-5 py-3 text-center text-sm text-muted-foreground">
					Press <kbd class="rounded border border-border/60 bg-muted px-1 py-0.5 font-mono text-[10px]">↵</kbd> or wait 3 s to search
				</div>
			{/if}
		</div>

		<!-- Results grid — wider panel below the search box -->
		{#if searchResults}
			{@const q = searchQuery.trim()}
			{@const hasAny = searchResults.anime.data.length || searchResults.manga.data.length || searchResults.characters.data.length || searchResults.staff.data.length || searchResults.studios.data.length || searchResults.users.data.length}

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
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Anime</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.anime.data as item}
									<button onclick={() => navigateAndClose(`/anime/${item.id}`)} class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40">
										{#if item.coverImage?.medium}
											<img src={item.coverImage.medium} alt="" class="h-9 w-6 shrink-0 rounded object-cover" />
										{:else}
											<div class="h-9 w-6 shrink-0 rounded bg-muted"></div>
										{/if}
										<div class="min-w-0 flex-1">
											<p class="truncate text-sm leading-snug">{item.title?.userPreferred ?? item.title?.romaji ?? '—'}</p>
											<p class="text-[11px] text-muted-foreground">{item.format ? String(item.format).replace(/_/g, ' ') : ''}
												{#if item.meanScore}<span class="ml-1 text-yellow-500">{item.meanScore}%</span>{/if}
											</p>
										</div>
									</button>
								{/each}
							</div>
							{#if searchResults.anime.data.length}
								<button onclick={() => navigateAndClose(`/browse/anime`)} class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30">
									View all anime results
								</button>
							{/if}
						</div>

						<!-- Manga -->
						<div class="flex flex-col">
							<div class="flex items-center justify-between border-b border-border/40 px-4 py-2.5">
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Manga</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.manga.data as item}
									<button onclick={() => navigateAndClose(`/manga/${item.id}`)} class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40">
										{#if item.coverImage?.medium}
											<img src={item.coverImage.medium} alt="" class="h-9 w-6 shrink-0 rounded object-cover" />
										{:else}
											<div class="h-9 w-6 shrink-0 rounded bg-muted"></div>
										{/if}
										<div class="min-w-0 flex-1">
											<p class="truncate text-sm leading-snug">{item.title?.userPreferred ?? item.title?.romaji ?? '—'}</p>
											<p class="text-[11px] text-muted-foreground">{item.format ? String(item.format).replace(/_/g, ' ') : ''}
												{#if item.meanScore}<span class="ml-1 text-yellow-500">{item.meanScore}%</span>{/if}
											</p>
										</div>
									</button>
								{/each}
							</div>
							{#if searchResults.manga.data.length}
								<button onclick={() => navigateAndClose(`/browse/manga`)} class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30">
									View all manga results
								</button>
							{/if}
						</div>

						<!-- Characters -->
						<div class="flex flex-col">
							<div class="flex items-center justify-between border-b border-border/40 px-4 py-2.5">
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Characters</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.characters.data as item}
									<button onclick={() => navigateAndClose(`/character/${item.id}`)} class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40">
										{#if item.image?.medium}
											<img src={item.image.medium} alt="" class="h-8 w-8 shrink-0 rounded-full object-cover" />
										{:else}
											<div class="h-8 w-8 shrink-0 rounded-full bg-muted"></div>
										{/if}
										<span class="flex-1 truncate text-sm">{item.name?.userPreferred ?? item.name?.full ?? '—'}</span>
									</button>
								{/each}
							</div>
							{#if searchResults.characters.data.length}
								<button onclick={() => navigateAndClose(`/search/characters`)} class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30">
									View all character results
								</button>
							{/if}
						</div>

					</div>

					<!-- Divider between rows -->
					<div class="border-t border-border/60"></div>

					<!-- Row 2 — Staff | Studios | Users -->
					<div class="grid grid-cols-3 divide-x divide-border/40">

						<!-- Staff -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Staff</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.staff.data as item}
									<button onclick={() => navigateAndClose(`/staff/${item.id}`)} class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40">
										{#if item.image?.medium}
											<img src={item.image.medium} alt="" class="h-8 w-8 shrink-0 rounded-full object-cover" />
										{:else}
											<div class="h-8 w-8 shrink-0 rounded-full bg-muted"></div>
										{/if}
										<span class="flex-1 truncate text-sm">{item.name?.userPreferred ?? item.name?.full ?? '—'}</span>
									</button>
								{/each}
							</div>
							{#if searchResults.staff.data.length}
								<button onclick={() => navigateAndClose(`/search/staff`)} class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30">
									View all staff results
								</button>
							{/if}
						</div>

						<!-- Studios -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Studios</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.studios.data as item}
									<button onclick={() => navigateAndClose(`/studio/${item.id}`)} class="flex w-full items-center gap-2.5 px-4 py-3 text-left transition-colors hover:bg-muted/40">
										<span class="flex-1 truncate text-sm">{item.name ?? '—'}</span>
										{#if item.isAnimationStudio}<span class="shrink-0 rounded bg-primary/10 px-1.5 py-0.5 text-[10px] text-primary">Animation</span>{/if}
									</button>
								{/each}
							</div>
							{#if searchResults.studios.data.length}
								<button onclick={() => navigateAndClose(`/search/studios`)} class="border-t border-border/40 py-2.5 text-center text-xs text-primary transition-colors hover:bg-muted/30">
									View all studio results
								</button>
							{/if}
						</div>

						<!-- Users -->
						<div class="flex flex-col">
							<div class="border-b border-border/40 px-4 py-2.5">
								<span class="text-[11px] font-semibold uppercase tracking-widest text-muted-foreground">Users</span>
							</div>
							<div class="flex-1 py-1">
								{#each searchResults.users.data as item}
									<button onclick={() => navigateAndClose(`/user/${item.name}`)} class="flex w-full items-center gap-2.5 px-4 py-2 text-left transition-colors hover:bg-muted/40">
										{#if item.avatar?.medium}
											<img src={item.avatar.medium} alt="" class="h-8 w-8 shrink-0 rounded-full object-cover" />
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
			<div class="mt-3 rounded-xl border border-border/50 bg-background px-10 py-6 text-center text-sm text-destructive shadow-2xl">{searchError}</div>
		{/if}

		<!-- Hint at the very bottom so it doesn't eat into the results -->
		<div class="mt-3 flex items-center gap-4 text-xs text-white/40">
			<span class="flex items-center gap-1"><kbd class="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 font-mono text-[10px]">↵</kbd> search</span>
			<span class="flex items-center gap-1"><kbd class="rounded border border-white/20 bg-white/10 px-1.5 py-0.5 font-mono text-[10px]">Esc</kbd> close</span>
			<span class="text-white/30">Powered by AniList</span>
		</div>
	</div>
{/if}

<style>
	[data-tauri-drag-region] {
		-webkit-app-region: drag;
	}

	[data-tauri-drag-region='false'],
	:global(button),
	:global(a),
	:global([role='button']) {
		-webkit-app-region: no-drag;
	}
</style>
