<script lang="ts">
	import type {
		ActivityUnion,
		ListActivity,
		TextActivity,
		MessageActivity,
	} from '$lib/types/anilist';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';
	import { goto } from '$app/navigation';

	let { activity }: { activity: ActivityUnion } = $props();

	function variant() {
		if (activity.__typename === 'ListActivity')
			return { kind: 'list' as const, data: activity as ListActivity };
		if (activity.__typename === 'TextActivity')
			return { kind: 'text' as const, data: activity as TextActivity };
		return { kind: 'message' as const, data: activity as MessageActivity };
	}

	function timeAgo(ts: number): string {
		const diff = Math.floor(Date.now() / 1000) - ts;
		if (diff < 60) return `${diff}s ago`;
		if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
		if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
		if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`;
		return `${Math.floor(diff / 604800)}w ago`;
	}

	/** Colour-coded status badge for list activities */
	function statusColor(status?: string | null): string {
		const s = (status ?? '').toLowerCase();
		if (s.includes('watch') || s.includes('read')) return 'text-blue-500 bg-blue-500/10';
		if (s.includes('complet')) return 'text-green-500 bg-green-500/10';
		if (s.includes('drop')) return 'text-red-500 bg-red-500/10';
		if (s.includes('plan')) return 'text-yellow-500 bg-yellow-500/10';
		if (s.includes('pause') || s.includes('hold')) return 'text-orange-500 bg-orange-500/10';
		return 'text-muted-foreground bg-muted';
	}
</script>

{#if variant().kind === 'list'}
	{@const act = variant().data as ListActivity}
	<article
		class="group flex gap-3 rounded-xl border bg-card p-4 transition-colors hover:bg-card/80 hover:shadow-sm"
	>
		<!-- User avatar -->
		<button
			class="shrink-0 self-start"
			onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
			aria-label="View {act.user?.name ?? 'user'}'s profile"
		>
			{#if act.user?.avatar?.medium}
				<CachedImage
					src={act.user.avatar.medium}
					alt={act.user.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				/>
			{:else}
				<div
					class="flex h-10 w-10 items-center justify-center rounded-full bg-muted ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				>
					<Icon icon="solar:user-bold" class="h-5 w-5 text-muted-foreground" />
				</div>
			{/if}
		</button>

		<div class="min-w-0 flex-1">
			<!-- Activity text -->
			<p class="mb-2.5 flex flex-wrap items-baseline gap-1.5 text-sm leading-snug">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
				>
					{act.user?.name ?? 'Unknown'}
				</button>
				{#if act.status}
					<span
						class="inline-flex items-center rounded px-1.5 py-0.5 text-xs font-medium {statusColor(act.status)}"
					>
						{act.status}
					</span>
				{/if}
				{#if act.progress}
					<span class="text-muted-foreground">ep {act.progress} of</span>
				{:else}
					<span class="text-muted-foreground">of</span>
				{/if}
				<button
					class="font-medium hover:text-primary hover:underline"
					onclick={() =>
						act.media?.id &&
						goto(`/${(act.media?.type ?? 'anime').toLowerCase()}/${act.media.id}`)}
				>
					{act.media?.title?.userPreferred ?? 'Unknown'}
				</button>
			</p>

			<!-- Cover + footer row -->
			<div class="flex items-end gap-3">
				{#if act.media?.coverImage?.medium}
					<button
						class="shrink-0"
						onclick={() =>
							act.media?.id &&
							goto(`/${(act.media?.type ?? 'anime').toLowerCase()}/${act.media.id}`)}
						aria-label="View {act.media?.title?.userPreferred}"
					>
						<CachedImage
							src={act.media.coverImage.medium}
							alt={act.media.title?.userPreferred ?? ''}
							class="h-16 w-11 rounded-md object-cover shadow-sm transition-transform group-hover:scale-105"
						/>
					</button>
				{/if}

				<div class="flex flex-1 flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
					<span class="flex items-center gap-1">
						<Icon icon="solar:heart-linear" class="size-3.5" />
						{act.likeCount ?? 0}
					</span>
					<span class="flex items-center gap-1">
						<Icon icon="solar:chat-square-linear" class="size-3.5" />
						{act.replyCount ?? 0}
					</span>
					<span class="ml-auto">{timeAgo(act.createdAt)}</span>
				</div>
			</div>
		</div>
	</article>
{:else if variant().kind === 'text'}
	{@const act = variant().data as TextActivity}
	<article
		class="group flex gap-3 rounded-xl border bg-card p-4 transition-colors hover:bg-card/80 hover:shadow-sm"
	>
		<button
			class="shrink-0 self-start"
			onclick={() => act.user?.id && goto(`/user/${act.user.id}`)}
			aria-label="View {act.user?.name ?? 'user'}'s profile"
		>
			{#if act.user?.avatar?.medium}
				<CachedImage
					src={act.user.avatar.medium}
					alt={act.user.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				/>
			{:else}
				<div
					class="flex h-10 w-10 items-center justify-center rounded-full bg-muted ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				>
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
				<span class="rounded bg-green-500/10 px-1.5 py-0.5 text-xs text-green-500">
					Status
				</span>
				<span class="ml-auto text-xs text-muted-foreground">{timeAgo(act.createdAt)}</span>
			</div>

			{#if act.text}
				<div class="max-h-[1000px] overflow-y-auto">
					<MarkdownRenderer body={act.text} class="text-sm" />
				</div>
			{/if}

			<div class="mt-3 flex items-center gap-3 text-xs text-muted-foreground">
				<span class="flex items-center gap-1">
					<Icon icon="solar:heart-linear" class="size-3.5" />
					{act.likeCount ?? 0}
				</span>
				<span class="flex items-center gap-1">
					<Icon icon="solar:chat-square-linear" class="size-3.5" />
					{act.replyCount ?? 0}
				</span>
			</div>
		</div>
	</article>
{:else}
	{@const act = variant().data as MessageActivity}
	<article
		class="group flex gap-3 rounded-xl border bg-card p-4 transition-colors hover:bg-card/80 hover:shadow-sm"
	>
		<button
			class="shrink-0 self-start"
			onclick={() => act.messenger?.id && goto(`/user/${act.messenger.id}`)}
			aria-label="View {act.messenger?.name ?? 'user'}'s profile"
		>
			{#if act.messenger?.avatar?.medium}
				<CachedImage
					src={act.messenger.avatar.medium}
					alt={act.messenger.name ?? 'User'}
					class="h-10 w-10 rounded-full object-cover ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				/>
			{:else}
				<div
					class="flex h-10 w-10 items-center justify-center rounded-full bg-muted ring-2 ring-transparent transition-all group-hover:ring-primary/30"
				>
					<Icon icon="solar:user-bold" class="h-5 w-5 text-muted-foreground" />
				</div>
			{/if}
		</button>

		<div class="min-w-0 flex-1">
			<div class="mb-2 flex flex-wrap items-center gap-1.5 text-sm">
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.messenger?.id && goto(`/user/${act.messenger.id}`)}
				>
					{act.messenger?.name ?? 'Unknown'}
				</button>
				<Icon icon="solar:arrow-right-linear" class="size-3 text-muted-foreground" />
				<button
					class="font-semibold hover:text-primary hover:underline"
					onclick={() => act.recipient?.id && goto(`/user/${act.recipient.id}`)}
				>
					{act.recipient?.name ?? 'Unknown'}
				</button>
				<span class="rounded bg-purple-500/10 px-1.5 py-0.5 text-xs text-purple-500">
					Message
				</span>
				<span class="ml-auto text-xs text-muted-foreground">{timeAgo(act.createdAt)}</span>
			</div>

			{#if act.message}
				<div class="max-h-[1000px] overflow-y-auto">
					<MarkdownRenderer body={act.message} class="text-sm" />
				</div>
			{/if}

			<div class="mt-3 flex items-center gap-3 text-xs text-muted-foreground">
				<span class="flex items-center gap-1">
					<Icon icon="solar:heart-linear" class="size-3.5" />
					{act.likeCount ?? 0}
				</span>
			</div>
		</div>
	</article>
{/if}

