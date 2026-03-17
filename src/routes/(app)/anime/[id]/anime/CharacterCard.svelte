<script lang="ts">
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

	export interface Character {
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

	let { character }: { character: Character } = $props();
</script>

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
				class="mb-0.5 block truncate text-[10px] font-bold tracking-wider text-muted-foreground uppercase"
			>
				CHARACTER
			</span>
			<span
				class="mb-1.5 truncate text-sm leading-none font-semibold text-foreground transition-colors group-hover:text-primary"
			>
				{character.name.full}
			</span>
			<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
				<Icon icon="solar:star-bold-duotone" class="size-3.5 shrink-0 text-current" />
				<span class="truncate capitalize">{character.role?.toLowerCase()}</span>
			</span>
		</div>
	</a>

	<!-- Voice Actor (Preferred Language) -->
	{#if character.voiceActors && character.voiceActors.length > 0}
		<!-- Try to find VA in selected language, otherwise first one -->
		{@const va = character.voiceActors[0]}

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
					class="mb-0.5 block w-full truncate text-right text-[10px] font-bold tracking-wider text-muted-foreground uppercase"
				>
					VOICE ACTOR
				</span>
				<span
					class="mb-1.5 w-full truncate text-right text-sm leading-none font-semibold text-foreground transition-colors group-hover:text-primary"
				>
					{va.name.full}
				</span>
				<span class="flex w-full items-center justify-end gap-1.5 text-xs text-muted-foreground">
					<Icon icon="solar:microphone-3-bold-duotone" class="size-3.5 shrink-0 text-current" />
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
