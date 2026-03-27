<script lang="ts">
	import type { EpisodeMeta } from '$lib/services/EpisodeMetadataService';
	import type { EpisodeDownloadStatus } from '$lib/stores/episodeTorrentStore.svelte';
	import EpisodeTorrentsList from './EpisodeTorrentsList.svelte';
	import EpisodeDownloadProgress from './EpisodeDownloadProgress.svelte';
	import Icon from '@iconify/svelte';
	import { formatTime } from '$lib/utils/data-filters';
	import { slide } from 'svelte/transition';

	let {
		episode,
		animeTitle,
		animeId,
		downloadStatus,
		isExpanded = false,
		onToggleExpand,
		onDownload,
		onWatch,
	} = $props<{
		episode: EpisodeMeta;
		animeTitle: string;
		animeId: number;
		downloadStatus: EpisodeDownloadStatus | undefined;
		isExpanded?: boolean;
		onToggleExpand?: (id: number) => void;
		onDownload: (magnetUri: string, episodeNumber: number, title?: string) => void;
		onWatch: (torrentId: number, fileId?: number, episodeNumber?: number) => void;
	}>();

	const isFutureEpisode = $derived(
		episode.airDate && new Date(episode.airDate).getTime() > Date.now()
	);

	function getFormatAirDate(dateStr: string | undefined) {
		if (!dateStr) return '';
		const d = new Date(dateStr);
		return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' });
	}

	const displayDate = $derived(getFormatAirDate(episode.airDate));
</script>

<div
	class="group relative flex w-full flex-col rounded-lg border bg-card transition-all focus-within:bg-accent/10 focus-within:shadow-sm hover:bg-accent/10 hover:shadow-sm {onToggleExpand
		? 'cursor-pointer'
		: ''}"
>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="flex w-full flex-col gap-4 p-4 sm:flex-row sm:items-center sm:gap-6"
		onclick={(e) => {
			if ((e.target as HTMLElement).closest('button')) return;
			onToggleExpand?.(episode.number);
		}}
	>
		<div class="flex min-w-0 flex-1 items-center gap-6">
			<div class="relative h-20 w-36 shrink-0 overflow-hidden rounded-md bg-muted shadow-sm">
				{#if episode.thumbnailUrl}
					<img
						src={episode.thumbnailUrl}
						alt={episode.title}
						class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
						loading="lazy"
					/>
				{:else}
					<div class="flex h-full w-full items-center justify-center bg-secondary">
						<Icon
							icon="solar:play-circle-bold-duotone"
							class="size-8 text-secondary-foreground/20"
						/>
					</div>
				{/if}
				{#if isFutureEpisode}
					<div
						class="absolute inset-0 flex items-center justify-center bg-background/60 backdrop-blur-[2px]"
					>
						<span
							class="rounded bg-background/80 px-2 py-1 text-xs font-bold tracking-wider text-muted-foreground uppercase shadow-sm"
						>
							Unaired
						</span>
					</div>
				{/if}
			</div>

			<div class="flex min-w-0 flex-col">
				<span class="mb-1 text-xs font-bold tracking-wider text-primary/80 uppercase">
					Episode {episode.number}
				</span>
				<h4
					class="mb-1 truncate text-base leading-tight font-semibold text-foreground"
					title={episode.title}
				>
					{episode.title}
				</h4>
				{#if displayDate}
					<span class="text-xs text-muted-foreground">
						{displayDate}
					</span>
				{/if}
			</div>
		</div>

		<div class="flex h-11 w-full shrink-0 items-center gap-3 sm:w-auto">
			{#if downloadStatus}
				<EpisodeDownloadProgress
					progress={downloadStatus.progress}
					downloadSpeed={downloadStatus.downloadSpeed}
					state={downloadStatus.state}
				/>

				{#if downloadStatus.canStream}
					<button
						class="flex h-full items-center gap-1.5 rounded-md bg-primary px-4 text-sm font-bold text-primary-foreground shadow-sm transition-all hover:-translate-y-0.5 active:scale-95 disabled:pointer-events-none disabled:opacity-50"
						onclick={() => onWatch(downloadStatus.torrentId, downloadStatus.fileId, episode.number)}
					>
						<Icon icon="solar:play-bold" class="size-4" />
						Watch
					</button>
				{/if}
			{:else if !isFutureEpisode}
				{#if onToggleExpand}
					<div
						class="ml-1 flex items-center justify-center rounded-full bg-transparent p-2 text-muted-foreground transition-all {isExpanded
							? 'rotate-180 text-primary'
							: ''}"
					>
						<Icon icon="solar:alt-arrow-down-linear" class="size-5" />
					</div>
				{/if}
			{/if}
		</div>
	</div>

	<!-- Virtual List underneath -->
	{#if isExpanded}
		<div
			class="mt-3 w-full border-t border-border/30 pt-4 pl-2"
			transition:slide={{ duration: 250 }}
		>
			<EpisodeTorrentsList {episode} {animeTitle} {animeId} onDownload={(magnet, title) => onDownload(magnet, episode.number, title)} />
		</div>
	{/if}
</div>
