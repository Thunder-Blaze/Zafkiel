<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import { recommendationApi } from '$lib/services/anilist';
	import type { Recommendation } from '$lib/types/anilist';

	let filter = $state<'all' | 'recent' | 'highest' | 'lowest'>('all');
	let page = $state(1);

	const recQ = createQuery(() => ({
		queryKey: ['recommendation', 'global', page, 25],
		queryFn: () => recommendationApi.getGlobal(page, 25),
		staleTime: 10 * 60 * 1000,
	}));

	const allRecs = $derived((recQ.data?.data?.data || recQ.data?.data || []) as Recommendation[]);

	const recs = $derived(() => {
		if (filter === 'highest') return [...allRecs].sort((a, b) => (b.rating ?? 0) - (a.rating ?? 0));
		if (filter === 'lowest') return [...allRecs].sort((a, b) => (a.rating ?? 0) - (b.rating ?? 0));
		return allRecs;
	});

	const FILTERS = [
		{ id: 'all', label: 'All' },
		{ id: 'recent', label: 'Recent' },
		{ id: 'highest', label: 'Highest Rated' },
		{ id: 'lowest', label: 'Lowest Rated' },
	] as const;

	function mediaTitle(r?: Recommendation['media'] | Recommendation['mediaRecommendation']): string {
		if (!r?.title) return 'Unknown';
		return r.title.userPreferred || r.title.romaji || r.title.english || 'Unknown';
	}
	function coverImg(r?: Recommendation['media'] | Recommendation['mediaRecommendation']): string {
		return r?.coverImage?.large || r?.coverImage?.medium || '';
	}
</script>

<div class="flex flex-col gap-6 px-6 py-8">
	<!-- Header -->
	<div class="flex items-center gap-3">
		<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
			<Icon icon="solar:magic-stick-3-bold-duotone" class="h-5 w-5 text-primary" />
		</div>
		<div>
			<h1 class="text-2xl font-bold">Recommendations</h1>
			<p class="text-sm text-muted-foreground">If you liked this, you might enjoy that</p>
		</div>
	</div>

	<!-- Filter tabs -->
	<div class="flex flex-wrap gap-2">
		{#each FILTERS as f}
			<button
				onclick={() => {
					filter = f.id;
					page = 1;
				}}
				class="rounded-full px-4 py-1.5 text-sm font-medium transition-colors {filter === f.id
					? 'bg-primary text-primary-foreground'
					: 'bg-muted text-muted-foreground hover:text-foreground'}"
			>
				{f.label}
			</button>
		{/each}
	</div>

	<!-- Grid -->
	{#if recQ.isLoading}
		<div class="flex h-64 items-center justify-center">
			<Icon icon="solar:spinner-bold" class="h-8 w-8 animate-spin text-muted-foreground" />
		</div>
	{:else if recs().length === 0}
		<div
			class="flex h-48 items-center justify-center rounded-xl border border-dashed text-muted-foreground"
		>
			<div class="text-center">
				<Icon icon="solar:magic-stick-3-linear" class="mx-auto mb-2 h-10 w-10 opacity-40" />
				<p>No recommendations found</p>
			</div>
		</div>
	{:else}
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
			{#each recs() as rec}
				<div
					class="flex flex-col overflow-hidden rounded-xl border border-border/50 bg-card transition-colors hover:border-primary/30"
				>
					<!-- Side-by-side covers -->
					<div class="flex h-36 overflow-hidden">
						<button
							class="w-1/2 overflow-hidden"
							onclick={() => rec.media?.id && goto(`/anime/${rec.media.id}`)}
						>
							{#if coverImg(rec.media)}
								<img
									src={coverImg(rec.media)}
									alt={mediaTitle(rec.media)}
									class="h-full w-full object-cover transition-transform duration-300 hover:scale-105"
									onerror={(e) => {
										(e.currentTarget as HTMLImageElement).style.display = 'none';
									}}
								/>
							{:else}
								<div class="flex h-full items-center justify-center bg-muted">
									<Icon icon="solar:image-linear" class="h-6 w-6 opacity-40" />
								</div>
							{/if}
						</button>

						<!-- Divider arrow -->
						<div class="relative shrink-0">
							<div
								class="absolute inset-0 z-10 flex w-7 items-center justify-center bg-background/80"
							>
								<Icon icon="solar:alt-arrow-right-bold" class="h-4 w-4 text-primary" />
							</div>
						</div>

						<button
							class="w-1/2 overflow-hidden"
							onclick={() =>
								rec.mediaRecommendation?.id && goto(`/anime/${rec.mediaRecommendation.id}`)}
						>
							{#if coverImg(rec.mediaRecommendation)}
								<img
									src={coverImg(rec.mediaRecommendation)}
									alt={mediaTitle(rec.mediaRecommendation)}
									class="h-full w-full object-cover transition-transform duration-300 hover:scale-105"
									onerror={(e) => {
										(e.currentTarget as HTMLImageElement).style.display = 'none';
									}}
								/>
							{:else}
								<div class="flex h-full items-center justify-center bg-muted">
									<Icon icon="solar:image-linear" class="h-6 w-6 opacity-40" />
								</div>
							{/if}
						</button>
					</div>

					<!-- Titles row -->
					<div class="flex gap-1 px-2 pt-2 text-[10px] leading-snug font-medium">
						<span class="w-1/2 truncate text-center">{mediaTitle(rec.media)}</span>
						<span class="w-1/2 truncate text-center text-primary"
							>{mediaTitle(rec.mediaRecommendation)}</span
						>
					</div>

					<!-- Rating row -->
					<div
						class="mt-auto flex items-center justify-between gap-2 border-t border-border/30 px-3 py-2.5"
					>
						{#if rec.user?.name}
							<span class="truncate text-[10px] text-muted-foreground">by {rec.user.name}</span>
						{:else}
							<span></span>
						{/if}
						<div class="flex shrink-0 items-center gap-2">
							<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
								<Icon icon="solar:like-bold" class="h-3.5 w-3.5 text-green-400" />
								<span>{rec.rating ?? 0}</span>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>

		<!-- Pagination -->
		<div class="flex items-center justify-center gap-3 pt-2">
			<button
				onclick={() => (page = Math.max(1, page - 1))}
				disabled={page === 1}
				class="flex h-8 w-8 items-center justify-center rounded-lg border border-border bg-card transition-colors hover:border-primary/40 disabled:opacity-40"
			>
				<Icon icon="solar:alt-arrow-left-linear" class="h-4 w-4" />
			</button>
			<span class="text-sm text-muted-foreground">Page {page}</span>
			<button
				onclick={() => (page += 1)}
				class="flex h-8 w-8 items-center justify-center rounded-lg border border-border bg-card transition-colors hover:border-primary/40"
			>
				<Icon icon="solar:alt-arrow-right-linear" class="h-4 w-4" />
			</button>
		</div>
	{/if}
</div>
