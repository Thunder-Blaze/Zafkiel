/**
 * ToshoProvider
 *
 * A TorrentExtension implementation backed by the Anime Tosho API.
 * This is the primary episode-aware torrent provider for Zafkiel.
 *
 * Unlike the basic NyaaProvider (which does free-text search only),
 * ToshoProvider supports precise per-episode lookups via AniDB IDs
 * obtained from the AniZip mappings API.
 */

import type { TorrentProvider, ExtensionManifest, TorrentInfo } from './ExtensionManager';
import type { AnimeLarge } from '$lib/types/anime';
import {
	searchTosho,
	fetchToshoEpisode,
	fetchToshoBatches,
	type EpisodeTorrentEntry,
} from './EpisodeMetadataService';
import { filterTitle } from '$lib/utils/data-filters';

export type TorrentQuality = '720p' | '1080p' | 'all';

// ─────────────────────────────────────────────────────────────────────────────

export class ToshoProvider implements TorrentProvider {
	readonly manifest: ExtensionManifest = {
		id: 'animetosho',
		name: 'Anime Tosho',
		version: '1.0.0',
		author: 'Zafkiel',
		type: 'torrent',
		description:
			'Episode-precise torrent search via Anime Tosho with AniDB ID integration. ' +
			'Falls back to Nyaa SubsPlease releases when Tosho has no results.',
	};

	// ── TorrentProvider interface ──────────────────────────────────────────

	/**
	 * Free-text search on Anime Tosho.
	 * Used by the manual search dialog.
	 */
	async search(query: string): Promise<TorrentInfo[]> {
		const entries = await searchTosho(query);
		return entries.map(toTorrentInfo);
	}

	/**
	 * Anime-level search – finds SubsPlease releases for the given anime title.
	 * For precise per-episode results use `searchEpisode()` instead.
	 */
	async searchAnime(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const title = filterTitle(anime.title ?? {});
		if (!title) return [];

		// Try primary title, fall back to romaji
		let results = await searchTosho(title);
		if (results.length === 0 && anime.title?.romaji && anime.title.romaji !== title) {
			results = await searchTosho(anime.title.romaji);
		}

		return results.map(toTorrentInfo);
	}

	async searchBatches(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const title = filterTitle(anime.title ?? {});
		if (!title) return [];
		const entries = await fetchToshoBatches(title);
		return entries.map(toTorrentInfo);
	}

	// ── Extended episode-precise search ───────────────────────────────────

	/**
	 * Search for torrents for a specific episode.
	 *
	 * When `anidbId` and `anidbEpisodeId` are provided (from AniZip) the
	 * lookup is precise and fast.
	 */
	async searchEpisode(
		anime: AnimeLarge,
		episodeNumber: number,
		anidbId?: number,
		anidbEpisodeId?: number,
		quality: TorrentQuality = 'all',
		absoluteEpisodeNumber?: number
	): Promise<TorrentInfo[]> {
		if (anidbId) {
			const entries = await fetchToshoEpisode(anidbId, anidbEpisodeId ?? null, quality);
			if (entries.length > 0) return entries.map(toTorrentInfo);
		}

		// No fallback here; fallback is handled by ExtensionManager trying the next provider
		return [];
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

function toTorrentInfo(e: EpisodeTorrentEntry): TorrentInfo {
	return {
		title: e.title,
		size: e.size,
		seeds: e.seeds,
		peers: e.peers,
		magnet: e.magnetUri,
		provider: e.provider,
		uploadedAt: e.uploadedAt,
		episode: parseEpisodeFromTitle(e.title),
		resolution: e.resolution,
		fansub: e.fansub,
		anidbId: e.anidbId,
		anidbEpisodeId: e.anidbEpisodeId,
	};
}

function parseEpisodeFromTitle(title: string): number | undefined {
	// "[SubsPlease] Show - 04 (1080p)" → 4
	const m = title.match(/\s-\s(\d{1,4})(?:v\d)?[\s\(]/);
	if (m) return parseInt(m[1], 10);
	return undefined;
}

export const toshoProvider = new ToshoProvider();
