<script lang="ts" module>
	import type { MediaList } from '$lib/types/anilist';

	const NOW = Math.floor(Date.now() / 1000);

	const currentEntry: MediaList = {
		id: 1,
		userId: 100,
		mediaId: 154587,
		status: 'CURRENT',
		score: 9.5,
		progress: 14,
		updatedAt: NOW - 3600,
		createdAt: NOW - 86400 * 10,
		media: {
			id: 154587,
			title: { userPreferred: 'Frieren: Beyond Journey\'s End' },
			format: 'TV',
			episodes: 28,
			type: 'ANIME',
			coverImage: {
				medium: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/bx154587-EVIONDIz5oXS.jpg',
			},
		},
	};

	const completedEntry: MediaList = {
		id: 2,
		userId: 100,
		mediaId: 21,
		status: 'COMPLETED',
		score: 10,
		progress: 720,
		updatedAt: NOW - 86400 * 30,
		createdAt: NOW - 86400 * 200,
		media: {
			id: 21,
			title: { userPreferred: 'One Piece' },
			format: 'TV',
			episodes: 1100,
			type: 'ANIME',
			coverImage: {
				medium: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/nx21-tXMN3Y20PIL9.jpg',
			},
		},
	};

	const planningEntry: MediaList = {
		id: 3,
		userId: 100,
		mediaId: 5114,
		status: 'PLANNING',
		score: 0,
		progress: 0,
		updatedAt: NOW - 86400 * 5,
		createdAt: NOW - 86400 * 5,
		media: {
			id: 5114,
			title: { userPreferred: 'Fullmetal Alchemist: Brotherhood' },
			format: 'TV',
			episodes: 64,
			type: 'ANIME',
			coverImage: {
				medium: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/n5114-ce9m7SzWDqhJ.jpg',
			},
		},
	};

	const droppedEntry: MediaList = {
		id: 4,
		userId: 100,
		mediaId: 100,
		status: 'DROPPED',
		score: 4,
		progress: 5,
		updatedAt: NOW - 86400 * 60,
		createdAt: NOW - 86400 * 70,
		media: {
			id: 100,
			title: { userPreferred: 'Some Dropped Anime' },
			format: 'TV',
			episodes: 24,
			type: 'ANIME',
			coverImage: {},
		},
	};

	const mangaEntry: MediaList = {
		id: 5,
		userId: 100,
		mediaId: 30002,
		status: 'CURRENT',
		score: 8.8,
		progress: 42,
		progressVolumes: 5,
		updatedAt: NOW - 7200,
		createdAt: NOW - 86400 * 20,
		media: {
			id: 30002,
			title: { userPreferred: 'Berserk' },
			format: 'MANGA',
			chapters: 370,
			type: 'MANGA',
			coverImage: {
				medium: 'https://s4.anilist.co/file/anilistcdn/media/manga/cover/medium/bx30002-OEVmxla7pqKT.jpg',
			},
		},
	};

	const noMediaEntry: MediaList = {
		id: 6,
		userId: 100,
		mediaId: 9999,
		status: 'PAUSED',
		score: 6,
		progress: 3,
		updatedAt: NOW - 86400,
		createdAt: NOW - 86400 * 3,
	};
</script>

<script lang="ts">
	import MediaListRow from '$lib/components/MediaListRow.svelte';
	import MediaListRowSkeleton from '$lib/components/MediaListRowSkeleton.svelte';
</script>

<!-- MediaListRow Stories -->

<h2 class="mb-4 text-xl font-bold">Status Variants</h2>
<div class="overflow-hidden rounded-lg border bg-card" style="max-width: 640px;">
	<MediaListRow entry={currentEntry} />
	<MediaListRow entry={completedEntry} />
	<MediaListRow entry={planningEntry} />
	<MediaListRow entry={droppedEntry} />
</div>

<h2 class="mb-4 mt-10 text-xl font-bold">Manga Entry</h2>
<div class="overflow-hidden rounded-lg border bg-card" style="max-width: 640px;">
	<MediaListRow entry={mangaEntry} />
</div>

<h2 class="mb-4 mt-10 text-xl font-bold">Entry Without Media</h2>
<div class="overflow-hidden rounded-lg border bg-card" style="max-width: 640px;">
	<MediaListRow entry={noMediaEntry} />
</div>

<h2 class="mb-4 mt-10 text-xl font-bold">Skeleton (10 rows)</h2>
<div class="overflow-hidden rounded-lg border bg-card" style="max-width: 640px;">
	<MediaListRowSkeleton count={5} />
</div>
