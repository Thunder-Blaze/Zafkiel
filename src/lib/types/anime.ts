import type { Media, Character, Staff, PageInfo, MediaCoverImage } from './anilist';

export interface AnimeLarge extends Media {
    characters?: {
        edges?: {
            node?: Character;
            role?: string;
            voiceActors?: (Staff & { languageV2?: string })[];
        }[];
        nodes?: Character[];
        pageInfo?: PageInfo;
    };
    staff?: {
        edges?: {
            node?: Staff;
            role?: string;
        }[];
        nodes?: Staff[];
        pageInfo?: PageInfo;
    };
    reviews?: {
        nodes?: {
            id: number;
            summary: string;
            body: string;
            rating: number;
            ratingAmount: number;
            user: {
                id: number;
                name: string;
                avatar: {
                    large: string;
                    medium: string;
                };
            };
        }[];
        pageInfo?: PageInfo;
    };
    relations?: {
        edges?: {
            node?: Media;
            relationType?: string;
        }[];
        nodes?: Media[];
        pageInfo?: PageInfo;
    };
    recommendations?: {
        edges?: {
            node?: {
                mediaRecommendation?: Media;
                rating?: number;
            };
        }[];
        nodes?: any[]; // Simplified for now
        pageInfo?: PageInfo;
    };
    nextAiringEpisode?: {
        id: number;
        episode: number;
        airingAt: number;
        timeUntilAiring: number;
    };
}

export type Anime = AnimeLarge;
