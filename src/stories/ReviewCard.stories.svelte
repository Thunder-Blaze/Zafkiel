<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ReviewCard from '$lib/components/ReviewCard.svelte';

	const { Story } = defineMeta({
		title: 'Components/ReviewCard',
		component: ReviewCard,
		tags: ['autodocs'],
	});

	const mockReview = {
		id: 1,
		score: 86,
		summary: 'An unforgettable masterpiece of modern anime',
		body: '<p>Shingeki no Kyojin delivers a breathtaking conclusion that subverts expectations at every turn. The animation quality, soundtrack, and character development reach new heights.</p>',
		rating: 124,
		ratingAmount: 203,
		createdAt: Math.floor(Date.now() / 1000) - 86400 * 3,
		user: {
			id: 1,
			name: 'CriticUser',
			avatar: { medium: 'https://s4.anilist.co/file/anilistcdn/user/avatar/medium/default.png' },
		},
		media: {
			id: 16498,
			type: 'ANIME',
			title: { userPreferred: 'Shingeki no Kyojin' },
			coverImage: {
				medium:
					'https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/bx16498-73IhOXpJZiMF.jpg',
			},
		},
	};
</script>

<!-- High score review with full content -->
<Story name="HighScore" args={{ review: mockReview }} />

<!-- Mid-range review -->
<Story
	name="MidScore"
	args={{
		review: {
			...mockReview,
			id: 2,
			score: 62,
			summary: 'Solid but flawed execution',
			body: '<p>Some great moments but pacing issues hold it back from greatness.</p>',
			rating: 31,
			ratingAmount: 87,
		},
	}}
/>

<!-- Low score review -->
<Story
	name="LowScore"
	args={{
		review: {
			...mockReview,
			id: 3,
			score: 38,
			summary: 'Disappointing final arc',
			body: '<p>The writing quality drops significantly in the final season.</p>',
			rating: 12,
			ratingAmount: 54,
		},
	}}
/>

<!-- Review without score -->
<Story
	name="NoScore"
	args={{
		review: {
			...mockReview,
			id: 4,
			score: undefined,
			summary: 'A journey worth taking',
		},
	}}
/>

<!-- Private review -->
<Story
	name="Private"
	args={{ review: { ...mockReview, id: 5, private: true, summary: 'My private thoughts' } }}
/>
