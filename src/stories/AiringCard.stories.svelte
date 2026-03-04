<script lang="ts" module>
	import type { Media } from '$lib/types/anilist';

	const NOW = Math.floor(Date.now() / 1000);

	const baseMedia: Media = {
		id: 1234,
		title: {
			userPreferred: "Frieren: Beyond Journey's End",
			english: "Frieren: Beyond Journey's End",
		},
		format: 'TV',
		averageScore: 90,
		episodes: 28,
		coverImage: {
			large:
				'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx154587-EVIONDIz5oXS.jpg',
		},
		nextAiringEpisode: {
			id: 1,
			airingAt: NOW + 7 * 3600,
			timeUntilAiring: 7 * 3600,
			episode: 15,
		},
	};

	const soonMedia: Media = {
		id: 5678,
		title: { userPreferred: 'Solo Leveling Season 2' },
		format: 'TV',
		averageScore: 83,
		coverImage: {
			large:
				'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx170942-Xj58JWLLL9d7.jpg',
		},
		nextAiringEpisode: {
			id: 2,
			airingAt: NOW + 45 * 60,
			timeUntilAiring: 45 * 60,
			episode: 3,
		},
	};

	const noCountdownMedia: Media = {
		id: 9012,
		title: { userPreferred: 'Attack on Titan: The Final Chapters' },
		format: 'SPECIAL',
		averageScore: 87,
		episodes: 2,
		coverImage: {
			large:
				'https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx131681-pVfFVzUnJH7I.jpg',
		},
	};

	const noScoreMedia: Media = {
		id: 3456,
		title: { userPreferred: 'New Upcoming Anime Title' },
		format: 'TV',
		averageScore: 0,
		coverImage: {},
		nextAiringEpisode: {
			id: 3,
			airingAt: NOW + 3 * 86400,
			timeUntilAiring: 3 * 86400,
			episode: 1,
		},
	};
</script>

<script lang="ts">
	import AiringCard from '$lib/components/AiringCard.svelte';
</script>

<!-- AiringCard Stories -->

<h2 class="mb-4 text-xl font-bold">Airing Soon (7h countdown)</h2>
<div class="w-40">
	<AiringCard media={baseMedia} />
</div>

<h2 class="mt-10 mb-4 text-xl font-bold">Airing Very Soon (45m countdown)</h2>
<div class="w-40">
	<AiringCard media={soonMedia} />
</div>

<h2 class="mt-10 mb-4 text-xl font-bold">No Countdown (Finished)</h2>
<div class="w-40">
	<AiringCard media={noCountdownMedia} />
</div>

<h2 class="mt-10 mb-4 text-xl font-bold">No Score (Upcoming)</h2>
<div class="w-40">
	<AiringCard media={noScoreMedia} />
</div>

<h2 class="mt-10 mb-4 text-xl font-bold">Grid View (Multiple cards)</h2>
<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4" style="max-width: 640px;">
	<AiringCard media={baseMedia} />
	<AiringCard media={soonMedia} />
	<AiringCard media={noCountdownMedia} />
	<AiringCard media={noScoreMedia} />
</div>
