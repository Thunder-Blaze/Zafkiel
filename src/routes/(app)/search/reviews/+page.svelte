<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import { reviewApi } from '$lib/services/anilist';
	import type { Review } from '$lib/types/anilist';

	let filter = $state<'all' | 'recent' | 'highest' | 'lowest'>('all');
	let page = $state(1);

	const reviewsQ = createQuery(() => ({
		queryKey: ['review', 'recent', page],
		queryFn: () => reviewApi.getRecent(page, 25),
		staleTime: 10 * 60 * 1000,
	}));

	const allReviews = $derived((reviewsQ.data?.data?.data || reviewsQ.data?.data || []) as Review[]);

	const reviews = $derived(() => {
		if (filter === 'highest')
			return [...allReviews].sort((a, b) => (b.score ?? 0) - (a.score ?? 0));
		if (filter === 'lowest') return [...allReviews].sort((a, b) => (a.score ?? 0) - (b.score ?? 0));
		return allReviews;
	});

	const FILTERS = [
		{ id: 'all', label: 'All' },
		{ id: 'recent', label: 'Recent' },
		{ id: 'highest', label: 'Highest Rated' },
		{ id: 'lowest', label: 'Lowest Rated' },
	] as const;

	function mediaTitle(r: Review) {
		return (
			r.media?.title?.userPreferred ||
			r.media?.title?.romaji ||
			r.media?.title?.english ||
			`Media #${r.mediaId}`
		);
	}
	function userName(r: Review) {
		return r.user?.name || 'Anonymous';
	}
	function coverImg(r: Review): string {
		return r.media?.bannerImage || r.media?.coverImage?.large || r.media?.coverImage?.medium || '';
	}
	function excerpt(r: Review): string {
		const text = r.summary || r.body || '';
		return text.length > 120 ? text.slice(0, 120) + '…' : text;
	}
	function scoreColor(score?: number) {
		if (!score) return 'text-muted-foreground';
		if (score >= 75) return 'text-green-400';
		if (score >= 50) return 'text-yellow-400';
		return 'text-red-400';
	}
</script>

<div class="flex flex-col gap-6 px-6 py-8">
	<!-- Header -->
	<div class="flex items-center gap-3">
		<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
			<Icon icon="solar:document-text-bold-duotone" class="h-5 w-5 text-primary" />
		</div>
		<div>
			<h1 class="text-2xl font-bold">Reviews</h1>
			<p class="text-sm text-muted-foreground">Community reviews from around the world</p>
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
	{#if reviewsQ.isLoading}
		<div class="flex h-64 items-center justify-center">
			<Icon icon="solar:spinner-bold" class="h-8 w-8 animate-spin text-muted-foreground" />
		</div>
	{:else if reviews().length === 0}
		<div
			class="flex h-48 items-center justify-center rounded-xl border border-dashed text-muted-foreground"
		>
			<div class="text-center">
				<Icon icon="solar:document-text-linear" class="mx-auto mb-2 h-10 w-10 opacity-40" />
				<p>No reviews found</p>
			</div>
		</div>
	{:else}
		<div class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
			{#each reviews() as review}
				<button
					onclick={() => goto(`/review/${review.id}`)}
					class="group flex flex-col overflow-hidden rounded-xl border border-border/50 bg-card text-left transition-colors hover:border-primary/30 hover:bg-card/80"
				>
					<!-- Cover image -->
					<div class="relative h-28 w-full overflow-hidden bg-muted">
						{#if coverImg(review)}
							<img
								src={coverImg(review)}
								alt={mediaTitle(review)}
								class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
								onerror={(e) => {
									(e.currentTarget as HTMLImageElement).style.display = 'none';
								}}
							/>
						{/if}
						<div class="absolute inset-0 bg-linear-to-t from-black/70 to-transparent"></div>
						{#if review.score}
							<div
								class="absolute right-2 bottom-2 rounded-md bg-black/60 px-1.5 py-0.5 text-xs font-bold {scoreColor(
									review.score
								)}"
							>
								{review.score}%
							</div>
						{/if}
					</div>

					<!-- Content -->
					<div class="flex flex-1 flex-col gap-1 p-3">
						<p class="line-clamp-2 text-xs leading-snug font-semibold">
							Review of <span class="text-primary">{mediaTitle(review)}</span>
						</p>
						<p class="text-[10px] text-muted-foreground">by {userName(review)}</p>
						{#if excerpt(review)}
							<p class="mt-1 line-clamp-3 text-[10px] leading-relaxed text-muted-foreground">
								{excerpt(review)}
							</p>
						{/if}
						<div class="mt-auto flex items-center gap-3 border-t border-border/30 pt-2">
							<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
								<Icon icon="solar:like-bold" class="h-3 w-3 text-green-400" />
								{review.rating ?? 0}
							</div>
							{#if review.ratingAmount}
								<div class="flex items-center gap-1 text-[10px] text-muted-foreground">
									<Icon icon="solar:users-group-two-rounded-linear" class="h-3 w-3" />
									{review.ratingAmount}
								</div>
							{/if}
						</div>
					</div>
				</button>
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
