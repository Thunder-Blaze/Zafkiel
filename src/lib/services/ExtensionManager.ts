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
	searchEpisode(
		anime: AnimeLarge,
		episodeNumber: number,
		anidbId?: number,
		anidbEpisodeId?: number,
		quality?: string,
		absoluteEpisodeNumber?: number
	): Promise<TorrentInfo[]>;
	searchBatches?(anime: AnimeLarge): Promise<TorrentInfo[]>;
}

class ExtensionManagerService {
	private providers: Map<string, TorrentProvider> = new Map();
	private prioritizedIds: string[] = ['animetosho', 'nyaa-si'];

	constructor() {
		this.registerProvider(new ToshoProvider());
		this.registerProvider(new NyaaProvider());
	}

	registerProvider(provider: TorrentProvider) {
		this.providers.set(provider.manifest.id, provider);
		console.log(`[Extensions] Registered: ${provider.manifest.name}`);
	}

	getProviders(): TorrentProvider[] {
		// Return providers in the preferred order, then any others
		const ordered = this.prioritizedIds
			.map((id) => this.providers.get(id))
			.filter((p): p is TorrentProvider => !!p);
		
		const others = Array.from(this.providers.values())
			.filter((p) => !this.prioritizedIds.includes(p.manifest.id));

		return [...ordered, ...others];
	}

	getProvider(id: string): TorrentProvider | undefined {
		return this.providers.get(id);
	}

	async searchAll(query: string): Promise<TorrentInfo[]> {
		const providers = this.getProviders();
		const promises = providers.map((p) =>
			p.search(query).catch((e) => {
				console.error(`[Search] ${p.manifest.name} error:`, e);
				return [];
			})
		);

		const results = await Promise.all(promises);
		return results.flat().sort((a, b) => b.seeds - a.seeds);
	}

	async searchAnimeAll(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const providers = this.getProviders();
		const promises = providers.map((p) =>
			p.searchAnime(anime).catch((e) => {
				console.error(`[SearchAnime] ${p.manifest.name} error:`, e);
				return [];
			})
		);

		const results = await Promise.all(promises);
		return results.flat().sort((a, b) => b.seeds - a.seeds);
	}

	async searchBatchesAll(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const providers = this.getProviders();
		const promises = providers.map((p) => {
			if (p.searchBatches) {
				return p.searchBatches(anime).catch((e) => {
					console.error(`[Batches] ${p.manifest.name} error:`, e);
					return [];
				});
			}
			const query = (anime.title?.english || anime.title?.romaji || '') + ' Batch';
			return p.search(query).catch(() => []);
		});

		const results = await Promise.all(promises);
		return results.flat().sort((a, b) => b.seeds - a.seeds);
	}
}

export const ExtensionManager = new ExtensionManagerService();
