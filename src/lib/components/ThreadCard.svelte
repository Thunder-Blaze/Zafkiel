<script lang="ts">
	import type { Thread } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { Badge } from '$lib/components/ui/badge';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	let { thread }: { thread: Thread } = $props();

	function timeAgo(ts?: number): string {
		if (!ts) return '';
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}
</script>

<button
	class="flex w-full flex-col gap-2 rounded-lg border bg-card p-4 text-left transition-colors hover:bg-card/80"
	onclick={() => goto(`/forum/${thread.id}`)}
>
	<div class="flex items-start gap-3">
		<!-- User avatar -->
		{#if thread.user?.avatar?.medium}
			<span
				role="button"
				tabindex="0"
				class="mt-0.5 shrink-0 cursor-pointer"
				onclick={(e) => {
					e.stopPropagation();
					thread.user?.id && goto(`/user/${thread.user.id}`);
				}}
				onkeydown={(e) => {
					if (e.key === 'Enter' || e.key === ' ') {
						e.stopPropagation();
						thread.user?.id && goto(`/user/${thread.user.id}`);
					}
				}}
			>
				<CachedImage
					src={thread.user.avatar.medium}
					alt={thread.user.name ?? ''}
					class="h-8 w-8 rounded-full object-cover"
				/>
			</span>
		{:else}
			<div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-muted">
				<Icon icon="solar:user-bold" class="h-4 w-4 text-muted-foreground" />
			</div>
		{/if}

		<div class="min-w-0 flex-1">
			<!-- Title -->
			<h3 class="mb-1 line-clamp-2 leading-snug font-semibold text-foreground">
				{thread.title ?? 'Untitled Thread'}
			</h3>

			<!-- Meta row -->
			<div class="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
				<span
					role="button"
					tabindex="0"
					class="cursor-pointer hover:text-primary hover:underline"
					onclick={(e) => {
						e.stopPropagation();
						thread.user?.id && goto(`/user/${thread.user.id}`);
					}}
					onkeydown={(e) => {
						if (e.key === 'Enter' || e.key === ' ') {
							e.stopPropagation();
							thread.user?.id && goto(`/user/${thread.user.id}`);
						}
					}}
				>
					{thread.user?.name ?? 'Unknown'}
				</span>
				{#if thread.createdAt}
					<span>· {timeAgo(thread.createdAt)}</span>
				{/if}
			</div>
		</div>

		<!-- Badges -->
		<div class="flex shrink-0 flex-col items-end gap-1">
			{#if thread.isSticky}
				<Badge variant="secondary" class="text-xs">
					<Icon icon="solar:pin-bold" class="mr-1 size-3" />
					Pinned
				</Badge>
			{/if}
			{#if thread.isLocked}
				<Badge variant="outline" class="text-xs">
					<Icon icon="solar:lock-bold" class="mr-1 size-3" />
					Locked
				</Badge>
			{/if}
		</div>
	</div>

	<!-- Categories -->
	{#if thread.categories && thread.categories.length > 0}
		<div class="flex flex-wrap gap-1">
			{#each thread.categories as cat}
				<Badge variant="outline" class="text-xs">{cat.name}</Badge>
			{/each}
		</div>
	{/if}

	<!-- Stats footer -->
	<div class="flex items-center gap-4 text-xs text-muted-foreground">
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
		{#if thread.repliedAt}
			<span class="ml-auto">
				last reply {timeAgo(thread.repliedAt)}
			</span>
		{/if}
	</div>
</button>
