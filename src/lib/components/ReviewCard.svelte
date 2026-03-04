<script lang="ts">
	import type { Review } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	let { review }: { review: Review } = $props();

	function timeAgo(ts?: number): string {
		if (!ts) return '';
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}

	const scoreColor = $derived(() => {
		if (!review.score) return 'text-muted-foreground';
		if (review.score >= 75) return 'text-green-500';
		if (review.score >= 50) return 'text-yellow-500';
		return 'text-red-500';
	});
</script>

<div class="flex flex-col gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-card/80">
	<!-- Header -->
	<div class="flex items-start gap-3">
		{#if review.media?.coverImage?.medium}
			<button
				onclick={() =>
					review.media?.id &&
					goto(`/${(review.media.type ?? 'anime').toLowerCase()}/${review.media.id}`)}
				class="shrink-0"
			>
				<CachedImage
					src={review.media.coverImage.medium}
					alt={review.media.title?.userPreferred ?? ''}
					class="h-16 w-11 rounded object-cover"
				/>
			</button>
		{/if}

		<div class="min-w-0 flex-1">
			{#if review.media}
				<button
					class="mb-1 line-clamp-1 text-sm font-semibold hover:text-primary hover:underline"
					onclick={() =>
						review.media?.id &&
						goto(`/${(review.media.type ?? 'anime').toLowerCase()}/${review.media.id}`)}
				>
					{review.media.title?.userPreferred ?? 'Unknown'}
				</button>
			{/if}
			<div class="flex items-center gap-2">
				{#if review.user?.avatar?.medium}
					<button onclick={() => review.user?.id && goto(`/user/${review.user.id}`)}>
						<CachedImage
							src={review.user.avatar.medium}
							alt={review.user.name ?? ''}
							class="h-6 w-6 rounded-full object-cover"
						/>
					</button>
				{/if}
				<button
					class="text-sm hover:text-primary hover:underline"
					onclick={() => review.user?.id && goto(`/user/${review.user.id}`)}
				>
					{review.user?.name ?? 'Unknown'}
				</button>
				{#if review.createdAt}
					<span class="text-xs text-muted-foreground">{timeAgo(review.createdAt)}</span>
				{/if}
			</div>
		</div>

		<!-- Score -->
		{#if review.score != null}
			<div class="flex shrink-0 flex-col items-center rounded-md border px-2 py-1">
				<span class="text-xs text-muted-foreground">Score</span>
				<span class="text-lg font-bold {scoreColor()}">{review.score}</span>
			</div>
		{/if}
	</div>

	<!-- Summary -->
	{#if review.summary}
		<h3 class="font-semibold">{review.summary}</h3>
	{/if}

	{#if review.body}
		<p class="line-clamp-4 text-sm text-muted-foreground">{@html review.body}</p>
	{/if}

	<!-- Footer -->
	<div class="flex items-center gap-3 text-xs text-muted-foreground">
		<span class="flex items-center gap-1">
			<Icon icon="solar:thumb-up-linear" class="size-3.5" />
			{review.rating ?? 0}
		</span>
		<span class="flex items-center gap-1">
			<Icon icon="solar:users-group-rounded-linear" class="size-3.5" />
			{review.ratingAmount ?? 0} votes
		</span>
		{#if review.private}
			<Badge variant="secondary" class="text-xs">Private</Badge>
		{/if}
	</div>
</div>
