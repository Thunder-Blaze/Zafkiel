<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { authStore, isAuthenticated, currentUser, authLoading } from '$lib/stores/auth';
	import { themeStore } from '$lib/stores/theme.svelte';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';

	const uiScale = useUiScale();
	let sliderValue = $state(uiScale.scale * 100);
	let isLoggingOut = $state(false);
	let activeSection = $state('general');

	// Settings state
	let autoPlayTrailers = $state(true);
	let showSpoilers = $state(false);
	let enableNotifications = $state(true);
	let compactMode = $state(false);
	let animationsEnabled = $state(true);

	// Theme switching state
	let switchingTheme = $state(false);

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

	async function handleThemeSwitch(themeId: string): Promise<void> {
		if (switchingTheme || themeStore.currentTheme === themeId) return;
		
		switchingTheme = true;
		try {
			await themeStore.switchTheme(themeId);
			toast.success(`Switched to ${themeId} theme`);
		} catch (error) {
			toast.error('Failed to switch theme');
		} finally {
			switchingTheme = false;
		}
	}

	async function toggleDarkMode(): Promise<void> {
		try {
			await themeStore.toggleDarkMode();
			toast.success(themeStore.isDark ? 'Dark mode enabled' : 'Light mode enabled');
		} catch (error) {
			toast.error('Failed to toggle dark mode');
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

	function setActiveSection(section: string): void {
		activeSection = section;
	}

	const sections = [
		{ id: 'general', label: 'General', icon: 'solar:settings-bold-duotone' },
		{ id: 'appearance', label: 'Appearance', icon: 'solar:palette-bold-duotone' },
		{ id: 'playback', label: 'Playback', icon: 'solar:play-circle-bold-duotone' },
		{ id: 'privacy', label: 'Privacy', icon: 'solar:shield-check-bold-duotone' },
		{ id: 'account', label: 'Account', icon: 'solar:user-bold-duotone' },
	];
</script>

{#if $authLoading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-center space-y-4">
			<Icon icon="solar:refresh-circle-line-duotone" class="mx-auto h-12 w-12 animate-spin text-primary" />
			<p class="text-muted-foreground">Loading settings...</p>
		</div>
	</div>
{:else if $isAuthenticated}
	<div class="container mx-auto p-6 max-w-7xl">
		<!-- Header -->
		<div class="mb-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
			<div class="flex items-center gap-3 mb-2">
				<div class="rounded-lg bg-primary/10 p-2">
					<Icon icon="solar:settings-bold-duotone" class="h-6 w-6 text-primary" />
				</div>
				<h1 class="text-4xl font-bold">Settings</h1>
			</div>
			<p class="text-muted-foreground">Customize your Zafkiel experience</p>
		</div>

		<div class="grid gap-6 lg:grid-cols-[240px_1fr]">
			<!-- Sidebar Navigation -->
			<aside class="animate-in fade-in slide-in-from-left-4 duration-500 delay-100">
				<Card class="sticky top-6">
					<CardContent class="p-2">
						<nav class="flex flex-col gap-1">
							{#each sections as section, i}
								<button
									onclick={() => setActiveSection(section.id)}
									class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200 hover:bg-accent
										{activeSection === section.id 
											? 'bg-primary text-primary-foreground shadow-sm' 
											: 'text-muted-foreground hover:text-foreground'}"
									style="animation-delay: {(i + 2) * 100}ms"
								>
									<Icon icon={section.icon} class="h-4 w-4" />
									{section.label}
								</button>
							{/each}
						</nav>
					</CardContent>
				</Card>
			</aside>

			<!-- Main Content -->
			<main class="space-y-6">
				<!-- General Settings -->
				{#if activeSection === 'general'}
					<div class="space-y-6 animate-in fade-in slide-in-from-right-4 duration-500">
						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:widget-2-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>Interface</CardTitle>
								</div>
								<CardDescription>Configure how the app looks and behaves</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								<!-- Compact Mode -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Compact Mode
										</Label>
										<p class="text-sm text-muted-foreground">
											Show more content by reducing spacing
										</p>
									</div>
									<Switch
										checked={compactMode}
										onCheckedChange={(checked) => {
											compactMode = checked ?? false;
											toast.success(compactMode ? 'Compact mode enabled' : 'Compact mode disabled');
										}}
									/>
								</div>

								<Separator />

								<!-- Animations -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Enable Animations
										</Label>
										<p class="text-sm text-muted-foreground">
											Smooth transitions and micro-interactions
										</p>
									</div>
									<Switch
										checked={animationsEnabled}
										onCheckedChange={(checked) => {
											animationsEnabled = checked ?? false;
											toast.success(animationsEnabled ? 'Animations enabled' : 'Animations disabled');
										}}
									/>
								</div>

								<Separator />

								<!-- Notifications -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Notifications
										</Label>
										<p class="text-sm text-muted-foreground">
											Get notified about new episodes and updates
										</p>
									</div>
									<Switch
										checked={enableNotifications}
										onCheckedChange={(checked) => {
											enableNotifications = checked ?? false;
											toast.success(enableNotifications ? 'Notifications enabled' : 'Notifications disabled');
										}}
									/>
								</div>
							</CardContent>
						</Card>
					</div>
				{/if}

				<!-- Appearance Settings -->
				{#if activeSection === 'appearance'}
					<div class="space-y-6 animate-in fade-in slide-in-from-right-4 duration-500">
						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:scale-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>UI Scale</CardTitle>
								</div>
								<CardDescription>Adjust the overall size of interface elements</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								<div class="space-y-4">
									<div class="flex items-center justify-between">
										<span class="text-sm font-medium">Current Scale:</span>
										<span class="text-3xl font-bold tabular-nums bg-primary/10 px-4 py-2 rounded-lg">
											{sliderValue.toFixed(0)}%
										</span>
									</div>
									
									<div class="relative">
										<input
											type="range"
											value={sliderValue}
											oninput={handleScaleChange}
											min="50"
											max="200"
											step="5"
											class="h-3 w-full cursor-pointer appearance-none rounded-full bg-secondary transition-all
												hover:bg-secondary/80
												accent-primary
												[&::-webkit-slider-thumb]:appearance-none
												[&::-webkit-slider-thumb]:h-6
												[&::-webkit-slider-thumb]:w-6
												[&::-webkit-slider-thumb]:rounded-full
												[&::-webkit-slider-thumb]:bg-primary
												[&::-webkit-slider-thumb]:shadow-lg
												[&::-webkit-slider-thumb]:transition-all
												[&::-webkit-slider-thumb]:hover:scale-110
												[&::-webkit-slider-thumb]:active:scale-95
												[&::-moz-range-thumb]:h-6
												[&::-moz-range-thumb]:w-6
												[&::-moz-range-thumb]:rounded-full
												[&::-moz-range-thumb]:bg-primary
												[&::-moz-range-thumb]:border-0
												[&::-moz-range-thumb]:shadow-lg
												[&::-moz-range-thumb]:transition-all
												[&::-moz-range-thumb]:hover:scale-110
												[&::-moz-range-thumb]:active:scale-95"
										/>
										<div class="absolute -top-8 left-0 right-0 flex justify-between text-xs text-muted-foreground pointer-events-none">
											<span>50%</span>
											<span>100%</span>
											<span>150%</span>
											<span>200%</span>
										</div>
									</div>

									<div class="grid grid-cols-2 gap-3 mt-6">
										<Button
											onclick={resetScale}
											variant="outline"
											class="group hover:border-primary transition-all duration-200"
										>
											<Icon icon="solar:refresh-bold" class="mr-2 h-4 w-4 group-hover:rotate-180 transition-transform duration-500" />
											Reset to Default
										</Button>
										<Button
											onclick={() => toast.success('Scale preview updated')}
											variant="secondary"
											class="group"
										>
											<Icon icon="solar:eye-bold" class="mr-2 h-4 w-4 group-hover:scale-110 transition-transform" />
											Preview
										</Button>
									</div>
								</div>
							</CardContent>
						</Card>

						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:palette-2-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>Theme</CardTitle>
								</div>
								<CardDescription>Choose your preferred color scheme</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								<!-- Dark Mode Toggle -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Dark Mode
										</Label>
										<p class="text-sm text-muted-foreground">
											Toggle between light and dark mode
										</p>
									</div>
									<Switch
										checked={themeStore.isDark}
										onCheckedChange={toggleDarkMode}
									/>
								</div>

								<Separator />

								<!-- Theme Selector -->
								<div class="space-y-3">
									<Label class="text-base font-medium">Color Theme</Label>
									<div class="grid grid-cols-2 md:grid-cols-3 gap-4">
										{#each themeStore.availableThemes as theme}
											{@const isActive = themeStore.currentTheme === theme.id}
											{@const colors = themeStore.isDark ? theme.colors.dark : theme.colors.light}
											
											<button
												onclick={() => handleThemeSwitch(theme.id)}
												disabled={switchingTheme}
												class="group relative flex flex-col gap-3 rounded-lg border-2 p-4 transition-all duration-200
													{isActive 
														? 'border-primary bg-accent scale-105 shadow-lg' 
														: 'border-muted hover:border-primary hover:bg-accent hover:scale-105'
													}
													{switchingTheme ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
											>
												<!-- Color Preview -->
												<div class="flex gap-1.5 h-8">
													<div 
														class="flex-1 rounded transition-transform group-hover:scale-105"
														style="background: {colors.primary}"
													></div>
													<div 
														class="flex-1 rounded transition-transform group-hover:scale-105"
														style="background: {colors.background}"
													></div>
													<div 
														class="flex-1 rounded transition-transform group-hover:scale-105"
														style="background: {colors.accent}"
													></div>
												</div>

												<!-- Theme Name -->
												<div class="text-center">
													<span class="text-sm font-medium">{theme.name}</span>
													<p class="text-xs text-muted-foreground mt-1">{theme.description}</p>
												</div>

												<!-- Active Indicator -->
												{#if isActive}
													<div class="absolute -top-1 -right-1 h-4 w-4 rounded-full bg-primary animate-pulse"></div>
												{/if}
											</button>
										{/each}
									</div>
								</div>
							</CardContent>
						</Card>
					</div>
				{/if}

				<!-- Playback Settings -->
				{#if activeSection === 'playback'}
					<div class="space-y-6 animate-in fade-in slide-in-from-right-4 duration-500">
						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:play-circle-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>Video Player</CardTitle>
								</div>
								<CardDescription>Configure video playback preferences</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								<!-- Auto-play Trailers -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Auto-play Trailers
										</Label>
										<p class="text-sm text-muted-foreground">
											Automatically play trailers when browsing
										</p>
									</div>
									<Switch
										checked={autoPlayTrailers}
										onCheckedChange={(checked) => {
											autoPlayTrailers = checked ?? false;
											toast.success(autoPlayTrailers ? 'Auto-play enabled' : 'Auto-play disabled');
										}}
									/>
								</div>

								<Separator />

								<!-- Quality Preference -->
								<div class="space-y-3">
									<Label class="text-base font-medium">Preferred Quality</Label>
									<div class="grid grid-cols-4 gap-2">
										{#each ['1080p', '720p', '480p', 'Auto'] as quality}
											<button
												class="rounded-lg border-2 p-3 text-sm font-medium transition-all duration-200
													{quality === '1080p' 
														? 'border-primary bg-primary/10 text-primary scale-105 shadow-md' 
														: 'border-muted hover:border-primary/50 hover:bg-accent hover:scale-105'}"
											>
												{quality}
											</button>
										{/each}
									</div>
								</div>
							</CardContent>
						</Card>
					</div>
				{/if}

				<!-- Privacy Settings -->
				{#if activeSection === 'privacy'}
					<div class="space-y-6 animate-in fade-in slide-in-from-right-4 duration-500">
						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:shield-check-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>Privacy & Security</CardTitle>
								</div>
								<CardDescription>Manage your privacy preferences</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								<!-- Show Spoilers -->
								<div class="flex items-center justify-between group">
									<div class="space-y-0.5">
										<Label class="text-base font-medium group-hover:text-primary transition-colors">
											Show Spoilers
										</Label>
										<p class="text-sm text-muted-foreground">
											Display spoiler content without warnings
										</p>
									</div>
									<Switch
										checked={showSpoilers}
										onCheckedChange={(checked) => {
											showSpoilers = checked ?? false;
											toast.success(showSpoilers ? 'Spoilers will be shown' : 'Spoilers will be hidden');
										}}
									/>
								</div>

								<Separator />

								<div class="rounded-lg bg-amber-500/10 border border-amber-500/20 p-4">
									<div class="flex gap-3">
										<Icon icon="solar:danger-triangle-bold-duotone" class="h-5 w-5 text-amber-500 flex-shrink-0 mt-0.5" />
										<div class="space-y-2">
											<p class="text-sm font-medium">Data Collection</p>
											<p class="text-xs text-muted-foreground">
												We only collect anonymous usage data to improve the app. Your watch history and preferences are stored locally and encrypted.
											</p>
										</div>
									</div>
								</div>
							</CardContent>
						</Card>
					</div>
				{/if}

				<!-- Account Settings -->
				{#if activeSection === 'account'}
					<div class="space-y-6 animate-in fade-in slide-in-from-right-4 duration-500">
						<Card class="hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:user-bold-duotone" class="h-5 w-5 text-primary" />
									<CardTitle>AniList Account</CardTitle>
								</div>
								<CardDescription>Manage your AniList integration</CardDescription>
							</CardHeader>
							<CardContent class="space-y-6">
								{#if $currentUser}
									<!-- Profile Info -->
									<div class="flex items-center gap-4 p-4 rounded-lg bg-accent/50 border">
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
										<div class="flex-1">
											<p class="font-semibold text-lg">{$currentUser.name}</p>
											<p class="text-sm text-muted-foreground">Connected to AniList</p>
										</div>
										<div class="h-3 w-3 rounded-full bg-green-500 animate-pulse"></div>
									</div>

									<Separator />

									<!-- Stats -->
									<div class="grid grid-cols-2 gap-4">
										<div class="space-y-1">
											<p class="text-sm text-muted-foreground">Total Anime</p>
											<p class="text-2xl font-bold">
												{$currentUser.statistics?.anime?.count || 0}
											</p>
										</div>
										<div class="space-y-1">
											<p class="text-sm text-muted-foreground">Mean Score</p>
											<p class="text-2xl font-bold">
												{$currentUser.statistics?.anime?.meanScore?.toFixed(1) || '0.0'}
											</p>
										</div>
									</div>

									<Separator />

									<!-- Actions -->
									<div class="space-y-3">
										<Button
											variant="outline"
											class="w-full group hover:border-primary transition-all duration-200"
											onclick={() => toast.success('Syncing with AniList...')}
										>
											<Icon icon="solar:refresh-bold" class="mr-2 h-4 w-4 group-hover:rotate-180 transition-transform duration-500" />
											Sync with AniList
										</Button>

										<Button
											variant="destructive"
											class="w-full group"
											onclick={handleLogout}
											disabled={isLoggingOut}
										>
											{#if isLoggingOut}
												<Icon icon="solar:refresh-circle-line-duotone" class="mr-2 h-4 w-4 animate-spin" />
												Logging out...
											{:else}
												<Icon icon="solar:logout-3-bold" class="mr-2 h-4 w-4 group-hover:translate-x-1 transition-transform" />
												Logout
											{/if}
										</Button>
									</div>
								{/if}
							</CardContent>
						</Card>

						<!-- Danger Zone -->
						<Card class="border-destructive/50 hover:shadow-lg transition-shadow duration-300">
							<CardHeader>
								<div class="flex items-center gap-2">
									<Icon icon="solar:danger-circle-bold-duotone" class="h-5 w-5 text-destructive" />
									<CardTitle class="text-destructive">Danger Zone</CardTitle>
								</div>
								<CardDescription>Irreversible actions - proceed with caution</CardDescription>
							</CardHeader>
							<CardContent class="space-y-4">
								<Button
									variant="outline"
									class="w-full border-destructive/50 text-destructive hover:bg-destructive hover:text-destructive-foreground group"
									onclick={() => toast.error('This feature is not yet implemented')}
								>
									<Icon icon="solar:trash-bin-trash-bold" class="mr-2 h-4 w-4 group-hover:scale-110 transition-transform" />
									Clear All Local Data
								</Button>

								<Button
									variant="outline"
									class="w-full border-destructive/50 text-destructive hover:bg-destructive hover:text-destructive-foreground group"
									onclick={() => toast.error('This feature is not yet implemented')}
								>
									<Icon icon="solar:user-cross-bold" class="mr-2 h-4 w-4 group-hover:scale-110 transition-transform" />
									Delete Account
								</Button>
							</CardContent>
						</Card>
					</div>
				{/if}
			</main>
		</div>
	</div>
{/if}

<style>
	/* Smooth animations */
	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slide-in-from-bottom {
		from {
			transform: translateY(1rem);
		}
		to {
			transform: translateY(0);
		}
	}

	@keyframes slide-in-from-left {
		from {
			transform: translateX(-1rem);
		}
		to {
			transform: translateX(0);
		}
	}

	@keyframes slide-in-from-right {
		from {
			transform: translateX(1rem);
		}
		to {
			transform: translateX(0);
		}
	}

	.animate-in {
		animation: fade-in 0.5s ease-out;
	}

	.fade-in {
		animation: fade-in 0.5s ease-out;
	}

	.slide-in-from-bottom-4 {
		animation: slide-in-from-bottom 0.5s ease-out;
	}

	.slide-in-from-left-4 {
		animation: slide-in-from-left 0.5s ease-out;
	}

	.slide-in-from-right-4 {
		animation: slide-in-from-right 0.5s ease-out;
	}
</style>
