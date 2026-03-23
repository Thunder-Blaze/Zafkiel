<script lang="ts">
	import { page } from '$app/state';
	import { useCharacterById } from '$lib/hooks/useAnilist.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import * as Select from '$lib/components/ui/select/index.js';

	const characterId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const characterQuery = $derived(useCharacterById(characterId));
	const character = $derived(characterQuery.data?.data);

	let selectedLanguage = $state('Japanese');

	function getTitle(title: any): string {
		if (!title) return 'Unknown Title';
		return title.userPreferred || title.english || title.romaji || title.native || 'Unknown Title';
	}

	function getVoiceActor(edge: any, lang: string) {
		if (!edge.voiceActorRoles || edge.voiceActorRoles.length === 0) return null;

		const filtered = edge.voiceActorRoles.filter((role: any) => {
			const vaLang = role.voiceActor?.languageV2 || role.voiceActor?.language;
			return vaLang === lang || (lang === 'Japanese' && !vaLang);
		});

		return filtered.length > 0 ? filtered[0] : edge.voiceActorRoles[0];
	}
</script>

{#if character?.media?.edges && character.media.edges.length > 0}
	<!-- Header Bar -->
	<div class="mb-8 flex items-center justify-between">
		<h3 class="text-xl font-bold tracking-tight text-foreground">Media Appearances</h3>

		<div class="flex items-center">
			<Select.Root type="single" bind:value={selectedLanguage}>
				<Select.Trigger
					class="h-9 w-[140px] gap-2 rounded-lg border-border/40 bg-card/60 px-4 shadow-sm backdrop-blur-md transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary"
				>
					<div class="flex items-center gap-2">
						<Icon icon="solar:global-bold-duotone" class="size-4 text-primary" />
						<span class="text-sm font-medium text-foreground">{selectedLanguage}</span>
					</div>
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="Japanese" label="Japanese">Japanese</Select.Item>
					<Select.Item value="English" label="English">English</Select.Item>
					<Select.Item value="Korean" label="Korean">Korean</Select.Item>
					<Select.Item value="Spanish" label="Spanish">Spanish</Select.Item>
					<Select.Item value="French" label="French">French</Select.Item>
					<Select.Item value="Portuguese" label="Portuguese">Portuguese</Select.Item>
					<Select.Item value="Italian" label="Italian">Italian</Select.Item>
					<Select.Item value="German" label="German">German</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
	</div>

	<!-- Media Grid -->
	<div
		class="grid grid-cols-2 gap-x-5 gap-y-12 pb-12 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
	>
		{#each character.media.edges as edge}
			{#if edge.node}
				{@const roleObj = getVoiceActor(edge, selectedLanguage)}
				{@const va = roleObj?.voiceActor}
				{@const animeUrl = `/${edge.node.type?.toLowerCase() || 'anime'}/${edge.node.id}`}

				<div class="group relative flex w-full flex-col">
					<!-- Image Area -->
					<div
						class="relative aspect-[2/3] w-full rounded-xl shadow-md transition-all duration-300 group-hover:-translate-y-1 group-hover:shadow-xl"
					>
						<!-- Anime Cover Link -->
						<a
							href={animeUrl}
							class="absolute inset-0 z-10 block overflow-hidden rounded-xl bg-muted ring-primary transition-all outline-none focus-visible:ring-2"
						>
							<CachedImage
								src={edge.node.coverImage?.large}
								alt={getTitle(edge.node.title)}
								class="h-full w-full object-cover"
							/>
							<div
								class="absolute inset-x-0 bottom-0 h-1/2 bg-gradient-to-t from-black/80 via-black/20 to-transparent opacity-60"
							></div>

							{#if edge.characterRole}
								<div
									class="absolute top-2 right-2 rounded-full bg-background/95 px-2.5 py-0.5 text-[10px] font-bold tracking-wider text-foreground shadow-sm backdrop-blur-md sm:text-xs"
								>
									{edge.characterRole}
								</div>
							{/if}
						</a>

						<!-- Voice Actor Avatar Overlapping (Z-20) -->
						{#if va?.image?.large}
							<a
								href="/staff/{va.id}"
								class="absolute right-2 bottom-2 z-20 block aspect-[2/3] w-[35%] max-w-[80px] min-w-[48px] overflow-hidden rounded-md border-2 border-background/80 bg-muted shadow-sm transition-all duration-300 outline-none hover:-translate-y-1 hover:shadow-md"
							>
								<CachedImage
									src={va.image.large}
									alt={va.name?.userPreferred || ''}
									class="h-full w-full object-cover"
								/>
							</a>
						{/if}
					</div>

					<!-- Text Content -->
					<div class="mt-2 flex flex-1 flex-col px-1">
						<!-- Anime Status Dot and Title -->
						<div class="flex w-full items-start gap-2 pt-1 pr-1">
							<span
								class="mt-1.5 size-2 shrink-0 rounded-full sm:size-2.5 {edge.node.status ===
								'RELEASING'
									? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)]'
									: 'bg-primary shadow-[0_0_8px_var(--color-primary)]'}"
							></span>
							<a
								href={animeUrl}
								class="line-clamp-2 text-sm leading-tight font-bold text-foreground/90 transition-colors outline-none hover:text-primary focus-visible:text-primary sm:text-base"
							>
								{getTitle(edge.node.title)}
							</a>
						</div>

						<!-- Voice Actor Info Below Title -->
						{#if va}
							<a
								href="/staff/{va.id}"
								class="group/vatext mt-1.5 ml-4 flex flex-col outline-none sm:ml-[18px]"
							>
								<span
									class="truncate text-xs font-semibold text-muted-foreground transition-colors group-hover/vatext:text-primary group-focus-visible/vatext:text-primary sm:text-sm"
									>{va.name?.userPreferred || va.name?.full}</span
								>
								{#if roleObj?.roleNotes}
									<span
										class="mt-0.5 truncate text-[10px] font-medium text-muted-foreground/60 transition-colors group-hover/vatext:text-muted-foreground sm:text-[11px]"
										>{roleObj.roleNotes}</span
									>
								{/if}
							</a>
						{/if}
					</div>
				</div>
			{/if}
		{/each}
	</div>
{:else}
	<div
		class="flex min-h-[300px] flex-col items-center justify-center rounded-xl border border-dashed border-border bg-card/50 text-center shadow-sm backdrop-blur-sm"
	>
		<div class="mb-4 flex h-20 w-20 items-center justify-center rounded-full bg-muted/50">
			<Icon icon="solar:gallery-wide-bold-duotone" class="h-10 w-10 text-muted-foreground/40" />
		</div>
		<h3 class="mb-1 text-lg font-semibold text-foreground">No Media Found</h3>
		<p class="text-sm text-muted-foreground">
			There are no documented media appearances for this character.
		</p>
	</div>
{/if}
