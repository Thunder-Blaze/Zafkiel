<script lang="ts">
	import { page } from '$app/state';
	import { useMediaRecommendations } from '$lib/hooks/useAnilist.svelte';
	import RecommendationCard from '$lib/components/RecommendationCard.svelte';
	import Icon from '@iconify/svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const recsQuery = $derived(useMediaRecommendations(animeId, 1, 20));
	const recs = $derived(recsQuery.data?.data?.data ?? []);
</script>

<div class="mt-6">
	{#if recsQuery.isLoading}
		<div class="flex items-center justify-center p-8">
			<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
		</div>
	{:else if recs.length > 0}
		<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
			{#each recs as rec}
				<RecommendationCard recommendation={rec} />
			{/each}
		</div>
	{:else}
		<div
			class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-8 text-center"
		>
			<Icon icon="solar:like-bold-duotone" class="h-12 w-12 text-muted-foreground" />
			<h3 class="text-lg font-semibold">No Recommendations</h3>
			<p class="text-sm text-muted-foreground">No recommendations available yet.</p>
		</div>
	{/if}
</div>
