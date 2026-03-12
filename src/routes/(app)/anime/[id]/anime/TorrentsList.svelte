<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { ExtensionManager } from '$lib/services/ExtensionManager';
	import type { AnimeLarge } from '$lib/types/anime';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { toast } from 'svelte-sonner';

	import MediaPlayerModal from '$lib/components/player/MediaPlayerModal.svelte';

	import type { TorrentInfo } from '$lib/services/ExtensionManager';

	import { TorrentService } from '$lib/services/TorrentService';
	import { invoke } from '@tauri-apps/api/core';

	let { anime }: { anime: AnimeLarge } = $props();
	let selectedTorrent = $state<TorrentInfo | null>(null);
	let isPlayerOpen = $state(false);
	let playerModalMode = $state<'internal' | 'libmpv'>('libmpv');
	let streamUrl = $state('');

	const torrentsQuery = createQuery(() => ({
		queryKey: ['torrents', anime.id],
		queryFn: async () => {
			if (!anime) return [];
			return await ExtensionManager.searchAnimeAll(anime);
		},
		enabled: !!anime,
		staleTime: 1000 * 60 * 5, // 5 minutes
	}));

	const torrents = $derived(torrentsQuery.data || []);
	const isLoading = $derived(torrentsQuery.isLoading);
	const error = $derived(torrentsQuery.error);

	function copyMagnet(magnet: string) {
		navigator.clipboard.writeText(magnet);
		toast.success('Magnet link copied to clipboard');
	}

	function openMagnet(magnet: string) {
		window.open(magnet, '_self');
	}

	async function onPlay(torrent: TorrentInfo, mode: 'internal' | 'libmpv' = 'libmpv') {
		try {
			selectedTorrent = torrent;
			playerModalMode = mode;
			toast.info('Starting stream...');
			// Start streaming via Rust backend
			const url = await TorrentService.streamTorrent(torrent.magnet);
			console.log('Stream URL:', url);

			// Set the stream URL for the player
			streamUrl = url;
			isPlayerOpen = true;
			toast.success('Stream started');
		} catch (error) {
			console.error('Failed to start stream:', error);
			toast.error('Failed to start stream: ' + error);
		}
	}

	async function onPlayExternal(torrent: TorrentInfo) {
		try {
			toast.info('Starting stream...');
			const url = await TorrentService.streamTorrent(torrent.magnet);
			await TorrentService.openInExternalPlayer(url);
			toast.success('Opened in external player');
		} catch (error) {
			console.error('Failed to open external player:', error);
			toast.error('Failed to open external player: ' + error);
		}
	}
</script>

<MediaPlayerModal
	bind:open={isPlayerOpen}
	src={streamUrl}
	magnet={selectedTorrent?.magnet}
	title={selectedTorrent?.title}
	poster={anime.coverImage?.extraLarge || anime.coverImage?.large}
	mode={playerModalMode}
/>

<div class="space-y-4">
	<div class="flex items-center justify-between">
		<h3 class="text-lg font-semibold">Available Torrents</h3>
		<Button
			variant="outline"
			size="sm"
			onclick={() => torrentsQuery.refetch()}
			disabled={isLoading}
		>
			<Icon icon="solar:refresh-bold" class={isLoading ? 'animate-spin' : ''} />
			Refresh
		</Button>
	</div>

	{#if isLoading}
		<div class="flex min-h-[200px] items-center justify-center">
			<div class="flex flex-col items-center space-y-4">
				<Icon
					icon="solar:refresh-circle-line-duotone"
					class="h-10 w-10 animate-spin text-primary"
				/>
				<p class="text-muted-foreground">Searching torrents...</p>
			</div>
		</div>
	{:else if error}
		<div class="rounded-lg border border-destructive bg-destructive/10 p-6 text-center">
			<Icon
				icon="solar:danger-circle-bold-duotone"
				class="mx-auto mb-2 h-10 w-10 text-destructive"
			/>
			<p class="text-destructive">Failed to load torrents</p>
			<p class="text-sm text-muted-foreground">{error.message}</p>
		</div>
	{:else if torrents.length === 0}
		<div class="rounded-lg border border-dashed p-8 text-center">
			<Icon
				icon="solar:box-minimalistic-bold-duotone"
				class="mx-auto mb-2 h-10 w-10 text-muted-foreground"
			/>
			<p class="text-muted-foreground">No torrents found for this anime.</p>
		</div>
	{:else}
		<div class="overflow-hidden rounded-lg border border-border bg-card">
			<div class="overflow-x-auto">
				<table class="w-full text-sm">
					<thead class="bg-muted/50 text-left text-xs font-medium text-muted-foreground uppercase">
						<tr>
							<th class="px-4 py-3">Release</th>
							<th class="px-4 py-3 text-center">Size</th>
							<th class="px-4 py-3 text-center">Seeds</th>
							<th class="px-4 py-3 text-center">Peers</th>
							<th class="px-4 py-3 text-right">Action</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-border">
						{#each torrents as torrent}
							<tr class="group hover:bg-muted/30">
								<td class="px-4 py-3">
									<div class="flex flex-col gap-1">
										<span class="line-clamp-2 font-medium" title={torrent.title}
											>{torrent.title}</span
										>
										<div class="flex items-center gap-2 text-xs text-muted-foreground">
											<Badge variant="outline" class="h-5 px-1.5 text-[10px]"
												>{torrent.provider}</Badge
											>
											<span>{torrent.uploadedAt || 'Unknown date'}</span>
										</div>
									</div>
								</td>
								<td class="px-4 py-3 text-center whitespace-nowrap">{torrent.size}</td>
								<td class="px-4 py-3 text-center font-medium text-green-500">{torrent.seeds}</td>
								<td class="px-4 py-3 text-center text-muted-foreground">{torrent.peers}</td>
								<td class="px-4 py-3 text-right">
									<div class="flex justify-end gap-1.5">
										<Button
											variant="ghost"
											size="icon"
											class="h-8 w-8"
											onclick={() => copyMagnet(torrent.magnet)}
											title="Copy Magnet"
										>
											<Icon icon="solar:copy-bold-duotone" class="h-4 w-4" />
										</Button>
										<Button
											variant="secondary"
											size="icon"
											class="h-8 w-8 text-primary"
											onclick={() => openMagnet(torrent.magnet)}
											title="Download with torrent client"
										>
											<Icon icon="solar:download-bold-duotone" class="h-4 w-4" />
										</Button>
										<Button
											variant="outline"
											size="icon"
											class="h-8 w-8"
											onclick={() => onPlay(torrent, 'internal')}
											title="Play in Browser (hls.js)"
										>
											<Icon icon="solar:monitor-smartphone-bold-duotone" class="h-4 w-4" />
										</Button>
										<Button
											variant="default"
											size="icon"
											class="h-8 w-8"
											onclick={() => onPlay(torrent, 'libmpv')}
											title="Play in Libmpv"
										>
											<Icon icon="solar:play-circle-bold-duotone" class="h-4 w-4" />
										</Button>
										<Button
											variant="secondary"
											size="icon"
											class="h-8 w-8"
											onclick={() => onPlayExternal(torrent)}
											title="Open in external player (mpv)"
										>
											<Icon icon="solar:export-bold-duotone" class="h-4 w-4" />
										</Button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{/if}
</div>
