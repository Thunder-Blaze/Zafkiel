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
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';

	let { children }: { children: any } = $props();

	// Initialize UI scale (applies Tauri webview zoom)
	const uiScale = useUiScale();

	// Initialize config and auth stores on app mount
	onMount(async () => {
		await configStore.init();
		await authStore.init();
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
