<script lang="ts">
	import { page } from '$app/state';
	import { useCharacterById } from '$lib/hooks/useAnilist.svelte';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';
	import Icon from '@iconify/svelte';

	const characterId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const characterQuery = $derived(useCharacterById(characterId));
	const character = $derived(characterQuery.data?.data);
</script>

{#if character}
	<div class="grid grid-cols-1 gap-8 xl:grid-cols-4">
		<!-- Left: Description (takes 3/4) -->
		<div class="xl:col-span-3">
			{#if character.description}
				<div class="custom-scrollbar markdown-wrapper text-sm leading-relaxed text-foreground/90">
					<MarkdownRenderer body={character.description} />
				</div>
			{/if}
		</div>

		<!-- Right: Additional Information Grid (takes 1/4) -->
		<div class="space-y-6 xl:col-span-1">
			<div class="rounded-lg border bg-card p-5">
				<h3 class="mb-4 text-sm font-semibold">Information</h3>
				<div class="space-y-3 text-sm">
					{#if character.dateOfBirth?.year || character.dateOfBirth?.month || character.dateOfBirth?.day}
						<div>
							<span class="mb-0.5 block text-xs text-muted-foreground">Birthday</span>
							<span class="font-medium">
								{character.dateOfBirth.day || '?'}/{character.dateOfBirth.month ||
									'?'}{#if character.dateOfBirth.year}/{character.dateOfBirth.year}{/if}
							</span>
						</div>
					{/if}
					{#if character.bloodType}
						<div>
							<span class="mb-0.5 block text-xs text-muted-foreground">Blood Type</span>
							<span class="font-medium">{character.bloodType}</span>
						</div>
					{/if}
					<div>
						<span class="mb-0.5 block text-xs text-muted-foreground">AniList Profile</span>
						<a
							href={`https://anilist.co/character/${character.id}`}
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-1 font-medium text-emerald-500 transition-colors hover:text-emerald-400 hover:underline"
						>
							View on AniList
							<Icon icon="lucide:external-link" class="size-3" />
						</a>
					</div>
				</div>
			</div>

			<!-- Alternative Names -->
			{#if character.name?.alternative && character.name.alternative.length > 0}
				<div class="rounded-lg border bg-card p-5">
					<h3 class="mb-4 text-sm font-semibold">Alternative Names</h3>
					<div class="flex flex-col gap-2">
						{#each character.name.alternative.filter(Boolean) as name}
							<div
								class="border-b border-border/50 pb-2 text-sm font-medium last:border-0 last:pb-0"
							>
								{name}
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}
