<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import { fade, fly } from 'svelte/transition';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { filterTitle, formatTime } from '$lib/utils/data-filters';
	import { toast } from 'svelte-sonner';
	import TrailerPill from '$lib/components/TrailerPill.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { 
		useAddAnimeToList, 
		useUpdateListStatus, 
		useUpdateListProgress, 
		useDeleteListEntry 
	} from '$lib/hooks/useAnilist.svelte';
	import type { MediaListStatus } from '$lib/types/anilist';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);

	const title = $derived(animeData ? filterTitle(animeData.title || {}) : '');
	const japaneseTitle = $derived(
		animeData ? animeData.title?.native || animeData.title?.romaji || title : ''
	);
	const nextAiringEpisodeTime = $derived(
		animeData?.nextAiringEpisode ? formatTime(animeData.nextAiringEpisode.timeUntilAiring || 0) : ''
	);

	let isFavorite = $state(false);
	$effect(() => {
		if (animeData) {
			isFavorite = animeData.isFavourite || false;
		}
	});

	const copyLink = () => {
		navigator.clipboard.writeText(window.location.href);
		toast.success('Link copied to clipboard');
	};

	const shareAnime = async () => {
		if (navigator.share) {
			try {
				await navigator.share({
					title: `${title} - Zafkiel`,
					text: `Check out ${title} on Zafkiel!`,
					url: window.location.href,
				});
			} catch (err) {
				copyLink();
			}
		} else {
			copyLink();
		}
	};

	const addMutation = useAddAnimeToList();
	const updateStatusMutation = useUpdateListStatus();
	const updateProgressMutation = useUpdateListProgress();
	const deleteMutation = useDeleteListEntry();

	const statusOptions: MediaListStatus[] = [
		'CURRENT',
		'PLANNING',
		'COMPLETED',
		'DROPPED',
		'PAUSED',
		'REPEATING',
	];

	const getStatusLabel = (s: string) => {
		return s.charAt(0) + s.slice(1).toLowerCase().replace('_', ' ');
	};
</script>

{#if animeData}
	<div class="space-y-4" in:fade={{ duration: 300 }}>
		<!-- Header Content (Relocated from layout) -->
		<div class="flex flex-col gap-2">
			<div>
				<h4 class="text-sm font-light overflow-ellipsis text-muted-foreground">
					{japaneseTitle}
					{animeData.seasonYear ? ` • ${animeData.seasonYear}` : ''}
					{animeData.duration ? ` • ${animeData.duration} mins` : ''}
				</h4>
			</div>

			<div class="flex gap-3">
				{#if !animeData.mediaListEntry}
					<Button
						class="cursor-pointer rounded-md bg-accent text-accent-foreground hover:bg-accent/90"
						onclick={() => addMutation.mutate({ mediaId: animeId, status: 'PLANNING' })}
						disabled={addMutation.isPending}
					>
						<Icon icon="lucide:plus" class="-mx-1 size-6" />
						Add to List
					</Button>
				{:else}
					<div class="flex items-center gap-2">
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button 
									variant="outline" 
									class="flex gap-2 rounded-md bg-primary/10 text-primary border-primary/20 hover:bg-primary/20"
								>
									<Icon icon="lucide:list" class="size-4" />
									<span class="text-xs font-bold uppercase tracking-wider">
										{getStatusLabel(animeData.mediaListEntry.status || '')}
									</span>
									{#if animeData.mediaListEntry.progress}
										<span class="mx-1 h-3 w-[1px] bg-primary/20"></span>
										<span class="text-sm font-black italic">EP {animeData.mediaListEntry.progress}</span>
									{/if}
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="start" class="z-[100] bg-background/95 backdrop-blur-md">
								{#each statusOptions as status}
									<DropdownMenu.Item 
										onclick={() => updateStatusMutation.mutate({ 
											entryId: animeData.mediaListEntry!.id, 
											status 
										})}
										class="flex items-center justify-between gap-4 cursor-pointer"
									>
										<span class="text-xs font-semibold">{getStatusLabel(status)}</span>
										{#if animeData.mediaListEntry.status === status}
											<Icon icon="lucide:check" class="size-3 text-primary" />
										{/if}
									</DropdownMenu.Item>
								{/each}
							</DropdownMenu.Content>
						</DropdownMenu.Root>

						<Button
							class="cursor-pointer rounded-md bg-accent text-accent-foreground hover:bg-accent/90"
							onclick={() => {
								const newProgress = (animeData.mediaListEntry!.progress || 0) + 1;
								updateProgressMutation.mutate({ 
									entryId: animeData.mediaListEntry!.id, 
									progress: newProgress 
								});
							}}
							disabled={updateProgressMutation.isPending}
						>
							<Icon icon="lucide:chevron-up" class="-ml-1 size-4" />
							Update
						</Button>
						
						<Button
							class="cursor-pointer rounded-md border-destructive/50 text-destructive hover:bg-destructive/10"
							variant="outline"
							onclick={() => {
								if (confirm('Are you sure you want to remove this from your list?')) {
									deleteMutation.mutate(animeData.mediaListEntry!.id);
								}
							}}
							disabled={deleteMutation.isPending}
						>
							<Icon icon="lucide:trash-2" class="size-4" />
						</Button>
					</div>
				{/if}
				<Button
					class="cursor-pointer rounded-md hover:bg-accent/10 hover:text-accent"
					variant="outline"
					onclick={() => {
						isFavorite = !isFavorite;
					}}
				>
					{#if isFavorite}
						<Icon icon="lucide:heart" class="-mx-1 size-6 fill-current" />
					{:else}
						<Icon icon="lucide:heart" class="-mx-1 size-6" />
					{/if}
				</Button>
				<Button
					class="cursor-pointer rounded-md hover:bg-accent/10 hover:text-accent"
					variant="outline"
					onclick={shareAnime}
				>
					<Icon icon="lucide:share-2" class="-mx-1 size-6" />
				</Button>
			</div>

			<!-- Statistics Section -->
			<div class="flex flex-wrap gap-4 text-sm">
				{#if animeData.averageScore}
					<div class="flex items-center gap-2 rounded-md bg-muted/30 px-3 py-2">
						<Icon icon="lucide:star" class="size-4" />
						<span class="font-semibold">{animeData.averageScore}%</span>
						<span class="text-muted-foreground">Score</span>
					</div>
				{/if}
				{#if animeData.popularity}
					<div class="flex items-center gap-2 rounded-md bg-muted/30 px-3 py-2">
						<Icon icon="lucide:trending-up" class="size-4" />
						<span class="font-semibold">
							{animeData.popularity > 1000
								? `${(animeData.popularity / 1000).toFixed(1)}k`
								: animeData.popularity}
						</span>
						<span class="text-muted-foreground">Popularity</span>
					</div>
				{/if}
				{#if animeData.episodes}
					<div class="flex items-center gap-2 rounded-md bg-muted/30 px-3 py-2">
						<Icon icon="lucide:play-circle" class="size-4" />
						<span class="font-semibold">{animeData.episodes}</span>
						<span class="text-muted-foreground">Episodes</span>
					</div>
				{/if}
				{#if animeData.status}
					<div class="flex items-center gap-2 rounded-md bg-muted/30 px-3 py-2">
						<Icon icon="lucide:info" class="size-4" />
						<span class="font-semibold">
							{animeData.status.charAt(0) + animeData.status.slice(1).toLowerCase()}
						</span>
					</div>
				{/if}
			</div>

			<div class="flex flex-wrap gap-3">
				{#each animeData.genres || [] as genre}
					<span
						class="shrink-0 rounded-sm bg-primary/20 px-2 py-0.5 text-[11px] font-semibold tracking-[0.01em] whitespace-nowrap text-primary"
						in:fade={{ duration: 150 }}
					>
						{genre}
					</span>
				{/each}
			</div>

			<div class="grid grid-cols-1 gap-6 md:grid-cols-3">
				<!-- Characters Preview -->
				{#if animeData.characters?.edges && animeData.characters.edges.length > 0}
					{@const sortedChars = [...animeData.characters.edges].sort((a, b) => {
						const aHasImage = a.node?.image?.large && !a.node.image.large.includes('default.jpg');
						const bHasImage = b.node?.image?.large && !b.node.image.large.includes('default.jpg');
						return aHasImage === bHasImage ? 0 : aHasImage ? -1 : 1;
					})}
					<div class="flex flex-col gap-2">
						<h4 class="text-sm font-medium text-muted-foreground/80">Characters</h4>
						<div class="flex items-center -space-x-3">
							<Tooltip.Provider>
								{#each sortedChars.slice(0, 5) as char, i}
									{#if char.node}
										<Tooltip.Root delayDuration={200}>
											<Tooltip.Trigger>
												<div
													class="relative h-12 w-12 overflow-hidden rounded-full border-2 border-background shadow-sm"
													style="z-index: {10 - i}"
												>
													<img
														src={char.node.image?.large || char.node.image?.medium}
														alt={char.node.name?.full}
														class="h-full w-full object-cover"
													/>
												</div>
											</Tooltip.Trigger>
											<Tooltip.Content>
												<p class="text-xs font-medium">{char.node.name?.full}</p>
											</Tooltip.Content>
										</Tooltip.Root>
									{/if}
								{/each}
							</Tooltip.Provider>
							<a
								href="/anime/{animeId}/characters"
								class="relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-background bg-muted text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
								style="z-index: 4"
							>
								<Icon icon="lucide:plus" class="size-5" />
							</a>
						</div>
					</div>
				{/if}

				<!-- Staff Preview -->
				{#if animeData.staff?.edges && animeData.staff.edges.length > 0}
					{@const sortedStaff = [...animeData.staff.edges].sort((a, b) => {
						const aHasImage = a.node?.image?.large && !a.node.image.large.includes('default.jpg');
						const bHasImage = b.node?.image?.large && !b.node.image.large.includes('default.jpg');
						return aHasImage === bHasImage ? 0 : aHasImage ? -1 : 1;
					})}
					<div class="flex flex-col gap-2">
						<h4 class="text-sm font-medium text-muted-foreground/80">Staff</h4>
						<div class="mt-1.5 flex items-center -space-x-3">
							<Tooltip.Provider>
								{#each sortedStaff.slice(0, 5) as staff, i}
									{#if staff.node}
										<Tooltip.Root delayDuration={200}>
											<Tooltip.Trigger>
												<div
													class="relative h-12 w-12 overflow-hidden rounded-full border-2 border-background shadow-sm"
													style="z-index: {10 - i}"
												>
													<img
														src={staff.node.image?.large || staff.node.image?.medium}
														alt={staff.node.name?.full}
														class="h-full w-full object-cover"
													/>
												</div>
											</Tooltip.Trigger>
											<Tooltip.Content>
												<p class="text-xs font-medium">{staff.node.name?.full}</p>
											</Tooltip.Content>
										</Tooltip.Root>
									{/if}
								{/each}
							</Tooltip.Provider>
							<a
								href="/anime/{animeId}/staff"
								class="relative flex h-12 w-12 items-center justify-center rounded-full border-2 border-background bg-muted text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
								style="z-index: 4"
							>
								<Icon icon="lucide:plus" class="size-5" />
							</a>
						</div>
					</div>
				{/if}

				<!-- Trailer -->
				{#if animeData.trailer}
					<TrailerPill
						trailer={animeData.trailer}
						banner={animeData.bannerImage || animeData.coverImage?.extraLarge || ''}
					/>
				{/if}
			</div>

			<!-- Airing Schedule -->
			{#if animeData.nextAiringEpisode}
				<div
					class="rounded-lg border border-primary/30 bg-linear-to-r from-primary/20 to-primary/10 p-4"
					in:fade={{ duration: 300 }}
				>
					<div class="flex items-center gap-3">
						<Icon icon="lucide:clock" class="size-5 text-primary" />
						<div>
							<h3 class="font-semibold">Next Episode</h3>
							<p class="text-sm text-muted-foreground">
								Episode {animeData.nextAiringEpisode.episode} airing in {nextAiringEpisodeTime}
							</p>
						</div>
					</div>
				</div>
			{/if}
		</div>

		<Separator />

		<!-- Synopsis Section -->
		{#if animeData.description}
			<div class="space-y-3">
				<h2 class="text-xl font-semibold">Synopsis</h2>
				<div class="prose prose-sm max-w-none leading-relaxed text-muted-foreground">
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html animeData.description}
				</div>
			</div>
			<Separator />
		{/if}

		<!-- Information Grid -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Information</h2>
			<div class="rounded-lg border bg-card p-6">
				<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
					<div class="space-y-4">
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Format</span>
							<span class="font-semibold">{animeData.format || 'Unknown'}</span>
						</div>
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Season</span>
							<span class="font-semibold">
								{animeData.season && animeData.seasonYear
									? `${animeData.season.charAt(0) + animeData.season.slice(1).toLowerCase()} ${animeData.seasonYear}`
									: 'Unknown'}
							</span>
						</div>
						{#if animeData.duration}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Episode Duration</span>
								<span class="font-semibold">{animeData.duration} minutes</span>
							</div>
						{/if}
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Status</span>
							<span class="font-semibold capitalize">
								{animeData.status ? animeData.status.toLowerCase().replace('_', ' ') : 'Unknown'}
							</span>
						</div>
					</div>
					<div class="space-y-4">
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Type</span>
							<span class="font-semibold">{animeData.type || 'TV'}</span>
						</div>
						{#if animeData.studios?.nodes && animeData.studios.nodes.length > 0}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Studio</span>
								<span class="font-semibold">{animeData.studios.nodes[0].name}</span>
							</div>
						{/if}
						{#if animeData.source}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Source</span>
								<span class="font-semibold capitalize">
									{animeData.source.toLowerCase().replace('_', ' ')}
								</span>
							</div>
						{/if}
						{#if animeData.idMal}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">MAL ID</span>
								<a
									href="https://myanimelist.net/anime/{animeData.idMal}"
									target="_blank"
									class="flex items-center gap-1 font-semibold text-primary hover:underline"
								>
									{animeData.idMal}
									<Icon icon="lucide:external-link" class="size-3" />
								</a>
							</div>
						{/if}
						<div class="flex items-center justify-between py-2">
							<span class="font-medium text-muted-foreground">AniList ID</span>
							<a
								href="https://anilist.co/anime/{animeData.id}"
								target="_blank"
								class="flex items-center gap-1 font-semibold text-primary hover:underline"
							>
								{animeData.id}
								<Icon icon="lucide:external-link" class="size-3" />
							</a>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center gap-4 py-20 text-center">
		<Icon icon="solar:widget-5-bold-duotone" class="size-16 text-muted-foreground/40" />
		<div>
			<h3 class="text-lg font-semibold text-muted-foreground">Loading...</h3>
		</div>
	</div>
{/if}
