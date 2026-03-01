<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
	import ProfileDropdown from '$lib/components/ProfileDropdown.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { DropdownMenu } from 'bits-ui';
	import { Window, getCurrentWindow } from '@tauri-apps/api/window';
	import type { UnlistenFn } from '@tauri-apps/api/event';
	import {
		initializeKeybindings,
		cleanupKeybindings,
		updateKeyBindingAction,
		formatKeyBinding,
	} from '$lib/utils/keybindings';

	const appWindow: Window | null = browser ? getCurrentWindow() : null;

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
	let browseOpen = $state(false);
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

	function focusSearch(): void {
		searchOpen = true;
	}

	$effect(() => {
		if (searchOpen && searchInput) {
			// Tick lets the DOM render first
			queueMicrotask(() => searchInput?.focus());
		}
	});

	function handleSearchKeydown(e: KeyboardEvent): void {
		if (e.key === 'Escape') {
			searchOpen = false;
			searchQuery = '';
		}
	}

	function handleSearchSubmit(): void {
		if (searchQuery.trim()) {
			searchOpen = false;
			goto(`/search?search=${encodeURIComponent(searchQuery.trim())}`);
			searchQuery = '';
		}
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
				<DropdownMenu.Root bind:open={browseOpen}>
					<DropdownMenu.Trigger
						class="flex h-8 items-center gap-1.5 rounded-md px-2.5 text-xs font-medium transition-colors
							{browseOpen ? 'bg-primary/10 text-primary' : 'text-foreground/70 hover:bg-foreground/5 hover:text-foreground'}"
					>
						<Icon icon="solar:compass-bold-duotone" class="h-4 w-4" />
						Browse
						<Icon icon="solar:alt-arrow-down-linear" class="h-3 w-3 transition-transform {browseOpen ? 'rotate-180' : ''}" />
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						class="z-50 w-72 rounded-xl border border-border/40 bg-background/95 p-3.5 shadow-xl backdrop-blur-xl"
						sideOffset={8}
						align="start"
					>
						<!-- Anime -->
						<div class="mb-2.5 flex items-center gap-3 rounded-lg bg-muted/40 px-3 py-2.5">
							<Icon icon="solar:play-circle-bold-duotone" class="h-5 w-5 shrink-0 text-primary" />
							<div class="flex flex-col">
								<button
									onclick={() => { browseOpen = false; goto('/search/anime'); }}
									class="mb-1 text-left text-sm font-semibold text-foreground transition-colors hover:text-primary"
								>
									Anime
								</button>
							<div class="flex items-center whitespace-nowrap text-xs text-muted-foreground">
									<button onclick={() => { browseOpen = false; goto('/browse/anime?sort=POPULARITY_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Top 100</button>
									<span class="mx-2 opacity-30">·</span>
									<button onclick={() => { browseOpen = false; goto('/browse/anime?sort=TRENDING_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Trending</button>
									<span class="mx-2 opacity-30">·</span>
									<button onclick={() => { browseOpen = false; goto('/browse/anime?format=MOVIE&sort=SCORE_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Top Movies</button>
								</div>
							</div>
						</div>

						<!-- Manga -->
						<div class="mb-1.5 flex items-center gap-3 rounded-lg bg-muted/40 px-3 py-2.5">
							<Icon icon="solar:book-2-bold-duotone" class="h-5 w-5 shrink-0 text-primary" />
							<div class="flex flex-col">
								<button
									onclick={() => { browseOpen = false; goto('/search/manga'); }}
									class="mb-1 text-left text-sm font-semibold text-foreground transition-colors hover:text-primary"
								>
									Manga
								</button>
							<div class="flex items-center whitespace-nowrap text-xs text-muted-foreground">
									<button onclick={() => { browseOpen = false; goto('/browse/manga?sort=POPULARITY_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Top 100</button>
									<span class="mx-2 opacity-30">·</span>
									<button onclick={() => { browseOpen = false; goto('/browse/manga?sort=TRENDING_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Trending</button>
									<span class="mx-2 opacity-30">·</span>
									<button onclick={() => { browseOpen = false; goto('/browse/manga?country=KR&sort=POPULARITY_DESC'); }} class="py-0.5 transition-colors hover:text-foreground">Top Manhwa</button>
								</div>
							</div>
						</div>

						<!-- Other -->
						<div class="grid grid-cols-[auto_auto] justify-between gap-x-2">
							<button onclick={() => { browseOpen = false; goto('/search/staff'); }} class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground">
								<Icon icon="solar:users-group-two-rounded-bold-duotone" class="h-3.5 w-3.5" />
								Staff
							</button>
							<button onclick={() => { browseOpen = false; goto('/search/characters'); }} class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground">
								<Icon icon="solar:user-circle-bold-duotone" class="h-3.5 w-3.5" />
								Characters
							</button>
							<button onclick={() => { browseOpen = false; goto('/search/reviews'); }} class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground">
								<Icon icon="solar:star-bold-duotone" class="h-3.5 w-3.5" />
								Reviews
							</button>
							<button onclick={() => { browseOpen = false; goto('/search/recommendations'); }} class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground">
								<Icon icon="solar:like-bold-duotone" class="h-3.5 w-3.5" />
								Recommendations
							</button>
						</div>
					</DropdownMenu.Content>
				</DropdownMenu.Root>

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
				class="flex h-8 w-48 items-center gap-2 rounded-md bg-muted/40 px-3 text-sm text-muted-foreground transition-colors hover:bg-muted/70 hover:text-foreground"
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

	<div class="h-12"></div>
{/if}

<!-- Search portal — rendered outside the titlebar stacking context -->
{#if browser && searchOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="fixed inset-0 z-[1000000] flex items-start justify-center bg-black/60 pt-16 backdrop-blur-md"
		onclick={(e) => { if (e.target === e.currentTarget) { searchOpen = false; searchQuery = ''; } }}
		onkeydown={handleSearchKeydown}
	>
		<div class="w-full max-w-2xl overflow-hidden rounded-2xl border border-border/50 bg-background shadow-2xl">
			<!-- Input row -->
			<form
				onsubmit={(e) => { e.preventDefault(); handleSearchSubmit(); }}
				class="flex items-center gap-3 border-b border-border/40 px-5 py-4"
			>
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
						onclick={() => (searchQuery = '')}
						class="rounded-md p-1 text-muted-foreground transition-colors hover:text-foreground"
						aria-label="Clear"
					>
						<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
					</button>
				{/if}
			</form>
			<!-- Footer hint row -->
			<div class="flex items-center justify-between px-5 py-2.5">
				<div class="flex items-center gap-4 text-xs text-muted-foreground">
					<span class="flex items-center gap-1.5">
						<kbd class="rounded border border-border/60 bg-muted px-1.5 py-0.5 font-mono text-[10px]">↵</kbd>
						to search
					</span>
					<span class="flex items-center gap-1.5">
						<kbd class="rounded border border-border/60 bg-muted px-1.5 py-0.5 font-mono text-[10px]">Esc</kbd>
						to close
					</span>
				</div>
				<span class="text-xs text-muted-foreground/50">Powered by AniList</span>
			</div>
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
