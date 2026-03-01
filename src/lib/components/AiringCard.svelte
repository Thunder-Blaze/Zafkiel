<script lang="ts">
	import { goto } from '$app/navigation';
	import { Badge } from '$lib/components/ui/badge';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import type { Media } from '$lib/types/anilist';
	import { formatCountdown, formatScore, scoreToColorClass } from '$lib/utils/format';
	import Icon from '@iconify/svelte';

	let { media }: { media: Media } = $props();

	const title = $derived(media.title?.userPreferred ?? media.title?.english ?? 'Unknown Title');
	const cover = $derived(
		media.coverImage?.large ?? media.coverImage?.medium ?? 'https://placehold.co/230x325',
	);
	const href = $derived(`/anime/${media.id}`);

	const nextEp = $derived(media.nextAiringEpisode);
	const countdown = $derived(
		nextEp?.timeUntilAiring != null ? formatCountdown(nextEp.timeUntilAiring) : null,
	);
	const airingAt = $derived(
		nextEp?.airingAt
			? new Date(nextEp.airingAt * 1000).toLocaleTimeString([], {
					hour: '2-digit',
					minute: '2-digit',
				})
			: null,
	);

	const score = $derived(media.averageScore);
	const scoreLabel = $derived(formatScore(score));
	const scoreClass = $derived(scoreToColorClass(score));

	const FORMAT_LABELS: Record<string, string> = {
		TV: 'TV',
		TV_SHORT: 'Short',
		MOVIE: 'Movie',
		OVA: 'OVA',
		ONA: 'ONA',
		SPECIAL: 'Special',
		MUSIC: 'Music',
	};
	const formatLabel = $derived(media.format ? (FORMAT_LABELS[media.format] ?? media.format) : null);
</script>

<button
	class="group relative flex w-full cursor-pointer flex-col overflow-hidden rounded-lg border bg-card text-left transition-all duration-200 hover:-translate-y-0.5 hover:shadow-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
	onclick={() => goto(href)}
>
	<!-- Cover image -->
	<div class="relative aspect-[2/3] w-full overflow-hidden bg-muted">
		<CachedImage
			src={cover}
			alt={title}
			class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
		/>

		<!-- Top-left: episode + time -->
		{#if nextEp}
			<div class="absolute left-2 top-2 flex flex-col gap-1">
				<span
					class="rounded-md bg-black/75 px-1.5 py-0.5 text-xs font-medium text-white backdrop-blur-sm"
				>
					Ep {nextEp.episode}
				</span>
				{#if airingAt}
					<span
						class="rounded-md bg-primary/85 px-1.5 py-0.5 text-xs font-medium text-primary-foreground backdrop-blur-sm"
					>
						{airingAt}
					</span>
				{/if}
			</div>
		{/if}

		<!-- Top-right: format badge -->
		{#if formatLabel}
			<div class="absolute right-2 top-2">
				<Badge variant="secondary" class="pointer-events-none px-1.5 py-0 text-[10px] leading-5">
					{formatLabel}
				</Badge>
			</div>
		{/if}

		<!-- Bottom overlay: countdown -->
		{#if countdown}
			<div
				class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 via-black/40 to-transparent px-2 pb-2 pt-6"
			>
				<div class="flex items-center gap-1 text-xs text-white">
					<Icon icon="solar:clock-circle-bold-duotone" class="size-3 shrink-0" />
					<span class="font-medium">{countdown}</span>
				</div>
			</div>
		{/if}
	</div>

	<!-- Info strip -->
	<div class="flex items-start justify-between gap-2 p-2">
		<p
			class="min-w-0 flex-1 truncate text-[11px] font-medium leading-tight text-foreground group-hover:text-primary"
		>
			{title}
		</p>
		{#if scoreLabel !== 'N/A'}
			<span class="shrink-0 text-[11px] font-semibold tabular-nums {scoreClass}">
				{scoreLabel}
			</span>
		{/if}
	</div>
</button>
