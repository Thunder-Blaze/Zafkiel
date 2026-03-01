<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ThreadCard from '$lib/components/ThreadCard.svelte';

	const { Story } = defineMeta({
		title: 'Components/ThreadCard',
		component: ThreadCard,
		tags: ['autodocs'],
	});

	const baseThread = {
		id: 1,
		title: 'What are your thoughts on the final season?',
		replyCount: 47,
		viewCount: 1284,
		likeCount: 63,
		createdAt: Math.floor(Date.now() / 1000) - 86400,
		repliedAt: Math.floor(Date.now() / 1000) - 1800,
		isSticky: false,
		isLocked: false,
		user: {
			id: 1,
			name: 'ForumUser',
			avatar: { medium: 'https://s4.anilist.co/file/anilistcdn/user/avatar/medium/default.png' },
		},
		categories: [{ id: 1, name: 'Anime Discussion' }],
	};
</script>

<!-- Standard thread -->
<Story name="Default" args={{ thread: baseThread }} />

<!-- Pinned thread -->
<Story
	name="Pinned"
	args={{
		thread: {
			...baseThread,
			id: 2,
			title: '📌 Community Rules & Guidelines',
			isSticky: true,
			replyCount: 0,
			viewCount: 8920,
		},
	}}
/>

<!-- Locked thread -->
<Story
	name="Locked"
	args={{
		thread: {
			...baseThread,
			id: 3,
			title: 'Spoiler Discussion Thread (Locked)',
			isLocked: true,
		},
	}}
/>

<!-- Pinned and locked thread -->
<Story
	name="PinnedAndLocked"
	args={{
		thread: {
			...baseThread,
			id: 4,
			title: 'Announcement: Site Maintenance This Weekend',
			isSticky: true,
			isLocked: true,
			categories: [],
		},
	}}
/>

<!-- Thread without user avatar -->
<Story
	name="NoAvatar"
	args={{
		thread: {
			...baseThread,
			id: 5,
			user: { id: 2, name: 'AnonymousUser' },
		},
	}}
/>
