<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { cn } from '$lib/utils';
	import ProxiedImage from '$lib/components/ProxiedImage.svelte';
	import type { Episode, StreamSource } from '$lib/types/extensions';
	import { useConfigState } from '$lib/stores/config.svelte';
	import { parseSourceLabel } from '$lib/utils/source-parser';
	import { fly, fade, scale } from 'svelte/transition';

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
		playbackSpeed = 1,
		onSpeedChange = () => {},
		episodes = [],
		currentEpisode = null,
		sources = [],
		currentSource = null,
		onEpisodeSelect,
		onSourceSelect,
		onOverlayToggle,
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
		playbackSpeed?: number;
		onSpeedChange?: (speed: number) => void;
		episodes?: Episode[];
		currentEpisode?: Episode | null;
		sources?: StreamSource[];
		currentSource?: StreamSource | null;
		onEpisodeSelect?: (ep: Episode) => void;
		onSourceSelect?: (src: StreamSource) => void;
		onOverlayToggle?: (open: boolean) => void;
	}>();

	const config = useConfigState();

	let showSubtitleMenu = $state(false);
	let showPlaylist = $state(false);
	let showQualityMenu = $state(false);
	let showShaderMenu = $state(false);
	let showSpeedMenu = $state(false);
	let showRemainingTime = $state(false);

	let availableShaders = $state<string[]>([]);

	$effect(() => {
		config.getAvailableShaders().then((s) => {
			availableShaders = s;
		});
	});

	// Notify parent when any overlay is open so it can prevent controls timeout
	$effect(() => {
		const anyOpen =
			showPlaylist || showSubtitleMenu || showQualityMenu || showShaderMenu || showSpeedMenu;
		onOverlayToggle?.(anyOpen);
	});

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
		const meta = parseSourceLabel({
			label: source.label,
			fansub: source.fansub,
			resolution: source.resolution,
			audio: source.audio,
		});

		const parts: string[] = [];
		if (meta.source) parts.push(`[${meta.source}]`);
		if (meta.quality) parts.push(meta.quality);
		if (meta.language === 'dub') parts.push('DUB');
		else if (meta.language === 'sub') parts.push('SUB');

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
		class={cn(
			'pointer-events-none absolute top-0 right-0 left-0 z-40 h-48 bg-gradient-to-b from-background/80 via-background/20 to-transparent',
			isLocked && 'hidden'
		)}
	></div>
	<div
		class={cn(
			'pointer-events-none absolute right-0 bottom-0 left-0 z-40 h-80 bg-gradient-to-t from-background/90 via-background/40 to-transparent',
			isLocked && 'hidden'
		)}
	></div>

	<!-- Strong gradients overlay (paused state) -->
	<div
		class={cn(
			'pointer-events-none absolute top-0 right-0 left-0 z-40 h-48 bg-gradient-to-b from-background via-background/60 to-transparent transition-opacity duration-300',
			isPlaying ? 'opacity-0' : 'opacity-60',
			isLocked && 'hidden'
		)}
	></div>
	<div
		class={cn(
			'pointer-events-none absolute right-0 bottom-0 left-0 z-40 h-80 bg-gradient-to-t from-background via-background/75 to-transparent transition-opacity duration-300',
			isPlaying ? 'opacity-0' : 'opacity-60',
			isLocked && 'hidden'
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
							class="h-10 w-10 rounded-full p-0 text-foreground drop-shadow-[0_0_8px_hsl(var(--background))] backdrop-blur-sm hover:bg-foreground/20 md:h-12 md:w-12"
							onclick={onLockToggle}
						>
							<Icon
								icon="mingcute:lock-fill"
								class="h-8 w-8 drop-shadow-[0_0_4px_hsl(var(--background))] md:h-10 md:w-10"
							/>
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
						<!-- Visual Thumb (Dot) -->
						<div
							class="pointer-events-none absolute z-20 h-4 w-4 rounded-full bg-primary opacity-0 shadow-lg transition-all group-hover/seekbar:opacity-100 md:h-5 md:w-5"
							style="left: {(currentTime / (duration || 1)) * 100}%; transform: translateX(-50%);"
						></div>
						<!-- Slider -->
						<input
							type="range"
							min="0"
							max={duration || 1}
							step="0.01"
							value={currentTime}
							oninput={(e) => handleSeek([parseFloat(e.currentTarget.value)])}
							class="absolute inset-x-0 z-30 h-full w-full cursor-pointer appearance-none bg-transparent outline-none focus:outline-none [&::-webkit-slider-thumb]:h-0 [&::-webkit-slider-thumb]:w-0 [&::-webkit-slider-thumb]:appearance-none"
						/>
					</div>

					<!-- Time Display -->
					<div
						class="pointer-events-auto flex shrink-0 justify-end text-xs font-semibold tracking-wide text-foreground/90 md:text-sm"
					>
						<button
							class="flex cursor-pointer items-center justify-end whitespace-nowrap transition-colors hover:text-white"
							onclick={(e) => {
								e.stopPropagation();
								showRemainingTime = !showRemainingTime;
							}}
						>
							{#if showRemainingTime}
								<span>-{formatTime(Math.max(0, duration - currentTime))}</span>
							{:else}
								<span>{formatTime(currentTime)}</span>
								<span class="mx-1 text-foreground/50">/</span>
								<span class="text-foreground/70">{formatTime(duration)}</span>
							{/if}
						</button>
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
										showShaderMenu = false;
										showSpeedMenu = false;
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
											showShaderMenu = false;
											showSpeedMenu = false;
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
								<div
									class="absolute right-0 bottom-full z-50 mb-4 flex w-52 origin-bottom-right flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl"
									transition:scale={{ duration: 150, start: 0.95, opacity: 0 }}
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
												showShaderMenu = false;
												showSpeedMenu = false;
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
										class="absolute right-0 bottom-full z-50 mb-4 flex w-60 origin-bottom-right flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl"
										transition:scale={{ duration: 150, start: 0.95, opacity: 0 }}
									>
										<div
											class="mb-1 shrink-0 border-b border-border px-3 py-2 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
										>
											Quality settings
										</div>
										<div
											class="max-h-[300px] w-full overflow-y-auto pr-1"
											data-lenis-prevent="true"
											onwheel={(e) => e.stopPropagation()}
										>
											<!-- Auto-select Next Stream Toggle -->
											<button
												class={cn(
													'mb-1 flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
													config.autoSelectNextStream && 'bg-primary/5 text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													config.setAutoSelectNextStream(!config.autoSelectNextStream);
												}}
											>
												<Icon
													icon={config.autoSelectNextStream
														? 'mingcute:check-circle-fill'
														: 'mingcute:circle-line'}
													class={cn(
														'mr-2.5 h-4 w-4',
														config.autoSelectNextStream ? 'text-primary' : 'text-muted-foreground'
													)}
												/>
												<span class="flex-1 font-medium">Auto-select Next</span>
											</button>

											<div class="mx-2 my-1 h-px bg-border/50"></div>

											{#each sources as source}
												{@const meta = parseSourceLabel({
													label: source.label,
													fansub: source.fansub,
													resolution: source.resolution,
													audio: source.audio,
												})}
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
													<div class="flex min-w-0 flex-1 items-center gap-2">
														{#if meta.source}
															<span
																class="max-w-[80px] truncate rounded bg-muted px-1.5 py-0.5 text-[10px] font-bold text-foreground uppercase"
															>
																{meta.source}
															</span>
														{/if}
														{#if meta.quality}
															<span
																class="rounded bg-primary/10 px-1.5 py-0.5 text-[10px] font-bold text-primary"
															>
																{meta.quality}
															</span>
														{/if}
														{#if meta.language === 'dub'}
															<span
																class="rounded bg-amber-500/10 px-1.5 py-0.5 text-[10px] font-bold text-amber-500"
															>
																DUB
															</span>
														{:else if meta.language === 'sub'}
															<span
																class="rounded bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-bold text-emerald-500"
															>
																SUB
															</span>
														{/if}
														{#if !meta.source && !meta.quality && !meta.language}
															<span class="truncate">{source.label || source.id}</span>
														{/if}
													</div>
												</button>
											{/each}
										</div>
									</div>
								{/if}
							</div>
						{/if}

						<div class="relative">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button
										variant="ghost"
										size="icon"
										class={cn(
											'h-8 w-8 p-0 text-foreground transition-colors hover:bg-foreground/20 md:h-10 md:w-10',
											showShaderMenu && 'bg-foreground/20 text-primary'
										)}
										onclick={(e) => {
											e.stopPropagation();
											showShaderMenu = !showShaderMenu;
											showPlaylist = false;
											showQualityMenu = false;
											showSubtitleMenu = false;
											showSpeedMenu = false;
										}}
									>
										<Icon icon="mingcute:magic-2-fill" class="h-7 w-7 md:h-9 md:w-9" />
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>Video Shaders</p>
								</Tooltip.Content>
							</Tooltip.Root>

							{#if showShaderMenu}
								<div
									class="absolute right-0 bottom-full z-50 mb-4 flex w-80 origin-bottom-right flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl"
									transition:scale={{ duration: 150, start: 0.95, opacity: 0 }}
								>
									<div
										class="mb-1 shrink-0 border-b border-border px-3 py-2 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
									>
										Shaders
									</div>
									<div
										class="max-h-[300px] w-full overflow-y-auto pr-1"
										data-lenis-prevent="true"
										onwheel={(e) => e.stopPropagation()}
									>
										<button
											class={cn(
												'mb-1 flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
												config.shaderConfig.enabled && 'bg-primary/5 text-primary'
											)}
											onclick={(e) => {
												e.stopPropagation();
												config.setShaderConfig(
													!config.shaderConfig.enabled,
													config.shaderConfig.selected_shaders
												);
											}}
										>
											<Icon
												icon={config.shaderConfig.enabled
													? 'mingcute:toggle-right-fill'
													: 'mingcute:toggle-left-line'}
												class={cn(
													'mr-2.5 h-5 w-5',
													config.shaderConfig.enabled ? 'text-primary' : 'text-muted-foreground'
												)}
											/>
											<span class="flex-1 font-medium">Enable Shaders</span>
										</button>

										<div class="mx-2 my-1 h-px bg-border/50"></div>

										{#each config.shaderConfig.selected_shaders as shader}
											{@const shaderName = shader.split('/').pop()?.replace('.glsl', '') || shader}
											<button
												class={cn(
													'flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
													'bg-primary/10 font-medium text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													let selected = [...config.shaderConfig.selected_shaders];
													selected = selected.filter((s) => s !== shader);
													config.setShaderConfig(config.shaderConfig.enabled, selected);
												}}
											>
												<Icon
													icon="mingcute:check-fill"
													class="mr-2.5 h-4 w-4 shrink-0 text-primary"
												/>
												<span class="break-all">{shaderName}</span>
											</button>
										{/each}

										<div class="my-1.5 px-3 text-[10px] font-bold text-muted-foreground uppercase">
											Available
										</div>

										{#each availableShaders.filter((s) => !config.shaderConfig.selected_shaders.includes(s)) as shader}
											{@const shaderName = shader.split('/').pop()?.replace('.glsl', '') || shader}
											<button
												class="flex w-full items-center rounded-lg px-3 py-2 text-left text-sm transition-colors hover:bg-muted"
												onclick={(e) => {
													e.stopPropagation();
													let selected = [...config.shaderConfig.selected_shaders, shader];
													config.setShaderConfig(config.shaderConfig.enabled, selected);
												}}
											>
												<div class="mr-2.5 h-4 w-4 shrink-0"></div>
												<span class="break-all text-foreground/80">{shaderName}</span>
											</button>
										{/each}
									</div>
								</div>
							{/if}
						</div>

						<div class="relative">
							<Tooltip.Root>
								<Tooltip.Trigger>
									<Button
										variant="ghost"
										size="icon"
										class={cn(
											'h-8 w-8 min-w-[2.5rem] p-0 text-foreground transition-colors hover:bg-foreground/20 md:h-10 md:w-10',
											showSpeedMenu && 'bg-foreground/20 text-primary'
										)}
										onclick={(e) => {
											e.stopPropagation();
											showSpeedMenu = !showSpeedMenu;
											showPlaylist = false;
											showQualityMenu = false;
											showSubtitleMenu = false;
											showShaderMenu = false;
										}}
									>
										<div class="pl-0 text-xs font-bold md:text-sm">{playbackSpeed}x</div>
									</Button>
								</Tooltip.Trigger>
								<Tooltip.Content>
									<p>Playback Speed</p>
								</Tooltip.Content>
							</Tooltip.Root>

							{#if showSpeedMenu}
								<div
									class="absolute right-0 bottom-full z-50 mb-4 flex w-32 origin-bottom-right flex-col rounded-xl border border-border bg-background/95 p-1.5 shadow-2xl backdrop-blur-xl"
									transition:scale={{ duration: 150, start: 0.95, opacity: 0 }}
								>
									<div
										class="mb-1 shrink-0 border-b border-border px-3 py-2 text-xs font-semibold tracking-wider text-muted-foreground uppercase"
									>
										Speed
									</div>
									<div
										class="max-h-[300px] w-full overflow-y-auto pr-1"
										data-lenis-prevent="true"
										onwheel={(e) => e.stopPropagation()}
									>
										{#each [0.5, 0.75, 1, 1.25, 1.5, 1.75, 2, 3, 4] as speed}
											<button
												class={cn(
													'flex w-full items-center rounded-lg px-3 py-2.5 text-left text-sm transition-colors hover:bg-muted',
													playbackSpeed === speed && 'bg-primary/10 font-medium text-primary'
												)}
												onclick={(e) => {
													e.stopPropagation();
													onSpeedChange(speed);
													showSpeedMenu = false;
												}}
											>
												{#if playbackSpeed === speed}
													<Icon
														icon="mingcute:check-fill"
														class="mr-2.5 h-4 w-4 shrink-0 text-primary"
													/>
												{:else}
													<div class="mr-2.5 h-4 w-4 shrink-0"></div>
												{/if}
												<span class="truncate">{speed}x</span>
											</button>
										{/each}
									</div>
								</div>
							{/if}
						</div>

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
		<!-- Backdrop for closing when clicking outside -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="absolute inset-0 z-[55] bg-black/20 backdrop-blur-sm"
			transition:fade={{ duration: 200 }}
			onclick={() => (showPlaylist = false)}
		></div>

		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="pointer-events-auto absolute top-0 right-0 z-[60] flex h-full w-full flex-col border-l border-border bg-background/90 shadow-2xl backdrop-blur-3xl sm:w-[28rem]"
			transition:fly={{ x: 400, duration: 250 }}
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
