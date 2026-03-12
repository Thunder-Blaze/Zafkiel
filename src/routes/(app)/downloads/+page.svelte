<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '$lib/components/ui/button';
	import { Progress } from '$lib/components/ui/progress';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';

	import MediaPlayerModal from '$lib/components/player/MediaPlayerModal.svelte';

	import { TorrentService, type Torrent as TorrentState } from '$lib/services/TorrentService'; // Renamed interface in Service
	// Wait, I renamed TorrentState to Torrent in the service.
	// I should check if I broke that too.
	// In Step 1516 (TorrentService.ts write), I exported `Torrent` and `TorrentFile`.
	// I did NOT export `TorrentState`.
	// So I need to alias `Torrent` to `TorrentState` here or update usages.
	// I'll alias it for minimal diff.

	let torrents = $state<TorrentState[]>([]);
	let interval: any;
	let isPlayerOpen = $state(false);
	let playerModalMode = $state<'internal' | 'libmpv'>('libmpv');
	let streamUrl = $state('');
	let selectedTorrent = $state<TorrentState | null>(null);

	async function fetchTorrents() {
		try {
			torrents = await TorrentService.getTorrents();
		} catch (error) {
			console.error('Failed to fetch torrents:', error);
		}
	}

	onMount(() => {
		fetchTorrents();
		interval = setInterval(fetchTorrents, 1000);
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});

	async function pauseTorrent(id: number) {
		try {
			await TorrentService.pauseTorrent(id);
			toast.success('Torrent paused');
			fetchTorrents();
		} catch (error) {
			toast.error('Failed to pause torrent: ' + error);
		}
	}

	async function resumeTorrent(id: number) {
		try {
			await TorrentService.resumeTorrent(id);
			toast.success('Torrent resumed');
			fetchTorrents();
		} catch (error) {
			toast.error('Failed to resume torrent: ' + error);
		}
	}

	async function deleteTorrent(id: number, deleteFiles: boolean) {
		try {
			await TorrentService.deleteTorrent(id, deleteFiles);
			toast.success('Torrent deleted');
			fetchTorrents();
		} catch (error) {
			toast.error('Failed to delete torrent: ' + error);
		}
	}

	async function onPlay(torrent: TorrentState, mode: 'internal' | 'libmpv' = 'libmpv') {
		try {
			selectedTorrent = torrent;
			playerModalMode = mode;
			toast.info('Starting stream...');
			const url = await TorrentService.streamTorrentById(torrent.id);
			console.log('Stream URL:', url);

			streamUrl = url;
			isPlayerOpen = true;
			toast.success('Stream started');
		} catch (error) {
			console.error('Failed to start stream:', error);
			toast.error('Failed to start stream: ' + error);
		}
	}

	// Filter torrents
	let activeTorrents = $derived(
		torrents.filter((t) => t.state === 'downloading' || t.state === 'starting' || t.progress < 100)
	);
	let completedTorrents = $derived(
		torrents.filter((t) => t.progress >= 100 || t.state === 'seeding')
	);

	function formatBytes(bytes: number, decimals = 2) {
		if (!+bytes) return '0 B';
		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
	}

	function cleanName(name: string | null | undefined) {
		if (!name) return 'Unknown Torrent';
		return name
			.replace(/^\[.*?\]\s*/, '') // Remove leading [Group]
			.replace(/\s*\(.*?\)$/, '') // Remove trailing (Info)
			.replace(/\s*\[.*?\]$/, '') // Remove trailing [Info]
			.replace(/\.mkv$|\.mp4$/i, ''); // Remove extension
	}
</script>

<MediaPlayerModal
	bind:open={isPlayerOpen}
	src={streamUrl}
	torrentId={selectedTorrent?.id}
	title={selectedTorrent ? cleanName(selectedTorrent.name) : 'Unknown'}
	mode={playerModalMode}
/>

<div class="container mx-auto space-y-8 p-6">
	<h1 class="text-3xl font-bold">Downloads</h1>

	{#if torrents.length === 0}
		<div
			class="flex h-64 flex-col items-center justify-center rounded-lg border border-dashed text-muted-foreground"
		>
			<Icon icon="solar:box-minimalistic-bold-duotone" class="mb-4 h-12 w-12" />
			<p>No active downloads</p>
		</div>
	{:else}
		{#snippet torrentList(title: string, list: TorrentState[])}
			{#if list.length > 0}
				<div class="space-y-4">
					<h2 class="text-xl font-semibold text-muted-foreground">{title} ({list.length})</h2>
					<div class="grid gap-4">
						{#each list as torrent (torrent.id)}
							<div
								class="relative overflow-hidden rounded-xl border bg-card/50 p-4 shadow-sm backdrop-blur-sm transition-all hover:bg-card hover:shadow-md"
							>
								<!-- Background Progress for Active Downloads -->
								{#if torrent.progress < 100}
									<div
										class="absolute top-0 bottom-0 left-0 bg-primary/5 transition-all duration-500 ease-out"
										style="width: {torrent.progress}%"
									></div>
								{/if}

								<div
									class="relative z-10 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
								>
									<div class="min-w-0 flex-1 space-y-2">
										<div class="flex items-center gap-2">
											<div
												class="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10 text-primary"
											>
												{#if torrent.state === 'paused'}
													<Icon icon="solar:pause-bold" class="h-4 w-4" />
												{:else if torrent.progress >= 100}
													<Icon icon="solar:check-circle-bold" class="h-4 w-4" />
												{:else}
													<Icon
														icon="solar:download-minimalistic-bold"
														class="h-4 w-4 animate-pulse"
													/>
												{/if}
											</div>
											<h3 class="truncate text-lg font-semibold" title={torrent.name || 'Unknown'}>
												{cleanName(torrent.name)}
											</h3>
										</div>

										<div class="flex flex-wrap gap-4 text-xs font-medium text-muted-foreground">
											<span
												class="flex items-center gap-1.5 rounded-full bg-secondary/50 px-2 py-0.5"
											>
												<Icon icon="solar:download-bold" class="text-green-500" />
												{formatBytes(torrent.speed)}/s
											</span>
											<span
												class="flex items-center gap-1.5 rounded-full bg-secondary/50 px-2 py-0.5"
											>
												<Icon icon="solar:upload-bold" class="text-blue-500" />
												{formatBytes(torrent.upload_speed)}/s
											</span>
											<span
												class="flex items-center gap-1.5 rounded-full bg-secondary/50 px-2 py-0.5"
											>
												<Icon icon="solar:users-group-rounded-bold" />
												{torrent.peers} peers
											</span>
											<span
												class="flex items-center gap-1.5 rounded-full bg-secondary/50 px-2 py-0.5"
											>
												<Icon icon="solar:database-bold" />
												{formatBytes(torrent.downloaded)} / {formatBytes(torrent.total_size)}
											</span>
											<span
												class="flex items-center gap-1.5 rounded-full bg-secondary/50 px-2 py-0.5 uppercase"
											>
												{torrent.state}
											</span>
										</div>
									</div>

									<div class="flex items-center gap-2 self-end sm:self-auto">
										{#if torrent.state === 'downloading' || torrent.state === 'seeding' || torrent.state === 'starting' || torrent.state === 'paused'}
											<Button
												variant="outline"
												size="icon"
												class="h-9 w-9 rounded-full shadow-sm"
												onclick={() => onPlay(torrent, 'internal')}
												title="Play in Browser (hls.js)"
											>
												<Icon icon="solar:monitor-smartphone-bold-duotone" class="h-4 w-4" />
											</Button>

											<Button
												variant="default"
												size="icon"
												class="h-9 w-9 rounded-full shadow-sm"
												onclick={() => onPlay(torrent, 'libmpv')}
												title="Play in Libmpv"
											>
												<Icon icon="solar:play-circle-bold-duotone" class="h-4 w-4" />
											</Button>

											<Button
												variant="secondary"
												size="icon"
												class="h-9 w-9 rounded-full shadow-sm"
												onclick={async () => {
													try {
														const url = await TorrentService.streamTorrentById(torrent.id);
														await TorrentService.openInExternalPlayer(url);
														toast.success('Opened in external player');
													} catch (e) {
														toast.error('Failed to open external player: ' + e);
													}
												}}
												title="Open in external player (mpv)"
											>
												<Icon icon="solar:export-bold-duotone" class="h-4 w-4" />
											</Button>
										{/if}

										{#if torrent.state === 'paused'}
											<Button
												variant="outline"
												size="icon"
												class="h-9 w-9 rounded-full"
												onclick={() => resumeTorrent(torrent.id)}
												title="Resume"
											>
												<Icon icon="lucide:play" class="h-4 w-4" />
											</Button>
										{:else}
											<Button
												variant="outline"
												size="icon"
												class="h-9 w-9 rounded-full"
												onclick={() => pauseTorrent(torrent.id)}
												title="Pause"
											>
												<Icon icon="lucide:pause" class="h-4 w-4" />
											</Button>
										{/if}

										<Button
											variant="ghost"
											size="icon"
											class="h-9 w-9 rounded-full text-destructive hover:bg-destructive/10 hover:text-destructive"
											onclick={() => deleteTorrent(torrent.id, true)}
											title="Delete"
										>
											<Icon icon="lucide:trash-2" class="h-4 w-4" />
										</Button>
									</div>
								</div>

								{#if torrent.progress < 100}
									<div class="relative mt-3 h-1 w-full overflow-hidden rounded-full bg-primary/20">
										<div
											class="absolute inset-y-0 left-0 bg-primary transition-all duration-300"
											style="width: {torrent.progress}%"
										></div>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/snippet}

		{@render torrentList('Active Downloads', activeTorrents)}
		{@render torrentList('Completed', completedTorrents)}
	{/if}
</div>
