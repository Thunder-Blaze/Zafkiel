<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import SearchHeroCarousel from '$lib/components/SearchHeroCarousel.svelte';
	import {
		useTrendingManga,
		usePopularManga,
		useBrowseMedia,
		useAddMangaToList,
	} from '$lib/hooks/useAnilist.svelte';
	import type { Media, MediaListStatus } from '$lib/types/anilist';
	import { hscroll } from '$lib/utils/actions';
	import { toast } from 'svelte-sonner';
	import { gsapReveal, gsapStagger } from '$lib/utils/gsap-animations';

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

	const trending = $derived(
		Array.isArray(trendingQ.data?.data) ? (trendingQ.data.data as Media[]) : []
	);
	const popular = $derived(
		Array.isArray(popularQ.data?.data) ? (popularQ.data.data as Media[]) : []
	);
	const manhwa = $derived(
		Array.isArray(manhwaQ.data?.data?.data)
			? (manhwaQ.data.data.data as Media[])
			: Array.isArray(manhwaQ.data?.data)
				? (manhwaQ.data.data as Media[])
				: []
	);
	const novels = $derived(
		Array.isArray(noVelQ.data?.data?.data)
			? (noVelQ.data.data.data as Media[])
			: Array.isArray(noVelQ.data?.data)
				? (noVelQ.data.data as Media[])
				: []
	);

	const heroes = $derived(trending.filter((m) => m.bannerImage).slice(0, 8));

	// ── Add to List ────────────────────────────────────────────────────────────
	const addToListMutation = useAddMangaToList();
	const addToListPending = $derived(addToListMutation.isPending);

	const statusOptions: { value: MediaListStatus; label: string; icon: string }[] = [
		{ value: 'PLANNING', label: 'Plan to Read', icon: 'solar:bookmark-linear' },
		{ value: 'CURRENT', label: 'Currently Reading', icon: 'solar:book-2-linear' },
		{ value: 'COMPLETED', label: 'Completed', icon: 'solar:check-circle-linear' },
		{ value: 'PAUSED', label: 'On Hold', icon: 'solar:pause-circle-linear' },
		{ value: 'DROPPED', label: 'Dropped', icon: 'solar:close-circle-linear' },
	];

	async function handleAddToList(status: MediaListStatus, item: Media) {
		try {
			await addToListMutation.mutateAsync({ mediaId: item.id, status });
			const labels: Record<MediaListStatus, string> = {
				CURRENT: 'Reading',
				COMPLETED: 'Completed',
				PLANNING: 'Plan to Read',
				DROPPED: 'Dropped',
				PAUSED: 'On Hold',
				REPEATING: 'Repeating',
			};
			toast.success(`Added: ${labels[status]}`);
		} catch {
			toast.error('Failed to add to list');
		}
	}
</script>

<div class="flex flex-col gap-0">
	<!-- ── Hero Carousel ────────────────────────────────────────────────────── -->
	<SearchHeroCarousel
		items={heroes}
		mediaType="manga"
		{statusOptions}
		{addToListPending}
		onAddToList={handleAddToList}
	/>

	<!-- ── Content Sections ─────────────────────────────────────────────────── -->
	<div class="flex flex-col gap-10 px-6 py-8">
		<!-- Trending Now -->
		<section id="trending" use:gsapReveal>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Trending Now</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto('/browse/manga?sort=TRENDING_DESC')}
					class="gap-1 text-xs text-muted-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if trendingQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else}
				<div use:hscroll>
					<div use:gsapStagger class="flex gap-8 py-4">
						{#each trending as media}
							<div class="w-fit shrink-0"><MediaCard {media} /></div>
						{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- All Time Popular -->
		<section id="popular" use:gsapReveal>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">All Time Popular</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto('/browse/manga?sort=POPULARITY_DESC')}
					class="gap-1 text-xs text-muted-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if popularQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else}
				<div use:hscroll>
					<div use:gsapStagger class="flex gap-8 py-4">
						{#each popular as media}
							<div class="w-fit shrink-0"><MediaCard {media} /></div>
						{/each}
					</div>
				</div>
			{/if}
		</section>

		<!-- Top Manhwa -->
		<section id="manhwa" use:gsapReveal>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Top Manhwa</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto('/browse/manga?country=KR&sort=POPULARITY_DESC')}
					class="gap-1 text-xs text-muted-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if manhwaQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if manhwa.length > 0}
				<div use:hscroll>
					<div use:gsapStagger class="flex gap-8 py-4">
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
		<section id="novels" use:gsapReveal>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-xl font-bold">Light Novels</h2>
				<Button
					variant="ghost"
					size="sm"
					onclick={() => goto('/browse/manga?format=NOVEL&sort=SCORE_DESC')}
					class="gap-1 text-xs text-muted-foreground"
				>
					View All <Icon icon="solar:arrow-right-linear" class="h-3.5 w-3.5" />
				</Button>
			</div>
			{#if noVelQ.isLoading}
				<div class="flex h-48 items-center justify-center">
					<Icon icon="solar:spinner-bold" class="h-6 w-6 animate-spin text-muted-foreground" />
				</div>
			{:else if novels.length > 0}
				<div use:hscroll>
					<div use:gsapStagger class="flex gap-8 py-4">
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
