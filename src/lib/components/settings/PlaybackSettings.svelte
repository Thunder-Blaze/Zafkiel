<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';
	
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
		<!-- Auto Skip Intro -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Auto-skip Intro</Label>
				<p class="text-sm text-foreground/70">Automatically skip opening sequences</p>
			</div>
			<Switch bind:checked={autoSkipIntro} />
		</div>

		<!-- Auto Skip Outro -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Auto-skip Outro</Label>
				<p class="text-sm text-foreground/70">Automatically skip ending credits</p>
			</div>
			<Switch bind:checked={autoSkipOutro} />
		</div>

		<!-- Auto-play Next Episode -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Auto-play Next Episode</Label>
				<p class="text-sm text-foreground/70">Continue watching with the next episode</p>
			</div>
			<Switch bind:checked={autoPlayNext} />
		</div>

		<!-- Prefer Dubbed -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Prefer Dubbed</Label>
				<p class="text-sm text-foreground/70">Choose dubbed audio over subtitles when available</p>
			</div>
			<Switch bind:checked={preferDub} />
		</div>
	</CardContent>
</Card>
</div>
