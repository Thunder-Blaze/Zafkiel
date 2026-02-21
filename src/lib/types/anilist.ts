/**
 * TypeScript types for AniList API
 * These mirror the Rust types from anilist_moe crate
 */

// ============================================================================
// Core Types
// ============================================================================

export interface AniListResponse<T> {
	success: boolean;
	data?: T;
	error?: string;
}

export interface PageInfo {
	total: number;
	perPage: number;
	currentPage: number;
	lastPage: number;
	hasNextPage: boolean;
}

export interface Page<T> {
	pageInfo: PageInfo;
	data: T;
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
	romaji?: string;
	english?: string;
	native?: string;
	userPreferred?: string;
}

export interface MediaCoverImage {
	extraLarge?: string;
	large?: string;
	medium?: string;
	color?: string;
}

export interface MediaTrailer {
	id?: string;
	site?: string;
	thumbnail?: string;
}

export interface MediaDate {
	year?: number;
	month?: number;
	day?: number;
}

export interface MediaTag {
	id: number;
	name: string;
	description?: string;
	category?: string;
	rank?: number;
	isGeneralSpoiler?: boolean;
	isMediaSpoiler?: boolean;
	isAdult?: boolean;
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
	type?: string;
	language?: string;
	color?: string;
	icon?: string;
}

export type MediaListStatus = 'CURRENT' | 'PLANNING' | 'COMPLETED' | 'DROPPED' | 'PAUSED' | 'REPEATING';

export interface MediaListEntry {
	id: number;
	status?: MediaListStatus;
	score?: number;
	progress?: number;
	progressVolumes?: number;
	repeat?: number;
	priority?: number;
	notes?: string;
	hiddenFromStatusLists?: boolean;
	customLists?: string[];
	startedAt?: MediaDate;
	finishedAt?: MediaDate;
	updatedAt?: number;
	createdAt?: number;
}

export interface MediaRanking {
	id: number;
	rank: number;
	type?: string;
	format?: MediaFormat;
	year?: number;
	season?: MediaSeason;
	allTime?: boolean;
	context?: string;
}

export interface MediaStats {
	scoreDistribution?: Array<{
		score: number;
		amount: number;
	}>;
	statusDistribution?: Array<{
		status: string;
		amount: number;
	}>;
}

export interface Media {
	id: number;
	idMal?: number;
	title?: MediaTitle;
	type?: MediaType;
	format?: MediaFormat;
	status?: MediaStatus;
	description?: string;
	startDate?: MediaDate;
	endDate?: MediaDate;
	season?: MediaSeason;
	seasonYear?: number;
	seasonInt?: number;
	episodes?: number;
	duration?: number;
	chapters?: number;
	volumes?: number;
	countryOfOrigin?: string;
	isLicensed?: boolean;
	source?: MediaSource;
	hashtag?: string;
	trailer?: MediaTrailer;
	updatedAt?: number;
	coverImage?: MediaCoverImage;
	bannerImage?: string;
	genres?: string[];
	synonyms?: string[];
	averageScore?: number;
	meanScore?: number;
	popularity?: number;
	isLocked?: boolean;
	trending?: number;
	favourites?: number;
	tags?: MediaTag[];
	isFavourite?: boolean;
	isAdult?: boolean;
	siteUrl?: string;
	mediaListEntry?: MediaListEntry;
	autoCreateForumThread?: boolean;
	isRecommendationBlocked?: boolean;
	modNotes?: string;
	studios?: {
		edges?: { isMain?: boolean; node?: MediaStudio }[];
		nodes?: MediaStudio[];
	} | null;
}

// ============================================================================
// User Types
// ============================================================================

export interface UserAvatar {
	large?: string;
	medium?: string;
}

export interface UserStatistics {
	anime?: {
		count?: number;
		meanScore?: number;
		standardDeviation?: number;
		minutesWatched?: number;
		episodesWatched?: number;
	};
	manga?: {
		count?: number;
		meanScore?: number;
		standardDeviation?: number;
		chaptersRead?: number;
		volumesRead?: number;
	};
}

export interface User {
	id: number;
	name: string;
	about?: string;
	avatar?: UserAvatar;
	bannerImage?: string;
	isFollowing?: boolean;
	isFollower?: boolean;
	isBlocked?: boolean;
	bans?: string;
	options?: Record<string, unknown>;
	mediaListOptions?: Record<string, unknown>;
	statistics?: UserStatistics;
	unreadNotificationCount?: number;
	siteUrl?: string;
	donatorTier?: number;
	donatorBadge?: string;
	moderatorRoles?: string[];
	createdAt?: number;
	updatedAt?: number;
}

// ============================================================================
// Studio Types
// ============================================================================

export interface Studio {
	id: number;
	name: string;
	isAnimationStudio: boolean;
	media?: {
		nodes?: Media[];
		edges?: { node?: Media; isMain?: boolean }[];
		pageInfo?: PageInfo;
	};
	siteUrl?: string;
	isFavourite?: boolean;
	favourites?: number;
}

// ============================================================================
// Character Types
// ============================================================================

export interface Character {
	id: number;
	name: {
		first?: string;
		last?: string;
		full?: string;
		native?: string;
		userPreferred?: string;
		alternative?: string[];
		alternativeSpoiler?: string[];
	};
	image?: {
		large?: string;
		medium?: string;
	};
	description?: string;
	gender?: string;
	dateOfBirth?: MediaDate;
	age?: string;
	bloodType?: string;
	isFavourite?: boolean;
	isFavouriteBlocked?: boolean;
	favourites?: number;
	siteUrl?: string;
	media?: {
		nodes?: Media[];
		edges?: { node?: Media; id?: number; characterRole?: string }[];
		pageInfo?: PageInfo;
	};
}

// ============================================================================
// Staff Types
// ============================================================================

export interface Staff {
	id: number;
	name: {
		first?: string;
		last?: string;
		full?: string;
		native?: string;
		userPreferred?: string;
		alternative?: string[];
	};
	image?: {
		large?: string;
		medium?: string;
	};
	description?: string;
	gender?: string;
	dateOfBirth?: MediaDate;
	dateOfDeath?: MediaDate;
	age?: number;
	yearsActive?: number[];
	homeTown?: string;
	bloodType?: string;
	isFavourite?: boolean;
	isFavouriteBlocked?: boolean;
	favourites?: number;
	siteUrl?: string;
	staffMedia?: {
		nodes?: Media[];
		edges?: { node?: Media; id?: number; staffRole?: string }[];
		pageInfo?: PageInfo;
	};
	characters?: {
		nodes?: Character[];
		edges?: { node?: Character; id?: number; role?: string }[];
		pageInfo?: PageInfo;
	};
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

export type MediaSort =
	| 'POPULARITY_DESC'
	| 'POPULARITY'
	| 'TRENDING_DESC'
	| 'TRENDING'
	| 'SCORE_DESC'
	| 'SCORE'
	| 'TITLE_ROMAJI'
	| 'TITLE_ROMAJI_DESC'
	| 'TITLE_ENGLISH'
	| 'TITLE_ENGLISH_DESC'
	| 'TITLE_NATIVE'
	| 'TITLE_NATIVE_DESC'
	| 'START_DATE'
	| 'START_DATE_DESC'
	| 'END_DATE'
	| 'END_DATE_DESC'
	| 'FAVOURITES_DESC'
	| 'FAVOURITES'
	| 'ID'
	| 'ID_DESC';

export interface BrowseParams extends PaginationParams {
	mediaType?: MediaType;
	search?: string;
	season?: MediaSeason;
	seasonYear?: number;
	format?: MediaFormat;
	status?: MediaStatus;
	source?: MediaSource;
	genres?: string[];
	genresExcluded?: string[];
	sortBy?: MediaSort[];
	isAdult?: boolean;
}
