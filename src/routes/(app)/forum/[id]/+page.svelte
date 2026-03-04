<script lang="ts">
	import { page } from '$app/state';
	import {
		useForumThread,
		useThreadComments,
		useReplyToThread,
		useToggleThreadSubscription,
		useToggleLikeThread,
		useToggleLikeComment,
		useReplyToComment,
	} from '$lib/hooks/useAnilist.svelte';
	import type { ThreadComment } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';
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
	const likeThreadMutation = useToggleLikeThread();
	const likeCommentMutation = useToggleLikeComment();
	const replyToCommentMutation = useReplyToComment();

	const thread = $derived(threadQuery.data?.data);
	const isLoadingThread = $derived(threadQuery.isLoading);
	const comments = $derived((commentsQuery.data?.data?.data ?? []) as ThreadComment[]);
	const commentPageInfo = $derived(commentsQuery.data?.data?.pageInfo);
	const isLoadingComments = $derived(commentsQuery.isLoading);

	let replyText = $state('');
	let isReplying = $state(false);
	// Per-comment reply state: commentId -> { open, text }
	let commentReplies = $state<Record<number, { open: boolean; text: string }>>({});

	function timeAgo(ts?: number): string {
		if (!ts) return '';
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		if (diff < 2592000) return `${Math.floor(diff / 86400)}d ago`;
		return new Date(ts * 1000).toLocaleDateString();
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

	async function toggleLikeThread() {
		try {
			await likeThreadMutation.mutateAsync({ id: threadId });
		} catch {
			toast.error('Failed to like thread');
		}
	}

	async function toggleLikeComment(commentId: number) {
		try {
			await likeCommentMutation.mutateAsync({ id: commentId, threadId });
		} catch {
			toast.error('Failed to like comment');
		}
	}

	function openCommentReply(commentId: number) {
		commentReplies[commentId] = { open: true, text: commentReplies[commentId]?.text ?? '' };
	}

	function closeCommentReply(commentId: number) {
		commentReplies[commentId] = { open: false, text: '' };
	}

	async function submitCommentReply(commentId: number) {
		const state = commentReplies[commentId];
		if (!state?.text.trim()) return;
		try {
			await replyToCommentMutation.mutateAsync({
				threadId,
				parentCommentId: commentId,
				comment: state.text,
			});
			closeCommentReply(commentId);
			toast.success('Reply posted');
		} catch {
			toast.error('Failed to post reply');
		}
	}
</script>

<svelte:head>
	<title>{thread?.title ?? 'Forum Thread'} — Zafkiel</title>
</svelte:head>

{#snippet CommentBlock(comment: ThreadComment, isChild: boolean)}
	<div class="flex gap-3 {isChild ? 'mt-2 ml-10' : ''}">
		<!-- Avatar -->
		<button class="shrink-0" onclick={() => comment.user?.id && goto(`/user/${comment.user.id}`)}>
			{#if comment.user?.avatar?.medium}
				<CachedImage
					src={comment.user.avatar.medium}
					alt={comment.user.name ?? ''}
					class="h-8 w-8 rounded-full object-cover"
				/>
			{:else}
				<div class="flex h-8 w-8 items-center justify-center rounded-full bg-muted">
					<Icon icon="solar:user-bold" class="size-4" />
				</div>
			{/if}
		</button>

		<!-- Content bubble -->
		<div class="min-w-0 flex-1 rounded-xl border bg-card px-4 py-3">
			<!-- Header -->
			<div class="mb-2 flex items-center gap-2 text-sm">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => comment.user?.id && goto(`/user/${comment.user.id}`)}
				>
					{comment.user?.name ?? 'Unknown'}
				</button>
				<span class="text-xs text-muted-foreground">{timeAgo(comment.createdAt)}</span>
			</div>

			<!-- Body -->
			{#if comment.comment}
				<div class="prose prose-sm dark:prose-invert max-w-none text-sm">
					<MarkdownRenderer body={comment.comment} />
				</div>
			{/if}

			<!-- Actions -->
			<div class="mt-3 flex items-center gap-3 text-xs text-muted-foreground">
				<!-- Like -->
				<button
					class="flex items-center gap-1 transition-colors hover:text-rose-500 {comment.isLiked
						? 'text-rose-500'
						: ''}"
					onclick={() => toggleLikeComment(comment.id!)}
					disabled={likeCommentMutation.isPending}
				>
					<Icon
						icon={comment.isLiked ? 'solar:heart-bold' : 'solar:heart-linear'}
						class="size-3.5"
					/>
					{comment.likeCount ?? 0}
				</button>

				<!-- Reply -->
				{#if !thread?.isLocked}
					<button
						class="flex items-center gap-1 transition-colors hover:text-primary"
						onclick={() => {
							if (commentReplies[comment.id!]?.open) {
								closeCommentReply(comment.id!);
							} else {
								openCommentReply(comment.id!);
							}
						}}
					>
						<Icon icon="solar:chat-square-linear" class="size-3.5" />
						Reply
					</button>
				{/if}
			</div>

			<!-- Inline reply form -->
			{#if commentReplies[comment.id!]?.open}
				<div class="mt-3 flex flex-col gap-2">
					<textarea
						bind:value={commentReplies[comment.id!].text}
						placeholder="Write a reply..."
						rows={3}
						class="w-full resize-y rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary focus:outline-none"
					></textarea>
					<div class="flex gap-2">
						<Button
							size="sm"
							onclick={() => submitCommentReply(comment.id!)}
							disabled={!commentReplies[comment.id!]?.text.trim() ||
								replyToCommentMutation.isPending}
						>
							{#if replyToCommentMutation.isPending}
								<Icon
									icon="solar:refresh-circle-line-duotone"
									class="mr-1.5 size-3.5 animate-spin"
								/>
							{/if}
							Post
						</Button>
						<Button size="sm" variant="ghost" onclick={() => closeCommentReply(comment.id!)}>
							Cancel
						</Button>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<!-- Nested child comments -->
	{#if comment.childComments && comment.childComments.length > 0}
		<div class="mt-1 flex flex-col gap-2">
			{#each comment.childComments as child (child.id)}
				{@render CommentBlock(child, true)}
			{/each}
		</div>
	{/if}
{/snippet}

<div class="container mx-auto max-w-4xl px-4 py-6">
	<!-- Back link -->
	<button
		class="mb-4 flex items-center gap-1 text-sm text-muted-foreground transition-colors hover:text-foreground"
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
		<!-- Thread card -->
		<div class="mb-6 overflow-hidden rounded-xl border bg-card shadow-sm">
			<!-- Categories accent bar -->
			{#if thread.categories && thread.categories.length > 0}
				<div class="flex gap-2 border-b bg-muted/40 px-6 py-2">
					{#each thread.categories as cat}
						<Badge variant="secondary" class="text-xs">{cat.name}</Badge>
					{/each}
				</div>
			{/if}

			<div class="p-6">
				<!-- Title row -->
				<div class="mb-4 flex items-start justify-between gap-3">
					<h1 class="text-xl leading-snug font-bold">{thread.title}</h1>
					<div class="flex shrink-0 items-center gap-1.5">
						{#if thread.isSticky}
							<Badge variant="secondary" class="gap-1">
								<Icon icon="solar:pin-bold" class="size-3" />
								Pinned
							</Badge>
						{/if}
						{#if thread.isLocked}
							<Badge variant="outline" class="gap-1">
								<Icon icon="solar:lock-bold" class="size-3" />
								Locked
							</Badge>
						{/if}
					</div>
				</div>

				<!-- Author info -->
				{#if thread.user}
					<div class="mb-5 flex items-center gap-2 text-sm text-muted-foreground">
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
						<button
							class="font-medium hover:text-foreground hover:underline"
							onclick={() => thread.user?.id && goto(`/user/${thread.user.id}`)}
						>
							{thread.user.name}
						</button>
						<span>·</span>
						<span>{timeAgo(thread.createdAt)}</span>
					</div>
				{/if}

				<!-- Body -->
				{#if thread.body}
					<div class="prose prose-sm dark:prose-invert max-w-none">
						<MarkdownRenderer body={thread.body} />
					</div>
				{/if}

				<!-- Footer action bar -->
				<div
					class="mt-5 flex flex-wrap items-center gap-3 border-t pt-4 text-sm text-muted-foreground"
				>
					<!-- Like thread -->
					<button
						class="flex items-center gap-1.5 rounded-md px-2 py-1 transition-colors hover:bg-muted hover:text-rose-500 {thread.isLiked
							? 'text-rose-500'
							: ''}"
						onclick={toggleLikeThread}
						disabled={likeThreadMutation.isPending}
					>
						<Icon
							icon={thread.isLiked ? 'solar:heart-bold' : 'solar:heart-linear'}
							class="size-4"
						/>
						{thread.likeCount ?? 0}
					</button>

					<!-- Reply count -->
					<span class="flex items-center gap-1.5">
						<Icon icon="solar:chat-square-linear" class="size-4" />
						{thread.replyCount ?? 0}
					</span>

					<!-- View count -->
					<span class="flex items-center gap-1.5">
						<Icon icon="solar:eye-linear" class="size-4" />
						{thread.viewCount ?? 0}
					</span>

					<div class="ml-auto flex items-center gap-2">
						<!-- Subscribe -->
						<Button
							variant={thread.isSubscribed ? 'secondary' : 'outline'}
							size="sm"
							onclick={toggleSubscribe}
							disabled={subscriptionMutation.isPending}
						>
							<Icon
								icon={thread.isSubscribed ? 'solar:bell-bold' : 'solar:bell-linear'}
								class="mr-1.5 size-4"
							/>
							{thread.isSubscribed ? 'Subscribed' : 'Subscribe'}
						</Button>

						<!-- Reply button -->
						{#if !thread.isLocked}
							<Button
								variant="outline"
								size="sm"
								onclick={() => {
									isReplying = !isReplying;
									if (!isReplying) replyText = '';
								}}
							>
								<Icon icon="solar:chat-square-linear" class="mr-1.5 size-4" />
								Reply
							</Button>
						{/if}
					</div>
				</div>

				<!-- Inline reply form -->
				{#if isReplying && !thread.isLocked}
					<div class="mt-4 flex flex-col gap-3 rounded-lg border bg-muted/30 p-4">
						<textarea
							bind:value={replyText}
							placeholder="Write your reply..."
							rows={4}
							class="w-full resize-y rounded-md border bg-background px-3 py-2 text-sm focus:ring-2 focus:ring-primary focus:outline-none"
						></textarea>
						<div class="flex gap-2">
							<Button onclick={handleReply} disabled={!replyText.trim() || replyMutation.isPending}>
								{#if replyMutation.isPending}
									<Icon
										icon="solar:refresh-circle-line-duotone"
										class="mr-1.5 size-4 animate-spin"
									/>
								{/if}
								Post Reply
							</Button>
							<Button
								variant="outline"
								onclick={() => {
									isReplying = false;
									replyText = '';
								}}
							>
								Cancel
							</Button>
						</div>
					</div>
				{/if}
			</div>
		</div>

		<!-- Comments section -->
		<div class="mb-4 flex items-center justify-between">
			<h2 class="text-lg font-semibold">
				Replies
				{#if thread.replyCount}
					<span class="ml-1 text-sm font-normal text-muted-foreground">({thread.replyCount})</span>
				{/if}
			</h2>
		</div>

		{#if isLoadingComments}
			<div class="flex min-h-[200px] items-center justify-center">
				<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
			</div>
		{:else if comments.length === 0}
			<div
				class="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed p-10 text-center"
			>
				<Icon icon="solar:chat-square-bold-duotone" class="h-12 w-12 text-muted-foreground" />
				<p class="text-sm text-muted-foreground">No replies yet. Be the first!</p>
			</div>
		{:else}
			<div class="flex flex-col gap-4">
				{#each comments as comment (comment.id)}
					{@render CommentBlock(comment, false)}
				{/each}
			</div>
		{/if}

		<!-- Pagination -->
		{#if commentPageInfo && (commentPage > 1 || commentPageInfo.hasNextPage)}
			<div class="mt-6 flex items-center justify-center gap-3">
				<Button variant="outline" disabled={commentPage <= 1} onclick={() => (commentPage -= 1)}>
					<Icon icon="solar:arrow-left-linear" class="size-4" />
					Previous
				</Button>
				<span class="text-sm text-muted-foreground">Page {commentPage}</span>
				<Button
					variant="outline"
					disabled={!commentPageInfo.hasNextPage}
					onclick={() => (commentPage += 1)}
				>
					Next
					<Icon icon="solar:arrow-right-linear" class="size-4" />
				</Button>
			</div>
		{/if}
	{:else}
		<div class="flex min-h-[300px] flex-col items-center justify-center gap-4 text-center">
			<h2 class="text-2xl font-bold">Thread Not Found</h2>
			<Button variant="outline" onclick={() => goto('/forum')}>Back to Forum</Button>
		</div>
	{/if}
</div>
