<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeCharactersById } from '$lib/hooks/useAnilist.svelte';
	import CharactersList from '../anime/CharactersList.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);

	let selectedLanguage = $state('Japanese');
	let charactersPage = $state(1);
	let charactersPerPage = $state(25);

	const animeQuery = $derived(
		useAnimeCharactersById(animeId, charactersPage, charactersPerPage, selectedLanguage)
	);
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
	const isLoading = $derived(animeQuery.isLoading);
</script>

<div class="mt-2 flex flex-col gap-4">
	<div class="mb-4 flex items-center justify-between">
		<h2 class="text-xl font-semibold">Characters</h2>
		<div class="flex items-center gap-2">
			<label for="va-language" class="mr-1 text-xs font-medium text-muted-foreground"
				>Voice Actor Language:</label
			>
			<select
				id="va-language"
				bind:value={selectedLanguage}
				class="rounded-md border border-border bg-background px-3 py-1.5 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
			>
				<option value="Japanese">Japanese</option>
				<option value="English">English</option>
				<option value="Korean">Korean</option>
				<option value="Spanish">Spanish</option>
				<option value="French">French</option>
				<option value="Portuguese">Portuguese</option>
				<option value="German">German</option>
				<option value="Italian">Italian</option>
			</select>
		</div>
	</div>

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
								voiceActors:
									edge.voiceActorRoles?.map((role) => ({
										id: role.voiceActor?.id || 0,
										name: role.voiceActor?.name || {
											first: '',
											last: '',
											full: '',
											native: '',
										},
										image: {
											large: role.voiceActor?.image?.large || '/api/placeholder/230/345',
											medium: role.voiceActor?.image?.medium || '/api/placeholder/115/172',
										},
										languageV2:
											role.voiceActor?.languageV2 || role.voiceActor?.language || selectedLanguage,
									})) || [],
							}
						: null
				)
				.filter((char): char is NonNullable<typeof char> => char !== null) || []}
			{isLoading}
		/>
	{/if}
</div>
