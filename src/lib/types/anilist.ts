/**
 * TypeScript types for AniList API
 * These mirror the Rust types from anilist_moe crate
 */

// ============================================================================
// Core Types
// ============================================================================

export interface AniListResponse<T> {
	success: boolean;
	data: T | null;
	error: string | null;
}

// ============================================================================
// Media Types (Anime & Manga)
// ============================================================================

export type MediaType = 'ANIME' | 'MANGA';

export type MediaFormat =
	| 'TV'
	| 'TV_SHORT'
	| 'MOVIE'
	| 'SPECIAL'
	| 'OVA'
	| 'ONA'
	| 'MUSIC'
	| 'MANGA'
	| 'NOVEL'
	| 'ONE_SHOT';

export type MediaStatus = 'FINISHED' | 'RELEASING' | 'NOT_YET_RELEASED' | 'CANCELLED' | 'HIATUS';

export type MediaSeason = 'WINTER' | 'SPRING' | 'SUMMER' | 'FALL';

export type MediaSource =
	| 'ORIGINAL'
	| 'MANGA'
	| 'LIGHT_NOVEL'
	| 'VISUAL_NOVEL'
	| 'VIDEO_GAME'
	| 'OTHER'
	| 'NOVEL'
	| 'DOUJINSHI'
	| 'ANIME'
	| 'WEB_NOVEL'
	| 'LIVE_ACTION'
	| 'GAME'
	| 'COMIC'
	| 'MULTIMEDIA_PROJECT'
	| 'PICTURE_BOOK';

export interface MediaTitle {
	romaji: string | null;
	english: string | null;
	native: string | null;
	userPreferred: string | null;
}

export interface MediaCoverImage {
	extraLarge: string | null;
	large: string | null;
	medium: string | null;
	color: string | null;
}

export interface MediaTrailer {
	id: string | null;
	site: string | null;
	thumbnail: string | null;
}

export interface MediaDate {
	year: number | null;
	month: number | null;
	day: number | null;
}

export interface MediaTag {
	id: number;
	name: string;
	description: string | null;
	category: string | null;
	rank: number | null;
	isGeneralSpoiler: boolean | null;
	isMediaSpoiler: boolean | null;
	isAdult: boolean | null;
}

export interface MediaGenre {
	name: string;
}

export interface MediaStudio {
	id: number;
	name: string;
	isAnimationStudio: boolean;
}

export interface MediaExternalLink {
	id: number;
	url: string;
	site: string;
	type: string | null;
	language: string | null;
	color: string | null;
	icon: string | null;
}

export interface MediaRanking {
	id: number;
	rank: number;
	type: string;
	format: MediaFormat | null;
	year: number | null;
	season: MediaSeason | null;
	allTime: boolean | null;
	context: string;
}

export interface MediaStats {
	scoreDistribution: Array<{
		score: number;
		amount: number;
	}> | null;
	statusDistribution: Array<{
		status: string;
		amount: number;
	}> | null;
}

export interface Media {
	id: number;
	idMal: number | null;
	title: MediaTitle | null;
	type: MediaType | null;
	format: MediaFormat | null;
	status: MediaStatus | null;
	description: string | null;
	startDate: MediaDate | null;
	endDate: MediaDate | null;
	season: MediaSeason | null;
	seasonYear: number | null;
	seasonInt: number | null;
	episodes: number | null;
	duration: number | null;
	chapters: number | null;
	volumes: number | null;
	countryOfOrigin: string | null;
	isLicensed: boolean | null;
	source: MediaSource | null;
	hashtag: string | null;
	trailer: MediaTrailer | null;
	updatedAt: number | null;
	coverImage: MediaCoverImage | null;
	bannerImage: string | null;
	genres: string[] | null;
	synonyms: string[] | null;
	averageScore: number | null;
	meanScore: number | null;
	popularity: number | null;
	isLocked: boolean | null;
	trending: number | null;
	favourites: number | null;
	tags: MediaTag[] | null;
	isFavourite: boolean;
	isAdult: boolean | null;
	siteUrl: string | null;
	autoCreateForumThread: boolean | null;
	isRecommendationBlocked: boolean | null;
	modNotes: string | null;
}

// ============================================================================
// User Types
// ============================================================================

export interface UserAvatar {
	large: string | null;
	medium: string | null;
}

export interface UserStatistics {
	anime: {
		count: number;
		meanScore: number;
		standardDeviation: number;
		minutesWatched: number;
		episodesWatched: number;
	} | null;
	manga: {
		count: number;
		meanScore: number;
		standardDeviation: number;
		chaptersRead: number;
		volumesRead: number;
	} | null;
}

export interface User {
	id: number;
	name: string;
	about: string | null;
	avatar: UserAvatar | null;
	bannerImage: string | null;
	isFollowing: boolean | null;
	isFollower: boolean | null;
	isBlocked: boolean | null;
	bans: string | null;
	options: Record<string, unknown> | null;
	mediaListOptions: Record<string, unknown> | null;
	statistics: UserStatistics | null;
	unreadNotificationCount: number | null;
	siteUrl: string | null;
	donatorTier: number | null;
	donatorBadge: string | null;
	moderatorRoles: string[] | null;
	createdAt: number | null;
	updatedAt: number | null;
}

// ============================================================================
// Request Parameters
// ============================================================================

export interface PaginationParams {
	page?: number;
	perPage?: number;
}

export interface SeasonalAnimeParams extends PaginationParams {
	season: MediaSeason;
	year: number;
}

export interface SearchParams extends PaginationParams {
	query: string;
}
