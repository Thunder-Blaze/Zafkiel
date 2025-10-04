<script lang="ts">
  import { browser } from '$app/environment'
  import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query'

  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
				staleTime: 1000 * 60 * 5, // 5 minutes
				refetchOnWindowFocus: false,
      },
    },
  })

	let { children } = $props();
</script>

<QueryClientProvider client={queryClient} children={children} />
