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
	import ProxiedImage from '$lib/components/ProxiedImage.svelte';

	const { animeTitle, animeId }: { animeTitle: string; animeId: number } = $props();

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
	let loadingStream = $state(false);

	// ── Download state ────────────────────────────────────────────────────────
	/** Map of sourceId → download record (in-progress or completed). */
	let downloadMap = $state<Map<string, ExtensionDownload>>(new Map());
	let downloadingSourceId = $state<string | null>(null);
	let downloadUnlisten: (() => void) | null = null;

	let cookieUnlisten: (() => void) | null = null;
	/** Raw "name=value; ..." cookie string for proxied image requests. */
	let cookieStr = $state<string | null>(null);
	/** Port of the local HLS proxy server (started by Tauri backend). */
	let hlsProxyPort = $state<number | null>(null);

	// ── Derived ───────────────────────────────────────────────────────────────
	const sourceExtensions = $derived(
		EXTENSION_CATALOG.filter((c) => c.type === 'source' && extensionStore.isInstalled(c.id))
	);

	// ── On mount — pick the first installed source extension ──────────────────
	onMount(async () => {
		// Grab the local HLS proxy port so we can bypass CDN CORS restrictions.
		try {
			hlsProxyPort = await invoke<number>('get_hls_proxy_port');
			console.debug('[watch] HLS proxy port:', hlsProxyPort);
		} catch (e) {
			console.warn('[watch] failed to get HLS proxy port — stream may fail:', e);
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
	function buildHlsProxySrc(url: string, headers: Record<string, string> = {}): string {
		if (!hlsProxyPort) {
			// Proxy not ready — fall back to direct URL (will likely CORS-fail for CDN streams)
			console.warn('[watch] buildHlsProxySrc called before proxy port is known');
			return url;
		}
		const params = new URLSearchParams({ url });
		if (headers['Referer']) params.set('referer', headers['Referer']);
		if (headers['referer']) params.set('referer', headers['referer']);
		// Prefer explicit Cookie header from extension, fall back to collected session cookies
		const cookieVal = headers['Cookie'] ?? headers['cookie'] ?? cookieStr ?? undefined;
		if (cookieVal) params.set('cookie', cookieVal);
		console.debug(
			'[watch] proxy URL:',
			`http://127.0.0.1:${hlsProxyPort}/proxy?${params.toString()}`
		);
		return `http://127.0.0.1:${hlsProxyPort}/proxy?${params.toString()}`;
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
			searchResults = await ext.search(q);
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
			const pageData = await ext.getEpisodes(selectedResult.id, page);
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

	async function playSource(source: StreamSource) {
		if (!ext) return;
		loadingStream = true;
		error = null;
		try {
			// Resolve the stream URL — fetches the kwik embed page and extracts
			// the packed m3u8 URL (e.g. vault-XX.owocdn.top/.../uwu.m3u8).
			// The proxy handles all auth headers (Referer, Cookie) transparently.
			const resolved = source.requiresResolution
				? await ext.resolveStream(source)
				: { url: source.id, type: 'hls' as const, headers: {} as Record<string, string> };

			resolvedStream = resolved;
			step = 'playing';
		} catch (e) {
			error = String(e);
		} finally {
			loadingStream = false;
		}
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
				<div class="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4">
					{#each searchResults as result}
						<button
							class="group flex flex-col overflow-hidden rounded-lg border bg-card transition-shadow hover:shadow-md"
							onclick={() => selectResult(result)}
						>
							{#if result.coverUrl}
								<ProxiedImage
									src={result.coverUrl}
									alt={result.title}
									class="aspect-[2/3] w-full object-cover"
									cookie={cookieStr}
									referer="https://animepahe.si/"
								/>
							{:else}
								<div class="flex aspect-[2/3] w-full items-center justify-center bg-muted">
									<Icon icon="solar:tv-bold" class="size-10 text-muted-foreground" />
								</div>
							{/if}
							<div class="p-2">
								<p class="line-clamp-2 text-xs font-medium">{result.title}</p>
								{#if result.year}
									<p class="mt-0.5 text-[10px] text-muted-foreground">{result.year}</p>
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
		<div class="space-y-3">
			<VideoPlayer
				url={resolvedStream.url}
				headers={resolvedStream.headers ?? {}}
				title={selectedResult?.title}
				subtitle={selectedEpisode?.title ?? `Episode ${selectedEpisode?.number}`}
				onBack={reset}
			/>
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

						<!-- Play button -->
						<Button
							variant="default"
							size="sm"
							disabled={loadingStream}
							onclick={() => playSource(source)}
						>
							{#if loadingStream}
								<Icon icon="solar:refresh-circle-line-duotone" class="mr-1 size-3.5 animate-spin" />
							{:else}
								<Icon icon="solar:play-bold" class="mr-1 size-3.5" />
							{/if}
							Play
						</Button>

						<!-- Download button -->
						<Button
							variant="outline"
							size="sm"
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
			<ScrollArea class="h-96">
				<div class="grid grid-cols-4 gap-2 sm:grid-cols-6 md:grid-cols-8">
					{#each episodes as ep}
						<button
							class="flex flex-col items-center gap-1 rounded-md border bg-card p-2 text-center transition-colors hover:bg-muted
								{selectedEpisode?.id === ep.id ? 'border-primary bg-primary/10' : ''}"
							onclick={() => playEpisode(ep)}
						>
							{#if ep.thumbnailUrl}
								<ProxiedImage
									src={ep.thumbnailUrl}
									alt={`Ep ${ep.number}`}
									class="aspect-video w-full rounded object-cover"
									cookie={cookieStr}
									referer="https://animepahe.si/"
								/>
							{/if}
							<span class="text-xs font-semibold">{ep.number}</span>
							{#if ep.title}
								<span class="line-clamp-1 text-[10px] text-muted-foreground">{ep.title}</span>
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
