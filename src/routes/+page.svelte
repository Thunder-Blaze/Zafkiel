<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { authStore, isAuthenticated, currentUser, authLoading } from '$lib/stores/auth';
	import { themeStore } from '$lib/stores/theme.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { goto } from '$app/navigation';
	import ThemeBadge from '$lib/components/dashboard/ThemeBadge.svelte';
	import CalendarWidget from '$lib/components/dashboard/CalendarWidget.svelte';
	import ProfileWidget from '$lib/components/dashboard/ProfileWidget.svelte';

	// Stats
	const stats = [
		{ label: 'Watching', value: 12, icon: 'solar:play-bold', bgColor: 'bg-primary/10', iconColor: 'text-primary' },
		{ label: 'Completed', value: 48, icon: 'solar:check-circle-bold', bgColor: 'bg-primary/10', iconColor: 'text-primary' },
		{ label: 'Plan to Watch', value: 23, icon: 'solar:bookmark-bold', bgColor: 'bg-primary/10', iconColor: 'text-primary' },
		{ label: 'Episodes', value: 1247, icon: 'solar:video-library-bold', bgColor: 'bg-primary/10', iconColor: 'text-primary' },
	];

	// Quick actions
	const quickActions = [
		{ label: 'Browse', icon: 'solar:magnifer-bold', path: '/anime' },
		{ label: 'My List', icon: 'solar:clipboard-list-bold', path: '/list' },
		{ label: 'Trending', icon: 'solar:fire-bold', path: '/trending' },
		{ label: 'Settings', icon: 'solar:settings-bold', path: '/settings' },
	];
</script>

{#if $authLoading}
	<Loader text="Loading your dashboard..." />
{:else}
	<div class="h-[calc(100vh-3rem)] overflow-hidden relative bg-black/40">
		<!-- Background Theme Image -->
		<div class="absolute flex items-center justify-center inset-0">
			<!-- Dynamic Text -->
			<h1
				class="absolute font-bold tracking-tight font-mono -z-10"
				style="
					font-size: {100 / (themeStore?.currentTheme?.length || 5)}vw;
					text-shadow: '2px 2px 4px rgba(0, 0, 0, 0.7)';
					user-select: 'none';
					pointer-events: 'none';
					color: transparent;
					-webkit-text-stroke-width: {20 / (themeStore?.currentTheme?.length || 5)}px;
					-webkit-text-stroke-color: var(--foreground);
				"
			>
				{themeStore?.currentTheme?.toUpperCase() || 'THEME'}
			</h1>
			<!-- Lighter Gradient Overlays for readability -->
			<div class="absolute inset-0 -z-20 bg-gradient-to-br from-background/70 via-background/20 to-background/50"></div>

			<img
				src="{themeStore.currentThemePath}/theme.png"
				onerror={(e) => e.target.src='/images/fallback-theme.png'}
				alt="Background Theme"
				class="object-contain w-full h-full"
			/>
		</div>

		<!-- Content Layer -->
		<div class="relative h-full w-full p-6">
			<!-- TOP LEFT: Welcome + Stats -->
			<div class="absolute top-6 left-6 space-y-2">
				<!-- Welcome Card -->
				<Card class="px-4 py-3 border-border/50 bg-card/70 backdrop-blur-md">
					<h1 class="text-lg font-bold tracking-tight">
						Welcome back! 👋
					</h1>
					<p class="text-xs text-muted-foreground mt-0.5">
						{$isAuthenticated && $currentUser?.name ? $currentUser.name : 'Guest'}
					</p>
				</Card>

				<!-- Stats Grid -->
				<div class="grid grid-cols-2 gap-1.5">
					{#each stats as stat}
						<Card class="px-2.5 py-2 hover:shadow-lg hover:scale-[1.02] transition-all cursor-pointer border-border/50 bg-card/70 backdrop-blur-md group">
							<div class="flex items-center gap-2">
								<div class="flex h-7 w-7 items-center justify-center rounded-lg {stat.bgColor} group-hover:scale-110 transition-transform">
									<Icon icon={stat.icon} class="h-3.5 w-3.5 {stat.iconColor}" />
								</div>
								<div class="flex-1 min-w-0">
									<p class="text-lg font-bold leading-none">{stat.value}</p>
									<p class="text-[9px] text-muted-foreground leading-tight mt-0.5 truncate">{stat.label}</p>
								</div>
							</div>
						</Card>
					{/each}
				</div>
			</div>

			<!-- TOP RIGHT: Calendar + Profile -->
			<div class="absolute top-6 right-6 flex flex-col items-end gap-4">
				<!-- Calendar Widget -->
				<CalendarWidget />

				<!-- Profile Card -->
				{#if $isAuthenticated && $currentUser}
					<ProfileWidget name={$currentUser.name} avatar={($currentUser.avatar?.large || $currentUser.avatar?.medium || null)} />
				{/if}
			</div>

			<!-- BOTTOM LEFT: Quick Actions -->
			<div class="absolute bottom-6 left-6">
				<Card class="p-4 border-border/50 bg-card/70 backdrop-blur-md">
					<h2 class="text-sm font-semibold mb-2.5 flex items-center gap-1.5">
						<Icon icon="solar:widget-4-bold" class="h-3.5 w-3.5 text-primary" />
						Quick Actions
					</h2>
					<div class="grid grid-cols-2 gap-1.5">
						{#each quickActions as action}
							<button
								onclick={() => goto(action.path)}
								class="flex items-center gap-2 px-3 py-2 rounded-lg border border-border/50 bg-background/50 hover:bg-primary/10 hover:border-primary/50 hover:shadow-md transition-all group"
							>
								<div class="flex h-8 w-8 items-center justify-center rounded-lg bg-primary/10 group-hover:bg-primary/20 group-hover:scale-110 transition-all shrink-0">
									<Icon icon={action.icon} class="h-4 w-4 text-primary" />
								</div>
								<span class="text-xs font-medium text-foreground/90 group-hover:text-primary transition-colors">{action.label}</span>
							</button>
						{/each}
					</div>
				</Card>
			</div>

			<!-- BOTTOM RIGHT: Theme Info / Actions -->
			 <div class="absolute bottom-6 right-6">
				<!-- Theme Badge -->
				<ThemeBadge />
			</div>
		</div>
	</div>
{/if}


