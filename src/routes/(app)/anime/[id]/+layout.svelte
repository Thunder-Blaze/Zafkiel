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

	// ── Tab configuration ─────────────────────────────────────────────────
	interface TabDef {
		slug: string;
		label: string;
		icon: string;
	}

	const tabs: TabDef[] = [
		{ slug: '', label: 'About', icon: 'solar:info-circle-bold' },
		{ slug: 'anime', label: 'Watch', icon: 'solar:play-circle-bold' },
		{ slug: 'characters', label: 'Characters', icon: 'solar:users-group-rounded-bold' },
		{ slug: 'torrents', label: 'Torrents', icon: 'solar:download-bold-duotone' },
		{ slug: 'staff', label: 'Staff', icon: 'solar:user-id-bold' },
		{ slug: 'reviews', label: 'Reviews', icon: 'solar:chat-square-bold' },
		{ slug: 'stats', label: 'Stats', icon: 'solar:chart-2-bold' },
		{ slug: 'related', label: 'Related', icon: 'solar:link-round-bold' },
		{ slug: 'recommendations', label: 'Recs', icon: 'solar:star-bold' },
	];

	function isTabActive(tabSlug: string): boolean {
		const pathname = page.url.pathname.replace(/\/$/, '');
		const base = `/anime/${animeId}`;
		if (tabSlug === '') {
			return pathname === base;
		}
		return pathname === `${base}/${tabSlug}` || pathname.startsWith(`${base}/${tabSlug}/`);
	}

	function tabHref(tabSlug: string): string {
		return tabSlug === '' ? `/anime/${animeId}` : `/anime/${animeId}/${tabSlug}`;
	}
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
		<!-- ── Banner ────────────────────────────────────────────────── -->
		<div class="relative flex h-[24vw] w-full flex-col">
			<img
				src={bannerImage || coverImage}
				alt={title}
				class="absolute inset-0 z-1 h-full w-full object-cover"
				in:fly={{ y: -100, duration: 300 }}
			/>
			{#if $glow}
				<img
					src={bannerImage || coverImage}
					alt="Blurred background"
					class="absolute inset-0 h-full w-full object-cover"
					style="filter: blur(30px); -webkit-filter: blur(30px);"
					in:fly={{ y: -100, duration: 300 }}
				/>
			{/if}
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

		<!-- ── Header row (title, buttons) ───────────────────────────── -->
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
				</div>
			</div>
		</div>

		<!-- ── Main content: vertical tabs + page slot ───────────────── -->
		<div class="mt-8 flex w-full gap-6 px-4 md:px-8 lg:px-10 pb-12" in:fade={{ duration: 300 }}>
			<!-- Vertical tab sidebar -->
			<nav class="hidden w-48 shrink-0 md:block">
				<div class="sticky top-16 space-y-1">
					{#each tabs as tab}
						{@const active = isTabActive(tab.slug)}
						<a
							href={tabHref(tab.slug)}
							class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-150
								{active
									? 'bg-primary/10 text-primary shadow-sm'
									: 'text-muted-foreground hover:bg-muted/50 hover:text-foreground'}"
						>
							<Icon icon={tab.icon} class="size-4 shrink-0" />
							{tab.label}
						</a>
					{/each}
				</div>
			</nav>

			<!-- Mobile horizontal tab bar (visible below md) -->
			<div class="contents md:hidden">
				<!-- This wrapper ensures the mobile bar + content flow vertically -->
			</div>

			<!-- Tab content area -->
			<div class="min-w-0 flex-1">
				<!-- Mobile horizontal scrollable tabs -->
				<div class="mb-6 -mx-4 px-4 md:hidden overflow-x-auto">
					<div class="flex gap-1 rounded-lg bg-muted p-1 w-max">
						{#each tabs as tab}
							{@const active = isTabActive(tab.slug)}
							<a
								href={tabHref(tab.slug)}
								class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-all
									{active
										? 'bg-background text-foreground shadow-sm'
										: 'text-muted-foreground hover:text-foreground'}"
							>
								<Icon icon={tab.icon} class="size-3.5" />
								{tab.label}
							</a>
						{/each}
					</div>
				</div>

				{@render children()}
			</div>
		</div>
	</div>
{/if}
