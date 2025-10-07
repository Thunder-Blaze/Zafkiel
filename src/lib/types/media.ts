export type MediaStatus = 'FINISHED' | 'RELEASING' | 'NOT_YET_RELEASED' | 'CANCELLED' | 'HIATUS';
export type MediaListStatus = 'CURRENT' | 'PLANNING' | 'COMPLETED' | 'DROPPED' | 'PAUSED' | 'REPEATING';
export type MediaSeason = 'WINTER' | 'SPRING' | 'SUMMER' | 'FALL';
export type MediaFormat = 'TV' | 'TV_SHORT' | 'MOVIE' | 'SPECIAL' | 'OVA' | 'ONA' | 'MUSIC' | 'MANGA' | 'NOVEL' | 'ONE_SHOT';

export interface MediaData {
	id: number;
	title?: string;
	englishTitle?: string;
	coverImage?: string;
	bannerImage?: string;
	score?: number;
	status?: MediaStatus;
	userStatus?: MediaListStatus;
	userProgress?: number;
	totalEpisodes?: number;
	genres?: string[];
	studios?: string[];
	year?: number;
	season?: MediaSeason;
	isAdult?: boolean;
	description?: string;
	format?: MediaFormat;
	duration?: number;
	popularity?: number;
	favourites?: number;
}
