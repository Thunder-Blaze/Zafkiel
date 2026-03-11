# Extension System – Overview & Reading Guide

This folder contains the full design specification for Zafkiel's extension system and improved torrent management.

---

## Files

| File                                                                     | What it covers                                                                           |
| ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- |
| [001_EXTENSION_INTERFACE.md](./001_EXTENSION_INTERFACE.md)               | TypeScript interface contracts – what every extension must implement and what it returns |
| [002_WASM_ARCHITECTURE.md](./002_WASM_ARCHITECTURE.md)                   | How WASM extensions are packaged (`.zext`), loaded at runtime, and sandboxed             |
| [003_ANIMEPAHE_EXTENSION.md](./003_ANIMEPAHE_EXTENSION.md)               | AnimePahe source extension – API surface, Kwik resolver, cookie auth flow                |
| [004_TORRENT_EPISODE_MANAGEMENT.md](./004_TORRENT_EPISODE_MANAGEMENT.md) | Episode-wise torrent system – AniZip, Anime Tosho, per-episode UI                        |

---

## Key Decisions Summary

### 1. Extension Format

- Extensions are packaged as **`.zext`** files (renamed ZIP)
- Contain: `manifest.json` + `extension.wasm` (Rust) OR `extension.js` (TypeScript)
- Loaded at runtime via `WebAssembly.instantiate()` or dynamic `import()`

### 2. Communication with Host

- Extensions **never** make direct network calls – all I/O goes through Tauri IPC:
  `fetch_url(url, headers)` → Rust backend → response back to JS/WASM
- This bypasses CORS and gives a single audit point for network access

### 3. First Extension: AnimePahe

- Start with **TypeScript** (not WASM) for simplicity and faster iteration
- Cookie auth via Tauri `WebviewWindow` pointed at `https://animepahe.si`
- Streams through Kwik (HLS) with custom PACKAD decoder
- Upgrade to Rust/WASM once the scraper logic is stable

### 4. Torrent Improvements

- Integrate **AniZip** (`api.ani.zip`) for episode titles, thumbnails, AniDB IDs
- Integrate **Anime Tosho** (`feed.animetosho.org`) for episode-precise magnet links
- Keep Nyaa RSS as fallback
- Per-episode download progress via filename parsing + `episodeTorrentStore`

---

## Implementation Order

```
IMMEDIATE (Torrent improvements):
  1. EpisodeMetadataService.ts  ← AniZip + Tosho APIs
  2. useAniZipEpisodes hook     ← TanStack Query
  3. useEpisodeTorrents hook    ← TanStack Query
  4. ToshoProvider.ts           ← new TorrentExtension
  5. Update TorrentInfo type
  6. EpisodeRow/TorrentButton UI components
  7. Wire into anime [id] page

NEXT (AnimePahe extension):
  8. extension types file (from 001)
  9. unpacker.ts port
  10. AnimePaheExtension class
  11. Cookie auth webview (Tauri)
  12. Stream quality picker UI

LATER (WASM infrastructure):
  13. WasmExtensionLoader.ts
  14. ext_storage_* Tauri commands
  15. Rust extension template / crate
  16. .zext packaging script
```

---

## Data Flow Diagram

```
AniList ID
    │
    ▼
api.ani.zip/mappings
    │
    ├─→ Episode titles, thumbnails, air dates (for UI)
    │
    └─→ AniDB Series ID + Episode IDs
            │
            ▼
   feed.animetosho.org/json
            │
            ▼
   Magnet links (per episode)
            │
            ▼
   librqbit (Rust torrent engine)
            │
            ├─→ Stream URL (HTTP range requests)
            │        │
            │        ▼
            │   Video Player (HLS.js / native)
            │
            └─→ Downloaded file
                     │
                     ▼
              Parse filename → episode number
              Link to episodeTorrentStore
```
