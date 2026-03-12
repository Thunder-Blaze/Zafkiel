<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api/core';
	import Icon from '@iconify/svelte';
	import { onDestroy, onMount } from 'svelte';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { Button } from '$lib/components/ui/button';
	import { extensionStore } from '$lib/stores/extensionStore.svelte';
	import { ExtensionLoader } from '$lib/services/ExtensionLoader';
	import { EXTENSION_CATALOG } from '$lib/services/extensionCatalog';
	import { useQueryClient } from '@tanstack/svelte-query';
	import type {
		SourceExtension,
		SearchResult,
		Episode,
		StreamSource,
		ResolvedStream,
		StartDownloadParams,
		ExtensionDownload,
		DownloadProgressEvent,
	} from '$lib/types/extensions';
	import VideoPlayer from '$lib/components/player/VideoPlayer.svelte';
	import InternalPlayer from '$lib/components/player/InternalPlayer.svelte';
	import ProxiedImage from '$lib/components/ProxiedImage.svelte';

	const { animeTitle, animeId }: { animeTitle: string; animeId: number } = $props();

	const CACHE_STALE_TIME = 5 * 60 * 1000; // 5 minutes
	const queryClient = useQueryClient();

	// ── Player Mode ───────────────────────────────────────────────────────────
	type PlayerMode = 'internal' | 'libmpv' | 'external';
	let playerMode = $state<PlayerMode>(
		(localStorage.getItem('zafkiel-player-mode') as PlayerMode | null) ?? 'libmpv'
	);

	$effect(() => {
		localStorage.setItem('zafkiel-player-mode', playerMode);
	});

	// ── State ──────────────────────────────────────────────────────────────────
	type Step =
		| 'init'
		| 'loading-ext'
		| 'checking-auth'
		| 'auth-needed'
		| 'searching'
		| 'results'
		| 'episodes'
		| 'loading-sources'
		| 'sources'
		| 'playing';

	let step = $state<Step>('init');
	let error = $state<string | null>(null);
	let activeExtId = $state<string | null>(null);
	let ext = $state<SourceExtension | null>(null);

	// Intentionally captures initial prop value as local mutable search input
	let searchQuery = $state((() => animeTitle)());
	let searchResults = $state<SearchResult[]>([]);
	let selectedResult = $state<SearchResult | null>(null);

	let episodes = $state<Episode[]>([]);
	let episodePage = $state(1);
	let episodeTotalPages = $state(1);
	let loadingEpisodes = $state(false);

	let selectedEpisode = $state<Episode | null>(null);
	let sources = $state<StreamSource[]>([]);
	let resolvedStream = $state<ResolvedStream | null>(null);
	let activeSource = $state<StreamSource | null>(null);
	let loadingStream = $state(false);

	// ── Download state ────────────────────────────────────────────────────────
	/** Map of sourceId → download record (in-progress or completed). */
	let downloadMap = $state<Map<string, ExtensionDownload>>(new Map());
	let downloadingSourceId = $state<string | null>(null);
	let downloadUnlisten: (() => void) | null = null;

	let cookieUnlisten: (() => void) | null = null;
	/** Raw "name=value; ..." cookie string for proxied image requests. */
	let cookieStr = $state<string | null>(null);

	// ── Derived ───────────────────────────────────────────────────────────────
	const sourceExtensions = $derived(
		EXTENSION_CATALOG.filter((c) => c.type === 'source' && extensionStore.isInstalled(c.id))
	);

	// ── On mount — pick the first installed source extension ──────────────────
	onMount(async () => {
		// Grab the local HLS proxy port so we can bypass CDN CORS restrictions.
		try {
			// NOTE: hlsProxyPort is no longer used — the InternalPlayer handles CORS
			// bypassing via a custom hls.js loader (invoke → fetch_url / fetch_bytes_base64).
			// This block is kept as a no-op reference and can be removed safely.
		} catch (e) {
			console.warn('[watch] note: HLS proxy is not used for InternalPlayer', e);
		}

		if (sourceExtensions.length === 0) return;
		await loadExt(sourceExtensions[0].id);

		// Listen for cookies from the auth webview
		cookieUnlisten = await listen<string>(`${activeExtId}-auth-cookies-ready`, async (event) => {
			const rawCookies = event.payload;
			console.debug(`[watch] raw cookies received (${rawCookies.length} chars):`, rawCookies);

			// Persist for proxied image requests (CDN requires Cookie header)
			cookieStr = rawCookies;

			if (ext?.onCookiesUpdated) {
				const parsed = rawCookies
					.split(';')
					.map((c) => c.trim())
					.filter(Boolean)
					.map((part) => {
						const [name, ...rest] = part.split('=');
						return { name: name.trim(), value: rest.join('=').trim() };
					});
				console.debug(
					`[watch] parsed ${parsed.length} cookies:`,
					parsed.map((c) => c.name)
				);
				// Await so WASM set_cookies() runs before doSearch
				await ext.onCookiesUpdated(parsed);
				console.debug('[watch] onCookiesUpdated complete, starting search');
			}
			step = 'searching';
			await doSearch(searchQuery);
		});

		// Listen for download progress events
		downloadUnlisten = await listen<DownloadProgressEvent>(
			'extension-download-progress',
			(event) => {
				const ev = event.payload;
				downloadMap = new Map(
					[...downloadMap.entries()].map(([key, rec]) => [
						key,
						rec.id === ev.id
							? {
									...rec,
									status: ev.status as ExtensionDownload['status'],
									progress: ev.progress,
									errorMsg: ev.errorMsg,
									filePath: ev.filePath,
								}
							: rec,
					])
				);
				if (ev.status !== 'downloading') {
					downloadingSourceId = null;
				}
			}
		);
	});

	onDestroy(() => {
		cookieUnlisten?.();
		downloadUnlisten?.();
	});

	// ── Helpers ───────────────────────────────────────────────────────────────
	async function loadExt(id: string) {
		activeExtId = id;
		step = 'loading-ext';
		error = null;

		const status = extensionStore.getStatus(id);
		if (status.kind !== 'installed' && status.kind !== 'ready') {
			error = `Extension "${id}" is not installed.`;
			step = 'init';
			return;
		}

		try {
			if (status.kind === 'installed') {
				await extensionStore.loadExtension(id);
			}
			ext = ExtensionLoader.get(id);
			if (!ext) throw new Error('Extension failed to load');
		} catch (e) {
			error = String(e);
			step = 'init';
			return;
		}

		// Check auth
		step = 'checking-auth';
		try {
			const authed = ext.checkAuth ? await ext.checkAuth() : true;
			if (!authed) {
				step = 'auth-needed';
				return;
			}
		} catch {
			step = 'auth-needed';
			return;
		}

		step = 'searching';
		await doSearch(searchQuery);
	}

	/** Build a localhost proxy URL for an HLS stream. */
	function buildHlsProxySrc(url: string, _headers: Record<string, string> = {}): string {
		// The HLS proxy server has been removed. CORS bypass for the InternalPlayer
		// is handled by the custom Tauri loader inside InternalPlayer.svelte.
		// This helper is kept only for the external player path which passes
		// headers directly to mpv instead.
		return url;
	}

	/**
	 * Lazily ensures the HLS proxy port is known.
	 * @deprecated The HLS proxy server has been removed. No-op, always returns true.
	 */
	async function ensureHlsProxyPort(): Promise<boolean> {
		return true;
	}

	async function openAuthWebview() {
		if (!activeExtId) return;
		try {
			await invoke('open_extension_auth_webview', {
				windowLabel: `${activeExtId}-auth`,
				url: 'https://animepahe.si',
				title: 'Sign in / Complete Challenge',
			});
		} catch (e) {
			error = `Failed to open auth window: ${e}`;
		}
	}

	async function doSearch(q: string) {
		if (!ext) return;
		step = 'searching';
		error = null;
		try {
			searchResults = await queryClient.fetchQuery({
				queryKey: ['ext-search', activeExtId, q],
				queryFn: () => ext!.search(q),
				staleTime: CACHE_STALE_TIME,
			});
			// Auto-select if only one result
			if (searchResults.length === 1) {
				await selectResult(searchResults[0]);
				return;
			}
			step = 'results';
		} catch (e) {
			error = String(e);
			step = 'results';
		}
	}

	async function selectResult(result: SearchResult) {
		if (!ext) return;
		selectedResult = result;
		episodes = [];
		episodePage = 1;
		loadingEpisodes = true;
		step = 'episodes';
		await loadEpisodePage(1);
	}

	async function loadEpisodePage(page: number) {
		if (!ext || !selectedResult) return;
		loadingEpisodes = true;
		try {
			// `id` on SearchResult from AnimePahe is the session slug
			const pageData = await queryClient.fetchQuery({
				queryKey: ['ext-episodes', activeExtId, selectedResult.id, page],
				queryFn: () => ext!.getEpisodes(selectedResult!.id, page),
				staleTime: CACHE_STALE_TIME,
			});
			if (page === 1) episodes = pageData.data;
			else episodes = [...episodes, ...pageData.data];
			episodePage = pageData.currentPage;
			episodeTotalPages = pageData.lastPage;
		} catch (e) {
			error = String(e);
		} finally {
			loadingEpisodes = false;
		}
	}

	async function playEpisode(ep: Episode) {
		if (!ext || !selectedResult) return;
		selectedEpisode = ep;
		sources = [];
		resolvedStream = null;
		step = 'loading-sources';
		try {
			sources = await ext.getStreamSources(selectedResult.id, ep.id);
			step = 'sources';
		} catch (e) {
			error = String(e);
			step = 'episodes';
		}
	}

	async function playSource(source: StreamSource, mode: PlayerMode = playerMode) {
		if (!ext) return;
		playerMode = mode;
		loadingStream = true;
		error = null;
		try {
			// Browser and External players both route through the HLS proxy.
			// Ensure the port is known before resolving the stream so we never
			// fall back to a raw CDN URL that will be blocked by CORS.
			if (mode === 'internal' || mode === 'external') {
				const ready = await ensureHlsProxyPort();
				if (!ready && mode === 'internal') {
					error =
						'HLS proxy server is not available. Try Libmpv or External player instead.';
					return;
				}
			}

			// Resolve the stream URL — fetches the kwik embed page and extracts
			// the packed m3u8 URL (e.g. vault-XX.owocdn.top/.../uwu.m3u8).
			// The proxy handles all auth headers (Referer, Cookie) transparently.
			const resolved = source.requiresResolution
				? await ext.resolveStream(source)
				: { url: source.id, type: 'hls' as const, headers: {} as Record<string, string> };

			if (mode === 'external') {
				// Open directly in the system's external player (mpv CLI).
				// Use the HLS proxy URL so the external player receives auth headers
				// (Referer, Cookie) transparently via the localhost proxy.
				const proxyUrl = buildHlsProxySrc(resolved.url, resolved.headers ?? {});
				await invoke('open_in_external_player', { url: proxyUrl });
				// Stay on the sources step — don't switch to playing.
				return;
			}

			resolvedStream = resolved;
			activeSource = source;
			step = 'playing';
		} catch (e) {
			error = String(e);
		} finally {
			loadingStream = false;
		}
	}

	/** Called when user picks an episode from the player's playlist panel. */
	async function handlePlayerEpisodeSelect(ep: Episode) {
		if (!ext || !selectedResult) return;
		selectedEpisode = ep;
		sources = [];
		activeSource = null;
		resolvedStream = null;
		try {
			sources = await ext.getStreamSources(selectedResult.id, ep.id);
			// Auto-play the first source
			if (sources.length > 0) {
				await playSource(sources[0]);
			} else {
				step = 'sources';
			}
		} catch (e) {
			error = String(e);
			step = 'episodes';
		}
	}

	/** Called when user picks a quality from the player's quality menu. */
	async function handlePlayerSourceSelect(source: StreamSource) {
		await playSource(source);
	}

	function reset() {
		step = 'results';
		resolvedStream = null;
		selectedEpisode = null;
		sources = [];
		error = null;
	}

	// ── Download ──────────────────────────────────────────────────────────────
	async function downloadSource(source: StreamSource) {
		if (!ext || !selectedResult || !selectedEpisode || downloadingSourceId) return;
		downloadingSourceId = source.id;
		error = null;

		try {
			// Reuse the same resolve logic as playback
			const resolved = source.requiresResolution
				? await ext.resolveStream(source)
				: { url: source.id, type: 'hls' as const, headers: {} as Record<string, string> };

			const season = 1; // AnimePahe doesn't expose seasons — default to 1
			const sourceLabel = [
				source.fansub,
				source.resolution ? `${source.resolution}p` : null,
				source.audio === 'jpn' ? 'JPN' : source.audio === 'eng' ? 'DUB' : source.audio,
			]
				.filter(Boolean)
				.join(' ');

			const params: StartDownloadParams = {
				animeName: selectedResult.title,
				anilistId: animeId,
				season,
				episodeNumber: selectedEpisode.number,
				sourceLabel: sourceLabel || source.label,
				extensionId: activeExtId ?? 'unknown',
				url: resolved.url,
				headers: resolved.headers ?? {},
			};

			const downloadId = await invoke<number>('start_extension_download', { params });
			const placeholder: ExtensionDownload = {
				id: downloadId,
				animeName: params.animeName,
				anilistId: params.anilistId,
				season: params.season,
				episodeNumber: params.episodeNumber,
				sourceLabel: params.sourceLabel,
				extensionId: params.extensionId,
				status: 'downloading',
				progress: 0,
				createdAt: Date.now() / 1000,
				updatedAt: Date.now() / 1000,
			};
			downloadMap = new Map([...downloadMap, [source.id, placeholder]]);
		} catch (e) {
			error = String(e);
			downloadingSourceId = null;
		}
	}
</script>

<div class="space-y-4">
	<!-- No extensions installed -->
	{#if sourceExtensions.length === 0}
		<div class="flex flex-col items-center gap-3 rounded-lg border border-dashed p-10 text-center">
			<Icon icon="solar:tv-bold-duotone" class="size-12 text-muted-foreground/50" />
			<h3 class="font-semibold">No source extensions installed</h3>
			<p class="max-w-sm text-sm text-muted-foreground">
				Install a source extension (e.g. AnimePahe) from the Extensions page to stream anime.
			</p>
			<Button variant="outline" href="/extensions">Go to Extensions</Button>
		</div>
	{/if}

	<!-- Loading extension -->
	{#if step === 'loading-ext' || step === 'checking-auth'}
		<div class="flex items-center justify-center gap-3 py-12 text-muted-foreground">
			<Icon icon="solar:refresh-circle-line-duotone" class="size-6 animate-spin" />
			<span class="text-sm"
				>{step === 'loading-ext' ? 'Loading extension…' : 'Checking authentication…'}</span
			>
		</div>
	{/if}

	<!-- Auth needed -->
	{#if step === 'auth-needed'}
		<div
			class="flex flex-col items-center gap-4 rounded-lg border border-amber-500/30 bg-amber-500/10 p-8 text-center"
		>
			<Icon icon="solar:shield-warning-bold-duotone" class="size-10 text-amber-500" />
			<div>
				<h3 class="font-semibold">Authentication Required</h3>
				<p class="mt-1 max-w-sm text-sm text-muted-foreground">
					AnimePahe requires completing a Cloudflare challenge once. A browser window will open —
					complete the check, then come back.
				</p>
			</div>
			<Button onclick={openAuthWebview}>
				<Icon icon="solar:global-bold" class="mr-2 size-4" />
				Open Challenge Window
			</Button>
		</div>
	{/if}

	<!-- Searching -->
	{#if step === 'searching'}
		<div class="flex items-center justify-center gap-3 py-12 text-muted-foreground">
			<Icon icon="solar:magnifer-line-duotone" class="size-6 animate-pulse" />
			<span class="text-sm">Searching for "{searchQuery}"…</span>
		</div>
	{/if}

	<!-- Search results -->
	{#if step === 'results'}
		<div class="space-y-3">
			<div class="flex items-center gap-2">
				<input
					type="text"
					class="flex-1 rounded-md border bg-background px-3 py-2 text-sm"
					bind:value={searchQuery}
					onkeydown={(e) => e.key === 'Enter' && doSearch(searchQuery)}
					placeholder="Search by title…"
				/>
				<Button variant="outline" onclick={() => doSearch(searchQuery)}>
					<Icon icon="solar:magnifer-bold" class="size-4" />
				</Button>
			</div>

			{#if error}
				<p class="rounded-md bg-destructive/10 px-3 py-2 text-sm text-destructive">{error}</p>
			{/if}

			{#if searchResults.length === 0 && !error}
				<p class="py-8 text-center text-sm text-muted-foreground">No results found.</p>
			{:else}
				<div class="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
					{#each searchResults as result}
						<button
							class="group flex flex-col rounded-lg border bg-card p-2 transition-all duration-200 hover:border-primary/40 hover:shadow-lg hover:shadow-primary/5"
							onclick={() => selectResult(result)}
						>
							<div class="relative overflow-hidden rounded-md">
								{#if result.coverUrl}
									<ProxiedImage
										src={result.coverUrl}
										alt={result.title}
										class="aspect-[2/3] w-full object-cover"
										cookie={cookieStr}
										referer="https://animepahe.si/"
									/>
								{:else}
									<div class="flex aspect-[2/3] w-full items-center justify-center rounded-md bg-muted">
										<Icon icon="solar:tv-bold" class="size-10 text-muted-foreground" />
									</div>
								{/if}
								<!-- Hover overlay -->
								<div class="absolute inset-0 flex items-center justify-center bg-black/0 transition-all duration-200 group-hover:bg-black/40">
									<Icon icon="solar:play-bold" class="size-8 text-white opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
								</div>
								{#if result.type}
									<span class="absolute top-1.5 right-1.5 rounded bg-black/70 px-1.5 py-0.5 text-[9px] font-semibold text-white/80">
										{result.type}
									</span>
								{/if}
							</div>
							<div class="pt-2">
								<p class="line-clamp-2 text-xs font-medium">{result.title}</p>
								{#if result.year || result.status}
									<p class="mt-1 flex items-center gap-1.5 text-[10px] text-muted-foreground">
										{#if result.year}<span>{result.year}</span>{/if}
										{#if result.year && result.status}<span class="text-muted-foreground/40">·</span>{/if}
										{#if result.status}<span>{result.status}</span>{/if}
									</p>
								{/if}
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Episodes list -->
	{#if step === 'episodes' || step === 'loading-sources' || step === 'sources' || step === 'playing'}
		<div class="space-y-3">
			<!-- Header with back -->
			<div class="flex items-center gap-2">
				<Button
					variant="ghost"
					size="sm"
					onclick={() => {
						step = 'results';
						selectedResult = null;
					}}
				>
					<Icon icon="solar:arrow-left-bold" class="mr-1 size-4" />
					Back
				</Button>
				{#if selectedResult}
					<span class="text-sm font-medium">{selectedResult.title}</span>
				{/if}
			</div>

			{#if error}
				<p class="rounded-md bg-destructive/10 px-3 py-2 text-sm text-destructive">{error}</p>
			{/if}
		</div>
	{/if}

	<!-- Video player -->
	{#if step === 'playing' && resolvedStream}
		<div>
			{#if playerMode === 'libmpv'}
				<VideoPlayer
					url={resolvedStream.url}
					headers={resolvedStream.headers ?? {}}
					title={selectedResult?.title}
					subtitle={selectedEpisode?.title ?? `Episode ${selectedEpisode?.number}`}
					onBack={reset}
					{episodes}
					currentEpisode={selectedEpisode}
					{sources}
					currentSource={activeSource}
					onEpisodeSelect={handlePlayerEpisodeSelect}
					onSourceSelect={handlePlayerSourceSelect}
				/>
			{:else}
				<InternalPlayer
					src={resolvedStream.url}
					headers={resolvedStream.headers ?? {}}
					title={selectedResult?.title}
					subtitle={selectedEpisode?.title ?? `Episode ${selectedEpisode?.number}`}
					onBack={reset}
				/>
			{/if}
			<div class="flex items-center justify-between text-sm text-muted-foreground">
				<span>
					{selectedResult?.title} — Ep {selectedEpisode?.number}
					{#if selectedEpisode?.title}
						· {selectedEpisode.title}{/if}
				</span>
				<Button variant="ghost" size="sm" onclick={reset}>
					<Icon icon="solar:list-bold" class="mr-1 size-4" />
					Episodes
				</Button>
			</div>
		</div>
	{/if}

	<!-- Sources picker -->
	{#if step === 'sources'}
		<div class="space-y-3">
			<p class="text-sm font-medium">
				Ep {selectedEpisode?.number}{selectedEpisode?.title ? ` — ${selectedEpisode.title}` : ''} — Choose
				Quality
			</p>
			<div class="space-y-2">
				{#each sources as source}
					{@const dl = downloadMap.get(source.id)}
					<div class="flex items-center gap-2 rounded-lg border bg-card px-3 py-2">
						<!-- Quality label -->
						<span class="flex-1 text-sm">
							{source.fansub ? `[${source.fansub}] ` : ''}{source.resolution
								? `${source.resolution}p`
								: source.label}
							{#if source.audio === 'jpn'}<span class="ml-1 text-[10px] text-muted-foreground"
									>JPN</span
								>{/if}
							{#if source.audio === 'eng'}<span class="ml-1 text-[10px] text-muted-foreground"
									>DUB</span
								>{/if}
						</span>

						<!-- Download status badge / progress -->
						{#if dl}
							{#if dl.status === 'downloading'}
								<span class="flex items-center gap-1 text-xs text-blue-500">
									<Icon icon="solar:download-minimalistic-bold" class="size-3.5" />
									{dl.progress.toFixed(0)}%
								</span>
							{:else if dl.status === 'completed'}
								<span class="flex items-center gap-1 text-xs text-green-500">
									<Icon icon="solar:check-circle-bold" class="size-3.5" />
									Saved
								</span>
							{:else if dl.status === 'failed'}
								<span class="flex items-center gap-1 text-xs text-destructive" title={dl.errorMsg}>
									<Icon icon="solar:close-circle-bold" class="size-3.5" />
									Failed
								</span>
							{/if}
						{/if}

						<!-- 3 play buttons: Internal · Libmpv · External -->
						{#if loadingStream && playerMode !== 'external'}
							<span class="flex items-center gap-1 text-xs text-muted-foreground">
								<Icon icon="solar:refresh-circle-line-duotone" class="size-3.5 animate-spin" />
								Loading…
							</span>
						{:else}
							<Button
								variant="outline"
								size="sm"
								disabled={loadingStream}
								onclick={() => playSource(source, 'internal')}
								title="Play in Browser (hls.js)"
							>
								<Icon icon="solar:monitor-smartphone-bold-duotone" class="mr-1 size-3.5" />
								Browser
							</Button>
							<Button
								variant="default"
								size="sm"
								disabled={loadingStream}
								onclick={() => playSource(source, 'libmpv')}
								title="Play in Libmpv (hardware-accelerated)"
							>
								<Icon icon="solar:play-circle-bold-duotone" class="mr-1 size-3.5" />
								Libmpv
							</Button>
							<Button
								variant="secondary"
								size="sm"
								disabled={loadingStream}
								onclick={() => playSource(source, 'external')}
								title="Open in external player (mpv)"
							>
								<Icon icon="solar:export-bold-duotone" class="mr-1 size-3.5" />
								External
							</Button>
						{/if}

						<!-- Download button -->
						<Button
							variant="ghost"
							size="icon"
							class="size-8 shrink-0"
							disabled={!!downloadingSourceId ||
								dl?.status === 'downloading' ||
								dl?.status === 'completed'}
							onclick={() => downloadSource(source)}
							title="Download episode"
						>
							{#if downloadingSourceId === source.id}
								<Icon icon="solar:refresh-circle-line-duotone" class="size-3.5 animate-spin" />
							{:else}
								<Icon icon="solar:download-minimalistic-bold" class="size-3.5" />
							{/if}
						</Button>
					</div>
				{/each}
			</div>
			<Button
				variant="ghost"
				size="sm"
				onclick={() => {
					step = 'episodes';
					selectedEpisode = null;
				}}
			>
				Back to episodes
			</Button>
		</div>
	{/if}

	<!-- Episodes loading / list -->
	{#if (step === 'episodes' || step === 'loading-sources') && selectedResult}
		{#if loadingEpisodes && episodes.length === 0}
			<div class="flex items-center justify-center gap-3 py-12 text-muted-foreground">
				<Icon icon="solar:refresh-circle-line-duotone" class="size-5 animate-spin" />
				<span class="text-sm">Loading episodes…</span>
			</div>
		{:else}
			<ScrollArea class="h-[28rem]">
				<div class="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
					{#each episodes as ep}
						<button
							class="group flex flex-col rounded-lg border bg-card p-2 transition-all duration-200 hover:border-primary/40 hover:shadow-lg
								{selectedEpisode?.id === ep.id ? 'border-primary ring-1 ring-primary/30' : ''}"
							onclick={() => playEpisode(ep)}
						>
							<div class="relative overflow-hidden rounded-md">
								{#if ep.thumbnailUrl}
									<ProxiedImage
										src={ep.thumbnailUrl}
										alt={`Ep ${ep.number}`}
										class="aspect-video w-full object-cover"
										cookie={cookieStr}
										referer="https://animepahe.si/"
									/>
								{:else}
									<div class="flex aspect-video w-full items-center justify-center rounded-md bg-muted">
										<Icon icon="solar:play-bold" class="size-6 text-muted-foreground/40" />
									</div>
								{/if}
								<!-- Episode number badge -->
								<span class="absolute bottom-1.5 left-1.5 rounded bg-black/70 px-1.5 py-0.5 text-[10px] font-bold text-white">
									EP {ep.number}
								</span>
								{#if selectedEpisode?.id === ep.id}
									<div class="absolute inset-0 flex items-center justify-center rounded-md bg-black/40">
										<Icon icon="solar:play-bold" class="size-6 text-primary" />
									</div>
								{/if}
								<!-- Hover overlay -->
								<div class="absolute inset-0 flex items-center justify-center rounded-md bg-black/0 transition-all duration-200 group-hover:bg-black/30">
									<Icon icon="solar:play-bold" class="size-6 text-white opacity-0 transition-opacity duration-200 group-hover:opacity-100" />
								</div>
							</div>
							{#if ep.title}
								<div class="pt-1.5">
									<p class="line-clamp-1 text-[11px] font-medium">{ep.title}</p>
								</div>
							{/if}
						</button>
					{/each}
				</div>

				{#if episodePage < episodeTotalPages}
					<div class="mt-4 flex justify-center">
						<Button
							variant="outline"
							size="sm"
							disabled={loadingEpisodes}
							onclick={() => loadEpisodePage(episodePage + 1)}
						>
							{loadingEpisodes ? 'Loading…' : 'Load more'}
						</Button>
					</div>
				{/if}
			</ScrollArea>
		{/if}
	{/if}
</div>
