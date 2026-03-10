<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import TorrentsList from '../anime/TorrentsList.svelte';
	import type { AnimeLarge } from '$lib/types/anime';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
</script>

{#if animeData}
	<div class="mt-6">
		<TorrentsList anime={animeData} />
	</div>
{/if}
