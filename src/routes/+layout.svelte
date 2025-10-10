<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemedToaster from '$lib/components/ThemedToaster.svelte';
	import ContextMenu from '$lib/components/ContextMenu.svelte';
	import ContextMenuProvider from '$lib/providers/context-menu.svelte';
	import TanstackProvider from '$lib/providers/tanstack.svelte';
	import LenisProvider from '$lib/providers/lenis.svelte';
	import AnimationProvider from '$lib/providers/animation.svelte';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { configStore } from '$lib/stores/config';
	import { authStore } from '$lib/stores/auth';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { themeStore } from '$lib/stores/theme.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';

	let { children }: { children: any } = $props();

	let isReady = $state(false);
	let uiScale = useUiScale();

	// Initialize config, auth, and theme stores on app mount
	onMount(async () => {
		try {
			// Initialize stores sequentially, with theme first
			await themeStore.initialize();
			await configStore.init();
			await authStore.init();

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

{#if isReady}
	<!-- Custom Title Bar -->
	<TitleBar />

	<!-- Portals at root level - webview zoom scales everything -->
	<ThemedToaster />
	<ContextMenu />

	<!-- Main app content -->
	<TanstackProvider>
		<LenisProvider>
			<ContextMenuProvider>
				<AnimationProvider>
					{@render children?.()}
				</AnimationProvider>
			</ContextMenuProvider>
		</LenisProvider>
		<SvelteQueryDevtools />
	</TanstackProvider>
{:else}
	<!-- Loading state with theme-aware background -->
	<Loader text="Loading Zafkiel..." />
{/if}
