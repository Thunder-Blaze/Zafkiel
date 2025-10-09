<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { authLoading, isAuthenticated } from '$lib/stores/auth';
	import Icon from '@iconify/svelte';
	import { fade, slide } from 'svelte/transition';

	// Import setting components
	import InterfaceSettings from '$lib/components/settings/InterfaceSettings.svelte';
	import UiScaleSettings from '$lib/components/settings/UiScaleSettings.svelte';
	import ThemeSettings from '$lib/components/settings/ThemeSettings.svelte';
	import PreferencesSettings from '$lib/components/settings/PreferencesSettings.svelte';
	import PlaybackSettings from '$lib/components/settings/PlaybackSettings.svelte';
	import AccountSettings from '$lib/components/settings/AccountSettings.svelte';

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
	}

	const sections = [
		{ id: 'general', label: 'General', icon: 'solar:settings-bold' },
		{ id: 'appearance', label: 'Appearance', icon: 'solar:pallete-2-bold' },
		{ id: 'preferences', label: 'Preferences', icon: 'solar:tuning-bold' },
		{ id: 'playback', label: 'Playback', icon: 'solar:play-circle-bold' },
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
	<div class="container mx-auto max-w-7xl p-6" transition:fade={{ duration: 300 }}>
		<!-- Header -->
		<div class="mb-8" transition:slide={{ duration: 300 }}>
			<div class="mb-2 flex items-center gap-3">
				<div class="rounded-lg bg-primary/10 p-2">
					<Icon icon="solar:settings-bold" class="h-6 w-6 text-primary" />
				</div>
				<h1 class="text-4xl font-bold">Settings</h1>
			</div>
			<p class="text-foreground/70">Customize your Zafkiel experience</p>
		</div>

		<div class="grid gap-6 lg:grid-cols-[240px_1fr]">
			<!-- Sidebar Navigation -->
			<aside transition:slide={{ duration: 300, delay: 100 }}>
				<Card class="sticky top-6">
					<CardContent class="p-2">
						<nav class="flex flex-col gap-1">
							{#each sections as section}
								<button
									onclick={() => setActiveSection(section.id)}
									class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200
										{activeSection === section.id
										? 'scale-[1.02] bg-primary/90 text-primary-foreground shadow-lg'
										: 'text-foreground/70 hover:scale-[1.01] hover:bg-primary/10 hover:text-foreground hover:shadow-md'}"
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
						<ThemeSettings />
					</div>
				{/if}

				<!-- Preferences Settings -->
				{#if activeSection === 'preferences'}
					<div class="space-y-6">
						<PreferencesSettings
							bind:show18Plus
							bind:showInList
							bind:selectedGenres
						/>
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

				<!-- Account Settings -->
				{#if activeSection === 'account'}
					<div class="space-y-6">
						<AccountSettings />
					</div>
				{/if}
			</main>
		</div>
	</div>
{/if}
