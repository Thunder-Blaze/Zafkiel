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
	nextAiringEpisode?: {
		id?: number;
		airingAt: number;
		timeUntilAiring: number;
		episode: number;
		mediaId?: number;
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
	countryOfOrigin?: string;
}

// ============================================================================
// Media List Types (full list management)
// ============================================================================

export interface MediaList {
	id: number;
	userId: number;
	mediaId: number;
	status?: MediaListStatus;
	score?: number;
	progress?: number;
	progressVolumes?: number;
	repeat?: number;
	priority?: number;
	private?: boolean;
	notes?: string;
	hiddenFromStatusLists?: boolean;
	customLists?: Record<string, boolean>;
	startedAt?: MediaDate;
	completedAt?: MediaDate;
	updatedAt?: number;
	createdAt?: number;
	media?: Media;
}

export interface MediaListGroup {
	name?: string;
	isCustomList?: boolean;
	isCompletedList?: boolean;
	entries?: MediaList[];
}

export interface MediaListCollection {
	lists?: MediaListGroup[];
	user?: User;
	hasNextChunk?: boolean;
}

// ============================================================================
// Activity Types
// ============================================================================

export interface ActivityReply {
	id: number;
	userId?: number;
	activityId?: number;
	text?: string;
	likeCount?: number;
	isLiked?: boolean;
	createdAt: number;
	user?: User;
	likes?: User[];
}

export interface ListActivity {
	__typename: 'ListActivity';
	id: number;
	userId?: number;
	replyCount?: number;
	status?: string;
	progress?: string;
	isLocked?: boolean;
	isSubscribed?: boolean;
	likeCount?: number;
	isLiked?: boolean;
	isPinned?: boolean;
	siteUrl?: string;
	createdAt: number;
	user?: User;
	media?: Media;
	replies?: ActivityReply[];
	likes?: User[];
}

export interface TextActivity {
	__typename: 'TextActivity';
	id: number;
	userId?: number;
	replyCount?: number;
	text?: string;
	siteUrl?: string;
	isLocked?: boolean;
	isSubscribed?: boolean;
	likeCount?: number;
	isLiked?: boolean;
	isPinned?: boolean;
	createdAt: number;
	user?: User;
	replies?: ActivityReply[];
	likes?: User[];
}

export interface MessageActivity {
	__typename: 'MessageActivity';
	id: number;
	recipientId?: number;
	messengerId?: number;
	replyCount?: number;
	message?: string;
	isLocked?: boolean;
	isSubscribed?: boolean;
	likeCount?: number;
	isLiked?: boolean;
	isPrivate?: boolean;
	siteUrl?: string;
	createdAt: number;
	recipient?: User;
	messenger?: User;
	replies?: ActivityReply[];
	likes?: User[];
}

export type ActivityUnion = ListActivity | TextActivity | MessageActivity;

// ============================================================================
// Notification Types
// ============================================================================

export type NotificationType =
	| 'ACTIVITY_MESSAGE'
	| 'ACTIVITY_REPLY'
	| 'FOLLOWING'
	| 'ACTIVITY_MENTION'
	| 'THREAD_COMMENT_MENTION'
	| 'THREAD_SUBSCRIBED'
	| 'THREAD_COMMENT_REPLY'
	| 'AIRING'
	| 'ACTIVITY_LIKE'
	| 'ACTIVITY_REPLY_LIKE'
	| 'THREAD_LIKE'
	| 'THREAD_COMMENT_LIKE'
	| 'ACTIVITY_REPLY_SUBSCRIBED'
	| 'RELATED_MEDIA_ADDITION'
	| 'MEDIA_DATA_CHANGE'
	| 'MEDIA_MERGE'
	| 'MEDIA_DELETION';

// Rust NotificationUnion uses #[serde(tag = "type")] + SCREAMING_SNAKE_CASE
// so JSON is: { "type": "AIRING", "id": 1, "animeId": 123, ... }

export interface AiringNotification {
	type: 'AIRING';
	id: number;
	animeId?: number;
	episode?: number;
	contexts?: string[];
	createdAt?: number;
	media?: Media;
}

export interface FollowingNotification {
	type: 'FOLLOWING';
	id: number;
	userId?: number;
	context?: string;
	createdAt?: number;
	user?: User;
}

export interface ActivityNotification {
	type:
		| 'ACTIVITY_MESSAGE'
		| 'ACTIVITY_REPLY'
		| 'ACTIVITY_MENTION'
		| 'ACTIVITY_LIKE'
		| 'ACTIVITY_REPLY_LIKE'
		| 'ACTIVITY_REPLY_SUBSCRIBED';
	id: number;
	userId?: number;
	activityId?: number;
	context?: string;
	createdAt?: number;
	user?: User;
}

export interface ThreadNotification {
	type:
		| 'THREAD_COMMENT_MENTION'
		| 'THREAD_SUBSCRIBED'
		| 'THREAD_COMMENT_REPLY'
		| 'THREAD_LIKE'
		| 'THREAD_COMMENT_LIKE';
	id: number;
	userId?: number;
	commentId?: number;
	threadId?: number;
	context?: string;
	createdAt?: number;
	thread?: Thread;
	comment?: ThreadComment;
	user?: User;
}

export interface MediaChangeNotification {
	type: 'RELATED_MEDIA_ADDITION' | 'MEDIA_DATA_CHANGE' | 'MEDIA_MERGE' | 'MEDIA_DELETION';
	id: number;
	mediaId?: number;
	context?: string;
	reason?: string;
	deletedMediaTitle?: string;
	deletedMediaTitles?: string[];
	createdAt?: number;
	media?: Media;
}

export type NotificationUnion =
	| AiringNotification
	| FollowingNotification
	| ActivityNotification
	| ThreadNotification
	| MediaChangeNotification;

// ============================================================================
// Forum / Thread Types
// ============================================================================

export interface ThreadCategory {
	id: number;
	name: string;
}

export interface Thread {
	id: number;
	title?: string;
	body?: string;
	userId?: number;
	replyUserId?: number;
	replyCommentId?: number;
	replyCount?: number;
	viewCount?: number;
	isLocked?: boolean;
	isSticky?: boolean;
	isSubscribed?: boolean;
	likeCount?: number;
	isLiked?: boolean;
	repliedAt?: number;
	createdAt?: number;
	updatedAt?: number;
	user?: User;
	replyUser?: User;
	likes?: User[];
	siteUrl?: string;
	categories?: ThreadCategory[];
	mediaCategories?: Media[];
}

export interface ThreadComment {
	id: number;
	userId?: number;
	threadId?: number;
	comment?: string;
	likeCount?: number;
	isLiked?: boolean;
	siteUrl?: string;
	createdAt?: number;
	updatedAt?: number;
	thread?: Thread;
	user?: User;
	likes?: User[];
	childComments?: ThreadComment[];
	isLocked?: boolean;
}

// ============================================================================
// Review Types
// ============================================================================

export type ReviewRating = 'NO_VOTE' | 'UP_VOTE' | 'DOWN_VOTE';

export interface Review {
	id: number;
	userId?: number;
	mediaId?: number;
	mediaType?: MediaType;
	summary?: string;
	body?: string;
	rating?: number;
	ratingAmount?: number;
	userRating?: ReviewRating;
	score?: number;
	private?: boolean;
	siteUrl?: string;
	createdAt?: number;
	updatedAt?: number;
	user?: User;
	media?: Media;
}

// ============================================================================
// Recommendation Types
// ============================================================================

export type RecommendationRating = 'NO_RATING' | 'RATE_UP' | 'RATE_DOWN';

export interface Recommendation {
	id: number;
	rating?: number;
	userRating?: RecommendationRating;
	media?: Media;
	mediaRecommendation?: Media;
	user?: User;
}

// ============================================================================
// Airing Schedule Types
// ============================================================================

export interface AiringSchedule {
	id: number;
	airingAt: number;
	timeUntilAiring: number;
	episode: number;
	mediaId: number;
	media?: Media;
}

// ============================================================================
// Combined Search Types  (mirror of the lean Rust structs in anilist.rs)
// ============================================================================

export interface SearchTitle {
	romaji: string | null;
	english: string | null;
	userPreferred: string | null;
}

export interface SearchCoverImage {
	medium: string | null;
}

/** Lean anime/manga result — only the fields returned by search_all */
export interface SearchMediaResult {
	id: number | null;
	/** "ANIME" | "MANGA" */
	type: string | null;
	/** "TV" | "OVA" | "MOVIE" | … */
	format: string | null;
	/** "FINISHED" | "RELEASING" | … */
	status: string | null;
	meanScore: number | null;
	title: SearchTitle | null;
	coverImage: SearchCoverImage | null;
}

export interface SearchPersonName {
	full: string | null;
	userPreferred: string | null;
}

export interface SearchPersonImage {
	medium: string | null;
}

export interface SearchCharacterResult {
	id: number | null;
	name: SearchPersonName | null;
	image: SearchPersonImage | null;
}

export interface SearchStaffResult {
	id: number | null;
	name: SearchPersonName | null;
	image: SearchPersonImage | null;
}

export interface SearchStudioResult {
	id: number | null;
	name: string | null;
	isAnimationStudio: boolean | null;
}

export interface SearchUserAvatar {
	medium: string | null;
}

export interface SearchUserResult {
	id: number | null;
	name: string | null;
	avatar: SearchUserAvatar | null;
}

export interface SearchAllResults {
	anime: { pageInfo?: PageInfo; data: SearchMediaResult[] };
	manga: { pageInfo?: PageInfo; data: SearchMediaResult[] };
	characters: { pageInfo?: PageInfo; data: SearchCharacterResult[] };
	staff: { pageInfo?: PageInfo; data: SearchStaffResult[] };
	studios: { pageInfo?: PageInfo; data: SearchStudioResult[] };
	users: { pageInfo?: PageInfo; data: SearchUserResult[] };
}
