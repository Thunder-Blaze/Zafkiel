<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { cn } from '$lib/utils';

	let {
		isPlaying,
		currentTime,
		duration,
		volume,
		title,
		subtitle,
		isLocked = false,
		onPlayPause,
		onSeek,
		onVolumeChange,
		onFullscreen,
		onLockToggle,
		onBack,
		onSkipIntro,
		showSkipIntro = false,
		isBuffering = false,
		tracks = [],
		currentTrackIndex = -1,
		onTrackChange = () => {},
	} = $props<{
		isPlaying: boolean;
		currentTime: number;
		duration: number;
		volume: number;
		title?: string;
		subtitle?: string;
		isLocked?: boolean;
		onPlayPause: () => void;
		onSeek: (time: number) => void;
		onVolumeChange: (vol: number) => void;
		onFullscreen: () => void;
		onLockToggle?: () => void;
		onBack?: () => void;
		onSkipIntro?: () => void;
		showSkipIntro?: boolean;
		isBuffering?: boolean;
		tracks?: { id: string; label: string; src: string; lang: string }[];
		currentTrackIndex?: number;
		onTrackChange?: (index: number) => void;
	}>();

	let showSubtitleMenu = $state(false);

	function formatTime(seconds: number): string {
		if (!seconds || isNaN(seconds)) return '00:00';
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		const hours = Math.floor(mins / 60);

		if (hours > 0) {
			const remainingMins = mins % 60;
			return `${hours}:${remainingMins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
		}

		return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}

	function handleSeek(vals: number[]) {
		onSeek(vals[0]);
	}
</script>

<Tooltip.Provider>
	<div class="absolute inset-0 z-50 flex flex-col justify-between bg-black/60 p-4">
		<!-- Locked State Overlay -->
		{#if isLocked}
			<div class="absolute inset-0 flex flex-col items-end justify-start p-4">
				<Tooltip.Root>
					<Tooltip.Trigger>
						<Button
							variant="ghost"
							size="icon"
							class="text-white hover:bg-white/20"
							onclick={onLockToggle}
						>
							<Icon icon="lucide:lock" class="h-6 w-6" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>Unlock Controls</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</div>
		{:else}
			<!-- Top Bar -->
			<div class="flex items-start justify-between">
				<div class="flex flex-col">
					<h2 class="line-clamp-1 text-lg font-bold text-white">{title || 'Unknown Title'}</h2>
					{#if subtitle}
						<p class="line-clamp-1 text-sm text-white/70">{subtitle}</p>
					{/if}
				</div>
				<div class="flex items-center gap-2">
					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
								<Icon icon="lucide:picture-in-picture-2" class="h-6 w-6" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Picture in Picture</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
								<Icon icon="lucide:settings" class="h-6 w-6" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Settings</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="text-white hover:bg-white/20"
								onclick={onLockToggle}
							>
								<Icon icon="lucide:unlock" class="h-6 w-6" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Lock Controls</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="text-white hover:bg-white/20"
								onclick={onBack}
							>
								<Icon icon="lucide:x" class="h-6 w-6" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Close</p>
						</Tooltip.Content>
					</Tooltip.Root>
				</div>
			</div>

			<!-- Center Controls -->
			<div class="pointer-events-none absolute inset-0 flex items-center justify-center">
				<div class="pointer-events-auto flex items-center gap-12">
					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="h-12 w-12 rounded-full text-white hover:bg-white/20"
								onclick={() => onSeek(Math.max(0, currentTime - 10))}
							>
								<Icon icon="lucide:skip-back" class="h-8 w-8" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Rewind 10s</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="h-16 w-16 rounded-full text-white hover:bg-white/20"
								onclick={onPlayPause}
							>
								{#if isBuffering}
									<Icon icon="lucide:loader-2" class="h-10 w-10 animate-spin" />
								{:else}
									<Icon icon={isPlaying ? 'lucide:pause' : 'lucide:play'} class="h-10 w-10" />
								{/if}
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>{isPlaying ? 'Pause' : 'Play'}</p>
						</Tooltip.Content>
					</Tooltip.Root>

					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="h-12 w-12 rounded-full text-white hover:bg-white/20"
								onclick={() => onSeek(Math.min(duration, currentTime + 10))}
							>
								<Icon icon="lucide:skip-forward" class="h-8 w-8" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Forward 10s</p>
						</Tooltip.Content>
					</Tooltip.Root>
				</div>
			</div>

			<!-- Skip Button (Overlay) -->
			<!-- 
				FIX: We use CSS opacity instead of {#if} here.
				Using {#if} causes the button to mount/unmount at specific timestamps (e.g. 1:30),
				which triggers a layout shift/repaint and causes the video to flicker/stutter.
				Always mounting it and toggling opacity prevents this.
			-->
			<div
				class="absolute right-4 bottom-24 transition-opacity duration-200"
				class:opacity-0={!showSkipIntro || !onSkipIntro}
				class:pointer-events-none={!showSkipIntro || !onSkipIntro}
			>
				<Button
					variant="secondary"
					class="bg-white font-semibold text-black hover:bg-white/90"
					onclick={onSkipIntro}
				>
					Skip Intro
					<Icon icon="lucide:skip-forward" class="ml-2 h-4 w-4" />
				</Button>
			</div>

			<!-- Bottom Controls -->
			<div class="flex w-full flex-col gap-2">
				<div class="flex items-center gap-3 text-sm font-medium text-white">
					<span>{formatTime(currentTime)}</span>
					<span class="text-white/50">/</span>
					<span class="text-white/70">{formatTime(duration)}</span>
				</div>

				<!-- Optimized Native Seek Slider -->
				<div class="relative flex h-4 w-full items-center">
					<input
						type="range"
						min="0"
						max={duration || 1}
						step="0.01"
						value={currentTime}
						oninput={(e) => handleSeek([parseFloat(e.currentTarget.value)])}
						class="w-full cursor-pointer appearance-none rounded-full bg-white/30 outline-none [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white [&::-webkit-slider-thumb]:transition-all hover:[&::-webkit-slider-thumb]:h-4 hover:[&::-webkit-slider-thumb]:w-4"
						style="--seek-pos: {(currentTime / (duration || 1)) *
							100}%; background: linear-gradient(to right, var(--primary) var(--seek-pos), rgba(255, 255, 255, 0.3) var(--seek-pos)); height: 4px;"
					/>
				</div>

				<div class="mt-1 flex items-center justify-between">
					<div class="flex items-center gap-2">
						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
									<Icon icon="lucide:list" class="h-5 w-5" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Playlist</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<div class="flex items-center">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
										<Icon
											icon={volume === 0 ? 'lucide:volume-x' : 'lucide:volume-2'}
											class="h-5 w-5"
										/>
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>{volume === 0 ? 'Unmute' : 'Mute'}</p>
								</Tooltip.Content>
							</Tooltip.Root>

							<!-- Optimized Native Volume Slider -->
							<div class="relative flex h-full w-24 items-center">
								<input
									type="range"
									min="0"
									max="1"
									step="0.01"
									value={volume}
									oninput={(e) => onVolumeChange(parseFloat(e.currentTarget.value))}
									class="w-full cursor-pointer appearance-none rounded-full bg-white/30 outline-none [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white [&::-webkit-slider-thumb]:transition-all hover:[&::-webkit-slider-thumb]:h-4 hover:[&::-webkit-slider-thumb]:w-4"
									style="--vol-pos: {volume *
										100}%; background: linear-gradient(to right, var(--primary) var(--vol-pos), rgba(255, 255, 255, 0.3) var(--vol-pos)); height: 4px;"
								/>
							</div>
						</div>
					</div>

					<div class="flex items-center gap-2">
						<!-- Subtitles Button -->
						<div class="relative">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button
										variant="ghost"
										size="icon"
										class={cn('text-white hover:bg-white/20', showSubtitleMenu && 'bg-white/20')}
										onclick={(e) => {
											e.stopPropagation();
											showSubtitleMenu = !showSubtitleMenu;
										}}
									>
										<Icon icon="lucide:captions" class="h-5 w-5" />
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>Subtitles / Captions</p>
								</Tooltip.Content>
							</Tooltip.Root>

							{#if showSubtitleMenu}
								<div
									class="absolute bottom-full left-1/2 mb-2 w-48 -translate-x-1/2 overflow-hidden rounded-md bg-black/90 p-1 shadow-lg backdrop-blur-sm"
								>
									{#if tracks.length === 0}
										<div class="px-3 py-2 text-center text-sm text-white/50">
											No subtitles available
										</div>
									{:else}
										<button
											class={cn(
												'flex w-full items-center rounded-sm px-3 py-2 text-left text-sm hover:bg-white/10',
												currentTrackIndex === -1 && 'font-medium text-primary'
											)}
											onclick={(e) => {
												e.stopPropagation();
												onTrackChange(-1);
												showSubtitleMenu = false;
											}}
										>
											{#if currentTrackIndex === -1}
												<Icon icon="lucide:check" class="mr-2 h-3 w-3" />
											{:else}
												<div class="mr-2 h-3 w-3"></div>
											{/if}
											<span class="text-white">Off</span>
										</button>
										{#each tracks as track, i}
											<button
												class={cn(
													'flex w-full items-center rounded-sm px-3 py-2 text-left text-sm hover:bg-white/10',
													currentTrackIndex === i && 'font-medium text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													onTrackChange(i);
													showSubtitleMenu = false;
												}}
											>
												{#if currentTrackIndex === i}
													<Icon icon="lucide:check" class="mr-2 h-3 w-3" />
												{:else}
													<div class="mr-2 h-3 w-3"></div>
												{/if}
												<span class="truncate text-white">{track.label}</span>
											</button>
										{/each}
									{/if}
								</div>
							{/if}
						</div>

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
									<Icon icon="lucide:rotate-ccw" class="h-5 w-5" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Reset Speed</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
									<Icon icon="lucide:cast" class="h-5 w-5" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Cast</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button variant="ghost" size="icon" class="text-white hover:bg-white/20">
									<Icon icon="lucide:picture-in-picture" class="h-5 w-5" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Mini Player</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button
									variant="ghost"
									size="icon"
									class="text-white hover:bg-white/20"
									onclick={onFullscreen}
								>
									<Icon icon="lucide:maximize" class="h-5 w-5" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Fullscreen</p>
							</Tooltip.Content>
						</Tooltip.Root>
					</div>
				</div>
			</div>
		{/if}
	</div>
</Tooltip.Provider>
