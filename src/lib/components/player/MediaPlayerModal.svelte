<script lang="ts">
	import { Dialog, DialogContent } from '$lib/components/ui/dialog';
	import VideoPlayer from './VideoPlayer.svelte';
	import InternalPlayer from './InternalPlayer.svelte';
	import { TorrentService, type TorrentFile } from '$lib/services/TorrentService';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import clsx from 'clsx';
	import { ScrollArea } from '$lib/components/ui/scroll-area';

	let {
		open = $bindable(false),
		src,
		title,
		magnet,
		torrentId,
		poster,
		mode,
		episodeNumber,
		animeId,
	} = $props<{
		open: boolean;
		src: string;
		title?: string;
		magnet?: string;
		torrentId?: number;
		poster?: string;
		mode?: 'internal' | 'libmpv';
		episodeNumber?: number;
		animeId?: number;
	}>();

	const mockEpisode = $derived(episodeNumber !== undefined ? { number: episodeNumber, id: 'torrent-ep' } : null);

	let files: TorrentFile[] = $state([]);
	let currentFileId: number | undefined = $state(undefined);
	let showPlaylist = $state(true);
	let loadingFiles = $state(false);
	let currentSrc = $state(src);
	let currentSubtitleTracks: { id: string; label: string; src: string; lang: string }[] = $state(
		[]
	);

	type PlayerMode = 'internal' | 'libmpv';
	// Initialise from the prop (set by the caller's button choice), falling back
	// to the last-used value persisted in localStorage.
	let playerMode = $state<PlayerMode>(
		(localStorage.getItem('zafkiel-player-mode') as PlayerMode | null) ?? 'libmpv'
	);

	// When the caller changes the `mode` prop (e.g. re-opening with a different button),
	// sync it into local state.
	$effect(() => {
		if (mode) playerMode = mode;
	});

	// Persist whenever the user switches inside the modal.
	$effect(() => {
		localStorage.setItem('zafkiel-player-mode', playerMode);
	});

	async function loadFiles() {
		if (!magnet && torrentId === undefined) return;
		loadingFiles = true;

		if (magnet) {
			files = await TorrentService.getTorrentFiles(magnet);
		} else if (torrentId !== undefined) {
			files = await TorrentService.getTorrentFilesById(torrentId);
		}

		loadingFiles = false;
	}

	async function selectFile(file: TorrentFile) {
		currentFileId = file.id;

		// Get new stream URL
		if (magnet) {
			const { url } = await TorrentService.streamTorrent(magnet, file.id);
			currentSrc = url;
		} else if (torrentId !== undefined) {
			const { url } = await TorrentService.streamTorrentById(torrentId, file.id);
			currentSrc = url;
		}

		// Reset subtitles for new file
		currentSubtitleTracks = [];
	}

	$effect(() => {
		if (open && magnet) {
			loadFiles();
		}
	});

	// Reset src when prop changes
	$effect(() => {
		currentSrc = src;
	});

	function formatSize(bytes: number) {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}
</script>

{#if open && playerMode === 'libmpv'}
	<!-- libmpv uses a fixed full-screen transparent overlay over the native
	     video layer. Wrapping it in a Dialog breaks that by constraining the
	     viewport hole — render it directly at the top level instead. -->
	<VideoPlayer 
		url={currentSrc} 
		{title} 
		image={poster}
		currentEpisode={mockEpisode}
		{animeId}
		onBack={() => (open = false)} 
	/>
	<!-- Mode switcher floats above the VideoPlayer's z-[100] overlay -->
	<div
		class="fixed top-3 left-1/2 z-[101] flex -translate-x-1/2 overflow-hidden rounded-full border border-white/20 bg-black/60 backdrop-blur-sm"
	>
		<button
			class="flex items-center gap-1.5 px-3 py-1.5 text-xs text-white/50 transition-colors hover:text-white"
			onclick={() => (playerMode = 'internal')}
			title="Browser (hls.js)"
		>
			<Icon icon="solar:monitor-smartphone-bold-duotone" class="size-3.5" />
			Browser
		</button>
		<div class="w-px bg-white/20"></div>
		<button
			class="flex items-center gap-1.5 bg-white/20 px-3 py-1.5 text-xs text-white transition-colors"
			onclick={() => (playerMode = 'libmpv')}
			title="Libmpv (hardware-accelerated)"
		>
			<Icon icon="solar:play-circle-bold-duotone" class="size-3.5" />
			Libmpv
		</button>
	</div>
{/if}

{#if playerMode === 'internal'}
	<Dialog
		bind:open
		onOpenChange={(v) => {
			if (!v) open = false;
		}}
	>
		<DialogContent
			class="flex h-[90vh] w-[90vw] max-w-none flex-row gap-0 overflow-hidden rounded-xl border-none bg-black p-0 shadow-2xl sm:max-w-none [&>button]:hidden"
		>
			{#if open && playerMode === 'internal'}
				<div
					class={clsx(
						'relative flex-1 transition-all duration-300',
						showPlaylist && files.length > 1 ? 'w-[75%]' : 'w-full'
					)}
				>
					<InternalPlayer 
						src={currentSrc} 
						{title} 
						image={poster}
						currentEpisode={mockEpisode}
						{animeId}
						onBack={() => (open = false)} 
					/>

					<!-- Player mode switcher overlay -->
					<div
						class="absolute top-3 left-1/2 z-50 flex -translate-x-1/2 overflow-hidden rounded-full border border-white/20 bg-black/60 backdrop-blur-sm"
					>
						<button
							class="flex items-center gap-1.5 bg-white/20 px-3 py-1.5 text-xs text-white transition-colors"
							onclick={() => (playerMode = 'internal')}
							title="Browser (hls.js)"
						>
							<Icon icon="solar:monitor-smartphone-bold-duotone" class="size-3.5" />
							Browser
						</button>
						<div class="w-px bg-white/20"></div>
						<button
							class="flex items-center gap-1.5 px-3 py-1.5 text-xs text-white/50 transition-colors hover:text-white"
							onclick={() => (playerMode = 'libmpv')}
							title="Libmpv (hardware-accelerated)"
						>
							<Icon icon="solar:play-circle-bold-duotone" class="size-3.5" />
							Libmpv
						</button>
					</div>
					{#if files.length > 1}
						<Button
							variant="ghost"
							size="icon"
							class="absolute top-4 right-4 z-50 bg-black/20 text-white/50 hover:bg-black/40 hover:text-white"
							onclick={() => (showPlaylist = !showPlaylist)}
						>
							<Icon icon="lucide:list-video" class="h-6 w-6" />
						</Button>
					{/if}
				</div>

				{#if showPlaylist && files.length > 1}
					<div
						class="flex h-full w-[20rem] shrink-0 flex-col border-l border-white/10 bg-[#1a1a1a]"
					>
						<div class="border-b border-white/10 p-4">
							<h3 class="mb-1 font-bold text-white">Playlist</h3>
							<p class="text-xs text-white/50">{files.length} Files</p>
						</div>
						<ScrollArea class="flex-1">
							<div class="flex flex-col gap-1 p-2">
								{#each files as file}
									<button
										class={clsx(
											'flex w-full flex-col items-start rounded p-3 text-left transition-colors',
											currentFileId === file.id
												? 'bg-white/10 text-white'
												: 'text-white/60 hover:bg-white/5 hover:text-white'
										)}
										onclick={() => selectFile(file)}
									>
										<span class="line-clamp-2 w-full text-sm font-medium">{file.name}</span>
										<span class="mt-1 text-xs opacity-50">{formatSize(file.size)}</span>
									</button>
								{/each}
							</div>
						</ScrollArea>
					</div>
				{/if}
			{/if}
		</DialogContent>
	</Dialog>
{/if}
