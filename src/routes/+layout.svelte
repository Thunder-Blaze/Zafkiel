<script lang="ts">
	import { onMount } from 'svelte';
	import { onNavigate } from '$app/navigation';
	import { browser } from '$app/environment';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemedToaster from '$lib/components/ThemedToaster.svelte';
	import ContextMenu from '$lib/components/ContextMenu.svelte';
	import ContextMenuProvider from '$lib/providers/context-menu.svelte';
	import SearchOverlay from '$lib/components/SearchOverlay.svelte';
	import TanstackProvider from '$lib/providers/tanstack.svelte';
	import LenisProvider from '$lib/providers/lenis.svelte';
	import AnimationProvider from '$lib/providers/animation.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { useConfigState } from '$lib/stores/config.svelte';
	import { authStore } from '$lib/stores/auth';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { useThemeState } from '$lib/stores/theme.svelte';
	import { playerStore } from '$lib/stores/player.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';

	import { ExtensionManager } from '$lib/services/ExtensionManager';
	import { SubsPleaseExtension } from '../extensions/torrent/subsplease';
	import { extensionStore } from '$lib/stores/extensionStore.svelte';

	let { children }: { children: any } = $props();

	let isReady = $state(false);
	let uiScale = useUiScale();
	let config = useConfigState();
	let theme = useThemeState();

	// Scroll container — fixed below the titlebar so the scrollbar never overlaps it
	let scrollEl = $state<HTMLElement | null>(null);

	// View transitions gated on config.animations
	if (browser) {
		onNavigate((navigation) => {
			if (!isReady || !config.animations) return;
			// Use View Transitions API when available
			if (document.startViewTransition) {
				return new Promise((resolve) => {
					document.startViewTransition(async () => {
						resolve();
						await navigation.complete;
					});
				});
			}
		});
	}

	// When the mpv player overlay is active, make html/body background transparent
	// so WebView2 renders alpha=0 pixels, revealing the native video layer underneath.
	// visibility:hidden on the UI group alone isn't enough — the body background is
	// still painted opaque and blocks the native mpv surface below the WebView.
	$effect(() => {
		if (playerStore.active) {
			document.documentElement.style.background = 'transparent';
			document.body.style.background = 'transparent';
		} else {
			document.documentElement.style.background = '';
			document.body.style.background = '';
		}
	});

	// Initialize config, auth, and theme stores on app mount
	onMount(async () => {
		try {
			// Register extensions
			ExtensionManager.registerProvider(SubsPleaseExtension);

			// Initialize stores sequentially, with theme first
			await config.init();
			await theme.init();
			await authStore.init();

			// Load installed-extension index (fast – reads index.json only)
			await extensionStore.init();

			// Mark as ready once theme is loaded
			isReady = true;
		} catch (error) {
			console.error('Failed to initialize app stores:', error);
			// Still mark as ready to prevent blank screen
			isReady = true;
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<svelte:body />

<svelte:boundary>
	{#snippet pending()}
		<div class="flex h-screen flex-col">
			<TitleBar />
			<div class="relative flex-1">
				<Loader text="Initializing Zafkiel..." />
			</div>
		</div>
	{/snippet}

	{#snippet failed(error, reset)}
		{@const message = error instanceof Error ? error.message : String(error)}
		{@const stack = error instanceof Error ? (error.stack ?? '') : ''}
		<div class="flex h-screen flex-col">
			<TitleBar />
			<div class="flex flex-1 flex-col items-center justify-center gap-4 p-8">
				<p class="text-lg font-semibold text-destructive">Something went wrong</p>
				<pre
					class="max-h-64 w-full max-w-2xl overflow-auto rounded-lg border border-destructive/30 bg-destructive/5 p-4 text-xs text-destructive">{message}</pre>
				{#if stack}
					<details class="w-full max-w-2xl">
						<summary class="cursor-pointer text-xs text-muted-foreground hover:text-foreground"
							>Stack trace</summary
						>
						<pre
							class="mt-2 max-h-48 overflow-auto rounded border border-border bg-muted/50 p-3 text-xs text-muted-foreground">{stack}</pre>
					</details>
				{/if}
				<button
					onclick={reset}
					class="rounded bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90"
					>Reload</button
				>
			</div>
		</div>
	{/snippet}

	<!-- Grouped UI — hidden instantly when the mpv player overlay is active so the
		     native video layer shows through the transparent WebView beneath it. -->
	<div
		class="contents"
		style:visibility={playerStore.active ? 'hidden' : ''}
		style:pointer-events={playerStore.active ? 'none' : ''}
	>
		<!-- Custom Title Bar -->
		<TitleBar />

		{#if isReady}
			<!-- Portals (except Sonner which must stay visible for notifications) -->
			<ContextMenu />
			<SearchOverlay />

			<!-- Main app content — fixed below titlebar; this owns the scrollbar so it never overlaps titlebar -->
			<div
				bind:this={scrollEl}
				class="fixed inset-x-0 top-12 bottom-0 overflow-x-hidden overflow-y-auto"
			>
				<TanstackProvider>
					<LenisProvider wrapper={scrollEl}>
						<ContextMenuProvider>
							<AnimationProvider>
								{@render children?.()}
							</AnimationProvider>
						</ContextMenuProvider>
					</LenisProvider>
					<SvelteQueryDevtools />
				</TanstackProvider>
			</div>
		{:else}
			<div class="fixed inset-x-0 top-12 bottom-0 bg-background">
				<Loader text="Loading Zafkiel..." />
			</div>
		{/if}
	</div>

	<!-- Sonner toaster stays outside the hidden group so notifications show over the player -->
	<ThemedToaster />
</svelte:boundary>
