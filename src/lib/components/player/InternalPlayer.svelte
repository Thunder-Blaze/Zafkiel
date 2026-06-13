<script lang="ts">
	/**
	 * InternalPlayer — Browser-native HTML5 video player.
	 *
	 * Uses hls.js to transmux HLS streams into fMP4 segments for the browser's
	 * MSE API. This does not require libmpv and works entirely in the WebView.
	 *
	 * Suitable as a fallback on platforms where libmpv is unavailable or
	 * behaviorally unexpected (Linux/macOS native window layering issues).
	 */
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import PlayerControls from './PlayerControls.svelte';
	import SyncPreferenceModal from './SyncPreferenceModal.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import type { Episode, StreamSource } from '$lib/types/extensions';
	import Hls from 'hls.js';
	import { discordStore } from '$lib/stores/discord.svelte';
	import { useConfigState } from '$lib/stores/config.svelte';

	let {
		src,
		headers = {},
		mediaSources = [],
		episodes = [],
		currentEpisode = null,
		sources = [],
		currentSource = null,
		onEpisodeSelect,
		onSourceSelect,
		tracks = [],
		title,
		subtitle,
		image,
		onBack,
		animeId,
	} = $props<{
		src?: string;
		/** HTTP request headers required by the stream (e.g. Referer, Cookie). */
		headers?: Record<string, string>;
		mediaSources?: { src: string; type: string }[];
		episodes?: Episode[];
		currentEpisode?: Episode | null;
		sources?: StreamSource[];
		currentSource?: StreamSource | null;
		onEpisodeSelect?: (ep: Episode) => void;
		onSourceSelect?: (src: StreamSource) => void;
		tracks?: { id: string; label: string; src: string; lang: string }[];
		title?: string;
		subtitle?: string;
		image?: string;
		onBack?: () => void;
		animeId?: number;
	}>();

	let videoElement: HTMLVideoElement;
	let containerElement: HTMLDivElement;
	let isPlaying = $state(false);
	let currentTime = $state(0);
	let duration = $state(0);
	let volume = $state(1);
	let showControls = $state(false);
	let isLocked = $state(false);
	let showSkipIntro = $state(false);
	let controlsTimeout: ReturnType<typeof setTimeout> | undefined;
	let currentTrackIndex = $state(-1); // -1 = off

	let isBuffering = $state(false);
	let hasError = $state(false);
	let errorMessage = $state('');
	let overlayOpen = $state(false);

	let hlsInstance: Hls | null = null;
	const config = useConfigState();

	let updateMode = $state<'yes' | 'no' | 'ask' | null>(null);
	let showSyncPreferenceModal = $state(false);
	let hasUpdatedProgress = $state(false);
	let showAskPrompt = $state(false);

	$effect(() => {
		if (animeId) {
			invoke<{ data: string | null }>('get_local_update_mode', { mediaId: animeId }).then(
				(res: any) => {
					updateMode = (res.data as 'yes' | 'no' | 'ask' | null) || null;
				}
			);
		}
	});

	async function checkProgress() {
		if (!animeId || !currentEpisode || hasUpdatedProgress || !duration || duration <= 0) return;

		const threshold = config.autoUpdateThreshold;
		if (currentTime / duration >= threshold) {
			if (updateMode === 'yes') {
				await performUpdate();
			} else if (updateMode === 'ask') {
				showAskPrompt = true;
			} else if (updateMode === null && config.autoUpdateProgress) {
				// Pause and ask
				videoElement.pause();
				showSyncPreferenceModal = true;
			}
		}
	}

	async function performUpdate() {
		if (!animeId || !currentEpisode || hasUpdatedProgress) return;
		hasUpdatedProgress = true;

		try {
			// Find 1-based index in the episode list
			const currentIndex = episodes.findIndex((e: Episode) => e.id === currentEpisode.id);
			const oneBasedIndex = currentIndex !== -1 ? currentIndex + 1 : currentEpisode.number;

			// Update AniList
			await invoke('update_media_progress', {
				mediaId: animeId,
				progress: oneBasedIndex,
			});

			// Update local DB
			await invoke('update_local_progress', {
				params: {
					anime_id: animeId,
					episode_number: currentEpisode.number,
					last_position: Math.floor(currentTime),
					total_duration: Math.floor(duration),
					update_mode: updateMode || 'yes',
				},
			});

			console.log(
				`[Player] Progress updated to episode ${oneBasedIndex} (local: ${currentEpisode.number})`
			);
		} catch (e) {
			console.error('[Player] Failed to update progress:', e);
			hasUpdatedProgress = false;
		}
	}

	async function handlePreferenceSelect(mode: 'yes' | 'no' | 'ask') {
		updateMode = mode;
		// Save to DB immediately
		if (animeId && currentEpisode) {
			await invoke('update_local_progress', {
				params: {
					anime_id: animeId,
					episode_number: currentEpisode.number,
					last_position: Math.floor(currentTime),
					total_duration: Math.floor(duration),
					update_mode: mode,
				},
			});
		}

		if (mode === 'yes') {
			await performUpdate();
		}

		// Resume playback
		videoElement.play().catch(console.error);
	}

	$effect(() => {
		if (videoElement) {
			videoElement.playbackRate = config.playbackSpeed;
		}
	});

	function handleSpeedChange(speed: number) {
		config.setPlaybackSpeed(speed);
		if (videoElement) {
			videoElement.playbackRate = speed;
		}
	}

	/**
	 * Returns a custom hls.js loader class that routes every manifest and segment
	 * request through Tauri's native HTTP client instead of the WebView's fetch/XHR.
	 *
	 * This is necessary for external CDN streams where:
	 *  - The browser's same-origin policy (CORS) would block the request, and
	 *  - Custom headers such as `Referer` or `Cookie` cannot be set by the WebView.
	 *
	 * The loader uses:
	 *  - `fetch_url`          for manifest requests (text/json responses)
	 *  - `fetch_bytes_base64` for segment requests (binary responses)
	 */
	function createTauriLoader(streamHeaders: Record<string, string>) {
		const hdrs = Object.keys(streamHeaders).length > 0 ? streamHeaders : undefined;
		return class TauriLoader {
			context: any;
			// Stats must be pre-initialized so hls.js ABR controller can read
			// stats.loading before load() is ever called on this instance.
			stats: any = {
				aborted: false,
				loaded: 0,
				retry: 0,
				total: 0,
				chunkCount: 0,
				bwEstimate: 0,
				loading: { start: 0, first: 0, end: 0 },
				parsing: { start: 0, end: 0 },
				buffering: { start: 0, first: 0, end: 0 },
			};
			private aborted = false;

			load(context: any, _config: any, callbacks: any) {
				this.context = context;
				const start = performance.now();
				this.stats.loading.start = start;
				const { url, responseType } = context;
				if (responseType === 'arraybuffer') {
					invoke<string>('fetch_bytes_base64', { url, headers: hdrs })
						.then((b64) => {
							if (this.aborted) return;
							const binary = atob(b64);
							const bytes = new Uint8Array(binary.length);
							for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
							const now = performance.now();
							this.stats.loaded = bytes.byteLength;
							this.stats.total = bytes.byteLength;
							this.stats.loading.first = now;
							this.stats.loading.end = now;
							callbacks.onSuccess({ url, data: bytes.buffer }, this.stats, context, null);
						})
						.catch((err) => {
							if (this.aborted) return;
							this.stats.loading.end = performance.now();
							callbacks.onError({ code: 0, text: String(err) }, context, null, this.stats);
						});
				} else {
					// text or json (manifest, subtitle tracks, etc.)
					invoke<string>('fetch_url', { url, headers: hdrs })
						.then((text) => {
							if (this.aborted) return;
							const now = performance.now();
							this.stats.loaded = text.length;
							this.stats.total = text.length;
							this.stats.loading.first = now;
							this.stats.loading.end = now;
							callbacks.onSuccess({ url, data: text }, this.stats, context, null);
						})
						.catch((err) => {
							if (this.aborted) return;
							this.stats.loading.end = performance.now();
							callbacks.onError({ code: 0, text: String(err) }, context, null, this.stats);
						});
				}
			}
			abort() {
				this.aborted = true;
				this.stats.aborted = true;
			}
			destroy() {
				this.aborted = true;
			}
		};
	}

	function destroyHls() {
		if (hlsInstance) {
			hlsInstance.destroy();
			hlsInstance = null;
		}
	}

	function attachHls(url: string) {
		destroyHls();
		if (!videoElement) return;
		if (Hls.isSupported()) {
			// enableWorker: false  — WebView2 dropped native video/mp2t MSE support;
			//                        transmux TS→fMP4 on the main thread instead.
			// preferManagedMediaSource: false — force standard MSE (ManagedMediaSource
			//                        is a Safari-only API; not available in WebView2).
			//
			// When stream headers are present (CDN with Referer/Cookie requirements),
			// inject the Tauri loader so all requests go through the Rust HTTP client
			// instead of the WebView's fetch, bypassing CORS and forbidden-header rules.
			const useTauriLoader =
				Object.keys(headers).length > 0 ||
				(!url.startsWith('http://127.') && !url.startsWith('http://localhost'));
			const hls = new Hls({
				enableWorker: false,
				lowLatencyMode: false,
				preferManagedMediaSource: false,
				...(useTauriLoader ? { loader: createTauriLoader(headers) } : {}),
			});
			hlsInstance = hls;
			hls.loadSource(url);
			hls.attachMedia(videoElement);

			hls.on(Hls.Events.MANIFEST_PARSED, (_, data) => {
				const summary = data.levels
					.map((l) => `${l.height ?? '?'}p v=${l.videoCodec ?? '?'} a=${l.audioCodec ?? '?'}`)
					.join(' | ');
				console.debug('[hls] manifest parsed →', summary);
			});

			let recoveryAttempts = 0;
			let networkRetries = 0;
			hls.on(Hls.Events.ERROR, (_, data) => {
				if (!data.fatal) return;
				console.error('[hls] fatal', data.type, data.details, data.mimeType, data.error?.message);
				if (data.type === Hls.ErrorTypes.MEDIA_ERROR) {
					if (data.details === Hls.ErrorDetails.BUFFER_ADD_CODEC_ERROR) {
						// The codec is not supported by MSE — recoverMediaError() cannot
						// help here. Show the rejected MIME type so the user knows which
						// codec failed (often HEVC on WebView2).
						const mime = data.mimeType ? `: ${data.mimeType}` : '';
						hasError = true;
						errorMessage = `Codec not supported${mime} — try a lower-quality source`;
						return;
					}
					recoveryAttempts++;
					if (recoveryAttempts === 1) {
						hls.recoverMediaError();
					} else if (recoveryAttempts === 2) {
						hls.swapAudioCodec();
						hls.recoverMediaError();
					} else {
						hasError = true;
						errorMessage = data.details ?? 'HLS fatal media error';
					}
				} else if (data.type === Hls.ErrorTypes.NETWORK_ERROR) {
					networkRetries++;
					if (networkRetries <= 3) {
						hls.startLoad();
					} else {
						hasError = true;
						errorMessage = `Network error: ${data.details}`;
					}
				} else {
					hasError = true;
					errorMessage = data.details ?? 'HLS fatal error';
				}
			});
		} else if (videoElement.canPlayType('application/vnd.apple.mpegurl')) {
			// Native HLS (Safari / WebKit)
			videoElement.src = url;
		} else {
			hasError = true;
			errorMessage = 'HLS is not supported in this environment.';
		}
	}

	$effect(() => {
		const activeSrc = src ?? mediaSources[0]?.src;
		if (!activeSrc) return;
		hasError = false;
		isBuffering = true;
		const isHls = activeSrc.includes('.m3u8') || activeSrc.includes('/m3u8');
		if (isHls) {
			attachHls(activeSrc);
		} else {
			destroyHls();
			if (videoElement) videoElement.load();
		}
	});

	// Animation frame ID for smooth slider updates
	let rafId: number;

	function updateTime() {
		if (videoElement && !videoElement.paused && !videoElement.ended) {
			if (showControls || isLocked) {
				currentTime = videoElement.currentTime;
			}
			checkProgress();
			rafId = requestAnimationFrame(updateTime);
		}
	}

	function togglePlay() {
		if (videoElement.paused) {
			videoElement.play().catch((e) => {
				if (e.name === 'AbortError') return;
				console.error('Play error:', e);
				hasError = true;
				errorMessage = e.message;
			});
		} else {
			videoElement.pause();
		}
	}

	function handlePlay() {
		isPlaying = true;
		hasError = false;
		cancelAnimationFrame(rafId);
		updateTime();
	}

	function handlePause() {
		isPlaying = false;
		cancelAnimationFrame(rafId);
	}

	let lastMouseMove = 0;

	function handleSeeking() {
		// Update currentTime immediately when the user seeks so the progress bar
		// tracks the target position even while the network buffers data there.
		currentTime = videoElement.currentTime;
		isBuffering = true;
	}

	function handleTimeUpdate() {
		// Always update when paused (no RAF loop running).
		// When playing, skip the update while controls are visible — the RAF
		// loop in updateTime() handles that case to avoid fighting user drags.
		if ((!isPlaying || !showControls) && !isLocked) {
			currentTime = videoElement.currentTime;
		}
		checkProgress();
		// Keep isPlaying in sync with the actual element state. This catches
		// cases where the video starts playing before our 'play' event handler
		// fires (e.g. autoplay, hls.js triggers, or rapid src swaps).
		const actuallyPlaying = !videoElement.paused && !videoElement.ended;
		if (isPlaying !== actuallyPlaying) isPlaying = actuallyPlaying;
		showSkipIntro = false;
		if (isBuffering && isPlaying) {
			isBuffering = false;
		}
		updateDiscordActivity();
	}

	function handleLoadedMetadata() {
		duration = videoElement.duration;
		if (tracks.length > 0 && currentTrackIndex === -1) {
			// Leave subtitles off by default unless user selects
		}
	}

	function handleCanPlay() {
		isBuffering = false;
		hasError = false;
	}

	function handleSeek(time: number) {
		videoElement.currentTime = time;
		currentTime = time;
	}

	function handleVolumeChange(vol: number) {
		volume = vol;
		videoElement.volume = vol;
		localStorage.setItem('zafkiel-player-volume', vol.toString());
	}

	function handleTrackChange(index: number) {
		currentTrackIndex = index;
		if (videoElement) {
			for (let i = 0; i < videoElement.textTracks.length; i++) {
				videoElement.textTracks[i].mode = i === index ? 'showing' : 'hidden';
			}
		}
	}

	function toggleFullscreen() {
		if (!document.fullscreenElement) {
			containerElement.requestFullscreen();
		} else {
			document.exitFullscreen();
		}
	}

	function toggleLock() {
		isLocked = !isLocked;
		showControls = true;
		resetControlsTimeout();
	}

	function handleSkipIntro() {
		videoElement.currentTime += 85;
	}

	function handleMouseMove() {
		if (isLocked) return;
		const now = performance.now();
		if (now - lastMouseMove < 100) return;
		lastMouseMove = now;
		if (!showControls) showControls = true;
		resetControlsTimeout();
	}

	function resetControlsTimeout() {
		clearTimeout(controlsTimeout);
		if (overlayOpen) return; // Don't hide controls while overlay is open
		controlsTimeout = setTimeout(() => {
			if (isPlaying) showControls = false;
		}, 3000);
	}

	function handleWaiting() {
		isBuffering = true;
	}

	function handlePlaying() {
		// The browser fires 'waiting' when the buffer is too low to continue;
		// there is no need to manually inspect the buffer here or force a pause.
		// Doing so just creates a play → pause → waiting → play oscillation.
		// 'playing' fires whenever the video actually starts rendering frames
		// (autoplay, hls.js implicit start, resume after buffering) — always sync here.
		isPlaying = true;
		isBuffering = false;
		hasError = false;
	}

	function handleError(e: Event) {
		console.error('Video error:', e);
		hasError = true;
		errorMessage = videoElement.error?.message || 'Unknown error occurred';
		isBuffering = false;
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (isLocked) return;
		if (!showControls && isPlaying) {
			showControls = true;
			resetControlsTimeout();
		}
		switch (e.key) {
			case 'Escape':
			case 'q':
			case 'Q':
				e.preventDefault();
				onBack?.();
				break;
			case ' ':
			case 'k':
				e.preventDefault();
				togglePlay();
				break;
			case 'f':
				e.preventDefault();
				toggleFullscreen();
				break;
			case 'ArrowRight':
				e.preventDefault();
				videoElement.currentTime += 5;
				break;
			case 'ArrowLeft':
				e.preventDefault();
				videoElement.currentTime -= 5;
				break;
			case 'ArrowUp': {
				e.preventDefault();
				const newVolUp = Math.min(1, videoElement.volume + 0.1);
				handleVolumeChange(newVolUp);
				break;
			}
			case 'ArrowDown': {
				e.preventDefault();
				const newVolDown = Math.max(0, videoElement.volume - 0.1);
				handleVolumeChange(newVolDown);
				break;
			}
		}
	}

	function handleContainerClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (target.tagName === 'VIDEO' || target.classList.contains('click-area')) {
			togglePlay();
		}
	}

	function handleContainerDoubleClick(e: MouseEvent) {
		const rect = containerElement.getBoundingClientRect();
		const x = e.clientX - rect.left;
		const width = rect.width;

		if (x < width / 3) {
			videoElement.currentTime -= 10;
		} else if (x > (width * 2) / 3) {
			videoElement.currentTime += 10;
		} else {
			toggleFullscreen();
		}
	}

	onMount(() => {
		// WebView2 rejects 'mp4a.40.1' (AAC Main Profile) in addSourceBuffer()
		// even though the bitstream is identical to mp4a.40.2 (AAC-LC).
		// hls.js 1.x derives this string from parsing the fMP4 init segment atoms,
		// not from the m3u8 CODECS attribute, so patching the m3u8 is not enough.
		// Normalise it at the MSE API boundary before the browser sees it.
		const _origAddSourceBuffer = MediaSource.prototype.addSourceBuffer;
		MediaSource.prototype.addSourceBuffer = function (mimeType: string) {
			return _origAddSourceBuffer.call(this, mimeType.replace(/mp4a\.40\.1\b/g, 'mp4a.40.2'));
		};

		const savedVol = localStorage.getItem('zafkiel-player-volume');
		if (savedVol) {
			volume = parseFloat(savedVol);
		}

		resetControlsTimeout();
		window.addEventListener('keydown', handleKeyDown);
	});

	function updateDiscordActivity() {
		let startTimestamp: number | undefined;
		let endTimestamp: number | undefined;

		if (isPlaying && duration > 0) {
			startTimestamp = Math.floor(Date.now() / 1000) - Math.floor(currentTime);
			endTimestamp = startTimestamp + Math.floor(duration);
		}

		discordStore.setActivity({
			state: isPlaying
				? currentEpisode
					? `Watching Episode ${currentEpisode.number}`
					: 'Watching Video'
				: 'Paused',
			details: title || 'Local Video',
			largeImage: image || 'logo',
			largeText: title || 'Zafkiel',
			smallImage: isPlaying ? 'play' : 'pause',
			smallText: isPlaying ? 'Playing' : 'Paused',
			startTimestamp,
			endTimestamp,
		});
	}

	$effect(() => {
		if (src || mediaSources.length > 0) {
			updateDiscordActivity();
		}
	});

	onDestroy(() => {
		clearTimeout(controlsTimeout);
		window.removeEventListener('keydown', handleKeyDown);
		discordStore.clearActivity();
		destroyHls();
	});
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	bind:this={containerElement}
	class="group relative aspect-video w-full cursor-pointer overflow-hidden rounded-lg bg-black outline-none"
	onmousemove={handleMouseMove}
	onmouseleave={() => (showControls = false)}
	onclick={handleContainerClick}
	ondblclick={handleContainerDoubleClick}
	role="application"
>
	<video
		bind:this={videoElement}
		class="h-full w-full object-contain"
		onplay={handlePlay}
		onpause={handlePause}
		onwaiting={handleWaiting}
		onplaying={handlePlaying}
		onerror={handleError}
		ontimeupdate={handleTimeUpdate}
		onloadedmetadata={handleLoadedMetadata}
		oncanplay={handleCanPlay}
		onseeking={handleSeeking}
	>
		{#if src}
			<source {src} />
		{:else if mediaSources.length > 0}
			{#each mediaSources as source}
				<source src={source.src} type={source.type} />
			{/each}
		{/if}

		{#each tracks as track}
			<track kind="subtitles" src={track.src} label={track.label} srclang={track.lang} />
		{/each}
	</video>

	{#if hasError}
		<div class="absolute inset-0 z-20 flex flex-col items-center justify-center bg-black/80">
			<Icon icon="lucide:alert-circle" class="mb-2 h-12 w-12 text-red-500" />
			<p class="font-medium text-white">Playback Error</p>
			<p class="text-sm text-white/70">{errorMessage}</p>
			<p class="max-w-md px-4 text-center text-xs text-white/50">
				The browser player cannot play this stream. Try switching to the Libmpv player or opening in
				an external player.
			</p>
			<div class="mt-4 flex gap-2">
				<Button
					variant="outline"
					size="sm"
					onclick={() => {
						hasError = false;
						const activeSrc = src ?? mediaSources[0]?.src;
						if (activeSrc) attachHls(activeSrc);
					}}>Retry</Button
				>
				<Button
					variant="default"
					size="sm"
					onclick={async () => {
						try {
							const activeSrc = src ?? mediaSources[0]?.src;
							if (activeSrc) await invoke('open_in_external_player', { url: activeSrc });
						} catch (e) {
							console.error('Failed to open external player:', e);
						}
					}}
				>
					<Icon icon="lucide:external-link" class="mr-2 h-4 w-4" />
					Open in Player
				</Button>
			</div>
		</div>
	{/if}

	{#if !isPlaying && !src && mediaSources.length === 0 && !hasError}
		<div class="absolute inset-0 flex items-center justify-center bg-black/50">
			<p class="text-white">No video source selected</p>
		</div>
	{/if}

	{#if !hasError}
		{#if showControls || !isPlaying || isLocked}
			<div class="absolute inset-0 z-10" transition:fade={{ duration: 200 }}>
				<PlayerControls
					{isPlaying}
					{currentTime}
					duration={isFinite(duration) ? duration : 0}
					{volume}
					{title}
					{subtitle}
					{isLocked}
					{showSkipIntro}
					{tracks}
					{currentTrackIndex}
					onPlayPause={togglePlay}
					onSeek={handleSeek}
					onVolumeChange={handleVolumeChange}
					onFullscreen={toggleFullscreen}
					onLockToggle={toggleLock}
					{onBack}
					onSkipIntro={handleSkipIntro}
					{isBuffering}
					onTrackChange={handleTrackChange}
					playbackSpeed={config.playbackSpeed}
					onSpeedChange={handleSpeedChange}
					{episodes}
					{currentEpisode}
					{sources}
					{currentSource}
					{onEpisodeSelect}
					{onSourceSelect}
					canUseShaders={false}
					onOverlayToggle={(open) => {
						overlayOpen = open;
						if (open) {
							clearTimeout(controlsTimeout);
							showControls = true;
						} else {
							resetControlsTimeout();
						}
					}}
				/>
			</div>
		{/if}
	{/if}

	<SyncPreferenceModal
		bind:open={showSyncPreferenceModal}
		animeTitle={title || 'this anime'}
		onSelect={handlePreferenceSelect}
	/>

	{#if showAskPrompt}
		<div
			class="absolute top-4 right-4 z-[110] flex flex-col gap-2 rounded-xl border border-white/20 bg-black/60 p-4 backdrop-blur-md transition-all duration-300"
			transition:fade
		>
			<div class="flex items-center gap-3">
				<div
					class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/20 text-primary"
				>
					<Icon icon="solar:check-read-linear" class="h-6 w-6" />
				</div>
				<div>
					<div class="text-sm font-bold tracking-tight text-white uppercase">Sync Progress?</div>
					<div class="text-xs text-white/70">
						Update AniList to episode {currentEpisode?.number}
					</div>
				</div>
			</div>
			<div class="flex gap-2 pt-1">
				<Button
					size="sm"
					variant="default"
					class="h-8 flex-1 rounded-lg text-xs font-bold tracking-wider uppercase"
					onclick={async () => {
						await performUpdate();
						showAskPrompt = false;
					}}
				>
					Sync
				</Button>
				<Button
					size="sm"
					variant="ghost"
					class="h-8 flex-1 rounded-lg text-xs font-bold tracking-wider text-white/50 uppercase hover:text-white"
					onclick={() => {
						showAskPrompt = false;
						hasUpdatedProgress = true; // Don't ask again for this ep
					}}
				>
					Ignore
				</Button>
			</div>
		</div>
	{/if}
</div>
