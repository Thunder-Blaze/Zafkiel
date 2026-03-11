# AnimePahe Extension – Implementation Plan

This document covers the full implementation plan for the first real **source extension**:
**AnimePahe** (`id: "animepahe-wasm"`, type: `"source"`).

---

## AnimePahe API Surface (Reverse-Engineered from Zenshin)

AnimePahe is a Cloudflare-protected site. All API calls require session cookies.

### Public API endpoints (JSON)

| Method   | URL                                        | Description                           |
| -------- | ------------------------------------------ | ------------------------------------- |
| Search   | `GET /api?m=search&q={query}`              | Search anime, returns `{data: []}`    |
| Airing   | `GET /api?m=airing&page={n}`               | Currently airing list                 |
| Episodes | `GET /api?m=release&id={animeId}&page={n}` | Episode list for an anime (paginated) |

### HTML-scraped pages

| Page         | URL                               | What to scrape                                                                     |
| ------------ | --------------------------------- | ---------------------------------------------------------------------------------- |
| Anime detail | `GET /anime/{slugId}`             | Title, poster, synopsis, AniList ID                                                |
| Play page    | `GET /play/{animeId}/{episodeId}` | Kwik.si/cx buttons with `data-src`, `data-fansub`, `data-resolution`, `data-audio` |

### Image proxies

| Resource | URL                                                                              |
| -------- | -------------------------------------------------------------------------------- |
| Poster   | `https://i.animepahe.si/posters/{filename}`                                      |
| Snapshot | `https://i.animepahe.si/snapshots/{filename}` or `/uploads/snapshots/{filename}` |

---

## Kwik Video Resolution (2-step obfuscation)

AnimePahe streams through **Kwik.si** (sometimes **kwik.cx**).
The resolution requires two HTTP hops + a JS deobfuscation step:

```
Step 1: fetch(kwik_url, { Referer: "https://animepahe.si" })
        → returns HTML with eval(f(...)) packed script

Step 2: decode the packed script (custom P,A,C,K,E,D unpacker)
        → extract: source='https://na-*.kwik.si/hls/...'

Final URL is an HLS (.m3u8) stream.
```

The **unpacker** (`common/unpacker.js` in zenshin) must be ported to Rust or reproduced in the WASM extension.

---

## WASM vs JS Decision for AnimePahe

**Decision: Start with a TypeScript (JS) extension, upgrade to WASM later.**

Reasoning:

- The Kwik unpacker is already a known algorithm; porting it to TypeScript is trivial
- WASM build chain (wasm-pack, CI) adds overhead for the first extension
- The performance-critical path is network I/O, not WASM CPU computation
- Once the TypeScript extension is proven correct, the Rust/WASM version can be a 1:1 port

The TypeScript extension lives at:

```
src/lib/extensions/source/animepahe/
├── index.ts           ← AnimePaheExtension class (implements SourceExtension)
├── api.ts             ← raw API fetchers
├── unpacker.ts        ← P,A,C,K,E,D decoder (from zenshin common/unpacker.js)
├── cookie-auth.ts     ← cookie storage + validation
└── types.ts           ← provider-internal types
```

---

## P,A,C,K,E,D Unpacker (TypeScript Port)

Based on the zenshin `common/unpacker.js`:

```typescript
// unpacker.ts
export function unpackEval(packedString: string): string {
	const evalExtract = packedString.match(/eval\((.+)\)\)/s);
	if (!evalExtract) return '';

	// Dean Edwards P,A,C,K,E,D unpacker logic
	const p = (
		p: string,
		a: number,
		c: number,
		k: string[],
		e: (n: number) => string,
		d: Record<string, string>
	) => {
		// Replace numeric tokens with dictionary lookups
		return p.replace(/\b\w+\b/g, (word) => {
			const n = parseInt(word, a);
			return d[n] || word;
		});
	};

	// Evaluate the packed expression in a safe manner
	// Extract p, a, c, k, e, d parameters from the string
	const match = packedString.match(/\('(.+)',(\d+),(\d+),'(.+)'\.split\('\|'\)/);
	if (!match) return '';

	const [, pVal, aVal, cVal, kVal] = match;
	const dict = kVal.split('|');

	return pVal.replace(/\b\w+\b/g, (word) => {
		const n = parseInt(word, parseInt(aVal));
		return dict[n] || word;
	});
}

export function extractVideoUrl(packedHtml: string): string | null {
	// Find all eval() blocks
	const scripts = packedHtml.match(/eval\(f.+?\}\)\)/g) ?? [];

	for (const script of scripts) {
		try {
			const decoded = unpackEval(script);
			const urlMatch = decoded.match(/source='(.+?)'/);
			if (urlMatch) return urlMatch[1];
		} catch {
			continue;
		}
	}
	return null;
}
```

---

## AnimePahe Extension Class

```typescript
// index.ts
import type {
	SourceExtension,
	SearchResult,
	AnimeDetails,
	Episode,
	Page,
	StreamSource,
	ResolvedStream,
	CookieEntry,
} from '$lib/types/extensions';
import { invoke } from '@tauri-apps/api/core';
import { extractVideoUrl } from './unpacker';

const PAHE_BASE = 'https://animepahe.si';

export class AnimePaheExtension implements SourceExtension {
	readonly manifest = {
		id: 'animepahe',
		name: 'AnimePahe',
		version: '0.1.0',
		author: 'Zafkiel',
		type: 'source' as const,
		description: 'AnimePahe streaming source with HLS playback',
		requires: {
			cookieAuth: true,
			network: true,
			storage: true,
		},
	};

	private cookies: string = '';

	// ── Cookie Auth ──────────────────────────────────────────

	onCookiesUpdated(cookies: CookieEntry[]): void {
		this.cookies = cookies.map((c) => `${c.name}=${c.value}`).join('; ');
	}

	async checkAuth(): Promise<boolean> {
		if (!this.cookies) {
			// Try loading from Tauri storage
			try {
				const stored = await invoke<string | null>('ext_storage_get', {
					extId: this.manifest.id,
					key: 'cookies',
				});
				if (stored) {
					this.cookies = stored;
				}
			} catch {
				return false;
			}
		}

		if (!this.cookies) return false;

		// Probe with a cheap API call
		try {
			const res = await this._fetch(`${PAHE_BASE}/api?m=airing&page=1`);
			return !res.includes('"status":403') && !res.includes('Just a moment');
		} catch {
			return false;
		}
	}

	// ── Core Fetch (proxied through Tauri) ───────────────────

	private async _fetch(url: string, extraHeaders: Record<string, string> = {}): Promise<string> {
		return invoke<string>('fetch_url', {
			url,
			headers: {
				Cookie: this.cookies,
				'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
				...extraHeaders,
			},
		});
	}

	// ── Search ───────────────────────────────────────────────

	async search(query: string): Promise<SearchResult[]> {
		const raw = await this._fetch(`${PAHE_BASE}/api?m=search&q=${encodeURIComponent(query)}`);
		const data: { data: AnimePaheSearchResult[] } = JSON.parse(raw);

		return (data.data ?? []).map((item) => ({
			id: item.session,
			title: item.title,
			coverUrl: item.poster,
			type: item.type as SearchResult['type'],
			status: item.status as SearchResult['status'],
			year: item.year,
		}));
	}

	async getByAnilistId(anilistId: number): Promise<AnimeDetails | null> {
		// AnimePahe exposes anilist_id in the HTML; search is the only entry point
		// We'd need a reverse mapping; return null and let the UI fall back to search.
		return null;
	}

	// ── Detail ───────────────────────────────────────────────

	async getAnimeDetails(id: string): Promise<AnimeDetails> {
		const html = await this._fetch(`${PAHE_BASE}/anime/${id}`);

		const title =
			html.match(/<span style="user-select:text">(.+?)<\/span>/)?.[1] ??
			html.match(/<meta property="og:title" content="(.+?)">/)?.[1] ??
			id;

		const coverUrl =
			html.match(/<a href="(https:\/\/i\.animepahe\.si\/posters.+?)"/)?.[1] ??
			html.match(/<meta property="og:image" content="(.+?)">/)?.[1];

		const description =
			html.match(/<div class="anime-synopsis">(.+?)<\/div>/)?.[1] ??
			html.match(/<meta property="og:description" content="(.+?)">/)?.[1];

		const anilistId = html.match(/<a href="\/\/anilist\.co\/anime\/(\d+)">/)?.[1];

		return {
			id,
			title,
			coverUrl,
			description,
			anilistId: anilistId ? parseInt(anilistId) : undefined,
			sessionId: id,
		};
	}

	// ── Episodes ─────────────────────────────────────────────

	async getEpisodes(animeSessionId: string, page = 1): Promise<Page<Episode>> {
		const raw = await this._fetch(`${PAHE_BASE}/api?m=release&id=${animeSessionId}&page=${page}`);
		const data: AnimePaheEpisodesResponse = JSON.parse(raw);

		const episodes: Episode[] = (data.data ?? []).map((ep) => ({
			id: ep.session,
			number: ep.episode,
			title: `Episode ${ep.episode}`,
			thumbnailUrl: ep.snapshot ? `https://i.animepahe.si/snapshots/${ep.snapshot}` : undefined,
			airDate: ep.created_at,
			slug: ep.session,
			snapshotUrl: ep.snapshot,
		}));

		return {
			data: episodes,
			currentPage: data.current_page,
			lastPage: data.last_page,
			total: data.total,
			perPage: data.per_page,
		};
	}

	/** Convenience: fetch ALL episodes (iterates pages) */
	async getAllEpisodes(animeSessionId: string): Promise<Episode[]> {
		let page = 1;
		const all: Episode[] = [];

		while (true) {
			const result = await this.getEpisodes(animeSessionId, page);
			all.push(...result.data);
			if (result.currentPage >= result.lastPage) break;
			page++;
		}

		return all;
	}

	// ── Stream Sources ───────────────────────────────────────

	async getStreamSources(animeId: string, episodeId: string): Promise<StreamSource[]> {
		const html = await this._fetch(`${PAHE_BASE}/play/${animeId}/${episodeId}`);

		// Kwik.si buttons
		const buttonRegex =
			/<button[^>]+data-src="(https:\/\/kwik\.(?:si|cx).+?)"[^>]+data-fansub="(.+?)"[^>]+data-resolution="(.+?)"[^>]+data-audio="(.+?)"[^>]*>/g;

		const sources: StreamSource[] = [];
		let match: RegExpExecArray | null;

		while ((match = buttonRegex.exec(html)) !== null) {
			const [, kwikUrl, fansub, resolution, audio] = match;
			sources.push({
				id: kwikUrl, // the Kwik URL acts as the ID
				label: `${fansub} ${resolution} (${audio})`,
				resolution,
				fansub,
				audio,
				requiresResolution: true,
			});
		}

		return sources;
	}

	// ── Resolve Stream (Kwik decode) ─────────────────────────

	async resolveStream(source: StreamSource): Promise<ResolvedStream> {
		const kwikHtml = await this._fetch(source.id, {
			Referer: PAHE_BASE,
		});

		const videoUrl = extractVideoUrl(kwikHtml);

		if (!videoUrl) {
			throw new Error(`[animepahe] Failed to extract video URL from Kwik page: ${source.id}`);
		}

		return {
			url: videoUrl,
			type: 'hls',
			headers: {
				Referer: 'https://kwik.si/',
			},
		};
	}
}

// ── Internal AnimePahe API types ──────────────────────────

interface AnimePaheSearchResult {
	session: string;
	title: string;
	poster: string;
	type: string;
	status: string;
	year: number;
}

interface AnimePaheEpisodeEntry {
	session: string;
	episode: number;
	snapshot: string;
	created_at: string;
}

interface AnimePaheEpisodesResponse {
	data: AnimePaheEpisodeEntry[];
	current_page: number;
	last_page: number;
	total: number;
	per_page: number;
}

export const animePaheExtension = new AnimePaheExtension();
```

---

## Cookie Auth Webview (Tauri)

When `checkAuth()` returns `false`, the frontend shows an "AnimePahe – Connect" button.
Pressing it invokes a Tauri command:

```typescript
// Frontend trigger
await invoke('open_animepahe_auth_webview');
// wait for tauri event:
const unlisten = await listen<CookieEntry[]>('animepahe-cookies-ready', (event) => {
	animePaheExtension.onCookiesUpdated(event.payload);
	// persist cookies
	invoke('ext_storage_set', {
		extId: 'animepahe',
		key: 'cookies',
		value: event.payload.map((c) => `${c.name}=${c.value}`).join('; '),
	});
	unlisten();
});
```

Rust command skeleton:

```rust
// commands/extensions.rs
#[command]
pub async fn open_animepahe_auth_webview(app: AppHandle) -> Result<(), String> {
    use tauri::WebviewWindowBuilder;

    let webview = WebviewWindowBuilder::new(
        &app,
        "animepahe-auth",
        tauri::WebviewUrl::External("https://animepahe.si".parse().unwrap())
    )
    .title("Connect to AnimePahe (solve Cloudflare challenge, then close)")
    .inner_size(900.0, 700.0)
    .build()
    .map_err(|e| e.to_string())?;

    // When the window closes, harvest cookies and emit event
    let app_handle = app.clone();
    webview.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            // TODO: read cookies from webview session → emit 'animepahe-cookies-ready'
            let _ = app_handle.emit("animepahe-cookies-ready", Vec::<()>::new());
        }
    });

    Ok(())
}
```

> **Note**: Tauri's WebviewWindow cookie access is limited.
> One approach: inject a JS snippet into the webview that reads `document.cookie` and sends it via IPC.

---

## AnimePahe vs Other Sources – Data Flow

```
User opens Anime Detail Page (has an AniList ID)
        │
        ▼
1. animePaheExtension.search(title) → find provider ID (session)
   (or getByAnilistId if supported in future)
        │
        ▼
2. animePaheExtension.getAllEpisodes(session)
   → episodes list with thumbnails
        │
        ▼
3. User taps an episode
        │
        ▼
4. animePaheExtension.getStreamSources(session, episodeSession)
   → list of StreamSources (one per quality/fansub)
        │
        ▼
5. User picks quality (or auto-select best)
        │
        ▼
6. animePaheExtension.resolveStream(source)
   → ResolvedStream { url: "https://cdn.kwik.si/hls/...", type: "hls" }
        │
        ▼
7. Pass URL to video player (HLS.js / native)
```

---

## Implementation Checklist

- [ ] `src/lib/types/extensions.ts` – full interface types (from `001_EXTENSION_INTERFACE.md`)
- [ ] `src/lib/extensions/source/animepahe/unpacker.ts` – PACKAD decoder
- [ ] `src/lib/extensions/source/animepahe/index.ts` – AnimePaheExtension class
- [ ] `src/lib/services/ExtensionRegistry.ts` – singleton registry (refactor ExtensionManager)
- [ ] `src-tauri/src/commands/extensions.rs` – `open_animepahe_auth_webview`, `ext_storage_get`, `ext_storage_set`
- [ ] Update `src-tauri/src/lib.rs` to register new commands
- [ ] Cookie auth UI component (`AnimePaheConnect.svelte`)
- [ ] Episode list component (`AnimePaheEpisodes.svelte`)
- [ ] Stream quality picker component (`StreamSourcePicker.svelte`)
- [ ] Integration with anime `[id]` route
- [ ] Storybook stories for new components
- [ ] Unit tests for `unpacker.ts`

---

## Rate Limiting & Caching

| Endpoint                    | Suggested cache `staleTime`                |
| --------------------------- | ------------------------------------------ |
| `search()`                  | 10 minutes                                 |
| `getAnimeDetails()`         | 30 minutes                                 |
| `getEpisodes()` (all pages) | 30 minutes                                 |
| `getStreamSources()`        | 5 minutes (Kwik URLs expire)               |
| `resolveStream()`           | Do NOT cache (stream URLs expire ~4 hours) |

All caches are managed by TanStack Query in the frontend layer.
The extension itself is stateless except for the cookie string.
