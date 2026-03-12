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

	let {
		url,
		headers = {},
		title,
		subtitle,
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
		onBack?: () => void;
		episodes?: Episode[];
		currentEpisode?: Episode | null;
		sources?: StreamSource[];
		currentSource?: StreamSource | null;
		onEpisodeSelect?: (ep: Episode) => void;
		onSourceSelect?: (src: StreamSource) => void;
	}>();

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

	// ── mpv observed properties ───────────────────────────────────────────────
	const OBSERVED_PROPERTIES = [
		['pause', 'flag'],
		['time-pos', 'double', 'none'],
		['duration', 'double', 'none'],
		['cache-buffering-state', 'int64'],
	] as const satisfies MpvObservableProperty[];

	// ── Load a URL into mpv ───────────────────────────────────────────────────
	async function loadUrl(streamUrl: string, streamHeaders: Record<string, string>) {
		hasError = false;
		isBuffering = true;
		isPlaying = false;
		currentTime = 0;
		duration = 0;
		try {
			const referer = streamHeaders['Referer'] ?? streamHeaders['referer'];
			if (referer) await setProperty('referrer', referer);
			const extraHeaders = Object.entries(streamHeaders)
				.filter(([k]) => k.toLowerCase() !== 'referer')
				.map(([k, v]) => `${k}: ${v}`)
				.join(',');
			if (extraHeaders) await setProperty('http-header-fields', extraHeaders);
			await command('loadfile', [streamUrl]);
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
			await command('cycle', ['pause']);
		} catch (e) {
			console.error('[mpv] togglePlay error:', e);
		}
	}

	async function handleSeek(time: number) {
		if (!isInitialized) return;
		currentTime = time;
		try {
			await command('seek', [String(time), 'absolute']);
		} catch (e) {
			console.error('[mpv] seek error:', e);
		}
	}

	async function handleVolumeChange(vol: number) {
		if (!isInitialized) return;
		volume = vol;
		try {
			await setProperty('volume', Math.round(vol * 100));
			localStorage.setItem('zafkiel-player-volume', vol.toString());
		} catch (e) {
			console.error('[mpv] volume error:', e);
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
				await command('seek', ['5', 'relative']);
				break;
			case 'ArrowLeft':
				e.preventDefault();
				await command('seek', ['-5', 'relative']);
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
					volume: Math.round(volume * 100),
				},
				observedProperties: OBSERVED_PROPERTIES,
			});

			unlistenProps = await observeProperties(
				OBSERVED_PROPERTIES,
				({ name, data }: { name: string; data: unknown }) => {
					switch (name) {
						case 'pause':
							isPlaying = data === false;
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
				}
			);

			unlistenEvents = await listenEvents((evt) => {
				switch (evt.event) {
					case 'file-loaded':
						isBuffering = false;
						hasError = false;
						break;
					case 'playback-restart':
						isBuffering = false;
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
			});

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

	onDestroy(async () => {
		playerStore.hide();
		clearTimeout(controlsTimeout);
		unlistenProps?.();
		unlistenEvents?.();
		try {
			await destroy();
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
				onSkipIntro={() => command('seek', ['85', 'relative'])}
				{isBuffering}
				onTrackChange={() => {}}
				{episodes}
				{currentEpisode}
				{sources}
				{currentSource}
				{onEpisodeSelect}
				{onSourceSelect}
			/>
		</div>
	{/if}
</div>
