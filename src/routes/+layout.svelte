<script lang="ts">
	import { onMount } from 'svelte';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemedToaster from '$lib/components/ThemedToaster.svelte';
	import ContextMenu from '$lib/components/ContextMenu.svelte';
	import ContextMenuProvider from '$lib/providers/context-menu.svelte';
	import TanstackProvider from '$lib/providers/tanstack.svelte';
	import LenisProvider from '$lib/providers/lenis.svelte';
	import { configStore } from '$lib/stores/config';
	import { authStore } from '$lib/stores/auth';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';

	let { children }: { children: any } = $props();

	// Initialize UI scale (applies Tauri webview zoom)
	const uiScale = useUiScale();

	// Initialize config, auth, and theme stores on app mount
	onMount(async () => {
		try {
			await configStore.init();
			await authStore.init();

			// Initialize theme store in background (non-blocking)
			themeStore.initialize().catch((error) => {
				console.warn('Theme store initialization failed, using defaults:', error);
			});
		} catch (error) {
			console.error('Failed to initialize app stores:', error);
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<!-- Portals at root level - webview zoom scales everything -->
<ThemedToaster />
<ContextMenu />

<!-- Main app content -->
<TanstackProvider>
	<LenisProvider>
		<ContextMenuProvider>
			{@render children?.()}
		</ContextMenuProvider>
	</LenisProvider>
</TanstackProvider>
