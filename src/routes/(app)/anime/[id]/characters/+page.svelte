<script lang="ts">
	import { page } from '$app/state';
	import { useInfiniteAnimeCharactersById } from '$lib/hooks/useAnilist.svelte';
	import CharactersList from '../anime/CharactersList.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import PageLoader from '$lib/components/PageLoader.svelte';

	import * as Select from '$lib/components/ui/select';
	import Icon from '@iconify/svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);

	let selectedLanguage = $state('Japanese');
	let charactersPerPage = $state(25);

	const animeQuery = $derived(
		useInfiniteAnimeCharactersById(animeId, charactersPerPage, selectedLanguage)
	);

	const characters = $derived(
		(animeQuery.data as any)?.pages.flatMap((page: any) => {
			const data = page.data as AnimeLarge | undefined;
			return (
				data?.characters?.edges
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
												role.voiceActor?.languageV2 ||
												role.voiceActor?.language ||
												selectedLanguage,
										})) || [],
								}
							: null
					)
					.filter((char): char is NonNullable<typeof char> => char !== null) || []
			);
		}) || []
	);

	const isLoading = $derived(animeQuery.isLoading);
	const isFetchingNextPage = $derived(animeQuery.isFetchingNextPage);
	const hasNextPage = $derived(animeQuery.hasNextPage);

	const languages = [
		'Japanese',
		'English',
		'Korean',
		'Spanish',
		'French',
		'Portuguese',
		'German',
		'Italian',
	];
</script>

<div class="mt-2 flex flex-col gap-4">
	<div class="mb-4 flex items-center justify-between">
		<h2 class="text-xl font-semibold">Characters</h2>
		<div class="flex items-center gap-3">
			<span class="text-xs font-medium text-muted-foreground">Voice Actor Language:</span>
			<div class="w-[160px]">
				<Select.Root type="single" bind:value={selectedLanguage}>
					<Select.Trigger
						class="h-9 w-full gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary"
					>
						<div class="flex items-center gap-2">
							<Icon icon="solar:global-bold" class="size-4 text-primary" />
							<span class="truncate text-sm font-medium text-foreground">{selectedLanguage}</span>
						</div>
					</Select.Trigger>
					<Select.Content>
						{#each languages as lang}
							<Select.Item value={lang} label={lang}>{lang}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		</div>
	</div>

	{#if isLoading && characters.length === 0}
		<PageLoader type="anime" />
	{:else}
		<CharactersList
			{characters}
			{isLoading}
			{isFetchingNextPage}
			{hasNextPage}
			onFetchNextPage={() => animeQuery.fetchNextPage()}
		/>
	{/if}
</div>
