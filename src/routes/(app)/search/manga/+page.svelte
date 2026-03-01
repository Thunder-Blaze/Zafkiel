<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import {
		useTrendingManga,
		usePopularManga,
		useBrowseMedia,
	} from '$lib/hooks/useAnilist.svelte';
	import type { Media } from '$lib/types/anilist';

	// ── Data queries ───────────────────────────────────────────────────────────
	const trendingQ = useTrendingManga({ page: 1, perPage: 20 });
	const popularQ = usePopularManga({ page: 1, perPage: 20 });
	const manhwaQ = useBrowseMedia({
		mediaType: 'MANGA',
		sortBy: ['POPULARITY_DESC'],
		genres: ['Action'],
		page: 1,
		perPage: 20,
	});
	const noVelQ = useBrowseMedia({
		mediaType: 'MANGA',
		format: 'NOVEL',
		sortBy: ['SCORE_DESC'],
		page: 1,
		perPage: 20,
	});

	const trending = $derived((trendingQ.data?.data || []) as Media[]);
	const popular = $derived((popularQ.data?.data || []) as Media[]);
	const manhwa = $derived((manhwaQ.data?.data?.data || manhwaQ.data?.data || []) as Media[]);
	const novels = $derived((noVelQ.data?.data?.data || noVelQ.data?.data || []) as Media[]);

	// ── Hero carousel ──────────────────────────────────────────────────────────
	const heroes = $derived(trending.filter((m) => m.bannerImage).slice(0, 8));
	let heroIdx = $state(0);
	let heroTimer: ReturnType<typeof setInterval> | null = null;

	$effect(() => {
		if (heroes.length > 1) {
			heroTimer = setInterval(() => { heroIdx = (heroIdx + 1) % heroes.length; }, 6000);
		}
		return () => { if (heroTimer) clearInterval(heroTimer); };
	});

	function prevHero() {
		heroIdx = (heroIdx - 1 + heroes.length) % heroes.length;
	}
	function nextHero() {
		heroIdx = (heroIdx + 1) % heroes.length;
	}

	const currentHero = $derived(heroes[heroIdx] ?? null);

	function mediaTitle(m: Media) {
		return m.title?.english || m.title?.romaji || m.title?.native || 'Unknown';
	}
</script>

<div class="flex flex-col gap-0">
	<!-- ── Hero Carousel ────────────────────────────────────────────────────── -->
	<div class="relative h-[480px] overflow-hidden bg-muted">
		{#if currentHero}
			<div class="absolute inset-0 transition-all duration-700">
				<img src={currentHero.bannerImage} alt={mediaTitle(currentHero)} class="h-full w-full object-cover" />
				<div class="absolute inset-0 bg-gradient-to-r from-black/80 via-black/40 to-transparent"></div>
				<div class="absolute inset-0 bg-gradient-to-t from-background via-transparent to-transparent"></div>
			</div>

			<div class="relative flex h-full max-w-3xl flex-col justify-end gap-3 px-8 pb-16">
				{#if currentHero.format}
					<div class="text-xs font-semibold tracking-widest text-primary uppercase">{currentHero.format}</div>
				{/if}
				<h1 class="text-3xl font-bold leading-tight text-white drop-shadow-lg">{mediaTitle(currentHero)}</h1>
				<div class="flex flex-wrap items-center gap-3 text-sm text-white/70">
					{#if currentHero.averageScore}
						<span class="flex items-center gap-1">
							<Icon icon="solar:star-bold" class="h-4 w-4 text-yellow-400" />
							{(currentHero.averageScore / 10).toFixed(1)}
						</span>
					{/if}
					{#if currentHero.chapters}<span>{currentHero.chapters} ch</span>{/if}
					{#if currentHero.volumes}<span>{currentHero.volumes} vol</span>{/if}
					{#each (currentHero.genres ?? []).slice(0, 3) as genre}
						<span class="rounded-full border border-white/20 px-2 py-0.5 text-xs">{genre}</span>
					{/each}
				</div>
				{#if currentHero.description}
					<p class="line-clamp-2 max-w-lg text-sm leading-relaxed text-white/60">
						{@html currentHero.description.replace(/<[^>]*>/g, '')}
					</p>
				{/if}
				<div class="flex gap-3 pt-1">
					<Button onclick={() => goto(`/manga/${currentHero.id}`)} class="gap-2">
						<Icon icon="solar:book-2-bold" class="h-4 w-4" />
						View Details
					</Button>
					<Button variant="outline" class="gap-2 border-white/20 bg-white/10 text-white hover:bg-white/20">
						<Icon icon="solar:add-circle-bold" class="h-4 w-4" />
						Add to List
					</Button>
				</div>
			</div>

			<button onclick={prevHero} class="absolute left-4 top-1/2 -translate-y-1/2 rounded-full bg-black/40 p-2 text-white backdrop-blur-sm transition-colors hover:bg-black/60" aria-label="Previous">
				<Icon icon="solar:alt-arrow-left-bold" class="h-5 w-5" />
			</button>
			<button onclick={nextHero} class="absolute right-4 top-1/2 -translate-y-1/2 rounded-full bg-black/40 p-2 text-white backdrop-blur-sm transition-colors hover:bg-black/60" aria-label="Next">
				<Icon icon="solar:alt-arrow-right-bold" class="h-5 w-5" />
			</button>

			<div class="absolute bottom-6 left-1/2 flex -translate-x-1/2 gap-1.5">
				{#each heroes as _, i}
					<button onclick={() => heroIdx = i} class="h-1.5 rounded-full transition-all {i === heroIdx ? 'w-6 bg-primary' : 'w-1.5 bg-white/40'}" aria-label="Go to slide {i + 1}"></button>
				{/each}
			</div>
		{:else}
			<div class="flex h-full items-center justify-center text-muted-foreground">
				<Icon icon="solar:spinner-bold" class="h-8 w-8 animate-spin" />
			</div>
		{/if}
	</div>

	<!-- ── Content Sections ─────────────────────────────────────────────────── -->
	<div class="flex flex-col gap-10 px-6 py-8">

		<!-- Trending Now -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Trending Now</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/search?type=MANGA&sort=TRENDING_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if trendingQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else}
				<div class="flex gap-3 overflow-x-auto pb-3" style="scrollbar-width: thin;">
					{#each trending as media}
						<div class="w-[140px] shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			{/if}
		</section>

		<!-- All Time Popular -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">All Time Popular</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/search?type=MANGA&sort=POPULARITY_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if popularQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else}
				<div class="flex gap-3 overflow-x-auto pb-3" style="scrollbar-width: thin;">
					{#each popular as media}
						<div class="w-[140px] shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			{/if}
		</section>

		<!-- Top Manhwa -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Top Manhwa</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/search?type=MANGA&sort=POPULARITY_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if manhwaQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if manhwa.length > 0}
				<div class="flex gap-3 overflow-x-auto pb-3" style="scrollbar-width: thin;">
					{#each manhwa as media}
						<div class="w-[140px] shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			{:else}
				<p class="text-sm text-muted-foreground">No results.</p>
			{/if}
		</section>

		<!-- Light Novels -->
		<section>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Light Novels</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/search?type=MANGA&format=NOVEL&sort=SCORE_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if noVelQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if novels.length > 0}
				<div class="flex gap-3 overflow-x-auto pb-3" style="scrollbar-width: thin;">
					{#each novels as media}
						<div class="w-[140px] shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			{:else}
				<p class="text-sm text-muted-foreground">No results.</p>
			{/if}
		</section>

	</div>
</div>
