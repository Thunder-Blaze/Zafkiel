import type { MediaType, MediaFormat, MediaStatus, MediaSeason, MediaSource } from "./anilist";

export type MediaListStatus =
	| 'CURRENT'
	| 'PLANNING'
	| 'COMPLETED'
	| 'DROPPED'
	| 'PAUSED'
	| 'REPEATING';

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
	studio?: string;
	year?: number;
	type?: MediaType;
	source?: MediaSource;
	season?: MediaSeason;
	isAdult?: boolean;
	description?: string;
	format?: MediaFormat;
	duration?: number;
	popularity?: number;
	favourites?: number;
}
