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
		{ slug: 'torrents', label: 'Torrents', icon: 'solar:download-bold-duotone' },
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
					</div>
				</div>
			</div>
		</div>

		<!-- ── Main content: vertical tabs + page slot ───────────────── -->
		<div class="mt-4 flex w-full gap-6 px-4 pb-12 md:px-8 lg:px-10" in:fade={{ duration: 300 }}>
			<!-- Vertical tab sidebar -->
			<nav class="hidden w-72 shrink-0 md:block">
				<div class="sticky top-16 space-y-1">
					<!-- Spacer to clear the absolutely positioned poster -->
					<div class="mb-4 h-80 w-full"></div>
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
				<div class="-mx-4 mb-6 overflow-x-auto px-4 md:hidden">
					<div class="flex w-max gap-1 rounded-lg bg-muted p-1">
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
