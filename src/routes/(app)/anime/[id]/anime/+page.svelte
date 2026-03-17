<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import WatchSection from './WatchSection.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import { filterTitle } from '$lib/utils/data-filters';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
	const title = $derived(animeData ? filterTitle(animeData.title || {}) : '');
</script>

{#if animeData}
	<div class="mt-2">
		<h2 class="mb-4 text-xl font-semibold">Watch</h2>
		<WatchSection animeTitle={title} {animeId} />
	</div>
{/if}
