<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import PlayerControls from './PlayerControls.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	let {
		src,
		sources = [],
		tracks = [],
		title,
		subtitle,
		poster,
		onBack,
	} = $props<{
		src?: string;
		sources?: { src: string; type: string }[];
		tracks?: { id: string; label: string; src: string; lang: string }[];
		title?: string;
		subtitle?: string;
		poster?: string;
		onBack?: () => void;
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
	let controlsTimeout: any;
	let currentTrackIndex = $state(-1); // -1 = off

	let isBuffering = $state(false);
	let hasError = $state(false);
	let errorMessage = $state('');

	// Animation frame ID for smooth slider updates
	let rafId: number;

	function updateTime() {
		if (videoElement && !videoElement.paused && !videoElement.ended) {
			if (showControls || isLocked) {
				currentTime = videoElement.currentTime;
			}
			rafId = requestAnimationFrame(updateTime);
		}
	}

	function togglePlay() {
		if (videoElement.paused) {
			videoElement.play().catch((e) => {
				// Ignore AbortError which happens when pausing while loading
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
		// Start RAF loop
		cancelAnimationFrame(rafId);
		updateTime();
	}

	function handlePause() {
		isPlaying = false;
		cancelAnimationFrame(rafId);
	}

	// Throttle UI updates for smoother playback
	let lastMouseMove = 0;

	function handleTimeUpdate() {
		// Use timeupdate mainly for low-freq logic if needed, or sync backup
		if (!showControls && !isLocked) {
			currentTime = videoElement.currentTime;
		}

		// Mock skip intro logic - show between 1:30 and 3:00
		// FIX: Commenting out to verify if this state change causes the specific-time flicker
		// showSkipIntro = currentTime > 90 && currentTime < 180;
		showSkipIntro = false;

		// If time is updating, we are definitely not buffering
		if (isBuffering && isPlaying) {
			isBuffering = false;
		}
	}

	$effect(() => {
		if (src || sources.length > 0) {
			hasError = false;
			isBuffering = true;
		}
	});

	function handleLoadedMetadata() {
		duration = videoElement.duration;
		// Initialize tracks
		if (tracks.length > 0 && currentTrackIndex === -1) {
			// Auto-select first track if English? Or leave off?
			// For now leave off unless user selects
		}
	}

	function handleCanPlay() {
		isBuffering = false;
		hasError = false;
	}

	function handleSeek(time: number) {
		videoElement.currentTime = time;
		currentTime = time; // Update immediately on seek
	}

	function handleVolumeChange(vol: number) {
		volume = vol;
		videoElement.volume = vol;
		localStorage.setItem('zafkiel-player-volume', vol.toString());
	}

	function handleTrackChange(index: number) {
		currentTrackIndex = index;
		if (videoElement) {
			// Loop through textTracks and set mode
			// The tracks in videoElement.textTracks correspond to the <track> tags in order
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

		// Throttle mouse move checks to every 100ms
		const now = performance.now();
		if (now - lastMouseMove < 100) return;
		lastMouseMove = now;

		if (!showControls) {
			showControls = true;
		}
		resetControlsTimeout();
	}

	function resetControlsTimeout() {
		clearTimeout(controlsTimeout);
		controlsTimeout = setTimeout(() => {
			if (isPlaying) showControls = false;
		}, 3000);
	}

	function handleWaiting() {
		isBuffering = true;
	}

	function handlePlaying() {
		// Prevent flickering: Ensure we have enough buffer before resuming
		// unless we are near the end of the video
		if (videoElement && duration) {
			const current = videoElement.currentTime;
			const buffered = videoElement.buffered;
			let bufferedEnd = 0;

			for (let i = 0; i < buffered.length; i++) {
				if (buffered.start(i) <= current && buffered.end(i) >= current) {
					bufferedEnd = buffered.end(i);
					break;
				}
			}

			const remainingBuffer = bufferedEnd - current;
			const remainingVideo = duration - current;

			// If we have less than 3s buffer and we are not at the end
			if (remainingBuffer < 3 && remainingVideo > 3) {
				console.log(`Low buffer (${remainingBuffer.toFixed(2)}s), forcing pause to buffer...`);
				videoElement.pause();
				isBuffering = true;
				return;
			}
		}

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
			case 'ArrowUp':
				e.preventDefault();
				const newVolUp = Math.min(1, videoElement.volume + 0.1);
				handleVolumeChange(newVolUp);
				break;
			case 'ArrowDown':
				e.preventDefault();
				const newVolDown = Math.max(0, videoElement.volume - 0.1);
				handleVolumeChange(newVolDown);
				break;
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
			// Left third: Seek back
			videoElement.currentTime -= 10;
		} else if (x > (width * 2) / 3) {
			// Right third: Seek forward
			videoElement.currentTime += 10;
		} else {
			// Center: Toggle Fullscreen
			toggleFullscreen();
		}
	}

	onMount(() => {
		// Load volume
		const savedVol = localStorage.getItem('zafkiel-player-volume');
		if (savedVol) {
			volume = parseFloat(savedVol);
		}

		resetControlsTimeout();
		window.addEventListener('keydown', handleKeyDown);
	});

	onDestroy(() => {
		clearTimeout(controlsTimeout);
		window.removeEventListener('keydown', handleKeyDown);
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
	>
		{#if src}
			<source {src} />
		{:else if sources.length > 0}
			{#each sources as source}
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
				The internal player cannot play this file format. Please try opening it in an external
				player.
			</p>
			<div class="mt-4 flex gap-2">
				<Button
					variant="outline"
					size="sm"
					onclick={() => {
						videoElement.load();
						hasError = false;
					}}>Retry</Button
				>
				<Button
					variant="default"
					size="sm"
					onclick={async () => {
						try {
							// For torrents, src might need to be passed differently or if it's a stream URL,
							// external player handling might need the original magnet.
							// Assuming src is the stream URL being played.
							// If src is the stream URL, the backend might expect a Magnet URI for "open_in_external_player"
							// OR the backend command handles stream URLs.
							// Let's assume we invoke the general command.
							// Wait, the previous implementation used `stream_torrent` which returned a port?
							// Actually, let's use the invoke command `open_in_external_player`
							const { invoke } = await import('@tauri-apps/api/core');
							await invoke('open_in_external_player', { url: src });
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

	{#if !isPlaying && !src && !hasError}
		<div class="absolute inset-0 flex items-center justify-center bg-black/50">
			<p class="text-white">No video source selected</p>
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
			/>
		</div>
	{/if}
</div>
