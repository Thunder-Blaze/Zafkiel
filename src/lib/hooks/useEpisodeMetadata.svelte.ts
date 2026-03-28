/**
 * useEpisodeMetadata – TanStack Query hooks for episode metadata and torrents.
 *
 * - useAniZipEpisodes: fetches episode titles, thumbnails, AniDB IDs from AniZip
 * - useEpisodeTorrents: fetches precise per-episode torrent listings (Tosho → Nyaa fallback)
 * - useAnimeTorrents: all-episode torrent listing for the downloads tab
 */

import { createQuery } from '@tanstack/svelte-query';
import {
	fetchAniZipMappings,
	buildEpisodeMetas,
	fetchToshoEpisode,
	searchEpisodeCascaded,
	type EpisodeMeta,
	type EpisodeTorrentEntry,
	type TorrentQuality,
} from '$lib/services/EpisodeMetadataService';
import { TorrentService, type PersistTorrent } from '$lib/services/TorrentService';

// ─────────────────────────────────────────────────────────────────────────────
// Query key factories
// ─────────────────────────────────────────────────────────────────────────────

export const episodeKeys = {
	all: ['episode-metadata'] as const,

	/** AniZip episode list keyed by AniList ID */
	anizip: (anilistId: number | null) => [...episodeKeys.all, 'anizip', anilistId] as const,

	/** Tosho torrent entries keyed by AniDB IDs + quality */
	episodeTorrents: (
		anidbId: number | null,
		anidbEpisodeId: number | null,
		quality: TorrentQuality,
		animeTitle: string | null
	) => [...episodeKeys.all, 'torrents', anidbId, anidbEpisodeId, quality, animeTitle] as const,

	/** All-episode torrent listing keyed by AniDB series ID */
	animeTorrents: (anidbId: number | null) =>
		[...episodeKeys.all, 'anime-torrents', anidbId] as const,

	/** Saved torrents from local DB keyed by AniList ID + episode */
	savedTorrents: (animeId: number | null, episodeNumber: number | null) =>
		[...episodeKeys.all, 'saved', animeId, episodeNumber] as const,
};

// ─────────────────────────────────────────────────────────────────────────────
// Hook: episode titles, thumbnails, AniDB IDs
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fetches normalised episode metadata for an anime from the AniZip API.
 *
 * Returns a sorted array of `EpisodeMeta` objects – one per episode.
 * The data includes episode titles, thumbnail URLs, air dates, and AniDB IDs
 * needed for precise torrent lookup.
 *
 * @param anilistId  AniList media ID (pass null to disable the query)
 */
export function useAniZipEpisodes(anilistId: number | null) {
	return createQuery(() => ({
		queryKey: episodeKeys.anizip(anilistId),
		queryFn: async (): Promise<EpisodeMeta[]> => {
			if (!anilistId) return [];
			const mappings = await fetchAniZipMappings(anilistId);
			if (!mappings) return [];
			return buildEpisodeMetas(mappings);
		},
		enabled: !!anilistId,
		staleTime: 1000 * 60 * 60, // 1 hour – episode data rarely changes
		gcTime: 1000 * 60 * 60 * 24, // 24 hours cache
		retry: 2,
	}));
}

// ─────────────────────────────────────────────────────────────────────────────
// Hook: per-episode torrent search
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fetches torrent listings for a single episode.
 *
 * Primary path: Anime Tosho with precise AniDB IDs (fastest, most accurate).
 * Fallback path: Nyaa free-text search (SubsPlease sub-group).
 *
 * Results are sorted by seeders descending.
 *
 * @param episode   EpisodeMeta from useAniZipEpisodes (pass null to disable)
 * @param quality   Quality filter: "720p", "1080p", or "all" (default "1080p")
 */
export function useEpisodeTorrents(
	animeTitle: string | null,
	episode: EpisodeMeta | null,
	quality: TorrentQuality = '1080p'
) {
	return createQuery(() => ({
		queryKey: episodeKeys.episodeTorrents(
			episode?.anidbId ?? null,
			episode?.anidbEpisodeId ?? null,
			quality,
			animeTitle
		),
		queryFn: async (): Promise<EpisodeTorrentEntry[]> => {
			if (!episode) return [];

			// Use the new modular cascading search (Tosho -> Nyaa -> SubsPlease)
			const results = await searchEpisodeCascaded(
				{ title: { english: animeTitle ?? '', romaji: animeTitle ?? '' } } as any,
				episode.number,
				episode.anidbId,
				episode.anidbEpisodeId,
				quality,
				episode.absoluteNumber
			);

			return results.sort((a, b) => b.seeds - a.seeds);
		},
		enabled: !!episode,
		staleTime: 1000 * 60 * 10,
		gcTime: 1000 * 60 * 30,
		retry: 1,
	}));
}

// ─────────────────────────────────────────────────────────────────────────────
// Hook: all-episode torrent listing (for downloads tab)
// ─────────────────────────────────────────────────────────────────────────────

/**
 * Fetches ALL torrent entries for an anime series from Anime Tosho.
 *
 * This is the "bulk" view used in the downloads tab – it does not filter by
 * episode and returns everything indexed for this AniDB series ID.
 *
 * @param anidbId  AniDB series ID from AniZip mappings (pass null to disable)
 */
export function useAnimeTorrents(anidbId: number | null) {
	return createQuery(() => ({
		queryKey: episodeKeys.animeTorrents(anidbId),
		queryFn: async (): Promise<EpisodeTorrentEntry[]> => {
			if (!anidbId) return [];
			const results = await fetchToshoEpisode(anidbId, null, 'all');
			return [...results].sort((a, b) => b.seeds - a.seeds);
		},
		enabled: !!anidbId,
		staleTime: 1000 * 60 * 15,
		gcTime: 1000 * 60 * 60,
		retry: 1,
	}));
}
/**
 * Fetches already-downloaded torrents for a specific episode from the local database.
 * 
 * @param animeId      AniList media ID
 * @param episodeNumber Episode number
 */
export function useSavedEpisodeTorrents(animeId: number | null, episodeNumber: number | null) {
	return createQuery(() => ({
		queryKey: episodeKeys.savedTorrents(animeId, episodeNumber),
		queryFn: async (): Promise<PersistTorrent[]> => {
			if (animeId === null || episodeNumber === null) return [];
			return await TorrentService.getSavedTorrentsForEpisode(animeId, episodeNumber);
		},
		enabled: animeId !== null && episodeNumber !== null,
		staleTime: 1000 * 60 * 5, // 5 minutes cache for local DB state
	}));
}
