<script lang="ts">
	import { useEpisodeTorrents } from '$lib/hooks/useEpisodeMetadata.svelte';
	import type { EpisodeMeta } from '$lib/services/EpisodeMetadataService';
	import { TorrentService, type PersistTorrent } from '$lib/services/TorrentService';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';

	let { episode, animeTitle, animeId, onDownload } = $props<{
		episode: EpisodeMeta;
		animeTitle: string;
		animeId: number;
		onDownload: (magnetUri: string, title?: string) => void;
	}>();

	const torrentsQuery = $derived(useEpisodeTorrents(animeTitle, episode, 'all'));
	const availableTorrents = $derived(torrentsQuery.data ?? []);
	const isLoading = $derived(torrentsQuery.isLoading);

	let savedTorrents = $state<PersistTorrent[]>([]);
	let isLoadingSaved = $state(true);

	const fetchSaved = async () => {
		isLoadingSaved = true;
		console.log(`[EpisodeTorrentsList] Fetching saved for animeId=${animeId}, ep=${episode.number}`);
		savedTorrents = await TorrentService.getSavedTorrentsForEpisode(animeId, episode.number);
		console.log(`[EpisodeTorrentsList] Found ${savedTorrents.length} saved torrents:`, savedTorrents);
		isLoadingSaved = false;
	};

	onMount(() => {
		fetchSaved();
	});

	// Filter available torrents to exclude saved ones (by info hash if possible, or title)
	const filteredAvailable = $derived(
		availableTorrents.filter(
			(avail) => !savedTorrents.some((saved) => avail.magnetUri.includes(saved.info_hash))
		)
	);
</script>

<div class="flex w-full flex-col">
	{#if isLoading}
		<div
			class="flex w-full items-center justify-center rounded-md border border-border/50 bg-muted/10 p-6 text-muted-foreground"
		>
			<Icon icon="solar:spinner-bold" class="mr-2 size-5 animate-spin" />
			<span class="text-sm font-medium">Fetching parsed releases...</span>
		</div>
	{:else if availableTorrents.length === 0 && savedTorrents.length === 0}
		<div
			class="flex flex-col items-center justify-center rounded-md border border-dashed border-border/80 bg-muted/20 p-6 text-center"
		>
			<Icon icon="solar:file-remove-bold-duotone" class="mb-2 size-8 text-muted-foreground/50" />
			<p class="text-sm font-medium text-foreground">No Parsed Releases Found</p>
			<p class="mt-1 text-xs text-muted-foreground">
				We couldn't find any matching `[Source]` formats for this episode.
			</p>
		</div>
	{:else}
		<!-- A list of matching Torrents -->
		<div
			class="custom-scrollbar grid max-h-[450px] grid-cols-1 gap-4 overflow-y-auto pr-1.5"
			data-lenis-prevent="true"
		>
			{#if savedTorrents.length > 0}
				<div class="space-y-2">
					<h4 class="flex items-center gap-2 text-xs font-bold tracking-wider text-primary uppercase">
						<Icon icon="solar:download-minimalistic-bold" class="size-3.5" />
						Downloaded / Active
					</h4>
					{#each savedTorrents as torrent}
						<div
							class="group flex flex-col justify-between rounded-md border border-primary/30 bg-primary/5 p-3.5 transition-all hover:bg-primary/10 sm:flex-row sm:items-center"
						>
							<div class="mb-3 flex min-w-0 flex-1 flex-col sm:mr-4 sm:mb-0">
								<span class="truncate text-xs font-bold text-foreground">
									{torrent.name || 'Torrent Release'}
								</span>
								<div class="mt-1 flex items-center gap-2 text-[10px] text-muted-foreground">
									<Badge variant="outline" class="h-4 px-1 text-[9px] border-primary/20 text-primary">Saved</Badge>
									<span>{torrent.info_hash.slice(0, 8)}...</span>
								</div>
							</div>
							<div class="flex items-center gap-2">
								<Button
									size="sm"
									variant="default"
									class="h-8 text-xs font-bold"
									onclick={() => onDownload(torrent.magnet, torrent.name)}
								>
									<Icon icon="solar:play-bold" class="mr-1.5 size-3" />
									Play / Resume
								</Button>
							</div>
						</div>
					{/each}
				</div>
			{/if}

			{#if filteredAvailable.length > 0}
				<div class="space-y-2">
					<h4 class="flex items-center gap-2 text-xs font-bold tracking-wider text-muted-foreground uppercase">
						<Icon icon="solar:globus-bold" class="size-3.5" />
						Available Releases
					</h4>
					{#each filteredAvailable as torrent}
						<div
							class="group flex flex-col justify-between rounded-md border border-border/50 bg-background p-3.5 shadow-[0_1px_2px_rgba(0,0,0,0.03)] transition-all hover:border-primary/40 hover:bg-muted/10 sm:flex-row sm:items-center dark:shadow-none"
						>
							<!-- Data -->
							<div class="mb-3 flex min-w-0 flex-1 flex-col sm:mr-4 sm:mb-0">
								<!-- Title -->
								<span
									class="mr-2 truncate text-xs leading-tight font-semibold text-foreground/90"
									title={torrent.title}
								>
									{torrent.title}
								</span>

								<!-- Informational Tags -->
								<div class="mt-2 flex flex-wrap items-center gap-2 text-[11px]">
									{#if torrent.fansub}
										<span
											class="rounded-sm bg-primary/10 px-1.5 py-0.5 font-bold tracking-wide text-primary"
										>
											{torrent.fansub}
										</span>
									{/if}

									{#if torrent.resolution}
										<span
											class="rounded-sm bg-secondary/80 px-1.5 py-0.5 font-semibold text-secondary-foreground"
										>
											{torrent.resolution}
										</span>
									{/if}

									{#if torrent.languages && torrent.languages.length > 0}
										{#each torrent.languages as lang}
											<span
												class="box-border rounded-sm bg-amber-500/10 px-1.5 py-0.5 font-bold text-amber-600 dark:text-amber-400"
											>
												{lang}
											</span>
										{/each}
									{/if}

									<div class="relative mx-0.5 h-3 w-[1.5px] rounded-full bg-border"></div>

									<!-- Storage -->
									<span
										class="flex items-center gap-1 font-medium text-muted-foreground"
										title="File Size"
									>
										<Icon icon="solar:folder-bold" class="size-3" />
										{torrent.size}
									</span>

									<div class="relative mx-0.5 h-3 w-[1.5px] rounded-full bg-border"></div>

									<!-- Analytics -->
									<span
										class="flex items-center gap-1 font-bold text-emerald-600 dark:text-emerald-500"
										title="Seeders"
									>
										<Icon icon="solar:arrow-up-bold" class="size-3" />
										{torrent.seeds}
									</span>

									<span
										class="flex items-center gap-1 font-semibold text-rose-600 dark:text-rose-500"
										title="Leechers"
									>
										<Icon icon="solar:arrow-down-bold" class="size-3" />
										{torrent.peers}
									</span>
								</div>
							</div>

							<!-- Direct Downloads & Copy -->
							<div class="mt-3 flex w-full shrink-0 items-center gap-2 sm:mt-0 sm:w-auto">
								<Button
									size="icon"
									variant="outline"
									class="size-8 shrink-0 text-muted-foreground hover:text-foreground"
									title="Copy Magnet Link"
									onclick={(e) => {
										e.stopPropagation();
										navigator.clipboard.writeText(torrent.magnetUri);
										toast.success('Magnet link copied!');
									}}
								>
									<Icon icon="solar:copy-outline" class="size-4" />
								</Button>

								<Button
									size="sm"
									variant="secondary"
									class="w-full text-xs font-bold shadow-sm transition-transform active:scale-95 sm:w-auto"
									onclick={(e) => {
										e.stopPropagation();
										onDownload(torrent.magnetUri, torrent.title);
									}}
								>
									<Icon icon="solar:download-square-linear" class="mr-1.5 size-3.5" />
									Download
								</Button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background-color: hsl(var(--muted));
		border-radius: 10px;
	}
	.custom-scrollbar:hover::-webkit-scrollbar-thumb {
		background-color: hsl(var(--primary) / 0.5);
	}
</style>
