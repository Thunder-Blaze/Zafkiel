<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { useConfigState } from '$lib/stores/config.svelte';

	interface Props {
		autoSkipIntro?: boolean;
		autoSkipOutro?: boolean;
		autoPlayNext?: boolean;
		preferDub?: boolean;
		onautoSkipIntroChange?: (value: boolean) => void;
		onautoSkipOutroChange?: (value: boolean) => void;
		onautoPlayNextChange?: (value: boolean) => void;
		onpreferDubChange?: (value: boolean) => void;
	}

	let {
		autoSkipIntro = $bindable(true),
		autoSkipOutro = $bindable(false),
		autoPlayNext = $bindable(true),
		preferDub = $bindable(false),
	}: Props = $props();

	const config = useConfigState();

	let externalPlayerPath = $state('');
	let isSavingPath = $state(false);
	let saveStatus = $state<'idle' | 'saved' | 'error'>('idle');

	onMount(async () => {
		try {
			const result = await invoke<{
				success: boolean;
				data?: { external_player_path?: string | null };
			}>('get_player_config');
			if (result.success && result.data?.external_player_path) {
				externalPlayerPath = result.data.external_player_path;
			}
		} catch (e) {
			console.error('Failed to load player config:', e);
		}
	});

	async function saveExternalPlayerPath() {
		isSavingPath = true;
		saveStatus = 'idle';
		try {
			const path = externalPlayerPath.trim() || null;
			await invoke('update_external_player_path', { path });
			saveStatus = 'saved';
			setTimeout(() => {
				saveStatus = 'idle';
			}, 2000);
		} catch (e) {
			console.error('Failed to save player path:', e);
			saveStatus = 'error';
		} finally {
			isSavingPath = false;
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:play-bold" class="h-5 w-5 text-primary" />
				</div>
				<div>
					<CardTitle>Video Player</CardTitle>
					<CardDescription>Configure playback behavior</CardDescription>
				</div>
			</div>
		</CardHeader>
		<CardContent class="space-y-6">
			<!-- External Player Path -->
			<div class="space-y-3 rounded-lg border border-border/50 bg-foreground/5 p-4">
				<div class="space-y-0.5">
					<Label class="text-base font-medium">External Player Path</Label>
					<p class="text-sm text-foreground/70">
						Path to the media player executable used for external playback (e.g.
						<code class="rounded bg-muted px-1 py-0.5 text-xs">C:\tools\mpv\mpv.exe</code>). Leave
						empty to use <code class="rounded bg-muted px-1 py-0.5 text-xs">mpv</code> from system PATH.
					</p>
				</div>
				<div class="flex gap-2">
					<Input
						bind:value={externalPlayerPath}
						placeholder="mpv"
						class="font-mono text-sm"
						onkeydown={(e: KeyboardEvent) => {
							if (e.key === 'Enter') saveExternalPlayerPath();
						}}
					/>
					<Button
						variant="outline"
						size="sm"
						onclick={saveExternalPlayerPath}
						disabled={isSavingPath}
						class="shrink-0"
					>
						{#if saveStatus === 'saved'}
							<Icon icon="lucide:check" class="mr-1 h-4 w-4 text-green-500" />
							Saved
						{:else if saveStatus === 'error'}
							<Icon icon="lucide:x" class="mr-1 h-4 w-4 text-red-500" />
							Error
						{:else}
							Save
						{/if}
					</Button>
				</div>
			</div>

			<!-- Auto Skip Intro -->
			<div
				class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4"
			>
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Auto-skip Intro</Label>
					<p class="text-sm text-foreground/70">Automatically skip opening sequences</p>
				</div>
				<Switch bind:checked={autoSkipIntro} />
			</div>

			<!-- Auto Skip Outro -->
			<div
				class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4"
			>
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Auto-skip Outro</Label>
					<p class="text-sm text-foreground/70">Automatically skip ending credits</p>
				</div>
				<Switch bind:checked={autoSkipOutro} />
			</div>

			<!-- Auto-play Next Episode -->
			<div
				class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4"
			>
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Auto-play Next Episode</Label>
					<p class="text-sm text-foreground/70">Continue watching with the next episode</p>
				</div>
				<Switch bind:checked={autoPlayNext} />
			</div>

			<!-- Auto-select Next Stream -->
			<div
				class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4"
			>
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Auto-select Next Stream</Label>
					<p class="text-sm text-foreground/70">
						Automatically pick the best stream based on preference
					</p>
				</div>
				<Switch
					checked={config.autoSelectNextStream}
					onCheckedChange={(checked) => config.setAutoSelectNextStream(checked)}
				/>
			</div>

			<!-- Default Playback Speed -->
			<div class="space-y-3 rounded-lg border border-border/50 bg-foreground/5 p-4">
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Default Playback Speed</Label>
					<p class="text-sm text-foreground/70">Set the standard playback speed for all videos</p>
				</div>
				<div class="flex gap-2">
					<select
						class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
						value={config.playbackSpeed}
						onchange={(e) => config.setPlaybackSpeed(parseFloat(e.currentTarget.value))}
					>
						{#each [0.5, 0.75, 1, 1.25, 1.5, 1.75, 2, 3, 4] as speed}
							<option value={speed}>{speed}x</option>
						{/each}
					</select>
				</div>
			</div>

			<!-- Prefer Dubbed -->
			<div
				class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4"
			>
				<div class="space-y-0.5">
					<Label class="text-base font-medium">Prefer Dubbed</Label>
					<p class="text-sm text-foreground/70">
						Choose dubbed audio over subtitles when available
					</p>
				</div>
				<Switch bind:checked={preferDub} />
			</div>
		</CardContent>
	</Card>
</div>
