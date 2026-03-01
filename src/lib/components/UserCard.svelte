<script lang="ts">
	/**
	 * Compact user card: avatar, name, badges, optional follow button.
	 * Clicking the card navigates to /user/[id].
	 */
	import { goto } from '$app/navigation';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { Button } from '$lib/components/ui/button';
	import type { User } from '$lib/types/anilist';

	let {
		user,
		showFollowButton = false,
		isFollowing = false,
		onFollowToggle,
		class: className = '',
	}: {
		user: User;
		showFollowButton?: boolean;
		isFollowing?: boolean;
		onFollowToggle?: (userId: number, currentlyFollowing: boolean) => void;
		class?: string;
	} = $props();

	const avatar = $derived(user.avatar?.large ?? user.avatar?.medium ?? null);
	const href = $derived(`/user/${user.id}`);

	const abbreviateCount = (n?: number) =>
		n == null ? null : n >= 1000 ? `${(n / 1000).toFixed(1)}k` : String(n);

	const followersLabel = $derived(abbreviateCount(user.statistics?.anime?.count));
</script>

<div
	class="group flex items-center gap-3 rounded-lg p-2 transition-colors hover:bg-muted/50 {className}"
>
	<!-- Avatar -->
	<button
		class="shrink-0 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
		onclick={() => goto(href)}
	>
		{#if avatar}
			<CachedImage
				src={avatar}
				alt={user.name}
				class="size-10 rounded-full object-cover ring-2 ring-border group-hover:ring-primary/50 transition-all"
			/>
		{:else}
			<div
				class="flex size-10 items-center justify-center rounded-full bg-muted text-muted-foreground text-sm font-bold uppercase"
			>
				{user.name?.slice(0, 1) ?? '?'}
			</div>
		{/if}
	</button>

	<!-- Info -->
	<button
		class="min-w-0 flex-1 cursor-pointer text-left focus-visible:outline-none"
		onclick={() => goto(href)}
	>
		<p
			class="truncate text-sm font-semibold leading-tight group-hover:text-primary group-hover:underline"
		>
			{user.name}
		</p>
		{#if user.donatorTier && user.donatorTier > 0}
			<p class="text-[10px] font-medium text-yellow-500">
				{user.donatorBadge ?? 'Supporter'}
			</p>
		{:else if user.moderatorRoles && user.moderatorRoles.length > 0}
			<p class="text-[10px] font-medium text-blue-400">Moderator</p>
		{:else if user.about}
			<p class="truncate text-xs text-muted-foreground">{user.about}</p>
		{/if}
	</button>

	<!-- Follow button -->
	{#if showFollowButton}
		<Button
			variant={isFollowing ? 'outline' : 'default'}
			size="sm"
			class="shrink-0 text-xs"
			onclick={() => onFollowToggle?.(user.id, isFollowing)}
		>
			{isFollowing ? 'Following' : 'Follow'}
		</Button>
	{/if}
</div>
