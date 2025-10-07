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

	let { children } = $props();

	// Initialize config store on app mount
	onMount(async () => {
		await configStore.init();
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<TanstackProvider>
	<LenisProvider>
		<ContextMenuProvider>
			<ThemedToaster />
			<ContextMenu />
			{@render children?.()}
		</ContextMenuProvider>
	</LenisProvider>
</TanstackProvider>
