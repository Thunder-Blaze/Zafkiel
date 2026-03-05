<script lang="ts">
	import { useActivityFeed, type ActivityFeedFilter } from '$lib/hooks/useAnilist.svelte';
	import { isAuthenticated, currentUser } from '$lib/stores/auth';
	import type { ActivityUnion } from '$lib/types/anilist';
	import ActivityCard from '$lib/components/ActivityCard.svelte';
	import ActivityComposer from '$lib/components/ActivityComposer.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { useQueryClient } from '@tanstack/svelte-query';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	const queryClient = useQueryClient();

	type FeedTab = 'global' | 'following';

	let activeTab = $state<FeedTab>('global');
	let activeFilter = $state<ActivityFeedFilter>('all');
	let currentPage = $state(1);
	let prefsLoaded = $state(false);

	onMount(async () => {
		try {
			type UiConfigResult = {
				success: boolean;
				data?: { activity_feed_tab: string; activity_feed_filter: string };
			};
			const result = await invoke<UiConfigResult>('get_ui_config');
			if (result.success && result.data) {
				activeTab = (result.data.activity_feed_tab as FeedTab) || 'global';
				activeFilter = (result.data.activity_feed_filter as ActivityFeedFilter) || 'all';
			}
		} catch {
			// Use defaults on error
		} finally {
			prefsLoaded = true;
		}
	});

	$effect(() => {
		if (!prefsLoaded) return;
		const tab = activeTab;
		const filter = activeFilter;
		invoke('update_activity_prefs', { tab, filter }).catch(() => {});
	});

	const isFollowing = $derived(activeTab === 'following');

	const feedQuery = $derived(
		useActivityFeed({
			isFollowing,
			filter: activeFilter,
			page: currentPage,
			perPage: 25,
		})
	);

	const isLoading = $derived(feedQuery.isLoading);
	const activities = $derived((feedQuery.data?.data?.data ?? []) as ActivityUnion[]);
	const pageInfo = $derived(feedQuery.data?.data?.pageInfo);
	const hasError = $derived(!!feedQuery.error);
	const errorMsg = $derived((feedQuery.error as Error | null)?.message ?? '');
	const dataSuccess = $derived(feedQuery.data?.success !== false);

	function setTab(t: FeedTab) {
		activeTab = t;
		currentPage = 1;
	}

	function setFilter(f: ActivityFeedFilter) {
		activeFilter = f;
		currentPage = 1;
	}

	function refetch() {
		feedQuery.refetch();
	}

	function invalidateAndRefetch() {
		queryClient.invalidateQueries({ queryKey: ['activity', 'feed'] });
	}

	const filterOptions: { label: string; value: ActivityFeedFilter; icon: string }[] = [
		{ label: 'All', value: 'all', icon: 'solar:widget-2-bold-duotone' },
		{ label: 'List Activity', value: 'list', icon: 'solar:list-heart-minimalistic-bold-duotone' },
		{ label: 'Text Status', value: 'text', icon: 'solar:chat-square-bold-duotone' },
	];
</script>

<svelte:head>
	<title>Activity — Zafkiel</title>
</svelte:head>

<div class="min-h-screen w-full px-4 py-6 sm:px-6 lg:px-10">
	<!-- Page header -->
	<div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<h1 class="text-2xl font-bold tracking-tight">Activity Feed</h1>

		<!-- Following / Global toggle -->
		<div class="flex items-center rounded-[var(--radius)] border bg-muted/50 p-1 text-sm">
			<button
				onclick={() => setTab('global')}
				class="flex items-center gap-1.5 rounded-[var(--radius)] px-4 py-1.5 font-medium transition-all {activeTab ===
				'global'
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				<Icon icon="solar:global-bold" class="size-3.5" />
				Global
			</button>
			<button
				onclick={() => setTab('following')}
				class="flex items-center gap-1.5 rounded-[var(--radius)] px-4 py-1.5 font-medium transition-all {activeTab ===
				'following'
					? 'bg-background text-foreground shadow-sm'
					: 'text-muted-foreground hover:text-foreground'}"
			>
				<Icon icon="solar:users-group-rounded-bold" class="size-3.5" />
				Following
			</button>
		</div>
	</div>

	<!-- Type filters -->
	<div class="mb-5 flex flex-wrap items-center gap-2">
		{#each filterOptions as opt (opt.value)}
			<button
				onclick={() => setFilter(opt.value)}
			class="flex items-center gap-1.5 rounded-[var(--radius)] border px-3 py-1.5 text-xs font-medium transition-all {activeFilter ===
				opt.value
					? 'border-primary bg-primary text-primary-foreground'
					: 'border-border bg-card text-muted-foreground hover:border-primary/50 hover:text-foreground'}"
			>
				<Icon icon={opt.icon} class="size-3.5" />
				{opt.label}
			</button>
		{/each}
	</div>

	<div class="grid grid-cols-1 gap-6 lg:grid-cols-[1fr_300px]">
		<!-- Main feed column -->
		<div class="min-w-0">
			<!-- Composer — only when authenticated -->
			{#if $isAuthenticated}
				<div class="mb-5">
					<ActivityComposer
						userAvatar={$currentUser?.avatar?.medium}
						userName={$currentUser?.name}
						onPublished={invalidateAndRefetch}
					/>
				</div>
			{/if}

			<!-- Following — not-authenticated prompt -->
			{#if activeTab === 'following' && !$isAuthenticated}
				<div
					class="flex min-h-[300px] flex-col items-center justify-center gap-4 rounded-xl border bg-card px-8 py-12 text-center"
				>
					<div class="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
						<Icon
							icon="solar:users-group-rounded-bold-duotone"
							class="h-8 w-8 text-muted-foreground"
						/>
					</div>
					<div>
						<h3 class="mb-1 text-base font-semibold">Sign in to see following activity</h3>
						<p class="text-sm text-muted-foreground">
							Log in to view activity from people you follow.
						</p>
					</div>
				</div>
			{:else if isLoading}
				<!-- Skeleton loader -->
				<div class="flex flex-col gap-3">
					{#each Array(6) as _, i (i)}
						<div class="flex animate-pulse gap-3 rounded-xl border bg-card p-4">
							<div class="h-10 w-10 shrink-0 rounded-full bg-muted"></div>
							<div class="flex-1 space-y-2 pt-1">
								<div class="h-3.5 w-1/3 rounded bg-muted"></div>
								<div class="h-3 w-2/3 rounded bg-muted"></div>
								<div class="h-3 w-1/4 rounded bg-muted"></div>
							</div>
						</div>
					{/each}
				</div>
			{:else if hasError || !dataSuccess}
				<div
					class="flex min-h-[300px] flex-col items-center justify-center gap-4 rounded-xl border bg-card px-8 py-12 text-center"
				>
					<Icon
						icon="solar:danger-triangle-bold-duotone"
						class="h-12 w-12 text-destructive"
					/>
					<p class="text-muted-foreground">{errorMsg || 'Failed to load activity'}</p>
					<Button variant="outline" onclick={refetch}>
						<Icon icon="solar:refresh-bold" class="mr-1.5 size-4" />
						Try Again
					</Button>
				</div>
			{:else if activities.length === 0}
				<div
					class="flex min-h-[300px] flex-col items-center justify-center gap-3 rounded-xl border bg-card px-8 py-12 text-center"
				>
					<div class="flex h-16 w-16 items-center justify-center rounded-full bg-muted">
						<Icon icon="solar:widget-2-bold-duotone" class="h-8 w-8 text-muted-foreground" />
					</div>
					<div>
						<h3 class="mb-1 text-base font-semibold">No activity yet</h3>
						<p class="text-sm text-muted-foreground">
							{activeTab === 'following'
								? 'Follow users to see their activity here.'
								: 'No recent global activity found.'}
						</p>
					</div>
				</div>
			{:else}
				<div class="flex flex-col gap-3">
					{#each activities as activity, i (i)}
						<ActivityCard {activity} />
					{/each}
				</div>

				<!-- Pagination -->
				<div class="mt-8 flex items-center justify-center gap-3">
					<Button
						variant="outline"
						size="sm"
						disabled={currentPage <= 1}
						onclick={() => (currentPage -= 1)}
					>
						<Icon icon="solar:arrow-left-linear" class="size-4" />
						Previous
					</Button>
					<span class="text-sm text-muted-foreground">Page {currentPage}</span>
					<Button
						variant="outline"
						size="sm"
						disabled={!pageInfo?.hasNextPage}
						onclick={() => (currentPage += 1)}
					>
						Next
						<Icon icon="solar:arrow-right-linear" class="size-4" />
					</Button>
				</div>
			{/if}
		</div>

		<!-- Sidebar (hidden on mobile) -->
		<aside class="hidden lg:block">
			<div class="sticky top-6 space-y-4">
				<div class="rounded-xl border bg-card p-4">
					<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
						Feed Info
					</h3>
					<ul class="space-y-2 text-sm">
						<li class="flex items-center justify-between">
							<span class="text-muted-foreground">Mode</span>
							<span class="font-medium capitalize">{activeTab}</span>
						</li>
						<li class="flex items-center justify-between">
							<span class="text-muted-foreground">Filter</span>
							<span class="font-medium">
								{filterOptions.find((o) => o.value === activeFilter)?.label ?? 'All'}
							</span>
						</li>
						<li class="flex items-center justify-between">
							<span class="text-muted-foreground">Showing</span>
							<span class="font-medium">{activities.length}</span>
						</li>
					</ul>
				</div>

				<Button variant="outline" class="w-full" onclick={refetch} disabled={isLoading}>
					<Icon
						icon="solar:refresh-bold"
						class="mr-1.5 size-4 {isLoading ? 'animate-spin' : ''}"
					/>
					Refresh Feed
				</Button>

				<div class="rounded-xl border bg-card p-4">
					<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
						Activity Types
					</h3>
					<ul class="space-y-2.5 text-xs text-muted-foreground">
						<li class="flex items-center gap-2">
							<Icon
								icon="solar:list-heart-minimalistic-bold-duotone"
								class="size-4 shrink-0 text-blue-500"
							/>
							Anime &amp; manga list updates
						</li>
						<li class="flex items-center gap-2">
							<Icon
								icon="solar:chat-square-bold-duotone"
								class="size-4 shrink-0 text-green-500"
							/>
							Text status posts
						</li>
						<li class="flex items-center gap-2">
							<Icon
								icon="solar:letter-bold-duotone"
								class="size-4 shrink-0 text-purple-500"
							/>
							Messages between users
						</li>
					</ul>
				</div>
			</div>
		</aside>
	</div>
</div>
