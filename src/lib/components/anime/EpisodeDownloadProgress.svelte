<script lang="ts">
	import { Progress } from '$lib/components/ui/progress';
	import Icon from '@iconify/svelte';
	import { formatBytes } from '$lib/utils/data-filters';

	let { progress, downloadSpeed, state } = $props<{
		progress: number;
		downloadSpeed: number;
		state: string;
	}>();

	const isComplete = $derived(state === 'seeding' || state === 'done' || progress >= 1);
</script>

<div class="flex w-full min-w-48 max-w-sm flex-col gap-1.5 rounded-md bg-muted/40 p-2 border border-border/50">
	<div class="flex items-center justify-between text-xs font-medium">
		<span class="flex items-center gap-1.5 text-muted-foreground">
			{#if state === 'paused'}
				<Icon icon="solar:pause-circle-bold" class="size-3.5" />
				Paused
			{:else if isComplete}
				<Icon icon="solar:check-circle-bold" class="size-3.5 text-primary" />
				Completed
			{:else}
				<Icon icon="solar:download-square-bold" class="size-3.5 text-primary" />
				Downloading
			{/if}
		</span>
		<div class="flex items-center gap-2">
			{#if !isComplete && state !== 'paused'}
				<span class="text-muted-foreground">{formatBytes(downloadSpeed)}/s</span>
			{/if}
			<span class="font-bold text-foreground">{(progress * 100).toFixed(1)}%</span>
		</div>
	</div>
	<Progress value={progress * 100} class="h-1.5" />
</div>
