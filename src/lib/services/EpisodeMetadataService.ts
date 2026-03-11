/**
 * EpisodeMetadataService
 *
 * Provides episode metadata (titles, thumbnails, AniDB IDs) via the AniZip API
 * and episode-precise torrent listings via the Anime Tosho API.
 *
 * This mirrors the logic from Zenshin's `getAniZipMappings` / `getToshoEpisodes`
 * but integrated natively into Zafkiel's Tauri-based architecture.
 */

import { invoke } from '@tauri-apps/api/core';

// ─────────────────────────────────────────────────────────────────────────────
// API constants
// ─────────────────────────────────────────────────────────────────────────────

const ANIZIP_BASE = 'https://api.ani.zip';
const TOSHO_BASE = 'https://feed.animetosho.org';
const NYAA_RSS_BASE = 'https://nyaa.si';

// ─────────────────────────────────────────────────────────────────────────────
// AniZip types
// ─────────────────────────────────────────────────────────────────────────────

export interface AniZipEpisode {
	/** String (e.g. "S1", "C1") or number */
	episode: number | string;
	/** AniDB Episode ID – used for precise Tosho lookup */
	anidbEid: number;
	title: {
		en?: string;
		ja?: string;
		'x-jat'?: string;
	};
	/** Thumbnail image URL */
	image?: string;
	/** ISO 8601 air date */
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

// ─────────────────────────────────────────────────────────────────────────────
// Normalised episode metadata (used throughout the app)
// ─────────────────────────────────────────────────────────────────────────────

export interface EpisodeMeta {
	/** Sequential episode number (e.g. 1, 2, 12.5) */
	number: number;
	/** Episode title, preferring English */
	title: string;
	/** Thumbnail / snapshot URL */
	thumbnailUrl?: string;
	/** ISO 8601 air date */
	airDate?: string;
	/** Episode synopsis */
	overview?: string;
	/** AniDB Series ID – needed for Tosho lookup */
	anidbId: number;
	/** AniDB Episode ID – needed for precise Tosho lookup */
	anidbEpisodeId: number;
}

// ─────────────────────────────────────────────────────────────────────────────
// Tosho / torrent types
// ─────────────────────────────────────────────────────────────────────────────

export interface EpisodeTorrentEntry {
	title: string;
	magnetUri: string;
	/** Human-readable file size (e.g. "1.40 GiB") */
	size: string;
	/** Raw size in bytes */
	sizeBytes: number;
	seeds: number;
	peers: number;
	uploadedAt: string;
	/** Parsed resolution from title (e.g. "1080p") */
	resolution?: string;
	/** Fansub group from title (e.g. "SubsPlease") */
	fansub?: string;
	/** AniDB series ID confirmed by Tosho */
	anidbId?: number;
	/** AniDB episode ID confirmed by Tosho */
	anidbEpisodeId?: number;
	/** Source provider name */
	provider: 'Anime Tosho' | 'Nyaa.si';
}

export type TorrentQuality = '720p' | '1080p' | 'all';

// ─────────────────────────────────────────────────────────────────────────────
// AniZip API
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fetch raw AniZip mappings for an AniList ID.
 * Throws on HTTP errors; returns null if not found (404).
 */
export async function fetchAniZipMappings(anilistId: number): Promise<AniZipMappings | null> {
	const url = `${ANIZIP_BASE}/mappings?anilist_id=${anilistId}`;
	try {
		const raw = await invoke<string>('fetch_url', { url, headers: null });
		const data = JSON.parse(raw) as AniZipMappings;
		return data;
	} catch (err) {
		const msg = String(err);
		if (msg.includes('404') || msg.includes('Not Found')) return null;
		throw err;
	}
}

/**
 * Convert raw AniZip mappings into a sorted, normalised `EpisodeMeta[]`.
 * Skips specials, OPs, and episodes with non-numeric numbers (<= 0).
 */
export function buildEpisodeMetas(mappings: AniZipMappings): EpisodeMeta[] {
	const anidbId = mappings.mappings.anidb_id;

	return Object.values(mappings.episodes)
		.map((ep): EpisodeMeta | null => {
			const rawNum = ep.episode;
			const num = typeof rawNum === 'string' ? parseFloat(rawNum) : rawNum;

			// Skip non-numeric entries (specials like "S1", "C1")
			if (isNaN(num) || num <= 0) return null;

			return {
				number: num,
				title: ep.title.en ?? ep.title['x-jat'] ?? ep.title.ja ?? `Episode ${num}`,
				thumbnailUrl: ep.image ?? undefined,
				airDate: ep.airdate ?? undefined,
				overview: ep.overview ?? undefined,
				anidbId,
				anidbEpisodeId: ep.anidbEid,
			};
		})
		.filter((ep): ep is EpisodeMeta => ep !== null)
		.sort((a, b) => a.number - b.number);
}

// ─────────────────────────────────────────────────────────────────────────────
// Anime Tosho API
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fetch torrent entries for a specific episode via AniDB IDs.
 *
 * This is the primary ("precise") lookup path.
 * When `anidbEpisodeId` is null the query is series-level (all episodes).
 */
export async function fetchToshoEpisode(
	anidbId: number,
	anidbEpisodeId: number | null,
	quality: TorrentQuality = 'all'
): Promise<EpisodeTorrentEntry[]> {
	let url: string;

	const qParam = quality !== 'all' ? `&q=${quality}` : '';

	if (!anidbEpisodeId) {
		url = `${TOSHO_BASE}/json?qx=1${qParam}&aids=${anidbId}`;
	} else {
		url = `${TOSHO_BASE}/json?qx=1${qParam}&aids=${anidbId}&eids=${anidbEpisodeId}`;
	}

	try {
		const raw = await invoke<string>('fetch_url', { url, headers: null });
		const data: ToshoRawEntry[] = JSON.parse(raw);
		return data.map((e) => normalizeToshoEntry(e, 'Anime Tosho'));
	} catch {
		return [];
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Nyaa RSS fallback
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fallback free-text Nyaa search for SubsPlease releases when Tosho has no results.
 */
export async function fetchNyaaEpisode(
	animeTitle: string,
	episodeNumber: number,
	quality: TorrentQuality = '1080p'
): Promise<EpisodeTorrentEntry[]> {
	const epStr = episodeNumber < 10 ? `0${episodeNumber}` : `${episodeNumber}`;
	const qualityStr = quality !== 'all' ? ` ${quality}` : '';
	const query = `[SubsPlease] ${animeTitle} - ${epStr}${qualityStr}`;
	const url = `${NYAA_RSS_BASE}/?page=rss&q=${encodeURIComponent(query)}&c=0_0&f=0`;

	try {
		const xml = await invoke<string>('fetch_url', { url, headers: null });
		return parseNyaaRss(xml);
	} catch {
		return [];
	}
}

/**
 * Free-text Tosho search (used by the manual search dialog and ToshoProvider).
 */
export async function searchTosho(query: string, page = 1): Promise<EpisodeTorrentEntry[]> {
	const url = `${TOSHO_BASE}/json?qx=1&q=${encodeURIComponent(query)}&page=${page}`;
	try {
		const raw = await invoke<string>('fetch_url', { url, headers: null });
		const data: ToshoRawEntry[] = JSON.parse(raw);
		return data.map((e) => normalizeToshoEntry(e, 'Anime Tosho'));
	} catch {
		return [];
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility: parse filename → episode number
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Attempt to extract an episode number from a torrent file name.
 *
 * Handles common naming conventions:
 *   [SubsPlease] Show - 04 (1080p).mkv        → 4
 *   [Erai-raws] Show - 12v2 [1080p].mkv       → 12
 *   Show S01E07.mkv                            → 7
 *   Show Episode 03.mkv                        → 3
 */
export function parseEpisodeNumber(filename: string): number | null {
	// " - 04 " or " - 04v2 "
	const dashEp = filename.match(/\s-\s(\d{1,4})(?:v\d)?\s/);
	if (dashEp) return parseInt(dashEp[1], 10);

	// S01E07 or S1E7
	const sXeX = filename.match(/[Ss]\d{1,2}[Ee](\d{1,4})/);
	if (sXeX) return parseInt(sXeX[1], 10);

	// "Episode 03"
	const epWord = filename.match(/[Ee]pisode\s+(\d{1,4})/i);
	if (epWord) return parseInt(epWord[1], 10);

	return null;
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal: normalisers and parsers
// ─────────────────────────────────────────────────────────────────────────────

function normalizeToshoEntry(
	e: ToshoRawEntry,
	provider: EpisodeTorrentEntry['provider']
): EpisodeTorrentEntry {
	return {
		title: e.title,
		magnetUri: e.magnet_uri ?? '',
		size: formatBytes(e.total_size ?? 0),
		sizeBytes: e.total_size ?? 0,
		seeds: e.num_seeders ?? 0,
		peers: e.num_leechers ?? 0,
		uploadedAt: e.timestamp ? new Date(e.timestamp * 1000).toLocaleDateString() : '',
		resolution: extractResolution(e.title),
		fansub: extractFansub(e.title),
		anidbId: e.anidb_aid,
		anidbEpisodeId: e.anidb_eid,
		provider,
	};
}

function parseNyaaRss(xml: string): EpisodeTorrentEntry[] {
	const parser = new DOMParser();
	const doc = parser.parseFromString(xml, 'text/xml');
	const items = doc.querySelectorAll('item');
	const results: EpisodeTorrentEntry[] = [];

	items.forEach((item) => {
		const title = item.querySelector('title')?.textContent ?? '';
		const link = item.querySelector('link')?.textContent ?? '';
		const pubDate = item.querySelector('pubDate')?.textContent ?? '';

		const seeds = parseInt(item.getElementsByTagName('nyaa:seeders')[0]?.textContent ?? '0', 10);
		const peers = parseInt(item.getElementsByTagName('nyaa:leechers')[0]?.textContent ?? '0', 10);
		const sizeStr = item.getElementsByTagName('nyaa:size')[0]?.textContent ?? '0 B';
		const infoHash = item.getElementsByTagName('nyaa:infoHash')[0]?.textContent;

		const magnet = infoHash
			? `magnet:?xt=urn:btih:${infoHash}&dn=${encodeURIComponent(title)}`
			: link;

		results.push({
			title,
			magnetUri: magnet,
			size: sizeStr,
			sizeBytes: 0, // not available in RSS
			seeds,
			peers,
			uploadedAt: pubDate ? new Date(pubDate).toLocaleDateString() : '',
			resolution: extractResolution(title),
			fansub: extractFansub(title),
			provider: 'Nyaa.si',
		});
	});

	return results;
}

function formatBytes(bytes: number): string {
	if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(2)} GiB`;
	if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(2)} MiB`;
	if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(1)} KiB`;
	return `${bytes} B`;
}

function extractResolution(title: string): string | undefined {
	return title.match(/\b(2160p|1080p|720p|480p|360p)\b/i)?.[1];
}

function extractFansub(title: string): string | undefined {
	// [SubsPlease] → "SubsPlease"
	return title.match(/^\[([^\]]+)\]/)?.[1];
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal Tosho raw API shape (from live API inspection + zenshin analysis)
// ─────────────────────────────────────────────────────────────────────────────

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
}
