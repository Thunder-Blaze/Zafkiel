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
				<Select.Trigger class="w-[140px] rounded-lg border-border/40 bg-card/60 backdrop-blur-md shadow-sm transition-colors hover:border-primary/50 focus:ring-1 focus:ring-primary gap-2 px-4 h-9">
					<div class="flex items-center gap-2">
						<Icon icon="solar:global-bold-duotone" class="size-4 text-primary" />
						<span class="font-medium text-foreground text-sm">{selectedLanguage}</span>
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
	<div class="grid grid-cols-2 gap-x-5 gap-y-12 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 pb-12">
		{#each character.media.edges as edge}
			{#if edge.node}
				{@const roleObj = getVoiceActor(edge, selectedLanguage)}
				{@const va = roleObj?.voiceActor}
				{@const animeUrl = `/${edge.node.type?.toLowerCase() || 'anime'}/${edge.node.id}`}
				
				<div class="group flex flex-col relative w-full">
					<!-- Image Area -->
					<div class="relative w-full aspect-[2/3] rounded-xl shadow-md transition-all duration-300 group-hover:shadow-xl group-hover:-translate-y-1">
						
						<!-- Anime Cover Link -->
						<a href={animeUrl} class="absolute inset-0 z-10 block overflow-hidden rounded-xl bg-muted outline-none ring-primary transition-all focus-visible:ring-2">
							<CachedImage
								src={edge.node.coverImage?.large}
								alt={getTitle(edge.node.title)}
								class="h-full w-full object-cover"
							/>
							<div class="absolute inset-x-0 bottom-0 h-1/2 bg-gradient-to-t from-black/80 via-black/20 to-transparent opacity-60"></div>

							{#if edge.characterRole}
								<div class="absolute top-2 right-2 rounded-full bg-background/95 px-2.5 py-0.5 text-[10px] sm:text-xs font-bold tracking-wider text-foreground shadow-sm backdrop-blur-md">
									{edge.characterRole}
								</div>
							{/if}
						</a>
						
						<!-- Voice Actor Avatar Overlapping (Z-20) -->
						{#if va?.image?.large}
							<a href="/staff/{va.id}" class="absolute bottom-2 right-2 z-20 block w-[35%] max-w-[80px] min-w-[48px] aspect-[2/3] overflow-hidden rounded-md border-2 border-background/80 bg-muted shadow-sm outline-none transition-all duration-300 hover:-translate-y-1 hover:shadow-md">
								<CachedImage
									src={va.image.large}
									alt={va.name?.userPreferred || ''}
									class="h-full w-full object-cover"
								/>
							</a>
						{/if}
					</div>

					<!-- Text Content -->
					<div class="flex flex-col flex-1 px-1 mt-2">
						<!-- Anime Status Dot and Title -->
						<div class="flex items-start gap-2 w-full pt-1 pr-1">
							<span class="mt-1.5 size-2 sm:size-2.5 shrink-0 rounded-full {edge.node.status === 'RELEASING' ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)]' : 'bg-primary shadow-[0_0_8px_var(--color-primary)]'}"></span>
							<a href={animeUrl} class="line-clamp-2 text-sm sm:text-base font-bold leading-tight text-foreground/90 transition-colors hover:text-primary outline-none focus-visible:text-primary">
								{getTitle(edge.node.title)}
							</a>
						</div>

						<!-- Voice Actor Info Below Title -->
						{#if va}
							<a href="/staff/{va.id}" class="ml-4 sm:ml-[18px] mt-1.5 flex flex-col group/vatext outline-none">
								<span class="text-xs sm:text-sm font-semibold text-muted-foreground transition-colors group-hover/vatext:text-primary group-focus-visible/vatext:text-primary truncate">{va.name?.userPreferred || va.name?.full}</span>
								{#if roleObj?.roleNotes}
									<span class="text-[10px] sm:text-[11px] font-medium text-muted-foreground/60 transition-colors group-hover/vatext:text-muted-foreground mt-0.5 truncate">{roleObj.roleNotes}</span>
								{/if}
							</a>
						{/if}
					</div>
				</div>
			{/if}
		{/each}
	</div>
{:else}
	<div class="flex min-h-[300px] flex-col items-center justify-center rounded-xl border border-dashed border-border bg-card/50 text-center shadow-sm backdrop-blur-sm">
		<div class="flex h-20 w-20 items-center justify-center rounded-full bg-muted/50 mb-4">
			<Icon icon="solar:gallery-wide-bold-duotone" class="h-10 w-10 text-muted-foreground/40" />
		</div>
		<h3 class="text-lg font-semibold text-foreground mb-1">No Media Found</h3>
		<p class="text-sm text-muted-foreground">There are no documented media appearances for this character.</p>
	</div>
{/if}
