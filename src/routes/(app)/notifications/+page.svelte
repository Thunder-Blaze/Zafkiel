<script lang="ts">
	import { useNotifications, useMarkNotificationsRead } from '$lib/hooks/useAnilist.svelte';
	import type { NotificationUnion } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { gsapReveal, gsapStagger } from '$lib/utils/gsap-animations';
	import PageLoader from '$lib/components/PageLoader.svelte';
	import { goto } from '$app/navigation';
	import { createVirtualizer } from '@tanstack/svelte-virtual';

	type FilterCategory = 'ALL' | 'AIRING' | 'ACTIVITY' | 'FORUM' | 'FOLLOWS' | 'MEDIA';

	const FILTER_TYPES: Partial<Record<FilterCategory, string[]>> = {
		AIRING: ['AIRING'],
		ACTIVITY: [
			'ACTIVITY_MESSAGE',
			'ACTIVITY_REPLY',
			'ACTIVITY_MENTION',
			'ACTIVITY_LIKE',
			'ACTIVITY_REPLY_LIKE',
			'ACTIVITY_REPLY_SUBSCRIBED',
		],
		FORUM: [
			'THREAD_COMMENT_MENTION',
			'THREAD_SUBSCRIBED',
			'THREAD_COMMENT_REPLY',
			'THREAD_LIKE',
			'THREAD_COMMENT_LIKE',
		],
		FOLLOWS: ['FOLLOWING'],
		MEDIA: ['RELATED_MEDIA_ADDITION', 'MEDIA_DATA_CHANGE', 'MEDIA_MERGE', 'MEDIA_DELETION'],
	};

	const FILTERS: { id: FilterCategory; label: string; icon: string }[] = [
		{ id: 'ALL', label: 'All', icon: 'solar:bell-bold-duotone' },
		{ id: 'AIRING', label: 'Airing', icon: 'solar:tv-bold-duotone' },
		{ id: 'ACTIVITY', label: 'Activity', icon: 'solar:chat-round-dots-bold-duotone' },
		{ id: 'FORUM', label: 'Forum', icon: 'solar:chat-square-bold-duotone' },
		{ id: 'FOLLOWS', label: 'Follows', icon: 'solar:user-plus-bold-duotone' },
		{ id: 'MEDIA', label: 'Media', icon: 'solar:play-circle-bold-duotone' },
	];

	let activeFilter = $state<FilterCategory>('ALL');
	let currentPage = $state(1);
	const notifQuery = $derived(useNotifications(currentPage, 30));
	const markReadMutation = useMarkNotificationsRead();

	const isLoading = $derived(notifQuery.isLoading);
	const allNotifications = $derived((notifQuery.data?.data?.data ?? []) as NotificationUnion[]);
	const pageInfo = $derived(notifQuery.data?.data?.pageInfo);

	const notifications = $derived(
		activeFilter === 'ALL'
			? allNotifications
			: allNotifications.filter((n) => (FILTER_TYPES[activeFilter] ?? []).includes(n.type))
	);

	function timeAgo(ts?: number): string {
		if (!ts) return '';
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}

	function userLink(id?: number, name?: string): string {
		const display = name ?? 'Someone';
		const href = id ? `/user/${id}` : '#';
		return `<a href="${href}" class="font-semibold text-foreground hover:text-primary transition-colors">${display}</a>`;
	}

	function mediaLink(id?: number, title?: string, type: 'anime' | 'manga' = 'anime'): string {
		const display = title ?? 'Unknown';
		const href = id ? `/${type}/${id}` : '#';
		return `<a href="${href}" class="font-semibold text-foreground hover:text-primary transition-colors">${display}</a>`;
	}

	interface NotifInfo {
		icon: string;
		iconClass: string;
		html: string;
		time: number | undefined;
		coverImage?: string;
		avatar?: string;
		link?: string;
	}

	function getNotifInfo(n: NotificationUnion): NotifInfo {
		switch (n.type) {
			case 'AIRING':
				return {
					icon: 'solar:tv-bold-duotone',
					iconClass: 'text-blue-500',
					html: `Episode <strong>${n.episode ?? '?'}</strong> of ${mediaLink(n.media?.id, n.media?.title?.userPreferred)} aired`,
					time: n.createdAt,
					coverImage: n.media?.coverImage?.medium ?? undefined,
					link: n.media?.id ? `/anime/${n.media.id}` : undefined,
				};
			case 'FOLLOWING':
				return {
					icon: 'solar:user-plus-bold-duotone',
					iconClass: 'text-green-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'started following you'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.user?.id ? `/user/${n.user.id}` : undefined,
				};
			case 'ACTIVITY_MESSAGE':
				return {
					icon: 'solar:chat-round-dots-bold-duotone',
					iconClass: 'text-purple-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'sent you a message'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.user?.id ? `/user/${n.user.id}` : undefined,
				};
			case 'ACTIVITY_MENTION':
				return {
					icon: 'solar:at-bold-duotone',
					iconClass: 'text-yellow-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'mentioned you in an activity'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'ACTIVITY_REPLY':
				return {
					icon: 'solar:chat-square-arrow-bold-duotone',
					iconClass: 'text-orange-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'replied to your activity'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'ACTIVITY_REPLY_SUBSCRIBED':
				return {
					icon: 'solar:bell-bold-duotone',
					iconClass: 'text-blue-400',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'replied to a subscribed activity'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'ACTIVITY_LIKE':
				return {
					icon: 'solar:heart-bold-duotone',
					iconClass: 'text-red-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'liked your activity'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'ACTIVITY_REPLY_LIKE':
				return {
					icon: 'solar:heart-bold-duotone',
					iconClass: 'text-pink-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'liked your reply'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'THREAD_COMMENT_MENTION':
				return {
					icon: 'solar:at-bold-duotone',
					iconClass: 'text-yellow-500',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'mentioned you in a forum comment'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.thread?.id ? `/forum/${n.thread.id}` : undefined,
				};
			case 'THREAD_COMMENT_REPLY':
				return {
					icon: 'solar:chat-square-arrow-bold-duotone',
					iconClass: 'text-orange-400',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'replied to your forum comment'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.thread?.id ? `/forum/${n.thread.id}` : undefined,
				};
			case 'THREAD_SUBSCRIBED':
				return {
					icon: 'solar:bell-bold-duotone',
					iconClass: 'text-blue-400',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'replied to a subscribed forum thread'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.thread?.id ? `/forum/${n.thread.id}` : undefined,
				};
			case 'THREAD_LIKE':
				return {
					icon: 'solar:heart-bold-duotone',
					iconClass: 'text-red-400',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'liked your forum thread'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
					link: n.thread?.id ? `/forum/${n.thread.id}` : undefined,
				};
			case 'THREAD_COMMENT_LIKE':
				return {
					icon: 'solar:heart-bold-duotone',
					iconClass: 'text-pink-400',
					html: `${userLink(n.user?.id, n.user?.name)} ${n.context ?? 'liked your forum comment'}`,
					time: n.createdAt,
					avatar: n.user?.avatar?.medium ?? undefined,
				};
			case 'RELATED_MEDIA_ADDITION':
				return {
					icon: 'solar:add-circle-bold-duotone',
					iconClass: 'text-teal-500',
					html: `${mediaLink(n.media?.id, n.media?.title?.userPreferred)} was added to your list as related media`,
					time: n.createdAt,
					coverImage: n.media?.coverImage?.medium ?? undefined,
					link: n.media?.id ? `/anime/${n.media.id}` : undefined,
				};
			case 'MEDIA_DATA_CHANGE':
				return {
					icon: 'solar:pen-bold-duotone',
					iconClass: 'text-amber-500',
					html: `${mediaLink(n.media?.id, n.media?.title?.userPreferred ?? 'A media')} had its data updated`,
					time: n.createdAt,
					coverImage: n.media?.coverImage?.medium ?? undefined,
					link: n.media?.id ? `/anime/${n.media.id}` : undefined,
				};
			case 'MEDIA_MERGE':
				return {
					icon: 'solar:merge-cells-bold-duotone',
					iconClass: 'text-indigo-500',
					html: `${mediaLink(n.media?.id, n.media?.title?.userPreferred ?? 'A media')} was merged into another entry`,
					time: n.createdAt,
					coverImage: n.media?.coverImage?.medium ?? undefined,
					link: n.media?.id ? `/anime/${n.media.id}` : undefined,
				};
			case 'MEDIA_DELETION':
				return {
					icon: 'solar:trash-bin-bold-duotone',
					iconClass: 'text-destructive',
					html: `<strong>${n.deletedMediaTitle ?? 'A media'}</strong> was deleted from the site`,
					time: n.createdAt,
				};
			default:
				return {
					icon: 'solar:bell-bold-duotone',
					iconClass: 'text-muted-foreground',
					html: 'New notification',
					time: undefined,
				};
		}
	}

	function handleRowClick(e: MouseEvent, info: NotifInfo) {
		const anchor = (e.target as HTMLElement).closest('a');
		if (anchor) {
			e.preventDefault();
			const href = anchor.getAttribute('href');
			if (href && href !== '#') goto(href);
			return;
		}
		if (info.link) goto(info.link);
	}

	function changeFilter(f: FilterCategory) {
		activeFilter = f;
		currentPage = 1;
	}

	let notifScrollEl = $state<HTMLDivElement>();

	const virtualizer = createVirtualizer({
		count: 0,
		getScrollElement: () => notifScrollEl ?? null,
		estimateSize: () => 80,
		overscan: 10,
	});

	$effect(() => {
		$virtualizer.setOptions({
			count: notifications.length,
			getScrollElement: () => notifScrollEl ?? null,
			estimateSize: () => 80,
			overscan: 10,
		});
	});

	function measure(node: HTMLElement) {
		$virtualizer.measureElement(node);
		return {
			destroy() {
				$virtualizer.measureElement(node);
			},
		};
	}
</script>

<svelte:head>
	<title>Notifications — Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-6 lg:px-8">
	<!-- Page header -->
	<div use:gsapReveal class="mb-6 flex items-center justify-between">
		<h1 class="text-2xl font-bold">Notifications</h1>
		<Button
			variant="outline"
			size="sm"
			onclick={() => markReadMutation.mutate({})}
			disabled={markReadMutation.isPending}
		>
			{#if markReadMutation.isPending}
				<Icon icon="solar:refresh-circle-line-duotone" class="mr-1.5 size-4 animate-spin" />
			{:else}
				<Icon icon="solar:check-read-linear" class="mr-1.5 size-4" />
			{/if}
			Mark all read
		</Button>
	</div>

	<div class="flex gap-6">
		<!-- ── Sidebar filters (md+) ── -->
		<aside class="hidden w-44 shrink-0 md:block">
			<nav class="flex flex-col gap-1" aria-label="Notification filters">
				{#each FILTERS as filter (filter.id)}
					<button
						class="flex items-center gap-2.5 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors
							{activeFilter === filter.id
							? 'bg-primary/10 text-primary'
							: 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
						onclick={() => changeFilter(filter.id)}
						aria-current={activeFilter === filter.id ? 'page' : undefined}
					>
						<Icon icon={filter.icon} class="size-4 shrink-0" />
						{filter.label}
					</button>
				{/each}
			</nav>
		</aside>

		<!-- ── Main column ── -->
		<div class="min-w-0 flex-1">
			<!-- Mobile filter pills -->
			<div class="mb-4 flex gap-1.5 overflow-x-auto pb-1 md:hidden">
				{#each FILTERS as filter (filter.id)}
					<button
						class="flex shrink-0 items-center gap-1.5 rounded-full px-3 py-1.5 text-xs font-medium transition-colors
							{activeFilter === filter.id
							? 'bg-primary text-primary-foreground'
							: 'bg-muted text-muted-foreground hover:bg-muted/80'}"
						onclick={() => changeFilter(filter.id)}
					>
						<Icon icon={filter.icon} class="size-3.5" />
						{filter.label}
					</button>
				{/each}
			</div>

			<!-- Loading -->
			{#if isLoading}
				<PageLoader type="default" />

				<!-- Error -->
			{:else if notifQuery.error || (notifQuery.data && !notifQuery.data.success)}
				<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
					<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
					<p class="text-muted-foreground">
						{notifQuery.error?.message || notifQuery.data?.error || 'Failed to load notifications'}
					</p>
					<Button variant="outline" onclick={() => notifQuery.refetch()}>Try Again</Button>
				</div>

				<!-- Empty -->
			{:else if notifications.length === 0}
				<div class="flex min-h-[200px] flex-col items-center justify-center gap-3 text-center">
					<Icon icon="solar:bell-off-bold-duotone" class="h-16 w-16 text-muted-foreground" />
					<h3 class="text-lg font-semibold">No notifications</h3>
					<p class="text-sm text-muted-foreground">
						{activeFilter === 'ALL' ? "You're all caught up!" : 'None in this category.'}
					</p>
				</div>

				<!-- Notification list -->
			{:else}
				<div
					class="h-[calc(100vh-280px)] min-h-[400px] w-full overflow-y-auto"
					data-lenis-prevent="true"
					bind:this={notifScrollEl}
				>
					<div style="height: {$virtualizer.getTotalSize()}px; width: 100%; position: relative;">
						{#each $virtualizer.getVirtualItems() as virtualRow (virtualRow.index)}
							{@const notif = notifications[virtualRow.index]}
							{#if notif}
							{@const info = getNotifInfo(notif)}
							<div
								use:measure
								data-index={virtualRow.index}
								style="position: absolute; top: 0; left: 0; width: 100%; transform: translateY({virtualRow.start}px);"
							>
								<div class="mb-2">
									<div
										class="group relative rounded-lg border bg-card transition-colors hover:bg-card/80"
									>
										<button
											type="button"
											class="flex w-full cursor-pointer items-center gap-3 p-4 text-left"
											onclick={(e) => handleRowClick(e, info)}
										>
											<!-- Cover image or Avatar -->
											<div class="relative shrink-0">
												{#if info.coverImage}
													<div
														class="relative h-16 w-11 overflow-hidden rounded-sm bg-muted shadow-sm"
													>
														<CachedImage
															src={info.coverImage}
															alt=""
															class="h-full w-full object-cover"
														/>
													</div>
													<div
														class="absolute -right-1 -bottom-1 rounded-full border border-border bg-card p-0.5"
													>
														<Icon icon={info.icon} class="size-3 {info.iconClass}" />
													</div>
												{:else if info.avatar}
													<div class="relative h-10 w-10">
														<CachedImage
															src={info.avatar}
															alt=""
															class="h-10 w-10 rounded-full object-cover ring-1 ring-border"
														/>
														<div
															class="absolute -right-1 -bottom-1 rounded-full border border-border bg-card p-0.5"
														>
															<Icon icon={info.icon} class="size-3 {info.iconClass}" />
														</div>
													</div>
												{:else}
													<div
														class="flex h-10 w-10 items-center justify-center rounded-full bg-muted"
													>
														<Icon icon={info.icon} class="size-5 {info.iconClass}" />
													</div>
												{/if}
											</div>

											<!-- Text content -->
											<div class="min-w-0 flex-1">
												<p class="text-sm leading-snug">
													<!-- eslint-disable-next-line svelte/no-at-html-tags -->
													{@html info.html}
												</p>
												{#if info.time}
													<p class="mt-1 text-xs text-muted-foreground">
														{timeAgo(info.time)}
													</p>
												{/if}
											</div>

											<!-- Navigation arrow -->
											{#if info.link}
												<Icon
													icon="solar:arrow-right-linear"
													class="size-4 shrink-0 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100"
												/>
											{/if}
										</button>
									</div>
								</div>
							</div>
							{/if}
						{/each}
					</div>
				</div>
			{/if}

			<!-- Pagination — always visible when pageInfo exists (even if filtered category is empty) -->
			{#if pageInfo && !isLoading}
				<div class="mt-6 flex items-center justify-center gap-3">
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
						disabled={!pageInfo.hasNextPage}
						onclick={() => (currentPage += 1)}
					>
						Next
						<Icon icon="solar:arrow-right-linear" class="size-4" />
					</Button>
				</div>
			{/if}
		</div>
	</div>
</div>
