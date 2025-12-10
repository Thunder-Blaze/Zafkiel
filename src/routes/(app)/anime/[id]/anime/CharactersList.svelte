<script lang="ts">
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

	interface Character {
		id: number;
		name: {
			first?: string;
			last?: string;
			full?: string;
			native?: string;
			userPreferred?: string;
		};
		image: {
			large: string;
			medium: string;
		};
		role: string;
		voiceActors?: {
			id: number;
			name: {
				first?: string;
				last?: string;
				full?: string;
				native?: string;
			};
			image: {
				large: string;
				medium: string;
			};
			languageV2: string;
		}[];
	}

	export let characters: Character[] = [];
	export let isLoading: boolean = false;
</script>

{#if isLoading}
	<div class="flex items-center justify-center p-8">
		<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
	</div>
{:else if characters.length > 0}
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
		{#each characters as character}
			<div
				class="flex justify-between rounded-lg border border-border bg-card p-2 transition-shadow hover:shadow-md"
			>
				<!-- Character -->
				<div class="flex gap-3">
					<a href="/character/{character.id}" class="shrink-0">
						<CachedImage
							src={character.image.medium}
							alt={character.name.full || 'Character'}
							class="h-16 w-12 rounded object-cover"
						/>
					</a>
					<div class="flex flex-col justify-center">
						<a
							href="/character/{character.id}"
							class="line-clamp-1 text-sm font-semibold hover:text-primary"
						>
							{character.name.full}
						</a>
						<span class="text-xs text-muted-foreground">{character.role}</span>
					</div>
				</div>

				<!-- Voice Actor (Japanese preferred) -->
				{#if character.voiceActors && character.voiceActors.length > 0}
					<!-- Try to find Japanese VA, otherwise first one -->
					{@const va =
						character.voiceActors.find((v) => v.languageV2 === 'Japanese') ||
						character.voiceActors[0]}
					<div class="flex gap-3 text-right">
						<div class="flex flex-col items-end justify-center">
							<a
								href="/staff/{va.id}"
								class="line-clamp-1 text-sm font-semibold hover:text-primary"
							>
								{va.name.full}
							</a>
							<span class="text-xs text-muted-foreground">{va.languageV2}</span>
						</div>
						<a href="/staff/{va.id}" class="shrink-0">
							<CachedImage
								src={va.image.medium}
								alt={va.name.full || 'Voice Actor'}
								class="h-16 w-12 rounded object-cover"
							/>
						</a>
					</div>
				{/if}
			</div>
		{/each}
	</div>
{:else}
	<div
		class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
	>
		<Icon icon="lucide:users" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
		<p class="text-muted-foreground">No characters found.</p>
	</div>
{/if}
