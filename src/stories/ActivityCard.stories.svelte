<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ActivityCard from '$lib/components/ActivityCard.svelte';

	const mockUser = {
		id: 1,
		name: 'AniUser',
		avatar: { medium: 'https://s4.anilist.co/file/anilistcdn/user/avatar/medium/default.png' },
	};

	const mockMedia = {
		id: 16498,
		type: 'ANIME',
		title: { userPreferred: 'Shingeki no Kyojin' },
		coverImage: { medium: 'https://s4.anilist.co/file/anilistcdn/media/anime/cover/medium/bx16498-73IhOXpJZiMF.jpg' },
		episodes: 25,
	};

	const { Story } = defineMeta({
		title: 'Components/ActivityCard',
		component: ActivityCard,
		tags: ['autodocs'],
	});
</script>

<!-- List Activity: user updating watch progress -->
<Story
	name="ListActivity"
	args={{
		activity: {
			__typename: 'ListActivity',
			id: 1,
			createdAt: Math.floor(Date.now() / 1000) - 3600,
			status: 'watched episode',
			progress: '12',
			likeCount: 5,
			replyCount: 2,
			user: mockUser,
			media: mockMedia,
		},
	}}
/>

<!-- Text Activity: user status update -->
<Story
	name="TextActivity"
	args={{
		activity: {
			__typename: 'TextActivity',
			id: 2,
			createdAt: Math.floor(Date.now() / 1000) - 7200,
			text: '<p>Just finished Shingeki no Kyojin. The final arc was incredible!</p>',
			likeCount: 18,
			replyCount: 4,
			user: mockUser,
		},
	}}
/>

<!-- Message Activity: user sending a message -->
<Story
	name="MessageActivity"
	args={{
		activity: {
			__typename: 'MessageActivity',
			id: 3,
			createdAt: Math.floor(Date.now() / 1000) - 1800,
			message: '<p>Have you seen the latest episode yet?</p>',
			likeCount: 0,
			replyCount: 1,
			messenger: mockUser,
			recipient: { id: 2, name: 'OtherUser', avatar: { medium: null } },
		},
	}}
/>
