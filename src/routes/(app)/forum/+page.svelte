<script lang="ts">
	import {
		useRecentForumThreads,
		usePopularForumThreads
	} from '$lib/hooks/useAnilist.svelte';
	import type { Thread } from '$lib/types/anilist';
	import ThreadCard from '$lib/components/ThreadCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	type ForumTab = 'recent' | 'popular';
	let activeTab = $state<ForumTab>('recent');
	let currentPage = $state(1);

	const recentQuery = $derived(useRecentForumThreads(currentPage, 20));
	const popularQuery = $derived(usePopularForumThreads(currentPage, 20));

	const activeQuery = $derived(activeTab === 'recent' ? recentQuery : popularQuery);
	const isLoading = $derived(activeQuery.isLoading);
	const threads = $derived((activeQuery.data?.data?.data ?? []) as Thread[]);
	const pageInfo = $derived(activeQuery.data?.data?.pageInfo);

	function setTab(t: ForumTab) {
		activeTab = t;
		currentPage = 1;
	}
</script>

<svelte:head>
	<title>Forum — Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-4xl px-4 py-6">
	<!-- Header -->
	<div class="mb-6 flex items-center justify-between">
		<h1 class="text-2xl font-bold">AniList Forum</h1>
	</div>

	<!-- Tabs -->
	<div class="mb-6 flex gap-2 border-b pb-0">
		<button
			onclick={() => setTab('recent')}
			class="px-4 py-2 text-sm font-medium transition-colors border-b-2 {activeTab === 'recent' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:clock-circle-bold-duotone" class="mr-1.5 inline size-4" />
			Recent
		</button>
		<button
			onclick={() => setTab('popular')}
			class="px-4 py-2 text-sm font-medium transition-colors border-b-2 {activeTab === 'popular' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:fire-bold-duotone" class="mr-1.5 inline size-4" />
			Popular
		</button>
	</div>

	<!-- Content -->
	{#if isLoading}
		<div class="flex min-h-[300px] items-center justify-center">
			<Icon icon="solar:refresh-circle-line-duotone" class="h-10 w-10 animate-spin text-primary" />
		</div>
	{:else if activeQuery.error || (activeQuery.data && !activeQuery.data.success)}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
			<p class="text-muted-foreground">
				{activeQuery.error?.message || activeQuery.data?.error || 'Failed to load threads'}
			</p>
			<Button variant="outline" onclick={() => activeQuery.refetch()}>Try Again</Button>
		</div>
	{:else if threads.length === 0}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-3 text-center">
			<Icon icon="solar:chat-square-bold-duotone" class="h-16 w-16 text-muted-foreground" />
			<h3 class="text-lg font-semibold">No threads found</h3>
		</div>
	{:else}
		<div class="flex flex-col gap-3">
			{#each threads as thread (thread.id)}
				<ThreadCard {thread} />
			{/each}
		</div>

		<!-- Pagination -->
		{#if pageInfo}
			<div class="mt-8 flex items-center justify-center gap-3">
				<Button variant="outline" disabled={currentPage <= 1} onclick={() => (currentPage -= 1)}>
					<Icon icon="solar:arrow-left-linear" class="size-4" />
					Previous
				</Button>
				<span class="text-sm text-muted-foreground">
					Page {pageInfo.currentPage} of {pageInfo.lastPage}
				</span>
				<Button variant="outline" disabled={!pageInfo.hasNextPage} onclick={() => (currentPage += 1)}>
					Next
					<Icon icon="solar:arrow-right-linear" class="size-4" />
				</Button>
			</div>
		{/if}
	{/if}
</div>
