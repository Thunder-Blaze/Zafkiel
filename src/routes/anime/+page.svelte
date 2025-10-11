<script lang="ts">
	import MediaCard from "$lib/components/MediaCard.svelte";
	import { useTrendingAnime } from "$lib/hooks/useAnilist.svelte";
	import type { Media } from "$lib/types/anilist";

	let animeData: Media[] = $state([]);

	$effect(() => {
		const { data, error } = useTrendingAnime();

		if (error) {
			console.error("Error fetching trending anime:", error);
			animeData = [];
			return;
		}

		animeData = data?.data || [];
	});
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
	{#if animeData && animeData.length > 0}
		{#each animeData as anime (anime.id)}
			<MediaCard media={anime} />
		{/each}
	{:else}
		<p class="col-span-full text-center text-gray-500">Loading trending anime...</p>
	{/if}
</div>
