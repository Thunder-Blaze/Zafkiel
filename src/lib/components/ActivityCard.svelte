<script lang="ts">
	import type { ActivityUnion, ListActivity, TextActivity, MessageActivity } from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	let { activity }: { activity: ActivityUnion } = $props();

	function variant() {
		if (activity.__typename === 'ListActivity') return { kind: 'list' as const, data: activity as ListActivity };
		if (activity.__typename === 'TextActivity') return { kind: 'text' as const, data: activity as TextActivity };
		return { kind: 'message' as const, data: activity as MessageActivity };
	}

	function timeAgo(ts: number): string {
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		return `${Math.floor(diff / 86400)}d ago`;
	}
</script>

{#if variant().kind === 'list'}
	{@const act = variant().data as ListActivity}
	<div class="flex gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-card/80">
		<!-- User avatar -->
		<button
			class="shrink-0"
			onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
		>
			{#if act.user?.avatar?.medium}
				<CachedImage
					src={act.user.avatar.medium}
					alt={act.user.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover"
				/>
			{:else}
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
					<Icon icon="solar:user-bold" class="h-5 w-5 text-muted-foreground" />
				</div>
			{/if}
		</button>

		<div class="min-w-0 flex-1">
			<!-- Header -->
			<div class="mb-1 flex flex-wrap items-center gap-1 text-sm">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
				>
					{act.user?.name ?? 'Unknown'}
				</button>
				<span class="text-muted-foreground">
					{act.status}{act.progress ? ` ${act.progress}` : ''} of
				</span>
				<button
					class="font-medium hover:text-primary hover:underline"
					onclick={() => act.media?.id && goto(`/${(act.media.type ?? 'anime').toLowerCase()}/${act.media.id}`)}
				>
					{act.media?.title?.userPreferred ?? 'Unknown'}
				</button>
			</div>

			<!-- Media thumbnail + footer -->
			<div class="flex items-center gap-3">
				{#if act.media?.coverImage?.medium}
					<button onclick={() => act.media?.id && goto(`/${(act.media.type ?? 'anime').toLowerCase()}/${act.media.id}`)}>
						<CachedImage
							src={act.media.coverImage.medium}
							alt={act.media.title?.userPreferred ?? ''}
							class="h-14 w-10 rounded object-cover"
						/>
					</button>
				{/if}
				<div class="flex items-center gap-2 text-xs text-muted-foreground">
					<Icon icon="solar:heart-linear" class="size-3.5" />
					<span>{act.likeCount ?? 0}</span>
					<Icon icon="solar:chat-square-linear" class="size-3.5" />
					<span>{act.replyCount ?? 0}</span>
					<span class="ml-auto">{timeAgo(act.createdAt)}</span>
				</div>
			</div>
		</div>
	</div>

{:else if variant().kind === 'text'}
	{@const act = variant().data as TextActivity}
	<div class="flex gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-card/80">
		<button
			class="shrink-0"
			onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
		>
			{#if act.user?.avatar?.medium}
				<CachedImage
					src={act.user.avatar.medium}
					alt={act.user.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover"
				/>
			{:else}
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
					<Icon icon="solar:user-bold" class="h-5 w-5 text-muted-foreground" />
				</div>
			{/if}
		</button>

		<div class="min-w-0 flex-1">
			<div class="mb-2 flex items-center gap-2 text-sm">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
				>
					{act.user?.name ?? 'Unknown'}
				</button>
				<span class="text-xs text-muted-foreground">posted a status</span>
			</div>
			{#if act.text}
				<p class="text-sm text-foreground/90 line-clamp-4">{@html act.text}</p>
			{/if}
			<div class="mt-2 flex items-center gap-2 text-xs text-muted-foreground">
				<Icon icon="solar:heart-linear" class="size-3.5" />
				<span>{act.likeCount ?? 0}</span>
				<Icon icon="solar:chat-square-linear" class="size-3.5" />
				<span>{act.replyCount ?? 0}</span>
				<span class="ml-auto">{timeAgo(act.createdAt)}</span>
			</div>
		</div>
	</div>

{:else}
	{@const act = variant().data as MessageActivity}
	<div class="flex gap-3 rounded-lg border bg-card p-4 transition-colors hover:bg-card/80">
		<button
			class="shrink-0"
			onclick={() => act.messenger?.id && goto(`/user/${act.messenger.id}`)}
		>
			{#if act.messenger?.avatar?.medium}
				<CachedImage
					src={act.messenger.avatar.medium}
					alt={act.messenger.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover"
				/>
			{:else}
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
					<Icon icon="solar:user-bold" class="h-5 w-5 text-muted-foreground" />
				</div>
			{/if}
		</button>

		<div class="min-w-0 flex-1">
			<div class="mb-2 flex items-center gap-1 text-sm">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.messenger?.id && goto(`/user/${act.messenger.id}`)}
				>
					{act.messenger?.name ?? 'Unknown'}
				</button>
				<span class="text-muted-foreground">→</span>
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.recipient?.id && goto(`/user/${act.recipient.id}`)}
				>
					{act.recipient?.name ?? 'Unknown'}
				</button>
			</div>
			{#if act.message}
				<p class="text-sm text-foreground/90 line-clamp-3">{@html act.message}</p>
			{/if}
			<div class="mt-2 flex items-center gap-2 text-xs text-muted-foreground">
				<Icon icon="solar:heart-linear" class="size-3.5" />
				<span>{act.likeCount ?? 0}</span>
				<span class="ml-auto">{timeAgo(act.createdAt)}</span>
			</div>
		</div>
	</div>
{/if}
