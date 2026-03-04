<script lang="ts">
	import { DropdownMenu } from 'bits-ui';
	import { fly } from 'svelte/transition';
	import Icon from '@iconify/svelte';
	import { currentUser, isAuthenticated } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { useConfigState } from '$lib/stores/config.svelte';

	const blurEffectsEnabled = $derived(useConfigState().blurEffects);

	type MenuItem = {
		icon: string;
		label: string;
		action: () => void;
		danger?: boolean;
	};

	type SeparatorItem = {
		type: 'separator';
	};

	type MenuItemType = MenuItem | SeparatorItem;

	let open = $state(false);

	// Menu items for authenticated users — reactive so username is always current
	const authenticatedMenuItems = $derived<MenuItemType[]>([
		{ icon: 'solar:user-bold', label: 'Profile', action: () => goto('/profile') },
		{
			icon: 'solar:play-circle-bold',
			label: 'Anime List',
			action: () => goto(`/user/${$currentUser?.name}/animelist`)
		},
		{
			icon: 'solar:book-2-bold',
			label: 'Manga List',
			action: () => goto(`/user/${$currentUser?.name}/mangalist`)
		},
		{ icon: 'solar:users-group-rounded-bold', label: 'Social', action: () => goto('/social') },
		{ icon: 'solar:bell-bold', label: 'Notifications', action: () => goto('/notifications') },
		{ icon: 'solar:chat-square-bold', label: 'Forum', action: () => goto('/forum') },
		{ type: 'separator' as const },
		{ icon: 'solar:settings-bold', label: 'Settings', action: () => goto('/settings') },
		{ type: 'separator' as const },
		{ icon: 'solar:logout-2-bold', label: 'Logout', action: () => goto('/logout'), danger: true },
	]);

	// Menu items for guest users
	const guestMenuItems: MenuItemType[] = [
		{ icon: 'solar:login-2-bold', label: 'Login', action: () => goto('/auth/login') },
		{ icon: 'solar:settings-bold', label: 'Settings', action: () => goto('/settings') },
	];

	// Get menu items based on auth status
	const menuItems = $derived($isAuthenticated ? authenticatedMenuItems : guestMenuItems);

	// Get avatar URL from UserAvatar object
	const getAvatarUrl = (avatar: any) => {
		if (!avatar) return null;
		return avatar.large || avatar.medium || null;
	};
</script>

<DropdownMenu.Root bind:open>
	<DropdownMenu.Trigger
		class="flex h-8 items-center gap-2 rounded-lg px-2 transition-colors hover:bg-foreground/5 active:bg-foreground/10"
		data-tauri-drag-region="false"
	>
		<!-- Avatar -->
		<div
			class="flex h-7 w-7 items-center justify-center overflow-hidden rounded-full bg-linear-to-br from-primary to-primary/70 shadow-md"
		>
			{#if $isAuthenticated && $currentUser?.avatar}
				{@const avatarUrl = getAvatarUrl($currentUser.avatar)}
				{#if avatarUrl}
					<img src={avatarUrl} alt={$currentUser.name} class="h-full w-full object-cover" />
				{:else}
					<Icon icon="solar:user-bold" class="h-4 w-4 text-primary-foreground" />
				{/if}
			{:else}
				<Icon icon="solar:user-bold" class="h-4 w-4 text-primary-foreground" />
			{/if}
		</div>

		<!-- Username -->
		<span class="max-w-24 truncate text-sm font-medium text-foreground/90">
			{$isAuthenticated && $currentUser?.name ? $currentUser.name : 'Guest'}
		</span>

		<!-- Chevron -->
		<Icon
			icon="solar:alt-arrow-down-linear"
			class="h-3 w-3 text-foreground/60 transition-transform {open ? 'rotate-180' : ''}"
		/>
	</DropdownMenu.Trigger>

	<DropdownMenu.Content
		class="z-50 min-w-48 rounded-xl border border-border/50 p-1 shadow-xl {blurEffectsEnabled ? 'bg-popover/85' : 'bg-popover'}"
		style={blurEffectsEnabled ? 'backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px);' : ''}
		sideOffset={8}
		align="end"
	>
		<!-- User Info Header -->
		<div class="mb-2 border-b border-border/50 px-3 py-2">
			<p class="text-sm font-semibold text-foreground">
				{$isAuthenticated && $currentUser?.name ? $currentUser.name : 'Guest User'}
			</p>
			{#if $isAuthenticated && $currentUser}
				<a
					href={'https://anilist.co/user/' + $currentUser.name}
					target="_blank"
					rel="noopener noreferrer"
					class="block truncate text-xs text-muted-foreground hover:text-primary"
				>
					View on AniList
				</a>
			{:else}
				<p class="text-xs text-muted-foreground">Not logged in</p>
			{/if}
		</div>

		<!-- Menu Items -->
		{#each menuItems as item}
			{#if 'type' in item && item.type === 'separator'}
				<DropdownMenu.Separator class="my-1 h-px bg-border/50" />
			{:else if 'icon' in item}
				{@const menuItem = item as MenuItem}
				<DropdownMenu.Item
					class="flex cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors hover:bg-foreground/5 active:bg-foreground/10 {menuItem.danger
						? 'text-destructive hover:bg-destructive/10'
						: 'text-foreground'}"
					onSelect={menuItem.action}
				>
					<Icon icon={menuItem.icon} class="h-4 w-4" />
					<span>{menuItem.label}</span>
				</DropdownMenu.Item>
			{/if}
		{/each}
	</DropdownMenu.Content>
</DropdownMenu.Root>
