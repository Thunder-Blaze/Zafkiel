<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { cn } from '$lib/utils';
	import ProxiedImage from '$lib/components/ProxiedImage.svelte';
	import type { Episode, StreamSource } from '$lib/types/extensions';

	let {
		isPlaying,
		currentTime = 0,
		duration = 0,
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

	function truncateString(str: string | undefined, num: number): string {
		if (!str) return '';
		if (str.length <= num) {
			return str;
		}
		return str.slice(0, num) + '...';
	}
</script>

<Tooltip.Provider>
	<!-- Subtle gradients base (playing state) -->
	<div
		class="pointer-events-none absolute top-0 right-0 left-0 z-40 h-48 bg-gradient-to-b from-background/80 via-background/20 to-transparent"
	></div>
	<div
		class="pointer-events-none absolute right-0 bottom-0 left-0 z-40 h-80 bg-gradient-to-t from-background/90 via-background/40 to-transparent"
	></div>

	<!-- Strong gradients overlay (paused state) -->
	<div
		class={cn(
			'pointer-events-none absolute top-0 right-0 left-0 z-40 h-48 bg-gradient-to-b from-background via-background/60 to-transparent transition-opacity duration-300',
			isPlaying ? 'opacity-0' : 'opacity-60'
		)}
	></div>
	<div
		class={cn(
			'pointer-events-none absolute right-0 bottom-0 left-0 z-40 h-80 bg-gradient-to-t from-background via-background/75 to-transparent transition-opacity duration-300',
			isPlaying ? 'opacity-0' : 'opacity-60'
		)}
	></div>

	<!-- Reduced outer padding to p-4 md:p-6 -->
	<div
		class="absolute inset-0 z-50 flex flex-col justify-between p-4 pb-6 md:p-6"
		data-lenis-prevent="true"
		onwheel={(e) => {
			if (isLocked) e.stopPropagation();
		}}
	>
		<!-- Locked State Overlay -->
		{#if isLocked}
			<div class="pointer-events-auto absolute inset-0 flex flex-col items-end justify-start p-4">
				<Tooltip.Root>
					<Tooltip.Trigger>
						<Button
							variant="ghost"
							size="icon"
							class="h-10 w-10 p-0 text-foreground hover:bg-foreground/20 md:h-12 md:w-12"
							onclick={onLockToggle}
						>
							<Icon icon="mingcute:lock-fill" class="h-8 w-8 md:h-10 md:w-10" />
						</Button>
					</Tooltip.Trigger>
					<Tooltip.Content>
						<p>Unlock Controls</p>
					</Tooltip.Content>
				</Tooltip.Root>
			</div>
		{:else}
			<!-- Top Bar -->
			<div class="pointer-events-none flex items-start justify-between">
				<!-- Just the Back Button on top left -->
				<div class="pointer-events-auto">
					<Tooltip.Root>
						<Tooltip.Trigger>
							<Button
								variant="ghost"
								size="icon"
								class="h-10 w-10 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-12 md:w-12"
								onclick={onBack}
							>
								<Icon icon="mingcute:arrow-left-fill" class="aspect-square w-full" />
							</Button>
						</Tooltip.Trigger>
						<Tooltip.Content>
							<p>Back</p>
						</Tooltip.Content>
					</Tooltip.Root>
				</div>
			</div>

			<!-- Center Content (Clickable area to play/pause in middle of screen) -->
			<!-- Removed focus borders and outline. Negative tabindex added -->
			<button
				type="button"
				class="pointer-events-auto flex-1 cursor-pointer appearance-none border-none bg-transparent ring-0 outline-none focus:ring-0 focus:outline-none"
				onclick={onPlayPause}
				tabindex="-1"
				aria-label="Toggle Playback"
			></button>

			<!-- Bottom Content Area -->
			<div
				class="pointer-events-auto flex w-full flex-col justify-end gap-3 drop-shadow-lg md:gap-4"
			>
				<!-- Title & Info Space (Only visible when paused) -->
				<div
					class="pointer-events-none mb-2 flex h-[5rem] flex-col justify-end px-2 transition-opacity duration-300 md:mb-4 md:h-[6rem]"
					class:opacity-0={isPlaying}
				>
					<h2
						class="truncate pb-1 text-3xl font-extrabold tracking-tight text-foreground uppercase drop-shadow-md md:text-5xl"
					>
						{title || 'Unknown Title'}
					</h2>
					{#if subtitle}
						<p class="mt-1 truncate text-base font-medium text-foreground/80 md:mt-2 md:text-xl">
							{subtitle}
						</p>
					{/if}
				</div>

				<!-- Progress Bar and Time -->
				<div class="flex w-full items-center gap-3 px-1 md:gap-4">
					<div
						class="group/seekbar relative flex h-5 flex-1 cursor-pointer items-center"
						onwheel={(e) => e.stopPropagation()}
					>
						<!-- Background track -->
						<div
							class="pointer-events-none absolute inset-x-0 z-0 h-1.5 rounded-full bg-white/20 md:h-2"
						></div>
						<!-- Progress Fill -->
						<div
							class="pointer-events-none absolute left-0 z-0 h-1.5 rounded-full bg-primary md:h-2"
							style="width: {(currentTime / (duration || 1)) * 100}%;"
						></div>
						<!-- Slider -->
						<input
							type="range"
							min="0"
							max={duration || 1}
							step="0.01"
							value={currentTime}
							oninput={(e) => handleSeek([parseFloat(e.currentTarget.value)])}
						class="absolute inset-0 z-10 h-full w-full cursor-pointer appearance-none bg-transparent outline-none focus:outline-none [&::-webkit-slider-thumb]:h-0 [&::-webkit-slider-thumb]:w-0 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-primary [&::-webkit-slider-thumb]:shadow-lg [&::-webkit-slider-thumb]:transition-all group-hover/seekbar:[&::-webkit-slider-thumb]:h-4 group-hover/seekbar:[&::-webkit-slider-thumb]:w-4 md:group-hover/seekbar:[&::-webkit-slider-thumb]:h-5 md:group-hover/seekbar:[&::-webkit-slider-thumb]:w-5"
						/>
					</div>

					<!-- Time Display -->
					<div
						class="pointer-events-none flex shrink-0 justify-end text-xs font-semibold tracking-wide text-foreground/90 md:text-sm"
					>
						<span>{formatTime(currentTime)}</span>
						<span class="mx-1 text-foreground/50">/</span>
						<span class="text-foreground/70">{formatTime(duration)}</span>
					</div>
				</div>

				<!-- Control Bar -->
				<div class="mt-1 flex items-center justify-between px-1 md:mt-2">
					<!-- Left Controls -->
					<div class="flex items-center gap-1.5 md:gap-3">
						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button
									variant="ghost"
									size="icon"
									class="h-10 w-10 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-90 md:h-12 md:w-12"
									onclick={onPlayPause}
								>
									{#if isBuffering}
										<Icon
											icon="mingcute:loading-fill"
											class="h-8 w-8 animate-spin md:h-10 md:w-10"
										/>
									{:else}
										<Icon
											icon={isPlaying ? 'mingcute:pause-fill' : 'mingcute:play-fill'}
											class="h-8 w-8 md:h-10 md:w-10"
										/>
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
									class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
									onclick={() => onSeek(Math.max(0, currentTime - 10))}
								>
									<!-- using rewind and forward fills -->
									<Icon icon="mingcute:fast-rewind-fill" class="h-7 w-7 md:h-9 md:w-9" />
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
									class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
									onclick={() => onSeek(Math.min(duration, currentTime + 10))}
								>
									<Icon icon="mingcute:fast-forward-fill" class="h-7 w-7 md:h-9 md:w-9" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Forward 10s</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<!-- Volume Slider -->
						<div class="group/volume ml-1 flex items-center gap-1 md:ml-2">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button
										variant="ghost"
										size="icon"
										class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
										onclick={() => onVolumeChange(volume === 0 ? 1 : 0)}
									>
										<Icon
											icon={volume === 0 ? 'mingcute:volume-mute-fill' : 'mingcute:volume-fill'}
											class="h-7 w-7 md:h-9 md:w-9"
										/>
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>{volume === 0 ? 'Unmute' : 'Mute'}</p>
								</Tooltip.Content>
							</Tooltip.Root>

							<div
							class="relative flex h-5 w-0 cursor-pointer items-center overflow-hidden opacity-0 transition-all duration-300 group-hover/volume:w-24 group-hover/volume:opacity-100 md:group-hover/volume:w-28"
								onwheel={(e) => e.stopPropagation()}
							>
								<div
									class="pointer-events-none absolute inset-x-0 z-0 h-1.5 rounded-full bg-white/20"
								></div>
								<div
									class="pointer-events-none absolute left-0 z-0 h-1.5 rounded-full bg-primary"
									style="width: {volume * 100}%;"
								></div>
								<input
									type="range"
									min="0"
									max="1"
									step="0.01"
									value={volume}
									oninput={(e) => onVolumeChange(parseFloat(e.currentTarget.value))}
									class="absolute inset-0 z-10 h-full w-full cursor-pointer appearance-none bg-transparent outline-none focus:outline-none [&::-webkit-slider-thumb]:h-0 [&::-webkit-slider-thumb]:w-0 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-foreground hover:[&::-webkit-slider-thumb]:h-3 hover:[&::-webkit-slider-thumb]:w-3 md:hover:[&::-webkit-slider-thumb]:h-4 md:hover:[&::-webkit-slider-thumb]:w-4"
								/>
							</div>
						</div>
					</div>

					<!-- Right Controls -->
					<div class="flex items-center gap-1.5 md:gap-3">
						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button
									variant="ghost"
									size="icon"
									class={cn(
										'h-8 w-8 p-0 text-foreground transition-colors hover:bg-foreground/20 md:h-10 md:w-10',
										showPlaylist && 'bg-foreground/20 text-primary'
									)}
									onclick={(e) => {
										e.stopPropagation();
										showPlaylist = !showPlaylist;
										showQualityMenu = false;
										showSubtitleMenu = false;
									}}
									disabled={episodes.length === 0}
								>
									<Icon icon="mingcute:playlist-2-fill" class="h-7 w-7 md:h-9 md:w-9" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Episodes</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<div class="relative">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button
										variant="ghost"
										size="icon"
										class={cn(
											'h-8 w-8 p-0 text-foreground transition-colors hover:bg-foreground/20 md:h-10 md:w-10',
											showSubtitleMenu && 'bg-foreground/20 text-primary'
										)}
										onclick={(e) => {
											e.stopPropagation();
											showSubtitleMenu = !showSubtitleMenu;
											showPlaylist = false;
											showQualityMenu = false;
										}}
									>
										<!-- mingcute doesn't have a specific subtitle CC. Usually closed caption -->
										<Icon icon="mingcute:subtitle-fill" class="h-7 w-7 md:h-9 md:w-9" />
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>Subtitles</p>
								</Tooltip.Content>
							</Tooltip.Root>

							{#if showSubtitleMenu}
								<!-- Replaced ScrollArea with a native overflow div to perfectly match lenis requirements -->
								<div
									class="absolute right-0 bottom-full z-50 mb-4 flex w-52 origin-bottom-right animate-in flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl duration-200 zoom-in-95 fade-in"
								>
									<div
										class="mb-1 shrink-0 border-b border-border px-3 py-2 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
									>
										Subtitles
									</div>
									<div
										class="max-h-[300px] w-full overflow-y-auto pr-1"
										data-lenis-prevent="true"
										onwheel={(e) => e.stopPropagation()}
									>
										{#if tracks.length === 0}
											<div class="px-3 py-3 text-center text-sm text-muted-foreground">
												No subtitles
											</div>
										{:else}
											<button
												class={cn(
													'flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
													currentTrackIndex === -1 && 'bg-primary/10 font-medium text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													onTrackChange(-1);
													showSubtitleMenu = false;
												}}
											>
												{#if currentTrackIndex === -1}
													<Icon icon="mingcute:check-fill" class="mr-2.5 h-4 w-4 text-primary" />
												{:else}
													<div class="mr-2.5 h-4 w-4"></div>
												{/if}
												<span class="truncate">Off</span>
											</button>
											{#each tracks as track, i}
												<button
													class={cn(
														'flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
														currentTrackIndex === i && 'bg-primary/10 font-medium text-primary'
													)}
													onclick={(e) => {
														e.stopPropagation();
														onTrackChange(i);
														showSubtitleMenu = false;
													}}
												>
													{#if currentTrackIndex === i}
														<Icon icon="mingcute:check-fill" class="mr-2.5 h-4 w-4 text-primary" />
													{:else}
														<div class="mr-2.5 h-4 w-4"></div>
													{/if}
													<span class="truncate">{track.label}</span>
												</button>
											{/each}
										{/if}
									</div>
								</div>
							{/if}
						</div>

						{#if sources.length > 0}
							<div class="relative">
								<Tooltip.Root>
									<Tooltip.Trigger>
										<Button
											variant="ghost"
											size="icon"
											class={cn(
												'h-8 w-8 p-0 text-foreground transition-colors hover:bg-foreground/20 md:h-10 md:w-10',
												showQualityMenu && 'bg-foreground/20 text-primary'
											)}
											onclick={(e) => {
												e.stopPropagation();
												showQualityMenu = !showQualityMenu;
												showSubtitleMenu = false;
												showPlaylist = false;
											}}
										>
											<Icon icon="mingcute:settings-1-fill" class="h-7 w-7 md:h-9 md:w-9" />
										</Button>
									</Tooltip.Trigger>
									<Tooltip.Content>
										<p>Quality Settings</p>
									</Tooltip.Content>
								</Tooltip.Root>

								{#if showQualityMenu}
									<div
										class="absolute right-0 bottom-full z-50 mb-4 flex w-60 origin-bottom-right animate-in flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl duration-200 zoom-in-95 fade-in"
									>
										<div
											class="mb-1 shrink-0 border-b border-border px-3 py-2 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
										>
											Quality settings
										</div>
										<!-- Swapped to native div overflow to fix lenis scroll locking issues -->
										<div
											class="max-h-[300px] w-full overflow-y-auto pr-1"
											data-lenis-prevent="true"
											onwheel={(e) => e.stopPropagation()}
										>
											{#each sources as source}
												<button
													class={cn(
														'flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
														currentSource?.id === source.id &&
															'bg-primary/10 font-medium text-primary'
													)}
													onclick={(e) => {
														e.stopPropagation();
														onSourceSelect?.(source);
														showQualityMenu = false;
													}}
												>
													{#if currentSource?.id === source.id}
														<Icon icon="mingcute:check-fill" class="mr-2.5 h-4 w-4 text-primary" />
													{:else}
														<div class="mr-2.5 h-4 w-4"></div>
													{/if}
													<span class="truncate">{formatSourceLabel(source)}</span>
												</button>
											{/each}
										</div>
									</div>
								{/if}
							</div>
						{/if}

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button
									variant="ghost"
									size="icon"
									class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
								>
									<Icon icon="mingcute:miniplayer-fill" class="h-7 w-7 md:h-9 md:w-9" />
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
									class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
									onclick={onFullscreen}
								>
									<Icon icon="mingcute:fullscreen-2-fill" class="h-7 w-7 md:h-9 md:w-9" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Fullscreen</p>
							</Tooltip.Content>
						</Tooltip.Root>

						<Tooltip.Root>
							<Tooltip.Trigger>
								<Button
									variant="ghost"
									size="icon"
									class="h-8 w-8 p-0 text-foreground transition-transform hover:bg-foreground/20 active:scale-95 md:h-10 md:w-10"
									onclick={onLockToggle}
								>
									<Icon icon="mingcute:unlock-fill" class="h-7 w-7 md:h-9 md:w-9" />
								</Button>
							</Tooltip.Trigger>
							<Tooltip.Content>
								<p>Lock Controls</p>
							</Tooltip.Content>
						</Tooltip.Root>
					</div>
				</div>
			</div>

			<!-- Skip Button (Overlay) placed in bottom right above controls -->
			<div
				class="absolute right-4 bottom-40 z-50 drop-shadow-2xl transition-all duration-300 md:right-6 md:bottom-48"
				class:opacity-0={!showSkipIntro || !onSkipIntro}
				class:scale-95={!showSkipIntro || !onSkipIntro}
				class:pointer-events-none={!showSkipIntro || !onSkipIntro}
			>
				<Button
					variant="secondary"
					class="rounded-full bg-foreground px-6 py-5 text-sm font-bold text-background shadow-xl transition-transform hover:scale-105 hover:bg-foreground/90 active:scale-95 md:px-8 md:py-6 md:text-base"
					onclick={onSkipIntro}
				>
					Skip Intro
					<Icon icon="mingcute:skip-forward-fill" class="ml-2.5 h-6 w-6" />
				</Button>
			</div>
		{/if}
	</div>

	<!-- Playlist Side Panel (Apple TV style width and backdrop) -->
	{#if showPlaylist && episodes.length > 0}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="pointer-events-auto absolute top-0 right-0 z-[60] flex h-full w-full animate-in flex-col border-l border-border bg-background/90 shadow-2xl backdrop-blur-3xl duration-200 slide-in-from-right sm:w-[28rem]"
			onclick={(e) => e.stopPropagation()}
			data-lenis-prevent="true"
			onwheel={(e) => e.stopPropagation()}
		>
			<div class="flex items-center justify-between border-b border-border/50 px-6 py-5">
				<div>
					<h3 class="text-2xl font-bold tracking-tight text-foreground">Episodes</h3>
					<p class="text-sm text-foreground/60">{episodes.length} episodes</p>
				</div>
				<Button
					variant="ghost"
					size="icon"
					class="h-10 w-10 rounded-full p-0 text-foreground/70 transition-colors hover:bg-foreground/10 hover:text-foreground"
					onclick={() => (showPlaylist = false)}
				>
					<Icon icon="mingcute:close-fill" class="h-8 w-8" />
				</Button>
			</div>

			<!-- Replaced ScrollArea with standard div for lenis compatibility -->
			<div
				class="w-full flex-1 overflow-y-auto bg-transparent"
				data-lenis-prevent="true"
				onwheel={(e) => e.stopPropagation()}
			>
				<div class="flex flex-col gap-1.5 p-4 pr-6">
					{#each episodes as ep (ep.id)}
						{@const isCurrent = currentEpisode?.id === ep.id}
						<button
							class={cn(
								'group flex w-full items-center gap-4 rounded-xl px-3 py-3 text-left transition-all duration-200',
								isCurrent ? 'bg-primary/10 ring-1 ring-primary/30' : 'hover:bg-foreground/10'
							)}
							onclick={(e) => {
								e.stopPropagation();
								onEpisodeSelect?.(ep);
							}}
						>
							<!-- Thumbnail -->
							<div
								class={cn(
									'relative h-20 w-[8.5rem] shrink-0 overflow-hidden rounded-[10px] bg-background/40 shadow-sm transition-transform duration-300',
									isCurrent && 'ring-2 ring-primary ring-offset-2 ring-offset-background',
									!isCurrent && 'group-hover:scale-105 group-hover:shadow-md'
								)}
							>
								{#if ep.thumbnailUrl}
									<ProxiedImage
										src={ep.thumbnailUrl}
										alt={`Ep ${ep.number}`}
										class="h-full w-full object-cover"
									/>
								{:else}
									<div class="flex h-full w-full items-center justify-center bg-foreground/10">
										<Icon icon="mingcute:play-fill" class="h-8 w-8 text-foreground/40" />
									</div>
								{/if}
								{#if isCurrent}
									<div
										class="absolute inset-0 flex items-center justify-center bg-black/40 backdrop-blur-[2px]"
									>
										<Icon icon="mingcute:play-fill" class="h-8 w-8 text-primary shadow-2xl" />
									</div>
								{/if}
							</div>
							<!-- Info -->
							<div class="min-w-0 flex-1">
								<p
									class={cn(
										'text-lg leading-tight font-bold drop-shadow-sm',
										isCurrent ? 'text-primary' : 'text-foreground'
									)}
								>
									Episode {ep.number}
								</p>
								{#if ep.title}
									<p
										class="mt-1 text-[13px] font-medium break-words text-foreground/70"
										title={ep.title}
									>
										{truncateString(ep.title, 60)}
									</p>
								{/if}
							</div>
						</button>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</Tooltip.Provider>
