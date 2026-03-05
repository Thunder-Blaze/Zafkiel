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
	} from '$lib/utils/keybindings';
	import { searchOverlay } from '$lib/stores/search-overlay.svelte';

	const appWindow: TauriWindow | null = browser ? getCurrentWindow() : null;

	let isMaximized = $state(false);
	let isFullscreen = $state(false);

	// Navigation history management
	let navigationHistory = $state<string[]>([]);
	let currentHistoryIndex = $state(-1);
	let canGoBack = $derived(currentHistoryIndex > 0);
	let canGoForward = $derived(currentHistoryIndex < navigationHistory.length - 1);

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

	const navItems = [
		{ path: '/activity', icon: 'solar:history-2-bold', label: 'Activity' },
		{ path: '/forum', icon: 'solar:chat-square-bold', label: 'Forum' },
		{ path: '/downloads', icon: 'solar:download-bold', label: 'Downloads' },
		{ path: '/settings', icon: 'solar:settings-bold', label: 'Settings' },
	];

	// Track window width to hide wide-only nav items at narrow widths
	let windowWidth = $state(browser ? window.innerWidth : 1920);
	$effect(() => {
		if (!browser) return;
		const onResize = () => {
			windowWidth = window.innerWidth;
		};
		window.addEventListener('resize', onResize);
		return () => window.removeEventListener('resize', onResize);
	});
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
				class="flex cursor-pointer items-center gap-2 rounded-lg px-2 transition-colors hover:bg-primary/5 active:bg-primary/10"
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

				<!-- Remaining nav items (Social/Forum/Settings hidden below 1366px) -->
				{#each navItems as item}
					{#if item.path === '/downloads' || windowWidth >= 1366}
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
					{/if}
				{/each}
			</div>
		</div>

		<!-- Right Section: Search icon + Notifications + Profile + Theme + Window Controls -->
		<div class="flex h-full items-center">
			<!-- Fake search bar -->
			<button
				type="button"
				onclick={searchOverlay.show}
				title="Search ({isMac ? '⌘K' : 'Ctrl+K'})"
				class="mr-2 flex h-8 w-48 items-center gap-2 rounded-md bg-muted/40 px-3 text-sm text-muted-foreground transition-colors hover:bg-muted/70 hover:text-foreground"
			>
				<Icon icon="solar:magnifer-bold" class="h-3.5 w-3.5 shrink-0" />
				<span class="flex-1 text-left text-xs">Search…</span>
				<span class="flex items-center gap-0.5">
					<kbd
						class="rounded border border-border/50 bg-background/50 px-1.5 py-0.5 font-mono text-[10px] leading-none"
						>{isMac ? '⌘' : 'Ctrl'}</kbd
					>
					<kbd
						class="rounded border border-border/50 bg-background/50 px-1.5 py-0.5 font-mono text-[10px] leading-none"
						>K</kbd
					>
				</span>
			</button>
			<Button
				variant="ghost"
				size="icon"
				class="relative h-8 w-8 {page.url.pathname === '/notifications'
					? 'text-primary'
					: 'text-foreground/70 hover:text-foreground'}"
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
