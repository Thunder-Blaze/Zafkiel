<script lang="ts">
	import { page } from '$app/state';
	import { useForumThread, useThreadComments, useReplyToThread, useToggleThreadSubscription } from '$lib/hooks/useAnilist.svelte';
	import type { ThreadComment } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';

	const threadId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const threadQuery = $derived(useForumThread(threadId));
	let commentPage = $state(1);
	const commentsQuery = $derived(useThreadComments(threadId, commentPage, 20));
	const replyMutation = useReplyToThread();
	const subscriptionMutation = useToggleThreadSubscription();

	const thread = $derived(threadQuery.data?.data);
	const isLoadingThread = $derived(threadQuery.isLoading);
	const comments = $derived((commentsQuery.data?.data?.data ?? []) as ThreadComment[]);
	const commentPageInfo = $derived(commentsQuery.data?.data?.pageInfo);
	const isLoadingComments = $derived(commentsQuery.isLoading);

	let replyText = $state('');
	let isReplying = $state(false);

	function timeAgo(ts?: number): string {
		if (!ts) return '';
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}

	async function handleReply() {
		if (!replyText.trim()) return;
		try {
			await replyMutation.mutateAsync({ threadId, comment: replyText });
			replyText = '';
			isReplying = false;
			toast.success('Reply posted');
		} catch {
			toast.error('Failed to post reply');
		}
	}

	async function toggleSubscribe() {
		if (!thread) return;
		try {
			await subscriptionMutation.mutateAsync({ threadId, subscribe: !thread.isSubscribed });
			toast.success(thread.isSubscribed ? 'Unsubscribed' : 'Subscribed');
		} catch {
			toast.error('Failed to update subscription');
		}
	}
</script>

<svelte:head>
	<title>{thread?.title ?? 'Forum Thread'} — Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-4xl px-4 py-6">
	<!-- Back link -->
	<button
		class="mb-4 flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
		onclick={() => goto('/forum')}
	>
		<Icon icon="solar:arrow-left-linear" class="size-4" />
		Back to Forum
	</button>

	{#if isLoadingThread}
		<div class="flex min-h-[300px] items-center justify-center">
			<Icon icon="solar:refresh-circle-line-duotone" class="h-10 w-10 animate-spin text-primary" />
		</div>
	{:else if threadQuery.error || (threadQuery.data && !threadQuery.data.success)}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
			<p class="text-muted-foreground">Failed to load thread</p>
			<Button variant="outline" onclick={() => threadQuery.refetch()}>Try Again</Button>
		</div>
	{:else if thread}
		<!-- Thread header -->
		<div class="mb-6 rounded-lg border bg-card p-6">
			<div class="mb-4 flex items-start justify-between gap-3">
				<h1 class="text-xl font-bold leading-snug">{thread.title}</h1>
				<div class="flex shrink-0 items-center gap-2">
					{#if thread.isSticky}
						<Badge variant="secondary">
							<Icon icon="solar:pin-bold" class="mr-1 size-3" />
							Pinned
						</Badge>
					{/if}
					{#if thread.isLocked}
						<Badge variant="outline">
							<Icon icon="solar:lock-bold" class="mr-1 size-3" />
							Locked
						</Badge>
					{/if}
					<Button
						variant={thread.isSubscribed ? 'secondary' : 'outline'}
						size="sm"
						onclick={toggleSubscribe}
						disabled={subscriptionMutation.isPending}
					>
						<Icon icon={thread.isSubscribed ? 'solar:bell-bold' : 'solar:bell-linear'} class="mr-1.5 size-4" />
						{thread.isSubscribed ? 'Subscribed' : 'Subscribe'}
					</Button>
				</div>
			</div>

			<!-- Author info -->
			{#if thread.user}
				<div class="mb-4 flex items-center gap-2 text-sm text-muted-foreground">
					<button onclick={() => thread.user?.id && goto(`/user/${thread.user.id}`)}>
						{#if thread.user.avatar?.medium}
							<CachedImage
								src={thread.user.avatar.medium}
								alt={thread.user.name ?? ''}
								class="h-7 w-7 rounded-full object-cover"
							/>
						{:else}
							<div class="flex h-7 w-7 items-center justify-center rounded-full bg-muted">
								<Icon icon="solar:user-bold" class="size-4" />
							</div>
						{/if}
					</button>
					<button class="hover:text-foreground hover:underline" onclick={() => thread.user?.id && goto(`/user/${thread.user.id}`)}>
						{thread.user.name}
					</button>
					<span>·</span>
					<span>{timeAgo(thread.createdAt)}</span>
				</div>
			{/if}

			<!-- Body -->
			{#if thread.body}
				<div class="prose prose-sm dark:prose-invert max-w-none">{@html thread.body}</div>
			{/if}

			<!-- Categories & stats -->
			{#if thread.categories && thread.categories.length > 0}
				<div class="mt-4 flex flex-wrap gap-1">
					{#each thread.categories as cat}
						<Badge variant="outline" class="text-xs">{cat.name}</Badge>
					{/each}
				</div>
			{/if}

			<div class="mt-4 flex items-center gap-4 text-xs text-muted-foreground">
				<span class="flex items-center gap-1">
					<Icon icon="solar:chat-square-linear" class="size-3.5" />
					{thread.replyCount ?? 0} replies
				</span>
				<span class="flex items-center gap-1">
					<Icon icon="solar:eye-linear" class="size-3.5" />
					{thread.viewCount ?? 0} views
				</span>
				<span class="flex items-center gap-1">
					<Icon icon="solar:heart-linear" class="size-3.5" />
					{thread.likeCount ?? 0}
				</span>
			</div>
		</div>

		<!-- Comments -->
		<div class="mb-4 flex items-center justify-between">
			<h2 class="text-lg font-semibold">Replies</h2>
		</div>

		{#if isLoadingComments}
			<div class="flex min-h-[200px] items-center justify-center">
				<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
			</div>
		{:else if comments.length === 0}
			<div class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-8 text-center">
				<Icon icon="solar:chat-square-bold-duotone" class="h-12 w-12 text-muted-foreground" />
				<p class="text-sm text-muted-foreground">No replies yet. Be the first!</p>
			</div>
		{:else}
			<div class="flex flex-col gap-3">
				{#each comments as comment (comment.id)}
					<div class="flex gap-3 rounded-lg border bg-card p-4">
						<button onclick={() => comment.user?.id && goto(`/user/${comment.user.id}`)}>
						{#if comment.user?.avatar?.medium}
							<CachedImage
								src={comment.user.avatar.medium}
									alt={comment.user.name ?? ''}
									class="h-9 w-9 rounded-full object-cover"
								/>
							{:else}
								<div class="flex h-9 w-9 items-center justify-center rounded-full bg-muted">
									<Icon icon="solar:user-bold" class="size-4" />
								</div>
							{/if}
						</button>
						<div class="min-w-0 flex-1">
							<div class="mb-2 flex items-center gap-2 text-sm">
								<button class="font-semibold hover:text-primary hover:underline" onclick={() => comment.user?.id && goto(`/user/${comment.user.id}`)}>
									{comment.user?.name ?? 'Unknown'}
								</button>
								<span class="text-xs text-muted-foreground">{timeAgo(comment.createdAt)}</span>
							</div>
							{#if comment.comment}
								<div class="prose prose-sm dark:prose-invert max-w-none text-sm">{@html comment.comment}</div>
							{/if}
							<div class="mt-2 flex items-center gap-2 text-xs text-muted-foreground">
								<span class="flex items-center gap-1">
									<Icon icon="solar:heart-linear" class="size-3.5" />
									{comment.likeCount ?? 0}
								</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Comment pagination -->
		{#if commentPageInfo}
			<div class="mt-6 flex items-center justify-center gap-3">
				<Button variant="outline" disabled={commentPage <= 1} onclick={() => (commentPage -= 1)}>
					<Icon icon="solar:arrow-left-linear" class="size-4" />
					Previous
				</Button>
				<span class="text-sm text-muted-foreground">Page {commentPage}</span>
				<Button variant="outline" disabled={!commentPageInfo.hasNextPage} onclick={() => (commentPage += 1)}>
					Next
					<Icon icon="solar:arrow-right-linear" class="size-4" />
				</Button>
			</div>
		{/if}

		<!-- Reply form -->
		{#if !thread.isLocked}
			<div class="mt-8 rounded-lg border bg-card p-4">
				<h3 class="mb-3 font-semibold">Post a Reply</h3>
				{#if isReplying}
					<div class="flex flex-col gap-3">
						<textarea
							bind:value={replyText}
							placeholder="Write your reply..."
							rows={4}
							class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary resize-y"
						></textarea>
						<div class="flex gap-2">
							<Button onclick={handleReply} disabled={!replyText.trim() || replyMutation.isPending}>
								{#if replyMutation.isPending}
									<Icon icon="solar:refresh-circle-line-duotone" class="mr-1.5 size-4 animate-spin" />
								{/if}
								Post Reply
							</Button>
							<Button variant="outline" onclick={() => { isReplying = false; replyText = ''; }}>Cancel</Button>
						</div>
					</div>
				{:else}
					<Button variant="outline" onclick={() => (isReplying = true)}>
						<Icon icon="solar:chat-square-linear" class="mr-1.5 size-4" />
						Write a Reply
					</Button>
				{/if}
			</div>
		{/if}
	{:else}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<h2 class="text-2xl font-bold">Thread Not Found</h2>
			<Button variant="outline" onclick={() => goto('/forum')}>Back to Forum</Button>
		</div>
	{/if}
</div>
