<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { cn } from '$lib/utils';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { Episode, StreamSource } from '$lib/types/extensions';

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
		episodes = [],
		currentEpisode = null,
		sources = [],
		currentSource = null,
		onEpisodeSelect,
		onSourceSelect,
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
		episodes?: Episode[];
		currentEpisode?: Episode | null;
		sources?: StreamSource[];
		currentSource?: StreamSource | null;
		onEpisodeSelect?: (ep: Episode) => void;
		onSourceSelect?: (src: StreamSource) => void;
	}>();

	let showSubtitleMenu = $state(false);
	let showPlaylist = $state(false);
	let showQualityMenu = $state(false);

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

	function formatSourceLabel(source: StreamSource): string {
		const parts: string[] = [];
		if (source.fansub) parts.push(`[${source.fansub}]`);
		if (source.resolution) parts.push(`${source.resolution}p`);
		else if (source.label) parts.push(source.label);
		if (source.audio === 'jpn') parts.push('JPN');
		else if (source.audio === 'eng') parts.push('DUB');
		else if (source.audio) parts.push(source.audio.toUpperCase());
		return parts.join(' ') || source.label || source.id;
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
								<Button
									variant="ghost"
									size="icon"
									class={cn('text-white hover:bg-white/20', showPlaylist && 'bg-white/20')}
									onclick={(e) => {
										e.stopPropagation();
										showPlaylist = !showPlaylist;
										showQualityMenu = false;
										showSubtitleMenu = false;
									}}
									disabled={episodes.length === 0}
								>
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

						<!-- Quality / Source Switcher -->
						{#if sources.length > 0}
							<div class="relative">
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button
											variant="ghost"
											size="icon"
											class={cn('text-white hover:bg-white/20', showQualityMenu && 'bg-white/20')}
											onclick={(e) => {
												e.stopPropagation();
												showQualityMenu = !showQualityMenu;
												showSubtitleMenu = false;
												showPlaylist = false;
											}}
										>
											<Icon icon="lucide:settings-2" class="h-5 w-5" />
										</Button>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Quality</p>
									</Tooltip.Content>
								</Tooltip.Root>

								{#if showQualityMenu}
									<div
										class="absolute bottom-full left-1/2 mb-2 w-56 -translate-x-1/2 overflow-hidden rounded-md bg-black/90 p-1 shadow-lg backdrop-blur-sm"
									>
										<div class="border-b border-white/10 px-3 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-white/40">
											Quality
										</div>
										{#each sources as source}
											<button
												class={cn(
													'flex w-full items-center rounded-sm px-3 py-2 text-left text-sm hover:bg-white/10',
													currentSource?.id === source.id && 'font-medium text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													onSourceSelect?.(source);
													showQualityMenu = false;
												}}
											>
												{#if currentSource?.id === source.id}
													<Icon icon="lucide:check" class="mr-2 h-3 w-3" />
												{:else}
													<div class="mr-2 h-3 w-3"></div>
												{/if}
												<span class="truncate text-white">{formatSourceLabel(source)}</span>
											</button>
										{/each}
									</div>
								{/if}
							</div>
						{/if}

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

	<!-- Playlist Side Panel -->
	{#if showPlaylist && episodes.length > 0}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="absolute top-0 right-0 z-50 flex h-full w-[22rem] flex-col border-l border-white/10 bg-black/95 backdrop-blur-sm"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex items-center justify-between border-b border-white/10 px-4 py-3">
				<div>
					<h3 class="text-sm font-bold text-white">Episodes</h3>
					<p class="text-[10px] text-white/40">{episodes.length} episodes</p>
				</div>
				<Button
					variant="ghost"
					size="icon"
					class="h-7 w-7 text-white/50 hover:bg-white/10 hover:text-white"
					onclick={() => (showPlaylist = false)}
				>
					<Icon icon="lucide:x" class="h-4 w-4" />
				</Button>
			</div>
			<ScrollArea class="flex-1">
				<div class="flex flex-col gap-0.5 p-1.5">
					{#each episodes as ep (ep.id)}
						{@const isCurrent = currentEpisode?.id === ep.id}
						<button
							class={cn(
								'flex w-full items-center gap-3 rounded-md px-2 py-1.5 text-left transition-colors',
								isCurrent
									? 'bg-primary/20 text-primary'
									: 'text-white/70 hover:bg-white/10 hover:text-white'
							)}
							onclick={(e) => {
								e.stopPropagation();
								onEpisodeSelect?.(ep);
							}}
						>
							<!-- Thumbnail -->
							<div class="relative h-12 w-20 shrink-0 overflow-hidden rounded">
								{#if ep.thumbnailUrl}
									<img
										src={ep.thumbnailUrl}
										alt={`Ep ${ep.number}`}
										class="h-full w-full object-cover"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center bg-white/5">
										<Icon icon="lucide:play" class="h-3 w-3 text-white/20" />
									</div>
								{/if}
								{#if isCurrent}
									<div class="absolute inset-0 flex items-center justify-center bg-black/50">
										<Icon icon="lucide:play" class="h-4 w-4 text-primary" />
									</div>
								{/if}
							</div>
							<!-- Info -->
							<div class="min-w-0 flex-1">
								<p class="text-xs font-medium">
									Episode {ep.number}
								</p>
								{#if ep.title}
									<p class="line-clamp-1 text-[10px] opacity-60">{ep.title}</p>
								{/if}
							</div>
						</button>
					{/each}
				</div>
			</ScrollArea>
		</div>
	{/if}
</Tooltip.Provider>
