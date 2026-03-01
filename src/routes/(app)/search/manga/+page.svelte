<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import { fade, fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { useConfigState } from '$lib/stores/config.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import {
		useTrendingManga,
		usePopularManga,
		useBrowseMedia,
	} from '$lib/hooks/useAnilist.svelte';
	import type { Media } from '$lib/types/anilist';
	import { hscroll } from '$lib/utils/actions';

	// ── Data queries ───────────────────────────────────────────────────────────
	const trendingQ = useTrendingManga({ page: 1, perPage: 20 });
	const popularQ = usePopularManga({ page: 1, perPage: 20 });
	const manhwaQ = useBrowseMedia({
		mediaType: 'MANGA',
		sortBy: ['POPULARITY_DESC'],
		countryOfOrigin: 'KR',
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

	const trending = $derived(Array.isArray(trendingQ.data?.data) ? (trendingQ.data.data as Media[]) : []);
	const popular = $derived(Array.isArray(popularQ.data?.data) ? (popularQ.data.data as Media[]) : []);
	const manhwa = $derived(Array.isArray(manhwaQ.data?.data?.data) ? (manhwaQ.data.data.data as Media[]) : Array.isArray(manhwaQ.data?.data) ? (manhwaQ.data.data as Media[]) : []);
	const novels = $derived(Array.isArray(noVelQ.data?.data?.data) ? (noVelQ.data.data.data as Media[]) : Array.isArray(noVelQ.data?.data) ? (noVelQ.data.data as Media[]) : []);

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
		heroDirection = 'left';
		heroIdx = (heroIdx - 1 + heroes.length) % heroes.length;
	}
	function nextHero() {
		heroDirection = 'right';
		heroIdx = (heroIdx + 1) % heroes.length;
	}

	const currentHero = $derived(heroes[heroIdx] ?? null);

	let heroDirection = $state<'left' | 'right'>('right');
	const config = useConfigState();
	const animationsEnabled = $derived(config.animations);
	const bgDuration = $derived(animationsEnabled ? 700 : 120);
	const contentDuration = $derived(animationsEnabled ? 500 : 120);
	const contentDelay = $derived(animationsEnabled ? 180 : 0);

	function mediaTitle(m: Media) {
		return m.title?.english || m.title?.romaji || m.title?.native || 'Unknown';
	}
</script>

<div class="flex flex-col gap-0">
	<!-- ── Hero Carousel ────────────────────────────────────────────────────── -->
	<div class="group relative h-[420px] overflow-hidden">
		{#if currentHero}
			<!-- Background -->
			{#key currentHero.id}
				<div
					in:fade={{ duration: bgDuration, easing: cubicOut }}
					out:fade={{ duration: Math.floor(bgDuration * 0.6), easing: cubicOut }}
					class="absolute inset-0 bg-cover bg-center"
					style="background-image: url('{currentHero.bannerImage}')"
				>
					<div class="absolute inset-0 bg-linear-to-r from-background/90 via-background/55 to-background/10"></div>
					<div class="absolute inset-0 bg-linear-to-t from-background/80 via-transparent to-transparent"></div>
				</div>
			{/key}

			<!-- Content grid: text | cover art -->
			<div class="relative grid h-full grid-cols-[1fr_auto] items-end gap-4 px-8 pb-10 lg:px-14">
				<div class="relative grid min-w-0">
				{#key currentHero.id}
					<div
						style="grid-area: 1/1"
						in:fly={{ x: heroDirection === 'right' ? 50 : -50, y: 0, duration: contentDuration, delay: contentDelay, easing: cubicOut }}
						out:fly={{ x: heroDirection === 'right' ? -30 : 30, y: 0, duration: Math.floor(contentDuration * 0.6), easing: cubicOut }}
						class="flex max-w-[760px] flex-col gap-2.5"
					>
						{#if currentHero.format}
							<div class="text-[11px] font-bold tracking-widest text-primary uppercase">{currentHero.format.replace(/_/g, ' ')}</div>
						{/if}
						<h1 class="line-clamp-1 text-3xl font-bold leading-tight drop-shadow-lg md:text-4xl">{mediaTitle(currentHero)}</h1>
						<div class="flex flex-wrap items-center gap-2 text-sm">
							{#if currentHero.averageScore}
								<span class="flex items-center gap-1 font-semibold">
									<Icon icon="solar:star-bold" class="h-3.5 w-3.5 text-yellow-400" />
									{(currentHero.averageScore / 10).toFixed(1)}
								</span>
							{/if}
							{#if currentHero.chapters}<span class="text-xs text-muted-foreground">{currentHero.chapters} ch</span>{/if}
							{#if currentHero.volumes}<span class="text-xs text-muted-foreground">{currentHero.volumes} vol</span>{/if}
							{#each (currentHero.genres ?? []).slice(0, 3) as genre}
									<span class="rounded-md border border-border/40 bg-background/40 px-2.5 py-0.5 text-[11px] backdrop-blur-sm">{genre}</span>
							{/each}
						</div>
						{#if currentHero.description}
							<p class="line-clamp-2 max-w-lg text-xs leading-relaxed text-muted-foreground">
								{@html currentHero.description.replace(/<[^>]*>/g, '').substring(0, 220)}
							</p>
						{/if}
						<div class="flex gap-2 pt-0.5">
							<Button size="sm" onclick={() => goto(`/manga/${currentHero.id}`)} class="gap-2">
								<Icon icon="solar:book-2-bold" class="h-4 w-4" />
								View Details
							</Button>
								<Button size="sm" class="gap-2 bg-foreground text-background hover:bg-foreground/90">
								<Icon icon="solar:add-circle-bold" class="h-4 w-4" />
								Add to List
							</Button>
						</div>
					</div>
				{/key}
				</div>

				<!-- Cover art -->
				<div class="relative mb-1 hidden shrink-0 md:grid">
					{#key currentHero.id}
						<div
							style="grid-area: 1/1"
							in:fly={{ x: 20, y: 0, duration: contentDuration, delay: Math.floor(contentDelay * 0.5), easing: cubicOut }}
							out:fade={{ duration: Math.floor(contentDuration * 0.4), easing: cubicOut }}
						>
							{#if currentHero.coverImage?.large || currentHero.coverImage?.medium}
								<CachedImage
									src={currentHero.coverImage.large ?? currentHero.coverImage.medium ?? ''}
									alt={mediaTitle(currentHero)}
									class="h-52 w-36 rounded-xl object-cover shadow-2xl ring-2 ring-border/40 transition-transform duration-300 group-hover:scale-[1.02]"
								/>
							{/if}
						</div>
					{/key}
				</div>
			</div>

			<!-- Navigation arrows -->
			<button
				onclick={prevHero}
				class="absolute left-4 top-1/2 -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 hover:bg-background/70 group-hover:opacity-100"
				aria-label="Previous"
			>
				<Icon icon="solar:alt-arrow-left-bold" class="h-5 w-5" />
			</button>
			<button
				onclick={nextHero}
				class="absolute right-4 top-1/2 -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 hover:bg-background/70 group-hover:opacity-100"
				aria-label="Next"
			>
				<Icon icon="solar:alt-arrow-right-bold" class="h-5 w-5" />
			</button>

			<!-- Dots -->
			<div class="absolute bottom-3 left-1/2 flex -translate-x-1/2 gap-2">
				{#each heroes as _, i}
					<button
						onclick={() => { heroDirection = i > heroIdx ? 'right' : 'left'; heroIdx = i; }}
						class="rounded-full transition-all duration-300 {i === heroIdx ? 'h-1.5 w-7 bg-primary' : 'h-1.5 w-1.5 bg-muted-foreground/40 hover:bg-muted-foreground/70'}"
						aria-label="Go to slide {i + 1}"
					></button>
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
		<section id="trending">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Trending Now</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/browse/manga?sort=TRENDING_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if trendingQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else}
			<div use:hscroll>
				<div class="flex gap-8 py-4">
					{#each trending as media}
					<div class="w-fit shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			</div>
			{/if}
		</section>

		<!-- All Time Popular -->
		<section id="popular">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">All Time Popular</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/browse/manga?sort=POPULARITY_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if popularQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else}
			<div use:hscroll>
				<div class="flex gap-8 py-4">
					{#each popular as media}
					<div class="w-fit shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			</div>
			{/if}
		</section>

		<!-- Top Manhwa -->
		<section id="manhwa">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Top Manhwa</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/browse/manga?country=KR&sort=POPULARITY_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if manhwaQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if manhwa.length > 0}
			<div use:hscroll>
				<div class="flex gap-8 py-4">
					{#each manhwa as media}
					<div class="w-fit shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			</div>
			{:else}
				<p class="text-sm text-muted-foreground">No results.</p>
			{/if}
		</section>

		<!-- Light Novels -->
		<section id="novels">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Light Novels</h2>
				<Button variant="ghost" size="sm" onclick={() => goto('/browse/manga?format=NOVEL&sort=SCORE_DESC')} class="gap-1 text-xs text-muted-foreground">
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if noVelQ.isLoading}
				<div class="flex h-48 items-center justify-center"><Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" /></div>
			{:else if novels.length > 0}
			<div use:hscroll>
				<div class="flex gap-8 py-4">
					{#each novels as media}
					<div class="w-fit shrink-0"><MediaCard {media} /></div>
					{/each}
				</div>
			</div>
			{:else}
				<p class="text-sm text-muted-foreground">No results.</p>
			{/if}
		</section>

	</div>
</div>
