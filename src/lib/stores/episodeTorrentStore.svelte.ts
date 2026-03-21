/**
 * episodeTorrentStore
 *
 * Svelte 5 runes-based store that links episode numbers to active torrent IDs.
 *
 * When the user clicks "Download Episode N":
 *   1. A torrent is added to librqbit (via TorrentService)
 *   2. The magnet + episode number are stored here
 *   3. The anime detail page polls TorrentService and updates progress
 *   4. When the file is ≥ 10% downloaded a "Watch" button appears
 */

import type { TorrentFile } from '$lib/services/TorrentService';
import { parseEpisodeNumber } from '$lib/services/EpisodeMetadataService';

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

export interface EpisodeTorrentLink {
	/** Episode number (e.g. 4) */
	episodeNumber: number;
	/** librqbit torrent ID */
	torrentId: number;
	/** librqbit file index within the torrent */
	fileId?: number;
	/** Original filename (for re-identification) */
	fileName?: string;
	/** Magnet URI used when adding the torrent */
	magnetUri: string;
}

export interface EpisodeDownloadStatus {
	episodeNumber: number;
	torrentId: number;
	fileId?: number;
	/** 0–1 */
	progress: number;
	/** bytes/s */
	downloadSpeed: number;
	state: 'downloading' | 'paused' | 'seeding' | 'error' | 'done';
	/** True when progress ≥ 0.1 (can start streaming) */
	canStream: boolean;
}

// ─────────────────────────────────────────────────────────────────────────────
// Store state (Svelte 5 runes)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Maps:  anilistId → Map<episodeNumber, EpisodeTorrentLink>
 */
const _links = $state<Map<number, Map<number, EpisodeTorrentLink>>>(new Map());

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

export const episodeTorrentStore = {
	/**
	 * Associate an episode with a torrent + optional file.
	 */
	link(anilistId: number, link: EpisodeTorrentLink): void {
		let animeMap = _links.get(anilistId);
		if (!animeMap) {
			animeMap = new Map();
			_links.set(anilistId, animeMap);
		}
		animeMap.set(link.episodeNumber, link);
	},

	/**
	 * Look up the active torrent for a given episode.
	 */
	getLink(anilistId: number, episodeNumber: number): EpisodeTorrentLink | undefined {
		return _links.get(anilistId)?.get(episodeNumber);
	},

	/**
	 * Get all linked episodes for an anime.
	 */
	getAnimeLinks(anilistId: number): EpisodeTorrentLink[] {
		const animeMap = _links.get(anilistId);
		if (!animeMap) return [];
		return Array.from(animeMap.values());
	},

	/**
	 * Remove the link for a specific episode (e.g. after torrent deletion).
	 */
	unlink(anilistId: number, episodeNumber: number): void {
		_links.get(anilistId)?.delete(episodeNumber);
	},

	/**
	 * Remove all links for an anime.
	 */
	clearAnime(anilistId: number): void {
		_links.delete(anilistId);
	},

	/**
	 * Auto-detect episodes from a torrent's file list and create links.
	 *
	 * librqbit returns TorrentFile[] for a torrent; this helper maps those
	 * files back to episode numbers using filename parsing.
	 */
	autoLinkFromFiles(
		anilistId: number,
		torrentId: number,
		magnetUri: string,
		files: TorrentFile[]
	): void {
		for (const file of files) {
			const epNum = parseEpisodeNumber(file.name);
			if (epNum === null) continue;

			this.link(anilistId, {
				episodeNumber: epNum,
				torrentId,
				fileId: file.id,
				fileName: file.name,
				magnetUri,
			});
		}
	},

	/**
	 * Reactive snapshot of the full links map (use in $derived).
	 * Reading this inside a Svelte 5 effect / template will re-run when changed.
	 */
	get snapshot(): Map<number, Map<number, EpisodeTorrentLink>> {
		return _links;
	},
};
