# Extension Interface Specification

This document defines the complete TypeScript/Rust interface contract that every Zafkiel extension must fulfill.

---

## Extension Categories

```
ExtensionType
├── "source"    – Streaming / scraping providers (AnimePahe, etc.)
├── "torrent"   – Torrent-index providers (Nyaa, Tosho, etc.)
└── "tracker"   – Not yet spec'd (MAL sync, Shikimori, etc.)
```

---

## Common Types

```typescript
// ─────────────────────────────────────────────────────────────
// Manifest  – static metadata shipped with every extension
// ─────────────────────────────────────────────────────────────
export interface ExtensionManifest {
  /** Globally unique slug, e.g. "animepahe-wasm" */
  id: string;
  name: string;
  version: string;        // SemVer
  author: string;
  description?: string;
  type: ExtensionType;

  /** Capabilities the host must provide */
  requires: {
    cookieAuth?: boolean;   // needs a managed-cookie webview session
    network?: boolean;      // needs Tauri fetch_url proxy
    storage?: boolean;      // needs persistent key-value storage
  };

  /** Optional icon URL (data URI or https) */
  iconUrl?: string;
}

export type ExtensionType = 'source' | 'torrent' | 'tracker';

// ─────────────────────────────────────────────────────────────
// Pagination wrapper used by list endpoints
// ─────────────────────────────────────────────────────────────
export interface Page<T> {
  data: T[];
  currentPage: number;
  lastPage: number;
  total: number;
  perPage: number;
}

// ─────────────────────────────────────────────────────────────
// Search result – lightweight card returned by search()
// ─────────────────────────────────────────────────────────────
export interface SearchResult {
  /** Provider-internal ID, used in subsequent calls */
  id: string;
  title: string;
  coverUrl?: string;
  /** AniList ID if the provider exposes it (enables cross-linking) */
  anilistId?: number;
  /** MAL ID if exposed */
  malId?: number;
  type?: 'TV' | 'MOVIE' | 'OVA' | 'ONA' | 'SPECIAL';
  status?: 'FINISHED' | 'RELEASING' | 'NOT_YET_RELEASED';
  episodeCount?: number;
  score?: number;
  year?: number;
}

// ─────────────────────────────────────────────────────────────
// Anime details – richer data for the detail view
// ─────────────────────────────────────────────────────────────
export interface AnimeDetails extends SearchResult {
  description?: string;
  genres?: string[];
  bannerUrl?: string;
  /** Provider session token / slug needed to fetch episodes */
  sessionId?: string;
}

// ─────────────────────────────────────────────────────────────
// Episode – a single episode card
// ─────────────────────────────────────────────────────────────
export interface Episode {
  /** Provider-internal episode ID */
  id: string;
  /** Numeric episode number (may be fractional, e.g. 12.5) */
  number: number;
  title?: string;
  thumbnailUrl?: string;
  airDate?: string;   // ISO 8601
  duration?: number;  // seconds
  /** Filler/recap flag */
  isFiller?: boolean;
  /** Provider-internal slug needed to fetch stream sources */
  slug?: string;
  /** Snapshot URL (AnimePahe style) */
  snapshotUrl?: string;
}

// ─────────────────────────────────────────────────────────────
// Stream source – one playback option for an episode
// ─────────────────────────────────────────────────────────────
export interface StreamSource {
  /** Provider-internal key to call resolveStream() */
  id: string;
  /** Human-readable label shown in quality picker */
  label: string;
  resolution?: '360p' | '480p' | '720p' | '1080p' | string;
  fansub?: string;
  /** "sub" | "dub" | "raw" */
  audio?: string;
  /** Whether a second resolve step is needed */
  requiresResolution: boolean;
  /** Direct URL if already resolved (skip resolveStream) */
  directUrl?: string;
}

// ─────────────────────────────────────────────────────────────
// Resolved stream – ready-to-play URL + metadata
// ─────────────────────────────────────────────────────────────
export interface ResolvedStream {
  url: string;
  /** "hls" | "mp4" | "mkv" */
  type: 'hls' | 'mp4' | 'mkv' | string;
  headers?: Record<string, string>;
  /** Subtitles bundled with the stream */
  subtitles?: SubtitleTrack[];
}

export interface SubtitleTrack {
  label: string;
  language: string;   // BCP-47
  url: string;
  format: 'vtt' | 'srt' | 'ass';
}
```

---

## Source Extension Interface

Every **source** extension must implement this interface.

```typescript
export interface SourceExtension {
  readonly manifest: ExtensionManifest;

  // ── Discovery ─────────────────────────────────────────────
  /**
   * Full-text search.  Called from the global search bar.
   * @param query  User-supplied search string
   */
  search(query: string): Promise<SearchResult[]>;

  /**
   * Look up an anime using its AniList ID.
   * Return null if this provider has no mapping for that ID.
   */
  getByAnilistId(anilistId: number): Promise<AnimeDetails | null>;

  // ── Detail + Episodes ─────────────────────────────────────
  /**
   * Full anime details from provider ID (returned by search).
   */
  getAnimeDetails(id: string): Promise<AnimeDetails>;

  /**
   * Paginated episode list.
   * Providers may return ALL episodes on page 1 (lastPage === 1).
   */
  getEpisodes(id: string, page?: number): Promise<Page<Episode>>;

  // ── Playback ──────────────────────────────────────────────
  /**
   * List available qualities / fansub groups for one episode.
   * @param animeId   Provider anime ID
   * @param episodeId Provider episode ID
   */
  getStreamSources(animeId: string, episodeId: string): Promise<StreamSource[]>;

  /**
   * Resolve a StreamSource that has requiresResolution === true.
   * May involve a second HTTP hop, JS evaluation, or decryption.
   */
  resolveStream(source: StreamSource): Promise<ResolvedStream>;

  // ── Cookie-gated providers (optional) ────────────────────
  /**
   * Called by the host after a successful cookie-auth webview session.
   * Extensions that do NOT need cookie auth may leave this a no-op.
   */
  onCookiesUpdated?(cookies: CookieEntry[]): void;

  /**
   * The host calls this on cold start to check if stored cookies
   * are still valid.  Returns false → host opens the cookie webview.
   */
  checkAuth?(): Promise<boolean>;
}

export interface CookieEntry {
  name: string;
  value: string;
  domain: string;
  path: string;
  secure: boolean;
  httpOnly: boolean;
  expirationDate?: number;
}
```

---

## Torrent Extension Interface

Every **torrent** extension must implement this interface.
(Extends the existing `TorrentProvider` shape with episode-awareness.)

```typescript
export interface TorrentInfo {
  title: string;
  size: string;
  seeds: number;
  peers: number;
  magnet: string;
  provider: string;
  uploadedAt?: string;
  /** Parsed episode number, if detectable from the title */
  episode?: number;
  /** Resolution string parsed from title */
  resolution?: string;
  /** Fansub group parsed from title */
  fansub?: string;
  /** Tosho / AniDB ids for precise episode matching */
  anidbId?: number;
  anidbEpisodeId?: number;
}

export interface EpisodeTorrentQuery {
  /** AniList ID of the parent anime */
  anilistId: number;
  /** AniDB Series ID (from AniZip) */
  anidbId?: number;
  /** AniDB Episode ID (from AniZip) */
  anidbEpisodeId?: number;
  /** Human-readable episode number (fallback query) */
  episodeNumber: number;
  /** Preferred anime title for free-text fallback */
  animeTitle: string;
  /** Quality filter, "all" = no filter */
  quality?: '720p' | '1080p' | 'all';
}

export interface TorrentExtension {
  readonly manifest: ExtensionManifest;

  /**
   * Free-text search.  Used by the manual search dialog.
   */
  search(query: string): Promise<TorrentInfo[]>;

  /**
   * Anime-level search – returns ALL episodes worth of torrents.
   * Used to populate the "Downloads" tab.
   */
  searchAnime(anilistId: number, title: string): Promise<TorrentInfo[]>;

  /**
   * Episode-precise search.  Must return only results for that exact
   * episode.  Providers that cannot do this should fall back to
   * searchAnime() + client-side filtering.
   */
  searchEpisode(query: EpisodeTorrentQuery): Promise<TorrentInfo[]>;
}
```

---

## Extension Host API (Tauri ↔ WASM bridge)

These are the Tauri commands the host provides to every extension.

```typescript
// Injected into WASM as imports (via wasm-bindgen extern C)
interface HostAPI {
  /** Proxy HTTP GET through the Rust backend (bypasses CORS) */
  fetchUrl(url: string, headers?: Record<string, string>): Promise<string>;

  /** Proxy HTTP POST */
  postUrl(url: string, body: string, headers?: Record<string, string>): Promise<string>;

  /** Read from per-extension persistent storage */
  storageGet(key: string): Promise<string | null>;

  /** Write to per-extension persistent storage */
  storageSet(key: string, value: string): Promise<void>;

  /** Log a message to the Tauri console */
  log(level: 'debug' | 'info' | 'warn' | 'error', msg: string): void;
}
```

---

## Extension Registry (TypeScript side)

```typescript
export interface ExtensionRegistry {
  /** Register a source extension (built-in or dynamically loaded) */
  registerSource(ext: SourceExtension): void;

  /** Register a torrent extension */
  registerTorrent(ext: TorrentExtension): void;

  /** Look up by ID */
  getSource(id: string): SourceExtension | undefined;
  getTorrent(id: string): TorrentExtension | undefined;

  /** All active source extensions */
  getSources(): SourceExtension[];

  /** All active torrent extensions */
  getTorrents(): TorrentExtension[];
}
```

---

## Error Handling Contract

All extension methods must:
- Return a resolved Promise with an empty array / null **on soft errors** (no results found)
- **Throw** a typed `ExtensionError` only for hard failures (network down, auth expired, etc.)

```typescript
export class ExtensionError extends Error {
  constructor(
    message: string,
    public readonly code:
      | 'AUTH_REQUIRED'    // needs re-auth / cookie refresh
      | 'RATE_LIMITED'     // 429 received
      | 'NOT_FOUND'        // resource does not exist on provider
      | 'PARSE_ERROR'      // HTML/JSON changed, scraper broke
      | 'NETWORK_ERROR'    // connectivity issue
      | 'UNKNOWN',
    public readonly extensionId: string,
  ) {
    super(message);
    this.name = 'ExtensionError';
  }
}
```

---

## AniZip Episode Metadata Type (shared utility)

Used by torrent extensions and optionally by source extensions to map episode numbers to AniDB IDs.

```typescript
export interface AniZipEpisode {
  episode: number | string;
  anidbEid: number;
  title: {
    en?: string;
    ja?: string;
    'x-jat'?: string;
  };
  image?: string;
  airdate?: string;
  overview?: string;
  duration?: number;
  length?: number;
}

export interface AniZipMappings {
  titles: {
    en?: string;
    ja?: string;
    'x-jat'?: string;
  };
  mappings: {
    anilist_id: number;
    anidb_id: number;
    mal_id?: number;
    thetvdb_id?: number;
    themoviedb_id?: number;
  };
  episodes: Record<string, AniZipEpisode>;
}
```
