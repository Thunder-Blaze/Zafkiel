<script lang="ts">
	import { browser } from '$app/environment';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import Icon from '@iconify/svelte';
	import { authStore, currentUser, isAuthenticated } from '$lib/stores/auth';
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
	let triggerEl = $state<HTMLButtonElement | null>(null);
	let menuEl = $state<HTMLDivElement | null>(null);
	let pos = $state({ top: 0, right: 0 });

	// Track window width — items shown in titlebar at >= 1366px are hidden here
	let windowWidth = $state(browser ? window.innerWidth : 1920);
	$effect(() => {
		if (!browser) return;
		const onResize = () => {
			windowWidth = window.innerWidth;
		};
		window.addEventListener('resize', onResize);
		return () => window.removeEventListener('resize', onResize);
	});

	const narrowMode = $derived(windowWidth < 1366);

	// Portal action — moves element to document.body so backdrop-filter escapes TitleBar stacking context
	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return {
			destroy() {
				node.parentNode?.removeChild(node);
			},
		};
	}

	function updatePosition() {
		if (!triggerEl) return;
		const rect = triggerEl.getBoundingClientRect();
		pos = { top: rect.bottom + 8, right: window.innerWidth - rect.right };
	}

	function toggle() {
		if (!open) updatePosition();
		open = !open;
	}

	function close() {
		open = false;
	}

	$effect(() => {
		if (!open || !browser) return;

		const handleClick = (e: MouseEvent) => {
			if (
				menuEl &&
				!menuEl.contains(e.target as Node) &&
				triggerEl &&
				!triggerEl.contains(e.target as Node)
			) {
				close();
			}
		};
		const handleKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') close();
		};

		document.addEventListener('mousedown', handleClick);
		document.addEventListener('keydown', handleKey);
		return () => {
			document.removeEventListener('mousedown', handleClick);
			document.removeEventListener('keydown', handleKey);
		};
	});

	// Menu items for authenticated users — reactive so username and width are always current
	const authenticatedMenuItems = $derived<MenuItemType[]>([
		{
			icon: 'solar:user-bold',
			label: 'Profile',
			action: () => goto(`/user/${$currentUser?.name}`),
		},
		{
			icon: 'solar:play-circle-bold',
			label: 'Anime List',
			action: () => goto(`/user/${$currentUser?.name}/animelist`),
		},
		{
			icon: 'solar:book-2-bold',
			label: 'Manga List',
			action: () => goto(`/user/${$currentUser?.name}/mangalist`),
		},
		...(narrowMode
			? [
					{
						icon: 'solar:history-2-bold',
						label: 'Activity',
						action: () => goto('/activity'),
					} as MenuItem,
					{
						icon: 'solar:chat-square-bold',
						label: 'Forum',
						action: () => goto('/forum'),
					} as MenuItem,
				]
			: []),
		{ type: 'separator' as const },
		...(narrowMode
			? [
					{
						icon: 'solar:settings-bold',
						label: 'Settings',
						action: () => goto('/settings'),
					} as MenuItem,
					{ type: 'separator' as const },
				]
			: []),
		{ icon: 'solar:logout-2-bold', label: 'Logout', action: async () => { await authStore.logout(); goto('/'); }, danger: true },
	]);

	// Menu items for guest users
	const guestMenuItems = $derived<MenuItemType[]>([
		{ icon: 'solar:login-2-bold', label: 'Login', action: () => goto('/auth/login') },
		...(narrowMode
			? [
					{
						icon: 'solar:settings-bold',
						label: 'Settings',
						action: () => goto('/settings'),
					} as MenuItem,
				]
			: []),
	]);

	// Get menu items based on auth status
	const menuItems = $derived($isAuthenticated ? authenticatedMenuItems : guestMenuItems);

	// Get avatar URL from UserAvatar object
	const getAvatarUrl = (avatar: any) => {
		if (!avatar) return null;
		return avatar.large || avatar.medium || null;
	};
</script>

<!-- Trigger -->
<button
	bind:this={triggerEl}
	type="button"
	onclick={toggle}
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
</button>

<!-- Dropdown panel — portaled to document.body so backdrop-filter escapes TitleBar stacking context -->
{#if open && browser}
	<div
		use:portal
		bind:this={menuEl}
		class="fixed z-999999"
		style="top: {pos.top}px; right: {pos.right}px;"
		transition:scale={{ duration: 150, easing: quintOut, start: 0.95, opacity: 0 }}
	>
		<div
			class="min-w-48 rounded-xl border border-border/50 p-1 shadow-xl {blurEffectsEnabled
				? 'bg-popover/85 backdrop-blur-xl'
				: 'bg-popover'}"
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
					<div class="my-1 h-px bg-border/50"></div>
				{:else if 'icon' in item}
					{@const menuItem = item as MenuItem}
					<button
						type="button"
						onclick={() => {
							close();
							menuItem.action();
						}}
						class="flex w-full cursor-pointer items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors hover:bg-foreground/5 active:bg-foreground/10 {menuItem.danger
							? 'text-destructive hover:bg-destructive/10'
							: 'text-foreground'}"
					>
						<Icon icon={menuItem.icon} class="h-4 w-4" />
						<span>{menuItem.label}</span>
					</button>
				{/if}
			{/each}
		</div>
	</div>
{/if}
