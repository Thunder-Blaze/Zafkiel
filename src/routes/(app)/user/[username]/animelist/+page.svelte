<script lang="ts">
	import { page } from '$app/state';
	import { useUserAnimeList } from '$lib/hooks/useAnilist.svelte';
	import type { MediaListStatus, MediaList } from '$lib/types/anilist';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import MediaListRow from '$lib/components/MediaListRow.svelte';
	import MediaListRowSkeleton from '$lib/components/MediaListRowSkeleton.svelte';
	import MediaCardSkeleton from '$lib/components/MediaCardSkeleton.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	const username = $derived(page.params.username ?? '');

	let activeStatus = $state<MediaListStatus | undefined>(undefined);
	let currentPage = $state(1);
	type ViewMode = 'grid' | 'list';
	let viewMode = $state<ViewMode>('list');

	const animeQuery = $derived(useUserAnimeList(username, activeStatus, currentPage, 50));
	const isLoading = $derived(animeQuery.isLoading);
	const entries = $derived((animeQuery.data?.data?.data ?? []) as MediaList[]);
	const pageInfo = $derived(animeQuery.data?.data?.pageInfo);

	const statusOptions: { label: string; value: MediaListStatus | undefined }[] = [
		{ label: 'All', value: undefined },
		{ label: 'Watching', value: 'CURRENT' },
		{ label: 'Completed', value: 'COMPLETED' },
		{ label: 'Planning', value: 'PLANNING' },
		{ label: 'Paused', value: 'PAUSED' },
		{ label: 'Dropped', value: 'DROPPED' },
		{ label: 'Repeating', value: 'REPEATING' }
	];

	function setStatus(s: MediaListStatus | undefined) {
		activeStatus = s;
		currentPage = 1;
	}
</script>

<svelte:head>
	<title>{username}'s Anime List — Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-6">
	<!-- Header -->
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold">{username}'s Anime List</h1>
		</div>
		<div class="flex items-center gap-3">
			<!-- Switch to manga list -->
			<Button
				variant="outline"
				size="sm"
				onclick={() => goto(`/user/${username}/mangalist`)}
			>
				<Icon icon="solar:book-2-bold-duotone" class="mr-1.5 size-4" />
				Manga List
			</Button>
			<!-- View mode toggle -->
			<div class="flex items-center gap-1 rounded-lg border bg-card p-1">
				<button
					onclick={() => (viewMode = 'list')}
					class="rounded-md p-1.5 transition-colors {viewMode === 'list' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
					title="List view"
				>
					<Icon icon="solar:list-bold-duotone" class="size-4" />
				</button>
				<button
					onclick={() => (viewMode = 'grid')}
					class="rounded-md p-1.5 transition-colors {viewMode === 'grid' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
					title="Grid view"
				>
					<Icon icon="solar:widget-2-bold-duotone" class="size-4" />
				</button>
			</div>
		</div>
	</div>

	<!-- Status filter chips -->
	<div class="mb-6 flex flex-wrap gap-2">
		{#each statusOptions as opt}
			<button
				onclick={() => setStatus(opt.value)}
				class="rounded-full px-3 py-1 text-xs font-medium transition-colors {activeStatus === opt.value ? 'bg-primary text-primary-foreground' : 'bg-card border hover:bg-card/80'}"
			>
				{opt.label}
			</button>
		{/each}
	</div>

	<!-- Content -->
	{#if isLoading}
		{#if viewMode === 'list'}
			<div class="overflow-hidden rounded-lg border bg-card">
				<div class="flex items-center gap-3 border-b bg-muted/40 px-3 py-2 text-xs font-medium text-muted-foreground">
					<div class="size-10 shrink-0"></div>
					<div class="flex-1">Title</div>
					<div class="w-20 shrink-0">Status</div>
					<div class="w-10 shrink-0 text-right">Score</div>
					<div class="w-16 shrink-0 text-right">Progress</div>
				</div>
				<MediaListRowSkeleton count={15} />
			</div>
		{:else}
			<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6">
				<MediaCardSkeleton count={18} />
			</div>
		{/if}
	{:else if animeQuery.error || (animeQuery.data && !animeQuery.data.success)}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
			<p class="text-muted-foreground">
				{animeQuery.error?.message || animeQuery.data?.error || 'Failed to load anime list'}
			</p>
			<Button variant="outline" onclick={() => animeQuery.refetch()}>Try Again</Button>
		</div>
	{:else if entries.length === 0}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-3 text-center">
			<Icon icon="solar:play-circle-bold-duotone" class="h-16 w-16 text-muted-foreground" />
			<h3 class="text-lg font-semibold">No anime found</h3>
			<p class="text-sm text-muted-foreground">
				{username} hasn't added any anime in this category yet.
			</p>
		</div>
	{:else}
		{#if viewMode === 'list'}
			<div class="overflow-hidden rounded-lg border bg-card" role="table">
				<div
					class="flex items-center gap-3 border-b bg-muted/40 px-3 py-2 text-xs font-medium text-muted-foreground"
					role="rowheader"
				>
					<div class="size-10 shrink-0"></div>
					<div class="flex-1">Title</div>
					<div class="w-20 shrink-0">Status</div>
					<div class="w-10 shrink-0 text-right">Score</div>
					<div class="w-16 shrink-0 text-right">Progress</div>
				</div>
				{#each entries as entry (entry.id)}
					<MediaListRow {entry} />
				{/each}
			</div>
		{:else}
			<div class="flex flex-wrap gap-4 justify-start">
				{#each entries as entry (entry.id)}
					{#if entry.media}
						<MediaCard media={entry.media} />
					{/if}
				{/each}
			</div>
		{/if}

		<!-- Pagination -->
		{#if pageInfo && (currentPage > 1 || pageInfo.hasNextPage)}
			<div class="mt-8 flex items-center justify-center gap-3">
				<Button
					variant="outline"
					disabled={currentPage <= 1}
					onclick={() => (currentPage -= 1)}
				>
					<Icon icon="solar:arrow-left-linear" class="size-4" />
					Previous
				</Button>
				<span class="text-sm text-muted-foreground">
					Page {pageInfo.currentPage} of {pageInfo.lastPage}
				</span>
				<Button
					variant="outline"
					disabled={!pageInfo.hasNextPage}
					onclick={() => (currentPage += 1)}
				>
					Next
					<Icon icon="solar:arrow-right-linear" class="size-4" />
				</Button>
			</div>
		{/if}
	{/if}
</div>
