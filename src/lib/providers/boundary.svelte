<script>
	import Loader from "$lib/components/Loader.svelte";

	let { children } = $props()
</script>

<svelte:boundary>
	{#snippet failed(error, reset)}
		<button onclick={reset}>Reload</button>
	{/snippet}

	{#snippet pending()}
		<Loader text="Loading Zafkiel..." />
	{/snippet}

	{@render children()}
</svelte:boundary>
