/**
 * Core type definitions for the Zafkiel extension system.
 *
 * Extensions are standalone packages distributed as `.zext` files
 * (a renamed ZIP containing manifest.json + extension.js or extension.wasm).
 * Once downloaded and extracted they live in:
 *   ~/.config/zafkiel/extensions/{id}/
 */

// ─────────────────────────────────────────────────────────────────────────────
// Manifests & Registry
// ─────────────────────────────────────────────────────────────────────────────

export type ExtensionType = 'source' | 'torrent' | 'other';

/** Mirrors `ExtensionManifest` in Rust (`commands/extensions.rs`). */
export interface ExtensionManifest {
	readonly id: string;
	readonly name: string;
	readonly version: string;
	readonly author: string;
	readonly description?: string;
	readonly type: ExtensionType;
	/** Filename of the entry-point: "extension.js" or "extension.wasm" */
	readonly entry: string;
	readonly minAppVersion?: string;
	/** Settings schema for the extension */
	readonly settings?: ExtensionSetting[];
}
export interface ExtensionSetting {
	id: string;
	label: string;
	type: 'string' | 'number' | 'boolean' | 'select';
	default: any;
	options?: { label: string; value: any }[];
	description?: string;

}
/** Mirrors `ExtensionIndexEntry` in Rust – represents an installed extension. */
export interface ExtensionIndexEntry {
	readonly id: string;
	readonly name: string;
	readonly version: string;
	readonly author: string;
	readonly description?: string;
	readonly type: ExtensionType;
	/** Absolute filesystem path returned by Tauri. */
	readonly path: string;
	/** Unix timestamp (seconds) of when the extension was installed. */
	readonly installedAt: number;
	readonly entry: string;
	readonly minAppVersion?: string;
}

/** Extension listing from the (currently hardcoded) store catalog. */
export interface CatalogExtension {
	readonly id: string;
	readonly name: string;
	readonly latestVersion: string;
	readonly author: string;
	/** Settings schema for the extension */
	readonly settings?: ExtensionSetting[];
	readonly description: string;
	readonly type: ExtensionType;
	/** Direct URL to the .zext bundle. */
	readonly downloadUrl: string;
	/** Optional icon shown in the extensions page. */
	readonly iconUrl?: string;
	readonly tags?: readonly string[];
	/** Whether the extension needs a special auth setup (e.g. Cloudflare cookies). */
	readonly requiresAuth?: boolean;
	readonly language?: string;
	/** Dev-only: absolute path to a local .zext for testing without a remote URL. */
	readonly localPath?: string;
}

// ─────────────────────────────────────────────────────────────────────────────
// Extension lifecycle state
// ─────────────────────────────────────────────────────────────────────────────

export type ExtensionStatus =
	| { kind: 'uninstalled' }
	| { kind: 'downloading'; progress: number | null }
	| { kind: 'installed'; entry: ExtensionIndexEntry }
	/** Extension files exist; JS module is being dynamically imported. */
	| { kind: 'loading' }
	| { kind: 'ready'; entry: ExtensionIndexEntry }
	| {
			kind: 'error';
			entry?: ExtensionIndexEntry;
			message: string;
			/** True when .bundle.zext exists and reinstall without download is possible. */
			canReinstall: boolean;
	  };

// ─────────────────────────────────────────────────────────────────────────────
// Source Extension API
// ─────────────────────────────────────────────────────────────────────────────

export interface SearchResult {
	readonly id: string;
	readonly title: string;
	readonly coverUrl?: string;
	readonly type?: 'TV' | 'Movie' | 'OVA' | 'ONA' | 'Special' | string;
	readonly status?: 'Ongoing' | 'Completed' | 'Upcoming' | string;
	readonly year?: number;
}

export interface AnimeDetails {
	readonly id: string;
	readonly title: string;
	readonly coverUrl?: string;
	readonly description?: string;
	readonly anilistId?: number;
	readonly sessionId?: string;
}

export interface Episode {
	readonly id: string;
	readonly number: number;
	readonly title?: string;
	readonly thumbnailUrl?: string;
	readonly airDate?: string;
	readonly slug?: string;
	readonly snapshotUrl?: string;
}

export interface Page<T> {
	readonly data: T[];
	readonly currentPage: number;
	readonly lastPage: number;
	readonly total: number;
	readonly perPage: number;
}

export interface StreamSource {
	readonly id: string;
	readonly label: string;
	readonly resolution?: string;
	readonly fansub?: string;
	readonly audio?: string;
	/** Whether resolveStream() must be called to get the final URL. */
	readonly requiresResolution: boolean;
}

export interface ResolvedStream {
	readonly url: string;
	/** Stream type: "hls" (.m3u8), "mp4" (progressive), etc. */
	readonly type: 'hls' | 'mp4' | string;
	readonly headers?: Record<string, string>;
}

export interface CookieEntry {
	readonly name: string;
	readonly value: string;
	readonly domain?: string;
	readonly path?: string;
}

/** Interface that any source extension JS module must export. */
export interface SourceExtension {
	readonly manifest: ExtensionManifest;
	onCookiesUpdated?(cookies: CookieEntry[]): Promise<void> | void;
	/** Optional: called after the user completes the Cloudflare challenge on kwik.si. */
	onKwikCookiesUpdated?(cookiesString: string): Promise<void> | void;
	checkAuth?(): Promise<boolean>;
	search(query: string): Promise<SearchResult[]>;
	getAnimeDetails(id: string): Promise<AnimeDetails>;
	getByAnilistId?(anilistId: number): Promise<AnimeDetails | null>;
	getEpisodes(animeId: string, page?: number): Promise<Page<Episode>>;
	getAllEpisodes?(animeId: string): Promise<Episode[]>;
	getStreamSources(animeId: string, episodeId: string): Promise<StreamSource[]>;
	resolveStream(source: StreamSource): Promise<ResolvedStream>;
}

// ─────────────────────────────────────────────────────────────────────────────
// Downloads
// ─────────────────────────────────────────────────────────────────────────────

/** Parameters passed to the `start_extension_download` Tauri command. */
export interface StartDownloadParams {
	animeName: string;
	anilistId?: number;
	/** Season number (1-based; most anime = 1). */
	season: number;
	/** Episode number; may be a decimal (e.g. 5.5 for specials). */
	episodeNumber: number;
	/** Human-readable quality label, e.g. "HorribleSubs 1080p (JPN)". */
	sourceLabel: string;
	extensionId: string;
	/** Resolved HLS / MP4 URL returned by resolveStream(). */
	url: string;
	headers: Record<string, string>;
}

/** Mirrors the `ExtensionDownload` Rust struct returned by `get_extension_downloads`. */
export interface ExtensionDownload {
	readonly id: number;
	readonly animeName: string;
	readonly anilistId?: number;
	readonly season: number;
	readonly episodeNumber: number;
	readonly sourceLabel: string;
	readonly extensionId: string;
	readonly status: 'pending' | 'downloading' | 'completed' | 'failed' | 'cancelled';
	readonly progress: number;
	readonly filePath?: string;
	readonly errorMsg?: string;
	readonly createdAt: number;
	readonly updatedAt: number;
}

/** Payload of the `extension-download-progress` Tauri event. */
export interface DownloadProgressEvent {
	readonly id: number;
	readonly progress: number;
	readonly status: ExtensionDownload['status'];
	readonly errorMsg?: string;
	readonly filePath?: string;
}
