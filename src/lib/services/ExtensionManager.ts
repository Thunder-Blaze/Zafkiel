import type { AnimeLarge } from '$lib/types/anime';
import { NyaaProvider } from './NyaaProvider';
import { ToshoProvider } from './ToshoProvider';

export interface ExtensionManifest {
	id: string;
	name: string;
	version: string;
	author: string;
	description?: string;
	type: 'torrent' | 'source' | 'other';
}

export interface TorrentInfo {
	title: string;
	size: string;
	seeds: number;
	peers: number;
	magnet: string;
	provider: string;
	uploadedAt?: string;

	// ── Episode-aware fields (populated when available) ──────────────────
	/** Parsed episode number from the torrent title */
	episode?: number;
	/** Parsed resolution string (e.g. "1080p") */
	resolution?: string;
	/** Fansub group name (e.g. "SubsPlease") */
	fansub?: string;
	/** AniDB series ID – enables precise Tosho re-queries */
	anidbId?: number;
	/** AniDB episode ID – enables precise Tosho re-queries */
	anidbEpisodeId?: number;
}

export interface TorrentProvider {
	manifest: ExtensionManifest;
	search(query: string): Promise<TorrentInfo[]>;
	searchAnime(anime: AnimeLarge): Promise<TorrentInfo[]>;
}

class ExtensionManagerService {
	private providers: Map<string, TorrentProvider> = new Map();

	constructor() {
		// Built-in providers – ordered by priority (Tosho first for precision)
		this.registerProvider(new ToshoProvider());
		this.registerProvider(new NyaaProvider());
	}

	registerProvider(provider: TorrentProvider) {
		if (this.providers.has(provider.manifest.id)) {
			console.warn(`Provider ${provider.manifest.id} already registered, overwriting.`);
		}
		this.providers.set(provider.manifest.id, provider);
		console.log(`Registered provider: ${provider.manifest.name}`);
	}

	getProviders(): TorrentProvider[] {
		return Array.from(this.providers.values());
	}

	getProvider(id: string): TorrentProvider | undefined {
		return this.providers.get(id);
	}

	async searchAll(query: string): Promise<TorrentInfo[]> {
		const promises = Array.from(this.providers.values()).map((p) =>
			p.search(query).catch((e) => {
				console.error(`Error searching provider ${p.manifest.name}:`, e);
				return [];
			})
		);

		const results = await Promise.all(promises);
		return results.flat().sort((a, b) => b.seeds - a.seeds);
	}

	async searchAnimeAll(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const promises = Array.from(this.providers.values()).map((p) =>
			p.searchAnime(anime).catch((e) => {
				console.error(`Error searching provider ${p.manifest.name}:`, e);
				return [];
			})
		);

		const results = await Promise.all(promises);
		return results.flat().sort((a, b) => b.seeds - a.seeds);
	}
}

export const ExtensionManager = new ExtensionManagerService();
