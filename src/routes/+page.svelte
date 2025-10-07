<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { authStore, isAuthenticated, currentUser, authLoading } from '$lib/stores/auth';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';

	const uiScale = useUiScale();
	let sliderValue = $state(uiScale.scale * 100);
	let isLoggingOut = $state(false);

	// Redirect to login if not authenticated
	$effect(() => {
		if (browser && !$authLoading && !$isAuthenticated) {
			goto('/login');
		}
	});

	// Update slider when scale changes
	$effect(() => {
		sliderValue = uiScale.scale * 100;
	});

	function handleScaleChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const scale = Number(target.value) / 100;
		sliderValue = Number(target.value);

		// Debounce the actual config update
		clearTimeout(scaleUpdateTimeout);
		scaleUpdateTimeout = setTimeout(async () => {
			try {
				await uiScale.setScale(scale);
				toast.success(`UI Scale set to ${scale.toFixed(2)}x`);
			} catch (error) {
				toast.error('Failed to update UI scale');
			}
		}, 300);
	}

	let scaleUpdateTimeout: ReturnType<typeof setTimeout>;

	async function resetScale(): Promise<void> {
		try {
			await uiScale.resetScale();
			toast.success('UI Scale reset to 1.0x');
		} catch (error) {
			toast.error('Failed to reset UI scale');
		}
	}

	async function handleLogout(): Promise<void> {
		isLoggingOut = true;
		try {
			await authStore.logout();
			toast.success('Logged out successfully');
			goto('/login');
		} catch (error) {
			toast.error('Failed to logout');
		} finally {
			isLoggingOut = false;
		}
	}

	function formatDate(timestamp: number): string {
		return new Date(timestamp * 1000).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

{#if $authLoading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-center space-y-4">
			<Icon icon="solar:refresh-circle-line-duotone" class="mx-auto h-12 w-12 animate-spin text-primary" />
			<p class="text-muted-foreground">Loading your profile...</p>
		</div>
	</div>
{:else if $currentUser}
	<div class="container mx-auto p-8">
		<!-- Header with Profile -->
		<div class="mb-8 flex items-start justify-between gap-6 flex-wrap">
			<div class="flex-1 min-w-0">
				<h1 class="mb-2 text-4xl font-bold">Welcome back, {$currentUser.name}!</h1>
				<p class="text-muted-foreground">Manage your anime experience and preferences</p>
			</div>
			
			<Card class="flex-shrink-0 w-full sm:w-auto">
				<CardContent class="flex items-center gap-4 p-4">
					{#if $currentUser.avatar?.large}
						<img 
							src={$currentUser.avatar.large} 
							alt={$currentUser.name}
							class="h-16 w-16 rounded-full object-cover ring-2 ring-primary/20"
						/>
					{:else}
						<div class="h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center ring-2 ring-primary/20">
							<Icon icon="solar:user-bold" class="h-8 w-8 text-primary" />
						</div>
					{/if}
					<div class="flex-1 min-w-0">
						<p class="font-semibold text-lg truncate">{$currentUser.name}</p>
						{#if $currentUser.createdAt}
							<p class="text-xs text-muted-foreground">Member since {formatDate($currentUser.createdAt)}</p>
						{/if}
					</div>
					<Button 
						variant="ghost" 
						size="icon"
						onclick={handleLogout}
						disabled={isLoggingOut}
						title="Logout"
					>
						{#if isLoggingOut}
							<Icon icon="solar:refresh-circle-line-duotone" class="h-5 w-5 animate-spin" />
						{:else}
							<Icon icon="solar:logout-3-bold" class="h-5 w-5" />
						{/if}
					</Button>
				</CardContent>
			</Card>
		</div>

		<!-- User Stats -->
		{#if $currentUser.statistics?.anime}
			<div class="mb-8 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<Card>
					<CardContent class="flex items-center gap-3 p-6">
						<div class="rounded-full bg-blue-500/10 p-3">
							<Icon icon="solar:play-circle-bold-duotone" class="h-6 w-6 text-blue-500" />
						</div>
						<div>
							<p class="text-2xl font-bold">{$currentUser.statistics.anime.count || 0}</p>
							<p class="text-xs text-muted-foreground">Total Anime</p>
						</div>
					</CardContent>
				</Card>
				
				<Card>
					<CardContent class="flex items-center gap-3 p-6">
						<div class="rounded-full bg-purple-500/10 p-3">
							<Icon icon="solar:video-library-bold-duotone" class="h-6 w-6 text-purple-500" />
						</div>
						<div>
							<p class="text-2xl font-bold">{$currentUser.statistics.anime.episodesWatched || 0}</p>
							<p class="text-xs text-muted-foreground">Episodes Watched</p>
						</div>
					</CardContent>
				</Card>
				
				<Card>
					<CardContent class="flex items-center gap-3 p-6">
						<div class="rounded-full bg-green-500/10 p-3">
							<Icon icon="solar:clock-circle-bold-duotone" class="h-6 w-6 text-green-500" />
						</div>
						<div>
							<p class="text-2xl font-bold">
								{Math.round(($currentUser.statistics.anime.minutesWatched || 0) / 60 / 24)}
							</p>
							<p class="text-xs text-muted-foreground">Days Watched</p>
						</div>
					</CardContent>
				</Card>
				
				<Card>
					<CardContent class="flex items-center gap-3 p-6">
						<div class="rounded-full bg-orange-500/10 p-3">
							<Icon icon="solar:star-bold-duotone" class="h-6 w-6 text-orange-500" />
						</div>
						<div>
							<p class="text-2xl font-bold">
								{$currentUser.statistics.anime.meanScore ? $currentUser.statistics.anime.meanScore.toFixed(1) : '0.0'}
							</p>
							<p class="text-xs text-muted-foreground">Mean Score</p>
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}

		<Separator class="my-8" />

		<!-- Settings Section -->
		<h2 class="mb-6 text-2xl font-bold">Application Settings</h2>
		
		<div class="grid gap-6 md:grid-cols-2">
			<!-- UI Scale Control -->
			<Card>
				<CardHeader>
					<div class="flex items-center gap-2">
						<Icon icon="solar:scale-bold-duotone" class="h-5 w-5 text-primary" />
						<CardTitle>UI Scale Control</CardTitle>
					</div>
					<CardDescription>Adjust the overall size of the UI elements (50% - 200%)</CardDescription>
				</CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<span class="text-sm font-medium">Current Scale:</span>
							<span class="text-2xl font-bold">{sliderValue.toFixed(0)}%</span>
						</div>
						<input
							type="range"
							value={sliderValue}
							oninput={handleScaleChange}
							min="50"
							max="200"
							step="5"
							class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-secondary accent-primary"
						/>
						<div class="flex justify-between text-xs text-muted-foreground">
							<span>50%</span>
							<span>100%</span>
							<span>200%</span>
						</div>
					</div>
					<Button onclick={resetScale} variant="outline" class="w-full">
						<Icon icon="solar:refresh-bold" class="mr-2 h-4 w-4" />
						Reset to Default (100%)
					</Button>
				</CardContent>
			</Card>

			<!-- Context Menu Info -->
			<Card>
				<CardHeader>
					<div class="flex items-center gap-2">
						<Icon icon="solar:menu-dots-bold-duotone" class="h-5 w-5 text-primary" />
						<CardTitle>Custom Context Menu</CardTitle>
					</div>
					<CardDescription>Right-click anywhere to try the custom context menu</CardDescription>
				</CardHeader>
				<CardContent>
				<ul class="space-y-2 text-sm">
						<li class="flex items-center gap-2">
							<Icon icon="solar:refresh-circle-bold" class="h-4 w-4 text-primary" />
							<span>Reload page</span>
						</li>
						<li class="flex items-center gap-2">
							<Icon icon="solar:arrow-left-bold" class="h-4 w-4 text-primary" />
							<span>Navigate back/forward</span>
						</li>
						<li class="flex items-center gap-2">
							<Icon icon="solar:code-bold" class="h-4 w-4 text-primary" />
							<span>Inspect Element (opens devtools)</span>
						</li>
					</ul>
				</CardContent>
			</Card>
		</div>

		<Separator class="my-8" />

		<!-- Quick Links -->
		<h2 class="mb-6 text-2xl font-bold">Quick Navigation</h2>
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			<Card class="group cursor-pointer transition-all hover:shadow-lg hover:border-primary/50">
				<a href="/anime" class="block p-6">
					<div class="flex items-start gap-4">
						<div class="rounded-lg bg-blue-500/10 p-3 group-hover:bg-blue-500/20 transition-colors">
							<Icon icon="solar:video-library-bold-duotone" class="h-8 w-8 text-blue-500" />
						</div>
						<div class="flex-1">
							<h3 class="font-semibold mb-1">Browse Anime</h3>
							<p class="text-sm text-muted-foreground">Discover trending and popular anime</p>
						</div>
					</div>
				</a>
			</Card>

			<Card class="group cursor-pointer transition-all hover:shadow-lg hover:border-primary/50">
				<a href="/demo" class="block p-6">
					<div class="flex items-start gap-4">
						<div class="rounded-lg bg-purple-500/10 p-3 group-hover:bg-purple-500/20 transition-colors">
							<Icon icon="solar:widget-bold-duotone" class="h-8 w-8 text-purple-500" />
						</div>
						<div class="flex-1">
							<h3 class="font-semibold mb-1">Component Demo</h3>
							<p class="text-sm text-muted-foreground">Explore UI components</p>
						</div>
					</div>
				</a>
			</Card>

			<Card class="group cursor-pointer transition-all hover:shadow-lg hover:border-primary/50">
				<a href="/config-demo" class="block p-6">
					<div class="flex items-start gap-4">
						<div class="rounded-lg bg-green-500/10 p-3 group-hover:bg-green-500/20 transition-colors">
							<Icon icon="solar:settings-bold-duotone" class="h-8 w-8 text-green-500" />
						</div>
						<div class="flex-1">
							<h3 class="font-semibold mb-1">Configuration</h3>
							<p class="text-sm text-muted-foreground">Manage app settings</p>
						</div>
					</div>
				</a>
			</Card>
		</div>

		<!-- About Section -->
		<Card class="mt-8">
			<CardHeader>
				<div class="flex items-center gap-2">
					<Icon icon="solar:info-circle-bold-duotone" class="h-5 w-5 text-primary" />
					<CardTitle>About Zafkiel</CardTitle>
				</div>
				<CardDescription>Your cross-platform anime watching companion</CardDescription>
			</CardHeader>
			<CardContent>
				<p class="text-sm leading-relaxed text-muted-foreground mb-4">
					Zafkiel is a modern, cross-platform anime watching application built with SvelteKit and Tauri. 
					It provides a seamless experience for tracking your anime list, discovering new shows, and 
					managing your viewing preferences across all your devices.
				</p>
				<div class="flex flex-wrap gap-2">
					<span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">SvelteKit</span>
					<span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">Tauri</span>
					<span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">TypeScript</span>
					<span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">Rust</span>
					<span class="px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium">AniList API</span>
				</div>
			</CardContent>
		</Card>
	</div>
{/if}