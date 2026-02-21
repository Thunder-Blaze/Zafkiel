<script lang="ts">
	import { Dialog, DialogContent } from '$lib/components/ui/dialog';
	import VideoPlayer from './VideoPlayer.svelte';
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
	} = $props<{
		open: boolean;
		src: string;
		title?: string;
		magnet?: string;
		torrentId?: number;
		poster?: string;
	}>();

	let files: TorrentFile[] = $state([]);
	let currentFileId: number | undefined = $state(undefined);
	let showPlaylist = $state(true);
	let loadingFiles = $state(false);
	let currentSrc = $derived(src);
	let currentSubtitleTracks: { id: string; label: string; src: string; lang: string }[] = $state(
		[]
	);

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
			currentSrc = await TorrentService.streamTorrent(magnet, file.id);
		} else if (torrentId !== undefined) {
			currentSrc = await TorrentService.streamTorrentById(torrentId, file.id);
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

<Dialog bind:open>
	<DialogContent
		class="flex aspect-video w-[80vw] max-w-[1200px] flex-row gap-0 overflow-hidden border-none bg-black p-0 shadow-2xl [&>button]:hidden"
	>
		{#if open}
			<div
				class={clsx(
					'relative flex-1 transition-all duration-300',
					showPlaylist && files.length > 1 ? 'w-[75%]' : 'w-full'
				)}
			>
				<VideoPlayer
					src={currentSrc}
					sources={[
						{ src: currentSrc, type: 'video/mp4' },
						{ src: currentSrc, type: 'video/webm' },
						{ src: currentSrc, type: 'video/x-matroska' },
					]}
					tracks={currentSubtitleTracks}
					{title}
					onBack={() => (open = false)}
				/>
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
				<div class="flex h-full w-[20rem] shrink-0 flex-col border-l border-white/10 bg-[#1a1a1a]">
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
