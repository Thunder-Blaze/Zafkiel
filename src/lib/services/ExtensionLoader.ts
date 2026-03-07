/**
 * ExtensionLoader – loads installed extensions from disk at runtime.
 *
 * Supports two extension types:
 *
 * **JS extensions** — `entry: "extension.js"` exports a plain `default` object
 *   implementing `SourceExtension` directly.
 *
 * **WASM extensions** (wasm-bindgen) — `entry: "extension.js"` but `default`
 *   is the wasm-bindgen `init()` function.  The loader:
 *   1. Injects `window.__zafkiel_host__` with Tauri IPC-backed HostAPI.
 *   2. Dynamically imports the JS glue module.
 *   3. Detects presence of `init` (function default export) → WASM extension.
 *   4. Calls `await init(wasmAssetUrl)` to link the WASM binary.
 *   5. Wraps the raw exports (which return JSON strings) into `SourceExtension`.
 */

import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import type {
	ExtensionIndexEntry,
	ExtensionManifest,
	SourceExtension,
	SearchResult,
	AnimeDetails,
	Episode,
	Page,
	StreamSource,
	ResolvedStream,
	CookieEntry,
} from '$lib/types/extensions';

// ─────────────────────────────────────────────────────────────────────────────
// Error types
// ─────────────────────────────────────────────────────────────────────────────

export type ExtensionErrorKind =
	| 'entry-missing'     // manifest.entry file not found on disk
	| 'import-failed'     // dynamic import threw (syntax / runtime error)
	| 'no-export'         // JS module has no `default` export
	| 'version-mismatch'  // minAppVersion > current app version
	| 'unknown';

export class ExtensionLoadError extends Error {
	constructor(
		public readonly extensionId: string,
		public readonly kind: ExtensionErrorKind,
		message: string,
		public readonly canReinstall: boolean = true,
	) {
		super(message);
		this.name = 'ExtensionLoadError';
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// HostAPI injection (WASM extensions only)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Injects `window.__zafkiel_host__` before loading a WASM extension.
 * The wasm-bindgen glue calls these functions via the global namespace.
 */
function injectHostAPI(extId: string): void {
	(window as unknown as Record<string, unknown>)['__zafkiel_host__'] = {
		fetchUrl: async (url: string, headersJson: string): Promise<string> => {
			const headers = JSON.parse(headersJson) as Record<string, string>;
			return await invoke<string>('fetch_url', { url, headers });
		},
		storageGet: async (key: string): Promise<string | null> => {
			return await invoke<string | null>('ext_storage_get', { extId, key });
		},
		storageSet: async (key: string, value: string): Promise<void> => {
			await invoke<void>('ext_storage_set', { extId, key, value });
		},
		log: (level: string, message: string): void => {
			const fn = (console as unknown as Record<string, unknown>)[level];
			if (typeof fn === 'function') (fn as (...a: unknown[]) => void).call(console, `[ext:${extId}]`, message);
			else console.log(`[ext:${extId}] [${level}]`, message);
		},
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// WASM wrapper
// ─────────────────────────────────────────────────────────────────────────────

type WasmExports = {
	check_auth?: () => Promise<boolean>;
	search: (query: string) => Promise<string>;
	get_anime_details: (session: string) => Promise<string>;
	get_episodes: (session: string, page: number) => Promise<string>;
	get_stream_sources: (animeSession: string, epSession: string) => Promise<string>;
	resolve_stream: (sourceJson: string) => Promise<string>;
	on_cookies_updated?: (cookies: string) => Promise<void>;
	on_kwik_cookies_updated?: (cookies: string) => Promise<void>;
	get_manifest: () => string;
};

/** Builds a `SourceExtension` by wrapping raw WASM exports (which return JSON strings). */
function buildWasmWrapper(entry: ExtensionIndexEntry, raw: WasmExports): SourceExtension {
	const manifest: ExtensionManifest = {
		id: entry.id,
		name: entry.name,
		version: entry.version,
		author: entry.author,
		description: entry.description,
		type: entry.type,
		entry: entry.entry,
		minAppVersion: entry.minAppVersion,
	};

	return {
		manifest,

		checkAuth: raw.check_auth
			? () => raw.check_auth!()
			: undefined,

		onCookiesUpdated: raw.on_cookies_updated
			? async (cookies: CookieEntry[]) => {
					// Serialize CookieEntry[] → "name=value; name2=value2" cookie header string
					const str = cookies.map((c) => `${c.name}=${c.value}`).join('; ');
					await raw.on_cookies_updated!(str);
				}
			: undefined,

		// Pass the raw cookie string directly — kwik cookies come pre-formatted
		onKwikCookiesUpdated: raw.on_kwik_cookies_updated
			? async (cookiesString: string) => {
					await raw.on_kwik_cookies_updated!(cookiesString);
				}
			: undefined,

		search: async (query: string): Promise<SearchResult[]> =>
			JSON.parse(await raw.search(query)),

		getAnimeDetails: async (id: string): Promise<AnimeDetails> =>
			JSON.parse(await raw.get_anime_details(id)),

		getEpisodes: async (animeId: string, page = 1): Promise<Page<Episode>> =>
			JSON.parse(await raw.get_episodes(animeId, page)),

		getStreamSources: async (animeId: string, episodeId: string): Promise<StreamSource[]> =>
			JSON.parse(await raw.get_stream_sources(animeId, episodeId)),

		resolveStream: async (source: StreamSource): Promise<ResolvedStream> =>
			JSON.parse(await raw.resolve_stream(JSON.stringify(source))),
	};
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

class ExtensionLoaderService {
	/** Already-loaded instances, keyed by extension id. */
	private readonly loaded = new Map<string, SourceExtension>();

	/**
	 * Loads an installed extension by ID.
	 *
	 * Returns the cached instance on subsequent calls.
	 * Throws `ExtensionLoadError` on any failure.
	 */
	async load(entry: ExtensionIndexEntry): Promise<SourceExtension> {
		if (this.loaded.has(entry.id)) {
			return this.loaded.get(entry.id)!;
		}

		// 1. Resolve absolute entry-point path via Tauri
		let absolutePath: string;
		try {
			absolutePath = await invoke<string>('get_extension_entry_path', { id: entry.id });
		} catch (err) {
			throw new ExtensionLoadError(
				entry.id,
				'entry-missing',
				`Extension "${entry.name}": entry file is missing. Try reinstalling. (${err})`,
				true,
			);
		}

		// 2. Convert to an asset:// URL the webview can fetch
		const assetUrl = convertFileSrc(absolutePath);

		// 3. For WASM extensions, inject HostAPI into global scope first
		const isWasmBacked = entry.entry === 'extension.js';
		if (isWasmBacked) {
			injectHostAPI(entry.id);
		}

		// 4. Dynamically import the JS module
		let mod: Record<string, unknown>;
		try {
			mod = await import(/* @vite-ignore */ assetUrl);
		} catch (err) {
			const msg = err instanceof Error ? err.message : String(err);
			throw new ExtensionLoadError(
				entry.id,
				'import-failed',
				`Extension "${entry.name}": failed to load module — ${msg}`,
				true,
			);
		}

		// 5a. WASM extension: `default` is the wasm-bindgen init() function
		if (isWasmBacked && typeof mod.default === 'function') {
			const dir = absolutePath.replace(/\/[^/]+$/, '');
			const wasmAssetUrl = convertFileSrc(`${dir}/extension.wasm`);

			try {
				await (mod.default as (url: string) => Promise<unknown>)(wasmAssetUrl);
			} catch (err) {
				const msg = err instanceof Error ? err.message : String(err);
				throw new ExtensionLoadError(
					entry.id,
					'import-failed',
					`Extension "${entry.name}": WASM initialization failed — ${msg}`,
					true,
				);
			}

			const raw = mod as unknown as WasmExports;
			const instance = buildWasmWrapper(entry, raw);
			this.loaded.set(entry.id, instance);
			return instance;
		}

		// 5b. Plain JS extension: expect a default object export
		const instance = (mod.default ?? mod[entry.id]) as SourceExtension | undefined;
		if (!instance || typeof instance !== 'object') {
			throw new ExtensionLoadError(
				entry.id,
				'no-export',
				`Extension "${entry.name}": module has no valid default export. ` +
					`Make sure extension.js exports an object as \`export default\`.`,
				false,
			);
		}

		this.loaded.set(entry.id, instance);
		return instance;
	}

	/** Evicts a cached instance (e.g. after reinstall). */
	evict(id: string): void {
		this.loaded.delete(id);
	}

	/** Whether an extension has already been loaded into memory. */
	isLoaded(id: string): boolean {
		return this.loaded.has(id);
	}

	/** Returns the already-loaded instance, or null if not yet loaded. */
	get(id: string): SourceExtension | null {
		return this.loaded.get(id) ?? null;
	}
}

export const ExtensionLoader = new ExtensionLoaderService();
