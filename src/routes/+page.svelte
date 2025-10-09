<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { authStore, isAuthenticated, currentUser, authLoading } from '$lib/stores/auth';
	import { themeStore } from '$lib/stores/theme.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { goto } from '$app/navigation';

	let currentTime = $state(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }));
	let currentDate = $state(new Date());

	$effect(() => {
		const interval = setInterval(() => {
			const now = new Date();
			currentTime = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
			currentDate = now;
		}, 1000);

		return () => clearInterval(interval);
	});

	// Stats
	const stats = [
		{ label: 'Watching', value: 12, icon: 'solar:play-bold', color: 'text-primary' },
		{ label: 'Completed', value: 48, icon: 'solar:check-circle-bold', color: 'text-green-500' },
		{ label: 'Plan to Watch', value: 23, icon: 'solar:bookmark-bold', color: 'text-blue-500' },
		{ label: 'Episodes', value: 1247, icon: 'solar:video-library-bold', color: 'text-purple-500' },
	];

	// Quick actions
	const quickActions = [
		{ label: 'Browse', icon: 'solar:magnifer-bold', path: '/anime' },
		{ label: 'My List', icon: 'solar:clipboard-list-bold', path: '/list' },
		{ label: 'Trending', icon: 'solar:fire-bold', path: '/trending' },
		{ label: 'Settings', icon: 'solar:settings-bold', path: '/settings' },
	];

	const getDayOfWeek = (date: Date) => {
		return date.toLocaleDateString('en-US', { weekday: 'short' });
	};

	const getFormattedDate = (date: Date) => {
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	};

	// Get avatar URL from UserAvatar object
	const getAvatarUrl = (avatar: any) => {
		if (!avatar) return null;
		return avatar.large || avatar.medium || null;
	};
</script>

{#if $authLoading}
	<Loader text="Loading your dashboard..." />
{:else}
	<div class="h-[calc(100vh-3rem)] overflow-hidden relative bg-black/40">
		<!-- Background Theme Image -->
		<div class="absolute flex items-center justify-center inset-0">
			<img
				src={themeStore.currentThemePath ? `${themeStore.currentThemePath}/theme.png` : '/images/fallback-theme.png'}
				alt="Background Theme"
				class="w-auto h-full object-fit"
			/>
			<!-- Lighter Gradient Overlays for readability -->
			<div class="absolute inset-0 -z-10 bg-gradient-to-br from-background/70 via-background/40 to-background/60"></div>
		</div>

		<!-- Content Layer -->
		<div class="relative h-full w-full p-6">
			<!-- TOP LEFT: Welcome + Stats -->
			<div class="absolute top-6 left-6 w-[30%] space-y-3">
				<!-- Welcome Card -->
				<Card class="p-4 border-border/50 bg-card/60 backdrop-blur-md">
					<h1 class="text-xl font-bold tracking-tight">
						Welcome back! 👋
					</h1>
					<p class="text-xs text-muted-foreground">
						{$isAuthenticated && $currentUser?.name ? $currentUser.name : 'Guest'}
					</p>
				</Card>

				<!-- Stats Grid -->
				<div class="grid grid-cols-2 gap-2">
					{#each stats as stat}
						<Card class="p-3 hover:shadow-lg transition-shadow cursor-pointer border-border/50 bg-card/60 backdrop-blur-md">
							<div class="flex items-center gap-2">
								<div class="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10">
									<Icon icon={stat.icon} class="h-4 w-4 {stat.color}" />
								</div>
								<div>
									<p class="text-xl font-bold">{stat.value}</p>
									<p class="text-[10px] text-muted-foreground">{stat.label}</p>
								</div>
							</div>
						</Card>
					{/each}
				</div>
			</div>

			<!-- TOP RIGHT: Calendar + Profile -->
			<div class="absolute top-6 right-6 w-[30%] space-y-3">
				<!-- Calendar Widget -->
				<Card class="p-5 border-border/50 bg-gradient-to-br from-primary/20 to-primary/30 backdrop-blur-md">
					<div class="text-center">
						<p class="text-xs font-medium text-muted-foreground mb-1">{getDayOfWeek(currentDate).toUpperCase()}</p>
						<p class="text-5xl font-bold tracking-tight mb-1">{currentDate.getDate()}</p>
						<p class="text-sm text-muted-foreground mb-2">
							{currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
						</p>
						<div class="pt-2 border-t border-border/50">
							<p class="text-lg font-mono font-medium">{currentTime}</p>
						</div>
					</div>
				</Card>

				<!-- Profile Card -->
				{#if $isAuthenticated && $currentUser}
					<Card class="p-4 border-border/50 bg-card/60 backdrop-blur-md">
						<div class="flex items-center gap-3">
							<div class="flex h-14 w-14 items-center justify-center overflow-hidden rounded-full bg-gradient-to-br from-primary to-primary/70 shadow-lg shrink-0">
								{#if $currentUser.avatar}
									{@const avatarUrl = getAvatarUrl($currentUser.avatar)}
									{#if avatarUrl}
										<img src={avatarUrl} alt={$currentUser.name} class="h-full w-full object-cover" />
									{:else}
										<Icon icon="solar:user-bold" class="h-7 w-7 text-primary-foreground" />
									{/if}
								{:else}
									<Icon icon="solar:user-bold" class="h-7 w-7 text-primary-foreground" />
								{/if}
							</div>
							<div class="flex-1 min-w-0">
								<h3 class="font-semibold text-sm truncate">{$currentUser.name}</h3>
								<p class="text-xs text-muted-foreground">AniList Member</p>
								<Button variant="outline" size="sm" class="mt-1.5 h-6 text-xs" onclick={() => goto('/profile')}>
									View Profile
								</Button>
							</div>
						</div>
					</Card>
				{/if}
			</div>

			<!-- BOTTOM LEFT: Quick Actions -->
			<div class="absolute bottom-6 left-6 w-[30%]">
				<Card class="p-5 border-border/50 bg-card/60 backdrop-blur-md">
					<h2 class="text-base font-semibold mb-3 flex items-center gap-2">
						<Icon icon="solar:widget-4-bold" class="h-4 w-4 text-primary" />
						Quick Actions
					</h2>
					<div class="grid grid-cols-4 gap-2">
						{#each quickActions as action}
							<button
								onclick={() => goto(action.path)}
								class="flex flex-col items-center justify-center gap-1.5 p-2.5 rounded-xl border border-border/50 bg-background/50 hover:bg-primary/5 hover:border-primary/30 transition-all group"
							>
								<div class="flex h-9 w-9 items-center justify-center rounded-full bg-primary/10 group-hover:bg-primary/20 transition-colors">
									<Icon icon={action.icon} class="h-4 w-4 text-primary" />
								</div>
								<span class="text-[10px] font-medium text-foreground/80">{action.label}</span>
							</button>
						{/each}
					</div>
				</Card>
			</div>

			<!-- BOTTOM RIGHT: Theme Info / Actions -->
			<div class="absolute bottom-6 right-6 w-[30%]">
				<Card class="p-5 border-border/50 bg-card/60 backdrop-blur-md">
					<h2 class="text-base font-semibold mb-3 flex items-center gap-2">
						<Icon icon="solar:palette-bold" class="h-4 w-4 text-primary" />
						Appearance
					</h2>
					<div class="space-y-2.5">
						<div class="flex items-center justify-between p-3 rounded-lg bg-background/50">
							<div class="flex items-center gap-3">
								<div class="h-9 w-9 rounded-full bg-gradient-to-br from-primary to-primary/70"></div>
								<div>
									<p class="text-sm font-medium capitalize">{themeStore.currentTheme}</p>
									<p class="text-xs text-muted-foreground">Active Theme</p>
								</div>
							</div>
							<Button variant="ghost" size="sm" onclick={() => goto('/settings')}>
								<Icon icon="solar:settings-bold" class="h-4 w-4" />
							</Button>
						</div>
						<Button variant="outline" class="w-full" onclick={() => goto('/settings')}>
							<Icon icon="solar:gallery-bold" class="h-4 w-4 mr-2" />
							Browse All Themes
						</Button>
					</div>
				</Card>
			</div>
		</div>
	</div>
{/if}


