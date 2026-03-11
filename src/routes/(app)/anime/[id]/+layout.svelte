<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import { filterCoverImage, filterTitle, formatTime } from '$lib/utils/data-filters';
	import { fade, fly } from 'svelte/transition';
	import Button from '$lib/components/ui/button/button.svelte';
	import Icon from '@iconify/svelte';
	import TrailerPill from '$lib/components/TrailerPill.svelte';
	import { glow } from '$lib/stores/zafkielStore';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { toast } from 'svelte-sonner';
	import PageLoader from '$lib/components/PageLoader.svelte';

	let { children } = $props();

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
	const isLoading = $derived(animeQuery.isLoading);
	const error = $derived(animeQuery.error);

	let isFavorite = $state(false);

	$effect(() => {
		console.log('Anime Layout Debug:', {
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

	const activeTabClass =
		'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-background text-foreground shadow-sm h-full';
	const inactiveTabClass =
		'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-background/50 text-muted-foreground hover:text-foreground h-full';
</script>

{#if isLoading}
	<div class="w-full px-4 py-8 md:px-8 lg:px-10">
		<PageLoader type="anime" />
	</div>
{:else if error || (animeQuery.data && !animeQuery.data.success)}
	<div class="w-full px-4 py-8 md:px-8 lg:px-10">
		<div class="rounded-lg border border-destructive bg-destructive/10 p-6 text-center">
			<h2 class="text-2xl font-bold text-destructive">Error Loading Anime</h2>
			<p class="text-muted-foreground">
				{error?.message || animeQuery.data?.error || 'Failed to load anime details'}
			</p>
			<Button variant="outline" class="mt-4" onclick={() => animeQuery.refetch()}>Try Again</Button>
		</div>
	</div>
{:else if !animeData}
	<div class="w-full px-4 py-8 md:px-8 lg:px-10">
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
				<div class="flex w-full flex-col justify-end px-4 md:px-8 lg:px-10">
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
		<div class="w-full px-4 md:px-8 lg:px-10">
			<div class="flex w-full flex-col gap-6 sm:flex-row">
				<!-- Spacer matching the cover image width to map the absolute overlap -->
				<div class="hidden w-72 shrink-0 sm:block"></div>

				<div class="flex min-w-0 flex-1 flex-col gap-3 pt-4">
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
											{animeData.status
												? animeData.status.toLowerCase().replace('_', ' ')
												: 'Unknown'}
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
						<div class="w-full">
							<div
								class="grid inline-flex h-10 w-full grid-cols-8 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground"
							>
								<a
								href={`/anime/${animeId}/anime`}
								class={page.url.pathname === `/anime/${animeId}/anime` ||
								page.url.pathname === `/anime/${animeId}/anime/`
									? activeTabClass
									: inactiveTabClass}
							>
								<div class="flex items-center gap-2">
									<Icon icon="solar:play-circle-bold" class="size-4" />
									Watch
								</div>
							</a>
								<a
									href={`/anime/${animeId}/characters`}
									class={page.url.pathname.endsWith('/characters')
										? activeTabClass
										: inactiveTabClass}
								>
									Characters
								</a>
								<a
									href={`/anime/${animeId}/torrents`}
									class={page.url.pathname.endsWith('/torrents')
										? activeTabClass
										: inactiveTabClass}
								>
									<div class="flex items-center gap-2">
										<Icon icon="solar:download-bold-duotone" class="size-4" />
										Torrents
									</div>
								</a>
								<a
									href={`/anime/${animeId}/staff`}
									class={page.url.pathname.endsWith('/staff') ? activeTabClass : inactiveTabClass}
								>
									Staff
								</a>
								<a
									href={`/anime/${animeId}/reviews`}
									class={page.url.pathname.endsWith('/reviews') ? activeTabClass : inactiveTabClass}
								>
									Reviews
								</a>
								<a
									href={`/anime/${animeId}/stats`}
									class={page.url.pathname.endsWith('/stats') ? activeTabClass : inactiveTabClass}
								>
									Stats
								</a>
								<a
									href={`/anime/${animeId}/related`}
									class={page.url.pathname.endsWith('/related') ? activeTabClass : inactiveTabClass}
								>
									Related
								</a>
								<a
									href={`/anime/${animeId}/recommendations`}
									class={page.url.pathname.endsWith('/recommendations')
										? activeTabClass
										: inactiveTabClass}
								>
									Recs
								</a>
							</div>

							<div class="mt-6 w-full">
								{@render children()}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
