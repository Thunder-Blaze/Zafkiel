<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import CharactersList from '../anime/CharactersList.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
	const isLoading = $derived(animeQuery.isLoading);
</script>

<div class="mt-6 flex flex-col gap-4">
	{#if isLoading && !animeData?.characters}
		<PageLoader type="anime" />
	{:else if animeData}
		<CharactersList
			characters={animeData.characters?.edges
				?.map((edge) =>
					edge.node
						? {
								id: edge.node.id,
								name: edge.node.name || {
									first: '',
									last: '',
									full: 'Unknown',
									native: '',
									userPreferred: 'Unknown',
								},
								image: {
									large: edge.node.image?.large || '/api/placeholder/230/345',
									medium: edge.node.image?.medium || '/api/placeholder/115/172',
								},
								description: edge.node.description,
								role: edge.role || 'Unknown',
								voiceActors: edge.voiceActors?.map((va) => ({
									id: va.id,
									name: va.name || {
										first: '',
										last: '',
										full: '',
										native: '',
									},
									image: {
										large: va.image?.large || '/api/placeholder/230/345',
										medium: va.image?.medium || '/api/placeholder/115/172',
									},
									languageV2: va.languageV2 || 'Unknown',
								})),
							}
						: null
				)
				.filter((char): char is NonNullable<typeof char> => char !== null) || []}
			isLoading={isLoading}
		/>
	{/if}
</div>
