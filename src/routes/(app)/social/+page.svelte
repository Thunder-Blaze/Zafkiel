<script lang="ts">
	import { useRecentActivity, useFollowingActivity } from '$lib/hooks/useAnilist.svelte';
	import { isAuthenticated } from '$lib/stores/auth';
	import type { ActivityUnion } from '$lib/types/anilist';
	import ActivityCard from '$lib/components/ActivityCard.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	type FeedTab = 'global' | 'following';
	let activeTab = $state<FeedTab>('global');
	let currentPage = $state(1);

	const globalQuery = $derived(useRecentActivity(currentPage, 20));
	const followingQuery = $derived(useFollowingActivity(currentPage, 20));

	// ✅ Extract properties directly — never switch between whole query objects
	const isLoading = $derived(activeTab === 'global' ? globalQuery.isLoading : followingQuery.isLoading);
	const activities = $derived(
		((activeTab === 'global' ? globalQuery.data : followingQuery.data)?.data?.data ?? []) as ActivityUnion[]
	);
	const pageInfo = $derived(
		(activeTab === 'global' ? globalQuery.data : followingQuery.data)?.data?.pageInfo
	);
	const hasError = $derived(activeTab === 'global' ? !!globalQuery.error : !!followingQuery.error);
	const errorMsg = $derived(
		activeTab === 'global'
			? (globalQuery.error as Error | null)?.message ?? ''
			: (followingQuery.error as Error | null)?.message ?? ''
	);
	const dataSuccess = $derived(
		activeTab === 'global'
			? globalQuery.data?.success !== false
			: followingQuery.data?.success !== false
	);

	function setTab(t: FeedTab) {
		activeTab = t;
		currentPage = 1;
	}

	function refetch() {
		if (activeTab === 'global') globalQuery.refetch();
		else followingQuery.refetch();
	}
</script>

<svelte:head>
	<title>Social Feed — Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-3xl px-4 py-6">
	<!-- Header -->
	<div class="mb-6 flex items-center justify-between">
		<h1 class="text-2xl font-bold">Activity Feed</h1>
	</div>

	<!-- Tabs -->
	<div class="mb-6 flex gap-2 border-b pb-0">
		<button
			onclick={() => setTab('global')}
			class="px-4 py-2 text-sm font-medium transition-colors border-b-2 {activeTab === 'global' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:global-bold-duotone" class="mr-1.5 inline size-4" />
			Global
		</button>
		<button
			onclick={() => setTab('following')}
			class="px-4 py-2 text-sm font-medium transition-colors border-b-2 {activeTab === 'following' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}"
		>
			<Icon icon="solar:users-group-rounded-bold-duotone" class="mr-1.5 inline size-4" />
			Following
		</button>
	</div>

	<!-- Content -->
	{#if isLoading}
		<div class="flex min-h-[300px] items-center justify-center">
			<Icon icon="solar:refresh-circle-line-duotone" class="h-10 w-10 animate-spin text-primary" />
		</div>
	{:else if activeTab === 'following' && !$isAuthenticated}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<Icon icon="solar:users-group-rounded-bold-duotone" class="h-12 w-12 text-muted-foreground" />
			<h3 class="text-lg font-semibold">Sign in to see following activity</h3>
			<p class="text-sm text-muted-foreground">Log in to view activity from people you follow.</p>
		</div>
	{:else if hasError || !dataSuccess}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
			<p class="text-muted-foreground">{errorMsg || 'Failed to load activity'}</p>
			<Button variant="outline" onclick={refetch}>Try Again</Button>
		</div>
	{:else if activities.length === 0}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-3 text-center">
			<Icon icon="solar:widget-2-bold-duotone" class="h-16 w-16 text-muted-foreground" />
			<h3 class="text-lg font-semibold">No activity yet</h3>
			<p class="text-sm text-muted-foreground">
				{activeTab === 'following'
					? 'Follow users to see their activity here.'
					: 'No recent global activity found.'}
			</p>
		</div>
	{:else}
		<div class="flex flex-col gap-3">
			{#each activities as activity, i (i)}
				<ActivityCard {activity} />
			{/each}
		</div>

		<!-- Pagination -->
		{#if pageInfo}
			<div class="mt-8 flex items-center justify-center gap-3">
				<Button
					variant="outline"
					disabled={currentPage <= 1}
					onclick={() => (currentPage -= 1)}
				>
					<Icon icon="solar:arrow-left-linear" class="size-4" />
					Previous
				</Button>
				<span class="text-sm text-muted-foreground">Page {currentPage}</span>
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
