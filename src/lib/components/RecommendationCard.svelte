<script lang="ts">
	import type { Recommendation } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	let {
		recommendation,
		sourceMediaId,
	}: { recommendation: Recommendation; sourceMediaId?: number } = $props();

	const target = $derived(recommendation.mediaRecommendation);
</script>

{#if target}
	<button
		class="group flex flex-col overflow-hidden rounded-lg border bg-card transition-all hover:bg-card/80 hover:ring-2 hover:ring-primary/30"
		onclick={() => target.id && goto(`/${(target.type ?? 'anime').toLowerCase()}/${target.id}`)}
	>
		<!-- Cover -->
		<div class="relative h-40 w-full overflow-hidden">
			{#if target.coverImage?.large}
				<CachedImage
					src={target.coverImage.large}
					alt={target.title?.userPreferred ?? ''}
					class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
				/>
			{:else}
				<div class="flex h-full w-full items-center justify-center bg-muted">
					<Icon icon="solar:image-broken-linear" class="h-10 w-10 text-muted-foreground" />
				</div>
			{/if}
			<!-- Rating badge -->
			{#if (recommendation.rating ?? 0) > 0}
				<div
					class="absolute top-2 right-2 flex items-center gap-1 rounded-md bg-card/85 px-2 py-0.5 text-xs backdrop-blur-sm"
				>
					<Icon icon="solar:thumbs-up-bold" class="size-3.5 text-green-500" />
					{recommendation.rating}
				</div>
			{/if}
		</div>

		<!-- Info -->
		<div class="flex flex-col gap-1 p-3">
			<h3 class="line-clamp-2 text-sm leading-tight font-semibold">
				{target.title?.userPreferred ?? 'Unknown'}
			</h3>
			<div class="flex items-center gap-2 text-xs text-muted-foreground">
				{#if target.format}
					<span>{target.format}</span>
				{/if}
				{#if target.averageScore}
					<span class="flex items-center gap-0.5">
						<Icon icon="solar:star-bold" class="size-3 text-primary" />
						{target.averageScore}%
					</span>
				{/if}
			</div>
		</div>
	</button>
{/if}
