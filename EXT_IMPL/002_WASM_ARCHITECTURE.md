# WASM Extension Architecture

This document describes exactly **how** extensions are packaged, distributed, loaded, and sandboxed in Zafkiel.

---

## Design Goals

| Goal | Decision |
|------|----------|
| First-class Rust support | Extensions are Rust crates compiled to `wasm32-unknown-unknown` |
| TypeScript extensions too | TS extensions transpile to a tiny WASM shim via `wasm-bindgen` OR run as plain JS modules (lighter path) |
| CORS bypass | All network I/O is routed through Tauri's Rust backend |
| Sandbox | WASM has no OS access; only the host-provided `HostAPI` imports are available |
| Hot-reload in dev | WASM bytes are loaded at runtime via `WebAssembly.instantiate()` |
| No bundler change | Extensions do not require changes to the main Vite build |

---

## Extension Packaging Format

An extension is a single **`.zext`** file (just a renamed `.zip`):

```
my-extension.zext
├── manifest.json      ← ExtensionManifest (JSON)
├── extension.wasm     ← compiled WASM module
└── icon.png           ← optional 64×64 icon
```

For TypeScript-only extensions (no Rust), `extension.wasm` is replaced by:

```
my-extension.zext
├── manifest.json
├── extension.js       ← ESM bundle (output of `bun build --target browser`)
└── icon.png
```

The host inspects `manifest.json` to decide which loader to use.

---

## Runtime Paths

```
┌──────────────────────────────────────────────────────────┐
│  SvelteKit Frontend                                       │
│                                                           │
│  ExtensionLoader.ts                                       │
│   ├── WasmExtensionLoader   (for .wasm)                   │
│   └── JsExtensionLoader     (for .js)                     │
│                                                           │
│  Each loader:                                             │
│   1. Downloads / reads .zext from disk                    │
│   2. Validates manifest                                   │
│   3. Constructs HostAPI proxy (Tauri invoke calls)        │
│   4. Instantiates module → returns SourceExtension or     │
│      TorrentExtension typed object                        │
│   5. Registers with ExtensionRegistry singleton           │
└──────────────────────────────────────────────────────────┘
                         │  Tauri IPC (invoke)
┌──────────────────────────────────────────────────────────┐
│  Rust / Tauri Backend                                     │
│                                                           │
│  commands/extensions.rs                                   │
│   ├── fetch_url(url, headers) → String                    │
│   ├── post_url(url, body, headers) → String               │
│   ├── ext_storage_get(ext_id, key) → Option<String>       │
│   └── ext_storage_set(ext_id, key, value)                 │
│                                                           │
│  (Existing torrent commands stay in commands/torrent.rs)  │
└──────────────────────────────────────────────────────────┘
```

---

## Rust WASM Extension Template

A Rust extension crate looks like this:

```toml
# Cargo.toml
[package]
name = "animepahe-ext"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console"] }
```

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// ── Host API imports ───────────────────────────────────────
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = fetchUrl)]
    async fn host_fetch_url(url: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = storageGet)]
    async fn host_storage_get(key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = storageSet)]
    async fn host_storage_set(key: &str, value: &str);
}

// ── Extension entry points ─────────────────────────────────
#[wasm_bindgen]
pub async fn search(query: String) -> JsValue {
    // call host_fetch_url, parse, return Vec<SearchResult> as JsValue
    todo!()
}

#[wasm_bindgen]
pub async fn get_anime_details(id: String) -> JsValue {
    todo!()
}

#[wasm_bindgen]
pub async fn get_episodes(id: String, page: u32) -> JsValue {
    todo!()
}

#[wasm_bindgen]
pub async fn get_stream_sources(anime_id: String, episode_id: String) -> JsValue {
    todo!()
}

#[wasm_bindgen]
pub async fn resolve_stream(source_json: String) -> JsValue {
    todo!()
}

#[wasm_bindgen]
pub async fn check_auth() -> bool {
    todo!()
}

#[wasm_bindgen]
pub fn get_manifest() -> JsValue {
    // Return the manifest as JSON / JsValue
    todo!()
}
```

Build command:
```bash
wasm-pack build --target web --out-dir pkg
# Then package: cp pkg/extension_bg.wasm extension.wasm && zip -j animepahe-ext.zext manifest.json extension.wasm icon.png
```

---

## TypeScript WASM Loader (`WasmExtensionLoader.ts`)

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { SourceExtension, ExtensionManifest } from '$lib/types/extensions';

/**
 * The host API object injected into the WASM module's import namespace.
 * WASM code calls these via the `extern "C"` declarations above.
 */
function buildHostAPI(extensionId: string) {
  return {
    fetchUrl: async (url: string, headers?: string): Promise<string> => {
      const parsedHeaders = headers ? JSON.parse(headers) : {};
      return invoke<string>('fetch_url', { url, headers: parsedHeaders });
    },
    postUrl: async (url: string, body: string, headers?: string): Promise<string> => {
      const parsedHeaders = headers ? JSON.parse(headers) : {};
      return invoke<string>('post_url', { url, body, headers: parsedHeaders });
    },
    storageGet: async (key: string): Promise<string | null> => {
      return invoke<string | null>('ext_storage_get', { extId: extensionId, key });
    },
    storageSet: async (key: string, value: string): Promise<void> => {
      return invoke<void>('ext_storage_set', { extId: extensionId, key, value });
    },
    log: (level: string, msg: string) => {
      console[level as 'log'](`[ext:${extensionId}] ${msg}`);
    },
  };
}

export async function loadWasmExtension(wasmBytes: ArrayBuffer, manifest: ExtensionManifest): Promise<SourceExtension> {
  const hostAPI = buildHostAPI(manifest.id);

  const { instance } = await WebAssembly.instantiate(wasmBytes, {
    // wasm-bindgen imports namespace
    './extension_bg.js': {}, // auto-generated glue
    __zafkiel_host__: hostAPI,
  });

  const exports = instance.exports as Record<string, CallableFunction>;

  // Wrap raw WASM exports into a typed SourceExtension
  return {
    manifest,
    search: async (query) => {
      const result = await exports.search(query);
      return JSON.parse(result as string);
    },
    getByAnilistId: async (anilistId) => {
      const result = await exports.get_by_anilist_id(anilistId);
      return result ? JSON.parse(result as string) : null;
    },
    getAnimeDetails: async (id) => {
      const result = await exports.get_anime_details(id);
      return JSON.parse(result as string);
    },
    getEpisodes: async (id, page = 1) => {
      const result = await exports.get_episodes(id, page);
      return JSON.parse(result as string);
    },
    getStreamSources: async (animeId, episodeId) => {
      const result = await exports.get_stream_sources(animeId, episodeId);
      return JSON.parse(result as string);
    },
    resolveStream: async (source) => {
      const result = await exports.resolve_stream(JSON.stringify(source));
      return JSON.parse(result as string);
    },
    checkAuth: async () => {
      return (exports.check_auth as () => Promise<boolean>)();
    },
  };
}
```

---

## JS Extension Loader (`JsExtensionLoader.ts`)

For TypeScript/JS extensions (simpler, no WASM compilation step):

```typescript
import type { SourceExtension, ExtensionManifest } from '$lib/types/extensions';

export async function loadJsExtension(jsCode: string, manifest: ExtensionManifest): Promise<SourceExtension> {
  // Create a sandboxed module via blob URL
  const blob = new Blob([jsCode], { type: 'application/javascript' });
  const url = URL.createObjectURL(blob);

  try {
    const module = await import(/* @vite-ignore */ url);
    const ext = module.default ?? module.extension;

    if (!ext) throw new Error(`JS extension ${manifest.id} has no default or .extension export`);

    return { ...ext, manifest };
  } finally {
    URL.revokeObjectURL(url);
  }
}
```

---

## Extension Storage (Rust side)

New Tauri commands in `src-tauri/src/commands/extensions.rs`:

```rust
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{command, State, AppHandle};
use crate::config::ConfigState;

/// Simple in-memory + persisted KV store per extension.
/// Backed by the same RON config file system we already have.
pub struct ExtensionStorage(pub Mutex<HashMap<String, HashMap<String, String>>>);

#[command]
pub async fn ext_storage_get(
    ext_id: String,
    key: String,
    storage: State<'_, ExtensionStorage>,
) -> Result<Option<String>, String> {
    let guard = storage.0.lock().unwrap();
    Ok(guard.get(&ext_id).and_then(|m| m.get(&key)).cloned())
}

#[command]
pub async fn ext_storage_set(
    ext_id: String,
    key: String,
    value: String,
    storage: State<'_, ExtensionStorage>,
) -> Result<(), String> {
    let mut guard = storage.0.lock().unwrap();
    guard.entry(ext_id).or_default().insert(key, value);
    Ok(())
}
```

---

## Cookie-Auth Flow (AnimePahe)

```
User presses "Connect to AnimePahe"
        │
        ▼
Tauri opens a WebviewWindow to https://animepahe.si
  (uses tauri-plugin-shell or WebviewWindow API)
        │
        ▼
User solves Cloudflare challenge
        │
        ▼
Tauri backend reads session cookies via
  webview.cookies() → forwards to ext_storage_set("animepahe", "cookies", ...)
        │
        ▼
AnimePahExtension.onCookiesUpdated(cookies) is called
  → extension caches cookies for subsequent requests
        │
        ▼
All subsequent fetch calls include Cookie header
  through host_fetch_url (Rust backend, no CORS)
```

---

## Extension Discovery & Loading Flow

```
App startup
  │
  ├── 1. Load built-in extensions from src/lib/extensions/
  │        (compiled into the app, no .zext file needed)
  │
  ├── 2. Scan ~/.config/zafkiel/extensions/ for .zext files
  │        (user-installed extensions)
  │
  └── 3. For each .zext:
           a. unzip in memory
           b. validate manifest schema
           c. pick WasmLoader or JsLoader
           d. instantiate → register in ExtensionRegistry
```

---

## File Layout (when implemented)

```
src/
  lib/
    extensions/
      loader/
        WasmExtensionLoader.ts     ← new
        JsExtensionLoader.ts       ← new
        ExtensionLoader.ts         ← orchestrator (new)
      torrent/
        subsplease.ts              ← existing (TypeScript extension)
        NyaaProvider.ts            ← existing
      source/
        AnimepaheExtension/        ← new (Rust WASM)
          Cargo.toml
          src/
            lib.rs
            scraper.rs
            auth.rs
    types/
      extensions.ts                ← new (the interface file)
    services/
      ExtensionManager.ts          ← refactor
      EpisodeMetadataService.ts    ← new (AniZip + Tosho)
      TorrentService.ts            ← extend (episode-aware)

src-tauri/
  src/
    commands/
      extensions.rs                ← new (storage + fetch proxy)
      torrent.rs                   ← extend (per-file tracking)
```

---

## Security Considerations

| Threat | Mitigation |
|--------|-----------|
| Malicious WASM reading OS files | WASM has no file system access; only `HostAPI` imports are available |
| Malicious extension exfiltrating auth tokens | `fetch_url` goes through Rust, which can check allowlisted domains per extension manifest |
| Prototype pollution from dynamic JS import | JS extension loader uses blob URLs in a new module scope; no `eval` |
| Infinite loops in extension | `WebAssembly.instantiate` can be wrapped in a 30s timeout |

---

## Open Questions (to resolve before implementation)

1. **Domain allowlist**: should we let the manifest declare which domains it can fetch, with Rust enforcing this?  E.g. `"allowedDomains": ["animepahe.si", "kwik.si"]`  → Yes, recommended.
2. **wasm-pack vs manual**: use `wasm-pack build --target web` for Rust extensions.  Keep JS path for lighter 1st-party providers.
3. **Extension update mechanism**: not scoped for v1 – manual `.zext` replacement is fine.
4. **Capability negotiation**: if host is missing a required capability (e.g. `cookieAuth`), fail loudly at load-time with a clear error.
