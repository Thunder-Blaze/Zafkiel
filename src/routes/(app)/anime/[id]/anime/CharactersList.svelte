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
			<article
				class="flex w-full items-center justify-between rounded-xl border border-border/60 bg-card/40 p-1.5 shadow-sm transition-all hover:bg-card/60 hover:shadow-md"
			>
				<!-- Character -->
				<a
					href="/character/{character.id}"
					class="group flex flex-1 items-center gap-3 overflow-hidden rounded-lg bg-muted/40 pr-3 transition-colors hover:bg-muted/60"
				>
					<CachedImage
						src={character.image.large || character.image.medium}
						alt={character.name.full || 'Character'}
						class="h-20 w-[60px] shrink-0 object-cover"
					/>
					<div class="flex min-w-0 flex-col overflow-hidden py-1">
						<span
							class="mb-0.5 block truncate text-[10px] font-bold tracking-wider text-muted-foreground"
						>
							Character
						</span>
						<span
							class="mb-1.5 truncate text-sm font-semibold leading-none text-foreground transition-colors group-hover:text-primary"
						>
							{character.name.full}
						</span>
						<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
							<Icon icon="solar:star-bold-duotone" class="size-3.5 shrink-0 text-yellow-500" />
							<span class="truncate">{character.role}</span>
						</span>
					</div>
				</a>

				<!-- Voice Actor (Japanese preferred) -->
				{#if character.voiceActors && character.voiceActors.length > 0}
					<!-- Try to find Japanese VA, otherwise first one -->
					{@const va =
						character.voiceActors.find((v) => v.languageV2 === 'Japanese' || v.languageV2 === 'JAPANESE') ||
						character.voiceActors[0]}
					
					<!-- Middle Icon -->
					<div class="flex shrink-0 items-center justify-center px-1.5 md:px-2">
						<Icon icon="solar:link-bold-duotone" class="size-4 -rotate-45 text-muted-foreground/40" />
					</div>

					<a
						href="/staff/{va.id}"
						class="group flex flex-1 items-center justify-end gap-3 overflow-hidden rounded-lg bg-muted/40 pl-3 text-right transition-colors hover:bg-muted/60"
					>
						<div class="flex min-w-0 flex-col items-end overflow-hidden py-1">
							<span
								class="mb-0.5 block w-full truncate text-right text-[10px] font-bold tracking-wider text-muted-foreground"
							>
								Voice Actor
							</span>
							<span
								class="mb-1.5 w-full truncate text-right text-sm font-semibold leading-none text-foreground transition-colors group-hover:text-primary"
							>
								{va.name.full}
							</span>
							<span class="flex w-full items-center justify-end gap-1.5 text-xs text-muted-foreground">
								<Icon
									icon="solar:microphone-3-bold-duotone"
									class="size-3.5 shrink-0 text-primary/70"
								/>
								<span class="truncate capitalize">{va.languageV2.toLowerCase()}</span>
							</span>
						</div>
						<CachedImage
							src={va.image.large || va.image.medium}
							alt={va.name.full || 'Voice Actor'}
							class="h-20 w-[60px] shrink-0 object-cover"
						/>
					</a>
				{/if}
			</article>
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
