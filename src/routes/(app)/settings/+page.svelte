<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { authLoading, isAuthenticated } from '$lib/stores/auth';
	import Icon from '@iconify/svelte';
	import { fade, slide, fly, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import type { PageData } from './$types';

	// Import setting components
	import InterfaceSettings from '$lib/components/settings/InterfaceSettings.svelte';
	import UiScaleSettings from '$lib/components/settings/UiScaleSettings.svelte';
	import ThemeSettings from '$lib/components/settings/ThemeSettings.svelte';
	import PreferencesSettings from '$lib/components/settings/PreferencesSettings.svelte';
	import PlaybackSettings from '$lib/components/settings/PlaybackSettings.svelte';
	import AccountSettings from '$lib/components/settings/AccountSettings.svelte';
	import ThemeModeSettings from '$lib/components/settings/ThemeModeSettings.svelte';
	import ImageCacheManager from '$lib/components/settings/ImageCacheManager.svelte';

	// Get data from load function
	let { data }: { data: PageData } = $props();

	let activeSection = $state('general');

	// Settings state
	let autoPlayTrailers = $state(true);
	let showSpoilers = $state(false);
	let enableNotifications = $state(true);
	let compactMode = $state(false);

	// Preferences state
	let show18Plus = $state(false);
	let autoSkipIntro = $state(true);
	let autoSkipOutro = $state(false);
	let autoPlayNext = $state(true);
	let showInList = $state(true);
	let preferDub = $state(false);
	let selectedGenres = $state<string[]>(['Action', 'Adventure', 'Comedy', 'Drama', 'Fantasy']);

	function setActiveSection(section: string): void {
		activeSection = section;
		if (browser) {
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	const sections = [
		{ id: 'general', label: 'General', icon: 'solar:settings-bold' },
		{ id: 'appearance', label: 'Appearance', icon: 'solar:pallete-2-bold' },
		{ id: 'preferences', label: 'Preferences', icon: 'solar:tuning-bold' },
		{ id: 'playback', label: 'Playback', icon: 'solar:play-circle-bold' },
		{ id: 'cache', label: 'Cache', icon: 'solar:download-bold' },
		{ id: 'account', label: 'Account', icon: 'solar:user-bold' },
	];
</script>

{#if $authLoading}
	<div class="flex min-h-screen items-center justify-center" transition:fade={{ duration: 200 }}>
		<div class="space-y-4 text-center">
			<Icon
				icon="solar:refresh-circle-line-duotone"
				class="mx-auto h-12 w-12 animate-spin text-primary"
			/>
			<p class="text-foreground/60">Loading settings...</p>
		</div>
	</div>
{:else if $isAuthenticated}
	<div class="container mx-auto max-w-7xl p-4 lg:p-8" transition:fade={{ duration: 300 }}>
		<!-- Header -->
		<div class="mb-8" transition:slide={{ duration: 300 }}>
			<div class="mb-2 flex items-center gap-3">
				<div class="rounded-xl bg-primary/10 p-3">
					<Icon icon="solar:settings-bold-duotone" class="h-8 w-8 text-primary" />
				</div>
				<div>
					<h1 class="text-3xl font-bold tracking-tight lg:text-4xl">Settings</h1>
					<p class="text-muted-foreground">Customize your Zafkiel experience</p>
				</div>
			</div>
		</div>

		<div class="grid gap-8 lg:grid-cols-[280px_1fr]">
			<!-- Sidebar Navigation (Desktop) -->
			<aside class="hidden lg:block" transition:slide={{ duration: 300, delay: 100 }}>
				<Card
					class="sticky top-24 overflow-hidden border-none bg-background/50 shadow-sm backdrop-blur-xl"
				>
					<CardContent class="p-2">
						<nav class="flex flex-col gap-1">
							{#each sections as section}
								<button
									onclick={() => setActiveSection(section.id)}
									class="group flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-all duration-200
										{activeSection === section.id
										? 'bg-primary text-primary-foreground shadow-md'
										: 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
								>
									<Icon
										icon={section.icon}
										class="h-5 w-5 transition-transform duration-200 group-hover:scale-110"
									/>
									{section.label}
									{#if activeSection === section.id}
										<div
											class="ml-auto h-1.5 w-1.5 rounded-full bg-white/50"
											transition:scale
										></div>
									{/if}
								</button>
							{/each}
						</nav>
					</CardContent>
				</Card>
			</aside>

			<!-- Mobile Navigation (Top Tabs) -->
			<div
				class="sticky top-16 z-40 -mx-4 mb-6 overflow-x-auto bg-background/80 px-4 py-2 backdrop-blur-md lg:hidden"
			>
				<div class="flex gap-2">
					{#each sections as section}
						<button
							onclick={() => setActiveSection(section.id)}
							class="flex shrink-0 items-center gap-2 rounded-full border px-4 py-2 text-sm font-medium transition-all
								{activeSection === section.id
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-transparent bg-muted/50 text-muted-foreground hover:bg-muted hover:text-foreground'}"
						>
							<Icon icon={section.icon} class="h-4 w-4" />
							{section.label}
						</button>
					{/each}
				</div>
			</div>

			<!-- Main Content -->
			<main class="min-h-[500px] space-y-6">
				{#key activeSection}
					<div
						in:slide={{ duration: 300, axis: 'y' }}
						out:slide={{ duration: 300, axis: 'y' }}
						class="space-y-6"
					>
						<!-- General Settings -->
						{#if activeSection === 'general'}
							<div class="space-y-6">
								<InterfaceSettings
									bind:autoPlayTrailers
									bind:showSpoilers
									bind:enableNotifications
									bind:compactMode
								/>
							</div>
						{/if}

						<!-- Appearance Settings -->
						{#if activeSection === 'appearance'}
							<div class="space-y-6">
								<UiScaleSettings />
								<ThemeModeSettings />
								<ThemeSettings />
							</div>
						{/if}

						<!-- Preferences Settings -->
						{#if activeSection === 'preferences'}
							<div class="space-y-6">
								<PreferencesSettings bind:show18Plus bind:showInList bind:selectedGenres />
							</div>
						{/if}

						<!-- Playback Settings -->
						{#if activeSection === 'playback'}
							<div class="space-y-6">
								<PlaybackSettings
									bind:autoSkipIntro
									bind:autoSkipOutro
									bind:autoPlayNext
									bind:preferDub
								/>
							</div>
						{/if}

						<!-- Cache Settings -->
						{#if activeSection === 'cache'}
							<div class="space-y-6">
								<ImageCacheManager />
							</div>
						{/if}

						<!-- Account Settings -->
						{#if activeSection === 'account'}
							<div class="space-y-6">
								<AccountSettings />
							</div>
						{/if}
					</div>
				{/key}
			</main>
		</div>
	</div>
{/if}
