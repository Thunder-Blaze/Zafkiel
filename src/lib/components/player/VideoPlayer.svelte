<script lang="ts">
	/**
	 * VideoPlayer — libmpv underlay implementation.
	 *
	 * Architecture ("underlay trick"):
	 *   1. tauri-plugin-libmpv grabs the raw OS window handle and renders video
	 *      to the native window layer beneath the WebView.
	 *   2. This component renders as a position:fixed full-viewport overlay with
	 *      background:transparent, creating a "see-through hole" to the native
	 *      video layer below.
	 *   3. PlayerControls sits inside the overlay with its own dark background,
	 *      rendering on top of the video.
	 *
	 * mpv handles HLS, DASH, MP4 and virtually every format/codec natively —
	 * no hls.js transmuxing, no CORS workarounds, no WebView codec constraints.
	 * The `referrer` and `http-header-fields` properties pass required CDN
	 * authentication headers (Referer, Cookie) directly from mpv's HTTP client.
	 */
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { playerStore } from '$lib/stores/player.svelte';
	import PlayerControls from './PlayerControls.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import {
		init,
		destroy,
		observeProperties,
		listenEvents,
		command,
		setProperty,
		type MpvObservableProperty,
	} from 'tauri-plugin-libmpv-api';
	import type { Episode, StreamSource } from '$lib/types/extensions';
	import { useConfigState } from '$lib/stores/config.svelte';
	import { discordStore } from '$lib/stores/discord.svelte';

	let {
		url,
		headers = {},
		title,
		subtitle,
		image,
		onBack,
		episodes = [],
		currentEpisode = null,
		sources = [],
		currentSource = null,
		onEpisodeSelect,
		onSourceSelect,
	} = $props<{
		url: string;
		headers?: Record<string, string>;
		title?: string;
		subtitle?: string;
		image?: string;
		onBack?: () => void;
		episodes?: Episode[];
		currentEpisode?: Episode | null;
		sources?: StreamSource[];
		currentSource?: StreamSource | null;
		onEpisodeSelect?: (ep: Episode) => void;
		onSourceSelect?: (src: StreamSource) => void;
	}>();

	const config = useConfigState();

	// ── State ────────────────────────────────────────────────────────────────
	let isPlaying = $state(false);
	let currentTime = $state(0);
	let duration = $state(0);
	let volume = $state(1);
	let showControls = $state(true);
	let isLocked = $state(false);
	let showSkipIntro = $state(false);
	let isBuffering = $state(true);
	let hasError = $state(false);
	let errorMessage = $state('');
	let isInitialized = $state(false);
	let controlsTimeout: ReturnType<typeof setTimeout> | undefined;
	let unlistenProps: (() => void) | null = null;
	let unlistenEvents: (() => void) | null = null;
	let lastMouseMove = 0;
	let overlayOpen = $state(false);

	// ── mpv observed properties ───────────────────────────────────────────────
	const OBSERVED_PROPERTIES = [
		['pause', 'flag'],
		['time-pos', 'double', 'none'],
		['duration', 'double', 'none'],
		['cache-buffering-state', 'int64'],
	] as const satisfies MpvObservableProperty[];
	const MPV_WINDOW_LABEL = 'main';

	// ── Load a URL into mpv ───────────────────────────────────────────────────
	async function loadUrl(streamUrl: string, streamHeaders: Record<string, string>) {
		hasError = false;
		isBuffering = true;
		isPlaying = false;
		currentTime = 0;
		duration = 0;
		try {
			const referer = streamHeaders['Referer'] ?? streamHeaders['referer'];
			if (referer) await setProperty('referrer', referer, MPV_WINDOW_LABEL);
			const extraHeaders = Object.entries(streamHeaders)
				.filter(([k]) => k.toLowerCase() !== 'referer')
				.map(([k, v]) => `${k}: ${v}`)
				.join(',');
			if (extraHeaders) await setProperty('http-header-fields', extraHeaders, MPV_WINDOW_LABEL);
			await command('loadfile', [streamUrl], MPV_WINDOW_LABEL);
		} catch (e) {
			console.error('[mpv] loadUrl error:', e);
			hasError = true;
			errorMessage = e instanceof Error ? e.message : String(e);
			isBuffering = false;
		}
	}

	// ── Controls ──────────────────────────────────────────────────────────────
	async function togglePlay() {
		if (!isInitialized) return;
		try {
			await command('cycle', ['pause'], MPV_WINDOW_LABEL);
		} catch (e) {
			console.error('[mpv] togglePlay error:', e);
		}
	}

	async function handleSeek(time: number) {
		if (!isInitialized) return;
		currentTime = time;
		try {
			await command('seek', [String(time), 'absolute'], MPV_WINDOW_LABEL);
		} catch (e) {
			console.error('[mpv] seek error:', e);
		}
	}

	async function handleVolumeChange(vol: number) {
		if (!isInitialized) return;
		volume = vol;
		try {
			await setProperty('volume', Math.round(vol * 100), MPV_WINDOW_LABEL);
			localStorage.setItem('zafkiel-player-volume', vol.toString());
		} catch (e) {
			console.error('[mpv] volume error:', e);
		}
	}

	async function handleSpeedChange(speed: number) {
		if (!isInitialized) return;
		config.setPlaybackSpeed(speed);
		try {
			await setProperty('speed', speed, MPV_WINDOW_LABEL);
		} catch (e) {
			console.error('[mpv] speed error:', e);
		}
	}

	function toggleFullscreen() {
		if (!document.fullscreenElement) {
			document.documentElement.requestFullscreen();
		} else {
			document.exitFullscreen();
		}
	}

	function toggleLock() {
		isLocked = !isLocked;
		showControls = true;
		resetControlsTimeout();
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

	async function handleKeyDown(e: KeyboardEvent) {
		if (!isInitialized) return;
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
				await togglePlay();
				break;
			case 'f':
				e.preventDefault();
				toggleFullscreen();
				break;
			case 'ArrowRight':
				e.preventDefault();
				await command('seek', ['5', 'relative'], MPV_WINDOW_LABEL);
				break;
			case 'ArrowLeft':
				e.preventDefault();
				await command('seek', ['-5', 'relative'], MPV_WINDOW_LABEL);
				break;
			case 'ArrowUp':
				e.preventDefault();
				await handleVolumeChange(Math.min(1, volume + 0.1));
				break;
			case 'ArrowDown':
				e.preventDefault();
				await handleVolumeChange(Math.max(0, volume - 0.1));
				break;
		}
	}

	// ── Lifecycle ─────────────────────────────────────────────────────────────
	onMount(async () => {
		playerStore.show();

		const savedVol = localStorage.getItem('zafkiel-player-volume');
		if (savedVol) volume = parseFloat(savedVol);

		try {
			await init({
				initialOptions: {
					// gpu-next (libplacebo) is the recommended renderer.
					// 'auto' selects the best hardware decoder per platform:
					// vaapi/nvdec on Linux, d3d11va/nvdec on Windows, VideoToolbox on macOS.
					vo: 'gpu-next',
					hwdec: 'auto',
					'keep-open': 'yes',
					'osd-level': '0',
					'input-default-bindings': 'no',
					'input-vo-keyboard': 'no',
					speed: config.playbackSpeed,
					volume: Math.round(volume * 100),
					'hidpi-window-scale': 'yes',
					'fbo-format': 'rgba16hf',
				},
				observedProperties: OBSERVED_PROPERTIES,
			}, MPV_WINDOW_LABEL);

			unlistenProps = await observeProperties(
				OBSERVED_PROPERTIES,
				({ name, data }: { name: string; data: unknown }) => {
					switch (name) {
						case 'pause':
							isPlaying = data === false;
							updateDiscordActivity();
							break;
						case 'time-pos':
							if (typeof data === 'number') currentTime = data;
							break;
						case 'duration':
							if (typeof data === 'number') duration = data;
							break;
						case 'cache-buffering-state':
							// 100 means fully buffered / not actively buffering
							isBuffering = typeof data === 'number' ? data < 100 : false;
							break;
					}
				},
				MPV_WINDOW_LABEL
			);

			unlistenEvents = await listenEvents((evt) => {
				switch (evt.event) {
					case 'file-loaded':
						isBuffering = false;
						hasError = false;
						isPlaying = true;
						break;
					case 'playback-restart':
						isBuffering = false;
						isPlaying = true;
						break;
					case 'end-file': {
						const reason =
							'data' in evt && evt.data && typeof evt.data === 'object' && 'reason' in evt.data
								? (evt.data as { reason?: string }).reason
								: undefined;
						if (reason === 'error') {
							hasError = true;
							errorMessage = 'Playback error — check stream URL and headers';
						}
						isPlaying = false;
						break;
					}
				}
			}, MPV_WINDOW_LABEL);

			isInitialized = true;
			resetControlsTimeout();

			// On Linux, mpv's X11 sub-window is created on top of the WebKit window.
			// Lower it so the controls overlay (inside the WebView) sits above the video.
			try {
				await invoke('lower_mpv_subwindow');
			} catch (e) {
				console.warn('[mpv] lower_mpv_subwindow:', e);
			}
		} catch (e) {
			console.error('[mpv] init error:', e);
			hasError = true;
			errorMessage = e instanceof Error ? e.message : 'Failed to initialize player';
		}
	});

	$effect(() => {
		if (isInitialized && url) loadUrl(url, headers);
	});

	$effect(() => {
		if (!isInitialized || !config.shaderConfig) return;
		
		// Explicitly access properties here so Svelte 5 tracks them as dependencies
		const enabled = config.shaderConfig.enabled;
		const selectedShaders = config.shaderConfig.selected_shaders;
		
		const updateShaders = async () => {
			try {
				if (enabled && selectedShaders.length > 0) {
					// Use ; on Windows and : on Unix
					const separator = navigator.userAgent.toLowerCase().includes('win') ? ';' : ':';
					const shaderPaths = selectedShaders.join(separator);
					console.log('[mpv] applying glsl-shaders:', shaderPaths);
					await setProperty('glsl-shaders', shaderPaths, MPV_WINDOW_LABEL);
				} else {
					console.log('[mpv] clearing glsl-shaders');
					await setProperty('glsl-shaders', '', MPV_WINDOW_LABEL);
				}
			} catch (e) {
				console.error('[mpv] failed to set glsl-shaders:', e);
			}
		};
		updateShaders();
	});

	function updateDiscordActivity() {
		if (!isInitialized) return;
        
        let startTimestamp: number | undefined;
        let endTimestamp: number | undefined;

        if (isPlaying && duration > 0) {
            startTimestamp = Math.floor(Date.now() / 1000) - Math.floor(currentTime);
            endTimestamp = startTimestamp + Math.floor(duration);
        }

		discordStore.setActivity({
			state: isPlaying ? (currentEpisode ? `Watching Episode ${currentEpisode.number}` : 'Watching Video') : 'Paused',
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
		// Update discord activity when title or isPlaying changes, if initialized
		if (isInitialized) {
            updateDiscordActivity();
        }
	});

	onDestroy(async () => {
		playerStore.hide();
		discordStore.clearActivity();
		clearTimeout(controlsTimeout);
		unlistenProps?.();
		unlistenEvents?.();
		try {
			await destroy(MPV_WINDOW_LABEL);
		} catch {}
	});
</script>

<!--
	Full-window transparent overlay.
	mpv renders into the native OS window layer below the WebView.
	This fixed div with background:transparent floats the UI on top while
	the transparent hole reveals the mpv video surface underneath.
-->
<svelte:window onkeydown={handleKeyDown} />

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	class="fixed inset-0 z-[100] cursor-pointer overflow-hidden bg-transparent"
	style="visibility: visible; pointer-events: auto"
	onmousemove={handleMouseMove}
	onmouseleave={() => {
		if (!isLocked) showControls = false;
	}}
	role="application"
	aria-label="Video player"
>
	{#if hasError}
		<div class="absolute inset-0 z-20 flex flex-col items-center justify-center bg-black/80">
			<Icon icon="lucide:alert-circle" class="mb-2 h-12 w-12 text-red-500" />
			<p class="font-medium text-white">Playback Error</p>
			<p class="text-sm text-white/70">{errorMessage}</p>
			<div class="mt-4 flex gap-2">
				<Button variant="ghost" size="sm" onclick={() => onBack?.()}>
					<Icon icon="lucide:x" class="mr-2 h-4 w-4" />
					Close
				</Button>
				<Button variant="outline" size="sm" onclick={() => loadUrl(url, headers)}>Retry</Button>
				<Button
					variant="default"
					size="sm"
					onclick={async () => {
						try {
							const { invoke } = await import('@tauri-apps/api/core');
							await invoke('open_in_external_player', { url });
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

	{#if !hasError}
		<div
			class="will-change-opacity absolute inset-0 z-10 transform-[translateZ(0)] transition-opacity duration-200"
			class:opacity-0={!showControls && isPlaying && !isLocked}
			class:pointer-events-none={!showControls && isPlaying && !isLocked}
		>
			<PlayerControls
				{isPlaying}
				{currentTime}
				duration={isFinite(duration) ? duration : 0}
				{volume}
				{title}
				{subtitle}
				{isLocked}
				{showSkipIntro}
				tracks={[]}
				currentTrackIndex={-1}
				onPlayPause={togglePlay}
				onSeek={handleSeek}
				onVolumeChange={handleVolumeChange}
				onFullscreen={toggleFullscreen}
				onLockToggle={toggleLock}
				{onBack}
				onSkipIntro={() => command('seek', ['85', 'relative'], MPV_WINDOW_LABEL)}
				{isBuffering}
				onTrackChange={() => {}}
				playbackSpeed={config.playbackSpeed}
				onSpeedChange={handleSpeedChange}
				{episodes}
				{currentEpisode}
				{sources}
				{currentSource}
				{onEpisodeSelect}
				{onSourceSelect}
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
</div>
