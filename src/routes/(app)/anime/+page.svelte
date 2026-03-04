<script lang="ts">
	import MediaCard from '$lib/components/MediaCard.svelte';
	import { useTrendingAnime } from '$lib/hooks/useAnilist.svelte';
	import type { Media } from '$lib/types/anilist';

	const trendingQuery = useTrendingAnime();

	let animeData: Media[] = $state([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	$effect(() => {
		isLoading = trendingQuery.isLoading;
		error = trendingQuery.error ? String(trendingQuery.error) : null;
		animeData = trendingQuery.data?.data || [];
	});

	$inspect(() => {
		console.log('Trending anime data:', animeData);
		console.log('Loading:', isLoading);
		console.log('Error:', error);
	});
</script>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
	{#if isLoading}
		<p class="col-span-full text-center text-gray-500">Loading trending anime...</p>
	{:else if error}
		<p class="col-span-full text-center text-red-500">Error: {error}</p>
	{:else if animeData && animeData.length > 0}
		{#each animeData as anime (anime.id)}
			<MediaCard media={anime} />
		{/each}
	{:else}
		<p class="col-span-full text-center text-gray-500">No trending anime found.</p>
	{/if}
</div>
