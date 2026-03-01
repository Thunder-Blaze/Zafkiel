<script lang="ts">
	import { goto } from '$app/navigation';
	import { Badge } from '$lib/components/ui/badge';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import type { MediaList } from '$lib/types/anilist';
	import { formatProgress, formatScore, scoreToColorClass } from '$lib/utils/format';

	let { entry }: { entry: MediaList } = $props();

	const media = $derived(entry.media);
	const title = $derived(media?.title?.userPreferred ?? media?.title?.english ?? 'Unknown Title');
	const cover = $derived(media?.coverImage?.medium ?? media?.coverImage?.large ?? '');
	const href = $derived(media ? `/anime/${media.id}` : '#');
	const total = $derived(media?.type === 'MANGA' ? media?.chapters : media?.episodes);

	const STATUS_LABELS: Record<string, string> = {
		CURRENT: 'Watching',
		COMPLETED: 'Completed',
		PLANNING: 'Planning',
		PAUSED: 'Paused',
		DROPPED: 'Dropped',
		REPEATING: 'Rewatching',
	};

	const STATUS_VARIANTS: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
		CURRENT: 'default',
		COMPLETED: 'secondary',
		PLANNING: 'outline',
		PAUSED: 'outline',
		DROPPED: 'destructive',
		REPEATING: 'default',
	};

	const statusLabel = $derived(entry.status ? (STATUS_LABELS[entry.status] ?? entry.status) : '—');
	const statusVariant = $derived(
		entry.status ? (STATUS_VARIANTS[entry.status] ?? 'outline') : 'outline',
	);

	const scoreClass = $derived(scoreToColorClass(entry.score ? entry.score * 10 : null));
	const progressStr = $derived(formatProgress(entry.progress, total));
</script>

<div
	class="group flex cursor-pointer items-center gap-3 border-b px-3 py-2.5 transition-colors last:border-b-0 hover:bg-muted/40"
	role="row"
	tabindex="0"
	onkeydown={(e) => e.key === 'Enter' && goto(href)}
	onclick={() => goto(href)}
>
	<!-- Cover -->
	{#if cover}
		<CachedImage
			src={cover}
			alt={title}
			class="size-10 shrink-0 rounded object-cover"
		/>
	{:else}
		<div class="size-10 shrink-0 rounded bg-muted"></div>
	{/if}

	<!-- Title -->
	<div class="min-w-0 flex-1">
		<p
			class="truncate text-sm font-medium leading-tight group-hover:text-primary group-hover:underline"
		>
			{title}
		</p>
		{#if media?.format}
			<p class="mt-0.5 truncate text-xs text-muted-foreground">{media.format.replace('_', ' ')}</p>
		{/if}
	</div>

	<!-- Status badge -->
	<Badge variant={statusVariant} class="shrink-0 text-xs">
		{statusLabel}
	</Badge>

	<!-- Score -->
	<span class="w-10 shrink-0 text-right text-sm font-semibold tabular-nums {scoreClass}">
		{entry.score ? formatScore(entry.score) : '—'}
	</span>

	<!-- Progress -->
	<span class="w-16 shrink-0 text-right text-xs text-muted-foreground tabular-nums">
		{progressStr}
	</span>
</div>
