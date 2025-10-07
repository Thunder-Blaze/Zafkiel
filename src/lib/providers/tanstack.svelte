<script lang="ts">
	import { browser } from '$app/environment';
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';

	// Only create QueryClient in browser
	const queryClient = browser ? new QueryClient({
		defaultOptions: {
			queries: {
				enabled: true,
				staleTime: 1000 * 60 * 5, // 5 minutes
				refetchOnWindowFocus: false,
			},
		},
	}) : null;

	let { children } = $props();
</script>

{#if browser && queryClient}
	<QueryClientProvider client={queryClient} {children} />
{:else}
	{@render children?.()}
{/if}
