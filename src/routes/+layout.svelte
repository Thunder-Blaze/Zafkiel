<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemedToaster from '$lib/components/ThemedToaster.svelte';
	import TanstackProvider from '$lib/providers/tanstack.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<TanstackProvider>
	<ThemedToaster />
	{@render children?.()}
</TanstackProvider>
