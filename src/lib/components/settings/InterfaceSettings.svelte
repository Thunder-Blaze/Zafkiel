<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { ConfigService, type UiConfig } from '$lib/services/config';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';

	interface Props {
		autoPlayTrailers?: boolean;
		showSpoilers?: boolean;
		enableNotifications?: boolean;
		compactMode?: boolean;
		onautoPlayTrailersChange?: (value: boolean) => void;
		onshowSpoilersChange?: (value: boolean) => void;
		onenableNotificationsChange?: (value: boolean) => void;
		oncompactModeChange?: (value: boolean) => void;
	}

	let {
		autoPlayTrailers = $bindable(true),
		showSpoilers = $bindable(false),
		enableNotifications = $bindable(true),
		compactMode = $bindable(false),
	}: Props = $props();

	// Load config-backed settings
	let animationsEnabled = $state(true);
	let glowEffectsEnabled = $state(true);
	let blurEffectsEnabled = $state(true);

	// Load settings from config
	$effect(() => {
		ConfigService.getUiConfig()
			.then((config: UiConfig) => {
				animationsEnabled = config.animations;
				glowEffectsEnabled = config.glow_effects;
				blurEffectsEnabled = config.blur_effects;
			})
			.catch((error) => {
				console.error('Failed to load UI config:', error);
			});
	});

	async function handleAnimationsToggle(): Promise<void> {
		const newValue = !animationsEnabled;
		try {
			await ConfigService.updateAnimations(newValue);
			animationsEnabled = newValue;
			toast.success(`Animations ${newValue ? 'enabled' : 'disabled'}`);
		} catch (error) {
			toast.error('Failed to update animations setting');
		}
	}

	async function handleGlowEffectsToggle(): Promise<void> {
		const newValue = !glowEffectsEnabled;
		try {
			await ConfigService.updateGlowEffects(newValue);
			glowEffectsEnabled = newValue;
			toast.success(`Glow effects ${newValue ? 'enabled' : 'disabled'}`);
		} catch (error) {
			toast.error('Failed to update glow effects setting');
		}
	}

	async function handleBlurEffectsToggle(): Promise<void> {
		const newValue = !blurEffectsEnabled;
		try {
			await ConfigService.updateBlurEffects(newValue);
			blurEffectsEnabled = newValue;
			toast.success(`Blur effects ${newValue ? 'enabled' : 'disabled'}`);
		} catch (error) {
			toast.error('Failed to update blur effects setting');
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:monitor-bold" class="h-5 w-5 text-primary" />
				</div>
				<div>
					<CardTitle>Interface</CardTitle>
					<CardDescription>Customize your viewing experience</CardDescription>
				</div>
			</div>
		</CardHeader>
	<CardContent class="space-y-6">
		<!-- Auto-play Trailers -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Auto-play Trailers</Label>
				<p class="text-sm text-foreground/70">Automatically play trailers when browsing</p>
			</div>
			<Switch bind:checked={autoPlayTrailers} />
		</div>

		<!-- Show Spoilers -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Show Spoilers</Label>
				<p class="text-sm text-foreground/70">Display spoiler content without warnings</p>
			</div>
			<Switch bind:checked={showSpoilers} />
		</div>

		<!-- Notifications -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Enable Notifications</Label>
				<p class="text-sm text-foreground/70">Get notified about new episodes and updates</p>
			</div>
			<Switch bind:checked={enableNotifications} />
		</div>

		<!-- Compact Mode -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Compact Mode</Label>
				<p class="text-sm text-foreground/70">Display more content in less space</p>
			</div>
			<Switch bind:checked={compactMode} />
		</div>

		<!-- Animations -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Animations</Label>
				<p class="text-sm text-foreground/70">Enable interface animations and transitions</p>
			</div>
			<Switch checked={animationsEnabled} onCheckedChange={handleAnimationsToggle} />
		</div>

		<!-- Glow Effects -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Glow Effects</Label>
				<p class="text-sm text-foreground/70">Add glow effects to images and cards</p>
			</div>
			<Switch checked={glowEffectsEnabled} onCheckedChange={handleGlowEffectsToggle} />
		</div>

		<!-- Blur Effects -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Blur Effects</Label>
				<p class="text-sm text-foreground/70">Enable backdrop blur effects on overlays</p>
			</div>
			<Switch checked={blurEffectsEnabled} onCheckedChange={handleBlurEffectsToggle} />
		</div>
	</CardContent>
</Card>
</div>
