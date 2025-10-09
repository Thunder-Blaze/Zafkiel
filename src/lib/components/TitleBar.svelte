<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
	import ProfileDropdown from '$lib/components/ProfileDropdown.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	let appWindow: any = null;
	let isMaximized = $state(false);
	let isFullscreen = $state(false);

	// Initialize Tauri window only in browser
	onMount(async () => {
		if (browser) {
			try {
				const { getCurrentWindow } = await import('@tauri-apps/api/window');
				appWindow = getCurrentWindow();

				// Check initial states
				isMaximized = await appWindow.isMaximized();
				isFullscreen = await appWindow.isFullscreen();
			} catch (error) {
				console.error('[TitleBar] Failed to initialize window:', error);
			}
		}
	});

	function minimize(): void {
		if (appWindow) {
			appWindow.minimize().catch((err: any) => {
				console.error('[TitleBar] Failed to minimize:', err);
			});
		}
	}

	function toggleMaximize(): void {
		if (appWindow) {
			appWindow.isMaximized().then((maximized: boolean) => {
				appWindow.toggleMaximize().then(() => {
					isMaximized = !maximized;
				});
			}).catch((err: any) => {
				console.error('[TitleBar] Failed to toggle maximize:', err);
			});
		}
	}

	function close(): void {
		if (appWindow) {
			appWindow.close().catch((err: any) => {
				console.error('[TitleBar] Failed to close window:', err);
			});
		}
	}	// Navigation items
	const navItems = [
		{ path: '/', icon: 'solar:home-bold', label: 'Home' },
		{ path: '/anime', icon: 'solar:video-library-bold', label: 'Anime' },
		{ path: '/media-demo', icon: 'solar:gallery-bold', label: 'Demo' },
		{ path: '/settings', icon: 'solar:settings-bold', label: 'Settings' },
	];
</script>

<!-- Only render in browser and when not in fullscreen -->
{#if browser && !isFullscreen}
	<div
		data-tauri-drag-region
		class="fixed top-0 left-0 right-0 z-50 flex h-12 select-none items-center justify-between border-b border-border/50 bg-background/95 backdrop-blur-xl"
	>
		<!-- Left: App branding and navigation -->
		<div class="flex h-full items-center gap-2 pl-3" data-tauri-drag-region>
			<!-- App Logo & Name -->
			<div class="flex items-center gap-2 px-2" data-tauri-drag-region>
				<div class="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-primary/70 shadow-lg shadow-primary/20">
					<Icon icon="solar:clock-circle-bold" class="h-4 w-4 text-primary-foreground" />
				</div>
				<div class="flex items-baseline gap-1" data-tauri-drag-region>
					<span class="text-base font-bold tracking-tight">Zafkiel</span>
					<sup class="text-[8px] font-semibold text-amber-600 dark:text-amber-400 border border-amber-500/30 bg-amber-500/10 px-1 py-1 rounded">
						ALPHA
					</sup>
				</div>
			</div>

			<!-- Navigation -->
			<div class="ml-2 flex h-full items-center gap-1">
				{#each navItems as item}
					{@const isActive = $page.url.pathname === item.path}
					<Button
						variant="ghost"
						size="sm"
						class="h-8 gap-2 {isActive ? 'bg-primary/10 text-primary' : 'text-foreground/70 hover:text-foreground'}"
						onclick={() => goto(item.path)}
					>
						<Icon icon={item.icon} class="h-4 w-4" />
						<span class="text-xs font-medium">{item.label}</span>
					</Button>
				{/each}
			</div>
		</div>

		<!-- Right: Profile, Theme switcher and window controls -->
		<div class="flex h-full items-center">
			<!-- Profile Dropdown -->
			<div class="px-2">
				<ProfileDropdown />
			</div>

			<!-- Theme Switcher -->
			<div class="px-2">
				<ThemeSwitcher />
			</div>

			<!-- Window Controls -->
			<div class="flex h-full items-center" data-tauri-drag-region="false">
				<!-- Minimize -->
				<button
					type="button"
					onclick={(e) => {
						e.preventDefault();
						e.stopPropagation();
						minimize();
					}}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
					aria-label="Minimize"
					data-tauri-drag-region="false"
				>
					<Icon icon="solar:minus-circle-line-duotone" class="h-5 w-5 text-foreground/70 hover:text-foreground" />
				</button>

				<!-- Maximize/Restore -->
				<button
					type="button"
					onclick={(e) => {
						e.preventDefault();
						e.stopPropagation();
						toggleMaximize();
					}}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
					aria-label={isMaximized ? 'Restore' : 'Maximize'}
					data-tauri-drag-region="false"
				>
					<Icon
						icon={isMaximized ? 'solar:minimize-square-2-line-duotone' : 'solar:square-top-down-line-duotone'}
						class="h-5 w-5 text-foreground/70 hover:text-foreground"
					/>
				</button>

				<!-- Close -->
				<button
					type="button"
					onclick={(e) => {
						e.preventDefault();
						e.stopPropagation();
						close();
					}}
					class="flex h-full w-11 items-center justify-center transition-colors hover:bg-red-500 hover:text-white active:bg-red-600"
					aria-label="Close"
					data-tauri-drag-region="false"
				>
					<Icon icon="solar:close-circle-line-duotone" class="h-5 w-5" />
				</button>
			</div>
		</div>
	</div>

	<!-- Spacer to push content below title bar -->
	<div class="h-12"></div>
{/if}

<style lang="postcss">
	[data-tauri-drag-region] {
		-webkit-app-region: drag;
	}

	[data-tauri-drag-region="false"],
	:global(button),
	:global(a),
	:global([role="button"]) {
		-webkit-app-region: no-drag;
	}
</style>
