<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById, useMediaRecommendations } from '$lib/hooks/useAnilist.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import { filterCoverImage, filterTitle, formatTime } from '$lib/utils/data-filters';
	import { fade, fly } from 'svelte/transition';
	import Button from '$lib/components/ui/button/button.svelte';
	import Icon from '@iconify/svelte';
	import TrailerPill from '$lib/components/TrailerPill.svelte';
	import { glow } from '$lib/stores/zafkielStore';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { toast } from 'svelte-sonner';
	import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/components/ui/tabs';
	import CharactersList from './anime/CharactersList.svelte';
	import ReviewsList from './anime/ReviewsList.svelte';
	import UserActivities from './anime/UserActivities.svelte';

	import TorrentsList from './anime/TorrentsList.svelte';
	import RecommendationCard from '$lib/components/RecommendationCard.svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
	const isLoading = $derived(animeQuery.isLoading);
	const error = $derived(animeQuery.error);

	let isFavorite = $state(false);

	$effect(() => {
		console.log('Anime Page Debug:', {
			params: page.params,
			animeId,
			isLoading,
			error,
			hasData: !!animeData,
			mounted,
		});
		if (animeData) {
			isFavorite = animeData.isFavourite || false;
		}
	});

	const title = $derived(animeData ? filterTitle(animeData.title || {}) : '');
	const japaneseTitle = $derived(
		animeData ? animeData.title?.native || animeData.title?.romaji || title : ''
	);
	const nextAiringEpisodeTime = $derived(
		animeData?.nextAiringEpisode ? formatTime(animeData.nextAiringEpisode.timeUntilAiring || 0) : ''
	);
	const coverImage = $derived(animeData ? filterCoverImage(animeData.coverImage) : '');
	const bannerImage = $derived(animeData?.bannerImage || '');

	let mounted = $state(false);

	$effect(() => {
		mounted = true;
	});

	// Additional functionality
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
</script>

{#if isLoading}
	<div class="container mx-auto max-w-7xl px-4 py-8">
		<PageLoader type="anime" />
	</div>
{:else if error || (animeQuery.data && !animeQuery.data.success)}
	<div class="container mx-auto max-w-7xl px-4 py-8">
		<div class="rounded-lg border border-destructive bg-destructive/10 p-6 text-center">
			<h2 class="text-2xl font-bold text-destructive">Error Loading Anime</h2>
			<p class="text-muted-foreground">
				{error?.message || animeQuery.data?.error || 'Failed to load anime details'}
			</p>
			<Button variant="outline" class="mt-4" onclick={() => animeQuery.refetch()}>Try Again</Button>
		</div>
	</div>
{:else if !animeData}
	<div class="container mx-auto max-w-7xl px-4 py-8">
		<div class="rounded-lg border border-dashed p-6 text-center">
			<h2 class="text-2xl font-bold">No Data Found</h2>
			<p class="text-muted-foreground">Could not load anime details. ID: {animeId}</p>
			<Button variant="outline" class="mt-4" onclick={() => history.back()}>Go Back</Button>
		</div>
	</div>
{:else if animeData && mounted}
	<div class="flex w-full flex-col items-center">
		<div class="relative flex h-[24vw] w-full flex-col">
			<!-- Background banner image -->
			<img
				src={bannerImage || coverImage}
				alt={title}
				class="absolute inset-0 z-1 h-full w-full object-cover"
				in:fly={{ y: -100, duration: 300 }}
			/>

			<!-- Blur effect layer -->
			{#if $glow}
				<img
					src={bannerImage || coverImage}
					alt="Blurred background"
					class="absolute inset-0 h-full w-full object-cover"
					style="filter: blur(30px); -webkit-filter: blur(30px);"
					in:fly={{ y: -100, duration: 300 }}
				/>
			{/if}

			<!-- Cover image positioned over everything -->
			<div class="absolute top-0 left-0 z-2 flex h-full w-full justify-center">
				<div class="container flex flex-col justify-end">
					<div class="relative top-70 h-100 w-72">
						<img
							src={coverImage}
							alt={title}
							class="h-full w-full rounded-lg object-cover shadow-lg"
							in:fly={{ y: 100, duration: 300 }}
						/>
					</div>
				</div>
			</div>
		</div>
		<div class="container flex flex-col gap-3 pt-4 pl-78">
			<div>
				<h1 class="text-2xl font-bold overflow-ellipsis">{title}</h1>
				<h4 class="text-sm font-light overflow-ellipsis text-muted-foreground">
					{japaneseTitle}
					{animeData.seasonYear ? ` • ${animeData.seasonYear}` : ''}
					{animeData.duration ? ` • ${animeData.duration} mins` : ''}
				</h4>
			</div>
			<div class="flex gap-3">
				<Button
					class="cursor-pointer rounded-md bg-accent text-accent-foreground hover:bg-accent/90"
				>
					<Icon icon="lucide:plus" class="-mx-1 size-6" />
					Add to List
				</Button>
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
				{#if animeData.duration}
					<div class="flex items-center gap-2 rounded-md bg-muted/30 px-3 py-2">
						<Icon icon="lucide:clock" class="size-4" />
						<span class="font-semibold">{animeData.duration}m</span>
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
			<div>
				{#if animeData.trailer}
					<TrailerPill trailer={animeData.trailer} banner={bannerImage || coverImage || ''} />
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

			<Separator class="my-6" />

			<!-- Synopsis Section -->
			{#if animeData.description}
				<div class="space-y-3" in:fade={{ duration: 300 }}>
					<h2 class="text-xl font-semibold">Synopsis</h2>
					<div class="prose prose-sm max-w-none leading-relaxed text-muted-foreground">
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						{@html animeData.description}
					</div>
				</div>
				<Separator class="my-6" />
			{/if}

			<!-- Information Grid -->
			<div class="space-y-4" in:fade={{ duration: 300 }}>
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

			<Separator class="my-6" />

			<!-- Tabs Section -->
			<div class="space-y-6" in:fade={{ duration: 300 }}>
				<Tabs value="characters" class="w-full">
					<TabsList class="grid w-full grid-cols-7">
						<TabsTrigger value="characters" class="flex items-center gap-2">Characters</TabsTrigger>
						<TabsTrigger value="torrents" class="flex items-center gap-2">
							<Icon icon="solar:download-bold-duotone" class="size-4" />
							Torrents
						</TabsTrigger>
						<TabsTrigger value="staff" class="flex items-center gap-2">Staff</TabsTrigger>
						<TabsTrigger value="reviews">Reviews</TabsTrigger>
						<TabsTrigger value="stats">Stats</TabsTrigger>
						<TabsTrigger value="related">Related</TabsTrigger>
						<TabsTrigger value="recommendations">Recs</TabsTrigger>
					</TabsList>

					<TabsContent value="characters" class="mt-6">
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
							isLoading={false}
						/>
					</TabsContent>

					<TabsContent value="staff" class="mt-6">
						<div class="space-y-4">
							{#if animeData.staff?.edges && animeData.staff.edges.length > 0}
								<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
									{#each animeData.staff.edges as staffEdge}
										{#if staffEdge.node}
											<div
												class="rounded-lg border border-border bg-card p-4 transition-shadow hover:shadow-md"
											>
												<div class="flex gap-3">
													<img
														src={staffEdge.node.image?.large || '/api/placeholder/80/120'}
														alt={staffEdge.node.name?.full || 'Staff'}
														class="h-16 w-12 rounded object-cover"
														loading="lazy"
													/>
													<div class="min-w-0 flex-1">
														<h4 class="truncate text-sm font-semibold">
															{staffEdge.node.name?.full || 'Unknown Staff'}
														</h4>
														{#if staffEdge.node.name?.native}
															<p class="truncate text-xs text-muted-foreground">
																{staffEdge.node.name.native}
															</p>
														{/if}
														<p class="mt-1 text-xs font-medium text-primary">
															{staffEdge.role || 'Unknown Role'}
														</p>
													</div>
												</div>
											</div>
										{/if}
									{/each}
								</div>
							{:else}
								<div
									class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
								>
									<Icon icon="lucide:users" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
									<p class="text-muted-foreground">No staff information available.</p>
								</div>
							{/if}
						</div>
					</TabsContent>

					<TabsContent value="torrents" class="mt-6">
						<TorrentsList anime={animeData} />
					</TabsContent>

					<TabsContent value="reviews" class="mt-6">
						<ReviewsList mediaId={animeId} />
					</TabsContent>

					<TabsContent value="stats" class="mt-6">
						<div class="space-y-6">
							<!-- Score Distribution -->
							<div class="rounded-lg border border-border bg-card p-6">
								<h3 class="mb-4 text-lg font-semibold">Score Distribution</h3>
								<div class="space-y-3">
									{#each [10, 9, 8, 7, 6, 5, 4, 3, 2, 1] as score}
										<div class="flex items-center gap-3">
											<span class="w-6 text-sm font-medium">{score}</span>
											<div class="h-2 flex-1 rounded-full bg-muted">
												<div
													class="h-full rounded-full bg-primary transition-all duration-300"
													style="width: {Math.random() * 100}%"
												></div>
											</div>
											<span class="w-12 text-right text-xs text-muted-foreground">
												{Math.floor(Math.random() * 1000)}
											</span>
										</div>
									{/each}
								</div>
							</div>

							<!-- Status Distribution -->
							<div class="rounded-lg border border-border bg-card p-6">
								<h3 class="mb-4 text-lg font-semibold">Status Distribution</h3>
								<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
									{#each [{ status: 'Completed', count: Math.floor(Math.random() * 10000), color: 'bg-green-500' }, { status: 'Watching', count: Math.floor(Math.random() * 5000), color: 'bg-blue-500' }, { status: 'Planning', count: Math.floor(Math.random() * 3000), color: 'bg-yellow-500' }, { status: 'Dropped', count: Math.floor(Math.random() * 1000), color: 'bg-red-500' }, { status: 'Paused', count: Math.floor(Math.random() * 500), color: 'bg-orange-500' }] as stat}
										<div class="text-center">
											<div class={`h-3 w-3 ${stat.color} mx-auto mb-1 rounded-full`}></div>
											<div class="text-lg font-semibold">
												{stat.count.toLocaleString()}
											</div>
											<div class="text-xs text-muted-foreground">
												{stat.status}
											</div>
										</div>
									{/each}
								</div>
							</div>

							<!-- Rankings -->
							{#if animeData.averageScore || animeData.popularity}
								<div class="rounded-lg border border-border bg-card p-6">
									<h3 class="mb-4 text-lg font-semibold">Rankings</h3>
									<div class="space-y-3">
										{#if animeData.averageScore}
											<div class="flex items-center justify-between rounded-lg bg-muted/30 p-3">
												<span class="text-sm font-medium">Highest Rated All Time</span>
												<span class="font-semibold text-primary"
													>#{Math.floor(Math.random() * 500) + 1}</span
												>
											</div>
										{/if}
										{#if animeData.popularity}
											<div class="flex items-center justify-between rounded-lg bg-muted/30 p-3">
												<span class="text-sm font-medium">Most Popular All Time</span>
												<span class="font-semibold text-primary"
													>#{Math.floor(Math.random() * 1000) + 1}</span
												>
											</div>
										{/if}
									</div>
								</div>
							{/if}
						</div>
					</TabsContent>

					<TabsContent value="related" class="mt-6">
						<div class="space-y-6">
							{#if animeData.relations?.edges && animeData.relations.edges.length > 0}
								<div class="space-y-4">
									{#each animeData.relations.edges as relation}
										{#if relation.node}
											<div
												class="rounded-lg border border-border bg-card p-4 transition-shadow hover:shadow-md"
											>
												<div class="flex gap-4">
													<img
														src={relation.node.coverImage?.medium || '/api/placeholder/80/120'}
														alt={relation.node.title?.userPreferred || 'Related Media'}
														class="h-20 w-16 rounded object-cover"
														loading="lazy"
													/>
													<div class="min-w-0 flex-1">
														<div class="mb-1 flex items-center gap-2">
															<span
																class="rounded-full bg-primary/20 px-2 py-1 text-xs font-medium text-primary"
															>
																{relation.relationType?.replace('_', ' ') || 'Related'}
															</span>
															<span class="text-xs text-muted-foreground">
																{relation.node.type} • {relation.node.format}
															</span>
														</div>
														<h4 class="mb-1 line-clamp-2 text-sm font-semibold">
															{relation.node.title?.userPreferred || 'Unknown Title'}
														</h4>
														<p class="text-xs text-muted-foreground">
															{relation.node.status} •
															{#if relation.node.episodes}
																{relation.node.episodes} episodes
															{:else if relation.node.chapters}
																{relation.node.chapters} chapters
															{:else}
																Unknown length
															{/if}
														</p>
													</div>
												</div>
											</div>
										{/if}
									{/each}
								</div>
							{:else}
								<div
									class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
								>
									<Icon
										icon="lucide:list-plus"
										class="mx-auto mb-3 size-12 text-muted-foreground/50"
									/>
									<p class="text-muted-foreground">No related anime found.</p>
								</div>
							{/if}
						</div>
					</TabsContent>

					<TabsContent value="recommendations" class="mt-6">
						{@const recsQuery = useMediaRecommendations(animeId, 1, 20)}
						{@const recs = recsQuery.data?.data?.data ?? []}
						{#if recsQuery.isLoading}
							<div class="flex items-center justify-center p-8">
								<Icon
									icon="solar:refresh-circle-line-duotone"
									class="h-8 w-8 animate-spin text-primary"
								/>
							</div>
						{:else if recs.length > 0}
							<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
								{#each recs as rec}
									<RecommendationCard recommendation={rec} />
								{/each}
							</div>
						{:else}
							<div
								class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-8 text-center"
							>
								<Icon icon="solar:like-bold-duotone" class="h-12 w-12 text-muted-foreground" />
								<h3 class="text-lg font-semibold">No Recommendations</h3>
								<p class="text-sm text-muted-foreground">No recommendations available yet.</p>
							</div>
						{/if}
					</TabsContent>
				</Tabs>
			</div>
		</div>
	</div>
{/if}
