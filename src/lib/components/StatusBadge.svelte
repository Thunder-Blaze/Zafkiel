<script lang="ts">
	/**
	 * Colored status badge for MediaList entries.
	 * Each status has a distinct, semantic color.
	 */
	import type { MediaListStatus } from '$lib/types/anilist';

	let {
		status,
		size = 'sm',
		class: className = '',
	}: {
		status?: MediaListStatus | null;
		size?: 'xs' | 'sm' | 'md';
		class?: string;
	} = $props();

	const STATUS_LABEL: Record<MediaListStatus, string> = {
		CURRENT: 'Watching',
		COMPLETED: 'Completed',
		PLANNING: 'Planning',
		PAUSED: 'Paused',
		DROPPED: 'Dropped',
		REPEATING: 'Rewatching',
	};

	/** Tailwind classes per status: bg + text */
	const STATUS_COLORS: Record<MediaListStatus, string> = {
		CURRENT: 'bg-blue-500/15 text-blue-600 dark:text-blue-400',
		COMPLETED: 'bg-green-500/15 text-green-600 dark:text-green-400',
		PLANNING: 'bg-purple-500/15 text-purple-600 dark:text-purple-400',
		PAUSED: 'bg-yellow-500/15 text-yellow-600 dark:text-yellow-400',
		DROPPED: 'bg-red-500/15 text-red-500',
		REPEATING: 'bg-cyan-500/15 text-cyan-600 dark:text-cyan-400',
	};

	const SIZE_CLASSES = {
		xs: 'px-1.5 py-0 text-[10px] rounded',
		sm: 'px-2 py-0.5 text-xs rounded-md',
		md: 'px-2.5 py-1 text-sm rounded-lg',
	};

	const colorClass = $derived(status ? STATUS_COLORS[status] : 'bg-muted text-muted-foreground');
	const label = $derived(status ? STATUS_LABEL[status] : '—');
</script>

<span class="inline-flex items-center font-medium {colorClass} {SIZE_CLASSES[size]} {className}">
	{label}
</span>
