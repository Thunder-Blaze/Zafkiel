<script lang="ts">
	import { useMediaReviews } from '$lib/hooks/useAnilist.svelte';
	import type { Review } from '$lib/types/anilist';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	let { mediaId }: { mediaId: number } = $props();

	let currentPage = $state(1);
	const reviewsQuery = $derived(useMediaReviews(mediaId, currentPage, 10));
	const isLoading = $derived(reviewsQuery.isLoading);
	const reviews = $derived((reviewsQuery.data?.data?.data ?? []) as Review[]);
	const pageInfo = $derived(reviewsQuery.data?.data?.pageInfo);
</script>

{#if isLoading}
	<div class="flex items-center justify-center p-8">
		<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
	</div>
{:else if reviewsQuery.error || (reviewsQuery.data && !reviewsQuery.data.success)}
	<div class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-8 text-center">
		<Icon icon="solar:danger-triangle-bold-duotone" class="h-10 w-10 text-destructive" />
		<p class="text-sm text-muted-foreground">Failed to load reviews</p>
		<Button variant="outline" size="sm" onclick={() => reviewsQuery.refetch()}>Retry</Button>
	</div>
{:else if reviews.length === 0}
	<div class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-8 text-center">
		<Icon icon="solar:document-text-bold-duotone" class="h-12 w-12 text-muted-foreground" />
		<h3 class="text-lg font-semibold">No Reviews Yet</h3>
		<p class="text-sm text-muted-foreground">Be the first to write a review on AniList!</p>
	</div>
{:else}
	<div class="flex flex-col gap-4">
		{#each reviews as review (review.id)}
			<ReviewCard {review} />
		{/each}
	</div>

	{#if pageInfo && (currentPage > 1 || pageInfo.hasNextPage)}
		<div class="mt-6 flex items-center justify-center gap-3">
			<Button variant="outline" disabled={currentPage <= 1} onclick={() => (currentPage -= 1)}>
				<Icon icon="solar:arrow-left-linear" class="size-4" />
				Previous
			</Button>
			<span class="text-sm text-muted-foreground">Page {currentPage}</span>
			<Button variant="outline" disabled={!pageInfo.hasNextPage} onclick={() => (currentPage += 1)}>
				Next
				<Icon icon="solar:arrow-right-linear" class="size-4" />
			</Button>
		</div>
	{/if}
{/if}

