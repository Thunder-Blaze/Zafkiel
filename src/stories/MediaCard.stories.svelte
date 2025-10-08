<script lang="ts">
	import MediaCard from '$lib/components/MediaCard.svelte';
	import type { MediaData } from '$lib/types/media';

	// Mock data for anime
	const mockAnimeData: MediaData = {
		id: 1,
		title: 'Steins;Gate',
		englishTitle: 'Steins;Gate',
		coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx9253-7pdcVzQSkKxT.jpg',
		bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/banner/9253-JIhmKgBKsWUN.jpg',
		score: 9.1,
		status: 'FINISHED',
		type: 'ANIME',
		format: 'TV',
		totalEpisodes: 24,
		duration: 24,
		genres: ['Sci-Fi', 'Thriller', 'Drama', 'Psychological'],
		year: 2011,
		season: 'SPRING',
		popularity: 312459,
		favourites: 28976,
		description: 'A group of friends have customized their microwave so that it can send text messages to the past. As they perform different experiments, an organization named SERN who has been doing their own research on time travel tracks them down and now the characters have to find a way to avoid being captured by them.',
		isAdult: false
	};

	// Mock data for anime with user progress
	const mockAnimeWithProgress: MediaData = {
		...mockAnimeData,
		userStatus: 'CURRENT',
		userProgress: 12
	};

	// Mock data for completed anime
	const mockCompletedAnime: MediaData = {
		...mockAnimeData,
		userStatus: 'COMPLETED',
		userProgress: 24
	};

	// Mock data for manga
	const mockMangaData: MediaData = {
		id: 2,
		title: 'Berserk',
		englishTitle: 'Berserk',
		coverImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/large/bx1-4r0JG7GQqWY9.jpg',
		bannerImage: 'https://s4.anilist.co/file/anilistcdn/media/manga/banner/1-OVV64aa26zaB.jpg',
		score: 9.5,
		status: 'FINISHED',
		type: 'MANGA',
		format: 'MANGA',
		totalEpisodes: 364,
		genres: ['Action', 'Adventure', 'Drama', 'Fantasy', 'Horror'],
		year: 1989,
		popularity: 178542,
		favourites: 19876,
		description: 'Guts, a former mercenary now known as the "Black Swordsman," is out for revenge. After a tumultuous childhood, he finally finds someone he respects and believes he can trust, only to have everything fall apart when this person takes away everything important to Guts for the purpose of fulfilling his own desires.',
		isAdult: false
	};

	// Mock data for adult anime
	const mockAdultAnime: MediaData = {
		id: 3,
		title: 'High School DxD',
		coverImage: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11617-6w7KJLHS2mIA.jpg',
		score: 7.5,
		status: 'FINISHED',
		type: 'ANIME',
		format: 'TV',
		totalEpisodes: 12,
		duration: 24,
		genres: ['Action', 'Comedy', 'Fantasy', 'Romance', 'Ecchi'],
		year: 2012,
		season: 'WINTER',
		popularity: 245678,
		favourites: 8765,
		description: 'High school student Issei Hyoudou is your run-of-the-mill pervert who does nothing productive with his life. However, his peaceful days are suddenly interrupted when he is called for a date by a girl named Yuuma Amano.',
		isAdult: true,
		userStatus: 'PLANNING'
	};

	// Mock data without image
	const mockNoImageData: MediaData = {
		id: 4,
		title: 'Unknown Anime Series',
		score: 6.8,
		type: 'ANIME',
		format: 'TV',
		totalEpisodes: 13,
		genres: ['Action', 'Adventure'],
		year: 2024,
		season: 'FALL',
		description: 'A mysterious anime with no available images or banner.',
		isAdult: false
	};

	function handleProgressUpdate(progress: number): void {
		console.log('Progress updated:', progress);
	}

	function handleStatusChange(status: string): void {
		console.log('Status changed:', status);
	}
</script>

<!-- Story: Anime - Default -->
<div class="story-section">
	<h2>Anime - Default</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockAnimeData}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: Anime - With User Progress -->
<div class="story-section">
	<h2>Anime - With User Progress</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockAnimeWithProgress}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: Anime - Completed -->
<div class="story-section">
	<h2>Anime - Completed</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockCompletedAnime}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: Manga - Default -->
<div class="story-section">
	<h2>Manga - Default</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockMangaData}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: Adult Content (18+) -->
<div class="story-section">
	<h2>Adult Content (18+)</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockAdultAnime}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: No Images Available -->
<div class="story-section">
	<h2>No Images Available</h2>
	<div class="p-8 max-w-sm">
		<MediaCard
			mediaData={mockNoImageData}
			onProgressUpdate={handleProgressUpdate}
			onStatusChange={handleStatusChange}
		/>
	</div>
</div>

<!-- Story: Grid Layout -->
<div class="story-section">
	<h2>Grid Layout</h2>
	<div class="p-8">
		<div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
			<MediaCard
				mediaData={mockAnimeData}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
			<MediaCard
				mediaData={mockAnimeWithProgress}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
			<MediaCard
				mediaData={mockCompletedAnime}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
			<MediaCard
				mediaData={mockMangaData}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
			<MediaCard
				mediaData={mockAdultAnime}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
			<MediaCard
				mediaData={mockNoImageData}
				onProgressUpdate={handleProgressUpdate}
				onStatusChange={handleStatusChange}
			/>
		</div>
	</div>
</div>

<style>
	.story-section {
		margin-bottom: 3rem;
		padding: 2rem;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
	}

	.story-section h2 {
		margin-bottom: 1rem;
		font-size: 1.5rem;
		font-weight: 600;
	}
</style>
