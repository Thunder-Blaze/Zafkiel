type TitleType = {
	romaji?: string;
	english?: string;
	native?: string;
	userPreferred?: string;
};

type TrailerType = {
	id?: string;
	site?: string;
	thumbnail?: string;
};

type CoverImageType = {
	extraLarge?: string;
	large?: string;
	medium?: string;
	color?: string;
};

export const filterTitle = (title: TitleType) => {
	return title.userPreferred || title.english || title.romaji || title.native || '';
};

/**
 * Strips Japanese brackets, season tags, and other metadata decorations for cleaner Nyaa searching.
 * e.g. "【OSHI NO KO】 Season 3" → "OSHI NO KO"
 * e.g. "My Hero Academia Season 7" → "My Hero Academia"
 */
export const cleanSearchTitle = (title: string) => {
	return title
		.replace(/【|】|［|］|\[|\]/g, ' ')           // Japanese/ASCII brackets
		.replace(/\s*[-:,]?\s*[Ss]eason\s*\d+/g, '')   // Strip "Season 3", "Season3"
		.replace(/\s*\bS\d+\b/g, '')                    // Strip bare "S3"/"S1"
		.replace(/\(.*?\)/g, ' ')                       // Remove parenthetical
		.replace(/\s+/g, ' ')
		.trim();
};

export const filterCoverImage = (coverImage: CoverImageType | undefined) => {
	if (!coverImage) return '';
	return coverImage.extraLarge || coverImage.large || coverImage.medium;
};

export const formatTrailerLink = (trailer: TrailerType | undefined) => {
	if (!trailer || !trailer.id || !trailer.site) return '';
	return `https://www.${trailer.site}.com/watch?v=${trailer.id}`;
};

export const formatTime = (time: number) => {
	const days = Math.floor(time / (24 * 60 * 60));
	const hours = Math.floor((time % (24 * 60 * 60)) / (60 * 60));
	const minutes = Math.floor((time % (60 * 60)) / 60);
	return `${days}d ${hours}h ${minutes}m`;
};

export const formatBytes = (bytes: number): string => {
	if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(2)} GiB`;
	if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(2)} MiB`;
	if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(1)} KiB`;
	return `${bytes} B`;
};
