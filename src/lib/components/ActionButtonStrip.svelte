<script lang="ts">
	import type { MediaListStatus } from '$lib/types/anilist';
	import { useAddAnimeToList, useAddMangaToList } from '$lib/hooks/useAnilist.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';

	let {
		userStatus = null,
		mediaId,
		mediaType = 'ANIME',
	}: {
		userStatus?: MediaListStatus | null;
		mediaId: number;
		mediaType?: 'ANIME' | 'MANGA';
	} = $props();

	const addAnimeMutation = useAddAnimeToList();
	const addMangaMutation = useAddMangaToList();

	const isPending = $derived(addAnimeMutation.isPending || addMangaMutation.isPending);

	async function handleStatusChange(newStatus: MediaListStatus) {
		if (isPending) return;
		try {
			if (mediaType === 'MANGA') {
				await addMangaMutation.mutateAsync({ mediaId, status: newStatus });
			} else {
				await addAnimeMutation.mutateAsync({ mediaId, status: newStatus });
			}
			const labels: Record<MediaListStatus, string> = {
				CURRENT: mediaType === 'MANGA' ? 'Reading' : 'Watching',
				COMPLETED: 'Completed',
				PLANNING: 'Plan to watch',
				DROPPED: 'Dropped',
				PAUSED: 'Paused',
				REPEATING: 'Repeating',
			};
			toast.success(labels[newStatus]);
		} catch {
			toast.error('Failed to update status');
		}
	}
</script>

<div class="flex items-center justify-between gap-1 rounded-md bg-border p-1">
	{#if isPending}
		<div class="flex w-full items-center justify-center py-1">
			<Icon icon="solar:refresh-circle-line-duotone" class="size-4 animate-spin text-primary" />
		</div>
	{:else}
		<button
			onclick={() => handleStatusChange('PLANNING')}
			class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
			'PLANNING'
				? 'bg-primary/30'
				: 'bg-background/50'}"
			title="Plan to Watch"
		>
			<Icon icon="solar:bookmark-linear" class="size-3.5" />
			Plan
		</button>
		<button
			onclick={() => handleStatusChange('CURRENT')}
			class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
			'CURRENT'
				? 'bg-primary/30'
				: 'bg-background/50'}"
			title={mediaType === 'MANGA' ? 'Reading' : 'Watching'}
		>
			<Icon icon="solar:play-circle-linear" class="size-3.5" />
			{mediaType === 'MANGA' ? 'Read' : 'Watch'}
		</button>
		<button
			onclick={() => handleStatusChange('COMPLETED')}
			class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
			'COMPLETED'
				? 'bg-primary/30'
				: 'bg-background/50'}"
			title="Completed"
		>
			<Icon icon="solar:check-circle-linear" class="size-3.5" />
			Done
		</button>
		<button
			onclick={() => handleStatusChange('PAUSED')}
			class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
			'PAUSED'
				? 'bg-primary/30'
				: 'bg-background/50'}"
			title="Paused"
		>
			<Icon icon="solar:pause-circle-linear" class="size-3.5" />
		</button>
		<button
			onclick={() => handleStatusChange('DROPPED')}
			class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
			'DROPPED'
				? 'bg-primary/30'
				: 'bg-background/50'}"
			title="Dropped"
		>
			<Icon icon="solar:close-circle-linear" class="size-3.5" />
		</button>
	{/if}
</div>
