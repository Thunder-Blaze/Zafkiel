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
