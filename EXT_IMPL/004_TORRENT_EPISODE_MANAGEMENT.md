# Episode-Wise Torrent Management

This document covers the plan and implementation for making Zafkiel's torrent system
**episode-aware**, matching the quality of Zenshin's approach.

---

## Problems with the Current System

| Current state | Desired state |
|--------------|---------------|
| Search returns anime-level results (all episodes mixed) | One-click torrent per episode |
| No episode thumbnails / titles on torrent results | Episode metadata shown (title, thumbnail, airdate) |
| No AniDB ID integration → generic Nyaa search | Precise per-episode Tosho lookup via AniDB IDs |
| File selection inside a multi-episode pack is manual | Auto-identify which file = which episode |
| No per-episode download progress tracking | Per-episode progress bar |
| Mock data in `NyaaProvider` | Real Nyaa RSS parsing via Tauri backend |

---

## APIs Used (from Zenshin Analysis)

### 1. AniZip Mappings API
**Base URL**: `http://zenshin-supabase-api-myig.onrender.com`
**Endpoint**: `GET /mappings?anilist_id={id}`

Returns:
- Episode titles (EN/JP/Romaji)
- Episode thumbnails
- Air dates
- AniDB IDs (`mappings.anidb_id`, and per-episode `anidbEid`)

> This is Zenshin's own Supabase instance mirroring `api.ani.zip`.
> We should use the official `api.ani.zip` directly:
> `GET https://api.ani.zip/mappings?anilist_id={id}`

### 2. Anime Tosho API
**Base URL**: `https://feed.animetosho.org`
**Endpoint**: `GET /json?qx=1&aids={anidb_id}&eids={anidb_episode_id}`

Returns an array of torrent entries, each with:
- `title` – torrent name
- `magnet_uri` – magnet link
- `total_size` – bytes
- `num_seeders`, `num_leechers`
- `timestamp` – upload date
- `anidb_aid`, `anidb_eid` – confirmed AniDB IDs
- `resolution` – parsed resolution string

### 3. Nyaa API (existing, via Tauri proxy)
**Endpoint**: `https://nyaa.si/?page=rss&q={query}&c=0_0&f=0`
Used as **fallback** when Tosho has no results.
Already implemented in `SubsPleaseExtension` / `NyaaProvider`.

---

## Episode Metadata Service

New file: `src/lib/services/EpisodeMetadataService.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AniZipMappings, AniZipEpisode } from '$lib/types/extensions';

const ANIZIP_BASE = 'https://api.ani.zip';
const TOSHO_BASE = 'https://feed.animetosho.org';

export interface EpisodeMeta {
  number: number;
  title: string;
  thumbnailUrl?: string;
  airDate?: string;
  overview?: string;
  /** AniDB Series ID */
  anidbId: number;
  /** AniDB Episode ID – needed for precise Tosho lookup */
  anidbEpisodeId: number;
}

export interface ToshoEntry {
  title: string;
  magnetUri: string;
  size: string;
  seeds: number;
  peers: number;
  uploadedAt: string;
  resolution?: string;
  fansub?: string;
  anidbId?: number;
  anidbEpisodeId?: number;
}

// ─── AniZip ──────────────────────────────────────────────────

export async function fetchAniZipMappings(anilistId: number): Promise<AniZipMappings> {
  const url = `${ANIZIP_BASE}/mappings?anilist_id=${anilistId}`;
  const raw = await invoke<string>('fetch_url', { url, headers: {} });
  return JSON.parse(raw) as AniZipMappings;
}

export function buildEpisodeMetas(mappings: AniZipMappings): EpisodeMeta[] {
  const anidbId = mappings.mappings.anidb_id;

  return Object.entries(mappings.episodes)
    .map(([, ep]) => {
      const num = typeof ep.episode === 'string' ? parseFloat(ep.episode) : ep.episode;
      if (isNaN(num) || num <= 0) return null; // skip specials/OPs

      return {
        number: num,
        title: ep.title.en ?? ep.title['x-jat'] ?? `Episode ${num}`,
        thumbnailUrl: ep.image ?? undefined,
        airDate: ep.airdate ?? undefined,
        overview: ep.overview ?? undefined,
        anidbId,
        anidbEpisodeId: ep.anidbEid,
      } satisfies EpisodeMeta;
    })
    .filter((ep): ep is EpisodeMeta => ep !== null)
    .sort((a, b) => a.number - b.number);
}

// ─── Anime Tosho ──────────────────────────────────────────────

/**
 * Fetch torrent entries for a specific episode via AniDB IDs.
 * This is the precise lookup path – mirrors Zenshin's getToshoEpisodes.
 */
export async function fetchToshoEpisode(
  anidbId: number,
  anidbEpisodeId: number | null,
  quality: '720p' | '1080p' | 'all' = 'all',
): Promise<ToshoEntry[]> {
  let url: string;

  if (!anidbEpisodeId) {
    // Series-level, no episode filter
    url = quality === 'all'
      ? `${TOSHO_BASE}/json?qx=1&aids=${anidbId}`
      : `${TOSHO_BASE}/json?qx=1&q=${quality}&aids=${anidbId}`;
  } else {
    url = quality === 'all'
      ? `${TOSHO_BASE}/json?qx=1&aids=${anidbId}&eids=${anidbEpisodeId}`
      : `${TOSHO_BASE}/json?qx=1&q=${quality}&aids=${anidbId}&eids=${anidbEpisodeId}`;
  }

  const raw = await invoke<string>('fetch_url', { url, headers: {} });
  const data: ToshoRawEntry[] = JSON.parse(raw);

  return data.map(normalizeToshoEntry);
}

/**
 * Fallback: Free-text Nyaa search for an episode.
 * Used when Tosho has no results (e.g. very recent releases not yet indexed).
 */
export async function fetchNyaaEpisode(
  animeTitle: string,
  episodeNumber: number,
  quality: '720p' | '1080p' | 'all' = '1080p',
): Promise<ToshoEntry[]> {
  const epStr = episodeNumber < 10 ? `0${episodeNumber}` : `${episodeNumber}`;
  const qualityStr = quality === 'all' ? '' : ` ${quality}`;
  const query = `[SubsPlease] ${animeTitle} - ${epStr}${qualityStr}`;
  const url = `https://nyaa.si/?page=rss&q=${encodeURIComponent(query)}&c=0_0&f=0`;

  const xml = await invoke<string>('fetch_url', { url, headers: {} });
  return parseNyaaRss(xml, 'Nyaa.si');
}

// ─── Normalizers ─────────────────────────────────────────────

function normalizeToshoEntry(e: ToshoRawEntry): ToshoEntry {
  return {
    title: e.title,
    magnetUri: e.magnet_uri ?? '',
    size: formatBytes(e.total_size),
    seeds: e.num_seeders ?? 0,
    peers: e.num_leechers ?? 0,
    uploadedAt: new Date(e.timestamp * 1000).toLocaleDateString(),
    resolution: extractResolution(e.title),
    fansub: extractFansub(e.title),
    anidbId: e.anidb_aid,
    anidbEpisodeId: e.anidb_eid,
  };
}

function parseNyaaRss(xml: string, provider: string): ToshoEntry[] {
  const parser = new DOMParser();
  const doc = parser.parseFromString(xml, 'text/xml');
  const items = doc.querySelectorAll('item');
  const results: ToshoEntry[] = [];

  items.forEach((item) => {
    const title = item.querySelector('title')?.textContent ?? '';
    const link = item.querySelector('link')?.textContent ?? '';
    const pubDate = item.querySelector('pubDate')?.textContent ?? '';
    const seeds = parseInt(item.getElementsByTagName('nyaa:seeders')[0]?.textContent ?? '0');
    const peers = parseInt(item.getElementsByTagName('nyaa:leechers')[0]?.textContent ?? '0');
    const size = item.getElementsByTagName('nyaa:size')[0]?.textContent ?? 'Unknown';
    const infoHash = item.getElementsByTagName('nyaa:infoHash')[0]?.textContent;

    const magnet = infoHash
      ? `magnet:?xt=urn:btih:${infoHash}&dn=${encodeURIComponent(title)}`
      : link;

    results.push({
      title, magnetUri: magnet, size, seeds, peers,
      uploadedAt: new Date(pubDate).toLocaleDateString(),
      resolution: extractResolution(title),
      fansub: extractFansub(title),
    });
  });

  return results;
}

// ─── Utilities ────────────────────────────────────────────────

function formatBytes(bytes: number): string {
  if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(2)} GiB`;
  if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(2)} MiB`;
  return `${bytes} B`;
}

function extractResolution(title: string): string | undefined {
  return title.match(/\b(2160p|1080p|720p|480p|360p)\b/i)?.[1];
}

function extractFansub(title: string): string | undefined {
  return title.match(/^\[(.+?)\]/)?.[1];
}

// ─── Raw Tosho API types (from live API inspection) ───────────

interface ToshoRawEntry {
  id: number;
  title: string;
  magnet_uri: string;
  total_size: number;
  num_seeders: number;
  num_leechers: number;
  timestamp: number;
  anidb_aid?: number;
  anidb_eid?: number;
  resolution?: string;
}
```

---

## TanStack Query Hooks

New file: `src/lib/hooks/useEpisodeTorrents.svelte.ts`

```typescript
import { createQuery } from '@tanstack/svelte-query';
import {
  fetchAniZipMappings,
  buildEpisodeMetas,
  fetchToshoEpisode,
  fetchNyaaEpisode,
  type EpisodeMeta,
  type ToshoEntry,
} from '$lib/services/EpisodeMetadataService';

// ── Episode metadata (titles, thumbnails) ─────────────────────

export function useAniZipEpisodes(anilistId: number | null) {
  return createQuery({
    queryKey: ['anizip-episodes', anilistId],
    queryFn: async () => {
      if (!anilistId) return null;
      const mappings = await fetchAniZipMappings(anilistId);
      return buildEpisodeMetas(mappings);
    },
    enabled: !!anilistId,
    staleTime: 1000 * 60 * 60, // 1 hour – very stable data
    gcTime: 1000 * 60 * 60 * 24,
  });
}

// ── Per-episode torrent search ────────────────────────────────

export function useEpisodeTorrents(
  episode: EpisodeMeta | null,
  quality: '720p' | '1080p' | 'all' = '1080p',
) {
  return createQuery({
    queryKey: ['episode-torrents', episode?.anidbId, episode?.anidbEpisodeId, quality],
    queryFn: async (): Promise<ToshoEntry[]> => {
      if (!episode) return [];

      // Primary: precise Tosho lookup
      let results = await fetchToshoEpisode(episode.anidbId, episode.anidbEpisodeId, quality);

      // Fallback: Nyaa free-text search
      if (results.length === 0) {
        results = await fetchNyaaEpisode(episode.title, episode.number, quality);
      }

      return results.sort((a, b) => b.seeds - a.seeds);
    },
    enabled: !!episode,
    staleTime: 1000 * 60 * 10, // 10 minutes
  });
}

// ── Anime-level torrent list (for downloads tab) ──────────────

export function useAnimeTorrents(anidbId: number | null) {
  return createQuery({
    queryKey: ['anime-torrents', anidbId],
    queryFn: () => {
      if (!anidbId) return [];
      return fetchToshoEpisode(anidbId, null, 'all');
    },
    enabled: !!anidbId,
    staleTime: 1000 * 60 * 15,
  });
}
```

---

## Updated `TorrentInfo` Type

The existing `TorrentInfo` in `ExtensionManager.ts` needs episode-aware fields:

```typescript
// Add to existing TorrentInfo interface
export interface TorrentInfo {
  title: string;
  size: string;
  seeds: number;
  peers: number;
  magnet: string;
  provider: string;
  uploadedAt?: string;
  // ── New fields ──
  episode?: number;           // parsed episode number
  resolution?: string;        // "1080p", "720p", etc.
  fansub?: string;            // "[SubsPlease]", "[Erai-raws]", etc.
  anidbId?: number;           // for Tosho integration
  anidbEpisodeId?: number;    // for Tosho integration
}
```

---

## `EpisodeTorrentExtension` Interface

Extends the generic `TorrentExtension` with the episode-precise method:

```typescript
export interface EpisodeTorrentExtension extends TorrentExtension {
  searchEpisode(query: EpisodeTorrentQuery): Promise<TorrentInfo[]>;
}
```

**ToshoProvider** (`src/lib/extensions/torrent/ToshoProvider.ts`) implements this:

```typescript
export class ToshoProvider implements EpisodeTorrentExtension {
  manifest = {
    id: 'animetosho',
    name: 'Anime Tosho',
    version: '1.0.0',
    author: 'Zafkiel',
    type: 'torrent' as const,
    description: 'Episode-precise torrent search via Anime Tosho + AniDB IDs',
    requires: { network: true },
  };

  async search(query: string): Promise<TorrentInfo[]> {
    // Free-text Tosho search
    const url = `https://feed.animetosho.org/json?qx=1&q=${encodeURIComponent(query)}`;
    const raw = await invoke<string>('fetch_url', { url, headers: {} });
    return parseToshoResponse(JSON.parse(raw));
  }

  async searchAnime(anilistId: number, title: string): Promise<TorrentInfo[]> {
    // Handled via AniZip → per-episode lookups at the UI layer
    return this.search(title);
  }

  async searchEpisode(query: EpisodeTorrentQuery): Promise<TorrentInfo[]> {
    if (query.anidbId && query.anidbEpisodeId) {
      const entries = await fetchToshoEpisode(query.anidbId, query.anidbEpisodeId, query.quality ?? 'all');
      return entries.map(e => ({ ...e, magnet: e.magnetUri, provider: 'Anime Tosho' }));
    }
    // Fallback to Nyaa
    const entries = await fetchNyaaEpisode(query.animeTitle, query.episodeNumber, query.quality ?? '1080p');
    return entries.map(e => ({ ...e, magnet: e.magnetUri, provider: 'Nyaa.si' }));
  }
}
```

---

## UI: Episode List with Per-Episode Torrents

The anime `[id]` page shows episodes with integrated torrent buttons:

```
┌──────────────────────────────────────────────────────────┐
│  Episode 01                            [↓ Download] [▶]  │
│  "Episode Title" · Apr 6 2025                            │
├──────────────────────────────────────────────────────────┤
│  Episode 02                            [↓ 720p] [↓ 1080p]│
│  "Episode Title" · Apr 13 2025                           │
├──────────────────────────────────────────────────────────┤
│  Episode 03   ████████░░░░  65%  ↓ 2.3 MB/s             │
│  "Episode Title" · Apr 20 2025          [▶ Watch]        │
└──────────────────────────────────────────────────────────┘
```

Components:
- `EpisodeRow.svelte` – renders one episode with torrent status
- `EpisodeTorrentButton.svelte` – quality picker dropdown + download trigger
- `EpisodeDownloadProgress.svelte` – live progress bar (polls `TorrentService`)

---

## Torrent ↔ Episode Linking

When a user clicks "Download Episode 4":
1. Look up `EpisodeMeta` for episode 4 (has `anidbId` + `anidbEpisodeId`)
2. Call `ToshoProvider.searchEpisode(query)` → get magnet list
3. Auto-select the best match (highest seeds, preferred quality)
4. Call `TorrentService.streamTorrent(magnet)` via existing Tauri command
5. Torrent is added to librqbit; `get_torrents` returns progress
6. Match the running torrent back to episode 4 by **filename parsing**:
   - librqbit gives us `TorrentFile.name`
   - Parse episode number from filename: `[SubsPlease] Show - 04 (1080p).mkv` → ep 4
   - Link torrent_id + file_id to episode number in a `Map<number, {torrentId, fileId}>`

---

## Episode–Torrent State Store

New file: `src/lib/stores/episodeTorrentStore.ts`

```typescript
import { writable, derived } from 'svelte/store';
import type { Torrent } from '$lib/services/TorrentService';

interface EpisodeTorrentLink {
  episodeNumber: number;
  torrentId: number;
  fileId?: number;
  fileName?: string;
}

// Maps anime anilistId → array of episode-torrent links
const _links = writable<Map<number, EpisodeTorrentLink[]>>(new Map());

export const episodeTorrentStore = {
  subscribe: _links.subscribe,

  link(anilistId: number, link: EpisodeTorrentLink) {
    _links.update(m => {
      const existing = m.get(anilistId) ?? [];
      m.set(anilistId, [...existing.filter(l => l.episodeNumber !== link.episodeNumber), link]);
      return m;
    });
  },

  getLink(anilistId: number, episodeNumber: number): EpisodeTorrentLink | undefined {
    let result: EpisodeTorrentLink | undefined;
    _links.subscribe(m => {
      result = m.get(anilistId)?.find(l => l.episodeNumber === episodeNumber);
    })();
    return result;
  },

  clearAnime(anilistId: number) {
    _links.update(m => { m.delete(anilistId); return m; });
  },
};
```

---

## Rust Backend: `fetch_url` with Headers

The current `fetch_url` command may not support arbitrary headers.
Ensure it accepts a `HashMap<String, String>`:

```rust
// commands/utils.rs (or api.rs)
#[command]
pub async fn fetch_url(
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut req = client.get(&url);

    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            req = req.header(&k, &v);
        }
    }

    let response = req.send().await.map_err(|e| e.to_string())?;
    response.text().await.map_err(|e| e.to_string())
}
```

---

## Implementation Steps (Ordered)

### Phase 1 – Data Layer (start here)
1. Add `fetch_url` with headers support to Rust backend (if not already)
2. Create `EpisodeMetadataService.ts` with AniZip + Tosho fetchers
3. Create `useAniZipEpisodes` and `useEpisodeTorrents` hooks
4. Create `ToshoProvider.ts` and register it in `ExtensionManager`
5. Update `TorrentInfo` type with episode-aware fields

### Phase 2 – UI
6. Create `EpisodeRow.svelte` component
7. Create `EpisodeTorrentButton.svelte` with quality picker
8. Create `EpisodeDownloadProgress.svelte`
9. Integrate into anime `[id]` page

### Phase 3 – State & Linking
10. Create `episodeTorrentStore.ts`
11. On torrent download start: parse filename → link episode → store link
12. Poll torrents, update progress in episode rows
13. Show "Watch" button when episode file is ≥10% downloaded

### Phase 4 – Quality of Life
14. Auto-select SubsPlease 1080p as default
15. Remember per-user quality preference in settings
16. "Download All Episodes" button
17. Storybook stories for all new components

---

## API URL Summary

| API | URL | Auth |
|-----|-----|------|
| AniZip mappings | `https://api.ani.zip/mappings?anilist_id={id}` | None |
| Anime Tosho (episode) | `https://feed.animetosho.org/json?qx=1&aids={aid}&eids={eid}` | None |
| Anime Tosho (series) | `https://feed.animetosho.org/json?qx=1&aids={aid}` | None |
| Nyaa RSS | `https://nyaa.si/?page=rss&q={query}&c=0_0&f=0` | None |
| AnimePahe | `https://animepahe.si/api?...` | Cloudflare cookies |
