<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import ReviewsList from '../anime/ReviewsList.svelte';
	import type { AnimeLarge } from '$lib/types/anime';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
</script>

{#if animeData}
	<div class="mt-6">
		<ReviewsList mediaId={animeId} />
	</div>
{/if}
