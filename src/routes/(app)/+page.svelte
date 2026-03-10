<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { authStore, isAuthenticated, currentUser, authLoading } from '$lib/stores/auth';
	import { useThemeState } from '$lib/stores/theme.svelte';
	import Loader from '$lib/components/Loader.svelte';
	import { goto } from '$app/navigation';
	import ThemeBadge from '$lib/components/dashboard/ThemeBadge.svelte';
	import CalendarWidget from '$lib/components/dashboard/CalendarWidget.svelte';
	import ProfileWidget from '$lib/components/dashboard/ProfileWidget.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	// Stats from auth store
	console.log($currentUser);
	const animeCount = $derived($currentUser?.statistics?.anime?.count ?? 0);
	const episodesWatched = $derived($currentUser?.statistics?.anime?.episodesWatched ?? 0);
	const mangaCount = $derived($currentUser?.statistics?.manga?.count ?? 0);
	const chaptersRead = $derived($currentUser?.statistics?.manga?.chaptersRead ?? 0);

	const stats = $derived([
		{
			label: 'Anime Count',
			value: animeCount,
			icon: 'solar:videocamera-record-bold',
			bgColor: 'bg-primary/10',
			iconColor: 'text-primary',
		},
		{
			label: 'Episodes Watched',
			value: episodesWatched,
			icon: 'solar:video-library-bold',
			bgColor: 'bg-primary/10',
			iconColor: 'text-primary',
		},
		{
			label: 'Manga Count',
			value: mangaCount,
			icon: 'solar:book-bold',
			bgColor: 'bg-primary/10',
			iconColor: 'text-primary',
		},
		{
			label: 'Chapters Read',
			value: chaptersRead,
			icon: 'solar:bookmark-bold',
			bgColor: 'bg-primary/10',
			iconColor: 'text-primary',
		},
	]);

	// Quick actions
	const quickActions = [
		{ label: 'Search', icon: 'solar:magnifer-bold', path: '/search' },
		{ label: 'Anime', icon: 'solar:videocamera-record-bold', path: '/browse/anime' },
		{ label: 'Manga', icon: 'solar:book-bold', path: '/browse/manga' },
		{ label: 'My List', icon: 'solar:list-bold', path: '/list' },
	];

	const fallbackThemeImage = '/images/fallback-theme.png';
	const themeState = useThemeState();

	// GSAP entrance animations for dashboard panels
	onMount(async () => {
		if (!browser) return;
		// Wait for auth state to settle a bit before animating
		const { gsap } = await import('gsap');
		// Short delay so panels are in the DOM
		setTimeout(() => {
			const panels = document.querySelectorAll('[data-dash-panel]');
			if (panels.length) {
				gsap.fromTo(
					panels,
					{ opacity: 0, y: 20, scale: 0.97 },
					{
						opacity: 1,
						y: 0,
						scale: 1,
						duration: 0.5,
						stagger: 0.08,
						ease: 'power3.out',
						clearProps: 'all',
					}
				);
			}
		}, 100);
	});
</script>

{#if $authLoading}
	<Loader text="Loading your dashboard..." />
{:else}
	<div
		class="relative h-[calc(100vh-3rem)] overflow-hidden {themeState.currentThemeMode === 'dark'
			? 'bg-black/40'
			: 'bg-white/40'}"
	>
		<!-- Background Theme Image -->
		<div class="absolute inset-0 flex items-center justify-center">
			<!-- Dynamic Text -->
			<h1
				class="absolute -z-10 font-bold tracking-tight text-foreground/20"
				style="
					font-size: {110 / ($currentUser?.name?.length || 5)}vw;
					text-shadow: '2px 2px 4px rgba(0, 0, 0, 0.7)';
					user-select: 'none';
					pointer-events: 'none';
					"
			>
				{$currentUser?.name || 'GUEST'}
			</h1>
			<!-- Lighter Gradient Overlays for readability -->
			<div
				class="absolute inset-0 -z-20 bg-linear-to-br from-background/70 via-background/20 to-background/50"
			></div>

			<img
				src={themeState.currentThemeImagePath || fallbackThemeImage}
				onerror={(e) => {
					if ((e.target as HTMLImageElement)?.src)
						(e.target as HTMLImageElement).src = fallbackThemeImage;
				}}
				alt="Background Theme"
				class="h-full w-full object-contain"
			/>
		</div>

		<!-- Content Layer -->
		<div class="relative h-full w-full p-6">
			<!-- TOP LEFT: Welcome + Stats -->
			<div data-dash-panel class="absolute top-6 left-6 space-y-2">
				<!-- Welcome Card -->
				<Card class="border-border/50 bg-card/70 px-4 py-3 backdrop-blur-md">
					<h1 class="text-lg font-bold tracking-tight">Welcome back! 👋</h1>
					<p class="mt-0.5 text-xs text-muted-foreground">
						{$isAuthenticated && $currentUser?.name ? $currentUser.name : 'Guest'}
					</p>
				</Card>

				<!-- Stats Grid -->
				<div class="grid grid-cols-2 gap-1.5">
					{#each stats as stat}
						<Card
							class="group cursor-pointer border-border/50 bg-card/70 px-2.5 py-2 backdrop-blur-md transition-all hover:scale-[1.02] hover:shadow-lg"
						>
							<div class="flex items-center gap-2">
								<div
									class="flex h-7 w-7 items-center justify-center rounded-lg {stat.bgColor} transition-transform group-hover:scale-110"
								>
									<Icon icon={stat.icon} class="h-3.5 w-3.5 {stat.iconColor}" />
								</div>
								<div class="min-w-0 flex-1">
									<p class="text-lg leading-none font-bold">{stat.value}</p>
									<p class="mt-0.5 truncate text-[9px] leading-tight text-muted-foreground">
										{stat.label}
									</p>
								</div>
							</div>
						</Card>
					{/each}
				</div>
			</div>

			<!-- TOP RIGHT: Calendar + Profile -->
			<div data-dash-panel class="absolute top-6 right-6 flex flex-col items-end gap-4">
				<!-- Calendar Widget -->
				<CalendarWidget />

				<!-- Profile Card -->
				{#if $isAuthenticated && $currentUser}
					<ProfileWidget currentUser={$currentUser} />
				{/if}
			</div>

			<!-- BOTTOM LEFT: Quick Actions -->
			<div data-dash-panel class="absolute bottom-6 left-6">
				<Card class="border-border/50 bg-card/70 p-4 backdrop-blur-md">
					<h2 class="mb-2.5 flex items-center gap-1.5 text-sm font-semibold">
						<Icon icon="solar:widget-4-bold" class="h-3.5 w-3.5 text-primary" />
						Quick Actions
					</h2>
					<div class="grid grid-cols-2 gap-1.5">
						{#each quickActions as action}
							<button
								onclick={() => goto(action.path)}
								class="group flex items-center gap-2 rounded-lg border border-border/50 bg-background/50 px-3 py-2 transition-all hover:border-primary/50 hover:bg-primary/10 hover:shadow-md"
							>
								<div
									class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-primary/10 transition-all group-hover:scale-110 group-hover:bg-primary/20"
								>
									<Icon icon={action.icon} class="h-4 w-4 text-primary" />
								</div>
								<span
									class="text-xs font-medium text-foreground/90 transition-colors group-hover:text-primary"
									>{action.label}</span
								>
							</button>
						{/each}
					</div>
				</Card>
			</div>

			<div class="absolute right-6 bottom-6">
				<!-- Theme Badge -->
				<ThemeBadge />
			</div>
		</div>
	</div>
{/if}
