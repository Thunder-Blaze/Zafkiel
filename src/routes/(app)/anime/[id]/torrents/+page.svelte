<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import { useAniZipEpisodes } from '$lib/hooks/useEpisodeMetadata.svelte';
	import {
		episodeTorrentStore,
		type EpisodeDownloadStatus,
	} from '$lib/stores/episodeTorrentStore.svelte';
	import { TorrentService } from '$lib/services/TorrentService';
	import type { AnimeLarge } from '$lib/types/anime';
	import EpisodeRow from '$lib/components/anime/EpisodeRow.svelte';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import TorrentsList from '../anime/TorrentsList.svelte';
	import { toast } from 'svelte-sonner';
	import { onMount, onDestroy } from 'svelte';
	import Icon from '@iconify/svelte';
	import MediaPlayerModal from '$lib/components/player/MediaPlayerModal.svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);

	const episodesQuery = $derived(useAniZipEpisodes(animeId));
	const episodes = $derived(episodesQuery.data ?? []);
	const isLoadingEpisodes = $derived(episodesQuery.isLoading);

	const storeSnapshot = $derived(episodeTorrentStore.snapshot);
	const animeLinks = $derived(storeSnapshot.get(animeId) ?? new Map());

	let torrentStatuses = $state<Map<number, any>>(new Map());
	let pollInterval: any;

	let expandedEpisodeId = $state<number | null>(null);

	function handleToggleExpand(id: number) {
		expandedEpisodeId = expandedEpisodeId === id ? null : id;
	}

	const updateStatuses = async () => {
		if (animeLinks.size === 0) return;

		const torrents = await TorrentService.getTorrents();
		const newStatuses = new Map();

		for (const t of torrents) {
			newStatuses.set(t.id, t);
		}

		torrentStatuses = newStatuses;
	};

	onMount(() => {
		pollInterval = setInterval(updateStatuses, 1000);
		updateStatuses();
	});

	onDestroy(() => {
		clearInterval(pollInterval);
	});

	async function handleDownload(magnetUri: string, episodeNumber: number, title?: string) {
		try {
			// Stream torrent implicitly adds it
			const url = await TorrentService.streamTorrent(magnetUri);
			
			// Open player immediately
			streamUrl = url;
			selectedTorrentId = undefined;
			selectedMagnet = magnetUri;
			selectedEpisodeNumber = episodeNumber;
			playerTitle = title || (animeData?.title?.english || animeData?.title?.romaji) || `Episode ${episodeNumber}`;
			isPlayerOpen = true;

			toast.success(`Started downloading Episode ${episodeNumber}`);

			// Save metadata to DB
			setTimeout(async () => {
				const activeTorrents = await TorrentService.getTorrents();
				// Find by magnet URI (simplified matching)
				const target = activeTorrents.find(t => 
					t.info_hash && magnetUri.toLowerCase().includes(t.info_hash.toLowerCase())
				);
				
				if (target && animeData) {
					await TorrentService.saveTorrentMetadata(target.info_hash, {
						anime_id: animeId,
						episode_number: episodeNumber,
						anime_title: animeData.title?.english || animeData.title?.romaji,
						anime_cover: animeData.coverImage?.extraLarge || animeData.coverImage?.large
					});
				}
				
				// Re-fetch to update linking
				for (const t of activeTorrents) {
					const files = await TorrentService.getTorrentFilesById(t.id);
					episodeTorrentStore.autoLinkFromFiles(animeId, t.id, magnetUri, files);
				}
				updateStatuses();
			}, 2000);
		} catch (e: any) {
			toast.error(`Failed to download: ${e.message || e}`);
		}
	}

	let isPlayerOpen = $state(false);
	let streamUrl = $state('');
	let selectedTorrentId = $state<number | undefined>(undefined);
	let selectedMagnet = $state<string | undefined>(undefined);
	let playerTitle = $state('');
	let selectedEpisodeNumber = $state<number | undefined>(undefined);

	async function handleWatch(torrentId: number, fileId?: number, episodeNumber?: number) {
		try {
			const url = await TorrentService.streamTorrentById(torrentId, fileId);
			streamUrl = url;
			selectedTorrentId = torrentId;
			selectedMagnet = undefined;
			selectedEpisodeNumber = episodeNumber;
			playerTitle = (animeData?.title?.english || animeData?.title?.romaji) ?? 'Watching Episode';
			isPlayerOpen = true;
			toast.success('Opening video player...');
		} catch (e: any) {
			toast.error(`Failed to play: ${e.message || e}`);
		}
	}

	function getStatusForEpisode(episodeNumber: number): EpisodeDownloadStatus | undefined {
		const link = animeLinks.get(episodeNumber);
		if (!link) return undefined;

		const t = torrentStatuses.get(link.torrentId);
		if (!t) return undefined;

		let progress = t.progress; // 0.0 – 100.0
		let canStream = progress >= 5; // allow streaming at 5%

		return {
			episodeNumber,
			torrentId: link.torrentId,
			fileId: link.fileId,
			progress,
			downloadSpeed: t.speed,
			state: t.state.toLowerCase() as any,
			canStream,
		};
	}
	let activeTab = $state('episodes');
	const animeTitle = $derived(animeData?.title?.english || animeData?.title?.romaji || '');
</script>

{#if animeData}
	<div class="mt-4">
		<Tabs bind:value={activeTab} class="w-full">
			<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
				<h2 class="text-2xl font-bold tracking-tight">Downloads</h2>
				<TabsList class="grid w-full grid-cols-2 sm:w-64">
					<TabsTrigger value="episodes">Episodes</TabsTrigger>
					<TabsTrigger value="batches">Batches</TabsTrigger>
				</TabsList>
			</div>

			<TabsContent value="episodes" class="m-0 focus-visible:ring-0 focus-visible:outline-none">
				{#if isLoadingEpisodes}
					<div class="flex flex-col items-center justify-center py-20">
						<Icon
							icon="solar:spinner-bold"
							class="mb-4 size-10 animate-spin text-muted-foreground/50"
						/>
						<h3 class="font-medium text-muted-foreground">Loading episodes...</h3>
					</div>
				{:else if episodes.length === 0}
					<div
						class="flex flex-col items-center justify-center rounded-lg border border-dashed border-border/60 bg-muted/20 p-12 text-center"
					>
						<Icon
							icon="solar:folder-error-bold-duotone"
							class="mb-4 size-12 text-muted-foreground/50"
						/>
						<h3 class="text-lg font-semibold text-foreground">No Episodes Found</h3>
						<p class="mt-1 max-w-sm text-sm text-muted-foreground">
							We couldn't find episode metadata for this anime on AniDB. Try the "Batches" tab
							for manual torrent browsing.
						</p>
					</div>
				{:else}
					<div class="flex flex-col gap-4">
						{#each episodes as episode (episode.number)}
							<EpisodeRow
								{episode}
								{animeTitle}
								downloadStatus={getStatusForEpisode(episode.number)}
								isExpanded={expandedEpisodeId === episode.number}
								onToggleExpand={handleToggleExpand}
								onDownload={handleDownload}
								onWatch={handleWatch}
							/>
						{/each}
					</div>
				{/if}
			</TabsContent>

			<!-- Batches manual list -->
			<TabsContent value="batches" class="m-0 focus-visible:ring-0 focus-visible:outline-none">
				<TorrentsList anime={animeData} mode="batches" enabled={activeTab === 'batches'} />
			</TabsContent>
		</Tabs>
	</div>

	<MediaPlayerModal
		bind:open={isPlayerOpen}
		src={streamUrl}
		torrentId={selectedTorrentId}
		magnet={selectedMagnet}
		title={playerTitle}
		poster={animeData?.coverImage?.extraLarge || animeData?.coverImage?.large}
		episodeNumber={selectedEpisodeNumber}
		{animeId}
	/>
{/if}
